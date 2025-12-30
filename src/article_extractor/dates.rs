use scraper::{Html, Selector};
use std::collections::HashSet;
use regex::Regex;
use crate::types::DateWithConfidence;

/// Extract publication dates with confidence scores
pub fn extract_publication_dates_with_confidence(document: &Html) -> Vec<DateWithConfidence> {
    use std::collections::HashMap as Map;
    
    // Track where each date appears: meta, json_ld, body
    let mut date_sources: Map<String, (bool, bool, bool)> = Map::new();
    
    // Extract dates from meta tags
    let meta_date_fields = vec![
        "article:published_time",
        "og:published_time",
        "pubdate",
        "date",
        "publication_date",
    ];
    
    for field in &meta_date_fields {
        if field.starts_with("article:") || field.starts_with("og:") {
            if let Ok(selector) = Selector::parse(&format!("meta[property='{}']", field)) {
                if let Some(meta) = document.select(&selector).next() {
                    if let Some(date) = meta.value().attr("content") {
                        let entry = date_sources.entry(date.to_string()).or_insert((false, false, false));
                        entry.0 = true; // meta tag
                    }
                }
            }
        } else {
            if let Ok(selector) = Selector::parse(&format!("meta[name='{}']", field)) {
                if let Some(meta) = document.select(&selector).next() {
                    if let Some(date) = meta.value().attr("content") {
                        let entry = date_sources.entry(date.to_string()).or_insert((false, false, false));
                        entry.0 = true; // meta tag
                    }
                }
            }
        }
    }
    
    // Extract dates from time elements
    if let Ok(selector) = Selector::parse("time[datetime]") {
        for time in document.select(&selector) {
            if let Some(datetime) = time.value().attr("datetime") {
                let entry = date_sources.entry(datetime.to_string()).or_insert((false, false, false));
                entry.0 = true; // meta tag (time element is structured metadata)
            }
        }
    }
    
    // Extract dates from JSON-LD
    let json_ld_dates = extract_all_json_ld_dates(document);
    for date in json_ld_dates {
        let entry = date_sources.entry(date).or_insert((false, false, false));
        entry.1 = true; // json-ld
    }
    
    // Extract dates from page body
    let body_dates = extract_dates_from_body(document);
    for date in body_dates {
        let entry = date_sources.entry(date).or_insert((false, false, false));
        entry.2 = true; // body
    }
    
    // Calculate confidence scores
    let total_dates = date_sources.len();
    
    // Count how many dates come from body only (for more aggressive penalty)
    let mut body_only_count = 0;
    
    for (_, (in_meta, in_json_ld, in_body)) in &date_sources {
        if *in_body && !*in_meta && !*in_json_ld {
            body_only_count += 1;
        }
    }
    
    let mut dates_with_confidence = Vec::new();
    
    for (date, (in_meta, in_json_ld, in_body)) in date_sources {
        let mut confidence = 0.0;
        
        // If date appears in all three sources, confidence = 1.0
        if in_meta && in_json_ld && in_body {
            confidence = 1.0;
        } else if in_meta && in_json_ld {
            // Meta + JSON-LD = high confidence
            confidence = 0.8;
        } else if in_meta || in_json_ld {
            // Only meta or JSON-LD = medium confidence
            confidence = 0.5;
        } else if in_body {
            // Only in body = low confidence
            confidence = 0.1;
        }
        
        // If there are many dates, reduce confidence for all
        // More dates = lower confidence (since it's ambiguous)
        if total_dates > 1 {
            // More aggressive reduction when there are many dates
            // If there are many body-only dates, they should have even lower confidence
            let reduction_factor = if in_body && !in_meta && !in_json_ld && body_only_count > 1 {
                // Body-only dates with many similar dates get heavily penalized
                1.0 / (1.0 + (body_only_count as f64 - 1.0) * 0.3)
            } else {
                // General reduction for other cases
                1.0 / (1.0 + (total_dates as f64 - 1.0) * 0.15)
            };
            confidence *= reduction_factor;
        }
        
        dates_with_confidence.push(DateWithConfidence {
            date,
            confidence,
        });
    }
    
    // Sort by confidence (highest first)
    dates_with_confidence.sort_by(|a, b| {
        b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    dates_with_confidence
}

/// Extract all dates from JSON-LD scripts
fn extract_all_json_ld_dates(document: &Html) -> Vec<String> {
    let mut dates = Vec::new();
    
    if let Ok(selector) = Selector::parse("script[type='application/ld+json']") {
        for script in document.select(&selector) {
            if let Some(text) = script.text().next() {
                // Try to extract datePublished
                let escaped_property = regex::escape("datePublished");
                let pattern = format!(r#""{}"\s*:\s*"([^"]+)""#, escaped_property);
                if let Ok(re) = Regex::new(&pattern) {
                    for captures in re.captures_iter(text) {
                        if let Some(value) = captures.get(1) {
                            dates.push(value.as_str().to_string());
                        }
                    }
                }
                
                // Also try to find any ISO 8601 dates in the JSON
                // This is a simple regex for ISO 8601 dates
                let iso_date_pattern = r#"\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})?)?"#;
                if let Ok(re) = Regex::new(iso_date_pattern) {
                    for captures in re.captures_iter(text) {
                        if let Some(date_match) = captures.get(0) {
                            dates.push(date_match.as_str().to_string());
                        }
                    }
                }
            }
        }
    }
    
    dates
}

/// Extract dates from the page body using regex patterns
fn extract_dates_from_body(document: &Html) -> Vec<String> {
    let mut dates = Vec::new();
    
    // Get all text content from the document body
    let body_selector = Selector::parse("body").unwrap_or_else(|_| {
        Selector::parse("html").unwrap()
    });
    
    let text = if let Some(body) = document.select(&body_selector).next() {
        body.text().collect::<Vec<_>>().join(" ")
    } else {
        document.root_element().text().collect::<Vec<_>>().join(" ")
    };
    
    // Common date patterns
    // ISO 8601: YYYY-MM-DD or YYYY-MM-DDTHH:MM:SS
    let iso_pattern = r#"\b\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})?)?\b"#;
    if let Ok(re) = Regex::new(iso_pattern) {
        for captures in re.captures_iter(&text) {
            if let Some(date_match) = captures.get(0) {
                dates.push(date_match.as_str().to_string());
            }
        }
    }
    
    // Common formats: MM/DD/YYYY, DD/MM/YYYY, YYYY/MM/DD
    let slash_pattern = r#"\b\d{1,2}/\d{1,2}/\d{4}\b"#;
    if let Ok(re) = Regex::new(slash_pattern) {
        for captures in re.captures_iter(&text) {
            if let Some(date_match) = captures.get(0) {
                dates.push(date_match.as_str().to_string());
            }
        }
    }
    
    // Month name formats: "January 1, 2024", "Jan 1, 2024", "1 January 2024"
    let month_pattern = r#"\b(January|February|March|April|May|June|July|August|September|October|November|December|Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+\d{1,2},?\s+\d{4}\b"#;
    if let Ok(re) = Regex::new(month_pattern) {
        for captures in re.captures_iter(&text) {
            if let Some(date_match) = captures.get(0) {
                dates.push(date_match.as_str().to_string());
            }
        }
    }
    
    // Remove duplicates
    let mut unique_dates: HashSet<String> = HashSet::new();
    dates.retain(|d| unique_dates.insert(d.clone()));
    
    dates
}

