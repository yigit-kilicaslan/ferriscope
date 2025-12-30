mod helpers;

use url::Url;
use crate::types::{LinkInfo, GroupedLinks, LinkSummary};
use crate::dom_index::DomIndex;
use std::collections::HashMap;

/// Extract links using pre-built DOM index (avoids re-traversing DOM)
/// 
/// # Arguments
/// * `dom_index` - Pre-built DOM index containing link data
/// * `base_url` - Base URL for resolving relative links and determining internal/external
/// * `filter_options` - Vec of filter options: "internal", "external", or "all" (empty vec means "all")
pub fn extract_links_with_index(dom_index: &DomIndex, base_url: &str, filter_options: &[String]) -> GroupedLinks {
    let base = Url::parse(base_url).ok();
    let mut all_links = Vec::new();

    // Use pre-indexed link data instead of traversing DOM again
    for (href, text) in dom_index.get_link_data() {
        // Only process links with non-empty text
        if text.trim().is_empty() {
            continue;
        }
        
        let absolute_url = if let Some(base) = &base {
            base.join(href).map(|u| u.to_string()).unwrap_or_else(|_| href.clone())
        } else {
            href.clone()
        };

        all_links.push(LinkInfo {
            url: absolute_url,
            text: text.clone(),
        });
    }

    // All links in all_links are already valid (non-empty text)
    let valid_links = all_links;

    let base_domain = helpers::extract_base_domain(base_url);

    let mut internal = Vec::new();
    let mut external = Vec::new();
    let mut by_domain: HashMap<String, Vec<LinkInfo>> = HashMap::new();

    for link in &valid_links {
        helpers::categorize_link(link, &base_domain, &mut internal, &mut external, &mut by_domain);
    }

    // Determine which links to include based on filter options
    let filter_config = helpers::parse_filter_options(filter_options);

    // Filter internal and external based on options
    let filtered_internal: Vec<LinkInfo> = if filter_config.wants_internal {
        internal
    } else {
        Vec::new()
    };

    let filtered_external: Vec<LinkInfo> = if filter_config.wants_external {
        external
    } else {
        Vec::new()
    };

    // Filter by_domain based on options
    let filtered_by_domain = helpers::filter_by_domain(by_domain, &base_domain, &filter_config);

    let total_count = filtered_internal.len() + filtered_external.len();
    let summary = LinkSummary {
        total: total_count,
        internal_count: filtered_internal.len(),
        external_count: filtered_external.len(),
        unique_domains: filtered_by_domain.len(),
    };

    GroupedLinks {
        internal: filtered_internal,
        external: filtered_external,
        by_domain: filtered_by_domain,
        summary,
    }
}
