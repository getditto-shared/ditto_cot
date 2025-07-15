//! Ditto integration for CoT events
//!
//! This module provides functionality to transform CoT (Cursor on Target) events
//! into Ditto documents according to the Ditto JSON schemas.

use crate::cot_events::CotEvent;
use crate::detail_parser::parse_detail_section;
use crate::ditto::r_field_flattening::flatten_document_r_field;

use anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// No unused imports remaining

pub use super::schema::*;

// Removed unused imports

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
            None => CotDocument::Generic(transform_generic_event(event, peer_key)),
        }
    } else if event_type.contains("a-u-r-loc-g")
        || event_type.contains("a-f-G-U-C")
        || event_type.contains("a-f-G-U")
        || event_type.contains("a-f-G-U-I")
        || event_type.contains("a-f-G-U-T")
        || event_type.contains("a-u-S")
        || event_type.contains("a-u-A")
        || event_type.contains("a-u-G")
    {
        // Handle location update events
        CotDocument::MapItem(transform_location_event(event, peer_key))
    } else if event_type.contains("file") || event_type.contains("attachment") {
        // Handle file events
        CotDocument::File(transform_file_event(event, peer_key))
    } else {
        // Fall back to generic document for all other event types
        CotDocument::Generic(transform_generic_event(event, peer_key))
    }
}

/// Convert a CoT event to a flattened Ditto document for DQL compatibility
pub fn cot_to_flattened_document(event: &CotEvent, peer_key: &str) -> Value {
    let event_type = &event.event_type;

    if event_type == "a-u-emergency-g" {
        // Handle emergency events
        transform_emergency_event_flattened(event, peer_key)
    } else if event_type.contains("b-t-f") || event_type.contains("chat") {
        // Handle chat events
        match transform_chat_event_flattened(event, peer_key) {
            Some(chat_doc) => chat_doc,
            None => transform_generic_event_flattened(event, peer_key),
        }
    } else if event_type.contains("a-u-r-loc-g")
        || event_type.contains("a-f-G-U-C")
        || event_type.contains("a-f-G-U")
        || event_type.contains("a-f-G-U-I")
        || event_type.contains("a-f-G-U-T")
        || event_type.contains("a-u-S")
        || event_type.contains("a-u-A")
        || event_type.contains("a-u-G")
    {
        // Handle location update events
        transform_location_event_flattened(event, peer_key)
    } else if event_type.contains("file") || event_type.contains("attachment") {
        // Handle file events
        transform_file_event_flattened(event, peer_key)
    } else {
        // Fall back to generic document for all other event types
        transform_generic_event_flattened(event, peer_key)
    }
}

/// Transform a location CoT event to a Ditto location document
pub fn transform_location_event(event: &CotEvent, peer_key: &str) -> MapItem {
    // Map CotEvent and peer_key to MapItem fields
    MapItem {
        id: event.uid.clone(),                          // Ditto document ID
        a: peer_key.to_string(),                        // Ditto peer key string
        b: event.point.ce,                              // Circular error (ce) value
        c: None,                  // Name/title not parsed from raw detail string
        d: event.uid.clone(),     // TAK UID of author
        d_c: 0,                   // Document counter (updates), default to 0
        d_r: false,               // Soft-delete flag, default to false
        d_v: 2,                   // Schema version (2)
        source: None,             // Source not parsed from raw detail string
        e: String::new(),         // Callsign not parsed from raw detail string
        f: None,                  // Visibility flag
        g: "".to_string(),        // Version string, default empty
        h: Some(event.point.ce),  // Circular Error
        i: Some(event.point.hae), // Height Above Ellipsoid
        j: Some(event.point.lat), // Latitude
        k: Some(event.point.le),  // Linear Error
        l: Some(event.point.lon), // Longitude
        n: Some(event.start.timestamp_micros() as f64), // Start (microseconds)
        o: Some(event.stale.timestamp_micros() as f64), // Stale (microseconds)
        p: event.how.clone(),     // How
        q: "".to_string(),        // Access, default empty
        r: HashMap::new(),        // Empty - will use flattened r_* fields
        s: "".to_string(),        // Opex, default empty
        t: "".to_string(),        // Qos, default empty
        u: "".to_string(),        // Caveat, default empty
        v: "".to_string(),        // Releasable to, default empty
        w: event.event_type.clone(), // Type
    }
}

