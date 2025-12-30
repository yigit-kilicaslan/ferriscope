use crate::error::ExtractionError;
use crate::types::{Activities, ExtractionResult, ContentInfo};
use crate::text_extractor::extract_text_content;
use crate::link_extractor::extract_links_with_index;
use crate::socials_extractor::extract_socials_with_index;
use crate::videos_extractor::extract_video;
use crate::products_extractor::extract_products;
use crate::article_extractor::extract_article_with_index;
use crate::dom_index::DomIndex;
use crate::robots::RobotsChecker;
use reqwest::{Client, ClientBuilder, header::HeaderMap, header::HeaderValue};
use scraper::Html;
use whatlang::detect;
use std::collections::HashMap;
use std::time::Duration;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub timeout: Option<Duration>,
    pub user_agent: Option<String>,
    pub random_user_agent: bool,
    pub headers: HashMap<String, String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            timeout: Some(Duration::from_secs(30)),
            user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string()),
            random_user_agent: false,
            headers: HashMap::new(),
        }
    }
}

fn generate_random_user_agent() -> &'static str {
    const USER_AGENTS: &[&str] = &[
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0",
        "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    ];
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..USER_AGENTS.len());
    USER_AGENTS[index]
}

pub struct WebExtractor {
    url: String,
    html: Option<String>,
    activities: Activities,
    client: Option<Client>,
    client_config: ClientConfig,
    robots_checker: Option<RobotsChecker>,
    robots_enabled: bool,
}

impl WebExtractor {
    pub fn new(url: String) -> Self {
        Self {
            url,
            html: None,
            activities: Activities::default(),
            client: None,
            client_config: ClientConfig::default(),
            robots_checker: None,
            robots_enabled: false,
        }
    }
    
    pub fn new_with_html(url: String, html: String) -> Self {
        Self {
            url,
            html: Some(html),
            activities: Activities::default(),
            client: None,
            client_config: ClientConfig::default(),
            robots_checker: None,
            robots_enabled: false,
        }
    }
    
    pub fn configure_client<F>(&mut self, f: F) -> Result<(), ExtractionError>
    where
        F: FnOnce(&mut reqwest::ClientBuilder) -> Result<(), ExtractionError>,
    {
        // Invalidate existing client so it will be rebuilt with new config
        self.client = None;
        
        // Create a temporary builder to apply the callback
        let mut builder = self.build_client_builder()?;
        
        // Apply user callback
        f(&mut builder)?;
        
        // Build and store client
        self.client = Some(
            builder
                .build()
                .map_err(|e| ExtractionError::HttpError(format!("Failed to create HTTP client: {}", e)))?
        );
        
        Ok(())
    }
    
    /// Build a client builder with current configuration
    fn build_client_builder(&self) -> Result<ClientBuilder, ExtractionError> {
        let mut builder = Client::builder();
        
        // Set timeout
        if let Some(timeout) = self.client_config.timeout {
            builder = builder.timeout(timeout);
        }
        
        // Set user agent
        let user_agent = if self.client_config.random_user_agent {
            generate_random_user_agent()
        } else if let Some(ref ua) = self.client_config.user_agent {
            ua.as_str()
        } else {
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
        };
        builder = builder.user_agent(user_agent);
        
        // Add custom headers
        if !self.client_config.headers.is_empty() {
            let mut header_map = HeaderMap::new();
            for (key, value) in &self.client_config.headers {
                let header_name = key.parse::<reqwest::header::HeaderName>()
                    .map_err(|e| ExtractionError::HttpError(format!("Invalid header name '{}': {}", key, e)))?;
                let header_value = HeaderValue::from_str(value)
                    .map_err(|e| ExtractionError::HttpError(format!("Invalid header value for '{}': {}", key, e)))?;
                header_map.insert(header_name, header_value);
            }
            builder = builder.default_headers(header_map);
        }
        
        Ok(builder)
    }
    
    pub fn set_timeout(&mut self, timeout_secs: u64) {
        self.client_config.timeout = Some(Duration::from_secs(timeout_secs));
        self.client = None; // Invalidate existing client
    }
    
