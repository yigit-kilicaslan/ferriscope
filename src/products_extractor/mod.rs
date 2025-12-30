mod basic;
mod pricing;
mod reviews;
mod helpers;

use std::collections::HashMap;
use scraper::Html;

/// Returns a list of all available product metadata field names
pub fn get_all_product_fields() -> Vec<String> {
    vec![
        "product_title".to_string(),
        "product_description".to_string(),
        "product_brand".to_string(),
        "product_category".to_string(),
        "product_sku".to_string(),
        "product_mpn".to_string(),
        "product_image".to_string(),
        "product_price".to_string(),
        "product_currency".to_string(),
        "product_availability".to_string(),
        "product_original_price".to_string(),
        "product_rating".to_string(),
        "product_review_count".to_string(),
        "product_best_rating".to_string(),
        "product_worst_rating".to_string(),
    ]
}

/// Normalize field name - converts aliases to full field names
fn normalize_field_name(field: &str) -> String {
    match field {
        // Short aliases
        "title" => "product_title".to_string(),
        "description" => "product_description".to_string(),
        "price" => "product_price".to_string(),
        "brand" => "product_brand".to_string(),
        "category" => "product_category".to_string(),
        "sku" => "product_sku".to_string(),
        "mpn" => "product_mpn".to_string(),
        "image" => "product_image".to_string(),
        "currency" => "product_currency".to_string(),
        "availability" => "product_availability".to_string(),
        "original_price" => "product_original_price".to_string(),
        "rating" => "product_rating".to_string(),
        "review_count" => "product_review_count".to_string(),
        "best_rating" => "product_best_rating".to_string(),
        "worst_rating" => "product_worst_rating".to_string(),
        // Full names pass through
        _ => field.to_string(),
    }
}

/// Extract product metadata from HTML document
pub fn extract_products(document: &Html, product_fields: &[String]) -> HashMap<String, String> {
    let mut products = HashMap::new();

    // Check if "all" is in the list
    let fields_to_extract = if product_fields.iter().any(|f| f == "all") {
        get_all_product_fields()
    } else {
        product_fields.iter().map(|f| normalize_field_name(f)).collect()
    };

    for field in &fields_to_extract {
        let value = match field.as_str() {
            "product_title" => basic::extract_product_title(document),
            "product_description" => basic::extract_product_description(document),
            "product_brand" => basic::extract_product_brand(document),
            "product_category" => basic::extract_product_category(document),
            "product_sku" => basic::extract_product_sku(document),
            "product_mpn" => basic::extract_product_mpn(document),
            "product_image" => basic::extract_product_image(document),
            "product_price" => pricing::extract_product_price(document),
            "product_currency" => pricing::extract_product_currency(document),
            "product_availability" => pricing::extract_product_availability(document),
            "product_original_price" => pricing::extract_product_original_price(document),
            "product_rating" => reviews::extract_product_rating(document),
            "product_review_count" => reviews::extract_product_review_count(document),
            "product_best_rating" => reviews::extract_product_best_rating(document),
            "product_worst_rating" => reviews::extract_product_worst_rating(document),
            _ => None,
        };

        if let Some(v) = value {
            products.insert(field.clone(), v);
        }
    }

    products
}

