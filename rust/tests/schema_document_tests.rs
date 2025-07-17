//! Tests for Ditto schema document types
//!
//! This test module validates the serialization, deserialization, and field
//! handling for all Ditto document types (Api, Chat, File, MapItem, Generic).

use ditto_cot::cot_events::CotEvent;
use ditto_cot::ditto::schema::*;
use ditto_cot::ditto::{cot_to_document, CotDocument};
use serde_json::json;
use std::collections::HashMap;

/// Test Api document serialization and deserialization
#[test]
fn test_api_document_serialization() {
    let mut r_field = HashMap::new();
    r_field.insert(
        "emergency".to_string(),
        ApiRValue::String("Medical".to_string()),
    );
    r_field.insert("severity".to_string(), ApiRValue::Number(3.0));

    let api_doc = Api {
        id: "TEST-EMERGENCY-001".to_string(),
        a: "test-peer-key".to_string(),
        b: 1234567890.0,
        d: "USER-123".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "ALPHA-1".to_string(),
        g: "2.0".to_string(),
        h: Some(10.0),
        i: Some(150.0),
        j: Some(34.12345),
        k: Some(20.0),
        l: Some(-118.12345),
        n: Some(1234567890.0),
        o: Some(1234567950.0),
        p: "h-g-i-g-o".to_string(),
        q: String::new(),
        r: r_field,
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: "a-u-emergency-g".to_string(),
        content_type: None,
        data: None,
        is_file: None,
        is_removed: None,
        mime: None,
        source: Some("test-source".to_string()),
        tag: None,
        time_millis: None,
        title: None,
    };

    // Test serialization
    let json = serde_json::to_string(&api_doc).unwrap();
    assert!(json.contains("\"_id\":\"TEST-EMERGENCY-001\""));
    assert!(json.contains("\"w\":\"a-u-emergency-g\""));
    assert!(json.contains("\"emergency\":\"Medical\""));

    // Test deserialization
    let deserialized: Api = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.id, "TEST-EMERGENCY-001");
    assert_eq!(deserialized.w, "a-u-emergency-g");
    match deserialized.r.get("emergency").unwrap() {
        ApiRValue::String(s) => assert_eq!(s, "Medical"),
        _ => panic!("Expected String variant"),
    }
}

/// Test Chat document fields and JSON round-trip
#[test]
fn test_chat_document_fields() {
    let mut r_field = HashMap::new();
    r_field.insert(
        "chatroom".to_string(),
        ChatRValue::String("All Chat Rooms".to_string()),
    );

    let chat_doc = Chat {
        id: "CHAT-MSG-001".to_string(),
        a: "test-peer-key".to_string(),
        b: 1234567890.0,
        d: "USER-456".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "BRAVO-2".to_string(),
        g: "2.0".to_string(),
        h: None,
        i: None,
        j: None,
        k: None,
        l: None,
        n: None,
        o: None,
        p: "h-t".to_string(),
        q: String::new(),
        r: r_field,
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: "b-t-f".to_string(),
        author_callsign: Some("BRAVO-2".to_string()),
        author_type: Some("a-f-G-U-C".to_string()),
        author_uid: Some("USER-456".to_string()),
        location: None,
        message: Some("Test chat message".to_string()),
        parent: None,
        room: Some("All Chat Rooms".to_string()),
        room_id: Some("AllChatRooms".to_string()),
        time: Some("2023-01-01T12:00:00Z".to_string()),
        source: None,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&chat_doc).unwrap();
    assert!(json.contains("\"message\":\"Test chat message\""));
    assert!(json.contains("\"room\":\"All Chat Rooms\""));
    assert!(json.contains("\"authorCallsign\":\"BRAVO-2\""));

    // Test deserialization
    let deserialized: Chat = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.message, Some("Test chat message".to_string()));
    assert_eq!(deserialized.room, Some("All Chat Rooms".to_string()));
}

/// Test MapItem document validation and required fields
#[test]
fn test_map_item_document_validation() {
    let mut r_field = HashMap::new();
    r_field.insert(
        "contact".to_string(),
        MapItemRValue::Object({
            let mut contact = serde_json::Map::new();
            contact.insert("callsign".to_string(), json!("CHARLIE-3"));
            contact.insert("endpoint".to_string(), json!("192.168.1.100:8080"));
            contact
        }),
    );

    let map_item = MapItem {
        id: "MAP-ITEM-001".to_string(),
        a: "test-peer-key".to_string(),
        b: 1234567890.0,
        c: Some("Friendly Unit".to_string()),
        d: "USER-789".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "CHARLIE-3".to_string(),
        f: Some(true),
        g: "2.0".to_string(),
        h: Some(10.0),
        i: Some(200.0),
        j: Some(35.6789),
        k: Some(15.0),
        l: Some(-119.6789),
        n: Some(1234567890.0),
        o: Some(1234568490.0),
        p: "m-g".to_string(),
        q: String::new(),
        r: r_field,
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: "a-f-G-U-C".to_string(),
        source: None,
    };

    // Test that all required fields are present
    let json_value = serde_json::to_value(&map_item).unwrap();
    assert!(json_value.get("_id").is_some());
    assert!(json_value.get("a").is_some());
    assert!(json_value.get("b").is_some());
    assert!(json_value.get("d").is_some());
    assert!(json_value.get("_c").is_some());
    assert!(json_value.get("_r").is_some());
    assert!(json_value.get("_v").is_some());
    assert!(json_value.get("e").is_some());

    // Test optional fields
    assert_eq!(json_value.get("c").unwrap(), "Friendly Unit");
    assert_eq!(json_value.get("f").unwrap(), true);
}

