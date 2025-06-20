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

    /// Helper to safely convert milliseconds since epoch to DateTime<Utc>
    fn millis_to_datetime(millis: i64) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(millis)
            .single()
            .unwrap_or_else(Utc::now)
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

            // Add any additional metadata that might be useful
            detail.insert("source".to_string(), "ditto_cot".to_string());
            detail.insert("original_type".to_string(), "api".to_string());

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

            // Add any additional metadata
            detail.insert("source".to_string(), "ditto_cot".to_string());
            detail.insert("original_type".to_string(), "chat".to_string());

            // No mime field in Chat struct

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

            // Add any additional metadata
            detail.insert("source".to_string(), "ditto_cot".to_string());
            detail.insert("original_type".to_string(), "file".to_string());

            // Use start (n) for both time and start for roundtrip fidelity
            let start_dt = millis_to_datetime(file.n);

            CotEvent {
                version: "2.0".to_string(),
                uid: file.id.clone(),
                event_type: file.w.clone(),
                time: start_dt,
                start: start_dt,
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

            // Map standard fields
            detail.insert("callsign".to_string(), map_item.e.clone());

            // Map map item specific fields
            add_opt_detail(&mut detail, "name", &map_item.c);

            // Add the event type to the detail map
            detail.insert("type".to_string(), map_item.w.clone());

            // Add the visibility field if it exists
            if let Some(visible) = map_item.f {
                detail.insert("visible".to_string(), visible.to_string());
            }

            // Add any additional metadata
            detail.insert("source".to_string(), "ditto_cot".to_string());
            detail.insert("original_type".to_string(), "map_item".to_string());

            // No mime field in MapItem struct

            // Use start (n) for both time and start for roundtrip fidelity
            let start_dt = millis_to_datetime(map_item.n);

            CotEvent {
                version: "2.0".to_string(),
                uid: map_item.id.clone(),
                event_type: map_item.w.clone(),
                time: start_dt,
                start: start_dt,
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
