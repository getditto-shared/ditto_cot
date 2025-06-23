//! Convert DittoDocument back into CotEvent for round-trip tests
use crate::cot_events::CotEvent;
use crate::ditto::DittoDocument;
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;

/// Convert a DittoDocument back into a CotEvent (best-effort mapping for round-trip tests)
///
/// This function attempts to reconstruct a CotEvent from a DittoDocument with the best possible
/// fidelity. Not all fields may be perfectly preserved in the round-trip conversion due to
/// differences in the data models.
pub fn cot_event_from_ditto_document(doc: &DittoDocument) -> CotEvent {
    use crate::cot_events::Point;

    /// Helper to safely convert microseconds since epoch to DateTime<Utc>
    /// Note: We use timestamp_micros to handle microsecond precision
    fn millis_to_datetime(micros: i64) -> DateTime<Utc> {
        // Convert microseconds to seconds and nanoseconds
        let secs = micros / 1_000_000;
        let nanos = ((micros % 1_000_000) * 1_000) as u32;
        
        // Use timestamp_opt for better error handling
        Utc.timestamp_opt(secs, nanos)
            .single()
            .unwrap_or_else(|| {
                eprintln!("WARN: Failed to convert timestamp {} to DateTime<Utc>", micros);
                Utc::now()
            })
    }

    /// Helper to add optional string field to detail map if it exists
    fn add_opt_detail(detail: &mut HashMap<String, String>, key: &str, value: &Option<String>) {
        if let Some(v) = value {
            detail.insert(key.to_string(), v.clone());
        }
    }

    /// Helper to add optional numeric field to detail map if it exists
    fn add_opt_num_detail<T: ToString>(
        detail: &mut HashMap<String, String>,
        key: &str,
        value: &Option<T>,
    ) {
        if let Some(v) = value {
            detail.insert(key.to_string(), v.to_string());
        }
    }

    match doc {
        DittoDocument::Api(api) => {
            let mut detail = HashMap::new();

            // Map standard fields
            detail.insert("callsign".to_string(), api.e.clone());

            // Map optional fields
            add_opt_detail(&mut detail, "type", &api.title);
            add_opt_detail(&mut detail, "message", &api.data);
            add_opt_detail(&mut detail, "mime", &api.mime);
            add_opt_detail(&mut detail, "status", &api.tag);
            add_opt_detail(&mut detail, "contentType", &api.content_type);
            add_opt_num_detail(&mut detail, "timeMillis", &api.time_millis);

            // Add source field if it exists in the API document
            if let Some(source) = &api.source {
                detail.insert("source".to_string(), source.clone());
            }

            CotEvent {
                version: "2.0".to_string(),
                uid: api.id.clone(),
                event_type: api.w.clone(),
                time: api
                    .time_millis
                    .map_or_else(Utc::now, millis_to_datetime),
                start: millis_to_datetime(api.n),
                stale: millis_to_datetime(api.o),
                how: api.p.clone(),
                point: Point {
                    lat: api.h.unwrap_or(0.0),
                    lon: api.i.unwrap_or(0.0),
                    hae: api.j.unwrap_or(0.0),
                    ce: api.b,
                    le: api.k.unwrap_or(0.0),
                },
                detail,
            }
        }
        DittoDocument::Chat(chat) => {
            let mut detail = HashMap::new();

            // Map standard fields
            detail.insert("callsign".to_string(), chat.e.clone());

            // Map optional chat-specific fields
            add_opt_detail(&mut detail, "chat", &chat.message);
            add_opt_detail(&mut detail, "chatroom", &chat.room);
            add_opt_detail(&mut detail, "chat_group_uid", &chat.room_id);
            add_opt_detail(&mut detail, "author_callsign", &chat.author_callsign);
            add_opt_detail(&mut detail, "author_type", &chat.author_type);
            add_opt_detail(&mut detail, "author_uid", &chat.author_uid);
            add_opt_detail(&mut detail, "location", &chat.location);
            add_opt_detail(&mut detail, "parent", &chat.parent);
            add_opt_detail(&mut detail, "time", &chat.time);

            // Add source field if it exists in the chat document
            if let Some(source) = &chat.source {
                detail.insert("source".to_string(), source.clone());
            }

            CotEvent {
                version: "2.0".to_string(),
                uid: chat.id.clone(),
                event_type: chat.w.clone(),
                time: chat
                    .time
                    .as_ref()
                    .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
                start: millis_to_datetime(chat.n),
                stale: millis_to_datetime(chat.o),
                how: chat.p.clone(),
                point: Point {
                    lat: chat.h.unwrap_or(0.0),
                    lon: chat.i.unwrap_or(0.0),
                    hae: chat.j.unwrap_or(0.0),
                    ce: chat.b,
                    le: chat.k.unwrap_or(0.0),
                },
                detail,
            }
        }
        DittoDocument::File(file) => {
            let mut detail = HashMap::new();

            // Map standard fields
            detail.insert("callsign".to_string(), file.e.clone());

            // Map file-specific fields
            add_opt_detail(&mut detail, "file_name", &file.c);
            add_opt_detail(&mut detail, "file_token", &file.file);
            add_opt_detail(&mut detail, "mime", &file.mime);
            add_opt_detail(&mut detail, "contentType", &file.content_type);
            add_opt_detail(&mut detail, "item_id", &file.item_id);
            add_opt_num_detail(&mut detail, "size", &file.sz);

            // Don't add default metadata fields to preserve original detail fields
            
            // Add source field if it exists in the file
            if let Some(source) = &file.source {
                detail.insert("source".to_string(), source.clone());
            }

            CotEvent {
                version: "2.0".to_string(),
                uid: file.id.clone(),
                event_type: file.w.clone(),
                time: millis_to_datetime(file.n),
                start: millis_to_datetime(file.n),
                stale: millis_to_datetime(file.o),
                how: file.p.clone(),
                point: Point {
                    lat: file.h.unwrap_or(0.0),
                    lon: file.i.unwrap_or(0.0),
                    hae: file.j.unwrap_or(0.0),
                    ce: file.b,
                    le: file.k.unwrap_or(0.0),
                },
                detail,
            }
        }
        DittoDocument::MapItem(map_item) => {
            let mut detail = HashMap::new();

            // Only add callsign if it's not empty
            if !map_item.e.is_empty() {
                detail.insert("callsign".to_string(), map_item.e.clone());
            }

            // Only add name if it exists and is not empty
            if let Some(name) = &map_item.c {
                if !name.is_empty() {
                    detail.insert("name".to_string(), name.clone());
                }
            }

            // Only add type to detail if it was in the original event's detail map
            // We don't add it by default to match the original event

            // Add the visibility field if it exists and is true
            if let Some(visible) = map_item.f {
                if visible {
                    detail.insert("visible".to_string(), "true".to_string());
                }
            }

            // Add source field if it exists in the map item
            if let Some(source) = &map_item.source {
                detail.insert("source".to_string(), source.clone());
            }

            CotEvent {
                version: "2.0".to_string(),
                uid: map_item.id.clone(),
                event_type: map_item.w.clone(),
                time: millis_to_datetime(map_item.n),
                start: millis_to_datetime(map_item.n),
                stale: millis_to_datetime(map_item.o),
                how: map_item.p.clone(),
                point: Point {
                    lat: map_item.h.unwrap_or(0.0),
                    lon: map_item.i.unwrap_or(0.0),
                    hae: map_item.j.unwrap_or(0.0),
                    ce: map_item.b,
                    le: map_item.k.unwrap_or(0.0),
                },
                detail,
            }
        }
    }
}
