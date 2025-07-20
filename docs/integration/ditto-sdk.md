# Ditto SDK Integration Guide

This guide covers comprehensive integration patterns for using the Ditto CoT library with the Ditto SDK across all supported languages.

> **Quick Navigation**: [Rust Examples](examples/rust.md) | [Java Examples](examples/java.md) | [API Reference](../reference/api-reference.md) | [Schema Reference](../reference/schema.md)

## Table of Contents

- [Overview](#overview)
- [SDK Integration Patterns](#sdk-integration-patterns)
- [Observer Document Conversion](#observer-document-conversion)
- [DQL Operations](#dql-operations)
- [Real-time Synchronization](#real-time-synchronization)
- [Authentication & Setup](#authentication--setup)
- [Advanced Patterns](#advanced-patterns)
- [Troubleshooting](#troubleshooting)

## Overview

The Ditto CoT library provides seamless integration with the Ditto SDK, enabling:

- **Real-time P2P synchronization** of CoT events
- **Observer pattern integration** for live updates
- **Type-safe document conversion** from observer callbacks
- **DQL support** for complex queries
- **CRDT optimization** for efficient network usage

### Integration Architecture

```
CoT XML → CotEvent → CotDocument → Ditto CRDT → P2P Network
    ↑                                              ↓
    └── Observer Callbacks ← DQL Queries ←────────┘
```

## SDK Integration Patterns

### 1. Document Storage Pattern

**Purpose**: Store CoT events as Ditto documents with proper collection routing

```rust
// Rust
use ditto_cot::ditto::{cot_to_document, CotDocument};
use dittolive_ditto::prelude::*;

async fn store_cot_event(ditto: &Ditto, cot_xml: &str, peer_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse and convert
    let event = CotEvent::from_xml(cot_xml)?;
    let doc = cot_to_document(&event, peer_id);
    
    // Route to appropriate collection
    let collection_name = match &doc {
        CotDocument::MapItem(_) => "map_items",
        CotDocument::Chat(_) => "chat_messages", 
        CotDocument::File(_) => "files",
        CotDocument::Api(_) => "api_events",
    };
    
    // Store with DQL
    let store = ditto.store();
    let doc_json = serde_json::to_value(&doc)?;
    let query = format!("INSERT INTO {} DOCUMENTS (:doc) ON ID CONFLICT DO MERGE", collection_name);
    let params = serde_json::json!({ "doc": doc_json });
    
    store.execute_v2((&query, params)).await?;
    Ok(())
}
```

```java
// Java
import com.ditto.cot.SdkDocumentConverter;

public void storeCotEvent(Ditto ditto, String cotXml, String peerId) {
    try {
        // Parse and convert
        CotEvent event = CotEvent.fromXml(cotXml);
        SdkDocumentConverter converter = new SdkDocumentConverter();
        Map<String, Object> docMap = converter.convertToDocumentMap(event, peerId);
        
        // Route to collection
        String collection = determineCollection(docMap);
        
        // Store via DQL
        String query = String.format("INSERT INTO %s DOCUMENTS (?) ON ID CONFLICT DO MERGE", collection);
        ditto.getStore().execute(query, docMap);
        
    } catch (Exception e) {
        logger.error("Failed to store CoT event", e);
    }
}
```

### 2. Observer Integration Pattern

**Purpose**: Convert observer documents to typed CoT objects for application use

```rust
// Rust Observer Pattern
use ditto_cot::ditto::sdk_conversion::{observer_json_to_cot_document, observer_json_to_json_with_r_fields};

async fn setup_cot_observers(ditto: &Ditto) -> Result<(), Box<dyn std::error::Error>> {
    let store = ditto.store();
    
    // Location updates observer
    let _subscription = store
        .collection("map_items")
        .find_all()
        .subscribe()
        .observe(|docs, _event| {
            for doc in docs {
                let boxed_doc = doc.value();
                
                // Convert observer document to typed CoT document
                match observer_json_to_cot_document(&boxed_doc) {
                    Ok(Some(CotDocument::MapItem(map_item))) => {
                        println!("Location update: {} at {},{}", 
                                map_item.e, 
                                map_item.j.unwrap_or(0.0), 
                                map_item.l.unwrap_or(0.0));
                        
                        // Handle location update
                        handle_location_update(&map_item);
                    },
                    Ok(Some(other)) => {
                        println!("Unexpected document type in map_items: {:?}", other);
                    },
                    Ok(None) => {
                        println!("Failed to convert observer document");
                    },
                    Err(e) => {
                        eprintln!("Conversion error: {}", e);
                    }
                }
            }
        })?;
    
    Ok(())
}

fn handle_location_update(map_item: &MapItem) {
    // Process location update
    if let Some(r_fields) = &map_item.r {
        // Access reconstructed detail hierarchy
        println!("Detail data: {:?}", r_fields);
    }
}
```

```java
// Java Observer Pattern
import com.ditto.cot.SdkDocumentConverter;
import com.ditto.cot.schema.*;

public void setupCotObservers(Ditto ditto) {
    SdkDocumentConverter converter = new SdkDocumentConverter();
    DittoStore store = ditto.getStore();
    
    // Chat messages observer
    store.registerObserver("SELECT * FROM chat_messages", (result, event) -> {
        for (DittoQueryResultItem item : result.getItems()) {
            Map<String, Object> docMap = item.getValue();
            
            // Convert to typed document
            Object typedDoc = converter.observerMapToTypedDocument(docMap);
            
            if (typedDoc instanceof ChatDocument) {
                ChatDocument chat = (ChatDocument) typedDoc;
                System.out.println("Chat from " + chat.getAuthorCallsign() + 
                                 ": " + chat.getMessage());
                
                // Handle chat message
                handleChatMessage(chat);
                
                // Get full JSON with r-fields
                String jsonWithRFields = converter.observerMapToJsonWithRFields(docMap);
                System.out.println("Full document: " + jsonWithRFields);
            }
        }
    });
}

private void handleChatMessage(ChatDocument chat) {
    // Process chat message
    String room = chat.getRoom();
    String message = chat.getMessage();
    // Update UI, send notifications, etc.
}
```

## Observer Document Conversion

### Understanding Observer Document Structure

Observer documents contain flattened `r_*` fields for DQL compatibility:

```json
{
  "_id": "location-001",
  "w": "a-u-r-loc-g",
  "j": 37.7749,
  "l": -122.4194,
  "r_contact_callsign": "Unit-Alpha",
  "r_track_speed": "15.0",
  "r_track_course": "90.0"
}
```

The conversion utilities reconstruct the hierarchical structure:

```json
{
  "_id": "location-001", 
  "w": "a-u-r-loc-g",
  "j": 37.7749,
  "l": -122.4194,
  "r": {
    "contact": {
      "callsign": "Unit-Alpha"
    },
    "track": {
      "speed": "15.0",
      "course": "90.0" 
    }
  }
}
```

### Conversion API Reference

#### Rust SDK Conversion

```rust
use ditto_cot::ditto::sdk_conversion::*;

// Convert observer document to typed CotDocument
let typed_doc = observer_json_to_cot_document(&boxed_doc)?;

// Reconstruct hierarchical JSON with r-fields
let json_with_r_fields = observer_json_to_json_with_r_fields(&boxed_doc)?;

// Extract document metadata
let doc_id = extract_document_id(&boxed_doc)?;
let doc_type = extract_document_type(&boxed_doc)?;
```

#### Java SDK Conversion

```java
SdkDocumentConverter converter = new SdkDocumentConverter();

// Convert to typed document
Object typedDoc = converter.observerMapToTypedDocument(docMap);

// Get JSON with reconstructed r-fields
String jsonWithRFields = converter.observerMapToJsonWithRFields(docMap);

// Extract metadata
String docId = converter.getDocumentId(docMap);
String docType = converter.getDocumentType(docMap);
```

## DQL Operations

### Query Patterns

#### Location-Based Queries

```rust
// Rust - Find nearby units
let query = "SELECT * FROM map_items WHERE 
    j BETWEEN ? AND ? AND 
    l BETWEEN ? AND ? AND 
    w LIKE 'a-f-%'";
    
let params = serde_json::json!([
    lat_min, lat_max,
    lon_min, lon_max
]);

let results = store.execute_v2((query, params)).await?;
```

```java
// Java - Find units by team
String query = "SELECT * FROM map_items WHERE r_group_name = ?";
DittoQueryResult results = store.execute(query, "Blue");
```

#### Chat and Communication Queries

```rust
// Recent chat messages in room
let query = "SELECT * FROM chat_messages 
    WHERE room_id = ? 
    ORDER BY b DESC 
    LIMIT 50";
```

#### File Sharing Queries

```java
// Files shared by specific user
String query = "SELECT * FROM files WHERE d = ? ORDER BY b DESC";
DittoQueryResult files = store.execute(query, authorUid);
```

### Collection Management

**Collection Naming Convention**:
- `map_items` - Location updates, map graphics
- `chat_messages` - Chat communications
- `files` - File sharing events
- `api_events` - Emergency/API events

**Document Routing Logic**:
```rust
// Use the built-in collection routing that distinguishes tracks from map items
let collection_name = doc.get_collection_name();

// Collections:
// - "track": PLI and location tracks (transient, with movement data)
// - "map_items": Map graphics and persistent items
// - "chat_messages": Chat and messaging
// - "files": File attachments
// - "api_events": API and emergency events
```

## Real-time Synchronization

### P2P Network Setup

```rust
// Rust P2P Configuration
use dittolive_ditto::prelude::*;

async fn setup_p2p_cot_sync() -> Result<Ditto, Box<dyn std::error::Error>> {
    let app_id = std::env::var("DITTO_APP_ID")?;
    let token = std::env::var("DITTO_PLAYGROUND_TOKEN")?;
    
    let ditto = Ditto::builder()
        .with_root(DittoRoot::from_current_exe()?)
        .with_identity(DittoIdentity::OnlinePlayground {
            app_id: app_id.clone(),
            token: token.clone(),
            enable_ditto_cloud_sync: true,
        })?
        .build()?;
    
    // Start sync
    ditto.start_sync()?;
    
    Ok(ditto)
}
```

```java
// Java P2P Configuration
DittoIdentity identity = new DittoIdentity.OnlinePlayground(
    appId, token, true
);

Ditto ditto = new Ditto(DittoRoot.fromCurrent(), identity);
ditto.startSync();
```

### Conflict Resolution

The CRDT optimization automatically handles conflicts:

```
Node A: Updates sensor_1.zoom = "20x"     // Field-level update
Node B: Updates sensor_1.type = "thermal" // Different field
Node C: Updates sensor_2.zoom = "10x"     // Different sensor

Result: All changes merge without conflicts
```

### Sync Efficiency

**Traditional Approach**: Full document sync
```json
// 2KB document syncs entirely for 1 field change
{"_id": "doc1", "field1": "old", "field2": "unchanged", ...}
{"_id": "doc1", "field1": "new", "field2": "unchanged", ...}
```

**CRDT-Optimized Approach**: Differential sync
```json
// Only changed field syncs (200 bytes)
{"field1": "new"}
```

## Authentication & Setup

### Environment Configuration

```bash
# Required environment variables
export DITTO_APP_ID="your-app-id"
export DITTO_PLAYGROUND_TOKEN="your-token"

# Optional configuration
export DITTO_LOG_LEVEL="info"
export DITTO_SYNC_TIMEOUT="30000"
```

### Credential Management

```rust
// Rust - Secure credential handling
use std::env;

fn get_ditto_credentials() -> Result<(String, String), Box<dyn std::error::Error>> {
    let app_id = env::var("DITTO_APP_ID")
        .map_err(|_| "DITTO_APP_ID environment variable not set")?;
    let token = env::var("DITTO_PLAYGROUND_TOKEN")
        .map_err(|_| "DITTO_PLAYGROUND_TOKEN environment variable not set")?;
    
    Ok((app_id, token))
}
```

```java
// Java - Configuration with fallbacks
public class DittoConfig {
    public static DittoIdentity createIdentity() {
        String appId = System.getenv("DITTO_APP_ID");
        String token = System.getenv("DITTO_PLAYGROUND_TOKEN");
        
        if (appId == null || token == null) {
            throw new IllegalStateException("Ditto credentials not configured");
        }
        
        return new DittoIdentity.OnlinePlayground(appId, token, true);
    }
}
```

## Advanced Patterns

### Multi-Collection Observers

```rust
// Monitor all CoT collections
async fn setup_comprehensive_monitoring(ditto: &Ditto) -> Result<(), Box<dyn std::error::Error>> {
    let collections = ["map_items", "chat_messages", "files", "api_events"];
    
    for collection in &collections {
        let store = ditto.store();
        let _sub = store
            .collection(collection)
            .find_all()
            .subscribe()
            .observe(move |docs, event| {
                println!("Collection {} updated: {} documents", collection, docs.len());
                for doc in docs {
                    process_document_update(collection, doc);
                }
            })?;
    }
    
    Ok(())
}
```

### Batch Operations

```java
// Java - Batch insert multiple CoT events
public void batchStoreCotEvents(Ditto ditto, List<String> cotXmlList, String peerId) {
    SdkDocumentConverter converter = new SdkDocumentConverter();
    Map<String, List<Map<String, Object>>> collectionGroups = new HashMap<>();
    
    // Group by collection
    for (String xml : cotXmlList) {
        try {
            CotEvent event = CotEvent.fromXml(xml);
            Map<String, Object> docMap = converter.convertToDocumentMap(event, peerId);
            String collection = determineCollection(docMap);
            
            collectionGroups.computeIfAbsent(collection, k -> new ArrayList<>()).add(docMap);
        } catch (Exception e) {
            logger.warn("Failed to parse CoT XML", e);
        }
    }
    
    // Batch insert by collection
    for (Map.Entry<String, List<Map<String, Object>>> entry : collectionGroups.entrySet()) {
        String collection = entry.getKey();
        List<Map<String, Object>> docs = entry.getValue();
        
        String query = String.format("INSERT INTO %s DOCUMENTS (?) ON ID CONFLICT DO MERGE", collection);
        for (Map<String, Object> doc : docs) {
            ditto.getStore().execute(query, doc);
        }
    }
}
```

### Performance Optimization

```rust
// Connection pooling and caching
struct CotSyncManager {
    ditto: Arc<Ditto>,
    converter_cache: Arc<Mutex<HashMap<String, CotDocument>>>,
}

impl CotSyncManager {
    async fn store_with_cache(&self, cot_xml: &str, peer_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Check cache first
        let cache_key = format!("{}-{}", peer_id, calculate_hash(cot_xml));
        
        if let Ok(cache) = self.converter_cache.lock() {
            if cache.contains_key(&cache_key) {
                return Ok(()); // Already processed
            }
        }
        
        // Process and cache
        let event = CotEvent::from_xml(cot_xml)?;
        let doc = cot_to_document(&event, peer_id);
        
        // Store in Ditto
        self.store_document(&doc).await?;
        
        // Update cache
        if let Ok(mut cache) = self.converter_cache.lock() {
            cache.insert(cache_key, doc);
        }
        
        Ok(())
    }
}
```

## Troubleshooting

### Common Integration Issues

**DQL Unsupported Error**:
```rust
// Solution: Check SDK version and sync configuration
if let Err(e) = store.execute_v2((query, params)).await {
    if e.to_string().contains("DqlUnsupported") {
        eprintln!("DQL mutations require proper SDK configuration");
        eprintln!("Ensure sync is enabled and SDK version supports DQL mutations");
    }
}
```

**Observer Document Conversion Failures**:
```java
// Solution: Validate document structure
Object typedDoc = converter.observerMapToTypedDocument(docMap);
if (typedDoc == null) {
    String docType = converter.getDocumentType(docMap);
    logger.warn("Failed to convert document of type: {}", docType);
    
    // Fallback to raw map processing
    processRawDocument(docMap);
}
```

**Network Connectivity Issues**:
```rust
// Solution: Implement retry logic with exponential backoff
async fn store_with_retry(ditto: &Ditto, doc: &CotDocument, max_retries: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut retry_count = 0;
    let mut delay = Duration::from_millis(100);
    
    loop {
        match store_document(ditto, doc).await {
            Ok(_) => return Ok(()),
            Err(e) if retry_count < max_retries => {
                eprintln!("Store failed (attempt {}): {}", retry_count + 1, e);
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
                retry_count += 1;
            },
            Err(e) => return Err(e),
        }
    }
}
```

### Performance Monitoring

```rust
// Monitor sync performance
struct SyncMetrics {
    documents_synced: AtomicU64,
    bytes_transferred: AtomicU64,
    sync_errors: AtomicU64,
}

impl SyncMetrics {
    fn log_performance(&self) {
        let docs = self.documents_synced.load(Ordering::Relaxed);
        let bytes = self.bytes_transferred.load(Ordering::Relaxed);
        let errors = self.sync_errors.load(Ordering::Relaxed);
        
        println!("Sync Stats - Docs: {}, Bytes: {}, Errors: {}", docs, bytes, errors);
    }
}
```

This comprehensive guide covers the essential patterns for integrating the Ditto CoT library with the Ditto SDK. For language-specific implementation details, see the individual integration guides for [Rust](examples/rust.md) and [Java](examples/java.md).

## See Also

- **[Rust Integration Examples](examples/rust.md)** - Rust-specific patterns and async handling
- **[Java Integration Examples](examples/java.md)** - Java/Android integration with Spring Boot
- **[API Reference](../reference/api-reference.md)** - Complete API documentation for SDK integration
- **[Schema Reference](../reference/schema.md)** - Document schemas and CRDT optimization details
- **[Troubleshooting](../reference/troubleshooting.md)** - Common integration issues and solutions
- **[Migration Guide](migration.md)** - Upgrading from legacy CoT implementations
- **[Architecture](../technical/architecture.md)** - Understanding the system design
- **[Performance](../technical/performance.md)** - Optimization techniques and benchmarks