use chrono::Utc;
use ditto_cot::cot_events::CotEvent;
use ditto_cot::ditto::{cot_event_from_flattened_json, cot_to_flattened_document};

fn main() {
    // Create a test CoT event
    let event = CotEvent {
        version: "2.0".to_string(),
        uid: "test-flattening-123".to_string(),
        event_type: "a-f-G-U-C".to_string(),
        time: Utc::now(),
        start: Utc::now(),
        stale: Utc::now() + chrono::Duration::minutes(30),
        how: "h-g-i-g-o".to_string(),
        point: ditto_cot::cot_events::Point {
            lat: 12.345,
            lon: 23.456,
            hae: 100.0,
            ce: 10.0,
            le: 5.0,
        },
        detail: r#"<detail>
            <takv os="35" version="5.4.0.11" device="GOOGLE PIXEL 7"/>
            <contact callsign="TEST-1" endpoint="192.168.1.100:8080"/>
        </detail>"#
            .to_string(),
    };

    println!("Original CoT Event:");
    println!("UID: {}", event.uid);
    println!("Type: {}", event.event_type);
    println!("Detail: {}", event.detail);
    println!();

    // Convert to flattened document
    let flattened_doc = cot_to_flattened_document(&event, "test-peer");
    println!("Flattened Document:");
    println!("{}", serde_json::to_string_pretty(&flattened_doc).unwrap());
    println!();

    // Convert back to CoT event
    let reconstructed_event = cot_event_from_flattened_json(&flattened_doc);
    println!("Reconstructed CoT Event:");
    println!("UID: {}", reconstructed_event.uid);
    println!("Type: {}", reconstructed_event.event_type);
    println!("Detail: {}", reconstructed_event.detail);
    println!();

    // Verify round-trip worked
    println!("Round-trip verification:");
    println!("UID match: {}", event.uid == reconstructed_event.uid);
    println!(
        "Type match: {}",
        event.event_type == reconstructed_event.event_type
    );
    println!(
        "Detail preserved: {}",
        !reconstructed_event.detail.trim().is_empty()
    );
}