/// Transform a location CoT event to a flattened JSON value for DQL compatibility
pub fn transform_location_event_flattened(event: &CotEvent, peer_key: &str) -> Value {
    // Parse detail section and flatten r field for DQL compatibility
    let extras = parse_detail_section(&event.detail);

    // Create base document as a HashMap for flattening
    let mut base_doc = HashMap::new();
    base_doc.insert("_id".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("a".to_string(), Value::String(peer_key.to_string()));
    base_doc.insert(
        "b".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("d".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("_c".to_string(), Value::Number(serde_json::Number::from(0)));
    base_doc.insert("_r".to_string(), Value::Bool(false));
    base_doc.insert("_v".to_string(), Value::Number(serde_json::Number::from(2)));
    base_doc.insert("e".to_string(), Value::String(String::new()));
    base_doc.insert("g".to_string(), Value::String("".to_string()));
    base_doc.insert(
        "h".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "i".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.hae).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "j".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lat).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "k".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.le).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "l".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lon).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "n".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.start.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "o".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.stale.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("p".to_string(), Value::String(event.how.clone()));
    base_doc.insert("q".to_string(), Value::String("".to_string()));
    base_doc.insert("s".to_string(), Value::String("".to_string()));
    base_doc.insert("t".to_string(), Value::String("".to_string()));
    base_doc.insert("u".to_string(), Value::String("".to_string()));
    base_doc.insert("v".to_string(), Value::String("".to_string()));
    base_doc.insert("w".to_string(), Value::String(event.event_type.clone()));

    // Apply flattening to the r field
    flatten_document_r_field(&mut base_doc, &extras);

    // Convert to JSON Value
    Value::Object(base_doc.into_iter().collect())
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
        n: Some(event.start.timestamp_micros() as f64),
        o: Some(event.stale.timestamp_micros() as f64),
        p: event.how.clone(),
        q: "".to_string(),
        // Empty r field - will use flattened r_* fields
        r: HashMap::new(),
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

/// Transform a chat CoT event to a flattened JSON value for DQL compatibility
pub fn transform_chat_event_flattened(event: &CotEvent, peer_key: &str) -> Option<Value> {
    // Extract chat message and room from event.detail
    let parts: Vec<&str> = event.detail.split_whitespace().collect();
    let message = if parts.len() >= 2 {
        Some(format!("{} {}", parts[0], parts[1]))
    } else {
        parts.first().map(|s| s.to_string())
    };
    let room = parts.get(2).map(|s| s.to_string());
    let location = Some(format!(
        "{},{},{}",
        event.point.lat, event.point.lon, event.point.hae
    ));

    // If there's no message, return None
    message.as_ref()?;

    // Parse detail section and flatten r field for DQL compatibility
    let extras = parse_detail_section(&event.detail);

    // Create base document as a HashMap for flattening
    let mut base_doc = HashMap::new();
    base_doc.insert("_id".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("a".to_string(), Value::String(peer_key.to_string()));
    base_doc.insert(
        "b".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("d".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("_c".to_string(), Value::Number(serde_json::Number::from(0)));
    base_doc.insert("_r".to_string(), Value::Bool(false));
    base_doc.insert("_v".to_string(), Value::Number(serde_json::Number::from(2)));
    base_doc.insert("e".to_string(), Value::String(String::new()));
    base_doc.insert("g".to_string(), Value::String("".to_string()));
    base_doc.insert(
        "h".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lat).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "i".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lon).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "j".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.hae).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "n".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.start.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "o".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.stale.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("p".to_string(), Value::String(event.how.clone()));
    base_doc.insert("q".to_string(), Value::String("".to_string()));
    base_doc.insert("s".to_string(), Value::String("".to_string()));
    base_doc.insert("t".to_string(), Value::String("".to_string()));
    base_doc.insert("u".to_string(), Value::String("".to_string()));
    base_doc.insert("v".to_string(), Value::String("".to_string()));
    base_doc.insert("w".to_string(), Value::String(event.event_type.clone()));
    base_doc.insert("time".to_string(), Value::String(event.time.to_rfc3339()));

    // Add chat-specific fields
    if let Some(msg) = message {
        base_doc.insert("message".to_string(), Value::String(msg));
    }
    if let Some(r) = room {
        base_doc.insert("room".to_string(), Value::String(r));
    }
    if let Some(loc) = location {
        base_doc.insert("location".to_string(), Value::String(loc));
    }
    base_doc.insert("authorUid".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("authorType".to_string(), Value::String("user".to_string()));

    // Apply flattening to the r field
    flatten_document_r_field(&mut base_doc, &extras);

    Some(Value::Object(base_doc.into_iter().collect()))
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
        n: Some(event.start.timestamp_micros() as f64),
        o: Some(event.stale.timestamp_micros() as f64),
        p: event.how.clone(),
        q: "".to_string(),
        // Empty r field - will use flattened r_* fields
        r: HashMap::new(),
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

/// Transform a file CoT event to a Ditto file document
fn transform_file_event(event: &CotEvent, peer_key: &str) -> File {
    let c = None;

    // Parse the detail section to extract file metadata
    let mut extras = parse_detail_section(&event.detail);

    // Extract filename from fileshare element if it exists
    let file = if let Some(fileshare) = extras.get("fileshare") {
        if let Some(filename) = fileshare.get("filename") {
            if let Some(name) = filename.as_str() {
                Some(name.to_string())
            } else {
                Some(event.uid.clone())
            }
        } else {
            Some(event.uid.clone())
        }
    } else {
        Some(event.uid.clone())
    };

    // Extract MIME type from fileshare element if it exists
    let mime = if let Some(fileshare) = extras.get("fileshare") {
        if let Some(mime_type) = fileshare.get("mime") {
            if let Some(m) = mime_type.as_str() {
                Some(m.to_string())
            } else {
                Some("application/octet-stream".to_string())
            }
        } else {
            Some("application/octet-stream".to_string())
        }
    } else {
        Some("application/octet-stream".to_string())
    };

    // Extract file size from fileshare element if it exists
    let sz = if let Some(fileshare) = extras.get("fileshare") {
        if let Some(size) = fileshare.get("size") {
            if let Some(s) = size.as_str() {
                s.parse::<f64>().ok().map(Some).unwrap_or(None)
            } else {
                size.as_f64()
            }
        } else {
            None
        }
    } else {
        None
    };

    let content_type = Some("file".to_string());
    let item_id = Some(event.uid.clone());

    // Store the circular error in a special key in the r map to avoid field overloading
    // Add ce as a special field in the detail map to preserve it during round-trip
    extras.insert(
        "_ce".to_string(),
        serde_json::Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );

    // Store timestamps in microseconds for better precision
    let time_micros = event.time.timestamp_micros();
    let stale_micros = event.stale.timestamp_micros();

    // Store timestamp values in special fields in the detail map to preserve them during round-trip
    extras.insert(
        "_time".to_string(),
        serde_json::Value::String(event.time.to_rfc3339()),
    );
    extras.insert(
        "_start".to_string(),
        serde_json::Value::String(event.start.to_rfc3339()),
    );
    extras.insert(
        "_stale".to_string(),
        serde_json::Value::String(event.stale.to_rfc3339()),
    );

    File {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: event.point.ce, // Store CE in b field for roundtrip compatibility
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
        n: Some(time_micros as f64),  // Store time in microseconds
        o: Some(stale_micros as f64), // Store stale in microseconds
        p: event.how.clone(),
        q: "".to_string(),
        // Empty r field - will use flattened r_* fields
        r: HashMap::new(),
        s: "".to_string(),
        sz,
        t: "".to_string(),
        u: "".to_string(),
        v: "".to_string(),
        w: event.event_type.clone(),
    }
}

/// Transform any CoT event to a generic Ditto document
fn transform_generic_event(event: &CotEvent, peer_key: &str) -> Generic {
    // Store the circular error in a special key in the r map to avoid field overloading
    let mut extras = parse_detail_section(&event.detail);
    // Add ce as a special field in the detail map to preserve it during round-trip
    extras.insert(
        "_ce".to_string(),
        serde_json::Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );

    // Store timestamps in microseconds for better precision
    let time_micros = event.time.timestamp_micros();
    let _start_micros = event.start.timestamp_micros();
    let stale_micros = event.stale.timestamp_micros();

    // Store timestamp values in special fields in the detail map to preserve them during round-trip
    extras.insert(
        "_time".to_string(),
        serde_json::Value::String(event.time.to_rfc3339()),
    );
    extras.insert(
        "_start".to_string(),
        serde_json::Value::String(event.start.to_rfc3339()),
    );
    extras.insert(
        "_stale".to_string(),
        serde_json::Value::String(event.stale.to_rfc3339()),
    );

    Generic {
        id: event.uid.clone(),
        a: peer_key.to_string(),
        b: event.point.ce, // Store CE in b field for roundtrip compatibility
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
        k: Some(event.point.le), // Store le properly
        l: None,
        n: Some(time_micros as f64),  // Store time in microseconds
        o: Some(stale_micros as f64), // Store stale in microseconds
        p: event.how.clone(),
        q: "".to_string(),
        // Empty r field - will use flattened r_* fields
        r: HashMap::new(),
        s: "".to_string(),
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
    /// Generic document type
    Generic(Generic),
    /// Map item document type
    MapItem(MapItem),
}

/// Transform an emergency CoT event to a flattened JSON value for DQL compatibility
pub fn transform_emergency_event_flattened(event: &CotEvent, peer_key: &str) -> Value {
    // Parse detail section and flatten r field for DQL compatibility
    let extras = parse_detail_section(&event.detail);

    // Create base document as a HashMap for flattening
    let mut base_doc = HashMap::new();
    base_doc.insert("_id".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("a".to_string(), Value::String(peer_key.to_string()));
    base_doc.insert(
        "b".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.time.timestamp_millis() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("d".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("_c".to_string(), Value::Number(serde_json::Number::from(0)));
    base_doc.insert("_r".to_string(), Value::Bool(false));
    base_doc.insert("_v".to_string(), Value::Number(serde_json::Number::from(2)));
    base_doc.insert("e".to_string(), Value::String(String::new()));
    base_doc.insert("g".to_string(), Value::String("".to_string()));
    base_doc.insert(
        "h".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lat).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "i".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lon).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "j".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.hae).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "n".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.start.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "o".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.stale.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("p".to_string(), Value::String(event.how.clone()));
    base_doc.insert("q".to_string(), Value::String("".to_string()));
    base_doc.insert("s".to_string(), Value::String("".to_string()));
    base_doc.insert("t".to_string(), Value::String("".to_string()));
    base_doc.insert("u".to_string(), Value::String("".to_string()));
    base_doc.insert("v".to_string(), Value::String("".to_string()));
    base_doc.insert("w".to_string(), Value::String(event.event_type.clone()));

    // Add API-specific fields
    base_doc.insert(
        "contentType".to_string(),
        Value::String("emergency".to_string()),
    );
    base_doc.insert(
        "mime".to_string(),
        Value::String("application/vnd.cot.emergency+json".to_string()),
    );
    base_doc.insert("isFile".to_string(), Value::Bool(false));
    base_doc.insert("isRemoved".to_string(), Value::Bool(false));
    base_doc.insert(
        "timeMillis".to_string(),
        Value::Number(serde_json::Number::from(event.time.timestamp_millis())),
    );

    // Apply flattening to the r field
    flatten_document_r_field(&mut base_doc, &extras);

    Value::Object(base_doc.into_iter().collect())
}

/// Transform a file CoT event to a flattened JSON value for DQL compatibility
pub fn transform_file_event_flattened(event: &CotEvent, peer_key: &str) -> Value {
    // Parse the detail section to extract file metadata
    let mut extras = parse_detail_section(&event.detail);

    // Extract filename from fileshare element if it exists
    let file = if let Some(fileshare) = extras.get("fileshare") {
        if let Some(filename) = fileshare.get("filename") {
            filename
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| event.uid.clone())
        } else {
            event.uid.clone()
        }
    } else {
        event.uid.clone()
    };

    // Extract MIME type
    let mime = if let Some(fileshare) = extras.get("fileshare") {
        if let Some(mime_type) = fileshare.get("mime") {
            mime_type
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string())
        } else {
            "application/octet-stream".to_string()
        }
    } else {
        "application/octet-stream".to_string()
    };

    // Extract file size
    let sz = if let Some(fileshare) = extras.get("fileshare") {
        if let Some(size) = fileshare.get("size") {
            if let Some(s) = size.as_str() {
                s.parse::<f64>().ok()
            } else {
                size.as_f64()
            }
        } else {
            None
        }
    } else {
        None
    };

    // Add metadata to extras
    extras.insert(
        "_ce".to_string(),
        serde_json::Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    extras.insert(
        "_time".to_string(),
        serde_json::Value::String(event.time.to_rfc3339()),
    );
    extras.insert(
        "_start".to_string(),
        serde_json::Value::String(event.start.to_rfc3339()),
    );
    extras.insert(
        "_stale".to_string(),
        serde_json::Value::String(event.stale.to_rfc3339()),
    );

    // Create base document as a HashMap for flattening
    let mut base_doc = HashMap::new();
    base_doc.insert("_id".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("a".to_string(), Value::String(peer_key.to_string()));
    base_doc.insert(
        "b".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("d".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("_c".to_string(), Value::Number(serde_json::Number::from(0)));
    base_doc.insert("_r".to_string(), Value::Bool(false));
    base_doc.insert("_v".to_string(), Value::Number(serde_json::Number::from(2)));
    base_doc.insert("e".to_string(), Value::String(String::new()));
    base_doc.insert("g".to_string(), Value::String("".to_string()));
    base_doc.insert(
        "h".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lat).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "i".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lon).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "j".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.hae).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "k".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.le).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "n".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.time.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "o".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.stale.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("p".to_string(), Value::String(event.how.clone()));
    base_doc.insert("q".to_string(), Value::String("".to_string()));
    base_doc.insert("s".to_string(), Value::String("".to_string()));
    base_doc.insert("t".to_string(), Value::String("".to_string()));
    base_doc.insert("u".to_string(), Value::String("".to_string()));
    base_doc.insert("v".to_string(), Value::String("".to_string()));
    base_doc.insert("w".to_string(), Value::String(event.event_type.clone()));

    // Add file-specific fields
    base_doc.insert("contentType".to_string(), Value::String("file".to_string()));
    base_doc.insert("file".to_string(), Value::String(file));
    base_doc.insert("mime".to_string(), Value::String(mime));
    base_doc.insert("itemId".to_string(), Value::String(event.uid.clone()));
    if let Some(size) = sz {
        base_doc.insert(
            "sz".to_string(),
            Value::Number(
                serde_json::Number::from_f64(size).unwrap_or(serde_json::Number::from(0)),
            ),
        );
    }

    // Apply flattening to the r field
    flatten_document_r_field(&mut base_doc, &extras);

    Value::Object(base_doc.into_iter().collect())
}

/// Transform any CoT event to a flattened generic Ditto document for DQL compatibility
pub fn transform_generic_event_flattened(event: &CotEvent, peer_key: &str) -> Value {
    // Store metadata in extras
    let mut extras = parse_detail_section(&event.detail);
    extras.insert(
        "_ce".to_string(),
        serde_json::Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    extras.insert(
        "_time".to_string(),
        serde_json::Value::String(event.time.to_rfc3339()),
    );
    extras.insert(
        "_start".to_string(),
        serde_json::Value::String(event.start.to_rfc3339()),
    );
    extras.insert(
        "_stale".to_string(),
        serde_json::Value::String(event.stale.to_rfc3339()),
    );

    // Create base document as a HashMap for flattening
    let mut base_doc = HashMap::new();
    base_doc.insert("_id".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("a".to_string(), Value::String(peer_key.to_string()));
    base_doc.insert(
        "b".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.ce).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("d".to_string(), Value::String(event.uid.clone()));
    base_doc.insert("_c".to_string(), Value::Number(serde_json::Number::from(0)));
    base_doc.insert("_r".to_string(), Value::Bool(false));
    base_doc.insert("_v".to_string(), Value::Number(serde_json::Number::from(2)));
    base_doc.insert("e".to_string(), Value::String(String::new()));
    base_doc.insert("g".to_string(), Value::String("".to_string()));
    base_doc.insert(
        "h".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lat).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "i".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.lon).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "j".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.hae).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "k".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.point.le).unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "n".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.time.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert(
        "o".to_string(),
        Value::Number(
            serde_json::Number::from_f64(event.stale.timestamp_micros() as f64)
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    base_doc.insert("p".to_string(), Value::String(event.how.clone()));
    base_doc.insert("q".to_string(), Value::String("".to_string()));
    base_doc.insert("s".to_string(), Value::String("".to_string()));
    base_doc.insert("t".to_string(), Value::String("".to_string()));
    base_doc.insert("u".to_string(), Value::String("".to_string()));
    base_doc.insert("v".to_string(), Value::String("".to_string()));
    base_doc.insert("w".to_string(), Value::String(event.event_type.clone()));

    // Apply flattening to the r field
    flatten_document_r_field(&mut base_doc, &extras);

    Value::Object(base_doc.into_iter().collect())
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

    /// Converts this CotDocument to a flattened JSON value for DQL compatibility
    pub fn to_flattened_json(&self) -> Value {
        match self {
            CotDocument::Api(api) => serde_json::to_value(api).unwrap_or(Value::Null),
            CotDocument::Chat(chat) => serde_json::to_value(chat).unwrap_or(Value::Null),
            CotDocument::File(file) => serde_json::to_value(file).unwrap_or(Value::Null),
            CotDocument::Generic(generic) => serde_json::to_value(generic).unwrap_or(Value::Null),
            CotDocument::MapItem(map_item) => serde_json::to_value(map_item).unwrap_or(Value::Null),
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
            CotDocument::Generic(_generic) => match key {
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

        if doc_type.contains("a-u-r-loc-g")
            || doc_type.contains("a-f-G-U-C")
            || doc_type.contains("a-f-G-U")
            || doc_type.contains("a-f-G-U-I")
            || doc_type.contains("a-f-G-U-T")
            || doc_type.contains("a-u-S")
            || doc_type.contains("a-u-A")
            || doc_type.contains("a-u-G")
        {
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
