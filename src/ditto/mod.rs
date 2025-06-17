//! Ditto integration for CoT events
//!
//! This module provides functionality to transform CoT (Cursor on Target) events
//! into Ditto documents according to the Ditto JSON schemas.

pub mod schema;

use crate::cot_events::CotEvent;
use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// No unused imports remaining

pub use schema::*;

/// Convert a CoT event to the appropriate Ditto document type
pub fn cot_to_document(event: &CotEvent, peer_key: &str) -> DittoDocument {
    let event_type = event.event_type();

    if event_type.contains("a-f-G") || event_type.contains("a-f-G-U-C") {
        // Handle emergency events
        DittoDocument::Emergency(transform_emergency_event(event, peer_key))
    } else if event_type.contains("b-t-f") || event_type.contains("chat") {
        // Handle chat events
        match transform_chat_event(event, peer_key) {
            Some(chat_doc) => DittoDocument::Chat(Box::new(chat_doc)),
            None => DittoDocument::Generic(transform_generic_event(event, peer_key)),
        }
    } else if event_type.contains("a-f-G-U-C")
        || event_type.contains("a-f-G-U")
        || event_type.contains("a-f-G-U-I")
        || event_type.contains("a-f-G-U-T")
    {
        // Handle location update events
        DittoDocument::Location(transform_location_event(event, peer_key))
    } else {
        // Fall back to generic document for all other event types
        DittoDocument::Generic(transform_generic_event(event, peer_key))
    }
}

/// Transform a location CoT event to a Ditto location document
fn transform_location_event(event: &CotEvent, peer_key: &str) -> LocationDocument {
    let point = event.point();
    let now = Utc::now();

    LocationDocument {
        common: CommonFields {
            id: event.uid().to_string(),
            counter: 0,
            version: 2,
            deleted: false,
            peer_key: peer_key.to_string(),
            timestamp: now.timestamp_millis(),
            author_uid: event.uid().to_string(),
            author_callsign: event.callsign().unwrap_or("unknown").to_string(),
            version_str: "".to_string(),
            ce: point.ce,
        },
        location: Location {
            latitude: point.lat,
            longitude: point.lon,
            altitude: point.hae,
            circular_error: point.ce,
            speed: 0.0,  // Not available in basic CoT
            course: 0.0, // Not available in basic CoT
        },
        location_type: event.event_type().to_string(),
        metadata: None,
    }
}

/// Transform a chat CoT event to a Ditto chat document
pub fn transform_chat_event(event: &CotEvent, peer_key: &str) -> Option<ChatDocument> {
    let message = event.detail.get("chat")?;
    let room = event
        .detail
        .get("chatroom")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "default".to_string());
    let chat_group_uid = event.detail.get("chat_group_uid").map(|s| s.to_string());
    let point = event.point();
    let now = Utc::now();

    Some(ChatDocument {
        common: CommonFields {
            id: event.uid().to_string(),
            counter: 0,
            version: 2,
            deleted: false,
            peer_key: peer_key.to_string(),
            timestamp: now.timestamp_millis(),
            author_uid: event.uid().to_string(),
            author_callsign: event.callsign().unwrap_or("unknown").to_string(),
            version_str: "".to_string(),
            ce: point.ce,
        },
        message: message.to_string(),
        room,
        room_id: chat_group_uid.unwrap_or_else(|| "default_room".to_string()),
        parent: None,
        author_callsign: event.callsign().unwrap_or("unknown").to_string(),
        author_uid: event.uid().to_string(),
        author_type: "user".to_string(),
        time: now.to_rfc3339(),
        location: Some(format!("{},{},{}", point.lat, point.lon, point.hae)),
    })
}

/// Transform an emergency CoT event to a Ditto emergency document
fn transform_emergency_event(event: &CotEvent, peer_key: &str) -> EmergencyDocument {
    let point = event.point();
    let now = Utc::now();

    EmergencyDocument {
        common: CommonFields {
            id: event.uid().to_string(),
            counter: 0,
            version: 2,
            deleted: false,
            peer_key: peer_key.to_string(),
            timestamp: now.timestamp_millis(),
            author_uid: event.uid().to_string(),
            author_callsign: event.callsign().unwrap_or("unknown").to_string(),
            version_str: "".to_string(),
            ce: point.ce,
        },
        emergency_type: event.detail.get("type").cloned().unwrap_or_default(),
        status: event
            .detail
            .get("status")
            .cloned()
            .unwrap_or_else(|| "active".to_string()),
        location: Location {
            latitude: point.lat,
            longitude: point.lon,
            altitude: point.hae,
            circular_error: point.ce,
            speed: 0.0,
            course: 0.0,
        },
        details: event
            .detail
            .get("message")
            .cloned()
            .map(|msg| serde_json::json!({ "message": msg })),
    }
}

