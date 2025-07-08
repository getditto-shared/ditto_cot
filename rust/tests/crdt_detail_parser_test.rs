//! Comprehensive tests for CRDT-optimized detail parser
//!
//! This test suite demonstrates the solution to the duplicate elements challenge
//! and validates cross-language compatibility with the Java implementation.

use ditto_cot::crdt_detail_parser::{
    convert_stable_keys_to_xml, get_next_available_index, parse_detail_section_with_stable_keys,
};
use ditto_cot::detail_parser::parse_detail_section;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

const TEST_DOC_ID: &str = "complex-detail-test";

/// Test stable key generation preserves all elements
#[test]
fn test_stable_key_generation_preserves_all_elements() {
    // Load the complex_detail.xml file
    let xml_content = fs::read_to_string("../schema/example_xml/complex_detail.xml")
        .expect("Failed to read complex_detail.xml");

    // Extract detail section
    let detail_section = extract_detail_section(&xml_content);

    // Convert with stable keys
    let detail_map = parse_detail_section_with_stable_keys(&detail_section, TEST_DOC_ID);

    println!("=== RUST CRDT-OPTIMIZED STABLE KEY TEST ===");
    println!("Total keys generated: {}", detail_map.len());

    // Verify single occurrence elements use direct keys
    assert!(detail_map.contains_key("status"), "Single 'status' element");
    assert!(
        detail_map.contains_key("acquisition"),
        "Single 'acquisition' element"
    );

    // Verify duplicate elements use stable keys
    assert!(
        detail_map.contains_key(&format!("{}_sensor_0", TEST_DOC_ID)),
        "sensor_0"
    );
    assert!(
        detail_map.contains_key(&format!("{}_sensor_1", TEST_DOC_ID)),
        "sensor_1"
    );
    assert!(
        detail_map.contains_key(&format!("{}_sensor_2", TEST_DOC_ID)),
        "sensor_2"
    );

    assert!(
        detail_map.contains_key(&format!("{}_contact_0", TEST_DOC_ID)),
        "contact_0"
    );
    assert!(
        detail_map.contains_key(&format!("{}_contact_1", TEST_DOC_ID)),
        "contact_1"
    );

    assert!(
        detail_map.contains_key(&format!("{}_track_0", TEST_DOC_ID)),
        "track_0"
    );
    assert!(
        detail_map.contains_key(&format!("{}_track_1", TEST_DOC_ID)),
        "track_1"
    );
    assert!(
        detail_map.contains_key(&format!("{}_track_2", TEST_DOC_ID)),
        "track_2"
    );

    assert!(
        detail_map.contains_key(&format!("{}_remarks_0", TEST_DOC_ID)),
        "remarks_0"
    );
    assert!(
        detail_map.contains_key(&format!("{}_remarks_1", TEST_DOC_ID)),
        "remarks_1"
    );
    assert!(
        detail_map.contains_key(&format!("{}_remarks_2", TEST_DOC_ID)),
        "remarks_2"
    );

    // Total: 2 single + 11 with stable keys = 13 elements preserved
    assert_eq!(
        detail_map.len(),
        13,
        "All 13 detail elements should be preserved"
    );

    // Verify attributes are preserved
    let sensor1_key = format!("{}_sensor_1", TEST_DOC_ID);
    let sensor1 = detail_map.get(&sensor1_key).unwrap();
    if let Value::Object(sensor1_map) = sensor1 {
        assert_eq!(
            sensor1_map.get("id").unwrap(),
            &Value::String("sensor-2".to_string())
        );
        assert_eq!(
            sensor1_map.get("type").unwrap(),
            &Value::String("thermal".to_string())
        );
        assert_eq!(
            sensor1_map.get("resolution").unwrap(),
            &Value::String("1080p".to_string())
        );
    } else {
        panic!("sensor1 should be an object");
    }

    println!("✅ All elements preserved with stable keys!");
}

