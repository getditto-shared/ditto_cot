//! R Field Flattening Module
//!
//! This module provides functionality to flatten and unflatten the 'r' field
//! (CoT detail elements) for DQL compatibility. The flattening converts nested
//! structures like r.takv.os to individual r_takv_os fields.

use serde_json::Value;
use std::collections::HashMap;

/// Flatten the r field into individual r_* fields for DQL compatibility
/// Converts r.takv.os -> r_takv_os, r.contact.endpoint -> r_contact_endpoint, etc.
pub fn flatten_r_field(
    r_map: &HashMap<String, impl Into<Value> + Clone>,
) -> HashMap<String, Value> {
    let mut flattened = HashMap::new();

    for (detail_type, detail_value) in r_map {
        let value_json: Value = detail_value.clone().into();

        if let Value::Object(obj) = value_json {
            // Flatten nested objects
            for (attribute, attr_value) in obj {
                let flattened_key = format!("r_{}_{}", detail_type, attribute);
                flattened.insert(flattened_key, attr_value);
            }
        } else {
            // Simple value
            let flattened_key = format!("r_{}", detail_type);
            flattened.insert(flattened_key, value_json);
        }
    }

    flattened
}

/// Reconstruct the r field from flattened r_* fields
/// Converts r_takv_os -> r.takv.os, r_contact_endpoint -> r.contact.endpoint, etc.
pub fn unflatten_r_field(flattened_map: &HashMap<String, Value>) -> HashMap<String, Value> {
    let mut r_map: HashMap<String, Value> = HashMap::new();

    for (key, value) in flattened_map {
        if let Some(without_r_prefix) = key.strip_prefix("r_") {
            // Handle the special case where detail_type starts with underscores (like __group)
            // Find the last underscore to properly split detail_type from attribute
            if let Some(last_underscore) = without_r_prefix.rfind('_') {
                let detail_type = &without_r_prefix[..last_underscore];
                let attribute = &without_r_prefix[last_underscore + 1..];

                // Only treat as nested if we found a meaningful split
                // (i.e., both parts are non-empty)
                if !detail_type.is_empty() && !attribute.is_empty() {
                    // Nested r_detailType_attribute case
                    let detail_obj = r_map
                        .entry(detail_type.to_string())
                        .or_insert_with(|| Value::Object(serde_json::Map::new()));

                    if let Value::Object(obj) = detail_obj {
                        obj.insert(attribute.to_string(), value.clone());
                    }
                } else {
                    // Simple r_field case (no meaningful split found)
                    r_map.insert(without_r_prefix.to_string(), value.clone());
                }
            } else {
                // Simple r_field case (no underscore found)
                r_map.insert(without_r_prefix.to_string(), value.clone());
            }
        }
    }

    r_map
}

/// Convert a nested HashMap<String, XxxRValue> to a flat HashMap for DQL storage
pub fn flatten_document_r_field<T>(
    document_map: &mut HashMap<String, Value>,
    r_field: &HashMap<String, T>,
) where
    T: Into<Value> + Clone,
{
    // Remove the original 'r' field
    document_map.remove("r");

    // Add flattened r_* fields
    for (detail_type, detail_value) in r_field {
        let value_json: Value = detail_value.clone().into();

        if let Value::Object(obj) = value_json {
            // Flatten nested objects
            for (attribute, attr_value) in obj {
                let flattened_key = format!("r_{}_{}", detail_type, attribute);
                document_map.insert(flattened_key, attr_value);
            }
        } else {
            // Simple value
            let flattened_key = format!("r_{}", detail_type);
            document_map.insert(flattened_key, value_json);
        }
    }
}

