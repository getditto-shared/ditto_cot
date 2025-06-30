//! Ditto integration for CoT events
//!
//! This module provides functionality to transform CoT (Cursor on Target) events
//! into Ditto documents according to the Ditto JSON schemas.

use crate::detail_parser::parse_detail_section;

use crate::cot_events::CotEvent;

use anyhow;
use serde::{Deserialize, Serialize};
// No unused imports remaining

pub use super::schema::*;

/// Convert a CoT event to the appropriate Ditto document type
pub fn cot_to_document(event: &CotEvent, peer_key: &str) -> CotDocument {
    let event_type = &event.event_type;

    if event_type == "a-u-emergency-g" {
        // Handle emergency events
        CotDocument::Api(transform_emergency_event(event, peer_key))
    } else if event_type.contains("b-t-f") || event_type.contains("chat") {
        // Handle chat events
        match transform_chat_event(event, peer_key) {
            Some(chat_doc) => CotDocument::Chat(chat_doc),
            None => CotDocument::File(transform_generic_event(event, peer_key)),
        }
    } else if event_type.contains("a-u-r-loc-g")
        || event_type.contains("a-f-G-U-C")
        || event_type.contains("a-f-G-U")
        || event_type.contains("a-f-G-U-I")
        || event_type.contains("a-f-G-U-T")
    {
        // Handle location update events
        CotDocument::MapItem(transform_location_event(event, peer_key))
    } else {
        // Fall back to generic document for all other event types
        CotDocument::File(transform_generic_event(event, peer_key))
    }
}

