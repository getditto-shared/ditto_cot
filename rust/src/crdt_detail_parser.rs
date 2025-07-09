//! CRDT-optimized parser for the detail section of CoT messages.
//!
//! This module provides functionality to parse the detail section of CoT messages
//! with stable key generation for duplicate elements, enabling differential updates
//! in CRDT-based P2P networks while preserving all data.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde_json::{Map, Value};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const TAG_METADATA: &str = "_tag";
// Removed redundant metadata: _docId and _elementIndex are already encoded in the key
const KEY_SEPARATOR: &str = "_";

/// Parses the <detail> section with CRDT-optimized stable keys for duplicate elements.
///
/// This function converts XML to a HashMap where:
/// - Single occurrence elements use direct keys (e.g., "status" -> value)
/// - Duplicate elements use stable keys (e.g., "docId_sensor_0" -> enhanced_value)
///
/// Each duplicate element is enhanced with minimal metadata for reconstruction:
/// - `_tag`: Original element name (docId and index are encoded in the key)
///
/// # Arguments
/// * `detail_xml` - XML content of the detail section
/// * `document_id` - Document identifier for stable key generation
///
/// # Returns
/// A HashMap with CRDT-optimized keys preserving all duplicate elements
///
/// # Example
/// ```rust
/// use ditto_cot::crdt_detail_parser::parse_detail_section_with_stable_keys;
/// use serde_json::Value;
///
/// let detail = r#"<detail>
///   <sensor type="optical" id="sensor-1"/>
///   <sensor type="thermal" id="sensor-2"/>
///   <status operational="true"/>
/// </detail>"#;
///
/// let result = parse_detail_section_with_stable_keys(detail, "test-doc");
///
/// // Single element uses direct key
/// assert!(result.contains_key("status"));
///
/// // Duplicate elements use stable keys (Base64 hash format)
/// // We can verify by counting sensor elements in the result
/// let sensor_count = result.values()
///     .filter(|v| {
///         if let Value::Object(obj) = v {
///             if let Some(Value::String(tag)) = obj.get("_tag") {
///                 return tag == "sensor";
///             }
///         }
///         false
///     })
///     .count();
/// assert_eq!(sensor_count, 2);
/// ```
pub fn parse_detail_section_with_stable_keys(
    detail_xml: &str,
    document_id: &str,
) -> HashMap<String, Value> {
    // First pass: count occurrences of each element type
    let element_counts = count_element_occurrences(detail_xml);

    // Second pass: parse with appropriate key generation
    parse_with_stable_keys(detail_xml, document_id, &element_counts)
}

