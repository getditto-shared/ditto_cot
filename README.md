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
cotditto = { git = "https://github.com/yourusername/ditto_cot", package = "cotditto" }
```

### Java

See the [Java README](java/README.md) for detailed documentation.

```xml
<dependency>
  <groupId>com.ditto</groupId>
  <artifactId>ditto-cot</artifactId>
  <version>1.0.0</version>
</dependency>
```

### C#

See the [C# README](csharp/README.md) for detailed documentation.

```xml
<PackageReference Include="Ditto.Cot" Version="1.0.0" />
```

## ✨ Features

- Full CoT XML ↔ Ditto Document ↔ JSON/CRDT round-trip conversion
- Schema-validated document types for Chat, Location, and Emergency events
- Automatic type inference from CoT event types
- Proper handling of underscore-prefixed fields in JSON serialization/deserialization
- Asynchronous Ditto SDK integration
- Comprehensive test coverage across all implementations

## 🔄 Usage Examples

### Converting CoT XML to Ditto Documents

```rust
// Parse CoT XML into a CotEvent
let cot_xml = "<event version='2.0' uid='ANDROID-123' type='a-f-G-U-C'...";
let cot_event = CotEvent::from_xml(cot_xml)?;

// Convert to a Ditto Document
let peer_id = "my-peer-id";
let ditto_doc = cot_to_document(&cot_event, peer_id);

// The document type is automatically inferred from the CoT event type
match ditto_doc {
    DittoDocument::MapItem(map_item) => {
        println!("Received a location update");
    },
    DittoDocument::Chat(chat) => {
        println!("Received a chat message");
    },
    // Other document types...
}
```

### Converting Ditto Documents to CoT XML

```rust
// Convert a Ditto document to a CoT event
let cot_event = cot_event_from_ditto_document(&ditto_doc);

// Serialize to XML
let xml = cot_event.to_xml()?;
println!("CoT XML: {}", xml);
```

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

### Working with Document Types

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
if let DittoDocument::MapItem(map_item) = doc {
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
if let DittoDocument::Api(emergency) = doc {
    println!("Emergency from {}", emergency.e); // callsign
    // Process emergency data
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

#### C#
The C# library uses the .NET SDK build system.

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

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