//! Parser for the detail section of CoT (Cursor on Target) messages.
//!
//! This module provides functionality to parse the detail section of CoT messages,
//! extracting structured information like callsign, group name, and additional
//! key-value pairs.

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde_json::Value;
use std::collections::HashMap;

/// Parses the <detail> section of a CoT message as a generic XML-to-map transformation.
///
/// This function converts all attributes and text content into a HashMap<String, Value>,
/// preserving the structure and content of <detail> without any special-case logic.
///
/// # Arguments
/// * `detail_xml` - A string slice containing the XML content of the detail section
///
/// # Returns
/// A HashMap<String, Value> representing all attributes and text content in <detail>.
///
/// # Example
/// ```
/// use ditto_cot::detail_parser::parse_detail_section;
/// use std::collections::HashMap;
/// use serde_json::Value;
///
/// let detail = r#"<contact callsign="TEST-123"/><__group name="Blue"/><status readiness="true"/>"#;
/// let extras = parse_detail_section(detail);
/// assert_eq!(extras.get("contact").unwrap()["callsign"], Value::String("TEST-123".to_string()));
/// assert_eq!(extras.get("__group").unwrap()["name"], Value::String("Blue".to_string()));
/// assert_eq!(extras.get("status").unwrap()["readiness"], Value::String("true".to_string()));
/// ```
pub fn parse_detail_section(detail_xml: &str) -> HashMap<String, Value> {
    use serde_json::{Map, Value};

    let mut reader = Reader::from_str(detail_xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut extras = HashMap::new();

    fn parse_element<R: std::io::BufRead>(
        reader: &mut Reader<R>,
        start: &BytesStart,
        buf: &mut Vec<u8>,
    ) -> Value {
        let _tag = String::from_utf8_lossy(start.name().as_ref()).to_string();
        let mut map = Map::new();
        // Parse attributes
        for attr_result in start.attributes() {
            if let Ok(attr) = attr_result {
                let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                let val = String::from_utf8_lossy(&attr.value).to_string();
                map.insert(key, Value::String(val));
            }
        }
        // Parse children
        let mut text_content = None;
        loop {
            buf.clear();
            match reader.read_event_into(buf) {
                Ok(Event::Start(e)) => {
                    let child_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let mut child_buf = Vec::new();
                    let child_val = parse_element(reader, &e, &mut child_buf);
                    map.insert(child_tag, child_val);
                }
                Ok(Event::Empty(e)) => {
                    let child_tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let mut child_map = Map::new();
                    for attr_result in e.attributes() {
                        if let Ok(attr) = attr_result {
                            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            child_map.insert(key, Value::String(val));
                        }
                    }
                    map.insert(child_tag, Value::Object(child_map));
                }
                Ok(Event::Text(t)) => {
                    let text = t.unescape().unwrap_or_default().to_string();
                    if !text.is_empty() {
                        text_content = Some(text);
                    }
                }
                Ok(Event::End(e)) if e.name() == start.name() => {
                    break;
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
        }
        // If there was only text content and no attributes/children, return as string
        if map.is_empty() {
            if let Some(text) = text_content {
                Value::String(text)
            } else {
                Value::Object(map)
            }
        } else {
            if let Some(text) = text_content {
                map.insert("_text".to_string(), Value::String(text));
            }
            Value::Object(map)
        }
    }

    // Main event loop
    let mut in_root = false;
    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if !in_root && tag == "detail" {
                    in_root = true;
                } else if in_root {
                    let mut child_buf = Vec::new();
                    let val = parse_element(&mut reader, e, &mut child_buf);
                    extras.insert(tag, val);
                }
            }
            Ok(Event::Empty(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut map = Map::new();
                for attr_result in e.attributes() {
                    if let Ok(attr) = attr_result {
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let val = String::from_utf8_lossy(&attr.value).to_string();
                        map.insert(key, Value::String(val));
                    }
                }
                if in_root {
                    extras.insert(tag, Value::Object(map));
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if in_root && tag == "detail" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }
    extras
}
