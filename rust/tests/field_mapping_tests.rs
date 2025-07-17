//! Tests for field mapping between CoT XML and Ditto documents
//!
//! This module validates the conversion logic that maps CoT event fields
//! to their corresponding Ditto document fields.

use chrono::{DateTime, Utc};
use ditto_cot::cot_events::{CotEvent, Point};
use ditto_cot::ditto::from_ditto::cot_event_from_ditto_document;
use ditto_cot::ditto::{cot_to_document, CotDocument};
use std::str::FromStr;

/// Test basic CoT to Ditto field mapping
#[test]
fn test_cot_to_ditto_field_mapping() {
    let event = CotEvent {
        version: "2.0".to_string(),
        uid: "TEST-UID-123".to_string(),
        event_type: "a-f-G-U-C".to_string(),
        time: Utc::now(),
        start: Utc::now(),
        stale: Utc::now() + chrono::Duration::minutes(5),
        how: "m-g".to_string(),
        point: Point {
            lat: 34.12345,
            lon: -118.12345,
            hae: 150.0,
            ce: 10.0,
            le: 20.0,
        },
        detail: "<detail><contact callsign=\"ALPHA-1\"/></detail>".to_string(),
    };

    let doc = cot_to_document(&event, "test-peer-key");

    match doc {
        CotDocument::MapItem(map_item) => {
            // Verify UID mapping
            assert_eq!(map_item.id, "TEST-UID-123"); // uid -> _id

            // Verify peer key
            assert_eq!(map_item.a, "test-peer-key"); // peer_key -> a

            // Verify event type mapping
            assert_eq!(map_item.w, "a-f-G-U-C"); // event_type -> w

            // Verify how mapping
            assert_eq!(map_item.p, "m-g"); // how -> p

            // Verify point mapping
            assert_eq!(map_item.j, Some(34.12345)); // lat -> j
            assert_eq!(map_item.l, Some(-118.12345)); // lon -> l
            assert_eq!(map_item.i, Some(150.0)); // hae -> i
            assert_eq!(map_item.h, Some(10.0)); // ce -> h
            assert_eq!(map_item.k, Some(20.0)); // le -> k

            // Verify version mapping
            assert_eq!(map_item.g, "2.0"); // version -> g
        }
        _ => panic!("Expected MapItem document for location event"),
    }
}

/// Test Ditto to CoT field mapping (reverse)
#[test]
fn test_ditto_to_cot_field_mapping() {
    use ditto_cot::ditto::schema::MapItem;
    use std::collections::HashMap;

    let map_item = MapItem {
        id: "REVERSE-TEST-001".to_string(),
        a: "peer-key".to_string(),
        b: 1234567890000.0,
        c: Some("Test Unit".to_string()),
        d: "USER-123".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        e: "BRAVO-2".to_string(),
        f: Some(true),
        g: "2.0".to_string(),
        h: Some(15.0),
        i: Some(200.0),
        j: Some(35.6789),
        k: Some(25.0),
        l: Some(-119.6789),
        n: Some(1234567890000.0),
        o: Some(1234568490000.0),
        p: "h-g-i-g-o".to_string(),
        q: String::new(),
        r: HashMap::new(),
        s: String::new(),
        t: String::new(),
        u: String::new(),
        v: String::new(),
        w: "a-f-G-U-C".to_string(),
        source: None,
    };

    let doc = CotDocument::MapItem(map_item);
    let event = cot_event_from_ditto_document(&doc);

    // Verify reverse mapping
    assert_eq!(event.uid, "REVERSE-TEST-001"); // _id -> uid
    assert_eq!(event.event_type, "a-f-G-U-C"); // w -> event_type
    assert_eq!(event.how, "h-g-i-g-o"); // p -> how
    assert_eq!(event.version, "2.0"); // g -> version

    // Verify point reverse mapping
    assert_eq!(event.point.lat, 35.6789); // j -> lat
    assert_eq!(event.point.lon, -119.6789); // l -> lon
    assert_eq!(event.point.hae, 200.0); // i -> hae
    assert_eq!(event.point.ce, 15.0); // h -> ce
    assert_eq!(event.point.le, 25.0); // k -> le
}

