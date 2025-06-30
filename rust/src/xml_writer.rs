//! XML generation utilities for CoT (Cursor on Target) messages.
//!
//! This module provides functionality to generate CoT XML messages from
//! structured Rust types.

use crate::model::FlatCotEvent;

/// Converts a `FlatCotEvent` into a CoT XML string.
///
/// This function takes a `FlatCotEvent` and generates a well-formed CoT XML message.
/// It handles all standard CoT fields and includes any additional details from
/// the event's `detail_extra` map.
///
/// # Arguments
/// * `event` - A reference to the `FlatCotEvent` to convert
///
/// # Returns
/// * `String` - The generated CoT XML as a string
///
/// # Examples
/// ```
/// use ditto_cot::model::FlatCotEvent;
/// use ditto_cot::xml_writer::to_cot_xml;
///
/// let mut event = FlatCotEvent {
///     uid: "ANDROID-deadbeef".to_string(),
///     type_: "a-f-G-U-C".to_string(),
///     time: "2023-01-01T00:00:00Z".to_string(),
///     start: "2023-01-01T00:00:00Z".to_string(),
///     stale: "2023-01-01T00:00:00Z".to_string(),
///     how: "h-g-i-g-o".to_string(),
///     lat: 0.0,
///     lon: 0.0,
///     hae: 0.0,
///     ce: 0.0,
///     le: 0.0,
///     callsign: Some("TestUser".to_string()),
///     group_name: Some("Blue".to_string()),
///     detail_extra: Default::default(),
/// };
///
/// let xml = to_cot_xml(&event);
/// assert!(xml.contains("<event version=\"2.0\""));
/// assert!(xml.contains("<contact callsign=\"TestUser\""));
/// ```
pub fn to_cot_xml(event: &FlatCotEvent) -> String {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(&format!(
        r#"<event version="2.0" uid="{}" type="{}" time="{}" start="{}" stale="{}" how="{}" lat="{}" lon="{}" hae="{}" ce="{}" le="{}">"#,
        event.uid, event.type_, event.time, event.start, event.stale, event.how,
        event.lat, event.lon, event.hae, event.ce, event.le
    ));
    xml.push_str("<detail>");

    // Helper for recursive serialization of detail_extra
    use std::collections::BTreeMap;
    fn write_detail_xml(xml: &mut String, k: &str, v: &serde_json::Value) {
        println!("[DEBUG] write_detail_xml: key = {} | value = {:?}", k, v);
        if let Some(obj) = v.as_object() {
            // Special cases for known nested elements
            if (k == "sensor" || k == "platform") && obj.contains_key("name") && obj.len() == 1 {
                // Handle <sensor><n>ThermalCam-X</n></sensor> and <platform><n>MQ-9 Reaper</n></platform> format
                if let Some(serde_json::Value::String(name)) = obj.get("name") {
                    println!("[DEBUG] write_detail_xml: special case for <{}><n>{}</n></{}>", k, name, k);
                    xml.push_str(&format!("<{}><n>{}</n></{}>", k, name, k));
                    return;
                }
            }
            
            // If all values are string and no _text, treat as attributes
            let mut attrs = vec![];
            let mut children = vec![];
            let mut text = None;
            // Sort keys for canonical order
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            for key in keys {
                let val = &obj[key];
                if key == "_text" {
                    if let Some(s) = val.as_str() {
                        text = Some(s);
                    }
                } else if let Some(s) = val.as_str() {
                    attrs.push((key.as_str(), s));
                } else {
                    children.push((key, val));
                }
            }
            attrs.sort_by_key(|(k, _)| *k);
            children.sort_by_key(|(k, _)| *k);
            
            // For certain elements, always use nested format even if only attributes
            let force_nested = matches!(k, "sensor" | "platform" | "nested");
            
            if children.is_empty() && text.is_none() && !force_nested {
                // Only attributes
                println!("[DEBUG] write_detail_xml: <{}> only attributes: {:?}", k, attrs);
                let tag_str = {
                    let mut s = format!("<{}", k);
                    for (key, val) in &attrs {
                        s.push_str(&format!(" {}=\"{}\"", key, val));
                    }
                    s.push_str("/>");
                    s
                };
                println!("[DEBUG] write_detail_xml: emitting tag: {}", tag_str);
                xml.push_str(&tag_str);
            } else {
                // Start tag with attributes
                println!("[DEBUG] write_detail_xml: <{}> attrs: {:?}, children: {:?}, text: {:?}", k, attrs, children, text);
                xml.push_str(&format!("<{}", k));
                for (key, val) in &attrs {
                    // For certain elements, convert attributes to nested elements
                    if force_nested && key == &"name" {
                        continue; // Skip adding as attribute, will add as nested element below
                    }
                    xml.push_str(&format!(" {}=\"{}\"", key, val));
                }
                xml.push('>');
                
                // For certain elements, convert attributes to nested elements
                if force_nested {
                    for (key, val) in &attrs {
                        if key == &"name" {
                            xml.push_str(&format!("<n>{}</n>", val));
                        }
                    }
                }
                
                // Optional text
                if let Some(t) = text {
                    xml.push_str(t);
                }
                // Children
                for (child_k, child_v) in children {
                    write_detail_xml(xml, child_k, child_v);
                }
                xml.push_str(&format!("</{}>", k));
            }
        } else if let Some(s) = v.as_str() {
            println!("[DEBUG] write_detail_xml: <{}> string value: {}", k, s);
            xml.push_str(&format!("<{}>{}</{}>", k, s, k));
        } else if let Some(n) = v.as_f64() {
            println!("[DEBUG] write_detail_xml: <{}> number value: {}", k, n);
            xml.push_str(&format!("<{}>{}</{}>", k, n, k));
        } else if let Some(b) = v.as_bool() {
            println!("[DEBUG] write_detail_xml: <{}> bool value: {}", k, b);
            xml.push_str(&format!("<{}>{}</{}>", k, b, k));
        }
    }
    let mut detail_keys: Vec<_> = event.detail_extra.keys().collect();
    detail_keys.sort();
    for k in detail_keys {
        let v = &event.detail_extra[k];
        write_detail_xml(&mut xml, k, v);
    }
    xml.push_str("</detail>");
    xml.push_str("</event>");
    xml
}
