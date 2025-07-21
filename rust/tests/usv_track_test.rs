use ditto_cot::cot_events::CotEvent;
use ditto_cot::ditto::to_ditto::{cot_to_document, CotDocument};
use ditto_cot::ditto::schema::ApiRValue;
use std::fs;

#[test]
fn test_usv_track_processing() {
    // Read the USV track XML file
    let xml_content = fs::read_to_string("../schema/example_xml/usv_track.xml")
        .expect("Failed to read usv_track.xml");
    
    println!("USV Track XML content:\n{}", xml_content);
    
    // Parse the XML into a CotEvent
    let cot_event = CotEvent::from_xml(&xml_content)
        .expect("Failed to parse USV track XML");
    
    println!("Parsed CotEvent:");
    println!("  UID: {}", cot_event.uid);
    println!("  Type: {}", cot_event.event_type);
    println!("  Detail: {}", cot_event.detail);
    
    // Convert to Ditto document
    let ditto_doc = cot_to_document(&cot_event, "test-peer-key");
    
    match ditto_doc {
        CotDocument::MapItem(map_item) => {
            println!("Ditto MapItem document:");
            println!("  ID: {}", map_item.id);
            println!("  Event type (w): {}", map_item.w);
            println!("  Callsign (e): {}", map_item.e);
            println!("  Author (a): {}", map_item.a);
            println!("  Document author (d): {}", map_item.d);
            println!("  Location: lat={:?}, lon={:?}", map_item.j, map_item.l);
            println!("  Detail fields (r): {:?}", map_item.r);
            
            // Verify the callsign is extracted correctly
            assert_eq!(map_item.e, "USV-4", "Callsign should be extracted to 'e' field");
            
            // Verify the UID is still used for 'a' and 'd' fields (not overridden by callsign)
            assert_eq!(map_item.a, "test-peer-key", "'a' field should be peer key");
            assert_eq!(map_item.d, "00000000-0000-0000-0000-333333333333", "'d' field should be UID");
            
            // Verify basic fields
            assert_eq!(map_item.id, "00000000-0000-0000-0000-333333333333");
            assert_eq!(map_item.w, "a-f-S-C-U");
            
            // Check if callsign appears in detail fields - print all r field entries for debugging
            println!("  All r field entries: {:?}", map_item.r);
            
            println!("✓ USV track XML processed correctly by Rust library");
        }
        _ => panic!("Expected MapItem document for USV track"),
    }
}

#[test]
fn test_usv_track_round_trip() {
    // Read the USV track XML file
    let xml_content = fs::read_to_string("../schema/example_xml/usv_track.xml")
        .expect("Failed to read usv_track.xml");
    
    // Parse XML -> CotEvent -> Ditto Document -> CotEvent
    let original_event = CotEvent::from_xml(&xml_content)
        .expect("Failed to parse USV track XML");
    
    let ditto_doc = cot_to_document(&original_event, "test-peer");
    
    let recovered_event = ditto_cot::ditto::from_ditto::cot_event_from_ditto_document(&ditto_doc);
    
    // Verify round-trip preservation
    assert_eq!(original_event.uid, recovered_event.uid);
    assert_eq!(original_event.event_type, recovered_event.event_type);
    assert_eq!(original_event.version, recovered_event.version);
    
    // Check that callsign information is preserved in detail during round trip
    println!("Original detail: {}", original_event.detail);
    println!("Recovered detail: {}", recovered_event.detail);
    
    println!("✓ USV track round-trip conversion completed");
}