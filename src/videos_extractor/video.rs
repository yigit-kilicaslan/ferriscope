use scraper::Html;
use super::helpers::extract_meta_property;

pub fn extract_video_duration(document: &Html) -> Option<String> {
    extract_meta_property(document, "video:duration")
}

pub fn extract_video_release_date(document: &Html) -> Option<String> {
    extract_meta_property(document, "video:release_date")
}

pub fn extract_video_tag(document: &Html) -> Option<String> {
    extract_meta_property(document, "video:tag")
}

pub fn extract_video_actor(document: &Html) -> Option<String> {
    extract_meta_property(document, "video:actor")
}

pub fn extract_video_director(document: &Html) -> Option<String> {
    extract_meta_property(document, "video:director")
}

pub fn extract_video_writer(document: &Html) -> Option<String> {
    extract_meta_property(document, "video:writer")
}

pub fn extract_video_series(document: &Html) -> Option<String> {
    extract_meta_property(document, "video:series")
}

