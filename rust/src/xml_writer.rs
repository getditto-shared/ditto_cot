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
        r#"<event version="2.0" uid="{}" type="{}" time="{}" start="{}" stale="{}" how="{}">"#,
        event.uid, event.type_, event.time, event.start, event.stale, event.how
    ));

    // Add point element with coordinates
    xml.push_str(&format!(
        r#"<point lat="{}" lon="{}" hae="{}" ce="{}" le="{}"/>"#,
        event.lat, event.lon, event.hae, event.ce, event.le
    ));

    xml.push_str("<detail>");

    // Add callsign if present and not empty
    if let Some(callsign) = &event.callsign {
        if !callsign.is_empty() {
            xml.push_str(&format!(r#"<contact callsign="{}"/>"#, callsign));
        }
    }

    // Add group_name if present and not empty
    if let Some(group_name) = &event.group_name {
        if !group_name.is_empty() {
            xml.push_str(&format!(r#"<__group name="{}"/>"#, group_name));
        }
    }

    // Helper for recursive serialization of detail_extra
    fn write_detail_xml(xml: &mut String, k: &str, v: &serde_json::Value) {
        log::trace!("write_detail_xml: key = {} | value = {:?}", k, v);
        if let Some(obj) = v.as_object() {
            // Special cases for known nested elements
            if (k == "sensor" || k == "platform") && obj.contains_key("name") && obj.len() == 1 {
                // Handle <sensor><n>ThermalCam-X</n></sensor> and <platform><n>MQ-9 Reaper</n></platform> format
                if let Some(serde_json::Value::String(name)) = obj.get("name") {
                    log::trace!(
                        "write_detail_xml: special case for <{}><n>{}</n></{}>",
                        k,
                        name,
                        k
                    );
                    xml.push_str(&format!("<{}><n>{}</n></{}>", k, name, k));
                    return;
                }
            }

            // If all values are string and no _text, treat as attributes
            let mut attrs = Vec::new();
            let mut children = Vec::new();
            let mut text = None;
            // Sort keys for canonical order
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();

            for key in keys {
                let val = &obj[key];
                if key == "_text" {
                    if let Some(s) = val.as_str() {
                        text = Some(s.to_string());
                    }
                } else if val.is_object() || val.is_array() {
                    children.push((key.as_str(), val));
                } else if let Some(s) = val.as_str() {
                    attrs.push((key.as_str(), s.to_string()));
                } else if let Some(n) = val.as_f64() {
                    let n_str = n.to_string();
                    attrs.push((key.as_str(), n_str));
                } else if let Some(b) = val.as_bool() {
                    let b_str = b.to_string();
                    attrs.push((key.as_str(), b_str));
                }
            }

            // If we have children or text, we need a full element
            if !children.is_empty() || text.is_some() {
                log::trace!(
                    "write_detail_xml: <{}> attrs: {:?}, children: {:?}, text: {:?}",
                    k,
                    attrs,
                    children,
                    text
                );
                // Start tag with attributes
                xml.push_str(&format!("<{}", k));
                for (attr_k, attr_v) in &attrs {
                    xml.push_str(&format!(" {}=\"{}\"", attr_k, attr_v));
                }
                xml.push('>');

                // Add text if any
                if let Some(t) = text {
                    xml.push_str(&t);
                }

                // Add children
                for (child_k, child_v) in children {
                    write_detail_xml(xml, child_k, child_v);
                }

                // Close tag
                xml.push_str(&format!("</{}>", k));
            } else {
                // Just attributes, no children or text
                log::trace!("write_detail_xml: <{}> only attributes: {:?}", k, attrs);
                xml.push_str(&format!("<{}", k));
                for (attr_k, attr_v) in &attrs {
                    xml.push_str(&format!(" {}=\"{}\"", attr_k, attr_v));
                }
                xml.push_str("/>");
                log::trace!("write_detail_xml: emitting tag: <{}/>", k);
            }
        } else if let Some(arr) = v.as_array() {
            log::trace!("write_detail_xml: <{}> array value: {:?}", k, arr);
            for item in arr {
                write_detail_xml(xml, k, item);
            }
        } else if let Some(s) = v.as_str() {
            log::trace!("write_detail_xml: <{}> string value: {}", k, s);
            xml.push_str(&format!("<{}>{}</{}>", k, s, k));
        } else if let Some(n) = v.as_f64() {
            log::trace!("write_detail_xml: <{}> number value: {}", k, n);
            xml.push_str(&format!("<{}>{}</{}>", k, n, k));
        } else if let Some(b) = v.as_bool() {
            log::trace!("write_detail_xml: <{}> bool value: {}", k, b);
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
