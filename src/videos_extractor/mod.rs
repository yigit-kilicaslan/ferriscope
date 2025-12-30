mod video;
mod book;
mod helpers;

use std::collections::HashMap;
use scraper::Html;

/// Returns a list of all available video/book metadata field names
pub fn get_all_video_fields() -> Vec<String> {
    vec![
        "video_duration".to_string(),
        "video_release_date".to_string(),
        "video_tag".to_string(),
        "video_actor".to_string(),
        "video_director".to_string(),
        "video_writer".to_string(),
        "video_series".to_string(),
        "book_author".to_string(),
        "book_isbn".to_string(),
        "book_release_date".to_string(),
        "book_tag".to_string(),
    ]
}

/// Extract video/book metadata from HTML document
pub fn extract_video(document: &Html, video_fields: &[String]) -> HashMap<String, String> {
    let mut videos = HashMap::new();

    // Check if "all" is in the list
    let fields_to_extract = if video_fields.iter().any(|f| f == "all") {
        get_all_video_fields()
    } else {
        video_fields.to_vec()
    };

    for field in &fields_to_extract {
        let value = match field.as_str() {
            "video_duration" => video::extract_video_duration(document),
            "video_release_date" => video::extract_video_release_date(document),
            "video_tag" => video::extract_video_tag(document),
            "video_actor" => video::extract_video_actor(document),
            "video_director" => video::extract_video_director(document),
            "video_writer" => video::extract_video_writer(document),
            "video_series" => video::extract_video_series(document),
            "book_author" => book::extract_book_author(document),
            "book_isbn" => book::extract_book_isbn(document),
            "book_release_date" => book::extract_book_release_date(document),
            "book_tag" => book::extract_book_tag(document),
            _ => None,
        };

        if let Some(v) = value {
            videos.insert(field.clone(), v);
        }
    }

    videos
}

