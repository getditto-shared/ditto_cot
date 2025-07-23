//! Tests for SDK document conversion utilities

use ditto_cot::ditto::{CotDocument, observer_json_to_cot_document, observer_json_to_json_with_r_fields, get_document_id_from_json, get_document_type_from_json};
use serde_json::json;

#[test]
fn test_observer_json_to_cot_document() {
    // Test MapItem conversion using the observer JSON function
    let map_item_json = json!({
        "_id": "test-map-001",
        "w": "a-u-r-loc-g",
        "a": "test-peer",
        "b": 1642248600000000.0,
        "d": "test-map-001",
        "_c": 1,
        "_r": false,
        "_v": 2,
        "e": "TestUnit",
        "g": "2.0",
        "h": 5.0,
        "i": 10.0,
        "j": 37.7749,
        "k": 2.0,
        "l": -122.4194,
        "n": 1642248600000000.0,
        "o": 1642252200000000.0,
        "p": "h-g-i-g-o",
        "q": "",
        "s": "",
        "t": "",
        "u": "",
        "v": "",
        "r_contact_callsign": "TestUnit",
        "r_track_speed": "15.0"
    });

    let json_str = serde_json::to_string(&map_item_json).unwrap();
    let result = observer_json_to_cot_document(&json_str);
    
    assert!(result.is_ok());
    if let Ok(CotDocument::MapItem(item)) = result {
        assert_eq!(item.id, "test-map-001");
        assert_eq!(item.w, "a-u-r-loc-g");
        assert_eq!(item.j, Some(37.7749));
        assert_eq!(item.l, Some(-122.4194));
    } else {
        panic!("Expected MapItem variant");
    }
}

#[test]
fn test_get_document_id_from_json() {
    let json_str = r#"{"_id": "test-doc-123", "w": "a-u-r-loc-g"}"#;
    let id = get_document_id_from_json(json_str);
    assert_eq!(id, Some("test-doc-123".to_string()));
}

#[test]
fn test_get_document_type_from_json() {
    let json_str = r#"{"_id": "test-doc-123", "w": "a-u-r-loc-g"}"#;
    let doc_type = get_document_type_from_json(json_str);
    assert_eq!(doc_type, Some("a-u-r-loc-g".to_string()));
}

#[test]
fn test_observer_json_to_json_with_r_fields() {
    // Test that flattened r_* fields get reconstructed properly
    let json_with_flattened_r = json!({
        "_id": "test-r-reconstruction",
        "w": "a-u-r-loc-g",
        "a": "test-peer",
        "b": 1642248600000000.0,
        "r_contact_callsign": "TestUnit",
        "r_track_speed": "15.0",
        "r_track_course": "90.0"
    });

    let json_str = serde_json::to_string(&json_with_flattened_r).unwrap();
    let result = observer_json_to_json_with_r_fields(&json_str);
    
    assert!(result.is_ok());
    let reconstructed = result.unwrap();
    
    // Verify r field was reconstructed
    let r_field = reconstructed.get("r").expect("r field should exist");
    assert!(r_field.is_object());
    
    let r_obj = r_field.as_object().unwrap();
    assert!(r_obj.contains_key("contact"));
    assert!(r_obj.contains_key("track"));
    
    // Verify nested structure
    let contact = r_obj.get("contact").unwrap().as_object().unwrap();
    assert_eq!(contact.get("callsign").unwrap().as_str(), Some("TestUnit"));
    
    let track = r_obj.get("track").unwrap().as_object().unwrap();
    assert_eq!(track.get("speed").unwrap().as_str(), Some("15.0"));
    assert_eq!(track.get("course").unwrap().as_str(), Some("90.0"));
}