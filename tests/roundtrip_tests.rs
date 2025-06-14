//! Round-trip tests for CoT XML parsing and serialization

use ditto_cot::cot_events::CotEvent;
use ditto_cot::error::CotError;
use chrono::{Utc, TimeZone};

/// Tests round-trip conversion for a location update event
#[test]
fn test_location_update_roundtrip() -> Result<(), CotError> {
    // Create a location update
    let mut event = CotEvent::new_location_update(
        "USER-123",
        "ALPHA-1",
        "Cyan",
        34.12345,
        -118.12345,
        150.0
    );
    
    // Set specific timestamps for testing
    let test_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
    event.time = test_time;
    event.start = test_time;
    event.stale = test_time + chrono::Duration::minutes(5);
    
    // Convert to XML and back
    let xml = event.to_xml()?;
    let parsed = CotEvent::from_xml(&xml)?;
    
    // Verify fields match
    assert_eq!(event.uid, parsed.uid);
    assert_eq!(event.event_type, parsed.event_type);
    assert_eq!(event.time, parsed.time);
    assert_eq!(event.start, parsed.start);
    assert_eq!(event.stale, parsed.stale);
    assert_eq!(event.how, parsed.how);
    assert_eq!(event.point.lat, parsed.point.lat);
    assert_eq!(event.point.lon, parsed.point.lon);
    assert_eq!(event.point.hae, parsed.point.hae);
    assert_eq!(event.detail, parsed.detail);
    
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
        "All Chat Rooms"
    );
    
    // Convert to XML and back
    let xml = event.to_xml()?;
    let parsed = CotEvent::from_xml(&xml)?;
    
    // Verify fields match
    assert_eq!(event.event_type, parsed.event_type);
    assert_eq!(event.detail.get("chat"), parsed.detail.get("chat"));
    assert_eq!(event.detail.get("chatroom"), parsed.detail.get("chatroom"));
    assert_eq!(event.detail.get("senderCallsign"), parsed.detail.get("senderCallsign"));
    
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
        "Need immediate assistance!"
    );
    
    // Convert to XML and back
    let xml = event.to_xml()?;
    let parsed = CotEvent::from_xml(&xml)?;
    
    // Verify fields match
    assert_eq!(event.event_type, parsed.event_type);
    assert_eq!(event.point.lat, parsed.point.lat);
    assert_eq!(event.point.lon, parsed.point.lon);
    assert_eq!(event.detail.get("emergency"), parsed.detail.get("emergency"));
    assert_eq!(event.detail.get("remarks"), parsed.detail.get("remarks"));
    
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
    
    // Verify detail elements
    assert_eq!(event.detail.get("contact.callsign").unwrap(), "ALPHA-1");
    assert_eq!(event.detail.get("contact.phone").unwrap(), "123-456-7890");
    assert_eq!(event.detail.get("__group.name").unwrap(), "Cyan");
    assert_eq!(event.detail.get("__group.role").unwrap(), "Team Member");
    assert_eq!(event.detail.get("track.course").unwrap(), "123.45");
    assert_eq!(event.detail.get("track.speed").unwrap(), "5.0");
    assert_eq!(event.detail.get("status.battery").unwrap(), "85");
    assert_eq!(event.detail.get("usericon.iconsetpath").unwrap(), "COT_MAPPING_2525B/...");
    
    // Test round-trip
    let xml_roundtrip = event.to_xml()?;
    let event_roundtrip = CotEvent::from_xml(&xml_roundtrip)?;
    assert_eq!(event.uid, event_roundtrip.uid);
    assert_eq!(event.event_type, event_roundtrip.event_type);
    assert_eq!(event.detail, event_roundtrip.detail);
    
    Ok(())
}
