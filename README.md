# Ditto CoT

Multi-language libraries for translating between [Cursor-on-Target (CoT)](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf) XML events and Ditto-compatible CRDT documents.

## 📁 Repository Structure

```
ditto_cot/
├── schema/               # Shared schema definitions
│   ├── cot_event.xsd     # XML Schema for CoT events
│   └── ditto.schema.json # JSON Schema for Ditto documents
├── rust/                 # Rust implementation
├── java/                 # Java implementation
└── csharp/              # C# implementation
```

## 🛠 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (for Rust implementation)
- [Java JDK](https://adoptium.net/) 11+ (for Java implementation)
- [.NET SDK](https://dotnet.microsoft.com/download) 6.0+ (for C# implementation)

## 📚 Language-Specific Documentation

### Rust

See the [Rust README](rust/README.md) for detailed documentation.

```toml
[dependencies]
ditto_cot = { git = "https://github.com/getditto-shared/ditto_cot", package = "ditto_cot" }
```

### End-to-End (e2e) Testing for Rust

The e2e test is located in `rust/examples/e2e_test.rs` and verifies integration with Ditto. To run it:

- Use `cargo run --example e2e_test` from the `rust` directory.
- Ensure Ditto dependencies are set up (e.g., via `make rust` or `cargo build`).

This test checks full workflows, including schema and conversion logic.

### Java

See the [Java README](java/README.md) for detailed documentation.

```xml
<dependency>
  <groupId>com.ditto</groupId>
  <artifactId>ditto-cot</artifactId>
  <version>1.0.0</version>
</dependency>
```

### C #

See the [C# README](csharp/README.md) for detailed documentation.

```xml
<PackageReference Include="Ditto.Cot" Version="1.0.0" />
```

## ✨ Features

- **Ergonomic Builder Patterns** (Rust): Create CoT events with fluent, chainable APIs
- **Full Round-trip Conversion**: CoT XML ↔ Ditto Document ↔ JSON/CRDT conversions
- **Schema-validated Document Types**: Chat, Location, Emergency, File, and Generic events
- **Automatic Type Inference**: CoT event types automatically mapped to document types
- **Flexible Point Construction** (Rust): Multiple ways to specify coordinates and accuracy
- **Proper Field Handling**: Underscore-prefixed fields in JSON serialization/deserialization
- **Asynchronous Ditto Integration**: Native support for Ditto's CRDT document model
- **Comprehensive Test Coverage**: All implementations thoroughly tested

## 🔄 Usage Examples

### Creating CoT Events with Builder Pattern (Rust)

The Rust implementation provides ergonomic builder patterns for creating CoT events:

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

// Point construction with fluent API
let point = Point::builder()
    .coordinates(34.0526, -118.2437, 100.0)
    .accuracy(3.0, 5.0)
    .build();
```

### Converting CoT XML to CotDocument

This section shows how to convert a CoT XML string into a `CotDocument`, which is the main enum used for Ditto/CoT transformations in this library. `CotDocument` is not a DittoDocument; it implements the `DittoDocument` trait for DQL/SDK support, but is itself the type you use for all CoT/Ditto conversions.

```rust
// Parse CoT XML into a CotEvent
let cot_xml = "<event version='2.0' uid='ANDROID-123' type='a-f-G-U-C'...";
let cot_event = CotEvent::from_xml(cot_xml)?;

// Convert to a CotDocument (the main enum for Ditto/CoT transformations)
let peer_id = "my-peer-id";
let cot_doc = cot_to_document(&cot_event, peer_id); // Returns a CotDocument

// The document type is automatically inferred from the CoT event type
// CotDocument is the enum used for CoT-to-Ditto transformations.
// Note: DittoDocument is a trait that CotDocument implements for DQL support, not a struct or enum.
match cot_doc {
    CotDocument::MapItem(map_item) => {
        println!("Received a location update");
    },
    CotDocument::Chat(chat) => {
        println!("Received a chat message");
    },
    CotDocument::File(file) => {
        println!("Received a file: {}", file.file.unwrap_or_default());
    },
    // Other document types...
}
```

> **Note:**
>
> - `CotEvent`: Struct representing a CoT event (parsed from XML).
> - `CotDocument`: Enum representing a Ditto-compatible document (used for transformations).
> - `DittoDocument`: Trait implemented by CotDocument for DQL/SDK support. Not a struct or enum.

### Converting CotDocument to CoT XML

```rust
// Convert a CotDocument back to a CotEvent (for round-trip or XML serialization)
let roundtrip_event = cot_event_from_ditto_document(&cot_doc); // Returns a CotEvent

// Serialize to CoT XML
let xml = roundtrip_event.to_xml()?;
println!("CoT XML: {}", xml);
```

*The function `cot_event_from_ditto_document` takes a `CotDocument` (not a DittoDocument) and returns a `CotEvent`.*

### CotDocument vs DittoDocument

- **CotDocument**: The main enum representing Ditto-compatible documents for CoT transformations. Used throughout this library for all conversions.
- **DittoDocument**: A trait implemented by CotDocument (and possibly other types) to provide DQL/SDK support. Do not instantiate DittoDocument directly; use CotDocument and its trait methods where needed.

---

## 🧩 Interacting with the DittoDocument Trait and Ditto DQL

### What is the DittoDocument Trait?

The `DittoDocument` trait is part of the Ditto SDK and defines the interface for documents that can be queried, mutated, and synchronized using Ditto's DQL (Ditto Query Language). It is not a struct or enum, but a set of methods that types (like `CotDocument`) implement to be compatible with Ditto's real-time database and query engine.

### How Does CotDocument Implement DittoDocument?

`CotDocument` implements the `DittoDocument` trait, so you can use any `CotDocument` (produced from a CoT event or elsewhere) directly with Ditto's DQL APIs. This allows you to store, query, and synchronize CoT-derived documents in Ditto collections.

### Example: CoT Event → CotDocument → DQL

```rust
use ditto_cot::{cot_events::CotEvent, ditto::{CotDocument, cot_to_document}};
use dittolive_ditto::prelude::*;

// Parse CoT XML and convert to CotDocument
// "peer-key" is the Ditto peer key (a unique identifier for the device or user in Ditto)
enum CotDocument = cot_to_document(&CotEvent::from_xml(cot_xml)?, "peer-key");

// Insert into a Ditto collection (DQL)
let collection = ditto.store().collection("cot_events");
collection.upsert(&DittoDocument::id(&cot_doc), &cot_doc)?;

// Query using DQL
let results = collection.find("e == $callsign").query_with_parameters(params)?;
for doc in results.documents() {
    // doc is a DittoDocument, which CotDocument implements
    let callsign: String = DittoDocument::get(&doc, "e").unwrap();
    println!("Found callsign: {}", callsign);
}
```

> **Note:**
> The `peer-key` argument should be set to the unique Ditto peer key for your device or user. This key is used to identify the origin of the document in Ditto's sync system. You can obtain or configure it according to your Ditto SDK setup.

### Example: DQL Document → CotDocument → CoT XML

When you receive a document from Ditto's DQL (e.g. as a `DittoDocument`), you can deserialize it to a `CotDocument` and then convert it back to a `CotEvent` for CoT XML serialization:

```rust
use ditto_cot::{ditto::CotDocument, cot_events::CotEvent};

// Suppose you get a DittoDocument from a query
let doc: CotDocument = DittoDocument::from_json_str(doc_json)?;

// Convert back to a CotEvent
let cot_event = cot_event_from_ditto_document(&doc);
let xml = cot_event.to_xml()?;
println!("CoT XML: {}", xml);
```

### Summary of the Flow

- **CoT XML → CotEvent → CotDocument → DittoDocument trait → DQL** (store/query)
- **DQL (DittoDocument) → CotDocument → CotEvent → CoT XML** (round-trip)

This ensures seamless, type-safe, and loss-minimized round-trip conversions between CoT XML, Ditto's CRDT/DQL world, and back.

> **Functional Testing:**
> End-to-end tests for these flows can be found in [`rust/tests/e2e_test.rs`](rust/tests/e2e_test.rs). These tests verify round-trip conversions, DQL queries, and the integration between CotEvent, CotDocument, and DittoDocument through Ditto's SDK. Check the test file for real usage and validation examples.
>
### Handling Underscore-Prefixed Fields

The library properly handles underscore-prefixed fields in JSON serialization/deserialization:

```rust
// Fields with underscore prefixes in JSON are properly mapped to Rust fields
// For example, in JSON: "_id", "_c", "_v", "_r"
// In Rust: "id", "d_c", "d_v", "d_r"

let map_item = MapItem {
    id: "my-unique-id".to_string(),
    d_c: 1,                        // Maps to "_c" in JSON
    d_v: 2,                        // Maps to "_v" in JSON
    d_r: false,                    // Maps to "_r" in JSON
    // ... other fields
};

// When serialized to JSON, the fields will have their underscore prefixes
let json = serde_json::to_string(&map_item)?;
// json contains: {"_id":"my-unique-id","_c":1,"_v":2,"_r":false,...}

// When deserializing from JSON, the underscore-prefixed fields are correctly mapped back
let deserialized: MapItem = serde_json::from_str(&json)?;
assert_eq!(deserialized.id, "my-unique-id");
assert_eq!(deserialized.d_c, 1);
```

### Working with CotDocument Types

#### 1. Chat Documents

```rust
if let CotDocument::Chat(chat) = doc {
    println!("Chat from {}: {}", chat.author_callsign, chat.message);
    println!("Room: {} (ID: {})", chat.room, chat.room_id);
    if let Some(loc) = chat.location {
        println!("Location: {}", loc);
    }
}
```

#### 2. Location Documents

```rust
if let CotDocument::MapItem(map_item) = doc {
    println!("Location update for {}", map_item.e); // e is callsign
    if let (Some(lat), Some(lon)) = (map_item.h, map_item.i) {
        println!("Position: {},{}", lat, lon);
    }
    if let Some(ce) = map_item.k {
        println!("Accuracy: ±{}m", ce); // circular error
    }
}
```

#### 3. Emergency Documents

```rust
if let CotDocument::Api(emergency) = doc {
    println!("Emergency from {}", emergency.e); // callsign
    // Process emergency data
}
```

#### 4. File Documents

```rust
if let CotDocument::File(file) = doc {
    println!("File: {}", file.file.unwrap_or_default()); // filename
    println!("MIME Type: {}", file.mime.unwrap_or_default()); // MIME type
    println!("Size: {}", file.sz.unwrap_or_default()); // file size
    println!("ID: {}", file.id); // unique identifier
    
    // File documents are created from CoT events with type "b-f-t-file"
    // and extract metadata from the <fileshare> element in the detail section
}
```

#### 5. Generic Documents

```rust
if let CotDocument::Generic(generic) = doc {
    println!("Generic document with ID: {}", generic.id);
    println!("Type: {}", generic.t); // CoT event type
    
    // Access point coordinates
    if let (Some(lat), Some(lon)) = (generic.a, generic.b) {
        println!("Position: {},{}", lat, lon);
    }
    
    // Access detail fields from the detail_map
    if let Some(detail_map) = &generic.detail_map {
        if let Some(value) = detail_map.get("_ce") {
            println!("Circular Error: {}", value);
        }
        // Access any other fields from the detail section
    }
    
    // Generic documents are a fallback for CoT events that don't match other specific types
    // They preserve all fields from the original CoT event for maximum flexibility
}
```

## 🧪 Testing

The library includes comprehensive tests for all functionality:

```bash
# Run all tests
cargo test --all-targets

# Run specific test
cargo test test_underscore_key_handling
```

## 🛠️ Build System

### Makefile

The repository includes a top-level `Makefile` that provides a unified build system for all language implementations:

```bash
# Build all language libraries
make all

# Build specific language libraries
make rust
make java
make csharp

# Run tests
make test        # Test all libraries
make test-rust   # Test only Rust library
make test-java   # Test only Java library
make test-csharp # Test only C# library

# Clean builds
make clean        # Clean all libraries
make clean-rust   # Clean only Rust library
make clean-java   # Clean only Java library
make clean-csharp # Clean only C# library

# Show available commands
make help
```

### Language-Specific Build Systems

#### Rust

The Rust library uses a custom build script (`build.rs`) to generate Rust code from the JSON schema. This includes special handling for underscore-prefixed fields to ensure proper serialization/deserialization.

#### Java

The Java library uses Gradle as its build system. The Gradle wrapper (`gradlew`) is included in the repository, so you don't need to install Gradle separately.

#### C #

The C# library uses the .NET SDK build system.

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### Ditto Integration

```rust
use ditto_cot::ditto_sync::{DittoContext, DittoError};

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
    // Query using DQL to retrieve CotDocument instances of type ChatDocument
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

// Convert to CoT document (CotDocument)
let doc = cot_to_document(&event, "peer-123");

// Convert CotDocument back to CotEvent
let event_again = doc.to_cot_event()?;

// Serialize back to XML
let xml_again = event_again.to_xml()?;
```

## 📚 CotDocument Schema

### Common Fields

All CotDocument instances include these common fields (Note: DittoDocument is the Ditto-specific API document used with DQL):

- `_id`: Unique document identifier
- `_c`: Document counter (updates)
- `_v`: Schema version
- `_r`: Soft-delete flag
- `a`: Ditto peer key
- `b`: Timestamp (ms since epoch)
- `d`: Author UID
- `e`: Author callsign
- `h`: Circular error (CE) in meters

### CotDocument Types

#### 1. Chat Document (`CotDocument::Chat`)

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

#### 2. Location Document (`CotDocument::Location`)

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

#### 3. Emergency Document (`CotDocument::Emergency`)

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
use ditto_cot::schema_validator::validate_against_cot_schema;

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

MITRE CoT Reference: <https://apps.dtic.mil/sti/pdfs/ADA637348.pdf>  
Ditto SDK Rust Docs: <https://software.ditto.live/rust/Ditto>

---

MIT Licensed.
