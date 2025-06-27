//! Utility to convert DittoDocument + r map to FlatCotEvent for XML serialization
use crate::model::FlatCotEvent;
use crate::ditto::DittoDocument;
use serde_json::Value;
use std::collections::HashMap;
use chrono::TimeZone;

/// Convert a DittoDocument to a FlatCotEvent for XML serialization
pub fn flat_cot_event_from_ditto(doc: &DittoDocument) -> FlatCotEvent {
    use std::collections::HashMap;
    use serde_json::Value;
    // Debug: print r field if present
    match doc {
        DittoDocument::Api(api) => {
            let map: HashMap<String, Value> = api.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
            println!("[DEBUG] flat_cot_event_from_ditto: Api.r = {:?}", map);
        },
        DittoDocument::Chat(chat) => {
            let map: HashMap<String, Value> = chat.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
            println!("[DEBUG] flat_cot_event_from_ditto: Chat.r = {:?}", map);
        },
        DittoDocument::File(file) => {
            let map: HashMap<String, Value> = file.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
            println!("[DEBUG] flat_cot_event_from_ditto: File.r = {:?}", map);
        },
        DittoDocument::MapItem(map_item) => {
            let map: HashMap<String, Value> = map_item.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
            println!("[DEBUG] flat_cot_event_from_ditto: MapItem.r = {:?}", map);
        },
    }

    match doc {
        DittoDocument::Api(api) => FlatCotEvent {
            uid: api.id.clone(),
            type_: api.w.clone(),
            time: chrono::Utc.timestamp_millis_opt(api.n).unwrap().to_rfc3339(),
            start: chrono::Utc.timestamp_millis_opt(api.n).unwrap().to_rfc3339(),
            stale: chrono::Utc.timestamp_millis_opt(api.o).unwrap().to_rfc3339(),
            how: api.p.clone(),
            lat: api.h.unwrap_or(0.0),
            lon: api.i.unwrap_or(0.0),
            hae: api.j.unwrap_or(0.0),
            ce: api.b,
            le: api.k.unwrap_or(0.0),
            callsign: api.e.clone().into(),
            group_name: api.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = api.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
                if api.r.contains_key("original_type") {
                    map.insert("original_type".to_string(), Value::String(api.w.clone()));
                }
                map
            },
        },
        DittoDocument::Chat(chat) => FlatCotEvent {
            uid: chat.id.clone(),
            type_: chat.w.clone(),
            time: chrono::Utc.timestamp_millis_opt(chat.n).unwrap().to_rfc3339(),
            start: chrono::Utc.timestamp_millis_opt(chat.n).unwrap().to_rfc3339(),
            stale: chrono::Utc.timestamp_millis_opt(chat.o).unwrap().to_rfc3339(),
            how: chat.p.clone(),
            lat: chat.h.unwrap_or(0.0),
            lon: chat.i.unwrap_or(0.0),
            hae: chat.j.unwrap_or(0.0),
            ce: chat.b,
            le: chat.k.unwrap_or(0.0),
            callsign: chat.e.clone().into(),
            group_name: chat.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = chat.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
                if chat.r.contains_key("original_type") {
                    map.insert("original_type".to_string(), Value::String(chat.w.clone()));
                }
                map
            },
        },
        DittoDocument::File(file) => FlatCotEvent {
            uid: file.id.clone(),
            type_: file.w.clone(),
            time: chrono::Utc.timestamp_millis_opt(file.n).unwrap().to_rfc3339(),
            start: chrono::Utc.timestamp_millis_opt(file.n).unwrap().to_rfc3339(),
            stale: chrono::Utc.timestamp_millis_opt(file.o).unwrap().to_rfc3339(),
            how: file.p.clone(),
            lat: file.h.unwrap_or(0.0),
            lon: file.i.unwrap_or(0.0),
            hae: file.j.unwrap_or(0.0),
            ce: file.b,
            le: file.k.unwrap_or(0.0),
            callsign: file.e.clone().into(),
            group_name: file.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = file.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
                if file.r.contains_key("original_type") {
                    map.insert("original_type".to_string(), Value::String(file.w.clone()));
                }
                map
            },
        },
        DittoDocument::MapItem(map_item) => FlatCotEvent {
            uid: map_item.id.clone(),
            type_: map_item.w.clone(),
            time: chrono::Utc.timestamp_millis_opt(map_item.n).unwrap().to_rfc3339(),
            start: chrono::Utc.timestamp_millis_opt(map_item.n).unwrap().to_rfc3339(),
            stale: chrono::Utc.timestamp_millis_opt(map_item.o).unwrap().to_rfc3339(),
            how: map_item.p.clone(),
            lat: map_item.h.unwrap_or(0.0),
            lon: map_item.i.unwrap_or(0.0),
            hae: map_item.j.unwrap_or(0.0),
            ce: map_item.k.unwrap_or(0.0),
            le: map_item.k.unwrap_or(0.0),
            callsign: map_item.e.clone().into(),
            group_name: map_item.g.clone().into(),
            detail_extra: {
                let mut map: HashMap<String, Value> = map_item.r.iter().map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))).collect();
                if map_item.r.contains_key("original_type") {
                    map.insert("original_type".to_string(), Value::String(map_item.w.clone()));
                }
                map
            },
        },
    }
}
