# cotditto

A high-performance Rust library for translating between [Cursor-on-Target (CoT)](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf) XML events and flat, Ditto-compatible CRDT documents.

## ✨ Features

- Full CoT XML → Flat Rust struct → JSON/CRDT → CoT XML round-trip
- Handles both known fields and dynamic `<detail>` content via hybrid parsing
- XML Schema validation against CoT XSD schema
- Asynchronous Ditto SDK integration for inserting and retrieving events
- Performance benchmark with Criterion
- Test suite with edge case validation
- Future extensibility for plugins or schema-aware detail parsers

## 📦 Installation

```
cargo build
```

## 🚀 Usage

### Parse and serialize:

```rust
use cotditto::{xml_parser::parse_cot, xml_writer::to_cot_xml};

let flat = parse_cot(cot_xml).unwrap();
let xml_out = to_cot_xml(&flat);
```

### Ditto storage:

```rust
use cotditto::ditto_sync::{insert_flat_cot_event, get_flat_cot_events};

insert_flat_cot_event(&ditto, &flat).await?;
let events = get_flat_cot_events(&ditto).await?;
```

## 🔍 XML Validation

The library provides basic XML well-formedness checking for CoT messages. Note that full XSD schema validation is not currently implemented.

```rust
use cotditto::schema_validator::validate_against_cot_schema;

let cot_xml = r#"
    <event version="2.0" 
          uid="TEST-123" 
          type="a-f-G-U-C" 
          time="2021-02-27T20:32:24.913Z" 
          start="2021-02-27T20:32:24.913Z" 
          stale="2021-02-27T20:38:39.913Z" 
          how="h-g-i-g-o">
        <point lat="1.234567" lon="3.456789" hae="9999999.0" ce="9999999.0" le="9999999.0"/>
        <detail>
            <contact callsign="TEST-USER"/>
            <__group name="Cyan" role="Team Member"/>
        </detail>
    </event>"#;

match validate_against_cot_schema(cot_xml) {
    Ok(_) => println!("Well-formed CoT XML"),
    Err(e) => eprintln!("XML error: {}", e),
}
```

### Note on XSD Validation

While the library includes the CoT XSD schema file (`src/schema/cot_event.xsd`), full XSD validation is not currently implemented due to limitations in available Rust XML schema validation libraries. For production use, you might want to:

1. Use an external tool like `xmllint` for schema validation
2. Implement a custom validation layer for your specific CoT message requirements
3. Use a different language with better XML schema support for validation

The current implementation provides basic XML well-formedness checking which catches many common errors in XML structure.

## 🧪 Tests

Run all tests including schema validation:

```
cargo test
```

Run only unit tests (without schema validation):

```
cargo test --lib
```

Run only integration tests:

```
cargo test --test integration
```

## 📈 Benchmarks

```
cargo bench
```

## 📚 Schema Reference

The CoT XML schema is based on the official Cursor on Target XSD schema. The schema file is located at `src/schema/cot_event.xsd`.

### Validation Rules

- All required CoT event attributes must be present
- Attribute values must conform to their defined types
- The XML structure must match the schema definition
- Custom elements in the `<detail>` section must be properly namespaced

## 🔬 Fuzz Testing

Scaffolded under `fuzz/` using `cargo-fuzz`.

To run:

```
cargo install cargo-fuzz
cargo fuzz run fuzz_parse_cot
```

## 🧰 Future Plans

- Expand `FlatCotEvent` with more typed `<detail>` variants (e.g., `takv`, `track`)
- Schema-aware XSD validation or compile-time CoT models
- Internal plugin registry for custom extensions

MITRE CoT Reference: https://apps.dtic.mil/sti/pdfs/ADA637348.pdf  
Ditto SDK Rust Docs: https://software.ditto.live/rust/Ditto

---

MIT Licensed.