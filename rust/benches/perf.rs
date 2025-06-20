use criterion::{criterion_group, criterion_main, Criterion};
use ditto_cot::{
    cot_events::CotEvent, cot_to_document, xml_parser::parse_cot, xml_writer::to_cot_xml,
};

// Benchmark for XML parsing and serialization round-trip
fn bench_xml_round_trip(c: &mut Criterion) {
    let xml = r#"<event version="2.0" uid="ABC123" type="a-h-G" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T01:00:00Z" how="m-g" lat="34.0" lon="-117.0" hae="100.0" ce="5.0" le="5.0"><detail><contact callsign="RAVEN"/><__group name="Blue"/></detail></event>"#;

    c.bench_function("XML round-trip", |b| {
        b.iter(|| {
            let flat = parse_cot(xml).unwrap();
            let _ = to_cot_xml(&flat);
        });
    });
}

// Benchmark for Cot to Ditto document conversion and back
fn bench_cot_to_document_round_trip(c: &mut Criterion) {
    let xml = r#"<event version="2.0" uid="ABC123" type="a-h-G" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T01:00:00Z" how="m-g" lat="34.0" lon="-117.0" hae="100.0" ce="5.0" le="5.0"><detail><contact callsign="RAVEN"/><__group name="Blue"/></detail></event>"#;

    c.bench_function("Cot to Ditto document round-trip", |b| {
        b.iter(|| {
            let cot_event = CotEvent::from_xml(xml).unwrap();
            let ditto_doc = cot_to_document(&cot_event, "test-peer");
            let _back_to_cot = ditto_doc.to_cot_event();
        });
    });
}

// Benchmark for full pipeline: XML -> Cot -> Ditto -> Cot -> XML
fn bench_full_pipeline(c: &mut Criterion) {
    let xml = r#"<event version="2.0" uid="ABC123" type="a-h-G" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T01:00:00Z" how="m-g" lat="34.0" lon="-117.0" hae="100.0" ce="5.0" le="5.0"><detail><contact callsign="RAVEN"/><__group name="Blue"/></detail></event>"#;

    c.bench_function("Full pipeline: XML -> Cot -> Ditto -> Cot -> XML", |b| {
        b.iter(|| {
            // XML -> CotEvent
            let cot_event = CotEvent::from_xml(xml).unwrap();

            // CotEvent -> Ditto Document
            let ditto_doc = cot_to_document(&cot_event, "test-peer");

            // Ditto Document -> CotEvent -> XML
            let back_to_cot = ditto_doc.to_cot_event();
            let _xml_again = back_to_cot.to_xml();
        });
    });
}

// Benchmark for different message types
fn bench_different_message_types(c: &mut Criterion) {
    let messages = [
        // Location update
        r#"<event version="2.0" uid="LOC123" type="a-f-G-U-C" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T01:00:00Z" how="m-g" lat="34.0" lon="-117.0" hae="100.0" ce="5.0" le="5.0"><detail><contact callsign="TEAM1"/><__group name="Blue"/></detail></event>"#,
        // Chat message
        r#"<event version="2.0" uid="CHAT123" type="b-t-f" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T01:00:00Z" how="h-g-i-g-o"><detail><__chat chatroom="All Chat" id="all" messageId="msg123" senderCallsign="USER1"><chatgrp id="all" uid0="USER1"/></__chat><link uid="USER1" type="a-f-G-U-C" relation="p-p"/><remarks source="BAO.F.ATAK.USER1" to="" time="2023-01-01T00:00:00Z">Hello, world!</remarks></detail></event>"#,
        // Emergency
        r#"<event version="2.0" uid="EMER123" type="a-f-G-U-C-E" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T01:00:00Z" how="m-g" lat="34.0" lon="-117.0" hae="100.0" ce="5.0" le="5.0"><detail><emergency type="9-Line" cancel="false"><__emergency id="EMER123"/></emergency><contact callsign="TEAM1"/><__group name="Blue"/></detail></event>"#,
    ];

    for (i, xml) in messages.iter().enumerate() {
        let name = match i {
            0 => "location_update",
            1 => "chat_message",
            2 => "emergency",
            _ => unreachable!(),
        };

        c.bench_function(&format!("Full pipeline: {}", name), |b| {
            b.iter(|| {
                let cot_event = CotEvent::from_xml(xml).unwrap();
                let ditto_doc = cot_to_document(&cot_event, "test-peer");
                let back_to_cot = ditto_doc.to_cot_event();
                let _xml_again = back_to_cot.to_xml();
            });
        });
    }
}

criterion_group!(
    benches,
    bench_xml_round_trip,
    bench_cot_to_document_round_trip,
    bench_full_pipeline,
    bench_different_message_types
);
criterion_main!(benches);
