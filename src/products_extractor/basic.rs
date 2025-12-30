use scraper::{Html, Selector};
use super::helpers::{extract_meta_property, extract_meta_name, extract_json_ld_property, extract_schema_property};

pub fn extract_product_title(document: &Html) -> Option<String> {
    // Try product:title meta property
    if let Some(title) = extract_meta_property(document, "product:title") {
        return Some(title);
    }

    // Try og:title (often used for products)
    if let Some(title) = extract_meta_property(document, "og:title") {
        return Some(title);
    }

    // Try JSON-LD Product schema
    if let Some(title) = extract_json_ld_property(document, &["name", "title"]) {
        return Some(title);
    }

    // Try schema.org Product
    if let Some(title) = extract_schema_property(document, "name") {
        return Some(title);
    }

    // Try h1 as fallback
    if let Ok(selector) = Selector::parse("h1") {
        if let Some(h1) = document.select(&selector).next() {
            let text = h1.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                return Some(text);
            }
        }
    }

    None
}

pub fn extract_product_description(document: &Html) -> Option<String> {
    // Try product:description meta property
    if let Some(desc) = extract_meta_property(document, "product:description") {
        return Some(desc);
    }

    // Try og:description
    if let Some(desc) = extract_meta_property(document, "og:description") {
        return Some(desc);
    }

    // Try JSON-LD Product schema
    if let Some(desc) = extract_json_ld_property(document, &["description"]) {
        return Some(desc);
    }

    // Try schema.org Product
    if let Some(desc) = extract_schema_property(document, "description") {
        return Some(desc);
    }

    // Try standard meta description
    if let Some(desc) = extract_meta_name(document, "description") {
        return Some(desc);
    }

    None
}

pub fn extract_product_brand(document: &Html) -> Option<String> {
    // Try product:brand meta property
    if let Some(brand) = extract_meta_property(document, "product:brand") {
        return Some(brand);
    }

    // Try JSON-LD Product schema
    if let Some(brand) = extract_json_ld_property(document, &["brand", "brand.name", "manufacturer.name"]) {
        return Some(brand);
    }

    // Try schema.org Product
    if let Some(brand) = extract_schema_property(document, "brand") {
        return Some(brand);
    }

    None
}

pub fn extract_product_category(document: &Html) -> Option<String> {
    // Try product:category meta property
    if let Some(category) = extract_meta_property(document, "product:category") {
        return Some(category);
    }

    // Try JSON-LD Product schema
    if let Some(category) = extract_json_ld_property(document, &["category", "productCategory"]) {
        return Some(category);
    }

    // Try schema.org Product
    if let Some(category) = extract_schema_property(document, "category") {
        return Some(category);
    }

    None
}

pub fn extract_product_sku(document: &Html) -> Option<String> {
    // Try product:sku meta property
    if let Some(sku) = extract_meta_property(document, "product:sku") {
        return Some(sku);
    }

    // Try JSON-LD Product schema
    if let Some(sku) = extract_json_ld_property(document, &["sku", "productID"]) {
        return Some(sku);
    }

    // Try schema.org Product
    if let Some(sku) = extract_schema_property(document, "sku") {
        return Some(sku);
    }

    None
}

pub fn extract_product_mpn(document: &Html) -> Option<String> {
    // Try product:mpn meta property
    if let Some(mpn) = extract_meta_property(document, "product:mpn") {
        return Some(mpn);
    }

    // Try JSON-LD Product schema
    if let Some(mpn) = extract_json_ld_property(document, &["mpn"]) {
        return Some(mpn);
    }

    // Try schema.org Product
    if let Some(mpn) = extract_schema_property(document, "mpn") {
        return Some(mpn);
    }

    None
}

pub fn extract_product_image(document: &Html) -> Option<String> {
    // Try product:image meta property
    if let Some(image) = extract_meta_property(document, "product:image") {
        return Some(image);
    }

    // Try og:image
    if let Some(image) = extract_meta_property(document, "og:image") {
        return Some(image);
    }

    // Try JSON-LD Product schema
    if let Some(image) = extract_json_ld_property(document, &["image", "image.url"]) {
        return Some(image);
    }

    // Try schema.org Product
    if let Some(image) = extract_schema_property(document, "image") {
        return Some(image);
    }

    None
}

