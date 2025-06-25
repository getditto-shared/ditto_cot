//! Ditto integration for CoT events
//!
//! This module provides functionality to transform CoT (Cursor on Target) events
//! into Ditto documents according to the Ditto JSON schemas.

use crate::cot_events::CotEvent;

use serde::{Deserialize, Serialize};
use anyhow;
// No unused imports remaining

pub use super::schema::*;

/// Convert a CoT event to the appropriate Ditto document type
pub fn cot_to_document(event: &CotEvent, peer_key: &str) -> DittoDocument {
    let event_type = &event.event_type;

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
        id: event.uid.clone(),   // Ditto document ID
        a: peer_key.to_string(), // Ditto peer key string
        b: event.time.timestamp_millis() as f64, // Time in millis since epoch
        c: None, // Name/title not parsed from raw detail string
        d: event.uid.clone(),    // TAK UID of author
        d_c: 0,                  // Document counter (updates), default to 0
        d_r: false,              // Soft-delete flag, default to false
        d_v: 2,                  // Schema version (2)
        source: None, // Source not parsed from raw detail string
        e: String::new(), // Callsign not parsed from raw detail string
        f: None,                 // Visibility flag
        g: "".to_string(),       // Version string, default empty
        h: Some(event.point.lat), // Latitude
        i: Some(event.point.lon), // Longitude
        j: Some(event.point.hae), // Altitude
        k: Some(event.point.le),  // Linear Error
        l: None,                 // Course (not in CotEvent)
        n: event.start.timestamp_micros(), // Start (microsecond precision)
        o: event.stale.timestamp_micros(), // Stale (microsecond precision)
        p: event.how.clone(),    // How
        q: "".to_string(),       // Access, default empty
        r: event.detail.clone(), // Detail (XML CotDetail)
        s: "".to_string(),       // Opex, default empty
        t: "".to_string(),       // Qos, default empty
        u: "".to_string(),       // Caveat, default empty
        v: "".to_string(),       // Releasable to, default empty
        w: event.event_type.clone(),
 // Type

    }
}

/// Transform a chat CoT event to a Ditto chat document
pub fn transform_chat_event(event: &CotEvent, peer_key: &str) -> Option<Chat> {
    // Extract chat message and room from event.detail
    // Naive parsing: split by spaces and assign to fields if possible
    let parts: Vec<&str> = event.detail.split_whitespace().collect();
    let message = if parts.len() >= 2 {
        Some(format!("{} {}", parts[0], parts[1]))
    } else {
        parts.get(0).map(|s| s.to_string())
    };
    let room = parts.get(2).map(|s| s.to_string());
    let room_id = None;
    let author_callsign = None;
    let author_uid = Some(event.uid.clone());
    let author_type = Some("user".to_string());
    let location = Some(format!(
        "{},{},{}",
        event.point.lat, event.point.lon, event.point.hae
    ));

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
        source: None,
        e: String::new(),
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
        time: Some(event.time.to_rfc3339()),

    })
}

