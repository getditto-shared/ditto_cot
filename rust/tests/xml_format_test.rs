//! Test to verify XML formatting for external elements

use chrono::Utc;
use ditto_cot::cot_events::CotEvent;

#[test]
fn test_xml_format_with_external_elements() -> Result<(), Box<dyn std::error::Error>> {
    // Generate RFC3339 timestamps
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(30)).to_rfc3339();
    let event_uid = format!("TEST-{}", uuid::Uuid::new_v4());

    // Create XML in the same format as the e2e test
    let cot_xml = format!(
        r#"<?xml version="1.0" standalone="yes"?>
<event how="m-d-a" stale="{}" start="{}" time="{}" type="a-u-S" uid="{}" version="2.0">
<detail>
</detail>
</event>
<track course="30.86376880675669" speed="1.3613854354920412" />
<point ce="500.0" hae="0.0" lat="37.32699544764403" le="100.0" lon="-75.2905272033264" />"#,
        stale_time, start_time, start_time, event_uid
    );

    println!("Generated XML:\n{}", cot_xml);

    // This should not fail with XML parsing error
    let cot_event = CotEvent::from_xml(&cot_xml)?;

    // Verify that external point data was parsed correctly
    assert_eq!(cot_event.event_type, "a-u-S");
    assert_eq!(cot_event.how, "m-d-a");
    assert_eq!(cot_event.point.lat, 37.32699544764403);
    assert_eq!(cot_event.point.lon, -75.2905272033264);
    assert_eq!(cot_event.point.hae, 0.0);
    assert_eq!(cot_event.point.ce, 500.0);
    assert_eq!(cot_event.point.le, 100.0);

    println!("✅ XML parsing successful!");
    println!("   Event type: {}", cot_event.event_type);
    println!("   Latitude: {}", cot_event.point.lat);
    println!("   Longitude: {}", cot_event.point.lon);

    Ok(())
}
