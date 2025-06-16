use cotditto::{xml_parser::parse_cot, xml_writer::to_cot_xml};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_round_trip(c: &mut Criterion) {
    let xml = r#"<event version="2.0" uid="ABC123" type="a-h-G" time="2023-01-01T00:00:00Z" start="2023-01-01T00:00:00Z" stale="2023-01-01T01:00:00Z" how="m-g" lat="34.0" lon="-117.0" hae="100.0" ce="5.0" le="5.0"><detail><contact callsign="RAVEN"/><__group name="Blue"/></detail></event>"#;

    c.bench_function("round-trip Cot XML", |b| {
        b.iter(|| {
            let flat = parse_cot(xml).unwrap();
            let _ = to_cot_xml(&flat);
        });
    });
}

criterion_group!(benches, bench_round_trip);
criterion_main!(benches);