/// Counts occurrences of each element type in the detail section.
fn count_element_occurrences(detail_xml: &str) -> HashMap<String, u32> {
    let mut reader = Reader::from_str(detail_xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut counts = HashMap::new();
    let mut in_detail = false;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if !in_detail && tag == "detail" {
                    in_detail = true;
                } else if in_detail {
                    *counts.entry(tag).or_insert(0) += 1;
                    // Skip to end of this element
                    let element_name = e.name().as_ref().to_vec();
                    let mut skip_buf = Vec::new();
                    skip_element(&mut reader, &element_name, &mut skip_buf);
                }
            }
            Ok(Event::Empty(ref e)) => {
                if in_detail {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    *counts.entry(tag).or_insert(0) += 1;
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if in_detail && tag == "detail" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }

    counts
}

/// Parses detail section with stable key generation based on element counts.
fn parse_with_stable_keys(
    detail_xml: &str,
    document_id: &str,
    element_counts: &HashMap<String, u32>,
) -> HashMap<String, Value> {
    let mut reader = Reader::from_str(detail_xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut result = HashMap::new();
    let mut element_indices: HashMap<String, u32> = HashMap::new();
    let mut in_detail = false;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if !in_detail && tag == "detail" {
                    in_detail = true;
                } else if in_detail {
                    let mut child_buf = Vec::new();
                    let value = parse_element(&mut reader, e, &mut child_buf);

                    let count = element_counts.get(&tag).unwrap_or(&0);
                    if *count > 1 {
                        // Generate stable key for duplicate
                        let index = element_indices.entry(tag.clone()).or_insert(0);
                        let stable_key = generate_stable_key(document_id, &tag, *index);
                        let enhanced_value =
                            enhance_with_metadata(value, &tag, document_id, *index);
                        result.insert(stable_key, enhanced_value);
                        *index += 1;
                    } else {
                        // Use direct key for single occurrence
                        result.insert(tag, value);
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                if in_detail {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let mut map = Map::new();
                    for attr in e.attributes().flatten() {
                        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                        let val = String::from_utf8_lossy(&attr.value).to_string();
                        map.insert(key, Value::String(val));
                    }
                    let value = Value::Object(map);

                    let count = element_counts.get(&tag).unwrap_or(&0);
                    if *count > 1 {
                        let index = element_indices.entry(tag.clone()).or_insert(0);
                        let stable_key = generate_stable_key(document_id, &tag, *index);
                        let enhanced_value =
                            enhance_with_metadata(value, &tag, document_id, *index);
                        result.insert(stable_key, enhanced_value);
                        *index += 1;
                    } else {
                        result.insert(tag, value);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if in_detail && tag == "detail" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }

    result
}

/// Parse a single XML element into a Value.
fn parse_element<R: std::io::BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    buf: &mut Vec<u8>,
) -> Value {
    let mut map = Map::new();

    // Parse attributes
    for attr in start.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
        let val = String::from_utf8_lossy(&attr.value).to_string();
        map.insert(key, Value::String(val));
    }

    // Parse children and text content
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
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = String::from_utf8_lossy(&attr.value).to_string();
                    child_map.insert(key, Value::String(val));
                }
                map.insert(child_tag, Value::Object(child_map));
            }
            Ok(Event::Text(t)) => {
                let text = t.unescape().unwrap_or_default().to_string();
                if !text.trim().is_empty() {
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

    // Return appropriate value based on content
    if map.is_empty() {
        text_content
            .map(Value::String)
            .unwrap_or(Value::Object(map))
    } else {
        if let Some(text) = text_content {
            map.insert("_text".to_string(), Value::String(text));
        }
        Value::Object(map)
    }
}

/// Skip to the end of an element during counting phase.
fn skip_element<R: std::io::BufRead>(
    reader: &mut Reader<R>,
    element_name: &[u8],
    buf: &mut Vec<u8>,
) {
    let mut depth = 1;
    loop {
        buf.clear();
        match reader.read_event_into(buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == element_name => {
                depth += 1;
            }
            Ok(Event::End(e)) if e.name().as_ref() == element_name => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }
}

/// Generate a stable key for duplicate elements using Base64 hash format.
/// Format: base64(hash(document_id + element_name))_index
fn generate_stable_key(document_id: &str, element_name: &str, index: u32) -> String {
    let mut hasher = DefaultHasher::new();
    format!("{}{}{}", document_id, element_name, "stable_key_salt").hash(&mut hasher);
    let hash = hasher.finish();

    // Convert hash to bytes and encode as base64
    let hash_bytes = hash.to_be_bytes();
    let b64_hash = URL_SAFE_NO_PAD.encode(hash_bytes);

    format!("{}{}{}", b64_hash, KEY_SEPARATOR, index)
}

/// Enhance a value with minimal metadata for reconstruction.
/// Only stores the tag name - document ID and index are encoded in the key.
fn enhance_with_metadata(value: Value, tag: &str, _doc_id: &str, _element_index: u32) -> Value {
    match value {
        Value::Object(mut map) => {
            map.insert(TAG_METADATA.to_string(), Value::String(tag.to_string()));
            Value::Object(map)
        }
        Value::String(text) => {
            let mut map = Map::new();
            map.insert(TAG_METADATA.to_string(), Value::String(tag.to_string()));
            map.insert("_text".to_string(), Value::String(text));
            Value::Object(map)
        }
        other => {
            // For other types, wrap in object with metadata
            let mut map = Map::new();
            map.insert(TAG_METADATA.to_string(), Value::String(tag.to_string()));
            map.insert("_value".to_string(), other);
            Value::Object(map)
        }
    }
}

/// Convert a stable key map back to XML.
///
/// This function reconstructs XML from a HashMap that may contain stable keys,
/// grouping duplicate elements by their original tag names and preserving
/// the relative order within each group.
///
/// # Arguments
/// * `detail_map` - HashMap with CRDT-optimized keys
///
/// # Returns
/// XML string representing the reconstructed detail section
pub fn convert_stable_keys_to_xml(detail_map: &HashMap<String, Value>) -> String {
    let mut xml = String::from("<detail>");

    // Separate direct elements from stable key elements
    let mut direct_elements = Vec::new();
    let mut stable_elements: HashMap<String, Vec<(u32, Value)>> = HashMap::new();

    for (key, value) in detail_map {
        if is_stable_key(key) {
            if let Some(index) = parse_stable_key(key) {
                // Extract tag name from metadata
                if let Value::Object(obj) = value {
                    if let Some(Value::String(tag)) = obj.get(TAG_METADATA) {
                        stable_elements
                            .entry(tag.clone())
                            .or_default()
                            .push((index, value.clone()));
                    }
                }
            }
        } else {
            direct_elements.push((key.clone(), value.clone()));
        }
    }

    // Add direct elements
    for (tag, value) in direct_elements {
        xml.push_str(&value_to_xml_element(&tag, &value, false));
    }

    // Add stable key elements, sorted by index within each group
    for (tag, mut elements) in stable_elements {
        elements.sort_by_key(|(index, _)| *index);
        for (_, value) in elements {
            xml.push_str(&value_to_xml_element(&tag, &value, true));
        }
    }

    xml.push_str("</detail>");
    xml
}

/// Check if a key is a stable key (base64 hash format with index).
fn is_stable_key(key: &str) -> bool {
    let parts: Vec<&str> = key.split(KEY_SEPARATOR).collect();
    parts.len() == 2 && parts.last().unwrap().parse::<u32>().is_ok()
}

/// Parse a stable key to extract index (tag name comes from metadata).
fn parse_stable_key(key: &str) -> Option<u32> {
    let parts: Vec<&str> = key.split(KEY_SEPARATOR).collect();
    if parts.len() == 2 {
        if let Ok(index) = parts.last().unwrap().parse::<u32>() {
            return Some(index);
        }
    }
    None
}

/// Convert a Value to an XML element, optionally removing metadata.
fn value_to_xml_element(tag: &str, value: &Value, remove_metadata: bool) -> String {
    match value {
        Value::Object(map) => {
            let mut attributes = Vec::new();
            let mut text_content = None;
            let mut child_elements = Vec::new();

            for (key, val) in map {
                if remove_metadata && key.starts_with('_') {
                    if key == "_text" {
                        if let Value::String(text) = val {
                            text_content = Some(text.clone());
                        }
                    }
                    // Skip metadata fields (_tag, _value, etc.)
                } else if key == "_text" {
                    if let Value::String(text) = val {
                        text_content = Some(text.clone());
                    }
                } else if key == "_value" {
                    // Handle wrapped primitive values
                    return value_to_xml_element(tag, val, false);
                } else if let Value::String(attr_val) = val {
                    attributes.push(format!("{}=\"{}\"", key, attr_val));
                } else {
                    child_elements.push(value_to_xml_element(key, val, false));
                }
            }

            let attr_str = if attributes.is_empty() {
                String::new()
            } else {
                format!(" {}", attributes.join(" "))
            };

            if child_elements.is_empty() && text_content.is_none() {
                format!("<{}{}/>", tag, attr_str)
            } else {
                format!(
                    "<{}{}>{}{}</{}>",
                    tag,
                    attr_str,
                    text_content.unwrap_or_default(),
                    child_elements.join(""),
                    tag
                )
            }
        }
        Value::String(text) => {
            format!("<{}>{}</{}>", tag, text, tag)
        }
        _ => {
            format!("<{}>{}</{}>", tag, value, tag)
        }
    }
}

/// Get the next available index for a given element type.
/// This is useful when adding new elements in a P2P network.
pub fn get_next_available_index(
    detail_map: &HashMap<String, Value>,
    document_id: &str,
    element_name: &str,
) -> u32 {
    // Generate the expected hash for this document_id + element_name combination
    let mut hasher = DefaultHasher::new();
    format!("{}{}{}", document_id, element_name, "stable_key_salt").hash(&mut hasher);
    let hash = hasher.finish();
    let hash_bytes = hash.to_be_bytes();
    let b64_hash = URL_SAFE_NO_PAD.encode(hash_bytes);

    let key_prefix = format!("{}{}", b64_hash, KEY_SEPARATOR);

    let mut max_index = 0u32;
    let mut found_any = false;

    for key in detail_map.keys() {
        if key.starts_with(&key_prefix) {
            if let Some(index_str) = key.strip_prefix(&key_prefix) {
                if let Ok(index) = index_str.parse::<u32>() {
                    max_index = max_index.max(index);
                    found_any = true;
                }
            }
        }
    }

    if found_any {
        max_index + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_detail() {
        let detail = r#"<detail><status operational="true"/></detail>"#;
        let result = parse_detail_section_with_stable_keys(detail, "test-doc");

        assert_eq!(result.len(), 1);
        assert!(result.contains_key("status"));

        let status = result.get("status").unwrap();
        assert_eq!(status["operational"], Value::String("true".to_string()));
    }

    #[test]
    fn test_parse_duplicate_elements() {
        let detail = r#"<detail>
            <sensor type="optical" id="sensor-1"/>
            <sensor type="thermal" id="sensor-2"/>
            <sensor type="radar" id="sensor-3"/>
            <status operational="true"/>
        </detail>"#;

        let result = parse_detail_section_with_stable_keys(detail, "test-doc");

        assert_eq!(result.len(), 4); // 3 sensors + 1 status

        // Single element uses direct key
        assert!(result.contains_key("status"));

        // Duplicate elements use stable keys (base64 hash format)
        let sensor_keys: Vec<String> = result
            .keys()
            .filter(|k| {
                k.contains("_") && k.ends_with("_0") || k.ends_with("_1") || k.ends_with("_2")
            })
            .filter(|k| {
                if let Some(Value::Object(obj)) = result.get(*k) {
                    if let Some(Value::String(tag)) = obj.get(TAG_METADATA) {
                        return tag == "sensor";
                    }
                }
                false
            })
            .cloned()
            .collect();

        assert_eq!(sensor_keys.len(), 3, "Should have 3 sensor keys");

        // Check metadata on first sensor (only _tag remains)
        let sensor0_key = sensor_keys.iter().find(|k| k.ends_with("_0")).unwrap();
        let sensor0 = result.get(sensor0_key).unwrap();
        assert_eq!(sensor0[TAG_METADATA], Value::String("sensor".to_string()));
        assert_eq!(sensor0["type"], Value::String("optical".to_string()));
    }

    #[test]
    fn test_round_trip_conversion() {
        let detail = r#"<detail>
            <sensor type="optical" id="sensor-1"/>
            <sensor type="thermal" id="sensor-2"/>
            <status operational="true"/>
        </detail>"#;

        let parsed = parse_detail_section_with_stable_keys(detail, "test-doc");
        let reconstructed = convert_stable_keys_to_xml(&parsed);

        // Should contain all elements
        assert!(reconstructed.contains("<sensor"));
        assert!(reconstructed.contains("type=\"optical\""));
        assert!(reconstructed.contains("type=\"thermal\""));
        assert!(reconstructed.contains("<status"));
        assert!(reconstructed.contains("operational=\"true\""));
    }

    #[test]
    fn test_get_next_available_index() {
        let mut detail_map = HashMap::new();

        // Generate expected hash for sensor elements
        let mut hasher = DefaultHasher::new();
        format!("{}{}{}", "test-doc", "sensor", "stable_key_salt").hash(&mut hasher);
        let hash = hasher.finish();
        let hash_bytes = hash.to_be_bytes();
        let b64_hash = URL_SAFE_NO_PAD.encode(hash_bytes);

        detail_map.insert(format!("{}_0", b64_hash), Value::Null);
        detail_map.insert(format!("{}_2", b64_hash), Value::Null);

        let next = get_next_available_index(&detail_map, "test-doc", "sensor");
        assert_eq!(next, 3); // Should be 3 (after max index 2)

        let next_contact = get_next_available_index(&detail_map, "test-doc", "contact");
        assert_eq!(next_contact, 0); // No existing contacts
    }
}
