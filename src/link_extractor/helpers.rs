use url::Url;
use crate::types::LinkInfo;
use std::collections::HashMap;

pub struct FilterConfig {
    pub wants_all: bool,
    pub wants_internal: bool,
    pub wants_external: bool,
}

/// Extract base domain from URL
pub fn extract_base_domain(base_url: &str) -> String {
    Url::parse(base_url)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
        .unwrap_or_else(|| String::new())
}

/// Parse filter options into a configuration struct
pub fn parse_filter_options(filter_options: &[String]) -> FilterConfig {
    let wants_all = filter_options.is_empty() || filter_options.iter().any(|opt| opt == "all");
    let wants_internal = wants_all || filter_options.iter().any(|opt| opt == "internal");
    let wants_external = wants_all || filter_options.iter().any(|opt| opt == "external");

    FilterConfig {
        wants_all,
        wants_internal,
        wants_external,
    }
}

/// Categorize a link as internal or external and add to appropriate collections
pub fn categorize_link(
    link: &LinkInfo,
    base_domain: &str,
    internal: &mut Vec<LinkInfo>,
    external: &mut Vec<LinkInfo>,
    by_domain: &mut HashMap<String, Vec<LinkInfo>>,
) {
    let link_clone = link.clone();
    
    if let Ok(parsed_url) = Url::parse(&link.url) {
        if let Some(link_domain) = parsed_url.host_str() {
            let domain_str = link_domain.to_string();
            
            // Group by domain
            by_domain.entry(domain_str.clone())
                .or_insert_with(Vec::new)
                .push(link_clone.clone());

            // Categorize as internal/external
            if link_domain == base_domain || link_domain.is_empty() {
                internal.push(link_clone);
            } else {
                external.push(link_clone);
            }
        } else {
            // If no host, add to external
            external.push(link_clone);
        }
    } else {
        // If parsing fails, add to external
        external.push(link_clone);
    }
}

/// Filter links by domain based on filter configuration
pub fn filter_by_domain(
    by_domain: HashMap<String, Vec<LinkInfo>>,
    base_domain: &str,
    filter_config: &FilterConfig,
) -> HashMap<String, Vec<LinkInfo>> {
    if filter_config.wants_all {
        by_domain
    } else {
        let mut filtered: HashMap<String, Vec<LinkInfo>> = HashMap::new();
        for (domain, links) in by_domain {
            let is_internal = domain == base_domain || domain.is_empty();
            if (is_internal && filter_config.wants_internal) || (!is_internal && filter_config.wants_external) {
                filtered.insert(domain, links);
            }
        }
        filtered
    }
}

