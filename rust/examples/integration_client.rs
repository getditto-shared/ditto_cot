use ditto_cot::{
    cot_events::CotEvent,
    ditto::{cot_to_document, from_ditto::cot_event_from_ditto_document},
};
use serde_json::json;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    // Create a sample CoT XML event
    let cot_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<event version="2.0" uid="ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279" type="b-m-p-s-p-loc" how="h-e" start="2023-10-15T10:30:00.000Z" time="2023-10-15T10:30:00.000Z" stale="2023-10-15T10:35:00.000Z">
    <point lat="35.091" lon="-106.558" hae="1618.8" ce="3.2" le="5.8"/>
    <detail>
        <contact callsign="PINKY" endpoint="192.168.1.10:4242:tcp"/>
        <__group name="Blue" role="Team Member"/>
        <color argb="-1"/>
        <usericon iconsetpath="COT_MAPPING_SPOTMAP/b-m-p-s-p-loc/spy.png"/>
        <link uid="ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279" type="b-m-p-s-p-loc" relation="p-p"/>
        <remarks>Equipment check complete</remarks>
        <status readiness="true"/>
        <track speed="12.5" course="45.0"/>
        <precisionlocation altsrc="GPS"/>
    </detail>
</event>"#;

    // Parse CoT XML to CotEvent
    let cot_event = CotEvent::from_xml(cot_xml)?;

    // Convert to Ditto Document
    let ditto_doc = cot_to_document(&cot_event, "integration-test-peer");

    // Convert back to CotEvent
    let roundtrip_cot_event = cot_event_from_ditto_document(&ditto_doc);

    // Convert back to XML
    let roundtrip_xml = roundtrip_cot_event.to_xml()?;

    // Output structured results for integration test
    let output = json!({
        "lang": "rust",
        "original_xml": cot_xml,
        "ditto_document": ditto_doc,
        "roundtrip_xml": roundtrip_xml,
        "success": true
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    io::stdout().flush()?;

    Ok(())
}