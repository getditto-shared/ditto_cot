# Ditto CoT Swift Library

Swift implementation of the Ditto Cursor-on-Target (CoT) library for translating between CoT XML events and Ditto-compatible CRDT documents.

## Phase 1: Foundation & Schema Integration ✅

This phase provides the foundational Swift infrastructure:

### Features Implemented

- **Swift Package Manager Integration**: Complete SPM setup with proper module structure
- **Schema Code Generation**: Automatic generation of Swift types from JSON schemas
- **JSON-Compatible Types**: Custom `JSONValue` type for handling dynamic CoT detail fields
- **Document Types**: Generated Swift structs for all CoT document types:
  - `ApiDocument`
  - `ChatDocument` 
  - `FileDocument`
  - `MapitemDocument`
  - `GenericDocument`
- **Union Type**: `DittoCoTDocument` enum for polymorphic document handling
- **Build Integration**: Makefile integration matching existing Rust/Java patterns
- **Testing Infrastructure**: Basic unit tests and integration test framework

### Project Structure

```
swift/
├── Package.swift                    # Swift Package Manager manifest
├── Sources/
│   ├── DittoCoTCore/               # Core types and schemas (no Ditto SDK dependency)
│   │   ├── Generated/              # Auto-generated from JSON schemas
│   │   │   ├── DittoDocument.swift
│   │   │   ├── DocumentTypes.swift
│   │   │   ├── DittoCoTDocument.swift
│   │   │   └── JSONValue.swift
│   │   └── DittoCoTCore.swift
│   ├── DittoCoT/                   # Ditto SDK integration (Phase 3)
│   │   └── DittoCoT.swift
│   └── CodeGen/                    # Schema code generation tool
│       └── main.swift
├── Tests/
│   ├── DittoCoTTests/              # Unit tests
│   └── IntegrationTests/           # Integration tests
└── README.md
```

### Code Generation

The library uses a custom Swift code generator that reads the shared JSON schemas and generates Swift types automatically:

```bash
# Generate Swift types from schemas
cd swift
swift build --product ditto-cot-codegen
.build/debug/ditto-cot-codegen --schema-path ../schema --output-path Sources/DittoCoTCore/Generated
```

### Building & Testing

The Swift library is integrated into the existing multi-language build system:

```bash
# Build Swift library (includes code generation)
make swift

# Test Swift library
make test-swift

# Clean Swift build artifacts
make clean-swift

# Build all languages including Swift
make all

# Test all languages including Swift
make test
```

### Generated Types

All document types follow the same pattern and are generated from the shared schemas:

```swift
public struct ApiDocument: DittoDocument {
    public let type = "api"
    
    // Ditto system fields
    public var _id: String
    public var _c: Int
    public var _v: Int { 2 }
    public var _r: Bool
    
    // Common CoT fields
    public var a: String              // Ditto peer key
    public var b: Double              // Millis since epoch
    public var d: String              // TAK UID of author
    public var e: String              // Callsign of author
    public var r: JSONValue = JSONValue([:])  // Detail (dynamic map)
    // ... other fields with defaults
    
    // API-specific fields
    public var contentType: String
    public var data: String
    // ... other API fields
}
```

The `JSONValue` type handles the dynamic `r` (detail) field that can contain any JSON-compatible data:

```swift
public enum JSONValue: Codable, Equatable {
    case null
    case bool(Bool)
    case number(Double)
    case string(String)
    case array([JSONValue])
    case object([String: JSONValue])
}
```

### Next Phases

- **Phase 2**: Core CoT Event Handling (XML parsing/serialization, validation)
- **Phase 3**: Ditto SDK Integration (document conversion, observers)
- **Phase 4**: SwiftUI Integration Layer (ObservableObject wrappers, view models)
- **Phase 5**: Testing Infrastructure (cross-language validation, performance tests)
- **Phase 6**: Documentation & Examples (API docs, sample apps)
- **Phase 7**: Advanced Features & Optimization (CRDT optimizations, performance)

### Dependencies

- **DittoSwift**: Ditto SDK for iOS/macOS (4.11.0+)
- **XMLCoder**: XML parsing and serialization (0.17.1+)
- **ArgumentParser**: Command-line tool for code generation (1.3.0+)

### Requirements

- Swift 5.9+
- iOS 15.0+ / macOS 12.0+ / watchOS 8.0+ / tvOS 15.0+
- Xcode 15.0+

This foundation provides a solid base for the remaining phases of Swift/SwiftUI integration while maintaining consistency with the existing Rust and Java implementations.