# Rust Implementation

[![Crates.io](https://img.shields.io/crates/v/ditto_cot)](https://crates.io/crates/ditto_cot)
[![Documentation](https://docs.rs/ditto_cot/badge.svg)](https://docs.rs/ditto_cot)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance Rust implementation of the Ditto CoT library with zero-copy parsing and async Ditto SDK integration.

## 🚀 Quick Start

Add to your `Cargo.toml`:
```toml
[dependencies]
ditto_cot = { git = "https://github.com/getditto-shared/ditto_cot" }
```

Basic usage:
```rust
use ditto_cot::{cot_events::CotEvent, ditto::cot_to_document};

let event = CotEvent::builder()
    .uid("USER-123")
    .event_type("a-f-G-U-C")
    .location(34.12345, -118.12345, 150.0)
    .callsign("ALPHA-1")
    .build();

let doc = cot_to_document(&event, "peer-123");
```

## 🏗️ Rust-Specific Features

### Type System
- **`CotEvent`**: Struct for CoT events (XML parsing/generation)
- **`CotDocument`**: Enum for Ditto documents (CRDT operations)  
- **`DittoDocument`**: Trait for DQL integration (not a struct/enum)

### Performance Features
- **Zero-Copy Parsing**: Direct byte-level XML processing with `quick-xml`
- **Async Ditto Integration**: Native `async`/`await` support
- **Memory Efficiency**: Arena allocators and careful lifetime management
- **SIMD Optimizations**: Vectorized operations where applicable

### Builder Patterns

Ergonomic, chainable APIs for creating CoT events:

```rust
use ditto_cot::cot_events::CotEvent;
use chrono::Duration;

// Location with accuracy
let event = CotEvent::builder()
    .uid("SNIPER-007")
    .event_type("a-f-G-U-C-I")
    .location_with_accuracy(34.068921, -118.445181, 300.0, 2.0, 5.0)
    .callsign_and_team("OVERWATCH", "Green")
    .stale_in(Duration::minutes(15))
    .build();

// Chat message convenience method
let chat = CotEvent::new_chat_message(
    "USER-456", "BRAVO-2", "Message received", "All Chat Rooms", "group-id"
);
```

### Point Construction

Multiple ways to specify geographic coordinates:

```rust
use ditto_cot::cot_events::Point;

// Builder pattern
let point = Point::builder()
    .coordinates(34.0526, -118.2437, 100.0)
    .accuracy(3.0, 5.0)
    .build();

// Direct constructors
let point1 = Point::new(34.0, -118.0, 100.0);
let point2 = Point::with_accuracy(34.0, -118.0, 100.0, 5.0, 10.0);
```

## 🔌 Ditto SDK Integration

### DittoDocument Trait

`CotDocument` implements the `DittoDocument` trait for DQL support:

```rust
use dittolive_ditto::prelude::*;
use ditto_cot::ditto::{CotDocument, cot_to_document};

// Convert to CotDocument
let cot_document = cot_to_document(&event, "my-peer-id");

// Use trait methods
let doc_id = DittoDocument::id(&cot_document);
let lat: f64 = DittoDocument::get(&cot_document, "h").unwrap();

// DQL operations
let collection_name = match &cot_document {
    CotDocument::MapItem(_) => "map_items",
    CotDocument::Chat(_) => "chat_messages",
    CotDocument::File(_) => "files",
    CotDocument::Api(_) => "api_events",
};
```

### SDK Observer Conversion

Convert observer documents to typed schema objects:

```rust
use ditto_cot::ditto::sdk_conversion::{
    observer_json_to_cot_document, 
    observer_json_to_json_with_r_fields
};

// In observer callback
let boxed_doc: BoxedDocument = item.value();
let typed_doc = observer_json_to_cot_document(&boxed_doc)?;

match typed_doc {
    Some(CotDocument::MapItem(map_item)) => {
        println!("Location: {} at {},{}", 
                 map_item.e, map_item.j.unwrap_or(0.0), map_item.l.unwrap_or(0.0));
    },
    Some(CotDocument::Chat(chat)) => {
        println!("Chat from {}: {}", chat.author_callsign, chat.message);
    },
    _ => println!("Other document type"),
}
```

## 🧪 Testing

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# E2E tests (requires Ditto credentials)
export DITTO_APP_ID="your-app-id"
export DITTO_PLAYGROUND_TOKEN="your-token"
cargo test e2e_

# Benchmarks
cargo bench
```

## 📚 Documentation

- **API Docs**: [docs.rs/ditto_cot](https://docs.rs/ditto_cot)
- **Examples**: `examples/` directory
- **Integration Guide**: [Rust Examples](../docs/integration/examples/rust.md)

For comprehensive documentation, see the [main documentation](../docs/).
