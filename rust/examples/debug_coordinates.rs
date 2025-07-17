use ditto_cot::ditto::cot_event_from_flattened_json;
use serde_json::Value;

fn main() {
    // Create JSON step by step to avoid macro recursion limits
    let mut raw_json = serde_json::Map::new();
    raw_json.insert("@type".to_string(), Value::String("mapitem".to_string()));
    raw_json.insert(
        "_id".to_string(),
        Value::String("ANDROID-62d0aaefb2bfa772".to_string()),
    );
    raw_json.insert(
        "h".to_string(),
        Value::Number(serde_json::Number::from_f64(9999999.0).unwrap()),
    );
    raw_json.insert(
        "i".to_string(),
        Value::Number(serde_json::Number::from_f64(87.314).unwrap()),
    );
    raw_json.insert(
        "j".to_string(),
        Value::Number(serde_json::Number::from_f64(-22.812649).unwrap()),
    );
    raw_json.insert(
        "k".to_string(),
        Value::Number(serde_json::Number::from_f64(9999999.0).unwrap()),
    );
    raw_json.insert(
        "l".to_string(),
        Value::Number(serde_json::Number::from_f64(150.1319535).unwrap()),
    );
    raw_json.insert(
        "b".to_string(),
        Value::Number(serde_json::Number::from_f64(1752460382331.0).unwrap()),
    );
    raw_json.insert(
        "n".to_string(),
        Value::Number(serde_json::Number::from_f64(1752460382331000.0).unwrap()),
    );
    raw_json.insert(
        "o".to_string(),
        Value::Number(serde_json::Number::from_f64(1752460457331000.0).unwrap()),
    );
    raw_json.insert("p".to_string(), Value::String("h-e".to_string()));
    raw_json.insert("w".to_string(), Value::String("a-f-G-U-C".to_string()));
    raw_json.insert("e".to_string(), Value::String("GRAY KNIGHT".to_string()));
    raw_json.insert("g".to_string(), Value::String("2.0".to_string()));

    // Add all the r_* fields from the real example
    raw_json.insert(
        "r_contact_callsign".to_string(),
        Value::String("GRAY KNIGHT".to_string()),
    );
    raw_json.insert(
        "r___group_name".to_string(),
        Value::String("Cyan".to_string()),
    );
    raw_json.insert(
        "r___group_role".to_string(),
        Value::String("Team Member".to_string()),
    );
    raw_json.insert(
        "r_takv_device".to_string(),
        Value::String("SAMSUNG SM-G781U".to_string()),
    );
    raw_json.insert("r_takv_os".to_string(), Value::String("31".to_string()));
    raw_json.insert(
        "r_takv_platform".to_string(),
        Value::String("ATAK-CIV".to_string()),
    );
    raw_json.insert(
        "r_takv_version".to_string(),
        Value::String("5.4.0.16 (55e727de).1750199949-CIV".to_string()),
    );
    raw_json.insert(
        "r_contact_endpoint".to_string(),
        Value::String("192.168.1.101:4242:tcp".to_string()),
    );
    raw_json.insert(
        "r_uid_Droid".to_string(),
        Value::String("GRAY KNIGHT".to_string()),
    );
    raw_json.insert(
        "r_precisionlocation_altsrc".to_string(),
        Value::String("SRTM1".to_string()),
    );
    raw_json.insert(
        "r_precisionlocation_geopointsrc".to_string(),
        Value::String("USER".to_string()),
    );
    raw_json.insert(
        "r_status_battery".to_string(),
        Value::String("100".to_string()),
    );
    raw_json.insert(
        "r_track_course".to_string(),
        Value::String("231.28733462490806".to_string()),
    );
    raw_json.insert(
        "r_track_speed".to_string(),
        Value::String("0.0".to_string()),
    );
    raw_json.insert(
        "r_ditto_a".to_string(),
        Value::String("pkAocCgkMDgySqaZBARTDL9CD73kJPU6xmx9J2sv4tIS2zkCW-X9Q".to_string()),
    );
    raw_json.insert(
        "r_ditto_deviceName".to_string(),
        Value::String("T2bfa772".to_string()),
    );
    raw_json.insert(
        "r_ditto_ip".to_string(),
        Value::String("192.168.1.101".to_string()),
    );
    raw_json.insert(
        "r_ditto_version".to_string(),
        Value::String("AndJ4.10.2_90aa996a2e".to_string()),
    );

    let json_value = Value::Object(raw_json);

    println!("🔍 Input JSON coordinate fields:");
    println!("  h (ce): {}", json_value["h"]);
    println!("  i (hae): {}", json_value["i"]);
    println!("  j (lat): {}", json_value["j"]);
    println!("  k (le): {}", json_value["k"]);
    println!("  l (lon): {}", json_value["l"]);

    // Let's also debug the unflattening process
    use ditto_cot::ditto::r_field_flattening::unflatten_document_r_field;
    use std::collections::HashMap;

    if let serde_json::Value::Object(obj) = &json_value {
        let mut document_map: HashMap<String, serde_json::Value> =
            obj.clone().into_iter().collect();
        let r_map = unflatten_document_r_field(&mut document_map);

        println!("\n🔍 Unflattened r_map contains {} elements:", r_map.len());
        for (key, value) in &r_map {
            println!("  {}: {:?}", key, value);
        }
    }

    let cot_event = cot_event_from_flattened_json(&json_value);

    println!("\n🔍 Converted CotEvent coordinates:");
    println!("  lat: {}", cot_event.point.lat);
    println!("  lon: {}", cot_event.point.lon);
    println!("  hae: {}", cot_event.point.hae);
    println!("  ce: {}", cot_event.point.ce);
    println!("  le: {}", cot_event.point.le);

    println!("\n🔍 Expected coordinates (from original XML):");
    println!("  lat: -22.812649");
    println!("  lon: 150.1319535");
    println!("  hae: 87.314");
    println!("  ce: 9999999.0");
    println!("  le: 9999999.0");

    match cot_event.to_xml() {
        Ok(xml) => {
            println!("\n🔍 Generated XML:");
            println!("{}", xml);
        }
        Err(e) => {
            println!("\n❌ Failed to generate XML: {}", e);
        }
    }
}
