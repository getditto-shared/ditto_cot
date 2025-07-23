# Java Implementation

Java implementation of the Ditto CoT library with type-safe document models and Android compatibility.

## 🚀 Quick Start

### Installation

**Gradle**:
```groovy
dependencies {
    implementation 'com.ditto:ditto-cot:1.0-SNAPSHOT'
}
```

**Maven**:
```xml
<dependency>
  <groupId>com.ditto</groupId>
  <artifactId>ditto-cot</artifactId>
  <version>1.0-SNAPSHOT</version>
</dependency>
```

### Basic Usage

```java
import com.ditto.cot.CotEvent;

CotEvent event = CotEvent.builder()
    .uid("USER-123")
    .type("a-f-G-U-C")
    .point(34.12345, -118.12345, 150.0)
    .callsign("ALPHA-1")
    .build();

String xml = event.toXml();
```

## 🏗️ Java-Specific Features

### Requirements
- **Java 17+** (LTS recommended)
- **Android API 26+** for mobile applications
- **Gradle 7.0+** build system

### Type System
- **Map-based Documents**: Java works with `Map<String, Object>` for Ditto integration
- **Schema DTOs**: Generated POJOs from JSON schema for type safety
- **Jackson Integration**: Seamless JSON serialization/deserialization

### Builder Pattern API

```java
import com.ditto.cot.CotEvent;
import java.time.Instant;

// Complex event with detail section
CotEvent event = CotEvent.builder()
    .uid("USER-123")
    .type("a-f-G-U-C")
    .time(Instant.now())
    .point(34.12345, -118.12345, 150.0, 10.0, 25.0)
    .detail()
        .callsign("ALPHA-1")
        .groupName("BLUE")
        .add("custom_field", "value")
        .build()
    .build();
```

### Android Support

Optimized for Android development:
- **Minimal APK Impact**: Core library < 500KB
- **ProGuard Ready**: Obfuscation rules included
- **API 26+ Compatible**: Modern Android versions
- **Background Services**: Efficient P2P sync

## 🔌 Ditto SDK Integration

### Document Conversion

```java
import com.ditto.cot.SdkDocumentConverter;
import com.ditto.cot.schema.*;

SdkDocumentConverter converter = new SdkDocumentConverter();

// Observer callback integration
store.registerObserver("SELECT * FROM map_items", (result, event) -> {
    for (DittoQueryResultItem item : result.getItems()) {
        Map<String, Object> docMap = item.getValue();
        
        // Convert to typed document
        Object typedDoc = converter.observerMapToTypedDocument(docMap);
        
        if (typedDoc instanceof MapItemDocument) {
            MapItemDocument mapItem = (MapItemDocument) typedDoc;
            System.out.println("Location: " + mapItem.getId());
        }
    }
});
```

### Fat JAR Command Line

```bash
# Build standalone JAR
./gradlew fatJar

# Convert files
java -jar build/libs/ditto-cot-all.jar convert input.xml output.json
```

## 🧪 Testing

```bash
# All tests with coverage
./gradlew test jacocoTestReport

# Specific test patterns
./gradlew test --tests "*CRDT*"
./gradlew test --tests "*IntegrationTest"

# Example demonstration
./gradlew test --tests "com.ditto.cot.example.SimpleExample"
```

## 🏗️ Build System

**Gradle Features**:
- Multi-module support
- Code generation from JSON schema
- JaCoCo coverage reporting (targeting 80%+)
- Checkstyle integration
- Javadoc generation

**Build Outputs**:
- `ditto-cot-1.0-SNAPSHOT.jar` - Main library
- `ditto-cot-all.jar` - Fat JAR with dependencies
- Coverage reports in `build/reports/jacoco/`

## 📚 Documentation

- **Javadoc**: Generated in `build/docs/javadoc/`
- **Examples**: `src/test/java/com/ditto/cot/example/`
- **Integration Guide**: [Java Examples](../docs/integration/examples/java.md)

For comprehensive documentation, see the [main documentation](../docs/).
