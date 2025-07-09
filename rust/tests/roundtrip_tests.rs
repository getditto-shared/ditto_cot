//! Round-trip tests for CoT XML parsing and serialization

use chrono::{TimeZone, Utc};
use ditto_cot::cot_events::CotEvent;
use ditto_cot::ditto::from_ditto::cot_event_from_ditto_document;
use ditto_cot::ditto::{cot_to_document, CotDocument};
use ditto_cot::error::CotError;

/// Tests round-trip conversion for a location update event
#[test]
fn test_location_update_roundtrip() -> Result<(), CotError> {
    // Create a location update
    let mut event =
        CotEvent::new_location_update("USER-123", "ALPHA-1", "Cyan", 34.12345, -118.12345, 150.0);

    // Set specific timestamps for testing
    let test_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
    event.time = test_time;
    event.start = test_time;
    event.stale = test_time + chrono::Duration::minutes(5);

    // Convert to XML and back
    let xml = event.to_xml()?;
    let parsed = CotEvent::from_xml(&xml)?;

    // Ditto round-trip: CoT -> Ditto -> CoT -> XML
    let ditto_doc = ditto_cot::ditto::cot_to_document(&parsed, "test-peer");
    assert!(ditto_doc.is_map_item());
    assert!(ditto_doc.as_map_item().is_some());
    assert!(ditto_doc.has_key("_id"));

    // This verifies that Serde is serializing the JSON with the "_id" field included
    let json = serde_json::to_string(&ditto_doc).unwrap();
    println!("Ditto JSON: {}", json);
    assert!(json.contains("_id"));

    let cot_from_ditto = cot_event_from_ditto_document(&ditto_doc);
    let xml_roundtrip = cot_from_ditto.to_xml()?;
    let parsed_roundtrip = CotEvent::from_xml(&xml_roundtrip)?;

    // Compare important fields for equivalence
    assert_eq!(event.uid, parsed_roundtrip.uid);
    assert_eq!(event.event_type, parsed_roundtrip.event_type);
    assert_eq!(event.time, parsed_roundtrip.time);
    assert_eq!(event.start, parsed_roundtrip.start);
    assert_eq!(event.stale, parsed_roundtrip.stale);
    assert_eq!(event.how, parsed_roundtrip.how);
    assert!((event.point.lat - parsed_roundtrip.point.lat).abs() < 1e-6);
    assert!((event.point.lon - parsed_roundtrip.point.lon).abs() < 1e-6);
    assert!((event.point.hae - parsed_roundtrip.point.hae).abs() < 1e-3);
    // Details may lose non-schema fields, so only check presence of key fields if needed

    Ok(())
}

/// Tests round-trip conversion for a chat message event
#[test]
fn test_chat_message_roundtrip() -> Result<(), CotError> {
    // Create a chat message
    let event = CotEvent::new_chat_message(
        "USER-123",
        "ALPHA-1",
        "Test message",
        "All Chat Rooms",
        "All Chat Rooms",
    );

    // Convert to XML and back
    let xml = event.to_xml()?;
    let parsed = CotEvent::from_xml(&xml)?;

    // Verify fields match
    assert_eq!(event.event_type, parsed.event_type);
    // Detail is now a string; just check it's non-empty and roundtrips
    assert!(!event.detail.is_empty());
    assert_eq!(event.detail, parsed.detail);
    assert!(
        event.detail.trim_start().starts_with("<detail"),
        "Detail should start with <detail>"
    );
    assert!(
        event.detail.trim_end().ends_with("</detail>"),
        "Detail should end with </detail>"
    );

    Ok(())
}

