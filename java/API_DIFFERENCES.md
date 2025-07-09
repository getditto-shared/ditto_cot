# Ditto SDK API Differences: Rust vs Java

## Key Differences Summary

### Package Structure
**Rust**: `dittolive_ditto::*`
**Java**: `com.ditto.java.*`

### Core Classes

| Concept | Rust | Java |
|---------|------|------|
| Main SDK class | `Ditto` | `com.ditto.java.Ditto` |
| Store operations | `Store` | `com.ditto.java.DittoStore` |
| Configuration | `Ditto::builder()` | `com.ditto.java.DittoConfig` |
| Query results | Direct collections | `com.ditto.java.DittoQueryResult` |
| Observers | Direct callbacks | `com.ditto.java.DittoStoreObserver` |

### Document Handling Differences

#### Rust Approach:
```rust
// Rust has DittoDocument trait and specific document types
pub trait DittoDocument {
    fn id(&self) -> String;
    // Document-specific methods
}

// Direct document manipulation
let store = ditto.store();
let result = store.execute_v2("SELECT * FROM collection").await?;
for doc in result.iter() {
    let json = doc.json_string();
    let cot_doc = CotDocument::from_json_str(&json)?;
}
```

#### Java Approach:
```java
// Java works with generic Map<String, Object> documents
DittoStore store = ditto.getStore();
DittoQueryResult result = store.execute("SELECT * FROM collection");
for (DittoQueryResultItem item : result.getItems()) {
    Map<String, Object> data = item.getValue();
    // Convert to CoT document
}
```

### Missing DittoDocument Concept in Java

**The Java SDK does NOT have an equivalent to Rust's `DittoDocument` trait.** Instead:

- **Rust**: Strongly-typed document classes implementing `DittoDocument`
- **Java**: Generic `Map<String, Object>` for all document operations

### Integration Strategy for Java

Since Java doesn't have `DittoDocument`, we need to:

1. **Use our CoT schema classes as DTOs** (Data Transfer Objects)
2. **Convert between CoT DTOs and Ditto Maps** via JSON serialization
3. **Implement our own document ID management**

```java
// Proposed Java integration pattern:
MapItemDocument cotDoc = (MapItemDocument) converter.convertToDocument(xml);

// Convert to Ditto-compatible Map
Map<String, Object> dittoDoc = objectMapper.convertValue(cotDoc, Map.class);

// Store in Ditto
store.execute("INSERT INTO cot_events DOCUMENTS (?)", dittoDoc);

// Retrieve from Ditto
DittoQueryResult result = store.execute("SELECT * FROM cot_events WHERE id = ?", docId);
Map<String, Object> data = result.getItems().get(0).getValue();

// Convert back to CoT
MapItemDocument retrieved = objectMapper.convertValue(data, MapItemDocument.class);
```

### Key API Methods to Implement

For Java integration, we need:

1. **CoTConverter.toMap()** - Convert CoT documents to Map<String,Object>
2. **CoTConverter.fromMap()** - Convert Map<String,Object> to CoT documents  
3. **JSON serialization utilities** for the conversion bridge
4. **ID management** since Java doesn't have built-in document ID handling

### Next Steps

1. Add Jackson serialization to convert between CoT documents and Maps
2. Implement toMap/fromMap methods in CoTConverter
3. Create integration tests with actual Ditto store operations
4. Build Java equivalent of the Rust multi-peer test