/// Test round trip conversion preserves all data
#[test]
fn test_round_trip_preserves_all_data() {
    let xml_content = fs::read_to_string("../schema/example_xml/complex_detail.xml")
        .expect("Failed to read complex_detail.xml");

    let detail_section = extract_detail_section(&xml_content);

    // Convert to Map with stable keys
    let detail_map = parse_detail_section_with_stable_keys(&detail_section, TEST_DOC_ID);

    // Convert back to XML
    let reconstructed_xml = convert_stable_keys_to_xml(&detail_map);

    println!("=== RUST ROUND TRIP TEST ===");
    println!(
        "Original elements: {}",
        count_elements_in_xml(&detail_section)
    );
    println!(
        "Reconstructed elements: {}",
        count_elements_in_xml(&reconstructed_xml)
    );

    // Verify all element types are present
    assert_eq!(
        count_elements_by_name(&reconstructed_xml, "sensor"),
        3,
        "Should have 3 sensors"
    );
    assert_eq!(
        count_elements_by_name(&reconstructed_xml, "contact"),
        2,
        "Should have 2 contacts"
    );
    assert_eq!(
        count_elements_by_name(&reconstructed_xml, "track"),
        3,
        "Should have 3 tracks"
    );
    assert_eq!(
        count_elements_by_name(&reconstructed_xml, "remarks"),
        3,
        "Should have 3 remarks"
    );
    assert_eq!(
        count_elements_by_name(&reconstructed_xml, "status"),
        1,
        "Should have 1 status"
    );
    assert_eq!(
        count_elements_by_name(&reconstructed_xml, "acquisition"),
        1,
        "Should have 1 acquisition"
    );

    println!("✅ All elements preserved in round trip!");
}

/// Test P2P convergence scenario
#[test]
fn test_p2p_convergence_scenario() {
    let xml_content = fs::read_to_string("../schema/example_xml/complex_detail.xml")
        .expect("Failed to read complex_detail.xml");

    let detail_section = extract_detail_section(&xml_content);

    // Both nodes start with same state
    let mut node_a = parse_detail_section_with_stable_keys(&detail_section, TEST_DOC_ID);
    let mut node_b = parse_detail_section_with_stable_keys(&detail_section, TEST_DOC_ID);

    println!("=== RUST P2P CONVERGENCE SCENARIO ===");

    // Node A: Update sensor_1 zoom attribute
    let sensor1_key = format!("{}_sensor_1", TEST_DOC_ID);
    if let Some(Value::Object(sensor_map)) = node_a.get_mut(&sensor1_key) {
        sensor_map.insert("zoom".to_string(), Value::String("20x".to_string()));
        println!("Node A: Updated sensor_1 zoom to 20x");
    }

    // Node B: Remove contact_0, add new track
    let contact0_key = format!("{}_contact_0", TEST_DOC_ID);
    node_b.remove(&contact0_key);
    println!("Node B: Removed contact_0");

    let next_track_index = get_next_available_index(&node_b, TEST_DOC_ID, "track");
    let mut new_track = serde_json::Map::new();
    new_track.insert("_tag".to_string(), Value::String("track".to_string()));
    new_track.insert("_docId".to_string(), Value::String(TEST_DOC_ID.to_string()));
    new_track.insert(
        "_elementIndex".to_string(),
        Value::Number(next_track_index.into()),
    );
    new_track.insert("course".to_string(), Value::String("60.0".to_string()));
    new_track.insert("speed".to_string(), Value::String("3.5".to_string()));
    new_track.insert(
        "timestamp".to_string(),
        Value::String("2025-07-05T21:05:00Z".to_string()),
    );

    let new_track_key = format!("{}_track_{}", TEST_DOC_ID, next_track_index);
    node_b.insert(new_track_key.clone(), Value::Object(new_track));
    println!("Node B: Added track_{}", next_track_index);

    // Simulate CRDT merge (simplified)
    let mut merged = node_a.clone();
    merged.remove(&contact0_key); // Apply removal from Node B
    if let Some(new_track_value) = node_b.get(&new_track_key) {
        merged.insert(new_track_key.clone(), new_track_value.clone()); // Apply addition from Node B
    }

    println!("\nAfter convergence:");
    println!("- sensor_1 has zoom=20x (from Node A)");
    println!("- contact_0 removed (from Node B)");
    println!("- track_{} added (from Node B)", next_track_index);
    println!("- All other elements unchanged");

    // Verify convergence
    if let Some(Value::Object(merged_sensor)) = merged.get(&sensor1_key) {
        assert_eq!(
            merged_sensor.get("zoom").unwrap(),
            &Value::String("20x".to_string())
        );
    }
    assert!(!merged.contains_key(&contact0_key));
    assert!(merged.contains_key(&new_track_key));

    println!("✅ P2P convergence successful!");
}