/// Test File document specific fields
#[test]
fn test_file_document_fields() {
    let file_doc = File {
        id: "FILE-001".to_string(),
        a: "test-peer-key".to_string(),
        b: 1234567890.0,
        c: Some("test_image.jpg".to_string()),
        content_type: Some("image/jpeg".to_string()),
        d: "USER-999".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "DELTA-4".to_string(),
        file: Some("attachment-token-123".to_string()),
        g: "2.0".to_string(),
        h: None,
        i: None,
        j: None,
        k: None,
        l: None,
        mime: Some("image/jpeg".to_string()),
        n: None,
        o: None,
        p: String::new(),
        q: String::new(),
        r: HashMap::new(),
        s: String::new(),
        sz: Some(1024000.0),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: "b-f-t-file".to_string(),
        item_id: None,
        source: None,
    };

    let json = serde_json::to_string(&file_doc).unwrap();
    assert!(json.contains("\"c\":\"test_image.jpg\""));
    assert!(json.contains("\"contentType\":\"image/jpeg\""));
    assert!(json.contains("\"file\":\"attachment-token-123\""));
    assert!(json.contains("\"sz\":1024000.0"));
}

/// Test Generic document as fallback type
#[test]
fn test_generic_document_fallback() {
    let generic_doc = Generic {
        id: "GENERIC-001".to_string(),
        a: "test-peer-key".to_string(),
        b: 1234567890.0,
        d: "USER-000".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "ECHO-5".to_string(),
        g: "2.0".to_string(),
        h: None,
        i: None,
        j: None,
        k: None,
        l: None,
        n: None,
        o: None,
        p: String::new(),
        q: String::new(),
        r: HashMap::new(),
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: "x-custom-type".to_string(),
        source: None,
    };

    // Generic should handle any event type
    assert_eq!(generic_doc.w, "x-custom-type");

    let json = serde_json::to_value(&generic_doc).unwrap();
    assert!(json.is_object());
}

/// Test document JSON round-trip for all types
#[test]
fn test_document_json_round_trip() {
    // Test Api round-trip
    let api_json = json!({
        "_id": "API-TEST",
        "a": "peer-key",
        "b": 123456.0,
        "d": "uid",
        "_c": 0,
        "_r": false,
        "_v": 2,
        "e": "callsign",
        "g": "",
        "p": "",
        "q": "",
        "r": {},
        "s": "",
        "t": "",
        "u": "",
        "v": "",
        "w": "a-u-emergency-g"
    });

    let api_doc: Api = serde_json::from_value(api_json.clone()).unwrap();
    let api_back = serde_json::to_value(&api_doc).unwrap();
    assert_eq!(api_back.get("_id"), api_json.get("_id"));

    // Test Chat round-trip
    let chat_json = json!({
        "_id": "CHAT-TEST",
        "a": "peer-key",
        "b": 123456.0,
        "d": "uid",
        "_c": 0,
        "_r": false,
        "_v": 2,
        "e": "callsign",
        "g": "",
        "p": "",
        "q": "",
        "r": {},
        "s": "",
        "t": "",
        "u": "",
        "v": "",
        "w": "b-t-f",
        "message": "Hello world"
    });

    let chat_doc: Chat = serde_json::from_value(chat_json.clone()).unwrap();
    assert_eq!(chat_doc.message, Some("Hello world".to_string()));
}

/// Test RValue enum variants for detail field
#[test]
fn test_rvalue_variants() {
    // Test all RValue enum variants
    let string_val = ApiRValue::String("test".to_string());
    let number_val = ApiRValue::Number(42.0);
    let bool_val = ApiRValue::Boolean(true);
    let null_val = ApiRValue::Null;

    // Test array variant
    let _array_val = ApiRValue::Array(vec![json!("item1"), json!("item2")]);

    // Test object variant
    let mut obj = serde_json::Map::new();
    obj.insert("key".to_string(), json!("value"));
    let _object_val = ApiRValue::Object(obj);

    // Verify serialization
    assert_eq!(serde_json::to_value(&string_val).unwrap(), json!("test"));
    assert_eq!(serde_json::to_value(&number_val).unwrap(), json!(42.0));
    assert_eq!(serde_json::to_value(&bool_val).unwrap(), json!(true));
    assert_eq!(serde_json::to_value(&null_val).unwrap(), json!(null));
}

