use anyhow::{Context, Result};
use chrono::Utc;
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{cot_to_document, CotDocument},
};

#[test]
fn test_sensor_unmanned_system_a_u_s_format() -> Result<()> {
    // Test the "a-u-S" (sensor/unmanned system) CoT format
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(30)).to_rfc3339();
    let event_uid = "sensor-test-001";

    let cot_xml = format!(
        r#"<?xml version="1.0" standalone="yes"?>
<event
how="m-d-a"
stale="{}"
start="{}"
time="{}"
type="a-u-S"
uid="{}"
version="2.0">
<detail>
</detail>
</event>
<track course="30.86376880675669" speed="1.3613854354920412" />
<point ce="500.0" hae="0.0" lat="37.32699544764403" le="100.0" lon="-75.2905272033264" />"#,
        stale_time, start_time, start_time, event_uid
    );

    // Parse the CoT XML into a CotEvent
    let cot_event = CotEvent::from_xml(&cot_xml)
        .with_context(|| format!("Failed to parse CoT XML: {}", cot_xml))?;

    // Verify basic CoT event properties
    assert_eq!(cot_event.event_type, "a-u-S");
    assert_eq!(cot_event.uid, event_uid);
    assert_eq!(cot_event.how, "m-d-a");
    assert_eq!(cot_event.version, "2.0");

    // Verify point data
    assert_eq!(cot_event.point.lat, 37.32699544764403);
    assert_eq!(cot_event.point.lon, -75.2905272033264);
    assert_eq!(cot_event.point.hae, 0.0);
    assert_eq!(cot_event.point.ce, 500.0);
    assert_eq!(cot_event.point.le, 100.0);

    // Convert CotEvent to Ditto document
    let ditto_doc = cot_to_document(&cot_event, "test_source");

    // Verify it resolves to a MapItem
    match &ditto_doc {
        CotDocument::MapItem(map_item) => {
            assert_eq!(map_item.id, event_uid);
            assert_eq!(map_item.w, "a-u-S"); // Event type
            assert_eq!(map_item.p, "m-d-a"); // How field

            // Verify point data (j=LAT, l=LON, i=HAE)
            assert_eq!(map_item.j, Some(37.32699544764403));
            assert_eq!(map_item.l, Some(-75.2905272033264));
            assert_eq!(map_item.i, Some(0.0));

            println!("✅ a-u-S CoT format correctly resolved to MapItem");
            println!("   - Type: {}", map_item.w);
            println!("   - How: {}", map_item.p);
            println!("   - Lat: {:?}", map_item.j);
            println!("   - Lon: {:?}", map_item.l);
            println!("   - HAE: {:?}", map_item.i);
        }
        _ => panic!("Expected MapItem document for a-u-S CoT format"),
    }

    Ok(())
}

#[test]
fn test_sensor_manual_data_acquisition_variants() -> Result<()> {
    // Test various sensor formats with manual data acquisition
    let test_cases = vec![
        ("a-u-S", "Unmanned System - Sensor"),
        ("a-u-A", "Unmanned System - Air"),
        ("a-u-G", "Unmanned System - Ground"),
        ("a-u-S-T", "Unmanned System - Sensor - Thermal"),
    ];

    for (event_type, description) in test_cases {
        println!("Testing {}: {}", event_type, description);

        let cot_xml = format!(
            r#"<event version="2.0" type="{}" uid="test-{}" time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" stale="2023-01-01T12:30:00Z" how="m-d-a">
  <point lat="35.0" lon="-120.0" hae="100.0" ce="50.0" le="25.0"/>
  <detail>
    <sensor type="thermal" status="active"/>
  </detail>
</event>"#,
            event_type,
            event_type.replace("-", "_")
        );

        let cot_event = CotEvent::from_xml(&cot_xml)?;
        let ditto_doc = cot_to_document(&cot_event, "test_source");

        // All should resolve to MapItem
        match &ditto_doc {
            CotDocument::MapItem(map_item) => {
                assert_eq!(map_item.w, event_type);
                assert_eq!(map_item.p, "m-d-a");
                println!("  ✅ {} correctly resolved to MapItem", event_type);
            }
            _ => panic!("Expected MapItem for {}", event_type),
        }
    }

    Ok(())
}
