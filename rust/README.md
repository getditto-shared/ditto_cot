# Rust Implementation

[![Crates.io](https://img.shields.io/crates/v/cotditto)](https://crates.io/crates/cotditto)
[![Documentation](https://docs.rs/cotditto/badge.svg)](https://docs.rs/cotditto)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust library for translating between [Cursor-on-Target (CoT)](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf) XML events and Ditto-compatible CRDT documents.

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cotditto = { git = "https://github.com/yourusername/ditto_cot" }
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

### Using DittoDocument Trait for DQL Support

The `CotDocument` enum implements Ditto's `DittoDocument` trait, allowing you to use CoT documents with Ditto's DQL (Ditto Query Language) interface:

```rust
use dittolive_ditto::prelude::*;
use cotditto::ditto::{CotDocument, cot_to_document};
use cotditto::cot_events::CotEvent;

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

Full API documentation is available on [docs.rs](https://docs.rs/cotditto).

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please see the [main CONTRIBUTING guide](../../CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
