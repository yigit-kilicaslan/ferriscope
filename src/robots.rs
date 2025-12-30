use crate::error::ExtractionError;
use url::Url;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use redis;

/// In-memory cache for robots.txt content
pub type RobotsCache = Arc<RwLock<HashMap<String, Arc<robots::Robots>>>>;

/// Robots.txt checker with caching support
pub struct RobotsChecker {
    /// In-memory cache (domain -> robots.txt)
    memory_cache: Option<RobotsCache>,
    /// Redis client for distributed caching (optional)
    redis_client: Option<redis::Client>,
    /// Redis TTL in seconds (default: 1800 = 30 minutes)
    redis_ttl: u64,
}

impl RobotsChecker {
    pub fn new() -> Self {
        Self {
            memory_cache: None,
            redis_client: None,
            redis_ttl: 1800, // 30 minutes default
        }
    }

    /// Enable in-memory caching
    pub fn enable_memory_cache(&mut self) {
        self.memory_cache = Some(Arc::new(RwLock::new(HashMap::new())));
    }

    /// Enable Redis caching
    pub fn enable_redis_cache(&mut self, redis_url: &str) -> Result<(), ExtractionError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| ExtractionError::Other(format!("Failed to connect to Redis: {}", e)))?;
        self.redis_client = Some(client);
        Ok(())
    }

    /// Set Redis TTL in seconds
    pub fn set_redis_ttl(&mut self, ttl_secs: u64) {
        self.redis_ttl = ttl_secs;
    }

    /// Get robots.txt URL for a given page URL
    fn get_robots_url(page_url: &str) -> Result<String, ExtractionError> {
        let url = Url::parse(page_url)
            .map_err(|e| ExtractionError::InvalidUrl(format!("Invalid URL: {}", e)))?;
        
        let robots_url = format!("{}://{}/robots.txt", 
            url.scheme(), 
            url.host_str().ok_or_else(|| ExtractionError::InvalidUrl("No host in URL".to_string()))?
        );
        Ok(robots_url)
    }

    /// Extract domain from URL for caching
    fn extract_domain(url: &str) -> Result<String, ExtractionError> {
        let parsed = Url::parse(url)
            .map_err(|e| ExtractionError::InvalidUrl(format!("Invalid URL: {}", e)))?;
        parsed.host_str()
            .ok_or_else(|| ExtractionError::InvalidUrl("No host in URL".to_string()))
            .map(|s| s.to_string())
    }

    /// Fetch robots.txt from URL
    async fn fetch_robots_txt(&self, robots_url: &str) -> Result<String, ExtractionError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| ExtractionError::HttpError(format!("Failed to create HTTP client: {}", e)))?;
        
        let response = client
            .get(robots_url)
            .send()
            .await
            .map_err(|e| ExtractionError::HttpError(format!("Failed to fetch robots.txt: {}", e)))?;

        if response.status().is_success() {
            response.text()
                .await
                .map_err(|e| ExtractionError::HttpError(format!("Failed to read robots.txt: {}", e)))
        } else {
            // If robots.txt doesn't exist (404), return empty content (allows all)
            Ok(String::new())
        }
    }

    /// Get robots.txt from Redis cache
    async fn get_from_redis(&self, domain: &str) -> Result<Option<String>, ExtractionError> {
        if let Some(ref client) = self.redis_client {
            let mut conn = client.get_async_connection().await
                .map_err(|e| ExtractionError::Other(format!("Failed to get Redis connection: {}", e)))?;
            
            let key = format!("robots:{}", domain);
            let result: Result<String, redis::RedisError> = redis::cmd("GET")
                .arg(&key)
                .query_async(&mut conn)
                .await;
            
            match result {
                Ok(content) => Ok(Some(content)),
                Err(redis::RedisError::from((redis::ErrorKind::TypeError, _))) => Ok(None),
                Err(e) => Err(ExtractionError::Other(format!("Redis error: {}", e))),
            }
        } else {
            Ok(None)
        }
    }

    /// Store robots.txt in Redis cache
    async fn set_in_redis(&self, domain: &str, content: &str) -> Result<(), ExtractionError> {
        if let Some(ref client) = self.redis_client {
            let mut conn = client.get_async_connection().await
                .map_err(|e| ExtractionError::Other(format!("Failed to get Redis connection: {}", e)))?;
            
            let key = format!("robots:{}", domain);
            redis::cmd("SETEX")
                .arg(&key)
                .arg(self.redis_ttl)
                .arg(content)
                .query_async(&mut conn)
                .await
                .map_err(|e| ExtractionError::Other(format!("Failed to set Redis cache: {}", e)))?;
        }
        Ok(())
    }

    /// Remove robots.txt from Redis cache
    pub async fn remove_from_redis(&self, domain: &str) -> Result<(), ExtractionError> {
        if let Some(ref client) = self.redis_client {
            let mut conn = client.get_async_connection().await
                .map_err(|e| ExtractionError::Other(format!("Failed to get Redis connection: {}", e)))?;
            
            let key = format!("robots:{}", domain);
            redis::cmd("DEL")
                .arg(&key)
                .query_async(&mut conn)
                .await
                .map_err(|e| ExtractionError::Other(format!("Failed to delete from Redis: {}", e)))?;
        }
        Ok(())
    }

    /// Get robots.txt content (from cache or fetch)
    pub async fn get_robots_txt(&self, page_url: &str) -> Result<Arc<robots::Robots>, ExtractionError> {
        let domain = Self::extract_domain(page_url)?;
        
        // Try memory cache first
        if let Some(ref cache) = self.memory_cache {
            let cache_read = cache.read().await;
            if let Some(robots) = cache_read.get(&domain) {
                return Ok(Arc::clone(robots));
            }
        }

        // Try Redis cache
        if let Some(content) = self.get_from_redis(&domain).await? {
            let robots = Arc::new(robots::Robots::new("*", content.as_bytes())
                .map_err(|e| ExtractionError::ParseError(format!("Failed to parse robots.txt: {}", e)))?);
            
            // Store in memory cache if enabled
            if let Some(ref cache) = self.memory_cache {
                let mut cache_write = cache.write().await;
                cache_write.insert(domain.clone(), Arc::clone(&robots));
            }
            
            return Ok(robots);
        }

        // Fetch from URL
        let robots_url = Self::get_robots_url(page_url)?;
        let content = self.fetch_robots_txt(&robots_url).await?;
        
        let robots = Arc::new(robots::Robots::new("*", content.as_bytes())
            .map_err(|e| ExtractionError::ParseError(format!("Failed to parse robots.txt: {}", e)))?);

        // Store in memory cache if enabled
        if let Some(ref cache) = self.memory_cache {
            let mut cache_write = cache.write().await;
            cache_write.insert(domain.clone(), Arc::clone(&robots));
        }

        // Store in Redis cache if enabled
        if self.redis_client.is_some() {
            self.set_in_redis(&domain, &content).await?;
        }

        Ok(robots)
    }

    /// Set robots.txt content directly (for manual input)
    pub async fn set_robots_txt(&self, page_url: &str, content: &str) -> Result<(), ExtractionError> {
        let domain = Self::extract_domain(page_url)?;
        
        let robots = Arc::new(robots::Robots::new("*", content.as_bytes())
            .map_err(|e| ExtractionError::ParseError(format!("Failed to parse robots.txt: {}", e)))?);

        // Store in memory cache if enabled
        if let Some(ref cache) = self.memory_cache {
            let mut cache_write = cache.write().await;
            cache_write.insert(domain.clone(), robots);
        }

        // Store in Redis cache if enabled
        if self.redis_client.is_some() {
            self.set_in_redis(&domain, content).await?;
        }

        Ok(())
    }

    /// Check if a URL is allowed by robots.txt
    pub async fn is_allowed(&self, page_url: &str, user_agent: &str) -> Result<bool, ExtractionError> {
        let robots = self.get_robots_txt(page_url).await?;
        // robots crate uses path and user_agent
        let url = Url::parse(page_url)
            .map_err(|e| ExtractionError::InvalidUrl(format!("Invalid URL: {}", e)))?;
        let path = url.path();
        Ok(robots.allowed(path, user_agent))
    }

    /// Clear memory cache
    pub async fn clear_memory_cache(&self) {
        if let Some(ref cache) = self.memory_cache {
            let mut cache_write = cache.write().await;
            cache_write.clear();
        }
    }
}

impl Default for RobotsChecker {
    fn default() -> Self {
        Self::new()
    }
}

