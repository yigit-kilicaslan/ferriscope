use scraper::{Html, Selector};
use std::collections::HashMap;

/// Index of DOM elements built from a single traversal
/// This allows reusing selected elements across multiple extractors
/// The index stores extracted data and element references tied to the document lifetime
pub struct DomIndex<'a> {
    /// All meta tags indexed by property attribute - stores content values
    pub meta_by_property: HashMap<String, Vec<String>>,
    /// All meta tags indexed by name attribute - stores content values
    pub meta_by_name: HashMap<String, Vec<String>>,
    /// Link data (href and text) extracted during traversal
    pub link_data: Vec<(String, String)>, // (href, text)
    /// JSON-LD script content
    pub json_ld_content: Vec<String>,
    /// Common elements by tag name - stores text content
    pub elements_by_tag: HashMap<String, Vec<String>>,
    /// Schema.org elements by itemprop - stores content or text
    pub schema_by_itemprop: HashMap<String, Vec<String>>,
    /// The original document (for cases where we need to traverse again)
    pub document: &'a Html,
}

impl<'a> DomIndex<'a> {
    /// Build an index by traversing the DOM once
    pub fn build(document: &'a Html) -> Self {
        let mut meta_by_property = HashMap::new();
        let mut meta_by_name = HashMap::new();
        let mut link_data = Vec::new();
        let mut json_ld_content = Vec::new();
        let mut elements_by_tag: HashMap<String, Vec<String>> = HashMap::new();
        let mut schema_by_itemprop = HashMap::new();

        // Single traversal: collect all meta tags
        if let Ok(meta_selector) = Selector::parse("meta") {
            for element in document.select(&meta_selector) {
                let content_opt = element.value().attr("content");
                
                // Index by property
                if let Some(property) = element.value().attr("property") {
                    if let Some(content) = content_opt {
                        meta_by_property
                            .entry(property.to_string())
                            .or_insert_with(Vec::new)
                            .push(content.to_string());
                    }
                }
                // Index by name
                if let Some(name) = element.value().attr("name") {
                    if let Some(content) = content_opt {
                        meta_by_name
                            .entry(name.to_string())
                            .or_insert_with(Vec::new)
                            .push(content.to_string());
                    }
                }
            }
        }

        // Single traversal: collect all links
        if let Ok(link_selector) = Selector::parse("a[href]") {
            for element in document.select(&link_selector) {
                if let Some(href) = element.value().attr("href") {
                    let text: String = element.text().collect();
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        link_data.push((href.to_string(), trimmed.to_string()));
                    }
                }
            }
        }

        // Single traversal: collect JSON-LD scripts
        if let Ok(script_selector) = Selector::parse("script[type='application/ld+json']") {
            for element in document.select(&script_selector) {
                if let Some(text) = element.text().next() {
                    json_ld_content.push(text.to_string());
                }
            }
        }

        // Single traversal: collect common elements by tag name
        let common_tags = ["title", "h1", "h2", "h3", "article", "main"];
        for tag in &common_tags {
            if let Ok(selector) = Selector::parse(tag) {
                let mut texts = Vec::new();
                for element in document.select(&selector) {
                    let text = element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        texts.push(text);
                    }
                }
                if !texts.is_empty() {
                    elements_by_tag.insert(tag.to_string(), texts);
                }
            }
        }

        // Single traversal: collect schema.org elements by itemprop
        if let Ok(schema_selector) = Selector::parse("[itemprop]") {
            for element in document.select(&schema_selector) {
                if let Some(itemprop) = element.value().attr("itemprop") {
                    // Try content attribute first, then text
                    let value = element.value().attr("content")
                        .map(|s| s.to_string())
                        .or_else(|| {
                            let text = element.text().collect::<String>().trim().to_string();
                            if !text.is_empty() {
                                Some(text)
                            } else {
                                None
                            }
                        });
                    
                    if let Some(v) = value {
                        schema_by_itemprop
                            .entry(itemprop.to_string())
                            .or_insert_with(Vec::new)
                            .push(v);
                    }
                }
            }
        }

        Self {
            meta_by_property,
            meta_by_name,
            link_data,
            json_ld_content,
            elements_by_tag,
            schema_by_itemprop,
            document,
        }
    }

    /// Get first meta tag content by property
    pub fn get_meta_by_property(&self, property: &str) -> Option<&String> {
        self.meta_by_property.get(property)?.first()
    }

    /// Get first meta tag content by name
    pub fn get_meta_by_name(&self, name: &str) -> Option<&String> {
        self.meta_by_name.get(name)?.first()
    }

    /// Get all link data
    pub fn get_link_data(&self) -> &[(String, String)] {
        &self.link_data
    }

    /// Get all JSON-LD script contents
    pub fn get_json_ld_content(&self) -> &[String] {
        &self.json_ld_content
    }

    /// Get first element text by tag name
    pub fn get_first_element_by_tag(&self, tag: &str) -> Option<&String> {
        self.elements_by_tag.get(tag)?.first()
    }

    /// Get first schema.org element by itemprop
    pub fn get_first_schema_by_itemprop(&self, itemprop: &str) -> Option<&String> {
        self.schema_by_itemprop.get(itemprop)?.first()
    }

    /// Get the original document for fallback
    pub fn document(&self) -> &'a Html {
        self.document
    }
}