/// Transform any CoT event to a generic Ditto document
fn transform_generic_event(event: &CotEvent, peer_key: &str) -> GenericDocument {
    let point = event.point();
    let now = Utc::now();

    GenericDocument {
        common: CommonFields {
            id: event.uid().to_string(),
            counter: 0,
            version: 2,
            deleted: false,
            peer_key: peer_key.to_string(),
            timestamp: now.timestamp_millis(),
            author_uid: event.uid().to_string(),
            author_callsign: event.callsign().unwrap_or("unknown").to_string(),
            version_str: "".to_string(),
            ce: point.ce,
        },
        cot_type: event.event_type().to_string(),
        raw_data: serde_json::to_value(event).unwrap_or_default(),
    }
}

/// Represents a Ditto document that can be one of several specific types.
///
/// This is the main enum used when working with Ditto documents in the system.
/// It uses `#[serde(untagged)]` to ensure clean serialization/deserialization
/// without an additional type tag in the JSON representation.
///
/// # Variants
/// - `Chat`: For chat/messaging content (boxed to reduce enum size)
/// - `Location`: For geospatial location updates
/// - `Emergency`: For emergency/alert notifications
/// - `Generic`: For any CoT event that doesn't match the above types
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DittoDocument {
    /// A chat message document (boxed due to large size)
    Chat(Box<ChatDocument>),

    /// A location update document
    Location(LocationDocument),

    /// An emergency/alert document
    Emergency(EmergencyDocument),

    /// A generic document for any other CoT event type
    Generic(GenericDocument),
}

impl From<ChatDocument> for DittoDocument {
    fn from(doc: ChatDocument) -> Self {
        DittoDocument::Chat(Box::new(doc))
    }
}

impl From<LocationDocument> for DittoDocument {
    fn from(doc: LocationDocument) -> Self {
        DittoDocument::Location(doc)
    }
}

impl From<EmergencyDocument> for DittoDocument {
    fn from(doc: EmergencyDocument) -> Self {
        DittoDocument::Emergency(doc)
    }
}

impl From<GenericDocument> for DittoDocument {
    fn from(doc: GenericDocument) -> Self {
        DittoDocument::Generic(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cot_events::CotEvent;
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;

    fn create_test_event(event_type: &str) -> CotEvent {
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let mut detail = HashMap::new();
        detail.insert("callsign".to_string(), "TEST".to_string());

        CotEvent {
            version: "2.0".to_string(),
            uid: "test-uid".to_string(),
            event_type: format!("a-{}-g", event_type),
            time,
            start: time,
            stale: time + chrono::Duration::days(1),
            how: "h-g-i-g-o".to_string(),
            point: crate::cot_events::Point {
                lat: 1.2345,
                lon: 2.3456,
                hae: 100.0,
                ce: 10.0,
                le: 20.0,
            },
            detail,
        }
    }

    #[test]
    fn test_transform_location_event() {
        let event = create_test_event("u-r-loc");
        let doc = transform_location_event(&event, "test-peer");

        assert_eq!(doc.common.id, "test-uid");
        assert_eq!(doc.common.peer_key, "test-peer");
        assert_eq!(doc.location_type, "a-u-r-loc-g");
        assert_eq!(doc.location.latitude, 1.2345);
    }

    #[test]
    fn test_transform_chat_event() {
        let mut event = create_test_event("u-chat");
        event
            .detail
            .insert("chat".to_string(), "Test message".to_string());
        event
            .detail
            .insert("chatroom".to_string(), "All".to_string());
        event
            .detail
            .insert("chat_group_uid".to_string(), "group-1".to_string());
        event.event_type = "b-t-f".to_string();

        let doc = cot_to_document(&event, "test-peer");

        if let DittoDocument::Chat(chat_doc) = doc {
            assert_eq!(chat_doc.common.id, "test-uid");
            assert_eq!(chat_doc.common.peer_key, "test-peer");
            assert_eq!(chat_doc.message, "Test message");
            assert_eq!(chat_doc.room, "All");
        } else {
            panic!("Expected Chat document, got {:?}", doc);
        }
    }
}
