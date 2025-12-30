use scraper::{Html, Selector};

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