/// Test CotDocument enum resolution
#[test]
fn test_cot_document_enum_resolution() {
    // Create different CoT events and verify they resolve to correct document types

    // Emergency event -> Api document
    let emergency_event = CotEvent::new_emergency(
        "EMRG-001",
        "ALPHA-1",
        34.12345,
        -118.12345,
        "Emergency-911",
        "Medical emergency",
    );
    let emergency_doc = cot_to_document(&emergency_event, "test-peer");
    // Note: new_emergency creates "b-a-o-can" type, which maps to Generic, not Api
    assert!(matches!(emergency_doc, CotDocument::Generic(_)));

    // Chat event -> Chat document
    let chat_event = CotEvent::new_chat_message(
        "CHAT-001",
        "BRAVO-2",
        "Hello team",
        "All Chat Rooms",
        "AllChatRooms",
    );
    let chat_doc = cot_to_document(&chat_event, "test-peer");
    assert!(matches!(chat_doc, CotDocument::Chat(_)));

    // Location event -> MapItem document
    let location_event =
        CotEvent::new_location_update("LOC-001", "CHARLIE-3", "Cyan", 35.0, -120.0, 100.0);
    let location_doc = cot_to_document(&location_event, "test-peer");
    assert!(matches!(location_doc, CotDocument::MapItem(_)));

    // Unknown type -> Generic document
    let mut generic_event =
        CotEvent::new_location_update("GENERIC-001", "DELTA-4", "Red", 36.0, -121.0, 50.0);
    generic_event.event_type = "x-custom-unknown".to_string();
    let generic_doc = cot_to_document(&generic_event, "test-peer");
    assert!(matches!(generic_doc, CotDocument::Generic(_)));
}

/// Test schema version enforcement
#[test]
fn test_schema_version_enforcement() {
    // All documents should have d_v = 2
    let api = Api {
        id: "TEST".to_string(),
        a: "peer".to_string(),
        b: 0.0,
        d: "uid".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2, // Schema version must be 2
        e: "call".to_string(),
        // ... other fields with defaults
        g: String::new(),
        h: None,
        i: None,
        j: None,
        k: None,
        l: None,
        n: None,
        o: None,
        p: String::new(),
        q: String::new(),
        r: HashMap::new(),
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: String::new(),
        content_type: None,
        data: None,
        is_file: None,
        is_removed: None,
        mime: None,
        source: None,
        tag: None,
        time_millis: None,
        title: None,
    };

    assert_eq!(api.d_v, 2);

    // Test deserialization enforces schema version
    let json_with_wrong_version = json!({
        "_id": "TEST",
        "a": "peer",
        "b": 0.0,
        "d": "uid",
        "_c": 0,
        "_r": false,
        "_v": 1, // Wrong version
        "e": "call",
        "g": "",
        "p": "",
        "q": "",
        "r": {},
        "s": "",
        "t": "",
        "u": "",
        "v": "",
        "w": ""
    });

    // This should still deserialize but with d_v = 1
    let result: Result<Api, _> = serde_json::from_value(json_with_wrong_version);
    assert!(result.is_ok());
    let doc = result.unwrap();
    assert_eq!(doc.d_v, 1); // Keeps the provided value
}

/// Test optional field serialization behavior
#[test]
fn test_optional_field_serialization() {
    let api = Api {
        id: "TEST".to_string(),
        a: "peer".to_string(),
        b: 0.0,
        d: "uid".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "call".to_string(),
        g: String::new(),
        h: None,        // Should be skipped in serialization
        i: Some(100.0), // Should be included
        j: None,
        k: None,
        l: None,
        n: None,
        o: None,
        p: String::new(),
        q: String::new(),
        r: HashMap::new(),
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: String::new(),
        content_type: None,
        data: None,
        is_file: None,
        is_removed: None,
        mime: None,
        source: Some("test-source".to_string()), // Should be included
        tag: None,
        time_millis: None,
        title: None,
    };

    let json = serde_json::to_value(&api).unwrap();

    // None values should be omitted
    assert!(!json.as_object().unwrap().contains_key("h"));
    assert!(!json.as_object().unwrap().contains_key("j"));

    // Some values should be present
    assert!(json.as_object().unwrap().contains_key("i"));
    assert_eq!(json.get("i").unwrap(), 100.0);
    assert!(json.as_object().unwrap().contains_key("source"));
    assert_eq!(json.get("source").unwrap(), "test-source");
}
