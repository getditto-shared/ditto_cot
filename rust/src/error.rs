//! Error types for CoT (Cursor on Target) operations.
//!
//! This module defines the error types used throughout the CoT library,
//! including XML parsing, field validation, and format conversion errors.

use quick_xml;
use quick_xml::events::attributes::AttrError;
use serde_json;
use thiserror::Error;

/// Main error type for CoT operations.
///
/// This enum represents all possible error conditions that can occur
/// during CoT message processing, including XML parsing, validation,
/// and serialization errors.
#[derive(Error, Debug)]
pub enum CotError {
    /// An error occurred during XML processing
    #[error("XML error: {0}")]
    XmlError(String),

    /// Failed to parse XML content
    #[error("XML parse error: {0}")]
    XmlParse(String),

    /// A required field was missing from the input
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// The input format was invalid
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Failed to parse a numeric value
    #[error("Invalid numeric value for field '{field}': '{value}' - {source}")]
    InvalidNumeric {
        /// The field name that failed to parse
        field: String,
        /// The value that failed to parse
        value: String,
        /// The underlying parse error
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Failed to parse a datetime value
    #[error("Invalid datetime format for field '{field}': '{value}' - expected RFC 3339 format")]
    InvalidDateTime {
        /// The field name that failed to parse
        field: String,
        /// The value that failed to parse
        value: String,
    },

    /// An error occurred during JSON serialization/deserialization
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<quick_xml::Error> for CotError {
    fn from(err: quick_xml::Error) -> Self {
        CotError::XmlError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for CotError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        CotError::XmlError(err.to_string())
    }
}

impl From<AttrError> for CotError {
    fn from(err: AttrError) -> Self {
        CotError::XmlError(err.to_string())
    }
}
