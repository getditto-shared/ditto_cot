# cotditto

A high-performance Rust library for translating between [Cursor-on-Target (CoT)](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf) XML events and Ditto-compatible CRDT documents.

## ✨ Features

- Full CoT XML ↔ Ditto Document ↔ JSON/CRDT round-trip conversion
- Schema-validated document types for Chat, Location, and Emergency events
- Automatic type inference from CoT event types
- Asynchronous Ditto SDK integration
- Built on `serde` for flexible serialization/deserialization
- Comprehensive test coverage

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cotditto = { git = "https://github.com/yourusername/cotditto" }
```

## 🚀 Usage

### Basic Conversion

Convert between CoT XML and Ditto documents:

```rust
use cotditto::{
    cot_events::CotEvent,
    ditto::{cot_to_document, DittoDocument},
};

// Parse CoT XML to CotEvent
let cot_xml = r#"<event version="2.0" ...></event>"#;
let event = CotEvent::from_xml(cot_xml)?;

// Convert to Ditto document
let doc = cot_to_document(&event, "peer-123");

// Serialize to JSON
let json = serde_json::to_string_pretty(&doc)?;
println!("{}", json);
```

### Document Types

#### 1. Chat Documents

```rust
if let DittoDocument::Chat(chat) = doc {
    println!("Chat from {}: {}", chat.author_callsign, chat.message);
    println!("Room: {} (ID: {})", chat.room, chat.room_id);
    if let Some(loc) = chat.location {
        println!("Location: {}", loc);
    }
}
```

#### 2. Location Documents

```rust
if let DittoDocument::Location(loc) = doc {
    println!("Location update for {}", loc.common.author_callsign);
    println!("Position: {},{}", 
        loc.location.latitude, 
        loc.location.longitude
    );
    println!("Accuracy: ±{}m", loc.location.circular_error);
}
```

#### 3. Emergency Documents

```rust
if let DittoDocument::Emergency(emergency) = doc {
    println!("EMERGENCY: {} ({})", 
        emergency.emergency_type,
        emergency.status
    );
    if let Some(details) = emergency.details {
        println!("Details: {}", details);
    }
}
```

### Ditto Integration

```rust
use cotditto::ditto_sync::{DittoContext, DittoError};

async fn store_cot_event(ditto: &DittoContext, cot_xml: &str) -> Result<(), DittoError> {
    // Parse CoT XML
    let event = CotEvent::from_xml(cot_xml)?;
    
    // Convert to Ditto document
    let doc = cot_to_document(&event, &ditto.peer_key);
    
    // Store in Ditto
    ditto.store_document(doc).await?;
    
    Ok(())
}

async fn query_chat_messages(ditto: &DittoContext, room: &str) -> Result<Vec<ChatDocument>, DittoError> {
    ditto.query_documents::<ChatDocument>(json!({ "room": room })).await
}
```

### Round-trip Example

```rust
// Start with CoT XML
let cot_xml = r#"
    <event version="2.0" type="b-t-f"...>
        <detail>
            <chat room="All">
                <chatgrp uid="user1" id="All" senderCallsign="User1">
                    Hello, world!
                </chatgrp>
            </chat>
        </detail>
    </event>
"#;

// Parse to CotEvent
let event = CotEvent::from_xml(cot_xml)?;

// Convert to Ditto document
let doc = cot_to_document(&event, "peer-123");

// Convert back to CotEvent
let event_again = doc.to_cot_event()?;

// Serialize back to XML
let xml_again = event_again.to_xml()?;
```

## 📚 Document Schema

### Common Fields
All Ditto documents include these common fields:

- `_id`: Unique document identifier
- `_c`: Document counter (updates)
- `_v`: Schema version
- `_r`: Soft-delete flag
- `a`: Ditto peer key
- `b`: Timestamp (ms since epoch)
- `d`: Author UID
- `e`: Author callsign
- `h`: Circular error (CE) in meters

### Document Types

#### 1. Chat Document (`DittoDocument::Chat`)

```json
{
  "_t": "c",
  "message": "Hello, world!",
  "room": "All",
  "room_id": "group-1",
  "author_callsign": "User1",
  "author_uid": "user1",
  "author_type": "user",
  "time": "2023-01-01T12:00:00Z",
  "location": "34.0522,-118.2437,100"
}
```

#### 2. Location Document (`DittoDocument::Location`)

```json
{
  "_t": "l",
  "location_type": "a-f-G-U-C",
  "location": {
    "lat": 34.0522,
    "lon": -118.2437,
    "hae": 100.0,
    "ce": 10.0,
    "speed": 0.0,
    "course": 0.0
  }
}
```

#### 3. Emergency Document (`DittoDocument::Emergency`)

```json
{
  "_t": "e",
  "emergency_type": "911",
  "status": "active",
  "location": {
    "lat": 34.0522,
    "lon": -118.2437,
    "hae": 100.0,
    "ce": 10.0
  },
  "details": {
    "message": "Medical emergency"
  }
}
```
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