/// Transform a location CoT event to a Ditto location document
pub fn transform_location_event(event: &CotEvent, peer_key: &str) -> MapItem {
    // Map CotEvent and peer_key to MapItem fields
    MapItem {
        id: event.uid.clone(),                   // Ditto document ID
        a: peer_key.to_string(),                 // Ditto peer key string
        b: event.point.ce,                       // Circular error (ce) value
        c: None,                                 // Name/title not parsed from raw detail string
        d: event.uid.clone(),                    // TAK UID of author
        d_c: 0,                                  // Document counter (updates), default to 0
        d_r: false,                              // Soft-delete flag, default to false
        d_v: 2,                                  // Schema version (2)
        source: None,                            // Source not parsed from raw detail string
        e: String::new(),                        // Callsign not parsed from raw detail string
        f: None,                                 // Visibility flag
        g: "".to_string(),                       // Version string, default empty
        h: Some(event.point.lat),                // Latitude
        i: Some(event.point.lon),                // Longitude
        j: Some(event.point.hae),                // Altitude
        k: Some(event.point.le),                 // Linear Error
        l: None,                                 // Course (not in CotEvent)
        n: event.start.timestamp_micros(),       // Start (microsecond precision)
        o: event.stale.timestamp_micros(),       // Stale (microsecond precision)
        p: event.how.clone(),                    // How
        q: "".to_string(),                       // Access, default empty
        r: {
            let extras = parse_detail_section(&event.detail);
            extras.into_iter().map(|(k, v)| (k, match v {
                serde_json::Value::String(s) => MapItemRValue::String(s),
                serde_json::Value::Bool(b) => MapItemRValue::from(b),
                serde_json::Value::Number(n) => MapItemRValue::from(n.as_f64().unwrap_or(0.0)),
                serde_json::Value::Object(obj) => MapItemRValue::from(obj.clone()),
                serde_json::Value::Array(arr) => MapItemRValue::from(arr.clone()),
                _ => MapItemRValue::from(false),
            })).collect()
        },
        s: "".to_string(),                       // Opex, default empty
        t: "".to_string(),                       // Qos, default empty
        u: "".to_string(),                       // Caveat, default empty
        v: "".to_string(),                       // Releasable to, default empty
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
        parts.first().map(|s| s.to_string())
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
        // Parse detail XML into map for CRDT support
        r: {
            let extras = parse_detail_section(&event.detail);
            extras.into_iter().map(|(k, v)| (k, match v {
                serde_json::Value::String(s) => ChatRValue::String(s),
                serde_json::Value::Bool(b) => ChatRValue::from(b),
                serde_json::Value::Number(n) => ChatRValue::from(n.as_f64().unwrap_or(0.0)),
                serde_json::Value::Object(obj) => {
                    let map = serde_json::Map::from_iter(obj.clone());
                    ChatRValue::Object(map)
                },
                serde_json::Value::Array(arr) => ChatRValue::Array(arr.clone()),
                _ => ChatRValue::Null,
            })).collect()
        },
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
        // Parse detail XML into map for CRDT support
        r: {
            let extras = parse_detail_section(&event.detail);
            extras.into_iter().map(|(k, v)| (k, match v {
                serde_json::Value::String(s) => ApiRValue::String(s),
                serde_json::Value::Bool(b) => ApiRValue::from(b),
                serde_json::Value::Number(n) => ApiRValue::from(n.as_f64().unwrap_or(0.0)),
                serde_json::Value::Object(obj) => {
                    let map = serde_json::Map::from_iter(obj.clone());
                    ApiRValue::Object(map)
                },
                serde_json::Value::Array(arr) => ApiRValue::Array(arr.clone()),
                _ => ApiRValue::Null,
            })).collect()
        },
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

    // Store the circular error in a special key in the r map to avoid field overloading
    let mut extras = parse_detail_section(&event.detail);
    // Add ce as a special field in the detail map to preserve it during round-trip
    extras.insert("_ce".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0))));
    
    // Store timestamps in microseconds for better precision
    let time_micros = event.time.timestamp_micros();
    let _start_micros = event.start.timestamp_micros();
    let stale_micros = event.stale.timestamp_micros();
    
    // Store timestamp values in special fields in the detail map to preserve them during round-trip
    extras.insert("_time".to_string(), serde_json::Value::String(event.time.to_rfc3339()));
    extras.insert("_start".to_string(), serde_json::Value::String(event.start.to_rfc3339()));
    extras.insert("_stale".to_string(), serde_json::Value::String(event.stale.to_rfc3339()));
    
    File {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: 0.0, // We're not using b for time anymore to avoid field overloading
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
        k: Some(event.point.le), // Store le properly
        l: None,
        item_id,
        mime,
        n: time_micros, // Store time in microseconds
        o: stale_micros, // Store stale in microseconds
        p: event.how.clone(),
        q: "".to_string(),
        // Parse detail XML into map for CRDT support
        r: {
            extras.into_iter().map(|(k, v)| (k, match v {
                serde_json::Value::String(s) => FileRValue::String(s),
                serde_json::Value::Bool(b) => FileRValue::from(b),
                serde_json::Value::Number(n) => FileRValue::from(n.as_f64().unwrap_or(0.0)),
                serde_json::Value::Object(obj) => {
                    let map = serde_json::Map::from_iter(obj.clone());
                    FileRValue::Object(map)
                },
                serde_json::Value::Array(arr) => FileRValue::Array(arr.clone()),
                _ => FileRValue::Null,
            })).collect()
        },
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
pub enum CotDocument {
    /// API document type
    Api(Api),
    /// Chat message document type
    Chat(Chat),
    /// File document type
    File(File),
    /// Map item document type
    MapItem(MapItem),
}

impl CotDocument {
    /// Returns true if this is a MapItem variant
    pub fn is_map_item(&self) -> bool {
        matches!(self, CotDocument::MapItem(_))
    }

    /// Returns a reference to the inner MapItem if this is a MapItem variant
    pub fn as_map_item(&self) -> Option<&MapItem> {
        if let CotDocument::MapItem(item) = self {
            Some(item)
        } else {
            None
        }
    }

    /// Returns true if this document has the specified key in its top-level fields
    /// This is a simplified implementation that only checks a few common fields
    pub fn has_key(&self, key: &str) -> bool {
        match self {
            CotDocument::Api(_api) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                _ => false,
            },
            CotDocument::Chat(_chat) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                "e" => true, // callsign
                _ => false,
            },
            CotDocument::File(_file) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                "e" => true, // callsign
                _ => false,
            },
            CotDocument::MapItem(_map_item) => match key {
                "_id" => true,
                "a" => true, // peer_key
                "b" => true, // ce
                "d" => true, // uid
                "e" => true, // callsign
                _ => false,
            },
        }
    }

    /// Deserialize a JSON string into a CotDocument, determining the variant based on the 'w' field.
    /// Handles defaults for missing fields in variants.
    pub fn from_json_str(json_str: &str) -> Result<Self, anyhow::Error> {
        let json_value: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;
        let doc_type = json_value
            .get("w")
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
            Ok(CotDocument::MapItem(map_item))
        } else if doc_type.contains("b-t-f") || doc_type.contains("chat") {
            // Deserialize as Chat
            let chat: Chat = serde_json::from_value(json_value)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize as Chat: {}", e))?;
            Ok(CotDocument::Chat(chat))
        } else if doc_type == "a-u-emergency-g" {
            // Deserialize as Api
            let api: Api = serde_json::from_value(json_value)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize as Api: {}", e))?;
            Ok(CotDocument::Api(api))
        } else {
            // Default to File for unknown types
            let file: File = serde_json::from_value(json_value)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize as File: {}", e))?;
            Ok(CotDocument::File(file))
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
    /// # use ditto_cot::ditto::CotDocument;
    /// # use ditto_cot::cot_events::CotEvent;
    /// # fn example(doc: CotDocument) -> CotEvent {
    /// let cot_event = doc.to_cot_event();
    /// // Now you can work with the CoT event
    /// cot_event
    /// # }
    /// ```
    pub fn to_cot_event(&self) -> CotEvent {
        crate::ditto::from_ditto::cot_event_from_ditto_document(self)
    }
}