/// Transform an emergency CoT event to a Ditto emergency document
pub fn transform_emergency_event(event: &CotEvent, peer_key: &str) -> Api {
    let title = None;
    let data = None;
    let mime = Some("application/vnd.cot.emergency+json".to_string());
    let content_type = Some("emergency".to_string());
    let is_file = Some(false);
    let is_removed = Some(false);
    let tag = None;

    Api {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: event.time.timestamp_millis() as f64, // Time
        content_type,
        d: event.uid.clone(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        source: None,
        data,
        e: String::new(),
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

        title,
        u: "".to_string(),
        v: "".to_string(),
        w: event.event_type.clone(),
        time_millis: Some(event.time.timestamp_millis()),
        is_file,
        is_removed,

    }
}

/// Transform any CoT event to a generic Ditto document
fn transform_generic_event(event: &CotEvent, peer_key: &str) -> File {
    let c = None;
    let file = None;
    let mime = None;
    let content_type = Some("generic".to_string());
    let item_id = None;
    let sz = None;

    File {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: event.time.timestamp_millis() as f64, // Time in millis since epoch
        c,
        content_type,
        d: event.uid.clone(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        source: None,
        e: String::new(),
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

/// Represents a Ditto document that can be one of several specific types.
///
/// This is the main enum used when working with Ditto documents in the system.
/// It uses `#[serde(untagged)]` to ensure clean serialization/deserialization
/// without an additional type tag in the JSON representation.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(untagged)]
pub enum DittoDocument {
    /// API document type
    Api(Api),
    /// Chat message document type
    Chat(Chat),
    /// File document type
    File(File),
    /// Map item document type
    MapItem(MapItem),
}

impl DittoDocument {
    /// Returns true if this is a MapItem variant
    pub fn is_map_item(&self) -> bool {
        matches!(self, DittoDocument::MapItem(_))
    }

    /// Returns a reference to the inner MapItem if this is a MapItem variant
    pub fn as_map_item(&self) -> Option<&MapItem> {
        if let DittoDocument::MapItem(item) = self {
            Some(item)
        } else {
            None
        }
    }

    /// Returns true if this document has the specified key in its top-level fields
    /// This is a simplified implementation that only checks a few common fields
    pub fn has_key(&self, key: &str) -> bool {
        match self {
            DittoDocument::Api(_api) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                _ => false,
            },
            DittoDocument::Chat(_chat) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                "e" => true, // callsign
                _ => false,
            },
            DittoDocument::File(_file) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                "e" => true, // callsign
                _ => false,
            },
            DittoDocument::MapItem(_map_item) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                "e" => true, // callsign
                _ => false,
            },
        }
    }

    /// Deserialize a JSON string into a DittoDocument, determining the variant based on the 'w' field.
    /// Handles defaults for missing fields in variants.
    pub fn from_json_str(json_str: &str) -> Result<Self, anyhow::Error> {
        let json_value: serde_json::Value = serde_json::from_str(json_str).map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;
        let doc_type = json_value.get("w")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Document is missing 'w' field"))?;
        
        if doc_type.starts_with("a-f-G-U") || doc_type.starts_with("a-u-r-loc") {
            // Deserialize as MapItem and handle defaults
            let mut map_item: MapItem = serde_json::from_value(json_value.clone())
                .map_err(|e| anyhow::anyhow!("Failed to deserialize as MapItem: {}", e))?;
            // Ensure required fields have default values if missing
            if map_item.d_c == 0 {
                let c_value = json_value.get("d_c").or_else(|| json_value.get("_c"));
                if c_value.is_none() {
                    map_item.d_c = 1; // Default document counter
                }
            }
            Ok(DittoDocument::MapItem(map_item))
        } else if doc_type.contains("b-t-f") || doc_type.contains("chat") {
            // Deserialize as Chat
            let chat: Chat = serde_json::from_value(json_value)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize as Chat: {}", e))?;
            Ok(DittoDocument::Chat(chat))
        } else if doc_type == "a-u-emergency-g" {
            // Deserialize as Api
            let api: Api = serde_json::from_value(json_value)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize as Api: {}", e))?;
            Ok(DittoDocument::Api(api))
        } else {
            // Default to File for unknown types
            let file: File = serde_json::from_value(json_value)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize as File: {}", e))?;
            Ok(DittoDocument::File(file))
        }
    }

    /// Converts this Ditto document back into a CoT (Cursor on Target) event.
    ///
    /// This performs a best-effort conversion, preserving as much information as possible.
    /// The conversion may not be perfectly lossless due to differences between the data models.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ditto_cot::ditto::DittoDocument;
    /// # use ditto_cot::cot_events::CotEvent;
    /// # fn example(doc: DittoDocument) -> CotEvent {
    /// let cot_event = doc.to_cot_event();
    /// // Now you can work with the CoT event
    /// cot_event
    /// # }
    /// ```
    pub fn to_cot_event(&self) -> CotEvent {
        crate::ditto::from_ditto::cot_event_from_ditto_document(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    

    use crate::cot_events::CotEvent;

    fn create_test_event(event_type: &str) -> CotEvent {
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        let detail = "TEST".to_string();

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
        event.detail = "Test message All group-1".to_string();
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

    // Tests for Ditto to CoT conversion
    mod ditto_to_cot_tests {
        use super::*;

        fn create_test_timestamp() -> i64 {
            Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
                .unwrap()
                .timestamp_millis()
        }

        #[test]
        fn test_api_to_cot() {
            let api = Api {
                id: "test-api-id".to_string(),
                a: "test-peer".to_string(),
                b: 100.0, // ce
                d: "test-uid".to_string(),
                d_c: 1,
                d_v: 2,
                d_r: false,
                e: "test-callsign".to_string(),
                g: "1.0".to_string(),
                h: None,
                i: None,
                j: None,
                k: None,
                l: None,
                mime: Some("text/plain".to_string()),
                n: create_test_timestamp(),
                o: create_test_timestamp() + 86400000, // +1 day
                p: "h-g-i-g-o".to_string(),
                q: "".to_string(),
                r: "test-callsign application/json Test message ditto_cot text/plain original_type=a-u-emergency-g".to_string(),
                source: Some("ditto_cot".to_string()),
                s: "".to_string(),
                t: "".to_string(),
                u: "".to_string(),
                v: "".to_string(),
                w: "a-u-emergency-g".to_string(),
                time_millis: Some(100),
                data: Some("Test message".to_string()),
                content_type: Some("application/json".to_string()),
                is_file: Some(false),
                is_removed: Some(false),
                tag: Some("status".to_string()),
        
                title: Some("Test Title".to_string()),
            };

            let doc = DittoDocument::Api(api);
            let event = doc.to_cot_event();

            assert_eq!(event.uid, "test-api-id");
            assert_eq!(event.event_type, "a-u-emergency-g");
            assert_eq!(event.how, "h-g-i-g-o");

            // Check point coordinates
            assert_eq!(event.point.lat, 0.0); // Default value when h is None
            assert_eq!(event.point.lon, 0.0); // Default value when i is None
            assert_eq!(event.point.hae, 0.0); // Default value when j is None
            assert_eq!(event.point.ce, 100.0); // From field b in the test data
            assert_eq!(event.point.le, 0.0); // Default value when l is None

            // Check detail fields
            assert!(event.detail.contains("test-callsign"));
            assert!(event.detail.contains("application/json"));
            assert!(event.detail.contains("Test message"));
            assert!(event.detail.contains("ditto_cot"));
            assert!(event.detail.contains("text/plain"));
            assert!(event.detail.contains("original_type=a-u-emergency-g"));
        }

        #[test]
        fn test_chat_to_cot() {
            let chat = Chat {
                id: "test-chat".to_string(),
                a: "test-peer".to_string(),
                b: 1234567890.0,
                d: "test-uid".to_string(),
                d_c: 1,
                d_r: false,
                d_v: 2,
                source: Some("ditto_cot".to_string()),
                e: "test-callsign".to_string(),
                g: "1.0".to_string(),
                h: None,
                i: None,
                j: None,
                k: None,
                l: None,
                n: 0,
                o: 0,
                p: "h-g-i-g-o".to_string(),
                q: "".to_string(),
                r: "test-callsign Test message test-room a-f-G-U-C test-uid 41.1234,-71.1234,0.0 2023-01-01T00:00:00Z ditto_cot original_type=b-t-f".to_string(),
                s: "".to_string(),
                t: "".to_string(),
                u: "".to_string(),
                v: "".to_string(),
                w: "b-t-f".to_string(),
                author_callsign: Some("test-callsign".to_string()),
                author_type: Some("a-f-G-U-C".to_string()),
                author_uid: Some("test-uid".to_string()),
                location: Some("41.1234,-71.1234,0.0".to_string()),
                message: Some("Test message".to_string()),
                parent: None,
                room: Some("test-room".to_string()),
                room_id: Some("test-room".to_string()),
                time: Some("2023-01-01T00:00:00Z".to_string()),
            };

            let doc = DittoDocument::Chat(chat);
            let event = doc.to_cot_event();

            assert_eq!(event.uid, "test-chat");
            assert_eq!(event.event_type, "b-t-f");
            assert_eq!(event.how, "h-g-i-g-o");

            // Check detail fields
            assert!(event.detail.contains("test-callsign"));
            assert!(event.detail.contains("Test message"));
            assert!(event.detail.contains("test-room"));
            assert!(event.detail.contains("a-f-G-U-C"));
            assert!(event.detail.contains("test-uid"));
            assert!(event.detail.contains("41.1234,-71.1234,0.0"));
            assert!(event.detail.contains("2023-01-01T00:00:00Z"));
            assert!(event.detail.contains("ditto_cot"));
            assert!(event.detail.contains("original_type=b-t-f"));
        }

        #[test]
        fn test_file_to_cot() {
            let file = File {
                id: "test-file-id".to_string(),
                a: "test-peer".to_string(),
                b: 1234567890.0,
                c: Some("test.txt".to_string()),
                content_type: Some("text/plain".to_string()),
                d: "test-uid".to_string(),
                d_c: 1,
                d_r: false,
                d_v: 2,
                e: "test-callsign".to_string(),
                file: Some("file-token-123".to_string()),
                g: "1.0".to_string(),
                h: None,
                i: None,
                item_id: Some("test-item-123".to_string()),
                j: None,
                k: None,
                l: None,
                mime: Some("text/plain".to_string()),
                n: 0,
                o: 0,
                p: "h-g-i-g-o".to_string(),
                q: "".to_string(),
                r: "test-callsign test.txt text/plain 1024 file-token-123 test-item-123 ditto_cot original_type=a-f-G-U".to_string(),
                source: Some("ditto_cot".to_string()),
                s: "".to_string(),
                sz: Some(1024.0),
                t: "".to_string(),
                u: "".to_string(),
                v: "".to_string(),
                w: "a-f-G-U".to_string(),
            };

            let doc = DittoDocument::File(file);
            let event = doc.to_cot_event();

            assert_eq!(event.uid, "test-file-id");
            assert_eq!(event.event_type, "a-f-G-U");
            assert_eq!(event.how, "h-g-i-g-o");

            // Check point coordinates
            assert_eq!(event.point.lat, 0.0); // Default value when h is None
            assert_eq!(event.point.lon, 0.0); // Default value when i is None
            assert_eq!(event.point.hae, 0.0); // Default value when j is None
            assert_eq!(event.point.ce, 1234567890.0); // From field b in the test data
            assert_eq!(event.point.le, 0.0); // Default value when l is None

            // Check detail fields
            assert!(event.detail.contains("test-callsign"));
            assert!(event.detail.contains("test.txt"));
            assert!(event.detail.contains("text/plain"));
            assert!(event.detail.contains("1024"));
            assert!(event.detail.contains("file-token-123"));
            assert!(event.detail.contains("test-item-123"));
            assert!(event.detail.contains("ditto_cot"));
            assert!(event.detail.contains("original_type=a-f-G-U"));
        }

        #[test]
        fn test_map_item_to_cot() {
            let map_item = MapItem {
                id: "test-map-item".to_string(),
                a: "test-peer".to_string(),
                b: 1.0, // ce
                c: Some("Test Map Item".to_string()),
                d: "test-uid".to_string(),
                d_c: 1,
                d_v: 2,
                d_r: false,
                e: "test-callsign".to_string(),
                f: Some(true), // visible
                g: "1.0".to_string(),
                h: Some(1.2345), // lat
                i: Some(2.3456), // lon
                j: Some(100.0),  // hae
                k: Some(1.0),    // ce
                l: Some(5.0),    // le
                n: create_test_timestamp(),
                o: create_test_timestamp() + 86400000, // +1 day
                p: "h-g-i-g-o".to_string(),
                q: "".to_string(),
                r: "test-callsign Test Map Item ditto_cot original_type=a-f-G-U".to_string(),
                source: Some("ditto_cot".to_string()),
                s: "".to_string(),
                t: "".to_string(),
                u: "".to_string(),
                v: "".to_string(),
                w: "a-f-G-U".to_string(),
            };

            let doc = DittoDocument::MapItem(map_item);
            let event = doc.to_cot_event();

            assert_eq!(event.uid, "test-map-item");
            assert_eq!(event.event_type, "a-f-G-U");
            assert_eq!(event.how, "h-g-i-g-o");

            // Check point coordinates
            assert_eq!(event.point.lat, 1.2345);
            assert_eq!(event.point.lon, 2.3456);
            assert_eq!(event.point.hae, 100.0);
            assert_eq!(event.point.ce, 1.0);
            assert_eq!(event.point.le, 1.0); // From field k in the test data

            // Check detail fields
            assert!(event.detail.contains("test-callsign"));
            assert!(event.detail.contains("Test Map Item"));
            assert!(event.detail.contains("ditto_cot"));
            assert!(event.detail.contains("original_type=a-f-G-U"));
        }

        #[test]
        fn test_round_trip_conversion() {
            // Create a test CoT event
            let original_event = create_test_event("u-r-loc");

            // Convert to Ditto document and back to CoT
            let doc = cot_to_document(&original_event, "test-peer");
            let round_tripped = doc.to_cot_event();

            // Check that key fields are preserved
            assert_eq!(round_tripped.uid, original_event.uid);
            assert_eq!(round_tripped.event_type, original_event.event_type);
            assert_eq!(round_tripped.how, original_event.how);
            assert_eq!(round_tripped.point.lat, original_event.point.lat);
            assert_eq!(round_tripped.point.lon, original_event.point.lon);
            assert_eq!(round_tripped.point.hae, original_event.point.hae);

            // Check that the callsign is preserved in the detail
            assert!(round_tripped.detail.contains(&original_event.detail));
        }
    }
}
