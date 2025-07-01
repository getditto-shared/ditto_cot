//! Tests for improved error handling

use ditto_cot::cot_events::CotEvent;
use ditto_cot::error::CotError;
use ditto_cot::xml_parser::parse_cot;

#[test]
fn test_invalid_numeric_error_handling() {
    // Test invalid latitude on point element
    let xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C" time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" stale="2023-01-01T12:05:00Z" how="h-g-i-g-o"><point lat="invalid" lon="0.0" hae="0.0" ce="10.0" le="20.0"/><detail></detail></event>"#;

    let result = CotEvent::from_xml(xml);
    assert!(result.is_err());

    match result.unwrap_err() {
        CotError::InvalidNumeric { field, value, .. } => {
            assert_eq!(field, "lat");
            assert_eq!(value, "invalid");
        }
        _ => panic!("Expected InvalidNumeric error"),
    }
}

#[test]
fn test_invalid_datetime_error_handling() {
    // Test invalid datetime format
    let xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C" time="not-a-date" start="2023-01-01T12:00:00Z" stale="2023-01-01T12:05:00Z" how="h-g-i-g-o"><point lat="0.0" lon="0.0" hae="0.0" ce="10.0" le="20.0"/><detail></detail></event>"#;

    let result = CotEvent::from_xml(xml);
    assert!(result.is_err());

    match result.unwrap_err() {
        CotError::InvalidDateTime { field, value } => {
            assert_eq!(field, "datetime");
            assert_eq!(value, "not-a-date");
        }
        _ => panic!("Expected InvalidDateTime error"),
    }
}

#[test]
fn test_parse_cot_invalid_numeric() {
    // Test parse_cot function with invalid numeric values
    let xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C" time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" stale="2023-01-01T12:05:00Z" how="h-g-i-g-o" lat="0.0" lon="not-a-number" hae="0.0" ce="10.0" le="20.0"><detail></detail></event>"#;

    let result = parse_cot(xml);
    assert!(result.is_err());

    match result.unwrap_err() {
        CotError::InvalidNumeric { field, value, .. } => {
            assert_eq!(field, "lon");
            assert_eq!(value, "not-a-number");
        }
        _ => panic!("Expected InvalidNumeric error"),
    }
}

#[test]
fn test_valid_parsing_still_works() {
    // Ensure valid parsing still works correctly
    let xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C" time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" stale="2023-01-01T12:05:00Z" how="h-g-i-g-o"><point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="20.0"/><detail></detail></event>"#;

    let result = CotEvent::from_xml(xml);
    assert!(result.is_ok());

    let event = result.unwrap();
    assert_eq!(event.uid, "TEST-123");
    assert_eq!(event.point.lat, 34.12345);
    assert_eq!(event.point.lon, -118.12345);
}