/// Tests round-trip conversion for an emergency event
#[test]
fn test_emergency_roundtrip() -> Result<(), CotError> {
    // Create an emergency event
    let event = CotEvent::new_emergency(
        "USER-123",
        "ALPHA-1",
        34.12345,
        -118.12345,
        "Emergency-911",
        "Need immediate assistance!",
    );

    // Convert to XML and back
    let xml = event.to_xml()?;
    let parsed = CotEvent::from_xml(&xml)?;

    // Verify fields match
    assert_eq!(event.event_type, parsed.event_type);
    assert_eq!(event.point.lat, parsed.point.lat);
    assert_eq!(event.point.lon, parsed.point.lon);
    // Detail is now a string; just check it's non-empty and roundtrips
    assert!(!event.detail.is_empty());
    assert_eq!(event.detail, parsed.detail);
    assert!(
        event.detail.trim_start().starts_with("<detail"),
        "Detail should start with <detail>"
    );
    assert!(
        event.detail.trim_end().ends_with("</detail>"),
        "Detail should end with </detail>"
    );

    Ok(())
}

/// Tests parsing of a complete CoT message with various elements
#[test]
fn test_complete_cot_parsing() -> Result<(), CotError> {
    let xml = r#"
    <event version="2.0" 
          uid="TEST-123" 
          type="a-f-G-U-C" 
          time="2023-01-01T12:00:00Z" 
          start="2023-01-01T12:00:00Z" 
          stale="2023-01-01T12:05:00Z" 
          how="h-g-i-g-o">
        <point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="20.0"/>
    <detail>
    <contact callsign="ALPHA-1" phone="123-456-7890"/>
    <__group name="Cyan" role="Team Member"/>
    <track course="123.45" speed="5.0"/>
    <status battery="85"/>
    <usericon iconsetpath="COT_MAPPING_2525B/..."/>
    </detail>
    </event>"#;

    let event = CotEvent::from_xml(xml)?;

    // Verify basic fields
    assert_eq!(event.uid, "TEST-123");
    assert_eq!(event.event_type, "a-f-G-U-C");
    assert_eq!(event.point.lat, 34.12345);
    assert_eq!(event.point.lon, -118.12345);

    // Since detail is now a raw string, check for presence of expected XML fragments as substrings
    let normalized_detail = event.detail.replace(['\n', '\r', ' '], "");
    if !normalized_detail.contains("<contactcallsign=\"ALPHA-1\"phone=\"123-456-7890\"/>") {
        println!("event.detail: {}", event.detail);
    }
    assert!(
        normalized_detail.contains("<contactcallsign=\"ALPHA-1\"phone=\"123-456-7890\"/>"),
        "Detail should contain contact element as raw string"
    );
    assert!(
        event.detail.contains("<__group name=\"Cyan\""),
        "Detail should contain __group element as raw string"
    );
    assert!(
        event.detail.contains("<track course=\"123.45\""),
        "Detail should contain track element as raw string"
    );
    assert!(
        event.detail.contains("<status battery=\"85\""),
        "Detail should contain status element as raw string"
    );
    assert!(
        event
            .detail
            .contains("<usericon iconsetpath=\"COT_MAPPING_2525B/...\""),
        "Detail should contain usericon element as raw string"
    );

    // Test round-trip
    let xml_roundtrip = event.to_xml()?;
    let event_roundtrip = CotEvent::from_xml(&xml_roundtrip)?;
    assert_eq!(event.uid, event_roundtrip.uid);
    assert_eq!(
        event.detail.trim(),
        event_roundtrip.detail.trim(),
        "Detail string should roundtrip"
    );

    Ok(())
}