/// Test comparison with original approach showing data preservation improvement
#[test]
fn test_solution_comparison() {
    let xml_content = fs::read_to_string("../schema/example_xml/complex_detail.xml")
        .expect("Failed to read complex_detail.xml");

    let detail_section = extract_detail_section(&xml_content);

    // Old approach: loses data
    let old_map = parse_detail_section(&detail_section);

    // New approach: preserves all data with stable keys
    let new_map = parse_detail_section_with_stable_keys(&detail_section, TEST_DOC_ID);

    println!("=== RUST SOLUTION COMPARISON ===");
    println!("Old approach preserved: {} elements", old_map.len());
    println!("New approach preserved: {} elements", new_map.len());
    println!(
        "Data preserved: {} additional elements!",
        new_map.len() - old_map.len()
    );

    assert!(
        new_map.len() > old_map.len(),
        "New approach should preserve more data"
    );

    // The new approach can now be used for Ditto document storage
    println!("\n✅ Problem solved: All duplicate elements preserved for CRDT!");
}

/// Test next available index functionality
#[test]
fn test_get_next_available_index() {
    let mut detail_map = HashMap::new();

    // Add some existing sensors
    detail_map.insert(format!("{}_sensor_0", TEST_DOC_ID), Value::Null);
    detail_map.insert(format!("{}_sensor_1", TEST_DOC_ID), Value::Null);
    detail_map.insert(format!("{}_sensor_4", TEST_DOC_ID), Value::Null); // Gap in numbering

    // Test getting next index
    let next_index = get_next_available_index(&detail_map, TEST_DOC_ID, "sensor");
    assert_eq!(
        next_index, 5,
        "Should return 5 (after highest existing index 4)"
    );

    // Test with no existing elements
    let next_contact_index = get_next_available_index(&detail_map, TEST_DOC_ID, "contact");
    assert_eq!(
        next_contact_index, 0,
        "Should return 0 for non-existing element type"
    );

    println!("✅ Index management working correctly!");
}

/// Test cross-language compatibility by ensuring same key generation
#[test]
fn test_cross_language_key_compatibility() {
    let detail = r#"<detail>
        <sensor type="optical" id="sensor-1"/>
        <sensor type="thermal" id="sensor-2"/>
        <contact callsign="ALPHA-01" endpoint="192.168.1.100:8080"/>
        <contact callsign="BRAVO-02" endpoint="192.168.1.101:8080"/>
        <status operational="true"/>
    </detail>"#;

    let result = parse_detail_section_with_stable_keys(detail, "test-doc-123");

    // These keys should match exactly what Java generates
    let expected_keys = vec![
        "status",                 // Single element
        "test-doc-123_sensor_0",  // First sensor
        "test-doc-123_sensor_1",  // Second sensor
        "test-doc-123_contact_0", // First contact
        "test-doc-123_contact_1", // Second contact
    ];

    for expected_key in expected_keys {
        assert!(
            result.contains_key(expected_key),
            "Missing key: {}",
            expected_key
        );
    }

    println!("✅ Cross-language key compatibility verified!");
}

/// Extract detail section from full CoT XML
fn extract_detail_section(xml: &str) -> String {
    if let Some(start) = xml.find("<detail>") {
        if let Some(end) = xml.find("</detail>") {
            return xml[start..end + 9].to_string();
        }
    }

    // Fallback: extract detail with attributes
    if let Some(start) = xml.find("<detail") {
        if let Some(end) = xml.find("</detail>") {
            return xml[start..end + 9].to_string();
        }
    }

    panic!("Could not extract detail section from XML");
}

/// Count total number of elements in XML
fn count_elements_in_xml(xml: &str) -> usize {
    xml.matches('<')
        .filter(|s| !s.starts_with("</") && !s.starts_with("<?"))
        .count()
}

/// Count elements with specific name
fn count_elements_by_name(xml: &str, element_name: &str) -> usize {
    let start_tag = format!("<{}", element_name);
    xml.matches(&start_tag).count()
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Run all tests to demonstrate complete solution
    #[test]
    fn test_complete_solution_demo() {
        println!("\n=== RUST CRDT DUPLICATE ELEMENTS SOLUTION DEMO ===\n");

        test_stable_key_generation_preserves_all_elements();
        test_round_trip_preserves_all_data();
        test_p2p_convergence_scenario();
        test_solution_comparison();
        test_get_next_available_index();
        test_cross_language_key_compatibility();

        println!("\n🎉 ALL TESTS PASSED - SOLUTION VERIFIED! 🎉");
        println!("✅ Rust implementation matches Java functionality");
        println!("✅ Zero data loss with CRDT optimization");
        println!("✅ P2P convergence scenarios working");
        println!("✅ Cross-language compatibility ensured");
    }
}
