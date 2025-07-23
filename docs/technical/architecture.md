# Ditto CoT Architecture

This document describes the architecture of the Ditto CoT library, a multi-language system for translating between Cursor-on-Target (CoT) XML events and Ditto-compatible CRDT documents.

> **Quick Navigation**: [CRDT Optimization](crdt-optimization.md) | [Performance Analysis](performance.md) | [API Reference](../reference/api-reference.md) | [Integration Guide](../integration/ditto-sdk.md)

## Table of Contents

- [System Overview](#system-overview)
- [Repository Structure](#repository-structure)
- [Core Components](#core-components)
- [Data Flow](#data-flow)
- [Language Implementations](#language-implementations)
- [Schema Management](#schema-management)
- [Integration Points](#integration-points)

## System Overview

The Ditto CoT library provides a unified approach to handling CoT events across multiple programming languages, with a focus on:

- **Data Preservation**: 100% preservation of all CoT XML elements
- **CRDT Optimization**: Efficient synchronization in P2P networks
- **Cross-Language Compatibility**: Consistent behavior across Java, Rust, and C#
- **Type Safety**: Schema-driven development with strong typing

### High-Level Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   CoT XML       │────▶│  Ditto CoT Lib   │────▶│ Ditto Document  │
│   Events        │◀────│  (Java/Rust/C#)  │◀────│   (CRDT)        │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                               │
                               ▼
                        ┌──────────────┐
                        │ JSON Schema  │
                        │ (Shared)     │
                        └──────────────┘
```

## Repository Structure

```
ditto_cot/
├── schema/                    # Shared schema definitions
│   ├── cot_event.xsd         # XML Schema for CoT events
│   └── ditto.schema.json     # JSON Schema for Ditto documents
├── rust/                     # Rust implementation
│   ├── src/
│   │   ├── cot_events/       # CoT event handling
│   │   ├── ditto/            # Ditto document types
│   │   └── lib.rs            # Library entry point
│   └── tests/                # Integration tests
├── java/                     # Java implementation
│   └── ditto_cot/
│       └── src/main/java/    # Java source code
└── csharp/                   # C# implementation (planned)
```

## Core Components

### 1. CoT Event Processing

**Purpose**: Parse and generate CoT XML events

**Key Types**:
- `CotEvent` - Main event structure
- `Point` - Geographic coordinates with accuracy
- `Detail` - Extensible detail section

**Features**:
- XML parsing with full element preservation
- Builder pattern for event creation
- Validation against CoT schema

### 2. CRDT Detail Parser

**Purpose**: Handle duplicate XML elements for CRDT compatibility

**Key Components**:
- Stable key generation algorithm
- Two-pass parsing for duplicate detection
- Metadata optimization for bandwidth efficiency

**Implementation**:
- Java: `CRDTOptimizedDetailConverter`
- Rust: `crdt_detail_parser`

### 3. Document Type System

**Purpose**: Type-safe representation of different CoT event types

**Document Types**:
- `MapItem` - Location updates and map graphics
- `Chat` - Chat messages
- `File` - File sharing events
- `Api` - API/emergency events
- `Generic` - Fallback for unknown types

### 4. SDK Integration Layer

**Purpose**: Bridge between CoT documents and Ditto SDK

**Features**:
- Observer document conversion
- R-field reconstruction
- DQL (Ditto Query Language) support

## Data Flow

### 1. CoT to Ditto Flow

```
CoT XML → Parse → CotEvent → Transform → CotDocument → Serialize → Ditto CRDT
```

1. **Parse**: XML parsed into structured `CotEvent`
2. **Transform**: Event type determines document type
3. **Serialize**: Document converted to CRDT-compatible format

### 2. Ditto to CoT Flow

```
Ditto CRDT → Deserialize → CotDocument → Transform → CotEvent → Generate → CoT XML
```

1. **Deserialize**: CRDT document to typed structure
2. **Transform**: Document converted back to event
3. **Generate**: Event serialized to XML

### 3. Observer Pattern Flow

```
Ditto Observer → Map<String,Object> → SDK Converter → Typed Document → Application
```

## Language Implementations

### Rust Implementation

**Build System**: Cargo with custom build.rs for code generation

**Key Features**:
- Zero-copy XML parsing
- Compile-time type safety
- Async Ditto integration
- Performance-optimized

**Dependencies**:
- `quick-xml` for XML processing
- `serde` for serialization
- `chrono` for time handling
- `dittolive_ditto` for SDK integration

### Java Implementation

**Build System**: Gradle with code generation

**Key Features**:
- Builder pattern APIs
- Jackson-based JSON handling
- Android compatibility
- Comprehensive test coverage

**Dependencies**:
- JAXB for XML processing
- Jackson for JSON
- Apache Commons for utilities

### C# Implementation

**Status**: Planned

**Build System**: .NET SDK

## Schema Management

### JSON Schema

**Location**: `schema/ditto.schema.json`

**Purpose**: 
- Define Ditto document structure
- Generate type-safe code
- Ensure cross-language compatibility

**Code Generation**:
- Rust: `typify` crate in build.rs
- Java: JSON Schema to POJO
- C#: NJsonSchema (planned)

### XML Schema

**Location**: `schema/cot_event.xsd`

**Purpose**:
- Validate CoT XML structure
- Document CoT event format
- Reference for implementations

## Integration Points

### 1. Ditto SDK Integration

**Rust**:
```rust
use dittolive_ditto::prelude::*;
let collection = ditto.store().collection("cot_events");
collection.upsert(doc).await?;
```

**Java**:
```java
DittoStore store = ditto.getStore();
store.execute("INSERT INTO cot_events DOCUMENTS (?)", dittoDoc);
```

### 2. P2P Network Integration

- Documents designed for CRDT merge semantics
- Stable keys enable differential sync
- Last-write-wins conflict resolution

### 3. Application Integration

- Type-safe document access
- Builder patterns for creation
- Observer pattern for updates
- Round-trip conversion support

## Design Principles

1. **Schema-First**: All data structures derived from schemas
2. **Cross-Language Parity**: Identical behavior across implementations
3. **Performance**: Optimize for P2P network efficiency
4. **Type Safety**: Compile-time guarantees where possible
5. **Extensibility**: Support for custom CoT extensions

## Future Considerations

1. **Schema Evolution**: Version migration strategies
2. **Custom Extensions**: Plugin system for domain-specific CoT types
3. **Performance**: Further CRDT optimizations
4. **Tooling**: CLI utilities for debugging and migration

## See Also

- **[CRDT Optimization](crdt-optimization.md)** - Deep dive into CRDT algorithms and optimization techniques
- **[Performance Analysis](performance.md)** - Benchmarks and performance characteristics
- **[Ditto SDK Integration](../integration/ditto-sdk.md)** - Real-world integration patterns
- **[Getting Started](../development/getting-started.md)** - Quick setup for development
- **[Schema Reference](../reference/schema.md)** - Complete document schema specification
- **[API Reference](../reference/api-reference.md)** - Complete API documentation for all languages