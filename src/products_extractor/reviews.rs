use scraper::{Html, Selector};
use super::helpers::{extract_json_ld_property, extract_schema_property};

pub fn extract_product_rating(document: &Html) -> Option<String> {
    // Try JSON-LD Product schema
    if let Some(rating) = extract_json_ld_property(document, &["aggregateRating.ratingValue", "ratingValue"]) {
        return Some(rating);
    }

    // Try schema.org Product
    if let Some(rating) = extract_schema_property(document, "ratingValue") {
        return Some(rating);
    }

    // Try common class names for rating
    let rating_selectors = [
        "[itemprop='ratingValue']", ".rating", ".product-rating",
        "[data-rating]", ".star-rating"
    ];

    for selector_str in &rating_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(rating_attr) = element.value().attr("content") {
                    return Some(rating_attr.to_string());
                }
                let text = element.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }
    }

    None
}

pub fn extract_product_review_count(document: &Html) -> Option<String> {
    // Try JSON-LD Product schema
    if let Some(count) = extract_json_ld_property(document, &["aggregateRating.reviewCount", "reviewCount"]) {
        return Some(count);
    }

    // Try schema.org Product
    if let Some(count) = extract_schema_property(document, "reviewCount") {
        return Some(count);
    }

    // Try common class names for review count
    let count_selectors = [
        "[itemprop='reviewCount']", ".review-count", ".reviews-count",
        "[data-review-count]"
    ];

    for selector_str in &count_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(count_attr) = element.value().attr("content") {
                    return Some(count_attr.to_string());
                }
                let text = element.text().collect::<String>().trim().to_string();
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }
    }

    None
}

pub fn extract_product_best_rating(document: &Html) -> Option<String> {
    // Try JSON-LD Product schema
    if let Some(rating) = extract_json_ld_property(document, &["aggregateRating.bestRating", "bestRating"]) {
        return Some(rating);
    }

    // Try schema.org Product
    if let Some(rating) = extract_schema_property(document, "bestRating") {
        return Some(rating);
    }

    None
}

pub fn extract_product_worst_rating(document: &Html) -> Option<String> {
    // Try JSON-LD Product schema
    if let Some(rating) = extract_json_ld_property(document, &["aggregateRating.worstRating", "worstRating"]) {
        return Some(rating);
    }

    // Try schema.org Product
    if let Some(rating) = extract_schema_property(document, "worstRating") {
        return Some(rating);
    }

    None
}