    pub fn set_user_agent(&mut self, user_agent: String) {
        self.client_config.user_agent = Some(user_agent);
        self.client_config.random_user_agent = false;
        self.client = None; // Invalidate existing client
    }
    
    pub fn set_random_user_agent(&mut self, enabled: bool) {
        self.client_config.random_user_agent = enabled;
        self.client = None; // Invalidate existing client
    }
    
    pub fn add_header(&mut self, name: String, value: String) {
        self.client_config.headers.insert(name, value);
        self.client = None; // Invalidate existing client
    }
    
    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.client_config.headers = headers;
        self.client = None; // Invalidate existing client
    }
    
    fn get_client(&mut self) -> Result<&Client, ExtractionError> {
        if self.client.is_none() {
            let builder = self.build_client_builder()?;
            self.client = Some(
                builder
                    .build()
                    .map_err(|e| ExtractionError::HttpError(format!("Failed to create HTTP client: {}", e)))?
            );
        }
        Ok(self.client.as_ref().unwrap())
    }

    pub fn extract_text(&mut self, language_detection: bool) {
        self.activities.extract_text.enabled = true;
        self.activities.extract_text.language_detection = language_detection;
    }

    pub fn extract_links(&mut self, fields: Vec<String>) {
        self.activities.extract_links = fields;
    }

    pub fn extract_socials(&mut self, fields: Vec<String>) {
        self.activities.extract_socials = fields;
    }

    pub fn extract_video(&mut self, fields: Vec<String>) {
        self.activities.extract_video = fields;
    }

    pub fn extract_product(&mut self, fields: Vec<String>) {
        self.activities.extract_product = fields;
    }

    pub fn extract_article(&mut self, fields: Vec<String>) {
        self.activities.extract_article = fields;
    }

    /// Enable robots.txt checking with in-memory cache
    pub fn enable_robots_check(&mut self) {
        let mut checker = RobotsChecker::new();
        checker.enable_memory_cache();
        self.robots_checker = Some(checker);
        self.robots_enabled = true;
    }

    /// Enable robots.txt checking with Redis cache
    pub fn enable_robots_check_with_redis(&mut self, redis_url: &str) -> Result<(), ExtractionError> {
        let mut checker = RobotsChecker::new();
        checker.enable_memory_cache();
        checker.enable_redis_cache(redis_url)?;
        self.robots_checker = Some(checker);
        self.robots_enabled = true;
        Ok(())
    }

    /// Set Redis TTL for robots.txt cache
    pub fn set_robots_redis_ttl(&mut self, ttl_secs: u64) -> Result<(), ExtractionError> {
        if let Some(ref mut checker) = self.robots_checker {
            checker.set_redis_ttl(ttl_secs);
            Ok(())
        } else {
            Err(ExtractionError::Other("Robots checker not enabled".to_string()))
        }
    }

    /// Set robots.txt content manually
    pub async fn set_robots_txt(&mut self, content: &str) -> Result<(), ExtractionError> {
        if let Some(ref checker) = self.robots_checker {
            checker.set_robots_txt(&self.url, content).await
        } else {
            Err(ExtractionError::Other("Robots checker not enabled".to_string()))
        }
    }

    /// Check if current URL is allowed by robots.txt
    pub async fn check_robots_allowed(&self) -> Result<bool, ExtractionError> {
        if let Some(ref checker) = self.robots_checker {
            let user_agent = if self.client_config.random_user_agent {
                generate_random_user_agent()
            } else if let Some(ref ua) = self.client_config.user_agent {
                ua.as_str()
            } else {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
            };
            checker.is_allowed(&self.url, user_agent).await
        } else {
            Ok(true) // If robots checking is not enabled, allow by default
        }
    }

    /// Remove robots.txt from Redis cache for current domain
    pub async fn remove_robots_from_redis(&self) -> Result<(), ExtractionError> {
        if let Some(ref checker) = self.robots_checker {
            use url::Url;
            let domain = Url::parse(&self.url)
                .ok()
                .and_then(|u| u.host_str().map(|s| s.to_string()))
                .unwrap_or_else(|| String::new());
            checker.remove_from_redis(&domain).await
        } else {
            Err(ExtractionError::Other("Robots checker not enabled".to_string()))
        }
    }

    /// Clear in-memory robots.txt cache
    pub async fn clear_robots_cache(&self) {
        if let Some(ref checker) = self.robots_checker {
            checker.clear_memory_cache().await;
        }
    }

    pub async fn run_async(&mut self) -> Result<ExtractionResult, ExtractionError> {
        // Check robots.txt if enabled
        if self.robots_enabled {
            let allowed = self.check_robots_allowed().await?;
            if !allowed {
                return Err(ExtractionError::Other(
                    format!("URL {} is disallowed by robots.txt", self.url)
                ));
            }
        }

        let mut result = ExtractionResult {
            url: self.url.clone(),
            text: None,
            language: None,
            language_confidence: None,
            links: None,
            socials: None,
            videos: None,
            product: None,
            article: None,
            content: None,
        };

        // Use provided HTML or download if needed
        let html_content = if self.activities.extract_text.enabled
            || !self.activities.extract_links.is_empty()
            || !self.activities.extract_socials.is_empty()
            || !self.activities.extract_video.is_empty()
            || !self.activities.extract_product.is_empty()
            || !self.activities.extract_article.is_empty()
            || self.activities.extract_text.language_detection
        {
            // Use provided HTML if available, otherwise download
            if let Some(ref provided_html) = self.html {
                Some(provided_html.clone())
            } else {
                let url = self.url.clone();
                let client = self.get_client()?;
                let response = client
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| ExtractionError::from(e))?;

                let html = response
                    .text()
                    .await
                    .map_err(|e| ExtractionError::HttpError(format!("Failed to read response: {}", e)))?;

                Some(html)
            }
        } else {
            None
        };

        // Parse HTML if we have content
        if let Some(html_content) = html_content {
            let document = Html::parse_document(&html_content);

            // Build DOM index once - traverse the tree once and reuse the index
            let dom_index = DomIndex::build(&document);

            // Extract text if requested or if language detection is needed
            let text_needed = self.activities.extract_text.enabled || self.activities.extract_text.language_detection;
            if text_needed {
                let extracted_text = extract_text_content(&document);
                
                // Store text if enabled
                if self.activities.extract_text.enabled {
                    result.text = Some(extracted_text.clone());
                }
                
                // Language detection if needed
                if self.activities.extract_text.language_detection {
                    if let Some(info) = detect(&extracted_text) {
                        result.language = Some(info.lang().code().to_string());
                        result.language_confidence = Some(info.confidence());
                    }
                }
            }

            // Extract links if requested (already grouped) - uses index
            if !self.activities.extract_links.is_empty() {
                let links = extract_links_with_index(&dom_index, &self.url, &self.activities.extract_links);
                result.links = Some(links);
            }

            // Extract socials if requested - uses index
            if !self.activities.extract_socials.is_empty() {
                let socials = extract_socials_with_index(&dom_index, &self.activities.extract_socials);
                result.socials = Some(socials);
            }

            // Extract videos if requested
            if !self.activities.extract_video.is_empty() {
                let videos = extract_video(&document, &self.activities.extract_video);
                result.videos = Some(videos);
            }

            // Extract product if requested
            if !self.activities.extract_product.is_empty() {
                let product = extract_products(&document, &self.activities.extract_product);
                result.product = Some(product);
            }

            // Extract article if requested - uses index
            if !self.activities.extract_article.is_empty() {
                let article = extract_article_with_index(&dom_index, &self.activities.extract_article);
                result.article = Some(article);
            }

            // Create content info
            let text_length = result.text.as_ref().map_or(0, |t| t.len());
            result.content = Some(ContentInfo {
                text: result.text.clone(),
                text_length,
            });
        } else {
            // Even if no HTML, create content info if text exists
            let text_length = result.text.as_ref().map_or(0, |t| t.len());
            result.content = Some(ContentInfo {
                text: result.text.clone(),
                text_length,
            });
        }

        Ok(result)
    }

    // Synchronous wrapper for backward compatibility
    pub fn run(&mut self) -> Result<ExtractionResult, ExtractionError> {
        // Create a runtime for blocking calls
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| ExtractionError::Other(format!("Failed to create runtime: {}", e)))?;
        rt.block_on(self.run_async())
    }
}