/// Tests round-trip conversion for sensor/unmanned system (a-u-S) format
#[test]
fn test_sensor_unmanned_system_roundtrip() -> Result<(), CotError> {
    let xml = r#"<?xml version="1.0" standalone="yes"?>
<event
how="m-d-a"
stale="2025-07-05T21:30:00Z"
start="2025-07-05T21:00:00Z"
time="2025-07-05T21:00:00Z"
type="a-u-S"
uid="sensor-unmanned-001"
version="2.0">
<detail>
<sensor type="thermal" status="active" temperature="85.5"/>
<platform name="UAV-SENSOR-01" model="Predator"/>
<battery level="78" voltage="24.2"/>
<remarks>Thermal sensor platform on patrol route Alpha</remarks>
</detail>
</event>
<track course="30.86376880675669" speed="1.3613854354920412" />
<point ce="500.0" hae="0.0" lat="37.32699544764403" le="100.0" lon="-75.2905272033264" />"#;

    // Parse the CoT XML
    let event = CotEvent::from_xml(xml)?;

    // Verify basic event properties
    assert_eq!(event.event_type, "a-u-S");
    assert_eq!(event.uid, "sensor-unmanned-001");
    assert_eq!(event.how, "m-d-a");
    assert_eq!(event.version, "2.0");

    // Verify point data
    assert_eq!(event.point.lat, 37.32699544764403);
    assert_eq!(event.point.lon, -75.2905272033264);
    assert_eq!(event.point.hae, 0.0);
    assert_eq!(event.point.ce, 500.0);
    assert_eq!(event.point.le, 100.0);

    // Verify detail elements are preserved
    assert!(event.detail.contains("sensor type=\"thermal\""));
    assert!(event.detail.contains("platform name=\"UAV-SENSOR-01\""));
    assert!(event.detail.contains("battery level=\"78\""));

    // Test CoT -> Ditto -> CoT round-trip
    let ditto_doc = cot_to_document(&event, "test-source");

    // Verify it resolves to MapItem
    match &ditto_doc {
        CotDocument::MapItem(map_item) => {
            assert_eq!(map_item.w, "a-u-S"); // Event type
            assert_eq!(map_item.p, "m-d-a"); // How field

            // Verify point data (j=LAT, l=LON, i=HAE)
            assert_eq!(map_item.j, Some(37.32699544764403));
            assert_eq!(map_item.l, Some(-75.2905272033264));
            assert_eq!(map_item.i, Some(0.0));
        }
        _ => panic!("Expected MapItem document for a-u-S CoT format"),
    }

    // Convert back from Ditto to CoT
    let recovered_event = cot_event_from_ditto_document(&ditto_doc);

    // Verify key fields are preserved
    assert_eq!(recovered_event.event_type, "a-u-S");
    assert_eq!(recovered_event.how, "m-d-a");
    assert_eq!(recovered_event.point.lat, 37.32699544764403);
    assert_eq!(recovered_event.point.lon, -75.2905272033264);

    // Test XML round-trip
    let xml_output = event.to_xml()?;
    let parsed_again = CotEvent::from_xml(&xml_output)?;

    assert_eq!(event.event_type, parsed_again.event_type);
    assert_eq!(event.uid, parsed_again.uid);
    assert_eq!(event.how, parsed_again.how);

    Ok(())
}

/// Tests manual data acquisition sensor variants
#[test]
fn test_manual_data_acquisition_sensors() -> Result<(), CotError> {
    let test_cases = vec![
        ("a-u-S", "Unmanned System - Sensor"),
        ("a-u-A", "Unmanned System - Air"),
        ("a-u-G", "Unmanned System - Ground"),
    ];

    for (event_type, description) in test_cases {
        let xml = format!(
            r#"<event version="2.0" type="{}" uid="test-{}" time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" stale="2023-01-01T12:30:00Z" how="m-d-a">
  <point lat="35.0" lon="-120.0" hae="100.0" ce="50.0" le="25.0"/>
  <detail>
    <sensor type="optical" status="active"/>
    <acquisition method="manual"/>
    <remarks>{}</remarks>
  </detail>
</event>"#,
            event_type,
            event_type.replace("-", "_"),
            description
        );

        // Parse and verify
        let event = CotEvent::from_xml(&xml)?;
        assert_eq!(event.event_type, event_type);
        assert_eq!(event.how, "m-d-a");

        // Convert to Ditto and verify it resolves to MapItem
        let ditto_doc = cot_to_document(&event, "test-source");
        match &ditto_doc {
            CotDocument::MapItem(map_item) => {
                assert_eq!(map_item.w, event_type);
                assert_eq!(map_item.p, "m-d-a");
            }
            _ => panic!("Expected MapItem for {}", event_type),
        }

        // Test XML round-trip
        let xml_output = event.to_xml()?;
        let parsed_again = CotEvent::from_xml(&xml_output)?;
        assert_eq!(event.event_type, parsed_again.event_type);
        assert_eq!(event.how, parsed_again.how);
    }

    Ok(())
}
