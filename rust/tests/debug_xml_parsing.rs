use chrono::Utc;
use ditto_cot::cot_events::CotEvent;
use uuid::Uuid;

#[test]
fn test_xml_parsing_with_timestamps() {
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(30)).to_rfc3339();
    let event_uid = format!("MULTI-PEER-TEST-{}", Uuid::new_v4());

    println!("start_time: {}", start_time);
    println!("stale_time: {}", stale_time);
    println!("event_uid: {}", event_uid);

    let cot_xml = format!(
        r#"<event version="2.0" uid="{}" type="a-u-S" time="{}" start="{}" stale="{}" how="m-d-a"><point ce="500.0" hae="0.0" lat="37.32699544764403" le="100.0" lon="-75.2905272033264" /><detail><track course="30.86376880675669" speed="1.3613854354920412" /></detail></event>"#,
        event_uid, start_time, start_time, stale_time
    );

    println!("Generated XML: {}", cot_xml);
    println!("XML length: {}", cot_xml.len());
    
    for (i, c) in cot_xml.chars().enumerate() {
        if i < 30 {
            println!("Position {}: '{}' (ASCII: {})", i, c, c as u32);
        }
    }

    match CotEvent::from_xml(&cot_xml) {
        Ok(event) => {
            println!("Successfully parsed event: {}", event.uid);
        },
        Err(e) => {
            println!("Failed to parse XML: {}", e);
            panic!("XML parsing failed: {}", e);
        }
    }
}