/// Test timestamp field conversions
#[test]
fn test_timestamp_field_conversions() {
    let now = Utc::now();
    let start = now - chrono::Duration::minutes(1);
    let stale = now + chrono::Duration::minutes(10);

    let event = CotEvent {
        version: "2.0".to_string(),
        uid: "TIME-TEST-001".to_string(),
        event_type: "a-f-G-U-C".to_string(),
        time: now,
        start,
        stale,
        how: "m-g".to_string(),
        point: Point::default(),
        detail: "<detail/>".to_string(),
    };

    let doc = cot_to_document(&event, "test-peer");

    match doc {
        CotDocument::MapItem(map_item) => {
            // b field should be time in microseconds
            let expected_micros = now.timestamp_micros() as f64;
            assert!((map_item.b - expected_micros).abs() < 1000.0); // Allow some tolerance

            // n field should be start in microseconds
            let expected_start = start.timestamp_micros() as f64;
            assert_eq!(map_item.n, Some(expected_start));

            // o field should be stale in microseconds
            let expected_stale = stale.timestamp_micros() as f64;
            assert_eq!(map_item.o, Some(expected_stale));
        }
        _ => panic!("Expected MapItem document"),
    }
}

/// Test custom field preservation in detail/r field
#[test]
fn test_custom_field_preservation() {
    let detail_xml = r#"<detail>
        <contact callsign="CHARLIE-3" phone="123-456-7890"/>
        <__group name="Blue" role="Team Lead"/>
        <status battery="85" readiness="green"/>
        <custom_field value="preserve_me" important="true"/>
    </detail>"#;

    let event = CotEvent {
        version: "2.0".to_string(),
        uid: "CUSTOM-TEST-001".to_string(),
        event_type: "a-f-G-U-C".to_string(),
        time: Utc::now(),
        start: Utc::now(),
        stale: Utc::now() + chrono::Duration::minutes(5),
        how: "m-g".to_string(),
        point: Point::default(),
        detail: detail_xml.to_string(),
    };

    let doc = cot_to_document(&event, "test-peer");

    match doc {
        CotDocument::MapItem(map_item) => {
            // Check that custom fields are preserved in r field
            assert!(
                !map_item.r.is_empty(),
                "r field should contain detail elements"
            );
            println!("r field contents: {:?}", map_item.r);

            // The detail parser should have preserved these fields
            // Note: exact structure depends on detail parser implementation
            println!("r field contents: {:?}", map_item.r);
        }
        _ => panic!("Expected MapItem document"),
    }
}

/// Test field type conversions (string to number, etc.)
#[test]
fn test_field_type_conversions() {
    // Test that string coordinates in XML are properly converted to numbers
    let xml = r#"<?xml version="1.0"?>
    <event version="2.0" uid="TYPE-CONV-001" type="a-f-G-U-C" 
           time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" 
           stale="2023-01-01T12:05:00Z" how="m-g">
        <point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="20.0"/>
        <detail/>
    </event>"#;

    let event = CotEvent::from_xml(xml).unwrap();
    let doc = cot_to_document(&event, "test-peer");

    match doc {
        CotDocument::MapItem(map_item) => {
            // Verify string-to-number conversions
            assert_eq!(map_item.j, Some(34.12345));
            assert_eq!(map_item.l, Some(-118.12345));
            assert_eq!(map_item.i, Some(150.0));
            assert_eq!(map_item.h, Some(10.0));
            assert_eq!(map_item.k, Some(20.0));

            // All numeric fields should be proper f64 values
            assert!(map_item.j.is_some());
            assert!(map_item.l.is_some());
        }
        _ => panic!("Expected MapItem document"),
    }
}

