mod helpers;
mod dates;

use std::collections::HashMap;
use crate::dom_index::DomIndex;

/// Returns a list of all available article metadata field names
pub fn get_all_article_fields() -> Vec<String> {
    vec![
        "title".to_string(),
        "author".to_string(),
        "description".to_string(),
        "publication_date".to_string(),
        "modified_date".to_string(),
        "article_section".to_string(),
        "article_tag".to_string(),
        "article_author".to_string(),
        "article_published_time".to_string(),
        "article_modified_time".to_string(),
        "article_expiration_time".to_string(),
        "categories".to_string(),
    ]
}

/// Normalize field name - converts aliases to full field names
fn normalize_field_name(field: &str) -> String {
    match field {
        // Short aliases
        "title" => "title".to_string(),
        "author" => "author".to_string(),
        "description" => "description".to_string(),
        "pub_date" => "publication_date".to_string(),
        "pub_date_time" => "article_published_time".to_string(),
        "modified_time" => "article_modified_time".to_string(),
        "expiration_time" => "article_expiration_time".to_string(),
        "section" => "article_section".to_string(),
        "tag" => "article_tag".to_string(),
        "tags" => "article_tag".to_string(),
        "category" => "categories".to_string(),
        // Full names pass through
        _ => field.to_string(),
    }
}

/// Extract article metadata from HTML document using DOM index
pub fn extract_article_with_index(dom_index: &DomIndex, article_fields: &[String]) -> HashMap<String, String> {
    use helpers::{extract_json_ld_property_from_index, extract_schema_property_from_index};
    use dates::extract_publication_dates_with_confidence;
    use scraper::Selector;
    use serde_json;
    
    let mut articles = HashMap::new();

    // Check if "all" is in the list
    let fields_to_extract = if article_fields.iter().any(|f| f == "all") {
        get_all_article_fields()
    } else {
        article_fields.iter().map(|f| normalize_field_name(f)).collect()
    };

    for field in &fields_to_extract {
        let value = match field.as_str() {
            "title" => {
                // Try Open Graph title first (from index)
                dom_index.get_meta_by_property("og:title")
                    .cloned()
                    // Try Twitter Card title
                    .or_else(|| dom_index.get_meta_by_name("twitter:title").cloned())
                    // Try JSON-LD (headline, name)
                    .or_else(|| extract_json_ld_property_from_index(dom_index, &["headline", "name"]))
                    // Try title tag
                    .or_else(|| dom_index.get_first_element_by_tag("title").cloned())
                    // Try h1 as fallback
                    .or_else(|| dom_index.get_first_element_by_tag("h1").cloned())
            },
            "author" => {
                dom_index.get_meta_by_property("article:author")
                    .cloned()
                    .or_else(|| dom_index.get_meta_by_name("author").cloned())
                    .or_else(|| dom_index.get_meta_by_property("og:article:author").cloned())
                    // Try rel="author" link
                    .or_else(|| {
                        if let Ok(selector) = Selector::parse("a[rel='author']") {
                            if let Some(link) = dom_index.document().select(&selector).next() {
                                let text = link.text().collect::<String>().trim().to_string();
                                if !text.is_empty() {
                                    Some(text)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    // Try schema.org author
                    .or_else(|| extract_schema_property_from_index(dom_index, "author"))
            },
            "description" => {
                dom_index.get_meta_by_property("og:description")
                    .cloned()
                    // Try Twitter Card description
                    .or_else(|| dom_index.get_meta_by_name("twitter:description").cloned())
                    // Try standard meta description
                    .or_else(|| dom_index.get_meta_by_name("description").cloned())
                    // Try schema.org description
                    .or_else(|| extract_schema_property_from_index(dom_index, "description"))
            },
            "publication_date" => {
                // For dates with confidence, we still need the full document
                let dates = extract_publication_dates_with_confidence(dom_index.document());
                if dates.is_empty() {
                    None
                } else {
                    serde_json::to_string(&dates).ok()
                }
            },
            "modified_date" => {
                dom_index.get_meta_by_property("article:modified_time")
                    .cloned()
                    .or_else(|| dom_index.get_meta_by_property("og:updated_time").cloned())
            },
            "article_section" => dom_index.get_meta_by_property("article:section").cloned(),
            "article_tag" => dom_index.get_meta_by_property("article:tag").cloned(),
            "article_author" => dom_index.get_meta_by_property("article:author").cloned(),
            "article_published_time" => dom_index.get_meta_by_property("article:published_time").cloned(),
            "article_modified_time" => dom_index.get_meta_by_property("article:modified_time").cloned(),
            "article_expiration_time" => dom_index.get_meta_by_property("article:expiration_time").cloned(),
            "categories" => {
                dom_index.get_meta_by_property("article:tag")
                    .cloned()
                    .or_else(|| dom_index.get_meta_by_property("article:section").cloned())
                    // Try JSON-LD (articleSection, keywords)
                    .or_else(|| extract_json_ld_property_from_index(dom_index, &["articleSection", "keywords"]))
                    // Try keywords meta tag
                    .or_else(|| dom_index.get_meta_by_name("keywords").cloned())
            },
            _ => None,
        };

        if let Some(v) = value {
            articles.insert(field.clone(), v);
        }
    }

    articles
}

