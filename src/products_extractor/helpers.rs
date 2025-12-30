use scraper::{Html, Selector};
use regex::Regex;
use serde_json;

/// Extract a property value from a meta tag with property attribute
pub fn extract_meta_property(document: &Html, property: &str) -> Option<String> {
    let selector = format!("meta[property='{}']", property);
    if let Ok(sel) = Selector::parse(&selector) {
        if let Some(meta) = document.select(&sel).next() {
            return meta.value().attr("content").map(|s| s.to_string());
        }
    }
    None
}

/// Extract a property value from a meta tag with name attribute
pub fn extract_meta_name(document: &Html, name: &str) -> Option<String> {
    let selector = format!("meta[name='{}']", name);
    if let Ok(sel) = Selector::parse(&selector) {
        if let Some(meta) = document.select(&sel).next() {
            return meta.value().attr("content").map(|s| s.to_string());
        }
    }
    None
}

/// Recursively extract a value from a JSON object, handling nested paths like "publisher.name"
pub fn extract_value_from_object(obj: &serde_json::Map<String, serde_json::Value>, path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current: &serde_json::Value = &serde_json::Value::Object(obj.clone());
    
    for part in parts {
        if let Some(map) = current.as_object() {
            if let Some(value) = map.get(part) {
                current = value;
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    
    // Extract string value, handling arrays
    match current {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Array(arr) => {
            // Return first string value from array
            for item in arr {
                if let Some(s) = item.as_str() {
                    return Some(s.to_string());
                }
            }
            None
        }
        serde_json::Value::Object(nested_obj) => {
            // For objects, try to get "name" or "@id" or "url"
            if let Some(name) = nested_obj.get("name").and_then(|v| v.as_str()) {
                return Some(name.to_string());
            }
            if let Some(id) = nested_obj.get("@id").and_then(|v| v.as_str()) {
                return Some(id.to_string());
            }
            if let Some(url) = nested_obj.get("url").and_then(|v| v.as_str()) {
                return Some(url.to_string());
            }
            None
        }
        _ => None,
    }
}

/// Extract a property value from JSON-LD, handling nested objects and arrays
pub fn extract_json_ld_property(document: &Html, properties: &[&str]) -> Option<String> {
    if let Ok(selector) = Selector::parse("script[type='application/ld+json']") {
        for script in document.select(&selector) {
            if let Some(text) = script.text().next() {
                // Try to parse as JSON
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(text) {
                    // Handle both single objects and arrays of objects
                    let objects = match json_value {
                        serde_json::Value::Object(obj) => vec![obj],
                        serde_json::Value::Array(arr) => {
                            arr.into_iter()
                                .filter_map(|v| v.as_object().cloned())
                                .collect()
                        }
                        _ => vec![],
                    };
                    
                    for obj in objects {
                        for property in properties {
                            if let Some(value) = extract_value_from_object(&obj, property) {
                                return Some(value);
                            }
                        }
                    }
                }
                
                // Fallback to regex for malformed JSON
                for property in properties {
                    let escaped_property = regex::escape(property);
                    let pattern = format!(r#""{}"\s*:\s*"([^"]+)""#, escaped_property);
                    if let Ok(re) = Regex::new(&pattern) {
                        if let Some(captures) = re.captures(text) {
                            if let Some(value) = captures.get(1) {
                                return Some(value.as_str().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Extract a property value from schema.org microdata or JSON-LD
pub fn extract_schema_property(document: &Html, property: &str) -> Option<String> {
    // Try JSON-LD with the property name
    if let Some(value) = extract_json_ld_property(document, &[property]) {
        return Some(value);
    }
    
    // Try microdata
    if let Ok(selector) = Selector::parse(&format!("[itemprop='{}']", property)) {
        if let Some(element) = document.select(&selector).next() {
            if let Some(content) = element.value().attr("content") {
                return Some(content.to_string());
            }
            let text = element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    
    None
}