/// Reconstruct the r field from a flat HashMap containing r_* fields
pub fn unflatten_document_r_field(
    document_map: &mut HashMap<String, Value>,
) -> HashMap<String, Value> {
    let mut r_map: HashMap<String, Value> = HashMap::new();
    let mut keys_to_remove = Vec::new();

    for (key, value) in document_map.iter() {
        if let Some(without_r_prefix) = key.strip_prefix("r_") {
            // Handle the special case where detail_type starts with underscores (like __group)
            // Find the last underscore to properly split detail_type from attribute
            if let Some(last_underscore) = without_r_prefix.rfind('_') {
                let detail_type = &without_r_prefix[..last_underscore];
                let attribute = &without_r_prefix[last_underscore + 1..];

                // Only treat as nested if we found a meaningful split
                // (i.e., both parts are non-empty)
                if !detail_type.is_empty() && !attribute.is_empty() {
                    // Nested r_detailType_attribute case
                    let detail_obj = r_map
                        .entry(detail_type.to_string())
                        .or_insert_with(|| Value::Object(serde_json::Map::new()));

                    if let Value::Object(obj) = detail_obj {
                        obj.insert(attribute.to_string(), value.clone());
                    }
                } else {
                    // Simple r_field case (no meaningful split found)
                    r_map.insert(without_r_prefix.to_string(), value.clone());
                }
            } else {
                // Simple r_field case (no underscore found)
                r_map.insert(without_r_prefix.to_string(), value.clone());
            }

            keys_to_remove.push(key.clone());
        }
    }

    // Remove the r_* fields from the main map
    for key in keys_to_remove {
        document_map.remove(&key);
    }

    // Add the reconstructed 'r' field if we found any r_* fields
    if !r_map.is_empty() {
        let r_map_copy = r_map.clone();
        document_map.insert(
            "r".to_string(),
            Value::Object(r_map_copy.into_iter().collect()),
        );
    }

    r_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_flatten_simple_r_field() {
        let mut r_map = HashMap::new();
        r_map.insert("simple".to_string(), json!("test_value"));

        let flattened = flatten_r_field(&r_map);

        assert_eq!(flattened.get("r_simple"), Some(&json!("test_value")));
    }

    #[test]
    fn test_flatten_nested_r_field() {
        let mut r_map = HashMap::new();
        r_map.insert(
            "takv".to_string(),
            json!({
                "os": "35",
                "version": "5.4.0.11",
                "device": "GOOGLE PIXEL 7"
            }),
        );
        r_map.insert(
            "contact".to_string(),
            json!({
                "endpoint": "192.168.5.241:4242:tcp",
                "callsign": "BRAMA"
            }),
        );

        let flattened = flatten_r_field(&r_map);

        assert_eq!(flattened.get("r_takv_os"), Some(&json!("35")));
        assert_eq!(flattened.get("r_takv_version"), Some(&json!("5.4.0.11")));
        assert_eq!(
            flattened.get("r_takv_device"),
            Some(&json!("GOOGLE PIXEL 7"))
        );
        assert_eq!(
            flattened.get("r_contact_endpoint"),
            Some(&json!("192.168.5.241:4242:tcp"))
        );
        assert_eq!(flattened.get("r_contact_callsign"), Some(&json!("BRAMA")));
    }

    #[test]
    fn test_unflatten_r_field() {
        let mut flattened = HashMap::new();
        flattened.insert("r_takv_os".to_string(), json!("35"));
        flattened.insert("r_takv_version".to_string(), json!("5.4.0.11"));
        flattened.insert(
            "r_contact_endpoint".to_string(),
            json!("192.168.5.241:4242:tcp"),
        );
        flattened.insert("r_contact_callsign".to_string(), json!("BRAMA"));
        flattened.insert("other_field".to_string(), json!("not_an_r_field"));

        let r_map = unflatten_r_field(&flattened);

        assert_eq!(
            r_map.get("takv"),
            Some(&json!({
                "os": "35",
                "version": "5.4.0.11"
            }))
        );
        assert_eq!(
            r_map.get("contact"),
            Some(&json!({
                "endpoint": "192.168.5.241:4242:tcp",
                "callsign": "BRAMA"
            }))
        );
        assert!(!r_map.contains_key("other_field"));
    }

    #[test]
    fn test_round_trip_flattening() {
        let mut original_r_map = HashMap::new();
        original_r_map.insert(
            "takv".to_string(),
            json!({
                "os": "35",
                "version": "5.4.0.11"
            }),
        );
        original_r_map.insert(
            "contact".to_string(),
            json!({
                "endpoint": "192.168.5.241:4242:tcp",
                "callsign": "BRAMA"
            }),
        );

        // Flatten
        let flattened = flatten_r_field(&original_r_map);

        // Unflatten
        let reconstructed = unflatten_r_field(&flattened);

        // Verify round-trip
        assert_eq!(
            reconstructed.get("takv"),
            Some(&json!({
                "os": "35",
                "version": "5.4.0.11"
            }))
        );
        assert_eq!(
            reconstructed.get("contact"),
            Some(&json!({
                "endpoint": "192.168.5.241:4242:tcp",
                "callsign": "BRAMA"
            }))
        );
    }
}
