#![allow(non_local_definitions)]

mod error;
mod types;
mod extractor;
mod text_extractor;
mod link_extractor;
mod socials_extractor;
mod videos_extractor;
mod products_extractor;
mod article_extractor;
mod dom_index;
mod robots;

pub use error::ExtractionError;
pub use types::{Activities, ExtractionResult, LinkInfo, GroupedLinks, ContentInfo, TextExtraction};
pub use extractor::WebExtractor;

use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;

/// Helper function to convert a LinkInfo to a Python dictionary
fn link_info_to_dict(py: Python, link: &LinkInfo) -> PyObject {
    let link_dict = PyDict::new(py);
    link_dict.set_item("url", &link.url).unwrap();
    link_dict.set_item("text", &link.text).unwrap();
    link_dict.into()
}

/// Helper function to convert a list of LinkInfo to a Python list
fn link_list_to_pylist(py: Python, links: &[LinkInfo]) -> PyObject {
    let list = PyList::empty(py);
    for link in links {
        list.append(link_info_to_dict(py, link)).unwrap();
    }
    list.into()
}

/// Helper function to convert GroupedLinks to a Python dictionary
fn grouped_links_to_dict(py: Python, gl: &GroupedLinks) -> PyObject {
    let dict = PyDict::new(py);
    
    dict.set_item("internal", link_list_to_pylist(py, &gl.internal)).unwrap();
    dict.set_item("external", link_list_to_pylist(py, &gl.external)).unwrap();
    
    // By domain
    let by_domain_dict = PyDict::new(py);
    for (domain, links) in &gl.by_domain {
        by_domain_dict.set_item(domain, link_list_to_pylist(py, links)).unwrap();
    }
    dict.set_item("by_domain", by_domain_dict).unwrap();
    
    // Summary
    let summary_dict = PyDict::new(py);
    summary_dict.set_item("total", gl.summary.total).unwrap();
    summary_dict.set_item("internal_count", gl.summary.internal_count).unwrap();
    summary_dict.set_item("external_count", gl.summary.external_count).unwrap();
    summary_dict.set_item("unique_domains", gl.summary.unique_domains).unwrap();
    dict.set_item("summary", summary_dict).unwrap();
    
    dict.into()
}

/// Helper function to convert a HashMap to a Python dictionary
fn hashmap_to_dict(py: Python, map: &HashMap<String, String>) -> PyObject {
    let dict = PyDict::new(py);
    for (k, v) in map {
        dict.set_item(k, v).unwrap();
    }
    dict.into()
}

// Python bindings
#[pymodule]
fn _ferriscope_native(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWebExtractor>()?;
    m.add_class::<PyExtractionResult>()?;
    m.add_class::<PyLinkInfo>()?;
    Ok(())
}

#[pyclass]
pub struct PyWebExtractor {
    extractor: WebExtractor,
}

#[pymethods]
impl PyWebExtractor {
    #[new]
    #[pyo3(signature = (url, html = None))]
    fn new(url: String, html: Option<String>) -> Self {
        if let Some(html_content) = html {
            PyWebExtractor {
                extractor: WebExtractor::new_with_html(url, html_content),
            }
        } else {
            PyWebExtractor {
                extractor: WebExtractor::new(url),
            }
        }
    }

    fn extract_text(&mut self, language_detection: bool) {
        self.extractor.extract_text(language_detection);
    }

    #[pyo3(signature = (fields = None))]
    fn extract_links(&mut self, fields: Option<Vec<String>>) {
        let fields = fields.unwrap_or_else(|| vec!["all".to_string()]);
        self.extractor.extract_links(fields);
    }

    #[pyo3(signature = (fields = None))]
    fn extract_socials(&mut self, fields: Option<Vec<String>>) {
        let fields = fields.unwrap_or_else(|| vec!["all".to_string()]);
        self.extractor.extract_socials(fields);
    }

    #[pyo3(signature = (fields = None))]
    fn extract_video(&mut self, fields: Option<Vec<String>>) {
        let fields = fields.unwrap_or_else(|| vec!["all".to_string()]);
        self.extractor.extract_video(fields);
    }

    #[pyo3(signature = (fields = None))]
    fn extract_product(&mut self, fields: Option<Vec<String>>) {
        let fields = fields.unwrap_or_else(|| vec!["all".to_string()]);
        self.extractor.extract_product(fields);
    }

    #[pyo3(signature = (fields = None))]
    fn extract_article(&mut self, fields: Option<Vec<String>>) {
        let fields = fields.unwrap_or_else(|| vec!["all".to_string()]);
        self.extractor.extract_article(fields);
    }

    fn set_timeout(&mut self, timeout_secs: u64) {
        self.extractor.set_timeout(timeout_secs);
    }

    fn set_user_agent(&mut self, user_agent: String) {
        self.extractor.set_user_agent(user_agent);
    }

    fn set_random_user_agent(&mut self, enabled: bool) {
        self.extractor.set_random_user_agent(enabled);
    }

    fn add_header(&mut self, name: String, value: String) {
        self.extractor.add_header(name, value);
    }

    fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.extractor.set_headers(headers);
    }

    fn enable_robots_check(&mut self) {
        self.extractor.enable_robots_check();
    }

    fn enable_robots_check_with_redis(&mut self, redis_url: String) -> PyResult<()> {
        self.extractor.enable_robots_check_with_redis(&redis_url)
            .map_err(|e| PyErr::from(e))
    }

    fn set_robots_redis_ttl(&mut self, ttl_secs: u64) -> PyResult<()> {
        self.extractor.set_robots_redis_ttl(ttl_secs)
            .map_err(|e| PyErr::from(e))
    }

    fn set_robots_txt(&mut self, content: String) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        rt.block_on(self.extractor.set_robots_txt(&content))
            .map_err(|e| PyErr::from(e))
    }

    fn check_robots_allowed(&self) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        rt.block_on(self.extractor.check_robots_allowed())
            .map_err(|e| PyErr::from(e))
    }

    fn remove_robots_from_redis(&self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))?;
        rt.block_on(self.extractor.remove_robots_from_redis())
            .map_err(|e| PyErr::from(e))
    }

    fn clear_robots_cache(&self) {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to create runtime: {}", e)))
            .ok();
        if let Some(rt) = rt {
            rt.block_on(self.extractor.clear_robots_cache());
        }
    }

    fn run(&mut self) -> PyResult<PyExtractionResult> {
        match self.extractor.run() {
            Ok(result) => Ok(PyExtractionResult { result }),
            Err(e) => Err(PyErr::from(e)),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyExtractionResult {
    result: ExtractionResult,
}

#[pymethods]
impl PyExtractionResult {
    #[getter]
    fn url(&self) -> String {
        self.result.url.clone()
    }

    #[getter]
    fn text(&self) -> Option<String> {
        self.result.text.clone()
    }

    #[getter]
    fn links(&self, py: Python) -> Option<PyObject> {
        self.result.links.as_ref().map(|gl| grouped_links_to_dict(py, gl))
    }

    #[getter]
    fn language(&self) -> Option<String> {
        self.result.language.clone()
    }

    #[getter]
    fn language_confidence(&self) -> Option<f64> {
        self.result.language_confidence
    }

    // Deprecated: Use links property instead
    #[getter]
    fn grouped_links(&self, py: Python) -> Option<PyObject> {
        self.links(py)
    }

    #[getter]
    fn socials(&self, py: Python) -> Option<PyObject> {
        self.result.socials.as_ref().map(|socials| hashmap_to_dict(py, socials))
    }

    #[getter]
    fn videos(&self, py: Python) -> Option<PyObject> {
        self.result.videos.as_ref().map(|videos| hashmap_to_dict(py, videos))
    }

    #[getter]
    fn product(&self, py: Python) -> Option<PyObject> {
        self.result.product.as_ref().map(|product| hashmap_to_dict(py, product))
    }

    #[getter]
    fn article(&self, py: Python) -> Option<PyObject> {
        self.result.article.as_ref().map(|article| hashmap_to_dict(py, article))
    }

    #[getter]
    fn content(&self, py: Python) -> Option<PyObject> {
        self.result.content.as_ref().map(|c| {
            let dict = PyDict::new(py);
            if let Some(ref text) = c.text {
                dict.set_item("text", text.clone()).unwrap();
            }
            dict.set_item("text_length", c.text_length).unwrap();
            dict.into()
        })
    }

    fn get_result(&self, py: Python) -> PyObject {
        // Return the grouped dictionary structure by category
        self.to_dict(py)
    }

    fn to_dict(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        
        dict.set_item("url", self.result.url.clone()).unwrap();
        
        // Group text-related data into "text" category
        if self.result.text.is_some() || self.result.language.is_some() || self.result.content.is_some() {
            let text_dict = PyDict::new(py);
            if let Some(ref text) = self.result.text {
                text_dict.set_item("content", text.clone()).unwrap();
            }
            if let Some(ref lang) = self.result.language {
                text_dict.set_item("language", lang.clone()).unwrap();
            }
            if let Some(confidence) = self.result.language_confidence {
                text_dict.set_item("language_confidence", confidence).unwrap();
            }
            if let Some(ref c) = self.result.content {
                text_dict.set_item("text_length", c.text_length).unwrap();
            }
            dict.set_item("text", text_dict).unwrap();
        }
        
        // Add links (grouped)
        if let Some(ref gl) = self.result.links {
            dict.set_item("links", grouped_links_to_dict(py, gl)).unwrap();
        }
        
        // Add socials
        if let Some(ref socials) = self.result.socials {
            dict.set_item("socials", hashmap_to_dict(py, socials)).unwrap();
        }
        
        // Add videos
        if let Some(ref videos) = self.result.videos {
            dict.set_item("videos", hashmap_to_dict(py, videos)).unwrap();
        }
        
        // Add product
        if let Some(ref product) = self.result.product {
            dict.set_item("product", hashmap_to_dict(py, product)).unwrap();
        }

        // Add article
        if let Some(ref article) = self.result.article {
            dict.set_item("article", hashmap_to_dict(py, article)).unwrap();
        }
        
        dict.into()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyLinkInfo {
    #[pyo3(get)]
    url: String,
    #[pyo3(get)]
    text: String,
}
