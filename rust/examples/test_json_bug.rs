use ditto_cot::ditto::r_field_flattening::unflatten_r_field;
use serde_json::{json, Value};
use std::collections::HashMap;

fn main() {
    // Test case that mimics the CLI data
    let mut flattened = HashMap::new();

    // Add the problematic fields from your CLI output
    flattened.insert("r___group_name".to_string(), json!("Cyan"));
    flattened.insert("r___group_role".to_string(), json!("Team Member"));
    flattened.insert("r_contact_callsign".to_string(), json!("GRAY KNIGHT"));
    flattened.insert(
        "r_contact_endpoint".to_string(),
        json!("192.168.1.101:4242:tcp"),
    );
    flattened.insert(
        "r_ditto_a".to_string(),
        json!("pkAocCgkMCvR_e8DXneZfAsm6MYWwtINhKPmkHdwAvEwW4IKYmnh0"),
    );
    flattened.insert("r_ditto_deviceName".to_string(), json!("T2bfa772"));
    flattened.insert("r_ditto_ip".to_string(), json!("192.168.1.101"));
    flattened.insert(
        "r_ditto_version".to_string(),
        json!("AndJ4.10.2_90aa996a2e"),
    );
    flattened.insert("r_precisionlocation_altsrc".to_string(), json!("GPS"));
    flattened.insert("r_precisionlocation_geopointsrc".to_string(), json!("GPS"));
    flattened.insert("r_status_battery".to_string(), json!("100"));
    flattened.insert("r_takv_device".to_string(), json!("SAMSUNG SM-G781U"));
    flattened.insert("r_takv_os".to_string(), json!("31"));
    flattened.insert("r_takv_platform".to_string(), json!("ATAK-CIV"));
    flattened.insert(
        "r_takv_version".to_string(),
        json!("5.4.0.16 (55e727de).1750199949-CIV"),
    );
    flattened.insert("r_track_course".to_string(), json!("244.80682660091918"));
    flattened.insert("r_track_speed".to_string(), json!("0.0"));
    flattened.insert("r_uid_Droid".to_string(), json!("GRAY KNIGHT"));

    println!("=== INPUT: Flattened r_* fields ===");
    for (key, value) in &flattened {
        println!("  {}: {}", key, value);
    }

    println!("\n=== OUTPUT: Reconstructed r field ===");
    let r_map = unflatten_r_field(&flattened);

    for (detail_type, detail_value) in &r_map {
        println!(
            "  {}: {}",
            detail_type,
            serde_json::to_string_pretty(&detail_value).unwrap()
        );
    }

    // Check specific reconstructed elements
    println!("\n=== VERIFICATION ===");

    if let Some(contact) = r_map.get("contact") {
        println!("✅ contact: {}", contact);
        if let Value::Object(contact_obj) = contact {
            if contact_obj.contains_key("callsign") && contact_obj.contains_key("endpoint") {
                println!("✅ contact has BOTH callsign and endpoint");
            } else {
                println!("❌ contact missing callsign or endpoint");
                println!(
                    "   callsign present: {}",
                    contact_obj.contains_key("callsign")
                );
                println!(
                    "   endpoint present: {}",
                    contact_obj.contains_key("endpoint")
                );
            }
        }
    } else {
        println!("❌ No contact object found");
    }

    if let Some(group) = r_map.get("__group") {
        println!("✅ __group: {}", group);
    } else {
        println!("❌ No __group object found");
    }

    if let Some(takv) = r_map.get("takv") {
        println!("✅ takv: {}", takv);
    } else {
        println!("❌ No takv object found");
    }
}
