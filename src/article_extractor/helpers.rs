use scraper::Selector;
use serde_json;
use regex::Regex;
use crate::dom_index::DomIndex;

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

/// Extract JSON-LD property from indexed JSON-LD content
pub fn extract_json_ld_property_from_index(dom_index: &DomIndex, properties: &[&str]) -> Option<String> {
    for json_content in dom_index.get_json_ld_content() {
        // Try to parse as JSON
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_content) {
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
                if let Some(captures) = re.captures(json_content) {
                    if let Some(value) = captures.get(1) {
                        return Some(value.as_str().to_string());
                    }
                }
            }
        }
    }
    None
}

/// Extract schema.org property using index and fallback to document
pub fn extract_schema_property_from_index(dom_index: &DomIndex, property: &str) -> Option<String> {
    // Try JSON-LD first
    if let Some(value) = extract_json_ld_property_from_index(dom_index, &[property]) {
        return Some(value);
    }
    
    // Try microdata from index
    if let Some(first) = dom_index.get_first_schema_by_itemprop(property) {
        return Some(first.clone());
    }
    
    // Fallback to document traversal for microdata
    if let Ok(selector) = Selector::parse(&format!("[itemprop='{}']", property)) {
        if let Some(element) = dom_index.document().select(&selector).next() {
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

