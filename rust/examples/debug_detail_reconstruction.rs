//! Debug the detail reconstruction issue
//!
//! This reproduces the exact issue the user reported: input XML has 7+ detail elements
//! but output only shows 2. Let's debug why.

use ditto_cot::ditto::cot_event_from_flattened_json;

fn main() {
    println!("🔍 Debugging Detail Reconstruction Issue");
    println!("========================================");

    // Load the definitive ATAK test XML to create accurate flattened structure
    let base_xml = std::fs::read_to_string("../schema/example_xml/atak_test.xml")
        .expect("Failed to read definitive atak_test.xml");

    println!("📋 Using definitive ATAK test XML from schema/example_xml/atak_test.xml");

    // Parse the XML to get the exact detail structure
    use ditto_cot::cot_events::CotEvent;
    use ditto_cot::ditto::cot_to_flattened_document;

    let cot_event = CotEvent::from_xml(&base_xml).expect("Failed to parse definitive ATAK XML");

    let flattened_json = cot_to_flattened_document(&cot_event, "debug-peer");

    // Count the r_* fields
    let r_field_count = flattened_json
        .as_object()
        .unwrap()
        .keys()
        .filter(|k| k.starts_with("r_"))
        .count();

    println!("📊 Input: {} flattened r_* fields found", r_field_count);

    // List all r_* fields
    println!("📋 r_* fields:");
    for (key, value) in flattened_json.as_object().unwrap() {
        if key.starts_with("r_") {
            println!("   {}: {:?}", key, value);
        }
    }

    // Convert using the library
    let cot_event = cot_event_from_flattened_json(&flattened_json);

    // Check the detail_extra contents after unflattening
    use ditto_cot::ditto::from_ditto_util::flat_cot_event_from_flattened_json;
    let flat_cot_event = flat_cot_event_from_flattened_json(&flattened_json);

    println!("\n🔍 After unflattening:");
    println!(
        "📊 detail_extra contains {} elements",
        flat_cot_event.detail_extra.len()
    );
    for (key, value) in &flat_cot_event.detail_extra {
        println!("   {}: {:?}", key, value);
    }

    // Generate XML and count detail elements
    match cot_event.to_xml() {
        Ok(xml) => {
            println!("\n🔍 Generated XML:");
            println!("{}", xml);

            // Count detail elements in the output
            let detail_count = count_detail_elements(&xml);
            println!("\n📊 Output: {} detail elements found in XML", detail_count);

            // List the detail elements found
            println!("📋 Detail elements in XML:");
            list_detail_elements(&xml);

            if detail_count < r_field_count {
                println!("\n❌ ISSUE IDENTIFIED:");
                println!("   Input had {} r_* fields", r_field_count);
                println!("   Output has only {} detail elements", detail_count);
                println!(
                    "   {} detail elements are missing!",
                    r_field_count - detail_count
                );
            } else {
                println!("\n✅ All detail elements preserved correctly");
            }
        }
        Err(e) => {
            println!("\n❌ Failed to generate XML: {}", e);
        }
    }
}

fn count_detail_elements(xml: &str) -> usize {
    let detail_start = xml.find("<detail>");
    let detail_end = xml.find("</detail>");

    if let (Some(start), Some(end)) = (detail_start, detail_end) {
        let detail_content = &xml[start + 8..end];

        // Count opening tags (not self-closing)
        let mut count = 0;
        let mut pos = 0;
        while let Some(tag_start) = detail_content[pos..].find('<') {
            let actual_pos = pos + tag_start;
            if actual_pos + 1 < detail_content.len()
                && !detail_content.chars().nth(actual_pos + 1).unwrap().eq(&'/')
            {
                count += 1;
            }
            pos = actual_pos + 1;
        }
        count
    } else {
        0
    }
}

fn list_detail_elements(xml: &str) -> Vec<String> {
    let mut elements = Vec::new();
    let detail_start = xml.find("<detail>");
    let detail_end = xml.find("</detail>");

    if let (Some(start), Some(end)) = (detail_start, detail_end) {
        let detail_content = &xml[start + 8..end];

        // Find all opening tags
        let mut pos = 0;
        while let Some(tag_start) = detail_content[pos..].find('<') {
            let actual_pos = pos + tag_start;
            if actual_pos + 1 < detail_content.len() {
                let next_char = detail_content.chars().nth(actual_pos + 1).unwrap();
                if next_char != '/' {
                    // Find the end of the tag name
                    if let Some(tag_end) = detail_content[actual_pos + 1..]
                        .find(|c: char| c.is_whitespace() || c == '>' || c == '/')
                    {
                        let tag_name = &detail_content[actual_pos + 1..actual_pos + 1 + tag_end];
                        elements.push(tag_name.to_string());
                        println!("   <{}>", tag_name);
                    }
                }
            }
            pos = actual_pos + 1;
        }
    }

    elements
}
