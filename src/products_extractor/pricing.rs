use scraper::{Html, Selector};
use super::helpers::{extract_meta_property, extract_json_ld_property, extract_schema_property};
use regex::Regex;

pub fn extract_product_price(document: &Html) -> Option<String> {
    // Try product:price:amount meta property
    if let Some(price) = extract_meta_property(document, "product:price:amount") {
        return Some(price);
    }

    // Try product:price meta property
    if let Some(price) = extract_meta_property(document, "product:price") {
        return Some(price);
    }

    // Try JSON-LD Product schema
    if let Some(price) = extract_json_ld_property(document, &["price", "offers.price", "offers.lowPrice"]) {
        return Some(price);
    }

    // Try schema.org Product
    if let Some(price) = extract_schema_property(document, "price") {
        return Some(price);
    }

    // Try to find price in common class names/ids
    let price_selectors = [
        ".price", ".product-price", ".price-current", ".current-price",
        "[itemprop='price']", "[data-price]", "#price"
    ];

    for selector_str in &price_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(price_attr) = element.value().attr("content") {
                    return Some(price_attr.to_string());
                }
                let text = element.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    // Try to extract numeric price from text
                    if let Some(price) = extract_price_from_text(&text) {
                        return Some(price);
                    }
                }
            }
        }
    }

    None
}

pub fn extract_product_currency(document: &Html) -> Option<String> {
    // Try product:price:currency meta property
    if let Some(currency) = extract_meta_property(document, "product:price:currency") {
        return Some(currency);
    }

    // Try JSON-LD Product schema
    if let Some(currency) = extract_json_ld_property(document, &["priceCurrency", "offers.priceCurrency"]) {
        return Some(currency);
    }

    // Try schema.org Product
    if let Some(currency) = extract_schema_property(document, "priceCurrency") {
        return Some(currency);
    }

    None
}

pub fn extract_product_availability(document: &Html) -> Option<String> {
    // Try product:availability meta property
    if let Some(availability) = extract_meta_property(document, "product:availability") {
        return Some(availability);
    }

    // Try JSON-LD Product schema
    if let Some(availability) = extract_json_ld_property(document, &["availability", "offers.availability"]) {
        return Some(availability);
    }

    // Try schema.org Product
    if let Some(availability) = extract_schema_property(document, "availability") {
        return Some(availability);
    }

    None
}

pub fn extract_product_original_price(document: &Html) -> Option<String> {
    // Try product:original_price meta property
    if let Some(price) = extract_meta_property(document, "product:original_price") {
        return Some(price);
    }

    // Try JSON-LD Product schema
    if let Some(price) = extract_json_ld_property(document, &["offers.highPrice", "originalPrice"]) {
        return Some(price);
    }

    // Try common class names for original/old price
    let price_selectors = [
        ".original-price", ".old-price", ".price-original", ".was-price",
        "[data-original-price]"
    ];

    for selector_str in &price_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(price_attr) = element.value().attr("content") {
                    return Some(price_attr.to_string());
                }
                let text = element.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    if let Some(price) = extract_price_from_text(&text) {
                        return Some(price);
                    }
                }
            }
        }
    }

    None
}

/// Extract price from text using regex (e.g., "$19.99", "€25,50", "£10.00")
fn extract_price_from_text(text: &str) -> Option<String> {
    // Match prices like $19.99, €25.50, £10.00, 19.99 USD
    let patterns = [
        r#"[£$€¥]\s*[\d,]+\.?\d*"#,  // Currency symbol before number
        r#"[\d,]+\.?\d*\s*[£$€¥]"#,  // Currency symbol after number
        r#"[\d,]+\.?\d*\s*(USD|EUR|GBP|JPY|CAD|AUD)"#,  // Currency code
    ];

    for pattern in &patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.find(text) {
                return Some(captures.as_str().trim().to_string());
            }
        }
    }

    None
}

