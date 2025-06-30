//! Ditto DQL support for CotDocument
//!
//! This module implements the Ditto DQL `DittoDocument` trait for our `CotDocument` enum,
//! allowing it to be used directly with Ditto's query interface.

use crate::ditto::CotDocument;
use dittolive_ditto::error::{DittoError, ErrorKind};
use dittolive_ditto::prelude::*;
use dittolive_ditto::store::query_builder::{DittoDocument, DocumentId};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

// Helper function to convert JSON values to CBOR values
fn json_to_cbor(json: JsonValue) -> Result<CborValue, String> {
    match json {
        JsonValue::Null => Ok(CborValue::Null),
        JsonValue::Bool(b) => Ok(CborValue::Bool(b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(CborValue::Integer(i as i128))
            } else if let Some(f) = n.as_f64() {
                Ok(CborValue::Float(f))
            } else {
                Err("Unsupported number type".to_string())
            }
        }
        JsonValue::String(s) => Ok(CborValue::Text(s)),
        JsonValue::Array(arr) => {
            let mut cbor_arr = Vec::with_capacity(arr.len());
            for item in arr {
                cbor_arr.push(json_to_cbor(item)?);
            }
            Ok(CborValue::Array(cbor_arr))
        }
        JsonValue::Object(obj) => {
            let mut cbor_map = BTreeMap::new();
            for (key, value) in obj {
                let cbor_key = CborValue::Text(key);
                let cbor_value = json_to_cbor(value)?;
                cbor_map.insert(cbor_key, cbor_value);
            }
            Ok(CborValue::Map(cbor_map))
        }
    }
}

impl DittoDocument for CotDocument {
    fn id(&self) -> DocumentId {
        // Get the ID string from the document
        let id_str = match self {
            CotDocument::Api(api) => api.id.clone(),
            CotDocument::Chat(chat) => chat.id.clone(),
            CotDocument::File(file) => file.id.clone(),
            CotDocument::MapItem(map_item) => map_item.id.clone(),
        };

        // Convert the ID string to a DocumentId
        DocumentId::new(&id_str).unwrap_or_else(|_| {
            // If conversion fails, return a default DocumentId
            let fallback = "invalid_id".to_string();
            DocumentId::new(&fallback).unwrap()
        })
    }

    fn to_cbor(&self) -> Result<CborValue, DittoError> {
        // Convert the document to a JSON value first
        let json_value = match self {
            CotDocument::Api(api) => serde_json::to_value(api),
            CotDocument::Chat(chat) => serde_json::to_value(chat),
            CotDocument::File(file) => serde_json::to_value(file),
            CotDocument::MapItem(map_item) => serde_json::to_value(map_item),
        }
        .map_err(|_| DittoError::from(ErrorKind::InvalidInput))?;

        // Convert the JSON value to a CBOR value
        json_to_cbor(json_value).map_err(|_| DittoError::from(ErrorKind::InvalidInput))
    }

    fn get<V: DeserializeOwned>(&self, path: &str) -> Result<V, DittoError> {
        // Convert the document to a JSON value first
        let json_value = match self {
            CotDocument::Api(api) => serde_json::to_value(api),
            CotDocument::Chat(chat) => serde_json::to_value(chat),
            CotDocument::File(file) => serde_json::to_value(file),
            CotDocument::MapItem(map_item) => serde_json::to_value(map_item),
        }
        .map_err(|_| DittoError::from(ErrorKind::InvalidInput))?;

        // Extract the value at the given path
        let value = match path {
            "id" | "_id" => {
                // Special case for ID, which is stored differently in our model vs Ditto
                match self {
                    CotDocument::Api(api) => serde_json::to_value(&api.id),
                    CotDocument::Chat(chat) => serde_json::to_value(&chat.id),
                    CotDocument::File(file) => serde_json::to_value(&file.id),
                    CotDocument::MapItem(map_item) => serde_json::to_value(&map_item.id),
                }
                .map_err(|_| DittoError::from(ErrorKind::NonExtant))?
            }
            _ => {
                // For other paths, navigate the JSON structure
                let mut current = &json_value;
                for segment in path.split('.') {
                    match current.get(segment) {
                        Some(value) => current = value,
                        None => return Err(DittoError::from(ErrorKind::NonExtant)),
                    }
                }
                current.clone()
            }
        };

        // Deserialize the extracted value to the requested type
        serde_json::from_value(value).map_err(|_| DittoError::from(ErrorKind::InvalidInput))
    }

