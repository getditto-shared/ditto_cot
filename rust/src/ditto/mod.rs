//! Ditto integration for CoT events
//!
//! This module provides functionality to transform CoT (Cursor on Target) events
//! into Ditto documents according to the Ditto JSON schemas.

pub mod dql_support;
pub mod from_ditto;
pub mod from_ditto_util;
#[rustfmt::skip]
pub mod schema;
pub mod to_ditto;

// Re-export the main types and functions from to_ditto
pub use to_ditto::{
    cot_to_document, transform_chat_event, transform_emergency_event, transform_location_event,
    CotDocument,
};

// Re-export the schema types
pub use schema::*;
