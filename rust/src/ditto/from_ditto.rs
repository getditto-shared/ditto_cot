//! Convert CotDocument back into CotEvent for round-trip tests
use crate::cot_events::CotEvent;
use crate::ditto::{CotDocument, File, FileRValue};
use chrono::{DateTime, TimeZone, Utc};

/// Convert a CotDocument back into a CotEvent (best-effort mapping for round-trip tests)
///
/// This function attempts to reconstruct a CotEvent from a CotDocument with the best possible
/// fidelity. Not all fields may be perfectly preserved in the round-trip conversion due to
/// differences in the data models.
pub fn cot_event_from_ditto_document(doc: &CotDocument) -> CotEvent {
    use crate::cot_events::Point;

    /// Helper to safely convert microseconds since epoch to DateTime<Utc>
    /// Note: We use timestamp_micros to handle microsecond precision
    fn millis_to_datetime(micros: i64) -> DateTime<Utc> {
        // Convert microseconds to seconds and nanoseconds
        let secs = micros / 1_000_000;
        let nanos = ((micros % 1_000_000) * 1_000) as u32;

        // Use timestamp_opt for better error handling
        Utc.timestamp_opt(secs, nanos).single().unwrap_or_else(|| {
            eprintln!(
                "WARN: Failed to convert timestamp {} to DateTime<Utc>",
                micros
            );
            Utc::now()
        })
    }

    match doc {
        CotDocument::Api(api) => CotEvent {
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
            // Serialize detail map to XML for round-trip fidelity
            detail: {
                use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                use crate::xml_writer::to_cot_xml;
                let flat = flat_cot_event_from_ditto(doc);
                // Extract only the <detail>...</detail> section
                let xml = to_cot_xml(&flat);
                // Find <detail>...</detail>
                let start = xml.find("<detail>").unwrap_or(0);
                let end = xml.find("</detail>").map(|i| i+"</detail>".len()).unwrap_or(xml.len());
                xml[start..end].to_string()
            },
        },
        CotDocument::Chat(chat) => CotEvent {
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
            // Serialize detail map to XML for round-trip fidelity
            detail: {
                use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                use crate::xml_writer::to_cot_xml;
                let flat = flat_cot_event_from_ditto(doc);
                let xml = to_cot_xml(&flat);
                let start = xml.find("<detail>").unwrap_or(0);
                let end = xml.find("</detail>").map(|i| i+"</detail>".len()).unwrap_or(xml.len());
                xml[start..end].to_string()
            },
        },
        CotDocument::File(file) => {
            // Extract the ce value from the _ce field in the detail map if it exists
            let ce = match &file.r.get("_ce") {
                Some(FileRValue::Number(n)) => *n,
                Some(FileRValue::String(s)) => s.parse::<f64>().unwrap_or(0.0),
                _ => 0.0, // Default if not found
            };
            
            // Extract timestamp values from the detail map if they exist
            let time = match &file.r.get("_time") {
                Some(FileRValue::String(s)) => {
                    match s.parse::<DateTime<Utc>>() {
                        Ok(dt) => dt,
                        Err(_) => if file.n != 0 { millis_to_datetime(file.n / 1000) } else { Utc::now() }
                    }
                },
                _ => if file.n != 0 { millis_to_datetime(file.n / 1000) } else { Utc::now() }
            };
            
            let start = match &file.r.get("_start") {
                Some(FileRValue::String(s)) => {
                    match s.parse::<DateTime<Utc>>() {
                        Ok(dt) => dt,
                        Err(_) => time.clone() // Default to time if parsing fails
                    }
                },
                _ => time.clone() // Default to time if not found
            };
            
            let stale = match &file.r.get("_stale") {
                Some(FileRValue::String(s)) => {
                    match s.parse::<DateTime<Utc>>() {
                        Ok(dt) => dt,
                        Err(_) => if file.o != 0 { millis_to_datetime(file.o / 1000) } else { time + chrono::Duration::minutes(30) }
                    }
                },
                _ => if file.o != 0 { millis_to_datetime(file.o / 1000) } else { time + chrono::Duration::minutes(30) }
            };
            
            // Create a copy of the detail map without the special fields for serialization
            let mut detail_map = file.r.clone();
            detail_map.remove("_ce"); // Remove the special fields so they don't appear in the XML
            detail_map.remove("_time");
            detail_map.remove("_start");
            detail_map.remove("_stale");
            
            // Create a modified File with the cleaned detail map for XML generation
            let modified_file = CotDocument::File(File {
                r: detail_map,
                ..file.clone()
            });
            
            CotEvent {
                version: "2.0".to_string(),
                uid: file.id.clone(),
                event_type: file.w.clone(),
                time,
                start,
                stale,
                how: file.p.clone(),
                point: Point {
                    lat: file.h.unwrap_or(0.0),
                    lon: file.i.unwrap_or(0.0),
                    hae: file.j.unwrap_or(0.0),
                    ce, // Use the extracted ce value
                    le: file.k.unwrap_or(0.0),
                },
                // Serialize detail map to XML for round-trip fidelity
                detail: {
                    use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                    use crate::xml_writer::to_cot_xml;
                    let flat = flat_cot_event_from_ditto(&modified_file); // Use the modified document
                    let xml = to_cot_xml(&flat);
                    let start = xml.find("<detail>").unwrap_or(0);
                    let end = xml.find("</detail>").map(|i| i+"</detail>".len()).unwrap_or(xml.len());
                    xml[start..end].to_string()
                },
            }
        },
        CotDocument::MapItem(map_item) => CotEvent {
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
                ce: map_item.b,
                le: map_item.k.unwrap_or(0.0),
            },
            // Serialize detail map to XML for round-trip fidelity
            detail: {
                use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                use crate::xml_writer::to_cot_xml;
                let flat = flat_cot_event_from_ditto(doc);
                let xml = to_cot_xml(&flat);
                let start = xml.find("<detail>").unwrap_or(0);
                let end = xml.find("</detail>").map(|i| i+"</detail>".len()).unwrap_or(xml.len());
                xml[start..end].to_string()
            },
        },
    }
}
