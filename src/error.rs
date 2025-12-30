use pyo3::exceptions::PyRuntimeError;
use pyo3::PyErr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ExtractionError {
    HttpError(String),
    ParseError(String),
    InvalidUrl(String),
    Timeout(String),
    Other(String),
}

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExtractionError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            ExtractionError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ExtractionError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            ExtractionError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            ExtractionError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ExtractionError {}

impl From<reqwest::Error> for ExtractionError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ExtractionError::Timeout(err.to_string())
        } else if err.is_request() {
            ExtractionError::HttpError(err.to_string())
        } else {
            ExtractionError::Other(err.to_string())
        }
    }
}

impl From<url::ParseError> for ExtractionError {
    fn from(err: url::ParseError) -> Self {
        ExtractionError::InvalidUrl(err.to_string())
    }
}

impl From<ExtractionError> for PyErr {
    fn from(err: ExtractionError) -> Self {
        PyRuntimeError::new_err(err.to_string())
    }
}

