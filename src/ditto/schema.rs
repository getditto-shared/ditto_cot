//! Ditto schema types and validation
//!
//! This module contains Rust types that correspond to Ditto's JSON schemas.
//! These types are used for type-safe serialization/deserialization and validation.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Common fields present in all Ditto documents
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CommonFields {
    /// Ditto document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document counter (updates)
    #[serde(rename = "_c")]
    pub counter: i32,

    /// Schema version
    #[serde(rename = "_v")]
    pub version: i32,

    /// Soft-delete flag
    #[serde(rename = "_r")]
    pub deleted: bool,

    /// Ditto peer key string
    #[serde(rename = "a")]
    pub peer_key: String,

    /// Timestamp in milliseconds since epoch
    #[serde(rename = "b")]
    pub timestamp: i64,

    /// TAK UID of author
    #[serde(rename = "d")]
    pub author_uid: String,

    /// Callsign of author
    #[serde(rename = "e")]
    pub author_callsign: String,

    /// Version
    #[serde(rename = "g", default)]
    pub version_str: String,

    /// Circular error (CE)
    #[serde(rename = "h", default)]
    pub ce: f64,
}

/// Location information for Ditto documents
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Location {
    /// Latitude in degrees
    #[serde(rename = "lat")]
    pub latitude: f64,

    /// Longitude in degrees
    #[serde(rename = "lon")]
    pub longitude: f64,

    /// Altitude in meters
    #[serde(rename = "hae", default)]
    pub altitude: f64,

    /// Circular error in meters
    #[serde(rename = "ce", default)]
    pub circular_error: f64,

    /// Speed in meters per second
    #[serde(rename = "speed", default)]
    pub speed: f64,

    /// Course in degrees (0-360)
    #[serde(rename = "course", default)]
    pub course: f64,
}

/// Chat message document
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChatDocument {
    /// Common fields
    #[serde(flatten)]
    pub common: CommonFields,

    /// Chat message content
    pub message: String,

    /// Room name
    pub room: String,

    /// Parent message ID (for threaded conversations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Room ID
    pub room_id: String,

    /// Author callsign
    pub author_callsign: String,

    /// Author UID
    pub author_uid: String,

    /// Author type
    pub author_type: String,

    /// Message timestamp
    pub time: String,

    /// Location as a GeoPoint string ("lat,lon,hae")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// Location update document
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocationDocument {
    /// Common fields
    #[serde(flatten)]
    pub common: CommonFields,

    /// Location information
    pub location: Location,

    /// Type of location update
    pub location_type: String,

    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Emergency alert document
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmergencyDocument {
    /// Common fields
    #[serde(flatten)]
    pub common: CommonFields,

    /// Emergency type
    pub emergency_type: String,

    /// Emergency status
    pub status: String,

    /// Location of the emergency
    pub location: Location,

    /// Additional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Generic document for unsupported CoT types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GenericDocument {
    /// Common fields
    #[serde(flatten)]
    pub common: CommonFields,

    /// Original CoT type
    pub cot_type: String,

    /// Raw CoT event data
    pub raw_data: serde_json::Value,
}

impl CommonFields {
    /// Create a new CommonFields with default values
    pub fn new(id: String, peer_key: String, author_uid: String, author_callsign: String) -> Self {
        Self {
            id,
            counter: 0,
            version: 2,
            deleted: false,
            peer_key,
            timestamp: chrono::Utc::now().timestamp_millis(),
            author_uid,
            author_callsign,
            version_str: String::new(),
            ce: 0.0,
        }
    }
}
