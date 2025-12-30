use std::collections::HashMap;
use crate::dom_index::DomIndex;

/// Returns a list of all available social metadata field names
pub fn get_all_social_fields() -> Vec<String> {
    vec![
        "twitter_card".to_string(),
        "twitter_site".to_string(),
        "twitter_creator".to_string(),
        "twitter_title".to_string(),
        "twitter_description".to_string(),
        "twitter_image".to_string(),
        "og_url".to_string(),
        "og_type".to_string(),
        "og_title".to_string(),
        "og_description".to_string(),
        "og_image".to_string(),
        "og_image_width".to_string(),
        "og_image_height".to_string(),
        "og_image_alt".to_string(),
        "og_site_name".to_string(),
        "og_locale".to_string(),
    ]
}

/// Extract social metadata using pre-built DOM index (avoids re-traversing DOM)
pub fn extract_socials_with_index(dom_index: &DomIndex, social_fields: &[String]) -> HashMap<String, String> {
    let mut socials = HashMap::new();

    // Check if "all" is in the list
    let fields_to_extract = if social_fields.iter().any(|f| f == "all") {
        get_all_social_fields()
    } else {
        social_fields.to_vec()
    };

    for field in &fields_to_extract {
        let value = match field.as_str() {
            "twitter_card" => dom_index.get_meta_by_name("twitter:card").cloned(),
            "twitter_site" => dom_index.get_meta_by_name("twitter:site").cloned(),
            "twitter_creator" => dom_index.get_meta_by_name("twitter:creator").cloned(),
            "twitter_title" => dom_index.get_meta_by_name("twitter:title").cloned(),
            "twitter_description" => dom_index.get_meta_by_name("twitter:description").cloned(),
            "twitter_image" => dom_index.get_meta_by_name("twitter:image").cloned(),
            "og_url" => dom_index.get_meta_by_property("og:url").cloned(),
            "og_type" => dom_index.get_meta_by_property("og:type").cloned(),
            "og_title" => dom_index.get_meta_by_property("og:title").cloned(),
            "og_description" => dom_index.get_meta_by_property("og:description").cloned(),
            "og_image" => dom_index.get_meta_by_property("og:image").cloned(),
            "og_image_width" => dom_index.get_meta_by_property("og:image:width").cloned(),
            "og_image_height" => dom_index.get_meta_by_property("og:image:height").cloned(),
            "og_image_alt" => dom_index.get_meta_by_property("og:image:alt").cloned(),
            "og_site_name" => dom_index.get_meta_by_property("og:site_name").cloned(),
            "og_locale" => dom_index.get_meta_by_property("og:locale").cloned(),
            _ => None,
        };

        if let Some(v) = value {
            socials.insert(field.clone(), v);
        }
    }

    socials
}

