//! Convert DittoDocument back into CotEvent for round-trip tests
use crate::cot_events::CotEvent;
use crate::ditto::DittoDocument;
use chrono::{DateTime, TimeZone, Utc};

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

    match doc {
        DittoDocument::Api(api) => CotEvent {
            version: "2.0".to_string(),
            uid: api.id.clone(),
            event_type: api.w.clone(),
            time: if api.n != 0 {
                millis_to_datetime(api.n)
            } else {
                Utc::now()
            },
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
            // Preserve detail exactly as in Ditto document, trimming whitespace
            detail: api.r.clone(),
        },
        DittoDocument::Chat(chat) => CotEvent {
            version: "2.0".to_string(),
            uid: chat.id.clone(),
            event_type: chat.w.clone(),
            time: if chat.n != 0 {
                millis_to_datetime(chat.n)
            } else {
                Utc::now()
            },
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
            detail: chat.r.clone(),
        },
        DittoDocument::File(file) => CotEvent {
            version: "2.0".to_string(),
            uid: file.id.clone(),
            event_type: file.w.clone(),
            time: if file.n != 0 {
                millis_to_datetime(file.n)
            } else {
                Utc::now()
            },
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
            detail: file.r.clone(),
        },
        DittoDocument::MapItem(map_item) => CotEvent {
            version: "2.0".to_string(),
            uid: map_item.id.clone(),
            event_type: map_item.w.clone(),
            time: if map_item.n != 0 {
                millis_to_datetime(map_item.n)
            } else {
                Utc::now()
            },
            start: millis_to_datetime(map_item.n),
            stale: millis_to_datetime(map_item.o),
            how: map_item.p.clone(),
            point: Point {
                lat: map_item.h.unwrap_or(0.0),
                lon: map_item.i.unwrap_or(0.0),
                hae: map_item.j.unwrap_or(0.0),
                ce: map_item.k.unwrap_or(0.0),
                le: map_item.k.unwrap_or(0.0),
            },
            detail: map_item.r.clone(),
        },
    }
}
