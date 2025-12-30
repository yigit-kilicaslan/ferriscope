mod helpers;

use scraper::{Html, Selector};

/// Extract text content from HTML document, filtering out boilerplate elements
pub fn extract_text_content(document: &Html) -> String {
    // First, try to find main content containers (these are usually the main article content)
    let main_content_selectors = [
        Selector::parse("article").ok(),
        Selector::parse("main").ok(),
        Selector::parse("[role='main']").ok(),
        Selector::parse(".main-content").ok(),
        Selector::parse(".content").ok(),
        Selector::parse("#main-content").ok(),
        Selector::parse("#content").ok(),
    ];
    
    // Try main content selectors first
    for selector_opt in main_content_selectors.iter() {
        if let Some(selector) = selector_opt {
            if let Some(element) = document.select(selector).next() {
                // Still filter boilerplate from main content (e.g., ads within articles)
                let text = helpers::extract_text_from_clean_elements(element);
                if !text.trim().is_empty() && text.len() > 50 {
                    // Only use if we got substantial content
                    return text.split_whitespace().collect::<Vec<_>>().join(" ");
                }
            }
        }
    }
    
    // Fallback to body/html with boilerplate removal
    let body_selector = Selector::parse("body").unwrap_or_else(|_| {
        Selector::parse("html").unwrap()
    });
    
    if let Some(body) = document.select(&body_selector).next() {
        // Extract text while excluding boilerplate elements
        let text = helpers::extract_text_from_clean_elements(body);
        
        // Clean up whitespace
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        document.root_element().text().collect::<Vec<_>>().join(" ")
    }
}
