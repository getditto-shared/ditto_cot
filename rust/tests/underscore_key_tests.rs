use ditto_cot::ditto::schema::MapItem;
use serde_json::json;

/// Tests the handling of underscore-prefixed keys in JSON schema
#[test]
fn test_underscore_key_handling() {
    let map_item = MapItem {
        id: "test-id-123".to_string(),
        a: "peer-key".to_string(),
        b: 10.0,
        c: Some("Test Item".to_string()),
        d: "author-uid".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "Author".to_string(),
        f: Some(true),
        g: String::new(),
        h: None,
        i: None,
        j: None,
        k: None,
        l: None,
        n: 0,
        o: 0,
        p: String::new(),
        q: String::new(),
        r: String::new(),
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: String::new(),
        source: Some("ditto_cot".to_string()), // Add source field
    };
    
    // Serialize to JSON
    let json = serde_json::to_string(&map_item).unwrap();
    println!("Serialized JSON: {}", json);
    
    // Verify that the JSON contains _id 
    assert!(json.contains("\"_id\":"));
    assert!(!json.contains("\"id\":"));
    
    // Verify that _c, _v, _r are used in JSON (not d_c, d_v, d_r)
    assert!(json.contains("\"_c\":"));
    assert!(!json.contains("\"d_c\":"));
    assert!(json.contains("\"_v\":"));
    assert!(!json.contains("\"d_v\":"));
    assert!(json.contains("\"_r\":"));
    assert!(!json.contains("\"d_r\":"));
    
    // Deserialize from JSON with underscore-prefixed keys
    let json_with_underscores = json!({
        "_id": "test-id-456",
        "a": "peer-key-2",
        "b": 20.0,
        "d": "author-uid-2",
        "_c": 1,
        "_r": true,
        "_v": 2,
        "e": "Author 2"
    });
    
    let deserialized: MapItem = serde_json::from_value(json_with_underscores).unwrap();
    
    // Verify that the underscore-prefixed fields were correctly mapped to their Rust field names
    assert_eq!(deserialized.id, "test-id-456");
    assert_eq!(deserialized.d_c, 1);
    assert_eq!(deserialized.d_r, true);
    assert_eq!(deserialized.d_v, 2);
}
