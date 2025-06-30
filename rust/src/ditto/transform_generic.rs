use crate::cot_events::CotEvent;
use crate::ditto::schema::{Generic, GenericRValue};

// Import parse_detail_section from the parent module
use super::parse_detail_section;
use serde_json;

/// Transform any CoT event to a generic Ditto document
pub fn transform_generic_event(event: &CotEvent, peer_key: &str) -> Generic {
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
        b: 0.0, // We're not using b for time anymore to avoid field overloading
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
        n: time_micros,  // Store time in microseconds
        o: stale_micros, // Store stale in microseconds
        p: event.how.clone(),
        q: "".to_string(),
        // Parse detail XML into map for CRDT support
        r: {
            extras
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        match v {
                            serde_json::Value::String(s) => GenericRValue::String(s),
                            serde_json::Value::Bool(b) => GenericRValue::from(b),
                            serde_json::Value::Number(n) => {
                                GenericRValue::from(n.as_f64().unwrap_or(0.0))
                            }
                            serde_json::Value::Object(obj) => {
                                let map = serde_json::Map::from_iter(obj.clone());
                                GenericRValue::Object(map)
                            }
                            serde_json::Value::Array(arr) => GenericRValue::Array(arr.clone()),
                            _ => GenericRValue::Null,
                        },
                    )
                })
                .collect()
        },
        s: "".to_string(),
        t: "".to_string(),
        u: "".to_string(),
        v: "".to_string(),
        w: event.event_type.clone(),
    }
}
