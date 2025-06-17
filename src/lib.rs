//! # Ditto CoT
//!
//! A high-performance Rust library for working with Cursor on Target (CoT) messages and Ditto.
//!
//! ## Features
//! - Parse and generate CoT XML messages
//! - Transform between CoT and Ditto document formats
//! - Validate CoT messages against XML schemas
//! - Asynchronous Ditto integration
//! - Support for chat, location, and emergency message types
//!
//! ## Modules
//! - `cot_events`: Core CoT event types and parsing
//! - `ditto`: Ditto document types and transformations
//! - `ditto_sync`: Ditto database integration
//! - `error`: Error types and utilities
//! - `model`: Data models and serialization
//! - `schema_validator`: XML schema validation
//! - `xml_parser`: XML parsing utilities
//! - `xml_writer`: XML generation utilities

#![warn(missing_docs)]

/// Core CoT event types and parsing
pub mod cot_events;

/// Detail section parsing utilities
pub mod detail_parser;

/// Ditto document types and transformations
pub mod ditto;

/// Ditto database integration
pub mod ditto_sync;

/// Error types and utilities
pub mod error;

/// Data models and serialization
pub mod model;

/// XML schema validation
pub mod schema_validator;

/// XML parsing utilities
pub mod xml_parser;

/// XML generation utilities
pub mod xml_writer;

// Re-export commonly used types
pub use ditto::{cot_to_document, DittoDocument};
