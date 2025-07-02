# Rust Implementation

[![Crates.io](https://img.shields.io/crates/v/ditto_cot)](https://crates.io/crates/ditto_cot)
[![Documentation](https://docs.rs/ditto_cot/badge.svg)](https://docs.rs/ditto_cot)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust library for translating between [Cursor-on-Target (CoT)](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf) XML events and Ditto-compatible CRDT documents.

## ✨ Key Features

- **Ergonomic Builder Patterns**: Create CoT events with fluent, chainable APIs
- **Type Safety**: Comprehensive error handling with structured error types  
- **High Performance**: Zero-copy XML parsing and efficient transformations
- **Flexible Point Construction**: Multiple ways to specify geographic coordinates and accuracy
- **Complete XML Support**: Parse, generate, and validate CoT XML messages
- **Seamless Ditto Integration**: Native support for Ditto's CRDT document model

> **Core Types:**
>
> - `CotEvent`: Struct representing a CoT event (parsed from XML)
> - `CotDocument`: Enum representing a Ditto-compatible document (used for CoT/Ditto transformations)
> - `DittoDocument`: Trait implemented by CotDocument for DQL/SDK support. Not a struct or enum.

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ditto_cot = { git = "https://github.com/getditto-shared/ditto_cot" }
```

## 🚀 Usage

### Creating CoT Events with Builder Pattern

The library provides ergonomic builder patterns for creating CoT events:

```rust
use ditto_cot::cot_events::CotEvent;
use chrono::Duration;

// Create a simple location update
let event = CotEvent::builder()
    .uid("USER-123")
    .event_type("a-f-G-U-C")
    .location(34.12345, -118.12345, 150.0)
    .callsign("ALPHA-1")
    .stale_in(Duration::minutes(10))
    .build();

// Create with team and accuracy information
let tactical_event = CotEvent::builder()
    .uid("BRAVO-2")
    .location_with_accuracy(35.0, -119.0, 200.0, 5.0, 10.0)
    .callsign_and_team("BRAVO-2", "Blue")
    .build();
```

### Point Construction with Fluent API

Create geographic points with builder pattern:

```rust
use ditto_cot::cot_events::Point;

// Simple coordinate specification
let point = Point::builder()
    .lat(34.0526)
    .lon(-118.2437)
    .hae(100.0)
    .build();

// Coordinates with accuracy in one call
let accurate_point = Point::builder()
    .coordinates(34.0526, -118.2437, 100.0)
    .accuracy(3.0, 5.0)
    .build();

// Alternative constructors
let point1 = Point::new(34.0, -118.0, 100.0);
let point2 = Point::with_accuracy(34.0, -118.0, 100.0, 5.0, 10.0);
```

### XML Parsing and Generation

```rust
use ditto_cot::cot_events::CotEvent;

// Parse CoT XML to CotEvent
let cot_xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C" 
             time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z" 
             stale="2023-01-01T12:05:00Z" how="h-g-i-g-o">
    <point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="20.0"/>
    <detail></detail>
</event>"#;

let event = CotEvent::from_xml(cot_xml)?;

// Generate XML from event
let xml_output = event.to_xml()?;
```

### Basic Conversion to Ditto Documents

Convert between CoT events and Ditto documents:

```rust
use ditto_cot::{
    cot_events::CotEvent,
    ditto::cot_to_document,
};

// Create event with builder
let event = CotEvent::builder()
    .uid("USER-456")
    .callsign("CHARLIE-3")
    .location(36.0, -120.0, 250.0)
    .build();

// Convert to CotDocument (main enum for Ditto/CoT transformations)
let doc = cot_to_document(&event, "peer-123");

// Serialize to JSON
let json = serde_json::to_string_pretty(&doc)?;
println!("{}", json);
```

### Quick Reference: Common Event Types

```rust
use ditto_cot::cot_events::CotEvent;
use chrono::Duration;

// Location Update (GPS tracker, unit position)
let location_event = CotEvent::builder()
    .uid("TRACKER-001")
    .event_type("a-f-G-U-C")  // Friendly ground unit
    .location(34.052235, -118.243683, 100.0)  // Los Angeles
    .callsign("ALPHA-1")
    .team("Blue")
    .stale_in(Duration::minutes(5))
    .build();

// Emergency Beacon
let emergency_event = CotEvent::builder()
    .uid("EMERGENCY-123")
    .event_type("b-a-o-can")  // Emergency beacon
    .location(34.073620, -118.240000, 50.0)
    .callsign("RESCUE-1")
    .detail("<detail><emergency type=\"Medical\" priority=\"High\"/></detail>")
    .stale_in(Duration::minutes(30))
    .build();

// Chat Message (using convenience method)
let chat_event = CotEvent::new_chat_message(
    "USER-456",
    "BRAVO-2", 
    "Message received, moving to coordinates",
    "All Chat Rooms",
    "All Chat Rooms"
);

// High-accuracy tactical position
let tactical_event = CotEvent::builder()
    .uid("SNIPER-007")
    .event_type("a-f-G-U-C-I")  // Infantry unit
    .location_with_accuracy(
        34.068921, -118.445181, 300.0,  // Position
        2.0, 5.0  // CE: 2m horizontal, LE: 5m vertical accuracy
    )
    .callsign_and_team("OVERWATCH", "Green")
    .how("h-g-i-g-o")  // Human-generated GPS
    .stale_in(Duration::minutes(15))
    .build();
```

### Using DittoDocument Trait for DQL Support

The `CotDocument` enum implements Ditto's `DittoDocument` trait, allowing you to use CoT documents with Ditto's DQL (Ditto Query Language) interface. **Note:** `DittoDocument` is a trait, not a struct or enum. You work with `CotDocument` and use trait methods as needed.

```rust
use dittolive_ditto::prelude::*;
use ditto_cot::ditto::{CotDocument, cot_to_document};
use ditto_cot::cot_events::CotEvent;

// Create a CotEvent and convert to CotDocument
let cot_event = CotEvent::new_location_update(/* parameters */);
let cot_document = cot_to_document(&cot_event, "my-peer-id");

// Use DittoDocument trait methods
let doc_id = DittoDocument::id(&cot_document);
println!("Document ID: {}", doc_id);

// Access specific fields using the get() method
let lat: f64 = DittoDocument::get(&cot_document, "h").unwrap(); // Field 'h' contains latitude
let lon: f64 = DittoDocument::get(&cot_document, "i").unwrap(); // Field 'i' contains longitude
println!("Location: {}, {}", lat, lon);

// Convert to CBOR for Ditto storage
let cbor_value = DittoDocument::to_cbor(&cot_document).unwrap();

// Insert into Ditto using DQL
let store = ditto.store();
let collection_name = match &cot_document {
    CotDocument::MapItem(_) => "map_items",
    CotDocument::Chat(_) => "chat_messages",
    CotDocument::File(_) => "files",
    CotDocument::Api(_) => "api_events",
};

// Convert document to JSON value for insertion
let doc_json = serde_json::to_value(&cot_document).unwrap();

// Insert using DQL with the full document object
let query = format!("INSERT INTO {} DOCUMENTS (:doc) ON ID CONFLICT DO MERGE", collection_name);
let params = serde_json::json!({ "doc": doc_json });
let query_result = store.execute_v2((&query, params)).await?;
```

> **Note:** For DQL mutations to work, your Ditto SDK must be configured correctly. If you encounter a `DqlUnsupported` error, you may need to disable sync with V3 or update your Ditto SDK version.

## 📚 Documentation

Full API documentation is available on [docs.rs](https://docs.rs/ditto_cot).

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please see the [main CONTRIBUTING guide](../../CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