/// Test chat message field mappings
#[test]
fn test_chat_message_field_mapping() {
    let chat_event = CotEvent::new_chat_message(
        "CHAT-MAP-001",
        "DELTA-4",
        "Test message content",
        "Operations Room",
        "ops-room-001",
    );

    let doc = cot_to_document(&chat_event, "test-peer");

    match doc {
        CotDocument::Chat(chat) => {
            assert_eq!(chat.id, "CHAT-MAP-001"); // UID preserved correctly
            assert_eq!(chat.message, Some("Test message content".to_string()));
            assert_eq!(chat.room, Some("Operations Room".to_string()));
            assert_eq!(chat.room_id, Some("ops-room-001".to_string()));
            assert_eq!(chat.e, "DELTA-4"); // callsign -> e
            assert_eq!(chat.author_callsign, Some("DELTA-4".to_string()));
        }
        _ => panic!("Expected Chat document for chat event"),
    }
}

/// Test emergency event field mappings
#[test]
fn test_emergency_event_field_mapping() {
    let emrg_event = CotEvent::new_emergency(
        "EMRG-MAP-001",
        "ECHO-5",
        36.0,
        -121.0,
        "Emergency-911",
        "Medical assistance required",
    );

    let doc = cot_to_document(&emrg_event, "test-peer");

    // Note: new_emergency creates "b-a-o-can" type, which maps to Generic, not Api
    match doc {
        CotDocument::Generic(generic) => {
            assert_eq!(generic.id, "EMRG-MAP-001");
            assert_eq!(generic.e, "ECHO-5");
            assert_eq!(generic.w, "b-a-o-can");
            assert_eq!(generic.j, Some(36.0));
            assert_eq!(generic.l, Some(-121.0));

            // Emergency details should be in r field
            assert!(!generic.r.is_empty());
        }
        _ => panic!("Expected Generic document for emergency event with b-a-o-can type"),
    }
}

/// Test file event field mappings
#[test]
fn test_file_event_field_mapping() {
    let file_xml = r#"<?xml version="1.0"?>
    <event version="2.0" uid="FILE-MAP-001" type="b-f-t-file" 
           time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" 
           stale="2023-01-01T12:05:00Z" how="h-e">
        <point lat="0" lon="0" hae="0" ce="0" le="0"/>
        <detail>
            <file name="test_document.pdf" size="2048000" hash="abc123def456"/>
        </detail>
    </event>"#;

    let event = CotEvent::from_xml(file_xml).unwrap();
    let doc = cot_to_document(&event, "test-peer");

    match doc {
        CotDocument::File(file) => {
            assert_eq!(file.id, "FILE-MAP-001");
            assert_eq!(file.w, "b-f-t-file");
            assert_eq!(file.p, "h-e");

            // File details should be preserved
            // Note: exact mapping depends on detail parser
        }
        _ => panic!("Expected File document for file event"),
    }
}

/// Test mapping of special CoT fields (access, qos, opex, caveat, releasability)
#[test]
fn test_special_cot_field_mapping() {
    let xml = r#"<?xml version="1.0"?>
    <event version="2.0" uid="SPECIAL-001" type="a-f-G-U-C" 
           time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" 
           stale="2023-01-01T12:05:00Z" how="m-g"
           access="Unclassified" qos="priority" opex="OPERATION-BLUE"
           caveat="FOUO" releasableTo="USA,GBR,CAN,AUS,NZL">
        <point lat="34.0" lon="-118.0" hae="100" ce="10" le="10"/>
        <detail/>
    </event>"#;

    // Note: Standard CoT XML doesn't include these as event attributes
    // They would typically be in the detail section
    // This test documents expected behavior if they were mapped

    let event = CotEvent::from_xml(xml).unwrap();
    let doc = cot_to_document(&event, "test-peer");

    match doc {
        CotDocument::MapItem(map_item) => {
            // Document the field mappings:
            // access -> q
            // qos -> t
            // opex -> s
            // caveat -> u
            // releasableTo -> v

            // These would need to be extracted from detail or custom parsing
            // Current implementation may use defaults
            assert_eq!(map_item.q, ""); // access field
            assert_eq!(map_item.t, ""); // qos field
            assert_eq!(map_item.s, ""); // opex field
            assert_eq!(map_item.u, ""); // caveat field
            assert_eq!(map_item.v, ""); // releasableTo field
        }
        _ => panic!("Expected MapItem document"),
    }
}

