use scraper::Html;
use super::helpers::extract_meta_property;

pub fn extract_book_author(document: &Html) -> Option<String> {
    extract_meta_property(document, "book:author")
}

pub fn extract_book_isbn(document: &Html) -> Option<String> {
    extract_meta_property(document, "book:isbn")
}

pub fn extract_book_release_date(document: &Html) -> Option<String> {
    extract_meta_property(document, "book:release_date")
}

pub fn extract_book_tag(document: &Html) -> Option<String> {
    extract_meta_property(document, "book:tag")
}

