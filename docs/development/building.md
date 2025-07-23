# Building Ditto CoT

This guide covers building the Ditto CoT library from source across all supported languages and platforms.

> **Quick Navigation**: [Getting Started](getting-started.md) | [Testing Guide](testing.md) | [Troubleshooting](../reference/troubleshooting.md) | [Architecture](../technical/architecture.md)

## Table of Contents

- [Prerequisites](#prerequisites)
- [Unified Build System](#unified-build-system)
- [Language-Specific Builds](#language-specific-builds)
- [Build Outputs](#build-outputs)
- [Development Builds](#development-builds)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### System Requirements

**All Platforms**:
- Git
- Make (optional but recommended)

**For Rust**:
- Rust 1.70+ with Cargo
- System dependencies for your platform

**For Java**:
- JDK 17 or later
- Gradle 7.0+ (wrapper included)

**For C#** (planned):
- .NET SDK 6.0+

### Platform-Specific Setup

**Linux/macOS**:
```bash
# Install development tools
# Ubuntu/Debian:
sudo apt-get install build-essential git

# macOS:
xcode-select --install
```

**Windows**:
```powershell
# Install Visual Studio Build Tools or Visual Studio
# Ensure Git is available in PATH
```

## Unified Build System

The repository includes a top-level `Makefile` providing unified commands across all languages:

### Quick Build Commands

```bash
# Build all language libraries
make all

# Build specific language
make rust
make java
make csharp

# Clean all builds
make clean

# Run tests for all languages
make test

# Show all available commands
make help
```

### Available Make Targets

| Command | Description |
|---------|-------------|
| `make all` | Build all language implementations |
| `make rust` | Build Rust library |
| `make java` | Build Java library |
| `make csharp` | Build C# library (planned) |
| `make test` | Run tests for all languages |
| `make test-rust` | Run Rust tests only |
| `make test-java` | Run Java tests only |
| `make clean` | Clean all build artifacts |
| `make example-rust` | Build and run Rust integration example |
| `make example-java` | Build and run Java integration example |
| `make test-integration` | Run cross-language integration tests |

## Language-Specific Builds

### Rust Build System

**Build Tool**: Cargo with custom build script

#### Build Commands

```bash
cd rust

# Standard build
cargo build

# Release build (optimized)
cargo build --release

# Build with all features
cargo build --all-features

# Build specific examples
cargo build --example e2e_test
```

#### Custom Build Script

The Rust implementation uses `build.rs` for:
- Code generation from JSON schema
- Underscore-prefixed field handling
- Cross-platform compatibility

#### Build Configuration

**Cargo.toml features**:
```toml
[features]
default = ["serde"]
serde = ["dep:serde", "dep:serde_json"]
ditto-sdk = ["dep:dittolive_ditto"]
```

#### Build Outputs

```
rust/target/
├── debug/
│   ├── libditto_cot.rlib     # Rust library
│   └── examples/             # Example binaries
└── release/
    └── libditto_cot.rlib     # Optimized library
```

### Java Build System

**Build Tool**: Gradle with wrapper

#### Build Commands

```bash
cd java

# Standard build (includes tests, Javadoc, fat JAR)
./gradlew build

# Quick compile without tests
./gradlew compileJava

# Generate Javadoc
./gradlew javadoc

# Build fat JAR with dependencies
./gradlew fatJar

# Run specific test suite
./gradlew test --tests "com.ditto.cot.*"
```

#### Gradle Tasks

| Task | Description |
|------|-------------|
| `build` | Full build with tests and documentation |
| `compileJava` | Compile source code only |
| `test` | Run unit tests |
| `javadoc` | Generate API documentation |
| `fatJar` | Create JAR with all dependencies |
| `clean` | Remove build artifacts |

#### Build Configuration

**Key Gradle settings**:
- Java compatibility: 17
- Encoding: UTF-8
- Test framework: JUnit 5
- Code coverage: JaCoCo

#### Build Outputs

```
java/build/
├── libs/
│   ├── ditto-cot-1.0-SNAPSHOT.jar         # Main JAR
│   ├── ditto-cot-1.0-SNAPSHOT-sources.jar # Source JAR
│   ├── ditto-cot-1.0-SNAPSHOT-javadoc.jar # Documentation JAR
│   └── ditto-cot-all.jar                  # Fat JAR (all dependencies)
├── docs/javadoc/                          # Generated documentation
└── reports/
    ├── tests/                             # Test reports
    └── jacoco/                            # Coverage reports
```

### C# Build System (Planned)

**Build Tool**: .NET SDK

#### Build Commands (Future)

```bash
cd csharp

# Build library
dotnet build

# Build release
dotnet build -c Release

# Run tests
dotnet test

# Create package
dotnet pack
```

## Build Outputs

### Rust Library

**Development**: `rust/target/debug/libditto_cot.rlib`
**Release**: `rust/target/release/libditto_cot.rlib`
**Documentation**: Generated via `cargo doc`

### Java Library

**Main JAR**: `java/build/libs/ditto-cot-1.0-SNAPSHOT.jar`
**Fat JAR**: `java/build/libs/ditto-cot-all.jar` (recommended for standalone use)
**Documentation**: `java/build/docs/javadoc/`

### Using Build Outputs

#### Java Fat JAR Usage

```bash
# Convert CoT XML file
java -jar build/libs/ditto-cot-all.jar convert input.xml output.json

# Show help
java -jar build/libs/ditto-cot-all.jar --help
```

## Development Builds

### Rust Development

```bash
# Enable debug logging
RUST_LOG=debug cargo build

# Fast incremental builds
cargo check

# Watch for changes (requires cargo-watch)
cargo install cargo-watch
cargo watch -x check

# Profile build times
cargo build --timings
```

### Java Development

```bash
# Continuous testing
./gradlew test --continuous

# Build without running tests
./gradlew assemble

# Parallel builds
./gradlew build --parallel

# Debug build issues
./gradlew build --info
```

### Schema Code Generation

Both implementations generate code from `schema/ditto.schema.json`:

**Rust**: Automatic during `cargo build` via `build.rs`
**Java**: Automatic during Gradle build

**Forcing Regeneration**:
```bash
# Rust
cargo clean && cargo build

# Java
./gradlew clean build
```

## Cross-Language Integration

### Integration Testing

```bash
# Build integration test clients
make example-rust  # Creates rust/target/debug/examples/integration_client
make example-java  # Creates java/build/distributions/integration-client

# Run cross-language compatibility test
make test-integration
```

### Schema Validation

Both implementations must produce identical output for the same input:

```bash
# Test schema compatibility
make test-integration

# Manual verification
make example-rust | jq '.ditto_document' > rust-output.json
make example-java | jq '.ditto_document' > java-output.json
diff rust-output.json java-output.json
```

## Troubleshooting

### Common Build Issues

#### Rust Issues

**Error**: "failed to run custom build command for `ditto_cot`"
**Solution**: Ensure build dependencies are installed:
```bash
cargo clean
cargo build -vv  # Verbose output for debugging
```

**Error**: Schema generation failures
**Solution**: Check JSON schema syntax:
```bash
# Validate schema
jq empty schema/ditto.schema.json
```

#### Java Issues

**Error**: "Unsupported class file major version"
**Solution**: Verify JDK version:
```bash
java -version  # Should be 17+
./gradlew --version
```

**Error**: Gradle wrapper permissions
**Solution**: Fix permissions:
```bash
chmod +x gradlew
```

**Error**: Test failures
**Solution**: Check test configuration:
```bash
./gradlew test --info  # Detailed test output
```

### Build Performance

#### Rust Optimization

```bash
# Use faster linker (Linux)
cargo install -f cargo-binutils
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Parallel compilation
export CARGO_BUILD_JOBS=4
```

#### Java Optimization

```bash
# Increase Gradle memory
export GRADLE_OPTS="-Xmx2g"

# Enable parallel builds
echo "org.gradle.parallel=true" >> gradle.properties
```

### Clean Rebuild

**Complete clean**:
```bash
make clean
git clean -fdx  # WARNING: Removes all untracked files
make all
```

**Language-specific clean**:
```bash
# Rust
cargo clean

# Java
./gradlew clean
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Test
on: [push, pull_request]

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: make rust
      - run: make test-rust

  java:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-java@v3
        with:
          java-version: '17'
          distribution: 'temurin'
      - run: make java
      - run: make test-java

  integration:
    needs: [rust, java]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: make all
      - run: make test-integration
```

## Next Steps

After successful builds:

1. **Run Tests**: Follow the [Testing Guide](testing.md)
2. **Integration**: See [Ditto SDK Integration](../integration/ditto-sdk.md)
3. **Contributing**: Review contribution guidelines
4. **Performance**: Benchmark with the [Performance Guide](../technical/performance.md)

## See Also

- **[Getting Started](getting-started.md)** - Initial setup and basic usage
- **[Testing Guide](testing.md)** - Running tests and debugging build issues
- **[Troubleshooting](../reference/troubleshooting.md)** - Common build problems and solutions
- **[Architecture](../technical/architecture.md)** - Understanding the multi-language build system
- **[Integration Examples](../integration/examples/)** - Using the built libraries in projects