//! Parser for the detail section of CoT (Cursor on Target) messages.
//!
//! This module provides functionality to parse the detail section of CoT messages,
//! extracting structured information like callsign, group name, and additional
//! key-value pairs.

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Parses the detail section of a CoT message.
///
/// This function extracts structured information from the detail section of a CoT message,
/// including the callsign, group name, and any additional key-value pairs.
///
/// # Arguments
/// * `detail_xml` - A string slice containing the XML content of the detail section
///
/// # Returns
/// A tuple containing:
/// 1. `Option<String>` - The callsign if found
/// 2. `Option<String>` - The group name if found
/// 3. `HashMap<String, Value>` - Additional key-value pairs from the detail section
///
/// # Examples
/// ```
/// use ditto_cot::detail_parser::parse_detail_section;
/// use std::collections::HashMap;
///
/// let detail = r#"
///     <contact callsign="TEST-123"/>
///     <__group name="Blue"/>
///     <status readiness="true"/>
/// "#;
///
/// let (callsign, group_name, extras) = parse_detail_section(detail);
/// assert_eq!(callsign, Some("TEST-123".to_string()));
/// assert_eq!(group_name, Some("Blue".to_string()));
/// assert_eq!(extras.get("status").and_then(|v| v.get("@readiness")), Some(&Value::String("true".to_string())));
/// ```
pub fn parse_detail_section(
    detail_xml: &str,
) -> (Option<String>, Option<String>, HashMap<String, Value>) {
    let mut callsign = None;
    let mut group_name = None;
    let mut extras = HashMap::new();

    let mut reader = Reader::from_str(detail_xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut depth = 0;

    // Helper function to process attributes
    fn process_attributes(
        tag: &str,
        e: &BytesStart,
    ) -> (Option<String>, Option<String>, HashMap<String, Value>) {
        let mut callsign = None;
        let mut group_name = None;
        let mut map = HashMap::new();

        for attr_result in e.attributes() {
            match attr_result {
                Ok(attr) => {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = String::from_utf8_lossy(&attr.value).to_string();

                    if tag == "contact" && key == "callsign" {
                        callsign = Some(val.clone());
                    } else if tag == "__group" && key == "name" {
                        group_name = Some(val.clone());
                    }

                    map.insert(key, Value::String(val));
                }
                Err(e) => eprintln!("Error parsing attribute: {}", e),
            }
        }

        (callsign, group_name, map)
    }

    loop {
        buf.clear();

        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                depth += 1;

                let (cs, gn, attrs) = process_attributes(&tag, &e);

                if let Some(cs) = cs {
                    callsign = Some(cs);
                }
                if let Some(gn) = gn {
                    group_name = Some(gn);
                }

                if !attrs.is_empty() {
                    extras.insert(tag, json!(attrs));
                }
            }

            Ok(Event::Empty(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();

                let (cs, gn, attrs) = process_attributes(&tag, &e);

                if let Some(cs) = cs {
                    callsign = Some(cs);
                }
                if let Some(gn) = gn {
                    group_name = Some(gn);
                }

                if !attrs.is_empty() {
                    extras.insert(tag, json!(attrs));
                }
            }

            Ok(Event::End(_e)) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }

            Ok(Event::Eof) => {
                break;
            }

            Ok(_) => {}

            Err(_) => {
                break;
            }
        }
    }

    (callsign, group_name, extras)
}