    fn typed<T: DeserializeOwned>(&self) -> Result<T, DittoError> {
        // Convert the document to a JSON value first
        let json_value = match self {
            CotDocument::Api(api) => serde_json::to_value(api),
            CotDocument::Chat(chat) => serde_json::to_value(chat),
            CotDocument::File(file) => serde_json::to_value(file),
            CotDocument::MapItem(map_item) => serde_json::to_value(map_item),
        }
        .map_err(|_| DittoError::from(ErrorKind::InvalidInput))?;

        // Deserialize to the requested type
        serde_json::from_value(json_value).map_err(|_| DittoError::from(ErrorKind::InvalidInput))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ditto::schema::{MapItem, MapItemRValue};
    use std::collections::HashMap;

    #[test]
    fn test_ditto_document_id() {
        // Create a test MapItem with minimal required fields
        let map_item = CotDocument::MapItem(MapItem {
            id: "test-id-123".to_string(),
            a: "peer-key".to_string(),
            b: 123.0,
            c: None,
            d: "tak-uid-123".to_string(),
            d_c: 1,
            d_r: false,
            d_v: 2,
            e: "Test Item".to_string(),
            f: None,
            g: "".to_string(),
            h: None,
            i: None,
            j: None,
            k: None,
            l: None,
            n: 1622548800000,
            o: 1622548800000,
            p: "".to_string(),
            q: "".to_string(),
            r: HashMap::new(),
            s: "".to_string(),
            source: None,
            t: "".to_string(),
            u: "".to_string(),
            v: "".to_string(),
            w: "a-f-G-U".to_string(),
        });

        // Test the id() method
        let doc_id = DittoDocument::id(&map_item);
        assert_eq!(doc_id.to_string(), "test-id-123");
    }

    #[test]
    fn test_ditto_document_get() {
        // Create a test MapItem with some data in the r field
        let mut r_map = HashMap::new();
        r_map.insert(
            "test_key".to_string(),
            MapItemRValue::String("test_value".to_string()),
        );

        let map_item = CotDocument::MapItem(MapItem {
            id: "test-id-123".to_string(),
            a: "peer-key".to_string(),
            b: 123.0,
            c: Some("Map Item Title".to_string()),
            d: "tak-uid-123".to_string(),
            d_c: 1,
            d_r: false,
            d_v: 2,
            e: "Test Item".to_string(),
            f: Some(true), // Visibility flag as boolean
            g: "".to_string(),
            h: None,
            i: None,
            j: None,
            k: None,
            l: None,
            n: 1622548800000,
            o: 1622548800000,
            p: "".to_string(),
            q: "".to_string(),
            r: r_map,
            s: "".to_string(),
            source: None,
            t: "".to_string(),
            u: "".to_string(),
            v: "".to_string(),
            w: "a-f-G-U".to_string(),
        });

        // Test the get() method for different paths
        let id: String = DittoDocument::get(&map_item, "_id").unwrap();
        assert_eq!(id, "test-id-123");

        let visibility: bool = DittoDocument::get(&map_item, "f").unwrap();
        assert_eq!(visibility, true);

        // This should fail with PathNotFound
        let result = DittoDocument::get::<String>(&map_item, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_cbor() {
        // Create a simple MapItem
        let map_item = CotDocument::MapItem(MapItem {
            id: "test-id-123".to_string(),
            a: "peer-key".to_string(),
            b: 123.0,
            c: None,
            d: "tak-uid-123".to_string(),
            d_c: 1,
            d_r: false,
            d_v: 2,
            e: "Test Item".to_string(),
            f: None,
            g: "".to_string(),
            h: None,
            i: None,
            j: None,
            k: None,
            l: None,
            n: 1622548800000,
            o: 1622548800000,
            p: "".to_string(),
            q: "".to_string(),
            r: HashMap::new(),
            s: "".to_string(),
            source: None,
            t: "".to_string(),
            u: "".to_string(),
            v: "".to_string(),
            w: "a-f-G-U".to_string(),
        });

        // Test CBOR conversion
        let cbor_result = DittoDocument::to_cbor(&map_item);
        assert!(
            cbor_result.is_ok(),
            "CBOR conversion failed: {:?}",
            cbor_result.err()
        );
    }
}
