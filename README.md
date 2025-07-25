# Ditto CoT

High-performance, multi-language libraries for translating between [Cursor-on-Target (CoT)](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf) XML events and Ditto-compatible CRDT documents. Built with advanced **CRDT optimization** for efficient P2P network synchronization.

## 🚀 Quick Start

### Installation

**Rust**:
```toml
[dependencies]
ditto_cot = { git = "https://github.com/getditto-shared/ditto_cot" }
```

**Java**:
```xml
<dependency>
  <groupId>com.ditto</groupId>
  <artifactId>ditto-cot</artifactId>
  <version>1.0.0</version>
</dependency>
```

**Swift**:
```swift
.package(url: "https://github.com/getditto-shared/ditto_cot", from: "1.0.0")
```

**C#** (planned):
```xml
<PackageReference Include="Ditto.Cot" Version="1.0.0" />
```

### Basic Usage

**Rust**:
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

**Java**:
```java
CotEvent event = CotEvent.builder()
    .uid("USER-123")
    .type("a-f-G-U-C")
    .point(34.12345, -118.12345, 150.0)
    .callsign("ALPHA-1")
    .build();

DittoDocument doc = event.toDittoDocument();
```

**Swift**:
```swift
import DittoCoT

let event = ApiDocument(
    _id: "USER-123",
    _c: 1,
    _r: false,
    a: "peer-123",
    b: Date().timeIntervalSince1970 * 1000,
    d: "USER-123",
    e: "ALPHA-1",
    contentType: "application/json",
    data: "sample-data",
    // ... other required fields
)

let unionDoc = DittoCoTDocument.api(event)
```

## 📁 Repository Structure

```
ditto_cot/
├── docs/                 # 📚 Documentation
│   ├── technical/        # Architecture, CRDT, Performance  
│   ├── development/      # Getting Started, Building, Testing
│   ├── integration/      # SDK integration guides
│   └── reference/        # API reference, schemas
├── schema/               # Shared schema definitions
├── rust/                 # Rust implementation
├── java/                 # Java implementation
├── swift/                # Swift implementation  
└── csharp/              # C# implementation (planned)
```

## ✨ Key Features

- **🔄 100% Data Preservation**: All duplicate CoT XML elements maintained vs 46% in legacy systems
- **⚡ CRDT-Optimized**: 70% bandwidth savings through differential field sync  
- **🌐 Cross-Language**: Identical behavior across Rust, Java, Swift, and C#
- **🛡️ Type-Safe**: Schema-driven development with strong typing
- **📱 SDK Integration**: Observer document conversion with r-field reconstruction
- **🔧 Builder Patterns**: Ergonomic APIs for creating CoT events
- **🧪 Comprehensive Testing**: E2E tests including multi-peer P2P scenarios

## 📚 Documentation

For detailed information, see our comprehensive documentation:

### 🏗️ Technical Deep Dives
- **[Architecture](docs/technical/architecture.md)** - System design and components
- **[CRDT Optimization](docs/technical/crdt-optimization.md)** - Advanced P2P synchronization
- **[Performance](docs/technical/performance.md)** - Benchmarks and optimization

### 🛠️ Development Guides  
- **[Getting Started](docs/development/getting-started.md)** - Quick setup for all languages
- **[Building](docs/development/building.md)** - Build procedures and requirements
- **[Testing](docs/development/testing.md)** - Testing strategies and E2E scenarios

### 🔌 Integration Guides
- **[Ditto SDK Integration](docs/integration/ditto-sdk.md)** - Observer patterns and DQL
- **[Rust Examples](docs/integration/examples/rust.md)** - Rust-specific patterns
- **[Java Examples](docs/integration/examples/java.md)** - Java-specific patterns
- **[Swift Examples](docs/integration/examples/swift.md)** - Swift/SwiftUI patterns
- **[Migration Guide](docs/integration/migration.md)** - Version upgrades and legacy system migration

### 📖 Reference
- **[API Reference](docs/reference/api-reference.md)** - Complete API documentation
- **[Schema Reference](docs/reference/schema.md)** - Document schemas and validation
- **[Troubleshooting](docs/reference/troubleshooting.md)** - Common issues and solutions

### 🎯 Language-Specific READMEs
- **[Rust Implementation](rust/README.md)** - Rust-specific APIs and patterns
- **[Java Implementation](java/README.md)** - Java-specific APIs and patterns
- **[Swift Implementation](swift/README.md)** - Swift/SwiftUI APIs and patterns

## 🚀 Quick Start

```bash
# Build all libraries
make all

# Run all tests
make test

# See all available commands
make help
```

## 🤝 Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Next Steps**: Check out our [Getting Started Guide](docs/development/getting-started.md) for detailed setup instructions, or browse the [Architecture](docs/technical/architecture.md) to understand the system design.
