use ditto_cot::ditto::from_ditto_util::flat_cot_event_from_ditto;
use ditto_cot::ditto::{CotDocument, MapItem};

fn main() {
    // Create a MapItem with the timestamp values you're seeing
    let map_item = MapItem {
        id: "ANDROID-6dd0f2492e2d3d91".to_string(),
        a: "test_peer_key".to_string(),
        b: 1752368289242.0, // CE value from your example
        c: None,
        d: "ANDROID-6dd0f2492e2d3d91".to_string(),
        d_c: 0,
        d_r: false,
        d_v: 2,
        source: None,
        e: "PERK".to_string(),
        f: Some(true),
        g: "2.0".to_string(),
        h: Some(254.6),
        i: Some(99.765),
        j: Some(-22.812764),
        k: Some(9999999.0),
        l: Some(150.13174),
        n: Some(1752368289242000.0), // Start timestamp in microseconds
        o: Some(1752368364242000.0), // Stale timestamp in microseconds (75 seconds later)
        p: "m-g".to_string(),
        q: "".to_string(),
        r: std::collections::HashMap::new(),
        s: "".to_string(),
        t: "".to_string(),
        u: "".to_string(),
        v: "".to_string(),
        w: "a-f-G-U-C".to_string(),
    };

    let doc = CotDocument::MapItem(map_item);
    let flat_event = flat_cot_event_from_ditto(&doc);

    println!("MapItem timestamp conversion test:");
    println!("Original n (microseconds): 1752368289242000");
    println!("Original o (microseconds): 1752368364242000");
    println!("Converted time: {}", flat_event.time);
    println!("Converted start: {}", flat_event.start);
    println!("Converted stale: {}", flat_event.stale);

    // Parse the timestamps to verify they're correct
    if let Ok(time) = chrono::DateTime::parse_from_rfc3339(&flat_event.time) {
        println!("Parsed time: {}", time.format("%Y-%m-%d %H:%M:%S%.3f UTC"));
    }

    if let Ok(stale) = chrono::DateTime::parse_from_rfc3339(&flat_event.stale) {
        println!(
            "Parsed stale: {}",
            stale.format("%Y-%m-%d %H:%M:%S%.3f UTC")
        );
    }

    // Expected: Should show July 13, 2025 timestamps, not 1970
}
