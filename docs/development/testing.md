# Testing Guide

This guide covers the comprehensive testing strategy for the Ditto CoT library, including unit tests, integration tests, end-to-end tests, and cross-language validation.

> **Quick Navigation**: [Getting Started](getting-started.md) | [Building Guide](building.md) | [Troubleshooting](../reference/troubleshooting.md) | [Performance](../technical/performance.md)

## Table of Contents

- [Testing Overview](#testing-overview)
- [Test Categories](#test-categories)
- [Running Tests](#running-tests)
- [Test Infrastructure](#test-infrastructure)
- [End-to-End Testing](#end-to-end-testing)
- [Cross-Language Testing](#cross-language-testing)
- [Performance Testing](#performance-testing)
- [Test Data and Fixtures](#test-data-and-fixtures)

## Testing Overview

The Ditto CoT library employs a multi-layered testing strategy:

1. **Unit Tests**: Test individual components and functions
2. **Integration Tests**: Test component interactions
3. **End-to-End Tests**: Test complete workflows with real Ditto instances
4. **Cross-Language Tests**: Validate consistency between implementations
5. **Performance Tests**: Benchmark and regression testing
6. **Fuzz Tests**: Stress testing with random inputs

## Test Categories

### Unit Tests
- **Rust**: `cargo test --lib`
- **Java**: `./gradlew test`
- **Coverage**: Core functionality, edge cases, error handling

### Integration Tests  
- **Rust**: `cargo test --test integration`
- **Java**: Integration test classes in `src/test/java`
- **Coverage**: Component interactions, schema validation

### End-to-End Tests
- **Rust**: `cargo test e2e_`
- **Java**: E2E test classes
- **Coverage**: Complete workflows with Ditto SDK

### Cross-Language Tests
- **Command**: `make test-integration`
- **Coverage**: Output compatibility between languages

## Running Tests

### Quick Test Commands

```bash
# Run all tests across all languages
make test

# Language-specific tests
make test-rust    # Rust tests only
make test-java    # Java tests only

# Integration tests
make test-integration

# From repository root
cargo test        # Rust tests from any directory
./gradlew test    # Java tests (from java/ directory)
```

### Rust Testing

#### Basic Test Commands
```bash
cd rust

# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration

# Specific test
cargo test test_underscore_key_handling

# With output
cargo test -- --nocapture

# Parallel test control
cargo test -- --test-threads=1
```

#### Rust Test Categories

**Unit Tests** (`src/lib.rs` and module tests):
```bash
cargo test cot_events      # CoT event handling
cargo test detail_parser   # XML detail parsing
cargo test crdt_detail     # CRDT optimization
cargo test sdk_conversion  # SDK utilities
```

**Integration Tests** (`tests/` directory):
```bash
cargo test integration     # Component integration
cargo test e2e_xml        # XML round-trip
cargo test e2e_multi_peer # Multi-peer scenarios
```

**Example Tests**:
```bash
cargo run --example e2e_test              # Basic E2E
cargo run --example integration_client    # Cross-language client
```

### Java Testing

#### Basic Test Commands
```bash
cd java

# All tests
./gradlew test

# Specific test class
./gradlew test --tests "com.ditto.cot.CotEventTest"

# Test pattern
./gradlew test --tests "*CRDT*"

# With detailed output
./gradlew test --info

# Continuous testing
./gradlew test --continuous
```

#### Java Test Categories

**Unit Tests**:
```bash
./gradlew test --tests "*Test"           # Standard unit tests
./gradlew test --tests "*CRDTTest"       # CRDT functionality
./gradlew test --tests "*ConverterTest"  # Conversion logic
```

**Integration Tests**:
```bash
./gradlew test --tests "*IntegrationTest"
./gradlew test --tests "SharedFixturesIntegrationTest"
```

**Coverage Reports**:
```bash
./gradlew jacocoTestReport
# Report: build/reports/jacoco/test/html/index.html
```

## Test Infrastructure

### Shared Test Fixtures

Both languages use consistent test data via shared fixtures:

**Location**: 
- Rust: `tests/fixtures/`
- Java: `src/test/java/com/ditto/cot/fixtures/`

**Standard Test Data**:
```rust
// Common coordinates
const SF_LAT: f64 = 37.7749;
const SF_LON: f64 = -122.4194;
const NYC_LAT: f64 = 40.7128;
const NYC_LON: f64 = -74.0060;

// Standard timestamps
const TEST_TIME: &str = "2024-01-15T10:30:00.000Z";
```

**Usage**:
```rust
// Rust
use fixtures::*;
let xml = CoTTestFixtures::create_map_item_xml(test_uids::MAP_ITEM_1);
```

```java
// Java
import com.ditto.cot.CoTTestFixtures;
String xml = CoTTestFixtures.createMapItemXml(CoTTestFixtures.TestUIDs.MAP_ITEM_1);
```

### Test Utilities

**Rust**:
```rust
// Round-trip testing
TestUtils::assert_round_trip_conversion(&xml, uid, cot_type)?;

// Document validation
TestUtils::assert_map_item_document(&document, uid, lat, lon)?;

// Performance testing
TestUtils::time_operation(|| conversion_function())?;
```

**Java**:
```java
// Round-trip testing
TestUtils.assertRoundTripConversion(xml, uid, cotType);

// Document validation  
TestUtils.assertMapItemDocument(document, uid, lat, lon);

// Concurrent testing
TestUtils.testConcurrentAccess(converter, testData);
```

## End-to-End Testing

### Prerequisites

E2E tests require Ditto credentials:

```bash
export DITTO_APP_ID="your-app-id"
export DITTO_PLAYGROUND_TOKEN="your-token"
```

### Single-Peer E2E Tests

**Purpose**: Verify complete workflows with real Ditto integration

**Rust**:
```bash
# Basic E2E round-trip
cargo test e2e_xml_roundtrip

# Multiple XML examples
cargo test e2e_xml_examples_roundtrip

# With specific XML file
E2E_XML_FILE="complex_detail.xml" cargo test e2e_xml_examples_roundtrip
```

**Test Flow**:
1. Connect to Ditto with authentication
2. Parse CoT XML → CotEvent → CotDocument
3. Store document in Ditto collection via DQL
4. Query document back from Ditto
5. Convert back to XML and verify semantic equality

### Multi-Peer E2E Tests

**Purpose**: Test distributed P2P scenarios with conflict resolution

**Test Scenario**:
1. **Setup**: Two peers establish connection
2. **Creation**: Peer A creates CoT MapItem document
3. **Sync**: Document syncs to Peer B automatically
4. **Offline**: Both peers go offline independently
5. **Conflicts**: Each peer makes different modifications
6. **Reconnect**: Peers come back online
7. **Resolution**: Validate CRDT merge behavior

**Commands**:
```bash
cargo test e2e_multi_peer_mapitem_sync_test
```

**Key Validations**:
- Automatic peer discovery
- Real-time synchronization
- Offline resilience
- Conflict resolution via CRDT semantics
- DQL integration

## Cross-Language Testing

### Integration Test System

**Purpose**: Validate compatibility between Rust and Java implementations

**Command**: `make test-integration`

**Process**:
1. Build Rust integration client
2. Build Java integration client  
3. Run both clients with identical CoT XML input
4. Compare JSON outputs for consistency
5. Validate round-trip conversions

### Manual Cross-Language Validation

```bash
# Generate test outputs
make example-rust > rust-output.json
make example-java > java-output.json

# Compare outputs
jq '.ditto_document' rust-output.json > rust-doc.json
jq '.ditto_document' java-output.json > java-doc.json
diff rust-doc.json java-doc.json

# Should show no differences for compatible implementations
```

### Validation Checks

- **Identical document structure**
- **Same stable key generation**
- **Compatible metadata formats**
- **Consistent field mappings**
- **Equivalent CRDT behavior**

## Performance Testing

### Rust Benchmarks

```bash
cd rust

# Install benchmark tools
cargo install criterion

# Run benchmarks
cargo bench

# Specific benchmarks
cargo bench xml_parsing
cargo bench crdt_conversion
cargo bench document_creation
```

### Java Performance Tests

```bash
cd java

# JMH benchmarks (if implemented)
./gradlew jmh

# Performance integration tests
./gradlew test --tests "*PerformanceTest"
```

### Performance Metrics

**Target Benchmarks**:
- XML parsing: < 1ms for 10KB documents
- CRDT conversion: < 100μs for typical documents
- Memory usage: < 10MB baseline
- Concurrent throughput: > 1000 ops/sec

## Test Data and Fixtures

### XML Test Files

**Location**: `schema/example_xml/`

**Files**:
- `simple_location.xml` - Basic location update
- `complex_detail.xml` - Complex detail with duplicates (13 elements)
- `chat_message.xml` - Chat event
- `file_share.xml` - File sharing event
- `emergency.xml` - Emergency/API event

### Document Types Tested

1. **MapItem Documents**
   - Location updates
   - Map graphics
   - Tracking data

2. **Chat Documents**
   - Messages
   - Room management
   - Author information

3. **File Documents**
   - File metadata
   - Sharing information
   - MIME types

4. **API Documents**
   - Emergency events
   - Custom API calls

5. **Generic Documents**
   - Fallback for unknown types
   - Custom detail preservation

### Test Scenarios

**CRDT Optimization Scenarios**:
- Duplicate element preservation
- Stable key generation
- Cross-language compatibility
- P2P convergence simulation

**Error Handling Scenarios**:
- Malformed XML
- Invalid CoT event types
- Missing required fields
- Network connectivity issues

## Debugging Tests

### Rust Test Debugging

```bash
# Debug output
cargo test -- --nocapture

# Specific test with logs
RUST_LOG=debug cargo test test_name -- --nocapture

# Debug build for better stack traces
cargo test --debug

# Running single test
cargo test test_specific_function -- --exact
```

### Java Test Debugging

```bash
# Debug output
./gradlew test --info

# Test debugging in IDE
./gradlew test --debug-jvm

# Detailed test reports
./gradlew test --continue
# View: build/reports/tests/test/index.html
```

### Common Test Issues

**Ditto Connection Failures**:
- Verify credentials are set
- Check network connectivity
- Ensure app ID is valid

**XML Parsing Failures**:
- Validate XML syntax
- Check character encoding
- Verify schema compliance

**Cross-Language Mismatches**:
- Verify schema versions match
- Check stable key generation
- Validate metadata consistency

## Test Maintenance

### Adding New Tests

1. **Create test data** in shared fixtures
2. **Add unit tests** for new functionality
3. **Update integration tests** for new workflows
4. **Ensure cross-language coverage**
5. **Document test purpose** and expected behavior

### Test Coverage Goals

- **Unit Test Coverage**: > 80%
- **Integration Coverage**: All major workflows
- **Cross-Language Compatibility**: 100% for core features
- **E2E Coverage**: Critical user journeys

### Continuous Integration

Tests run automatically on:
- Pull requests
- Commits to main branch
- Nightly builds for extended test suites
- Cross-language compatibility validation

This comprehensive testing strategy ensures the Ditto CoT library maintains high quality and reliability across all supported languages and use cases.

## See Also

- **[Getting Started](getting-started.md)** - Initial setup and basic usage examples
- **[Building Guide](building.md)** - Build system and compilation processes
- **[Troubleshooting](../reference/troubleshooting.md)** - Debugging failing tests and common issues
- **[Performance](../technical/performance.md)** - Performance testing and benchmarking
- **[Integration Examples](../integration/examples/)** - Real-world usage patterns for testing
- **[API Reference](../reference/api-reference.md)** - Complete API documentation for test development