use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct TextExtraction {
    pub enabled: bool,
    pub language_detection: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Activities {
    pub extract_text: TextExtraction,
    pub extract_links: Vec<String>,
    pub extract_socials: Vec<String>,
    pub extract_video: Vec<String>,
    pub extract_product: Vec<String>,
    pub extract_article: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub url: String,
    pub text: Option<String>,
    pub language: Option<String>,
    pub language_confidence: Option<f64>,
    // Grouped data (extracted directly, no separate grouping step needed)
    pub links: Option<GroupedLinks>,
    pub socials: Option<std::collections::HashMap<String, String>>,
    pub videos: Option<std::collections::HashMap<String, String>>,
    pub product: Option<std::collections::HashMap<String, String>>,
    pub article: Option<std::collections::HashMap<String, String>>,
    pub content: Option<ContentInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub url: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateWithConfidence {
    pub date: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupedLinks {
    pub internal: Vec<LinkInfo>,
    pub external: Vec<LinkInfo>,
    pub by_domain: HashMap<String, Vec<LinkInfo>>,
    pub summary: LinkSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSummary {
    pub total: usize,
    pub internal_count: usize,
    pub external_count: usize,
    pub unique_domains: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentInfo {
    pub text: Option<String>,
    pub text_length: usize,
}

