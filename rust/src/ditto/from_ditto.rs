//! Convert CotDocument back into CotEvent for round-trip tests
use crate::cot_events::CotEvent;
use crate::ditto::r_field_flattening::unflatten_document_r_field;
use crate::ditto::{CotDocument, File, FileRValue};
use chrono::{DateTime, TimeZone, Utc};
use serde_json::Value;
use std::collections::HashMap;

/// Convert a CotDocument back into a CotEvent (best-effort mapping for round-trip tests)
///
/// This function attempts to reconstruct a CotEvent from a CotDocument with the best possible
/// fidelity. Not all fields may be perfectly preserved in the round-trip conversion due to
/// differences in the data models.
pub fn cot_event_from_ditto_document(doc: &CotDocument) -> CotEvent {
    use crate::cot_events::Point;

    /// Helper to safely convert microseconds since epoch to DateTime<Utc>
    fn micros_to_datetime(micros: i64) -> DateTime<Utc> {
        // Convert microseconds to seconds and nanoseconds
        let secs = micros / 1_000_000;
        let nanos = ((micros % 1_000_000) * 1_000) as u32;

        // Use timestamp_opt for better error handling
        Utc.timestamp_opt(secs, nanos).single().unwrap_or_else(|| {
            eprintln!(
                "WARN: Failed to convert timestamp {} microseconds to DateTime<Utc>",
                micros
            );
            Utc::now()
        })
    }

    match doc {
        CotDocument::Api(api) => CotEvent {
            version: api.g.clone(), // g = VERSION
            uid: api.id.clone(),
            event_type: api.w.clone(),
            time: micros_to_datetime(api.b as i64), // b = TIME in microseconds
            start: micros_to_datetime(api.n.unwrap_or(0.0) as i64),
            stale: micros_to_datetime(api.o.unwrap_or(0.0) as i64),
            how: api.p.clone(),
            point: Point {
                lat: api.j.unwrap_or(0.0), // j = LAT
                lon: api.l.unwrap_or(0.0), // l = LON
                hae: api.i.unwrap_or(0.0), // i = HAE
                ce: api.h.unwrap_or(0.0),  // h = CE
                le: api.k.unwrap_or(0.0),  // k = LE
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
                let end = xml
                    .find("</detail>")
                    .map(|i| i + "</detail>".len())
                    .unwrap_or(xml.len());
                xml[start..end].to_string()
            },
        },
        CotDocument::Chat(chat) => CotEvent {
            version: chat.g.clone(), // g = VERSION
            uid: chat.id.clone(),
            event_type: chat.w.clone(),
            time: micros_to_datetime(chat.b as i64), // b = TIME in microseconds
            start: micros_to_datetime(chat.n.unwrap_or(0.0) as i64),
            stale: micros_to_datetime(chat.o.unwrap_or(0.0) as i64),
            how: chat.p.clone(),
            point: Point {
                lat: chat.j.unwrap_or(0.0), // j = LAT
                lon: chat.l.unwrap_or(0.0), // l = LON
                hae: chat.i.unwrap_or(0.0), // i = HAE
                ce: chat.h.unwrap_or(0.0),  // h = CE
                le: chat.k.unwrap_or(0.0),  // k = LE
            },
            // Serialize detail map to XML for round-trip fidelity
            detail: {
                use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                use crate::xml_writer::to_cot_xml;
                let flat = flat_cot_event_from_ditto(doc);
                let xml = to_cot_xml(&flat);
                let start = xml.find("<detail>").unwrap_or(0);
                let end = xml
                    .find("</detail>")
                    .map(|i| i + "</detail>".len())
                    .unwrap_or(xml.len());
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

            // Extract timestamp values - use b field as primary source (contains time in microseconds)
            let time = if file.b != 0.0 {
                micros_to_datetime(file.b as i64) // b = TIME in microseconds
            } else {
                // Fallback to _time field in detail map
                match &file.r.get("_time") {
                    Some(FileRValue::String(s)) => match s.parse::<DateTime<Utc>>() {
                        Ok(dt) => dt,
                        Err(_) => Utc::now(),
                    },
                    _ => Utc::now(),
                }
            };

            let start = match &file.r.get("_start") {
                Some(FileRValue::String(s)) => {
                    match s.parse::<DateTime<Utc>>() {
                        Ok(dt) => dt,
                        Err(_) => time, // Default to time if parsing fails
                    }
                }
                _ => time, // Default to time if not found
            };

            let stale = match &file.r.get("_stale") {
                Some(FileRValue::String(s)) => match s.parse::<DateTime<Utc>>() {
                    Ok(dt) => dt,
                    Err(_) => {
                        if file.o.unwrap_or(0.0) != 0.0 {
                            micros_to_datetime(file.o.unwrap_or(0.0) as i64)
                        } else {
                            time + chrono::Duration::minutes(30)
                        }
                    }
                },
                _ => {
                    if file.o.unwrap_or(0.0) != 0.0 {
                        micros_to_datetime(file.o.unwrap_or(0.0) as i64)
                    } else {
                        time + chrono::Duration::minutes(30)
                    }
                }
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
                version: file.g.clone(), // g = VERSION
                uid: file.id.clone(),
                event_type: file.w.clone(),
                time,
                start,
                stale,
                how: file.p.clone(),
                point: Point {
                    lat: file.j.unwrap_or(0.0), // j = LAT
                    lon: file.l.unwrap_or(0.0), // l = LON
                    hae: file.i.unwrap_or(0.0), // i = HAE
                    ce,                         // Use the extracted ce value from _ce field
                    le: file.k.unwrap_or(0.0),  // k = LE
                },
                // Serialize detail map to XML for round-trip fidelity
                detail: {
                    use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                    use crate::xml_writer::to_cot_xml;
                    let flat = flat_cot_event_from_ditto(&modified_file); // Use the modified document
                    let xml = to_cot_xml(&flat);
                    let start = xml.find("<detail>").unwrap_or(0);
                    let end = xml
                        .find("</detail>")
                        .map(|i| i + "</detail>".len())
                        .unwrap_or(xml.len());
                    xml[start..end].to_string()
                },
            }
        }
        CotDocument::MapItem(map_item) => CotEvent {
            version: map_item.g.clone(), // g = VERSION
            uid: map_item.id.clone(),
            event_type: map_item.w.clone(),
            time: micros_to_datetime(map_item.b as i64), // b = TIME in microseconds
            start: micros_to_datetime(map_item.n.unwrap_or(0.0) as i64),
            stale: micros_to_datetime(map_item.o.unwrap_or(0.0) as i64),
            how: map_item.p.clone(),
            point: Point {
                lat: map_item.j.unwrap_or(0.0), // j = LAT
                lon: map_item.l.unwrap_or(0.0), // l = LON
                hae: map_item.i.unwrap_or(0.0), // i = HAE
                ce: map_item.h.unwrap_or(0.0),  // h = CE
                le: map_item.k.unwrap_or(0.0),  // k = LE
            },
            // Serialize detail map to XML for round-trip fidelity
            detail: {
                use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                use crate::xml_writer::to_cot_xml;
                let flat = flat_cot_event_from_ditto(doc);
                let xml = to_cot_xml(&flat);
                let start = xml.find("<detail>").unwrap_or(0);
                let end = xml
                    .find("</detail>")
                    .map(|i| i + "</detail>".len())
                    .unwrap_or(xml.len());
                xml[start..end].to_string()
            },
        },
        CotDocument::Generic(generic) => CotEvent {
            version: generic.g.clone(), // g = VERSION
            uid: generic.id.clone(),
            event_type: generic.w.clone(),
            time: micros_to_datetime(generic.b as i64), // b = TIME in microseconds
            start: micros_to_datetime(generic.n.unwrap_or(0.0) as i64),
            stale: micros_to_datetime(generic.o.unwrap_or(0.0) as i64),
            how: generic.p.clone(),
            point: Point {
                lat: generic.j.unwrap_or(0.0), // j = LAT
                lon: generic.l.unwrap_or(0.0), // l = LON
                hae: generic.i.unwrap_or(0.0), // i = HAE
                ce: generic.h.unwrap_or(0.0),  // h = CE
                le: generic.k.unwrap_or(0.0),  // k = LE
            },
            // Serialize detail map to XML for round-trip fidelity
            detail: {
                use crate::ditto::from_ditto_util::flat_cot_event_from_ditto;
                use crate::xml_writer::to_cot_xml;
                let flat = flat_cot_event_from_ditto(doc);
                let xml = to_cot_xml(&flat);
                let start = xml.find("<detail>").unwrap_or(0);
                let end = xml
                    .find("</detail>")
                    .map(|i| i + "</detail>".len())
                    .unwrap_or(xml.len());
                xml[start..end].to_string()
            },
        },
    }
}

/// Convert a flattened JSON document (with r_* fields) back into a CotEvent
pub fn cot_event_from_flattened_json(json_value: &Value) -> CotEvent {
    use crate::cot_events::Point;

    /// Helper to safely convert microseconds since epoch to DateTime<Utc>
    fn micros_to_datetime(micros: i64) -> DateTime<Utc> {
        // Convert microseconds to seconds and nanoseconds
        let secs = micros / 1_000_000;
        let nanos = ((micros % 1_000_000) * 1_000) as u32;

        // Use timestamp_opt for better error handling
        Utc.timestamp_opt(secs, nanos).single().unwrap_or_else(|| {
            eprintln!(
                "WARN: Failed to convert timestamp {} microseconds to DateTime<Utc>",
                micros
            );
            Utc::now()
        })
    }

    if let Value::Object(obj) = json_value {
        let mut document_map: HashMap<String, Value> = obj.clone().into_iter().collect();

        // Unflatten r_* fields back to nested r field for detail reconstruction
        let r_map = unflatten_document_r_field(&mut document_map);

        // Helper function to get string value from JSON
        let get_string = |key: &str| -> String {
            document_map
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };

        // Helper function to get f64 value from JSON (unused but kept for future use)
        let _get_f64 = |key: &str| -> f64 {
            document_map
                .get(key)
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0)
        };

        // Helper function to get optional f64 value from JSON
        let get_opt_f64 =
            |key: &str| -> Option<f64> { document_map.get(key).and_then(|v| v.as_f64()) };

        // Determine document type to use correct coordinate mappings
        let event_type = get_string("w");
        let is_map_item = event_type.contains("a-u-r-loc-g")
            || event_type.contains("a-f-G-U-C")
            || event_type.contains("a-f-G-U")
            || event_type.contains("a-f-G-U-I")
            || event_type.contains("a-f-G-U-T")
            || event_type.contains("a-u-S")
            || event_type.contains("a-u-A")
            || event_type.contains("a-u-G");
        let is_file = event_type.contains("file")
            || event_type.contains("attachment")
            || event_type.contains("b-f-t-file");

        CotEvent {
            version: get_string("g"), // g = VERSION
            uid: get_string("_id"),
            event_type: get_string("w"),
            time: {
                let b = get_opt_f64("b").unwrap_or(0.0);
                if b != 0.0 {
                    micros_to_datetime(b as i64) // b = TIME in microseconds
                } else {
                    Utc::now()
                }
            },
            start: {
                let n = get_opt_f64("n").unwrap_or(0.0);
                micros_to_datetime(n as i64)
            },
            stale: {
                let o = get_opt_f64("o").unwrap_or(0.0);
                micros_to_datetime(o as i64)
            },
            how: get_string("p"),
            point: Point {
                lat: if is_map_item {
                    get_opt_f64("j").unwrap_or(0.0)
                } else {
                    // For file and other documents, lat is stored in h field
                    get_opt_f64("h").unwrap_or(0.0)
                },
                lon: if is_map_item {
                    get_opt_f64("l").unwrap_or(0.0)
                } else {
                    // For file and other documents, lon is stored in i field
                    get_opt_f64("i").unwrap_or(0.0)
                },
                hae: if is_map_item {
                    get_opt_f64("i").unwrap_or(0.0)
                } else {
                    // For file and other documents, hae is stored in j field
                    get_opt_f64("j").unwrap_or(0.0)
                },
                ce: if is_file {
                    // For file documents, CE is stored in r__ce field, but after unflattening it would be in r_map["_ce"]
                    r_map.get("_ce").and_then(|v| v.as_f64()).unwrap_or(0.0)
                } else {
                    get_opt_f64("h").unwrap_or(0.0) // h = CE for other document types
                },
                le: get_opt_f64("k").unwrap_or(0.0),
            },
            // Reconstruct detail XML from the unflattened r_map
            detail: {
                use crate::model::FlatCotEvent;
                use crate::xml_writer::to_cot_xml;

                // Create a FlatCotEvent with the properly reconstructed detail_extra
                let flat = FlatCotEvent {
                    uid: get_string("_id"),
                    type_: get_string("w"),
                    time: {
                        let n = get_opt_f64("n").unwrap_or(0.0);
                        if n != 0.0 {
                            let secs = (n as i64) / 1_000_000;
                            let nanos = (((n as i64) % 1_000_000) * 1_000) as u32;
                            chrono::Utc
                                .timestamp_opt(secs, nanos)
                                .single()
                                .unwrap_or_else(chrono::Utc::now)
                                .to_rfc3339()
                        } else {
                            chrono::Utc::now().to_rfc3339()
                        }
                    },
                    start: {
                        let n = get_opt_f64("n").unwrap_or(0.0);
                        let secs = (n as i64) / 1_000_000;
                        let nanos = (((n as i64) % 1_000_000) * 1_000) as u32;
                        chrono::Utc
                            .timestamp_opt(secs, nanos)
                            .single()
                            .unwrap_or_else(chrono::Utc::now)
                            .to_rfc3339()
                    },
                    stale: {
                        let o = get_opt_f64("o").unwrap_or(0.0);
                        let secs = (o as i64) / 1_000_000;
                        let nanos = (((o as i64) % 1_000_000) * 1_000) as u32;
                        chrono::Utc
                            .timestamp_opt(secs, nanos)
                            .single()
                            .unwrap_or_else(chrono::Utc::now)
                            .to_rfc3339()
                    },
                    how: get_string("p"),
                    lat: if is_map_item {
                        get_opt_f64("j").unwrap_or(0.0)
                    } else {
                        get_opt_f64("h").unwrap_or(0.0)
                    },
                    lon: if is_map_item {
                        get_opt_f64("l").unwrap_or(0.0)
                    } else {
                        get_opt_f64("i").unwrap_or(0.0)
                    },
                    hae: if is_map_item {
                        get_opt_f64("i").unwrap_or(0.0)
                    } else {
                        get_opt_f64("j").unwrap_or(0.0)
                    },
                    ce: if is_file {
                        get_opt_f64("r__ce").unwrap_or(0.0)
                    } else {
                        get_opt_f64("h").unwrap_or(0.0) // h = CE for other types
                    },
                    le: get_opt_f64("k").unwrap_or(0.0),
                    callsign: None,      // Comes from detail_extra
                    group_name: None,    // Comes from detail_extra
                    detail_extra: r_map, // Use the properly reconstructed r_map!
                };

                let xml = to_cot_xml(&flat);
                // Extract only the <detail>...</detail> section
                let start = xml.find("<detail>").unwrap_or(0);
                let end = xml
                    .find("</detail>")
                    .map(|i| i + "</detail>".len())
                    .unwrap_or(xml.len());
                xml[start..end].to_string()
            },
        }
    } else {
        // Fallback for non-object JSON
        CotEvent {
            version: "2.0".to_string(), // Default version
            uid: "unknown".to_string(),
            event_type: "unknown".to_string(),
            time: Utc::now(),
            start: Utc::now(),
            stale: Utc::now(),
            how: "".to_string(),
            point: Point {
                lat: 0.0,
                lon: 0.0,
                hae: 0.0,
                ce: 0.0,
                le: 0.0,
            },
            detail: "<detail></detail>".to_string(),
        }
    }
}
