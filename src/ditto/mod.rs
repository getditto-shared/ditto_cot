//! Ditto integration for CoT events
//!
//! This module provides functionality to transform CoT (Cursor on Target) events
//! into Ditto documents according to the Ditto JSON schemas.

pub mod schema;

use crate::cot_events::CotEvent;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// No unused imports remaining

pub use schema::*;

/// Convert a CoT event to the appropriate Ditto document type
pub fn cot_to_document(event: &CotEvent, peer_key: &str) -> DittoDocument {
    let event_type = event.event_type();

    if event_type == "a-u-emergency-g" {
        // Handle emergency events
        DittoDocument::Api(transform_emergency_event(event, peer_key))
    } else if event_type.contains("b-t-f") || event_type.contains("chat") {
        // Handle chat events
        match transform_chat_event(event, peer_key) {
            Some(chat_doc) => DittoDocument::Chat(chat_doc),
            None => DittoDocument::File(transform_generic_event(event, peer_key)),
        }
    } else if event_type.contains("a-u-r-loc-g")
        || event_type.contains("a-f-G-U-C")
        || event_type.contains("a-f-G-U")
        || event_type.contains("a-f-G-U-I")
        || event_type.contains("a-f-G-U-T")
    {
        // Handle location update events
        DittoDocument::MapItem(transform_location_event(event, peer_key))
    } else {
        // Fall back to generic document for all other event types
        DittoDocument::File(transform_generic_event(event, peer_key))
    }
}

/// Transform a location CoT event to a Ditto location document
pub fn transform_location_event(event: &CotEvent, peer_key: &str) -> MapItem {
    // Map CotEvent and peer_key to MapItem fields
    MapItem {
        id: event.uid.clone(), // Ditto document ID
        a: peer_key.to_string(), // Ditto peer key string
        b: event.point.ce, // Circular error as a best guess
        c: Some(event.detail.get("name").cloned().unwrap_or_default()), // Name/title if present
        d: event.uid.clone(), // TAK UID of author
        d_c: 0, // Document counter (updates), default to 0
        d_r: false, // Soft-delete flag, default to false
        d_v: 2, // Schema version (2)
        e: event.detail.get("callsign").cloned().unwrap_or_default(), // Callsign of author
        f: None, // Visibility flag
        g: "".to_string(), // Version string, default empty
        h: Some(event.point.lat), // Latitude
        i: Some(event.point.lon), // Longitude
        j: Some(event.point.hae), // Altitude
        k: None, // Speed (not in CotEvent)
        l: None, // Course (not in CotEvent)
        n: event.start.timestamp_millis(), // Start
        o: event.stale.timestamp_millis(), // Stale
        p: event.how.clone(), // How
        q: "".to_string(), // Access, default empty
        r: "".to_string(), // Detail (XML CotDetail), default empty
        s: "".to_string(), // Opex, default empty
        t: "".to_string(), // Qos, default empty
        u: "".to_string(), // Caveat, default empty
        v: "".to_string(), // Releasable to, default empty
        w: event.event_type.clone(), // Type
    }
}

