# Getting Started with Ditto CoT

This guide helps you quickly set up and start using the Ditto CoT library in your preferred programming language.

> **Quick Navigation**: [Building Guide](building.md) | [Testing Guide](testing.md) | [Integration Examples](../integration/examples/) | [API Reference](../reference/api-reference.md)

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [First Steps](#first-steps)
- [Basic Usage Examples](#basic-usage-examples)
- [Next Steps](#next-steps)

## Prerequisites

Choose the language(s) you want to work with:

### For Rust Development
- **Rust 1.70+**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Git**: For cloning the repository

### For Java Development  
- **Java JDK 17+**: [Download JDK](https://adoptium.net/)
- **Gradle 7.0+**: Included via wrapper in repository
- **Git**: For cloning the repository

### For C# Development (Planned)
- **.NET SDK 6.0+**: [Download .NET](https://dotnet.microsoft.com/download)
- **Git**: For cloning the repository

### Optional but Recommended
- **Make**: For unified build commands across languages
- **Ditto Account**: For testing P2P synchronization features

## Installation

### Option 1: Direct Dependency (Recommended)

**Rust** - Add to your `Cargo.toml`:
```toml
[dependencies]
ditto_cot = { git = "https://github.com/getditto-shared/ditto_cot" }
```

**Java** - Add to your `build.gradle`:
```groovy
dependencies {
    implementation 'com.ditto:ditto-cot:1.0.0'
}
```

**Maven** - Add to your `pom.xml`:
```xml
<dependency>
  <groupId>com.ditto</groupId>
  <artifactId>ditto-cot</artifactId>
  <version>1.0.0</version>
</dependency>
```

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/getditto-shared/ditto_cot.git
cd ditto_cot

# Build all languages (requires prerequisites installed)
make all

# Or build specific language
make rust    # Build Rust library
make java    # Build Java library
```

## First Steps

### 1. Verify Installation

**Rust**:
```bash
cd rust
cargo test --lib
```

**Java**:
```bash
cd java
./gradlew test
```

### 2. Run a Simple Example

**Rust**:
```rust
use ditto_cot::{cot_events::CotEvent, ditto::cot_to_document};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple location event
    let event = CotEvent::builder()
        .uid("USER-123")
        .event_type("a-f-G-U-C")
        .location(34.0522, -118.2437, 100.0)
        .callsign("Test User")
        .build();
    
    // Convert to Ditto document
    let doc = cot_to_document(&event, "peer-123");
    
    println!("Created document: {}", serde_json::to_string_pretty(&doc)?);
    Ok(())
}
```

**Java**:
```java
import com.ditto.cot.CotEvent;

public class FirstExample {
    public static void main(String[] args) {
        // Create a simple location event
        CotEvent event = CotEvent.builder()
            .uid("USER-123")
            .type("a-f-G-U-C")
            .point(34.0522, -118.2437, 100.0)
            .callsign("Test User")
            .build();
        
        // Convert to XML
        String xml = event.toXml();
        System.out.println("Created XML: " + xml);
    }
}
```

## Basic Usage Examples

### Creating Different Types of CoT Events

#### Location Update
```rust
// Rust
let location_event = CotEvent::builder()
    .uid("TRACKER-001")
    .event_type("a-f-G-U-C")  // Friendly ground unit
    .location_with_accuracy(34.052235, -118.243683, 100.0, 5.0, 10.0)
    .callsign("ALPHA-1")
    .team("Blue")
    .build();
```

```java
// Java
CotEvent locationEvent = CotEvent.builder()
    .uid("TRACKER-001")
    .type("a-f-G-U-C")
    .point(34.052235, -118.243683, 100.0, 5.0, 10.0)
    .detail()
        .callsign("ALPHA-1")
        .groupName("Blue")
        .build()
    .build();
```

#### Chat Message
```rust
// Rust
let chat_event = CotEvent::new_chat_message(
    "USER-456",
    "BRAVO-2", 
    "Message received, moving to coordinates",
    "All Chat Rooms",
    "All Chat Rooms"
);
```

```java
// Java
CotEvent chatEvent = CotEvent.builder()
    .uid("USER-456")
    .type("b-t-f")
    .detail()
        .chat("All Chat Rooms", "Message received, moving to coordinates")
        .callsign("BRAVO-2")
        .build()
    .build();
```

### Working with XML

#### Parse CoT XML
```rust
// Rust
let cot_xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C"...>"#;
let event = CotEvent::from_xml(cot_xml)?;
```

```java
// Java
String cotXml = "<event version=\"2.0\" uid=\"TEST-123\" type=\"a-f-G-U-C\"...>";
CotEvent event = CotEvent.fromXml(cotXml);
```

#### Generate XML
```rust
// Rust
let xml = event.to_xml()?;
```

```java
// Java
String xml = event.toXml();
```

### Converting to Ditto Documents

```rust
// Rust
use ditto_cot::ditto::cot_to_document;

let doc = cot_to_document(&event, "my-peer-id");

match doc {
    CotDocument::MapItem(map_item) => {
        println!("Location: {} at {},{}", 
                 map_item.e, map_item.j.unwrap_or(0.0), map_item.l.unwrap_or(0.0));
    },
    CotDocument::Chat(chat) => {
        println!("Chat: {}", chat.message);
    },
    _ => println!("Other document type"),
}
```

```java
// Java
import com.ditto.cot.schema.*;

Object doc = converter.convertToDocument(event);

if (doc instanceof MapItemDocument) {
    MapItemDocument mapItem = (MapItemDocument) doc;
    System.out.println("Location: " + mapItem.getE() + 
                      " at " + mapItem.getJ() + "," + mapItem.getL());
} else if (doc instanceof ChatDocument) {
    ChatDocument chat = (ChatDocument) doc;
    System.out.println("Chat: " + chat.getMessage());
}
```

## Common Workflows

### 1. XML Processing Workflow
```
CoT XML → Parse → CotEvent → Validate → Process
```

### 2. Document Creation Workflow  
```
Builder → CotEvent → Convert → Ditto Document → Store
```

### 3. P2P Synchronization Workflow
```
Local Change → CRDT Update → Differential Sync → Remote Apply
```

## Next Steps

Now that you have the basics working, explore these areas:

### 🏗️ Architecture Understanding
- Read the [Architecture Guide](../technical/architecture.md) to understand system design
- Learn about [CRDT Optimization](../technical/crdt-optimization.md) for P2P benefits

### 🛠️ Development
- Follow the [Building Guide](building.md) for development setup
- Set up [Testing](testing.md) for your contribution workflow

### 🔌 Integration
- Explore [Ditto SDK Integration](../integration/ditto-sdk.md) for real-time sync
- Check language-specific examples:
  - [Rust Examples](../integration/examples/rust.md)
  - [Java Examples](../integration/examples/java.md)

### 📚 Advanced Topics
- [Performance Optimization](../technical/performance.md)
- [API Reference](../reference/api-reference.md)
- [Schema Documentation](../reference/schema.md)

## Getting Help

- **Issues**: Report bugs on [GitHub Issues](https://github.com/getditto-shared/ditto_cot/issues)
- **Discussions**: Ask questions in [GitHub Discussions](https://github.com/getditto-shared/ditto_cot/discussions)
- **Documentation**: Check our comprehensive [docs](../README.md)

## Common First Steps Issues

**Rust Build Errors**: Ensure you have Rust 1.70+ and all dependencies installed
**Java Compilation Issues**: Verify JDK 17+ and Gradle wrapper permissions
**Missing Dependencies**: Run `make all` to ensure all components are built
**Test Failures**: Some tests require Ditto credentials - see [Testing Guide](testing.md)

## See Also

- **[Building Guide](building.md)** - Detailed build procedures for all languages
- **[Testing Guide](testing.md)** - Comprehensive testing strategies and troubleshooting
- **[Integration Examples](../integration/examples/)** - Language-specific usage patterns
- **[API Reference](../reference/api-reference.md)** - Complete API documentation
- **[Troubleshooting](../reference/troubleshooting.md)** - Common issues and solutions
- **[Architecture](../technical/architecture.md)** - Understanding the system design