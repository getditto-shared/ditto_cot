# Troubleshooting Guide

Common issues, solutions, and debugging strategies for the Ditto CoT library.

## Table of Contents

- [Quick Diagnostics](#quick-diagnostics)
- [Build Issues](#build-issues)
- [Runtime Errors](#runtime-errors)
- [Integration Problems](#integration-problems)
- [Performance Issues](#performance-issues)
- [Ditto SDK Issues](#ditto-sdk-issues)
- [Debugging Tools](#debugging-tools)

## Quick Diagnostics

### Environment Check

Run these commands to verify your setup:

```bash
# Check versions
rustc --version     # Should be 1.70+
java -version       # Should be 17+
node --version      # If using Node.js tools

# Check environment variables
echo $DITTO_APP_ID
echo $DITTO_PLAYGROUND_TOKEN

# Test build
make test           # Run all tests
```

### Common Issues Checklist

- [ ] Correct language versions installed
- [ ] Environment variables set
- [ ] Network connectivity to Ditto
- [ ] Valid Ditto credentials
- [ ] Required dependencies installed

## Build Issues

### Rust Build Problems

#### Error: "failed to run custom build command for `ditto_cot`"

**Symptoms:**
```
error: failed to run custom build command for `ditto_cot v0.1.0`
Caused by: process didn't exit successfully
```

**Solutions:**

1. **Check build dependencies:**
```bash
# Ensure build tools are installed
cargo clean
cargo build -vv  # Verbose output for debugging
```

2. **Schema generation issues:**
```bash
# Validate JSON schema
jq empty schema/ditto.schema.json

# Check schema file permissions
ls -la schema/ditto.schema.json
```

3. **Platform-specific issues:**
```bash
# Linux: Install build essentials
sudo apt-get install build-essential

# macOS: Install Xcode command line tools
xcode-select --install

# Windows: Install Visual Studio Build Tools
```

#### Error: "linker `cc` not found"

**Solution:**
```bash
# Install C compiler
# Ubuntu/Debian:
sudo apt-get install gcc

# CentOS/RHEL:
sudo yum install gcc

# macOS:
xcode-select --install
```

#### Error: "could not find native static library `ditto`"

**Solution:**
```bash
# Ensure Ditto SDK is properly linked
export DITTO_SDK_PATH=/path/to/ditto/sdk
cargo clean && cargo build
```

### Java Build Problems

#### Error: "Unsupported class file major version"

**Symptoms:**
```
java.lang.UnsupportedClassFileVersionError: 
Unsupported major.minor version
```

**Solution:**
```bash
# Check Java version
java -version
javac -version

# Ensure Java 17+
export JAVA_HOME=/path/to/java17
./gradlew --version
```

#### Error: "Could not resolve dependencies"

**Solutions:**

1. **Check network connectivity:**
```bash
# Test Maven Central access
curl -I https://repo1.maven.org/maven2/

# Configure proxy if needed
./gradlew -Dhttp.proxyHost=proxy.company.com -Dhttp.proxyPort=8080 build
```

2. **Clear Gradle cache:**
```bash
./gradlew clean
rm -rf ~/.gradle/caches
./gradlew build --refresh-dependencies
```

#### Error: "Gradle wrapper permissions"

**Solution:**
```bash
chmod +x gradlew
./gradlew build
```

### Schema Generation Issues

#### Error: "Schema file not found"

**Symptoms:**
```
Build failed: could not read schema/ditto.schema.json
```

**Solutions:**

1. **Verify file exists:**
```bash
ls -la schema/
cat schema/ditto.schema.json | head
```

2. **Check file permissions:**
```bash
chmod 644 schema/ditto.schema.json
```

3. **Validate JSON syntax:**
```bash
jq . schema/ditto.schema.json > /dev/null && echo "Valid JSON" || echo "Invalid JSON"
```

## Runtime Errors

### XML Parsing Errors

#### Error: "XML parsing failed"

**Common Causes:**

1. **Malformed XML:**
```xml
<!-- BAD: Missing closing tag -->
<event version="2.0" uid="TEST">
  <point lat="34.0" lon="-118.0"/>
<!-- Missing </event> -->

<!-- GOOD: Well-formed XML -->
<event version="2.0" uid="TEST">
  <point lat="34.0" lon="-118.0"/>
</event>
```

2. **Invalid characters:**
```xml
<!-- BAD: Unescaped characters -->
<detail>Message with & symbols</detail>

<!-- GOOD: Escaped characters -->
<detail>Message with &amp; symbols</detail>
```

3. **Encoding issues:**
```bash
# Check file encoding
file -bi your_file.xml

# Convert to UTF-8 if needed
iconv -f ISO-8859-1 -t UTF-8 input.xml > output.xml
```

**Debugging XML Issues:**

```bash
# Validate XML syntax
xmllint --noout your_file.xml

# Pretty print XML
xmllint --format your_file.xml

# Check specific element
xmllint --xpath "//event/@uid" your_file.xml
```

### Document Conversion Errors

#### Error: "Document conversion failed"

**Rust Debugging:**
```rust
use ditto_cot::{cot_events::CotEvent, ditto::cot_to_document};

match CotEvent::from_xml(xml) {
    Ok(event) => {
        println!("Parsed event: {:?}", event);
        let doc = cot_to_document(&event, "debug-peer");
        println!("Converted document: {:?}", doc);
    },
    Err(e) => {
        eprintln!("Parse error: {}", e);
        eprintln!("XML content: {}", xml);
    }
}
```

**Java Debugging:**
```java
try {
    CotEvent event = CotEvent.fromXml(xml);
    System.out.println("Parsed event: " + event);
    
    SdkDocumentConverter converter = new SdkDocumentConverter();
    Map<String, Object> doc = converter.convertToDocumentMap(event, "debug-peer");
    System.out.println("Converted document: " + doc);
    
} catch (Exception e) {
    System.err.println("Conversion error: " + e.getMessage());
    System.err.println("XML content: " + xml);
    e.printStackTrace();
}
```

### Validation Errors

#### Error: "Invalid coordinates"

**Common Issues:**

1. **Out of range values:**
```rust
// BAD: Invalid latitude
let point = Point::new(91.0, -118.0, 100.0); // Latitude > 90

// GOOD: Valid coordinates
let point = Point::new(34.0, -118.0, 100.0);
```

2. **NaN or infinite values:**
```java
// Check for invalid values
if (Double.isNaN(lat) || Double.isInfinite(lat)) {
    throw new IllegalArgumentException("Invalid latitude: " + lat);
}
```

#### Error: "Required field missing"

**Solutions:**

1. **Check required fields:**
```rust
// Ensure UID is provided
let event = CotEvent::builder()
    .uid("REQUIRED-UID")  // This is required
    .event_type("a-f-G-U-C")  // This is required
    .build();
```

2. **Validate before processing:**
```java
public void validateCotEvent(CotEvent event) {
    if (event.getUid() == null || event.getUid().trim().isEmpty()) {
        throw new ValidationException("UID is required");
    }
    
    if (event.getType() == null || event.getType().trim().isEmpty()) {
        throw new ValidationException("Event type is required");
    }
}
```

## Integration Problems

### Ditto SDK Issues

#### Error: "DQL mutations not supported"

**Symptoms:**
```
DittoError: DqlUnsupported
```

**Solutions:**

1. **Check SDK version:**
```rust
// Ensure you're using a compatible Ditto SDK version
println!("Ditto version: {}", ditto.version());
```

2. **Verify sync configuration:**
```rust
let ditto = Ditto::builder()
    .with_identity(DittoIdentity::OnlinePlayground {
        app_id: app_id.clone(),
        token: token.clone(),
        enable_ditto_cloud_sync: true,  // Important for DQL
    })?
    .build()?;
```

3. **Use alternative query methods:**
```rust
// If DQL mutations fail, use collection operations
let collection = ditto.store().collection("map_items");
collection.upsert(doc).await?;
```

#### Error: "Authentication failed"

**Solutions:**

1. **Verify credentials:**
```bash
# Check environment variables
echo "App ID: $DITTO_APP_ID"
echo "Token: $DITTO_PLAYGROUND_TOKEN"

# Test credentials with curl
curl -H "Authorization: Bearer $DITTO_PLAYGROUND_TOKEN" \
     "https://portal.ditto.live/api/v1/apps/$DITTO_APP_ID"
```

2. **Check token expiration:**
```rust
// Tokens may expire - regenerate from Ditto portal
let identity = DittoIdentity::OnlinePlayground {
    app_id: "your-app-id".to_string(),
    token: "fresh-token".to_string(),
    enable_ditto_cloud_sync: true,
};
```

### Observer Issues

#### Error: "Observer not receiving updates"

**Debugging Steps:**

1. **Verify observer registration:**
```rust
let subscription = store
    .collection("map_items")
    .find_all()
    .subscribe()
    .observe(|docs, event| {
        println!("Observer triggered with {} docs", docs.len());
        for doc in docs {
            println!("Document: {:?}", doc.value());
        }
    })?;

// Keep subscription alive
std::mem::forget(subscription);
```

2. **Check query syntax:**
```java
// Ensure DQL query is valid
String query = "SELECT * FROM map_items WHERE w LIKE 'a-f-%'";
try {
    DittoQueryResult result = store.execute(query);
    System.out.println("Query returned " + result.getItems().size() + " items");
} catch (Exception e) {
    System.err.println("Query failed: " + e.getMessage());
}
```

3. **Verify data exists:**
```bash
# Use Ditto CLI to check data
ditto-cli query "SELECT COUNT(*) FROM map_items"
```

### Network Connectivity Issues

#### Error: "Connection timeout"

**Solutions:**

1. **Check network connectivity:**
```bash
# Test basic connectivity
ping portal.ditto.live

# Test HTTPS connectivity
curl -I https://portal.ditto.live

# Check for proxy issues
curl --proxy http://proxy:8080 -I https://portal.ditto.live
```

2. **Configure timeouts:**
```rust
use tokio::time::{timeout, Duration};

let result = timeout(Duration::from_secs(30), async {
    ditto.store().execute_v2((query, params)).await
}).await??;
```

3. **Implement retry logic:**
```java
public void storeWithRetry(Map<String, Object> doc, int maxRetries) {
    int attempts = 0;
    while (attempts < maxRetries) {
        try {
            store.execute("INSERT INTO collection DOCUMENTS (?)", doc);
            return; // Success
        } catch (Exception e) {
            attempts++;
            if (attempts >= maxRetries) {
                throw new RuntimeException("Max retries exceeded", e);
            }
            
            try {
                Thread.sleep(1000 * attempts); // Exponential backoff
            } catch (InterruptedException ie) {
                Thread.currentThread().interrupt();
                throw new RuntimeException("Interrupted", ie);
            }
        }
    }
}
```

## Performance Issues

### Slow Processing

#### Symptoms

- High CPU usage during document conversion
- Slow XML parsing
- Memory leaks during batch processing

#### Solutions

1. **Profile performance:**
```rust
use std::time::Instant;

let start = Instant::now();
let doc = cot_to_document(&event, peer_id);
let duration = start.elapsed();
println!("Conversion took: {:?}", duration);
```

2. **Optimize batch processing:**
```java
// Use parallel processing
List<CompletableFuture<String>> futures = xmlList.parallelStream()
    .map(xml -> CompletableFuture.supplyAsync(() -> processXml(xml)))
    .collect(Collectors.toList());

CompletableFuture.allOf(futures.toArray(new CompletableFuture[0])).join();
```

3. **Cache frequently used objects:**
```rust
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref CONVERTER_CACHE: Mutex<HashMap<String, CotDocument>> = 
        Mutex::new(HashMap::new());
}

fn get_or_convert(xml: &str, peer_id: &str) -> CotDocument {
    let cache_key = format!("{}-{}", hash(xml), peer_id);
    
    if let Ok(cache) = CONVERTER_CACHE.lock() {
        if let Some(cached) = cache.get(&cache_key) {
            return cached.clone();
        }
    }
    
    // Convert and cache
    let event = CotEvent::from_xml(xml).unwrap();
    let doc = cot_to_document(&event, peer_id);
    
    if let Ok(mut cache) = CONVERTER_CACHE.lock() {
        cache.insert(cache_key, doc.clone());
    }
    
    doc
}
```

### Memory Issues

#### High Memory Usage

**Solutions:**

1. **Monitor memory usage:**
```rust
// In Rust, use instruments or valgrind
// cargo install cargo-profiler
cargo profiler --release --bin your_app

// Check for memory leaks
cargo test --release -- --test-threads=1
```

2. **Implement memory limits:**
```java
// Set JVM memory limits
java -Xmx2g -Xms1g YourApplication

// Monitor memory usage
MemoryMXBean memoryBean = ManagementFactory.getMemoryMXBean();
MemoryUsage heapMemoryUsage = memoryBean.getHeapMemoryUsage();
System.out.println("Used memory: " + heapMemoryUsage.getUsed() + " bytes");
```

3. **Clean up resources:**
```rust
// Ensure proper cleanup
impl Drop for YourStruct {
    fn drop(&mut self) {
        // Clean up resources
        println!("Cleaning up resources");
    }
}
```

## Debugging Tools

### Logging Configuration

#### Rust Logging

```rust
// Add to Cargo.toml
[dependencies]
env_logger = "0.10"
log = "0.4"

// In main.rs
use log::{info, debug, error};

fn main() {
    env_logger::init();
    
    debug!("Debug message");
    info!("Info message");
    error!("Error message");
}
```

Run with logging:
```bash
RUST_LOG=debug cargo run
RUST_LOG=ditto_cot=trace cargo run  # Library-specific logging
```

#### Java Logging

```java
import java.util.logging.Logger;
import java.util.logging.Level;

public class YourClass {
    private static final Logger logger = Logger.getLogger(YourClass.class.getName());
    
    public void someMethod() {
        logger.info("Processing CoT event");
        logger.log(Level.FINE, "Debug details: {0}", details);
    }
}
```

Configure logging:
```properties
# logging.properties
.level = INFO
com.ditto.cot.level = FINE
java.util.logging.ConsoleHandler.level = ALL
```

### Debug Output

#### Detailed Error Information

```rust
// Enhanced error reporting
match CotEvent::from_xml(xml) {
    Ok(event) => { /* success */ },
    Err(e) => {
        eprintln!("Parse error: {}", e);
        eprintln!("Error type: {:?}", e);
        eprintln!("XML length: {}", xml.len());
        eprintln!("XML preview: {}", &xml[..std::cmp::min(200, xml.len())]);
        
        // Try to identify problematic section
        if let Some(line) = find_error_line(xml, &e) {
            eprintln!("Problematic line: {}", line);
        }
    }
}
```

### Testing Tools

#### Unit Test Debugging

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn debug_conversion_issue() {
        let xml = r#"<your xml here>"#;
        
        // Enable logging for this test
        let _ = env_logger::builder().is_test(true).try_init();
        
        let result = CotEvent::from_xml(xml);
        println!("Result: {:?}", result);
        
        assert!(result.is_ok(), "Expected successful parsing");
    }
}
```

#### Integration Test Debugging

```bash
# Run single test with output
cargo test debug_conversion_issue -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo test -- --nocapture

# Java test debugging
./gradlew test --tests YourTestClass --info
```

### Performance Profiling

#### Rust Profiling

```bash
# Install profiling tools
cargo install cargo-profiler
cargo install flamegraph

# Profile your application
cargo flamegraph --bin your_app

# Memory profiling with valgrind
cargo build --release
valgrind --tool=massif target/release/your_app
```

#### Java Profiling

```bash
# Enable JFR profiling
java -XX:+FlightRecorder -XX:StartFlightRecording=duration=60s,filename=profile.jfr YourApp

# Analyze with JMC or convert to text
jfr print --categories GC,Memory profile.jfr
```

### Common Debugging Scenarios

#### Scenario 1: XML Not Parsing

1. **Validate XML syntax:**
```bash
xmllint --noout your_file.xml
```

2. **Check encoding:**
```bash
file -bi your_file.xml
```

3. **Test with minimal XML:**
```xml
<event version="2.0" uid="TEST" type="a-f-G-U-C" time="2024-01-15T10:30:00.000Z" start="2024-01-15T10:30:00.000Z" stale="2024-01-15T10:35:00.000Z">
  <point lat="34.0" lon="-118.0" hae="100.0"/>
  <detail></detail>
</event>
```

#### Scenario 2: Documents Not Syncing

1. **Check Ditto connection:**
```rust
println!("Ditto status: {:?}", ditto.presence_graph());
```

2. **Verify document structure:**
```bash
# Query documents directly
ditto-cli query "SELECT * FROM your_collection LIMIT 5"
```

3. **Check observer registration:**
```java
// Add debug logging to observer
store.registerObserver("SELECT * FROM collection", (result, event) -> {
    System.out.println("Observer triggered: " + result.getItems().size() + " items");
    System.out.println("Event type: " + event);
});
```

#### Scenario 3: Performance Degradation

1. **Profile hot paths:**
```rust
use std::time::Instant;

let start = Instant::now();
// Your code here
println!("Operation took: {:?}", start.elapsed());
```

2. **Monitor resource usage:**
```bash
# System monitoring
top -p $(pgrep your_process)
htop
iostat 1
```

3. **Check for memory leaks:**
```java
// Java heap dump
jcmd <pid> GC.run_finalization
jcmd <pid> VM.gc
jmap -dump:format=b,file=heap.hprof <pid>
```

For additional help, check:
- [API Reference](api-reference.md) for correct usage patterns
- [Integration Examples](../integration/) for working code samples
- [GitHub Issues](https://github.com/getditto-shared/ditto_cot/issues) for known issues and solutions