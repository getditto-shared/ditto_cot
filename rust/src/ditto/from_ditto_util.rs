//! Utility to convert CotDocument + r map to FlatCotEvent for XML serialization
use crate::ditto::r_field_flattening::unflatten_document_r_field;
use crate::ditto::CotDocument;
use crate::model::FlatCotEvent;
use chrono::TimeZone;
use serde_json::Value;
use std::collections::HashMap;

/// Convert a CotDocument to a FlatCotEvent for XML serialization
pub fn flat_cot_event_from_ditto(doc: &CotDocument) -> FlatCotEvent {
    use serde_json::Value;
    use std::collections::HashMap;

    // Log the r field contents at trace level for debugging if needed
    match doc {
        CotDocument::Api(api) => {
            let map: HashMap<String, Value> = api
                .r
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                .collect();
            log::trace!("flat_cot_event_from_ditto: Api.r = {:?}", map);
        }
        CotDocument::Chat(chat) => {
            let map: HashMap<String, Value> = chat
                .r
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                .collect();
            log::trace!("flat_cot_event_from_ditto: Chat.r = {:?}", map);
        }
        CotDocument::File(file) => {
            let map: HashMap<String, Value> = file
                .r
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                .collect();
            log::trace!("flat_cot_event_from_ditto: File.r = {:?}", map);
        }
        CotDocument::Generic(generic) => {
            let map: HashMap<String, Value> = generic
                .r
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                .collect();
            log::trace!("flat_cot_event_from_ditto: Generic.r = {:?}", map);
        }
        CotDocument::MapItem(map_item) => {
            let map: HashMap<String, Value> = map_item
                .r
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                .collect();
            log::trace!("flat_cot_event_from_ditto: MapItem.r = {:?}", map);
        }
    }

    match doc {
        CotDocument::Api(api) => FlatCotEvent {
            uid: api.id.clone(),
            type_: api.w.clone(),
            time: chrono::Utc
                .timestamp_opt(
                    (api.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((api.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            start: chrono::Utc
                .timestamp_opt(
                    (api.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((api.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            stale: chrono::Utc
                .timestamp_opt(
                    (api.o.unwrap_or(0.0) as i64) / 1_000_000,
                    (((api.o.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            how: api.p.clone(),
            lat: api.h.unwrap_or(0.0),
            lon: api.i.unwrap_or(0.0),
            hae: api.j.unwrap_or(0.0),
            ce: api.b,
            le: api.k.unwrap_or(0.0),
            callsign: api.e.clone().into(),
            group_name: api.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = api
                    .r
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                    .collect();
                if api.r.contains_key("original_type") {
                    map.insert("original_type".to_string(), Value::String(api.w.clone()));
                }
                map
            },
        },
        CotDocument::Chat(chat) => FlatCotEvent {
            uid: chat.id.clone(),
            type_: chat.w.clone(),
            time: chrono::Utc
                .timestamp_opt(
                    (chat.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((chat.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            start: chrono::Utc
                .timestamp_opt(
                    (chat.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((chat.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            stale: chrono::Utc
                .timestamp_opt(
                    (chat.o.unwrap_or(0.0) as i64) / 1_000_000,
                    (((chat.o.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            how: chat.p.clone(),
            lat: chat.h.unwrap_or(0.0),
            lon: chat.i.unwrap_or(0.0),
            hae: chat.j.unwrap_or(0.0),
            ce: chat.b,
            le: chat.k.unwrap_or(0.0),
            callsign: chat.e.clone().into(),
            group_name: chat.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = chat
                    .r
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                    .collect();
                if chat.r.contains_key("original_type") {
                    map.insert("original_type".to_string(), Value::String(chat.w.clone()));
                }
                map
            },
        },
        CotDocument::File(file) => FlatCotEvent {
            uid: file.id.clone(),
            type_: file.w.clone(),
            time: chrono::Utc
                .timestamp_opt(
                    (file.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((file.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            start: chrono::Utc
                .timestamp_opt(
                    (file.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((file.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            stale: chrono::Utc
                .timestamp_opt(
                    (file.o.unwrap_or(0.0) as i64) / 1_000_000,
                    (((file.o.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            how: file.p.clone(),
            lat: file.h.unwrap_or(0.0),
            lon: file.i.unwrap_or(0.0),
            hae: file.j.unwrap_or(0.0),
            ce: file.b,
            le: file.k.unwrap_or(0.0),
            callsign: file.e.clone().into(),
            group_name: file.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = file
                    .r
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                    .collect();
                if file.r.contains_key("original_type") {
                    map.insert("original_type".to_string(), Value::String(file.w.clone()));
                }
                map
            },
        },
        CotDocument::Generic(generic) => FlatCotEvent {
            uid: generic.id.clone(),
            type_: generic.w.clone(),
            time: chrono::Utc
                .timestamp_opt(
                    (generic.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((generic.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            start: chrono::Utc
                .timestamp_opt(
                    (generic.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((generic.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            stale: chrono::Utc
                .timestamp_opt(
                    (generic.o.unwrap_or(0.0) as i64) / 1_000_000,
                    (((generic.o.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            how: generic.p.clone(),
            lat: generic.h.unwrap_or(0.0),
            lon: generic.i.unwrap_or(0.0),
            hae: generic.j.unwrap_or(0.0),
            ce: generic.b,
            le: generic.k.unwrap_or(0.0),
            callsign: generic.e.clone().into(),
            group_name: generic.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = generic
                    .r
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                    .collect();
                if generic.r.contains_key("original_type") {
                    map.insert(
                        "original_type".to_string(),
                        Value::String(generic.w.clone()),
                    );
                }
                map
            },
        },
        CotDocument::MapItem(map_item) => FlatCotEvent {
            uid: map_item.id.clone(),
            type_: map_item.w.clone(),
            time: chrono::Utc
                .timestamp_opt(
                    (map_item.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((map_item.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            start: chrono::Utc
                .timestamp_opt(
                    (map_item.n.unwrap_or(0.0) as i64) / 1_000_000,
                    (((map_item.n.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            stale: chrono::Utc
                .timestamp_opt(
                    (map_item.o.unwrap_or(0.0) as i64) / 1_000_000,
                    (((map_item.o.unwrap_or(0.0) as i64) % 1_000_000) * 1_000) as u32,
                )
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
            how: map_item.p.clone(),
            lat: map_item.j.unwrap_or(0.0), // For MapItems: j = lat
            lon: map_item.l.unwrap_or(0.0), // For MapItems: l = lon
            hae: map_item.i.unwrap_or(0.0), // For MapItems: i = hae
            ce: map_item.b,                 // b = ce (time in millis, but used for ce)
            le: map_item.k.unwrap_or(0.0),  // k = le
            callsign: map_item.e.clone().into(),
            group_name: map_item.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = map_item
                    .r
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
                    .collect();
                if map_item.r.contains_key("original_type") {
                    map.insert(
                        "original_type".to_string(),
                        Value::String(map_item.w.clone()),
                    );
                }
                map
            },
        },
    }
}

/// Convert a flattened JSON document (with r_* fields) to a FlatCotEvent for XML serialization
pub fn flat_cot_event_from_flattened_json(json_value: &Value) -> FlatCotEvent {
    // Convert JSON Value to HashMap and unflatten r_* fields
    if let Value::Object(obj) = json_value {
        let mut document_map: HashMap<String, Value> = obj.clone().into_iter().collect();

        // Unflatten r_* fields back to a nested r field
        let r_map = unflatten_document_r_field(&mut document_map);

        // Helper function to get string value from JSON
        let get_string = |key: &str| -> String {
            document_map
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };

        // Helper function to get f64 value from JSON
        let get_f64 = |key: &str| -> f64 {
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

        // Helper to convert microseconds to RFC3339 string
        let micros_to_rfc3339 = |micros: f64| -> String {
            let secs = (micros as i64) / 1_000_000;
            let nanos = (((micros as i64) % 1_000_000) * 1_000) as u32;
            chrono::Utc
                .timestamp_opt(secs, nanos)
                .single()
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339()
        };

        FlatCotEvent {
            uid: get_string("_id"),
            type_: get_string("w"),
            time: {
                let n = get_opt_f64("n").unwrap_or(0.0);
                if n != 0.0 {
                    micros_to_rfc3339(n)
                } else {
                    chrono::Utc::now().to_rfc3339()
                }
            },
            start: {
                let n = get_opt_f64("n").unwrap_or(0.0);
                micros_to_rfc3339(n)
            },
            stale: {
                let o = get_opt_f64("o").unwrap_or(0.0);
                micros_to_rfc3339(o)
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
            ce: get_f64("b"),
            le: get_opt_f64("k").unwrap_or(0.0),
            callsign: None,   // Callsign info comes from detail_extra, not e field
            group_name: None, // Group info comes from detail_extra, not g field (which is version)
            detail_extra: r_map,
        }
    } else {
        // Fallback for non-object JSON
        FlatCotEvent {
            uid: "unknown".to_string(),
            type_: "unknown".to_string(),
            time: chrono::Utc::now().to_rfc3339(),
            start: chrono::Utc::now().to_rfc3339(),
            stale: chrono::Utc::now().to_rfc3339(),
            how: "".to_string(),
            lat: 0.0,
            lon: 0.0,
            hae: 0.0,
            ce: 0.0,
            le: 0.0,
            callsign: Some("".to_string()),
            group_name: Some("".to_string()),
            detail_extra: HashMap::new(),
        }
    }
}
