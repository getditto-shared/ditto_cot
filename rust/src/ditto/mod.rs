//! Ditto integration for CoT events
//!
//! This module provides functionality to transform CoT (Cursor on Target) events
//! into Ditto documents according to the Ditto JSON schemas.

pub mod dql_support;
pub mod from_ditto;
pub mod from_ditto_util;
pub mod r_field_flattening;
#[rustfmt::skip]
pub mod schema;
pub mod to_ditto;
pub mod sdk_conversion;

// Re-export the main types and functions from to_ditto
pub use to_ditto::{
    cot_to_document, cot_to_flattened_document, transform_chat_event, transform_emergency_event,
    transform_location_event, CotDocument,
};

// Re-export the conversion functions from from_ditto
pub use from_ditto::{cot_event_from_ditto_document, cot_event_from_flattened_json};
pub use from_ditto_util::{flat_cot_event_from_ditto, flat_cot_event_from_flattened_json};

// Re-export the schema types
pub use schema::*;

// Re-export observer document conversion utilities
pub use sdk_conversion::{
    observer_json_to_cot_document, observer_json_to_json_with_r_fields,
    get_document_id_from_value, get_document_id_from_json,
    get_document_type_from_value, get_document_type_from_json
};