/// Test that unknown event types map to Generic document
#[test]
fn test_unknown_type_to_generic_mapping() {
    let unknown_event = CotEvent {
        version: "2.0".to_string(),
        uid: "UNKNOWN-001".to_string(),
        event_type: "x-custom-special-type".to_string(),
        time: Utc::now(),
        start: Utc::now(),
        stale: Utc::now() + chrono::Duration::minutes(5),
        how: "m-g".to_string(),
        point: Point::default(),
        detail: "<detail><custom>Special data</custom></detail>".to_string(),
    };

    let doc = cot_to_document(&unknown_event, "test-peer");

    match doc {
        CotDocument::Generic(generic) => {
            assert_eq!(generic.id, "UNKNOWN-001");
            assert_eq!(generic.w, "x-custom-special-type");
            // Generic should preserve all standard fields
            assert_eq!(generic.p, "m-g");
            assert_eq!(generic.g, "2.0");
        }
        _ => panic!("Expected Generic document for unknown event type"),
    }
}

/// Test sensor event type mappings (a-u-S, a-u-A, a-u-G)
#[test]
fn test_sensor_event_type_mapping() {
    let sensor_types = vec![
        ("a-u-S", "Unmanned System - Sensor"),
        ("a-u-A", "Unmanned System - Air"),
        ("a-u-G", "Unmanned System - Ground"),
    ];

    for (event_type, _description) in sensor_types {
        let mut event = CotEvent::new_location_update(
            &format!("SENSOR-{}", event_type),
            "SENSOR-1",
            "Blue",
            35.0,
            -120.0,
            500.0,
        );
        event.event_type = event_type.to_string();

        let doc = cot_to_document(&event, "test-peer");

        match doc {
            CotDocument::MapItem(map_item) => {
                assert_eq!(map_item.w, event_type);
                assert_eq!(map_item.id, format!("SENSOR-{}", event_type));
                // Sensor events should map to MapItem
            }
            _ => panic!("Expected MapItem for sensor event type {}", event_type),
        }
    }
}

/// Test round-trip field preservation
#[test]
fn test_round_trip_field_preservation() {
    let original_event = CotEvent {
        version: "2.0".to_string(),
        uid: "ROUND-TRIP-001".to_string(),
        event_type: "a-f-G-U-C".to_string(),
        time: DateTime::from_str("2023-06-15T14:30:00Z").unwrap(),
        start: DateTime::from_str("2023-06-15T14:30:00Z").unwrap(),
        stale: DateTime::from_str("2023-06-15T14:35:00Z").unwrap(),
        how: "m-g".to_string(),
        point: Point {
            lat: 37.123456,
            lon: -122.123456,
            hae: 123.45,
            ce: 5.5,
            le: 10.5,
        },
        detail: r#"<detail><contact callsign="FOXTROT-6"/></detail>"#.to_string(),
    };

    // Convert to Ditto document
    let doc = cot_to_document(&original_event, "test-peer");

    // Convert back to CoT event
    let recovered_event = cot_event_from_ditto_document(&doc);

    // Verify all fields are preserved
    assert_eq!(original_event.uid, recovered_event.uid);
    assert_eq!(original_event.event_type, recovered_event.event_type);
    assert_eq!(original_event.version, recovered_event.version);
    assert_eq!(original_event.how, recovered_event.how);

    // Points should match (with floating point tolerance)
    assert!((original_event.point.lat - recovered_event.point.lat).abs() < 0.000001);
    assert!((original_event.point.lon - recovered_event.point.lon).abs() < 0.000001);
    assert!((original_event.point.hae - recovered_event.point.hae).abs() < 0.01);
    assert!((original_event.point.ce - recovered_event.point.ce).abs() < 0.1);
    assert!((original_event.point.le - recovered_event.point.le).abs() < 0.1);

    // Times should be within 1 second (due to millisecond conversion)
    let time_diff = (original_event.time - recovered_event.time)
        .num_seconds()
        .abs();
    assert!(time_diff <= 1);
}