/// Transform a chat CoT event to a Ditto chat document
pub fn transform_chat_event(event: &CotEvent, peer_key: &str) -> Option<Chat> {
    // Extract chat message and room from event.detail
    let message = event.detail.get("chat").cloned();
    let room = event.detail.get("chatroom").cloned();
    let room_id = event.detail.get("chat_group_uid").cloned();
    let author_callsign = event.detail.get("callsign").cloned();
    let author_uid = Some(event.uid.clone());
    let author_type = Some("user".to_string());
    let location = Some(format!("{},{},{}", event.point.lat, event.point.lon, event.point.hae));
    let time = Some(event.time.to_rfc3339());
    
    // If there's no message, return None
    message.as_ref()?;

    Some(Chat {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: event.point.ce,

        d: event.uid.clone(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: author_callsign.clone().unwrap_or_default(),
        g: "".to_string(),
        h: Some(event.point.lat),
        i: Some(event.point.lon),
        j: Some(event.point.hae),
        k: None,
        l: None,
        n: event.start.timestamp_millis(),
        o: event.stale.timestamp_millis(),
        p: event.how.clone(),
        q: "".to_string(),
        r: "".to_string(),
        s: "".to_string(),
        t: "".to_string(),
        u: "".to_string(),
        v: "".to_string(),
        w: event.event_type.clone(),
        author_callsign,
        author_type,
        author_uid,
        location,
        message,
        parent: None,
        room,
        room_id,
        time,
    })
}

/// Transform an emergency CoT event to a Ditto emergency document
pub fn transform_emergency_event(event: &CotEvent, peer_key: &str) -> Api {
    let title = event.detail.get("type").cloned();
    let data = event.detail.get("message").cloned();
    let mime = Some("application/vnd.cot.emergency+json".to_string());
    let content_type = Some("emergency".to_string());
    let is_file = Some(false);
    let is_removed = Some(false);
    let tag = event.detail.get("status").cloned();
    let time_millis = Some(event.time.timestamp_millis());

    Api {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: event.point.ce,
        content_type,
        d: event.uid.clone(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        data,
        e: event.detail.get("callsign").cloned().unwrap_or_default(),
        g: "".to_string(),
        h: Some(event.point.lat),
        i: Some(event.point.lon),
        j: Some(event.point.hae),
        k: None,
        l: None,
        mime,
        n: event.start.timestamp_millis(),
        o: event.stale.timestamp_millis(),
        p: event.how.clone(),
        q: "".to_string(),
        r: "".to_string(),
        s: "".to_string(),
        t: "".to_string(),
        tag,
        time_millis,
        title,
        u: "".to_string(),
        v: "".to_string(),
        w: event.event_type.clone(),
        is_file,
        is_removed,
    }
}

/// Transform any CoT event to a generic Ditto document
fn transform_generic_event(event: &CotEvent, peer_key: &str) -> File {
    let c = event.detail.get("file_name").cloned();
    let file = event.detail.get("file_token").cloned();
    let mime = event.detail.get("mime").cloned();
    let content_type = Some("generic".to_string());
    let item_id = event.detail.get("item_id").cloned();
    let sz = event.detail.get("size").and_then(|s| s.parse().ok());

    File {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: event.point.ce,
        c,
        content_type,
        d: event.uid.clone(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: event.detail.get("callsign").cloned().unwrap_or_default(),
        file,
        g: "".to_string(),
        h: Some(event.point.lat),
        i: Some(event.point.lon),
        j: Some(event.point.hae),
        k: None,
        l: None,
        item_id,
        mime,
        n: event.start.timestamp_millis(),
        o: event.stale.timestamp_millis(),
        p: event.how.clone(),
        q: "".to_string(),
        r: "".to_string(),
        s: "".to_string(),
        sz,
        t: "".to_string(),
        u: "".to_string(),
        v: "".to_string(),
        w: event.event_type.clone(),
    }
}

///   Represents a Ditto document that can be one of several specific types.
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
///   Represents a Ditto document that can be one of several specific types.
///
/// This is the main enum used when working with Ditto documents in the system.
/// It uses `#[serde(untagged)]` to ensure clean serialization/deserialization
/// without an additional type tag in the JSON representation.
pub enum DittoDocument {
    /// For API/emergency/alert documents
    Api(Api),
    /// For chat/messaging content
    Chat(Chat),
    /// For generic file/documents
    File(File),
    /// For geospatial map/location items
    MapItem(MapItem),
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

        assert_eq!(doc.id, "test-uid");
        assert_eq!(doc.a, "test-peer");
        assert_eq!(doc.w, "a-u-r-loc-g");
        assert_eq!(doc.h, Some(1.2345));
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
            assert_eq!(chat_doc.id, "test-uid");
            assert_eq!(chat_doc.a, "test-peer");
            assert_eq!(chat_doc.message, Some("Test message".to_string()));
            assert_eq!(chat_doc.room, Some("All".to_string()));
        } else {
            panic!("Expected Chat document, got {:?}", doc);
        }
    }

    #[test]
    fn test_transform_emergency_event() {
        let event = create_test_event("u-emergency");
        let doc = cot_to_document(&event, "test-peer");

        if let DittoDocument::Api(api_doc) = doc {
            assert_eq!(api_doc.id, "test-uid");
            assert_eq!(api_doc.a, "test-peer");
            // No emergency_type field in Api; check title or w if needed
        } else {
            panic!("Expected Api document, got {:?}", doc);
        }
    }

    #[test]
    fn test_transform_generic_event() {
        let event = create_test_event("u-generic");
        let doc = cot_to_document(&event, "test-peer");

        if let DittoDocument::File(file_doc) = doc {
            assert_eq!(file_doc.id, "test-uid");
            assert_eq!(file_doc.a, "test-peer");
            assert_eq!(file_doc.w, "a-u-generic-g");
        } else {
            panic!("Expected File document, got {:?}", doc);
        }
    }
}
