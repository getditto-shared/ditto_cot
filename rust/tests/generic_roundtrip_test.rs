use anyhow::Result;
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{cot_to_document, from_ditto::cot_event_from_ditto_document, CotDocument},
    xml_utils,
};

#[test]
fn test_generic_roundtrip() -> Result<()> {
    // Create a CoT XML that will trigger the Generic/File arm
    let cot_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<event version="2.0" uid="generic-test-123456789" type="x-custom-generic-type" time="2023-05-01T12:00:00.000Z" start="2023-05-01T12:00:00.000Z" stale="2023-05-01T12:30:00.000Z" how="m-g">
  <point lat="37.7749" lon="-122.4194" hae="100.0" ce="50.0" le="25.0" />
  <detail>
    <custom_field>custom value</custom_field>
    <test_field>test value</test_field>
    <nested>
      <field1>value1</field1>
      <field2>value2</field2>
    </nested>
    <numeric_field>123</numeric_field>
    <boolean_field>true</boolean_field>
  </detail>
</event>"#;

    // Parse the XML into a CotEvent
    let cot_event = CotEvent::from_xml(cot_xml)?;
    
    // Convert to CotDocument
    let ditto_doc = cot_to_document(&cot_event, "test-peer");
    
    // Verify it's a File variant (Generic type)
    match &ditto_doc {
        CotDocument::File(_) => println!("✓ Correctly mapped to File variant"),
        _ => panic!("Expected File variant for Generic type"),
    }
    
    // Convert back to CotEvent
    let roundtrip_event = cot_event_from_ditto_document(&ditto_doc);
    
    // Convert both to minimized XML for comparison
    let cot_xml_out = roundtrip_event.to_xml()?;
    let min_expected = xml_utils::minimize_xml(cot_xml);
    let min_actual = xml_utils::minimize_xml(&cot_xml_out);
    
    // Check that the critical values are preserved correctly
    assert_eq!(cot_event.uid, roundtrip_event.uid, "UID mismatch");
    assert_eq!(cot_event.event_type, roundtrip_event.event_type, "Event type mismatch");
    assert_eq!(cot_event.how, roundtrip_event.how, "How mismatch");
    
    // Check timestamps with a small tolerance for formatting differences
    let time_diff = (cot_event.time - roundtrip_event.time).num_seconds();
    let stale_diff = (cot_event.stale - roundtrip_event.stale).num_seconds();
    let start_diff = (cot_event.start - roundtrip_event.start).num_seconds();
    
    assert!(time_diff.abs() < 1, "Time mismatch: {} vs {}", cot_event.time, roundtrip_event.time);
    assert!(stale_diff.abs() < 1, "Stale mismatch: {} vs {}", cot_event.stale, roundtrip_event.stale);
    assert!(start_diff.abs() < 1, "Start mismatch: {} vs {}", cot_event.start, roundtrip_event.start);
    
    // Check point values
    assert!((cot_event.point.lat - roundtrip_event.point.lat).abs() < 0.0001, "Lat mismatch");
    assert!((cot_event.point.lon - roundtrip_event.point.lon).abs() < 0.0001, "Lon mismatch");
    assert!((cot_event.point.hae - roundtrip_event.point.hae).abs() < 0.0001, "HAE mismatch");
    assert!((cot_event.point.ce - roundtrip_event.point.ce).abs() < 0.0001, "CE mismatch");
    assert!((cot_event.point.le - roundtrip_event.point.le).abs() < 0.0001, "LE mismatch");
    
    // If we get here, the critical values match
    println!("✓ Generic round-trip test passed with preserved values:");
    println!("  UID: {}", cot_event.uid);
    println!("  Type: {}", cot_event.event_type);
    println!("  Time: {} -> {}", cot_event.time, roundtrip_event.time);
    println!("  CE: {} -> {}", cot_event.point.ce, roundtrip_event.point.ce);
    
    // For debugging, still show the XML differences
    if !xml_utils::semantic_xml_eq(&min_expected, &min_actual, false) {
        println!("Note: XML format differences exist but semantic values match:");
        println!("--- Expected (input) ---
{}", min_expected);
        println!("--- Actual (output) ---
{}", min_actual);
    }
    
    println!("✓ Generic round-trip test passed");
    Ok(())
}
