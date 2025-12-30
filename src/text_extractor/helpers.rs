/// Check if an element is a boilerplate element (nav, header, footer, etc.)
pub fn is_boilerplate_element(element: &scraper::element_ref::ElementRef) -> bool {
    let tag_name = element.value().name();
    
    // Check common boilerplate tag names
    if matches!(tag_name, "nav" | "header" | "footer" | "aside" | "script" | "style" | "noscript") {
        return true;
    }
    
    // Check role attribute
    if let Some(role) = element.value().attr("role") {
        if matches!(role, "navigation" | "banner" | "contentinfo" | "complementary") {
            return true;
        }
    }
    
    // Check element's id
    if let Some(id) = element.value().attr("id") {
        let id_lower = id.to_lowercase();
        if id_lower.contains("nav") || id_lower.contains("header") || id_lower.contains("footer")
            || id_lower.contains("sidebar") || id_lower.contains("ad") || id_lower.contains("social")
            || id_lower.contains("comment") || id_lower.contains("breadcrumb") || id_lower.contains("cookie")
            || id_lower.contains("menu") || id_lower.contains("navigation") {
            return true;
        }
    }
    
    // Check element's classes
    if let Some(classes) = element.value().attr("class") {
        let classes_lower = classes.to_lowercase();
        if classes_lower.contains("nav") || classes_lower.contains("header") || classes_lower.contains("footer")
            || classes_lower.contains("sidebar") || classes_lower.contains("ad") || classes_lower.contains("social")
            || classes_lower.contains("comment") || classes_lower.contains("breadcrumb") || classes_lower.contains("cookie")
            || classes_lower.contains("menu") || classes_lower.contains("navigation") || classes_lower.contains("advertisement")
            || classes_lower.contains("newsletter") || classes_lower.contains("subscribe") {
            return true;
        }
    }
    
    false
}

/// Recursively extract text from non-boilerplate elements
pub fn extract_text_from_clean_elements(element: scraper::element_ref::ElementRef) -> String {
    let mut text_parts = Vec::new();
    
    // Recursively extract text from non-boilerplate elements
    for child in element.children() {
        if let Some(_elem) = child.value().as_element() {
            let elem_ref = scraper::ElementRef::wrap(child).unwrap();
            
            // Skip if this is a boilerplate element
            if is_boilerplate_element(&elem_ref) {
                continue;
            }
            
            // Recursively extract from children
            let child_text = extract_text_from_clean_elements(elem_ref);
            if !child_text.trim().is_empty() {
                text_parts.push(child_text);
            }
        } else if child.value().is_text() {
            // Direct text node - include it
            let text = child.value().as_text().unwrap().text.trim();
            if !text.is_empty() {
                text_parts.push(text.to_string());
            }
        }
    }
    
    text_parts.join(" ")
}

