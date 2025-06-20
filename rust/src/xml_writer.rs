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
    if let Some(ref cs) = event.callsign {
        xml.push_str(&format!(r#"<contact callsign="{}"/>"#, cs));
    }
    if let Some(ref group) = event.group_name {
        xml.push_str(&format!(r#"<__group name="{}"/>"#, group));
    }
    for (k, v) in &event.detail_extra {
        if let Some(obj) = v.as_object() {
            xml.push_str(&format!(r#"<{}"#, k));
            for (key, val) in obj {
                if let Some(s) = val.as_str() {
                    xml.push_str(&format!(r#" {}="{}""#, key, s));
                }
            }
            xml.push_str("/>");
        }
    }
    xml.push_str("</detail>");
    xml.push_str("</event>");
    xml
}
