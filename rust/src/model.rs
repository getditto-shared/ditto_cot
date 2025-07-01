//! Data models for CoT (Cursor on Target) message processing.
//!
//! This module contains the core data structures used for representing
//! and transforming CoT messages in a flattened format.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A flattened representation of a CoT (Cursor on Target) event.
///
/// This struct provides a simplified, flat structure for working with CoT events,
/// making it easier to serialize/deserialize and work with in a type-safe manner.
/// It includes all standard CoT fields plus additional metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlatCotEvent {
    /// Unique identifier for the event
    pub uid: String,

    /// Event type identifier (e.g., "a-f-G-U-C" for military ground unit)
    pub type_: String,

    /// ISO 8601 timestamp when the event was generated
    pub time: String,

    /// ISO 8601 timestamp when the event becomes valid
    pub start: String,

    /// ISO 8601 timestamp when the event expires
    pub stale: String,

    /// How the event was generated (e.g., "h-g-i-g-o" for human-generated)
    pub how: String,

    /// Latitude in decimal degrees (WGS84)
    pub lat: f64,

    /// Longitude in decimal degrees (WGS84)
    pub lon: f64,

    /// Height Above Ellipsoid in meters
    pub hae: f64,

    /// Circular Error in meters (horizontal accuracy)
    pub ce: f64,

    /// Linear Error in meters (vertical accuracy)
    pub le: f64,

    /// Optional callsign of the entity
    pub callsign: Option<String>,

    /// Optional group name the entity belongs to
    pub group_name: Option<String>,

    /// Additional event-specific details in a key-value format
    pub detail_extra: HashMap<String, Value>,
}
