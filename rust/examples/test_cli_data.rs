use ditto_cot::ditto::cot_event_from_flattened_json;

fn main() {
    // Use the exact JSON from the CLI output to test our fix
    let json_str = r#"{
        "@type": "mapitem",
        "_c": 1,
        "_id": "ANDROID-62d0aaefb2bfa772",
        "_r": false,
        "_v": 2,
        "a": "pkAocCgkMCvR_e8DXneZfAsm6MYWwtINhKPmkHdwAvEwW4IKYmnh0",
        "b": 1752554957714.0,
        "c": "GRAY KNIGHT",
        "d": "ANDROID-62d0aaefb2bfa772",
        "e": "GRAY KNIGHT",
        "f": true,
        "g": "2.0",
        "h": 31.3,
        "i": 86.965,
        "j": -22.812687,
        "k": 9999999.0,
        "l": 150.131864,
        "n": 1752554957714000.0,
        "o": 1752555032714000.0,
        "p": "m-g",
        "q": "",
        "r___group_name": "Cyan",
        "r___group_role": "Team Member",
        "r_contact_callsign": "GRAY KNIGHT",
        "r_contact_endpoint": "192.168.1.101:4242:tcp",
        "r_ditto_a": "pkAocCgkMCvR_e8DXneZfAsm6MYWwtINhKPmkHdwAvEwW4IKYmnh0",
        "r_ditto_deviceName": "T2bfa772",
        "r_ditto_ip": "192.168.1.101",
        "r_ditto_version": "AndJ4.10.2_90aa996a2e",
        "r_precisionlocation_altsrc": "GPS",
        "r_precisionlocation_geopointsrc": "GPS",
        "r_status_battery": "100",
        "r_takv_device": "SAMSUNG SM-G781U",
        "r_takv_os": "31",
        "r_takv_platform": "ATAK-CIV",
        "r_takv_version": "5.4.0.16 (55e727de).1750199949-CIV",
        "r_track_course": "235.10653730015372",
        "r_track_speed": "0.0",
        "r_uid_Droid": "GRAY KNIGHT",
        "s": "",
        "source": "cot-converter",
        "t": "",
        "u": "",
        "v": "",
        "w": "a-f-G-U-C"
    }"#;

    println!("=== Testing CLI Data with Our Fix ===");

    let json_value: serde_json::Value =
        serde_json::from_str(json_str).expect("Failed to parse CLI JSON");

    // This should use our fixed cot_event_from_flattened_json function
    let cot_event = cot_event_from_flattened_json(&json_value);

    println!("Reconstructed UID: {}", cot_event.uid);
    println!("Reconstructed Type: {}", cot_event.event_type);
    println!("Detail length: {} chars", cot_event.detail.len());
    println!("Detail content: {}", cot_event.detail);

    // Convert to XML
    match cot_event.to_xml() {
        Ok(xml) => {
            println!("\n=== Generated XML ===");
            println!("{}", xml);

            // Verify our fix worked
            if cot_event.detail.len() > 200
                && cot_event.detail.contains("contact")
                && cot_event.detail.contains("callsign=\"GRAY KNIGHT\"")
                && cot_event
                    .detail
                    .contains("endpoint=\"192.168.1.101:4242:tcp\"")
                && cot_event.detail.contains("takv")
                && cot_event.detail.contains("device=\"SAMSUNG SM-G781U\"")
                && cot_event.detail.contains("status")
                && cot_event.detail.contains("battery=\"100\"")
                && cot_event.detail.contains("__group")
                && cot_event.detail.contains("name=\"Cyan\"")
                && cot_event.detail.contains("role=\"Team Member\"")
            {
                println!("\n✅ SUCCESS: All detail elements properly reconstructed!");
                println!("   - Contact with callsign and endpoint ✓");
                println!("   - Group with name and role ✓");
                println!("   - TAKV with device info ✓");
                println!("   - Status with battery ✓");
                println!(
                    "   - Rich detail length: {} chars ✓",
                    cot_event.detail.len()
                );
            } else {
                println!("\n❌ FAILED: Detail reconstruction is incomplete");
                println!("   Detail length: {}", cot_event.detail.len());
                println!("   Missing elements detected");
            }
        }
        Err(e) => {
            println!("❌ Failed to convert to XML: {}", e);
        }
    }
}
