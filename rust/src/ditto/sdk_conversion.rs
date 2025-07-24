//! Observer document conversion utilities
//!
//! This module provides utilities to convert documents from Ditto SDK observer callbacks
//! to CotDocument and JSON representations with proper r-field reconstruction.

use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::ditto::{r_field_flattening::unflatten_document_r_field, CotDocument};

/// Convert observer document JSON to a typed CotDocument
///
/// This function takes the JSON string from an observer document (via `doc.json_string()`)
/// and converts it to the appropriate CotDocument variant based on the document's
/// 'w' field (event type). This is the main function for getting typed access to observer documents.
///
/// # Arguments
/// * `observer_doc_json` - JSON string from `doc.json_string()` in observer callback
///
/// # Returns
/// * `Result<CotDocument, anyhow::Error>` - The converted CotDocument or an error
///
/// # Example
/// ```no_run
/// use ditto_cot::ditto::{observer_json_to_cot_document, CotDocument};
///
/// // Example with JSON string from observer
/// let json_str = r#"{"_id": "test", "w": "a-u-r-loc-g", "j": 37.7749, "l": -122.4194}"#;
/// match observer_json_to_cot_document(json_str) {
///     Ok(cot_doc) => {
///         match cot_doc {
///             CotDocument::MapItem(item) => {
///                 println!("Received map item: {}", item.id);
///             }
///             CotDocument::Chat(chat) => {
///                 println!("Received chat: {:?}", chat.message);
///             }
///             _ => println!("Received other document type"),
///         }
///     }
///     Err(e) => println!("Conversion error: {}", e),
/// }
/// ```
pub fn observer_json_to_cot_document(observer_doc_json: &str) -> Result<CotDocument> {
    // Use existing from_json_str method
    CotDocument::from_json_str(observer_doc_json)
}

/// Convert observer document JSON to JSON with reconstructed r-fields
///
/// This function takes the JSON string from an observer document and reconstructs
/// the hierarchical r-field structure from flattened r_* fields. This gives you
/// the full document structure as it would appear in the original CoT event.
///
/// # Arguments
/// * `observer_doc_json` - JSON string from `doc.json_string()` in observer callback
///
/// # Returns
/// * `Result<Value, anyhow::Error>` - The complete JSON representation with r-field reconstruction
///
/// # Example
/// ```no_run
/// use ditto_cot::ditto::observer_json_to_json_with_r_fields;
///
/// // Example with flattened r_* fields
/// let json_str = r#"{"_id": "test", "w": "a-u-r-loc-g", "r_contact_callsign": "TestUnit"}"#;
/// match observer_json_to_json_with_r_fields(json_str) {
///     Ok(json_value) => {
///         // Full document structure with nested r-field
///         println!("Document JSON: {}", serde_json::to_string_pretty(&json_value).unwrap());
///         
///         // Access nested detail information
///         if let Some(r_field) = json_value.get("r") {
///             println!("Detail section: {}", r_field);
///         }
///     }
///     Err(e) => println!("Conversion error: {}", e),
/// }
/// ```
pub fn observer_json_to_json_with_r_fields(observer_doc_json: &str) -> Result<Value> {
    // Parse the JSON string
    let doc_value: Value = serde_json::from_str(observer_doc_json)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    // Convert to a mutable map for unflattening
    if let Value::Object(obj) = &doc_value {
        let mut document_map: HashMap<String, Value> = obj.clone().into_iter().collect();

        // Unflatten r_* fields back to nested r field
        let r_map = unflatten_document_r_field(&mut document_map);

        // Add the reconstructed r field if it has content
        if !r_map.is_empty() {
            document_map.insert("r".to_string(), Value::Object(r_map.into_iter().collect()));
        }

        Ok(Value::Object(document_map.into_iter().collect()))
    } else {
        // Return the document as-is if it's not an object
        Ok(doc_value)
    }
}

/// Extract document ID from observer document JSON or Value
///
/// This is a convenience function that extracts just the document ID,
/// which is commonly needed in observer callbacks for logging or processing.
///
/// # Arguments
/// * `doc_value` - Either JSON string or serde_json::Value from `doc.json_string()` or `doc.value()`
///
/// # Returns
/// * `Option<String>` - The document ID if present
///
/// # Example
/// ```no_run
/// use ditto_cot::ditto::{get_document_id_from_value, get_document_id_from_json};
/// use serde_json::{json, Value};
///
/// // From JSON Value
/// let doc_value: Value = json!({"_id": "test-123", "w": "a-u-r-loc-g"});
/// if let Some(id) = get_document_id_from_value(&doc_value) {
///     println!("Document ID: {}", id);
/// }
///
/// // From JSON string
/// let json_str = r#"{"_id": "test-456", "w": "a-u-r-loc-g"}"#;
/// if let Some(id) = get_document_id_from_json(json_str) {
///     println!("Document ID: {}", id);
/// }
/// ```
pub fn get_document_id_from_value(doc_value: &Value) -> Option<String> {
    doc_value
        .get("_id")
        .or_else(|| doc_value.get("id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract document ID from observer document JSON string
pub fn get_document_id_from_json(observer_doc_json: &str) -> Option<String> {
    let doc_value: Value = serde_json::from_str(observer_doc_json).ok()?;
    get_document_id_from_value(&doc_value)
}

/// Extract document type from observer document JSON or Value
///
/// This is a convenience function that extracts the document type (w field),
/// which determines the CotDocument variant. Useful for filtering or routing
/// different document types in observer callbacks.
///
/// # Arguments
/// * `doc_value` - serde_json::Value from `doc.value()`
///
/// # Returns
/// * `Option<String>` - The document type if present (e.g., "a-u-r-loc-g", "b-t-f")
///
/// # Example
/// ```no_run
/// use ditto_cot::ditto::get_document_type_from_value;
/// use serde_json::{json, Value};
///
/// // From JSON Value
/// let doc_value: Value = json!({"_id": "test", "w": "a-u-r-loc-g"});
/// if let Some(doc_type) = get_document_type_from_value(&doc_value) {
///     match doc_type.as_str() {
///         "a-u-r-loc-g" => println!("Received location update"),
///         "b-t-f" => println!("Received chat message"),
///         _ => println!("Received {}", doc_type),
///     }
/// }
/// ```
pub fn get_document_type_from_value(doc_value: &Value) -> Option<String> {
    doc_value
        .get("w")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract document type from observer document JSON string
pub fn get_document_type_from_json(observer_doc_json: &str) -> Option<String> {
    let doc_value: Value = serde_json::from_str(observer_doc_json).ok()?;
    get_document_type_from_value(&doc_value)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_json_value_extraction() {
        // Create a mock document value with _id field
        let doc_value = json!({
            "_id": "test-doc-123",
            "w": "a-u-r-loc-g"
        });

        // Test extracting ID from JSON Value directly (simulating DittoDocument.value())
        let id = doc_value
            .get("_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        assert_eq!(id, Some("test-doc-123".to_string()));
    }

    #[test]
    fn test_document_type_extraction() {
        let doc_value = json!({
            "_id": "test-doc-123",
            "w": "a-u-r-loc-g"
        });

        let doc_type = doc_value
            .get("w")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        assert_eq!(doc_type, Some("a-u-r-loc-g".to_string()));
    }
}
