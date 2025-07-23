# Migration Guide

Guide for migrating between versions of the Ditto CoT library and upgrading from legacy CoT implementations.

## Table of Contents

- [Version Migration](#version-migration)
- [Legacy System Migration](#legacy-system-migration)
- [Breaking Changes](#breaking-changes)
- [Migration Tools](#migration-tools)
- [Best Practices](#best-practices)

## Version Migration

### Schema Version 1 to Version 2

Version 2 introduces significant improvements in CRDT optimization and document structure.

#### Major Changes

1. **Detail Storage Format**
   - **V1**: Ditto map/object structure
   - **V2**: XML string representation with CRDT optimization

2. **Field Names**
   - **V1**: Full descriptive names
   - **V2**: Shortened single-character names

3. **Common Properties**
   - **V1**: Mixed naming convention
   - **V2**: Underscore-prefixed system properties

4. **Counter Addition**
   - **V1**: No update tracking
   - **V2**: `_c` field tracks document updates

#### Migration Process

##### Automated Migration

**Rust Migration Tool:**
```rust
use ditto_cot::migration::{migrate_v1_to_v2, MigrationResult};

async fn migrate_documents(ditto: &Ditto) -> Result<MigrationResult, Box<dyn std::error::Error>> {
    let collections = ["map_items", "chat_messages", "files", "api_events"];
    let mut total_migrated = 0;
    let mut total_errors = 0;
    
    for collection in &collections {
        println!("Migrating collection: {}", collection);
        
        // Query V1 documents
        let query = format!("SELECT * FROM {} WHERE _v = 1 OR _v IS NULL", collection);
        let results = ditto.store().execute_v2((&query, serde_json::json!({}))).await?;
        
        for item in results.items() {
            let v1_doc = item.json_value();
            
            match migrate_v1_to_v2(&v1_doc) {
                Ok(v2_doc) => {
                    // Store migrated document
                    let update_query = format!("UPDATE {} SET DOCUMENTS (:doc) WHERE _id = :id", collection);
                    let params = serde_json::json!({
                        "doc": v2_doc,
                        "id": v1_doc.get("_id").unwrap()
                    });
                    
                    ditto.store().execute_v2((&update_query, params)).await?;
                    total_migrated += 1;
                    
                    println!("Migrated document: {}", v1_doc.get("_id").unwrap());
                },
                Err(e) => {
                    eprintln!("Migration failed for {}: {}", 
                             v1_doc.get("_id").unwrap_or(&serde_json::Value::String("unknown".to_string())), e);
                    total_errors += 1;
                }
            }
        }
    }
    
    Ok(MigrationResult {
        migrated: total_migrated,
        errors: total_errors,
    })
}
```

**Java Migration Tool:**
```java
import com.ditto.cot.migration.DocumentMigrator;
import com.ditto.cot.migration.MigrationResult;

public class V1ToV2Migrator {
    private final Ditto ditto;
    private final DocumentMigrator migrator;
    
    public V1ToV2Migrator(Ditto ditto) {
        this.ditto = ditto;
        this.migrator = new DocumentMigrator();
    }
    
    public MigrationResult migrateAllDocuments() {
        String[] collections = {"map_items", "chat_messages", "files", "api_events"};
        int totalMigrated = 0;
        int totalErrors = 0;
        
        for (String collection : collections) {
            System.out.println("Migrating collection: " + collection);
            
            try {
                // Query V1 documents
                String query = String.format("SELECT * FROM %s WHERE _v = 1 OR _v IS NULL", collection);
                DittoQueryResult results = ditto.getStore().execute(query);
                
                for (DittoQueryResultItem item : results.getItems()) {
                    Map<String, Object> v1Doc = item.getValue();
                    
                    try {
                        Map<String, Object> v2Doc = migrator.migrateV1ToV2(v1Doc);
                        
                        // Store migrated document
                        String updateQuery = String.format("UPDATE %s SET DOCUMENTS (?) WHERE _id = ?", collection);
                        ditto.getStore().execute(updateQuery, v2Doc, v1Doc.get("_id"));
                        
                        totalMigrated++;
                        System.out.println("Migrated document: " + v1Doc.get("_id"));
                        
                    } catch (Exception e) {
                        System.err.println("Migration failed for " + v1Doc.get("_id") + ": " + e.getMessage());
                        totalErrors++;
                    }
                }
                
            } catch (Exception e) {
                System.err.println("Failed to query collection " + collection + ": " + e.getMessage());
                totalErrors++;
            }
        }
        
        return new MigrationResult(totalMigrated, totalErrors);
    }
}
```

##### Manual Migration Steps

1. **Backup Existing Data**
```bash
# Export existing documents
ditto-cli export --collection map_items --output backup_v1_map_items.json
ditto-cli export --collection chat_messages --output backup_v1_chat.json
```

2. **Update Library Version**
```toml
# Rust Cargo.toml
[dependencies]
ditto_cot = { git = "https://github.com/getditto-shared/ditto_cot", tag = "v2.0.0" }
```

```xml
<!-- Java pom.xml -->
<dependency>
  <groupId>com.ditto</groupId>
  <artifactId>ditto-cot</artifactId>
  <version>2.0.0</version>
</dependency>
```

3. **Run Migration Tool**
```bash
# Rust
cargo run --bin migrate_v1_to_v2

# Java
java -jar ditto-cot-migration-tool.jar --source-version 1 --target-version 2
```

#### Field Migration Mapping

| V1 Field | V2 Field | Type | Notes |
|----------|----------|------|-------|
| `version` | `_v` | Integer | Now required, set to 2 |
| `counter` | `_c` | Counter | New field, starts at 0 |
| `id` | `_id` | String | No change |
| `removed` | `_r` | Boolean | Renamed with underscore |
| `peer_key` | `a` | String | Shortened name |
| `timestamp` | `b` | Number | Shortened name |
| `author_uid` | `d` | String | Shortened name |
| `callsign` | `e` | String | Shortened name |
| `detail` | `r` | Object/String | Format changed |

#### Detail Section Migration

**V1 Detail Structure:**
```json
{
  "detail": {
    "contact": {
      "callsign": "ALPHA-1"
    },
    "group": {
      "name": "Blue",
      "role": "Team Leader"
    }
  }
}
```

**V2 Detail Structure:**
```json
{
  "r": "<detail><contact callsign=\"ALPHA-1\"/><__group name=\"Blue\" role=\"Team Leader\"/></detail>"
}
```

## Legacy System Migration

### From TAK/ATAK Integration

#### Common Legacy Patterns

**Legacy Direct XML Processing:**
```java
// Old approach - direct XML parsing
Document doc = DocumentBuilderFactory.newInstance()
    .newDocumentBuilder()
    .parse(new InputSource(new StringReader(cotXml)));

Element event = doc.getDocumentElement();
String uid = event.getAttribute("uid");
String type = event.getAttribute("type");
```

**New Ditto CoT Approach:**
```java
// New approach - structured object model
CotEvent event = CotEvent.fromXml(cotXml);
String uid = event.getUid();
String type = event.getType();

// Convert to Ditto document for sync
SdkDocumentConverter converter = new SdkDocumentConverter();
Map<String, Object> doc = converter.convertToDocumentMap(event, peerId);
```

#### Migration Strategy

1. **Identify Integration Points**
   - XML parsing code
   - Document storage/retrieval
   - Real-time updates
   - Network synchronization

2. **Replace XML Processing**
```java
// Before: Manual XML parsing
public class LegacyCotProcessor {
    public void processXml(String xml) {
        // Complex XML parsing logic
        DocumentBuilder builder = DocumentBuilderFactory.newInstance().newDocumentBuilder();
        Document doc = builder.parse(new InputSource(new StringReader(xml)));
        
        // Extract fields manually
        Element event = doc.getDocumentElement();
        String uid = event.getAttribute("uid");
        // ... more manual extraction
    }
}

// After: Structured object model
public class ModernCotProcessor {
    private final SdkDocumentConverter converter = new SdkDocumentConverter();
    
    public void processXml(String xml) {
        try {
            CotEvent event = CotEvent.fromXml(xml);
            
            // Type-safe field access
            String uid = event.getUid();
            String type = event.getType();
            Point location = event.getPoint();
            
            // Convert to Ditto document
            Map<String, Object> doc = converter.convertToDocumentMap(event, peerId);
            
            // Store in Ditto for sync
            ditto.getStore().execute("INSERT INTO map_items DOCUMENTS (?)", doc);
            
        } catch (CotEventException e) {
            logger.error("Failed to process CoT event", e);
        }
    }
}
```

3. **Replace Storage Layer**
```java
// Before: File or database storage
public class LegacyStorage {
    public void storeCotEvent(String xml) {
        // Store in local database or file
        database.execute("INSERT INTO cot_events (xml, timestamp) VALUES (?, ?)", xml, System.currentTimeMillis());
    }
    
    public List<String> getCotEvents() {
        // Retrieve from local storage
        return database.query("SELECT xml FROM cot_events ORDER BY timestamp DESC");
    }
}

// After: Ditto CRDT storage
public class DittoStorage {
    private final Ditto ditto;
    private final SdkDocumentConverter converter;
    
    public void storeCotEvent(String xml) {
        CotEvent event = CotEvent.fromXml(xml);
        Map<String, Object> doc = converter.convertToDocumentMap(event, peerId);
        
        String collection = determineCollection(doc);
        ditto.getStore().execute(
            String.format("INSERT INTO %s DOCUMENTS (?) ON ID CONFLICT DO MERGE", collection), 
            doc
        );
    }
    
    public void observeCotEvents(Consumer<CotEvent> handler) {
        // Real-time updates via observers
        ditto.getStore().registerObserver("SELECT * FROM map_items", (result, event) -> {
            for (DittoQueryResultItem item : result.getItems()) {
                Object typedDoc = converter.observerMapToTypedDocument(item.getValue());
                if (typedDoc instanceof MapItemDocument) {
                    // Process typed document
                    handler.accept(convertToEvent((MapItemDocument) typedDoc));
                }
            }
        });
    }
}
```

### From Custom CoT Libraries

#### Migration Checklist

- [ ] **Inventory Current Code**
  - XML parsing logic
  - Document models
  - Storage mechanisms
  - Network protocols

- [ ] **Plan Replacement Strategy**
  - Identify Ditto CoT equivalents
  - Map custom fields to standard schema
  - Plan data migration approach

- [ ] **Implement Gradual Migration**
  - Start with new features
  - Replace components incrementally
  - Maintain backward compatibility during transition

- [ ] **Validate Migration**
  - Test with existing data
  - Verify XML round-trip compatibility
  - Performance testing

#### Custom Field Migration

**Legacy Custom Fields:**
```xml
<detail>
  <custom_sensor id="123" type="radar" range="5000"/>
  <proprietary_data value="secret123"/>
</detail>
```

**Migration to Standard Format:**
```xml
<detail>
  <sensor id="123" type="radar">
    <range>5000</range>
  </sensor>
  <custom proprietary_data="secret123"/>
</detail>
```

**Code Migration:**
```rust
// Before: Custom parsing
struct CustomSensor {
    id: String,
    sensor_type: String,
    range: f64,
    proprietary_data: String,
}

fn parse_custom_detail(xml: &str) -> CustomSensor {
    // Custom XML parsing logic
}

// After: Standard schema with extensions
let event = CotEvent::builder()
    .uid("SENSOR-123")
    .event_type("a-f-G-U-C")
    .detail(r#"<detail>
        <sensor id="123" type="radar">
            <range>5000</range>
        </sensor>
        <custom proprietary_data="secret123"/>
    </detail>"#)
    .build();

let doc = cot_to_document(&event, peer_id);
```

## Breaking Changes

### Version 2.0 Breaking Changes

1. **Schema Version Required**
   - All documents must have `_v: 2`
   - V1 documents require migration

2. **Detail Format Change**
   - V1: Object/Map structure
   - V2: XML string with CRDT keys

3. **Field Name Changes**
   - Many fields shortened to single characters
   - System fields prefixed with underscore

4. **API Changes**
   - Some method signatures updated
   - Error types restructured

### Handling Breaking Changes

#### Compatibility Layer

```rust
// Provide compatibility for V1 APIs
pub mod v1_compat {
    use super::*;
    
    #[deprecated(note = "Use CotEvent::builder() instead")]
    pub fn create_location_event(uid: &str, lat: f64, lon: f64) -> CotEvent {
        CotEvent::builder()
            .uid(uid)
            .event_type("a-f-G-U-C")
            .location(lat, lon, 0.0)
            .build()
    }
    
    #[deprecated(note = "Use cot_to_document() instead")]
    pub fn convert_to_ditto_v1(event: &CotEvent) -> serde_json::Value {
        let doc = cot_to_document(event, "legacy-peer");
        serde_json::to_value(doc).unwrap()
    }
}
```

#### Gradual Migration

```java
public class MigrationHelper {
    private final boolean useV2Format;
    
    public MigrationHelper(boolean useV2Format) {
        this.useV2Format = useV2Format;
    }
    
    public Map<String, Object> convertDocument(CotEvent event, String peerId) {
        if (useV2Format) {
            // Use new V2 converter
            SdkDocumentConverter converter = new SdkDocumentConverter();
            return converter.convertToDocumentMap(event, peerId);
        } else {
            // Use legacy V1 converter
            return convertToV1Format(event, peerId);
        }
    }
    
    @Deprecated
    private Map<String, Object> convertToV1Format(CotEvent event, String peerId) {
        // Legacy conversion logic
        Map<String, Object> doc = new HashMap<>();
        doc.put("id", event.getUid());
        doc.put("version", 1);
        // ... other V1 fields
        return doc;
    }
}
```

## Migration Tools

### Command Line Tools

#### Rust Migration CLI

```bash
# Install migration tool
cargo install ditto-cot-migration

# Migrate documents
ditto-cot-migration \
  --app-id $DITTO_APP_ID \
  --token $DITTO_PLAYGROUND_TOKEN \
  --from-version 1 \
  --to-version 2 \
  --collections map_items,chat_messages \
  --dry-run

# Apply migration
ditto-cot-migration \
  --app-id $DITTO_APP_ID \
  --token $DITTO_PLAYGROUND_TOKEN \
  --from-version 1 \
  --to-version 2 \
  --collections map_items,chat_messages
```

#### Java Migration Tool

```bash
# Run migration JAR
java -jar ditto-cot-migration-tool.jar \
  --app-id $DITTO_APP_ID \
  --token $DITTO_PLAYGROUND_TOKEN \
  --source-version 1 \
  --target-version 2 \
  --collections map_items,chat_messages \
  --batch-size 100
```

### Migration Scripts

#### Data Export/Import

```bash
#!/bin/bash
# backup_and_migrate.sh

set -e

COLLECTIONS=("map_items" "chat_messages" "files" "api_events")
BACKUP_DIR="backup_$(date +%Y%m%d_%H%M%S)"

echo "Creating backup directory: $BACKUP_DIR"
mkdir -p "$BACKUP_DIR"

# Backup existing data
for collection in "${COLLECTIONS[@]}"; do
    echo "Backing up $collection..."
    ditto-cli export \
        --collection "$collection" \
        --output "$BACKUP_DIR/${collection}_v1.json"
done

# Run migration
echo "Running migration..."
ditto-cot-migration \
    --app-id "$DITTO_APP_ID" \
    --token "$DITTO_PLAYGROUND_TOKEN" \
    --from-version 1 \
    --to-version 2 \
    --collections "$(IFS=,; echo "${COLLECTIONS[*]}")"

# Verify migration
echo "Verifying migration..."
for collection in "${COLLECTIONS[@]}"; do
    count=$(ditto-cli query "SELECT COUNT(*) as count FROM $collection WHERE _v = 2" | jq -r '.items[0].count')
    echo "$collection: $count documents migrated to V2"
done

echo "Migration complete. Backup saved in $BACKUP_DIR"
```

### Validation Tools

#### Schema Compliance Checker

```rust
use ditto_cot::validation::validate_schema_compliance;

async fn check_migration_compliance(ditto: &Ditto) -> Result<(), Box<dyn std::error::Error>> {
    let collections = ["map_items", "chat_messages", "files", "api_events"];
    
    for collection in &collections {
        println!("Checking schema compliance for: {}", collection);
        
        let query = format!("SELECT * FROM {}", collection);
        let results = ditto.store().execute_v2((&query, serde_json::json!({}))).await?;
        
        let mut compliant = 0;
        let mut non_compliant = 0;
        
        for item in results.items() {
            let doc = item.json_value();
            
            match validate_schema_compliance(&doc) {
                Ok(_) => compliant += 1,
                Err(e) => {
                    non_compliant += 1;
                    eprintln!("Non-compliant document {}: {}", 
                             doc.get("_id").unwrap_or(&serde_json::Value::String("unknown".to_string())), e);
                }
            }
        }
        
        println!("Collection {}: {} compliant, {} non-compliant", collection, compliant, non_compliant);
    }
    
    Ok(())
}
```

## Best Practices

### Migration Planning

1. **Test Migration in Development**
   - Use test data sets
   - Validate all use cases
   - Performance test with realistic data volumes

2. **Gradual Rollout**
   - Start with non-critical systems
   - Monitor for issues
   - Have rollback plan ready

3. **Data Validation**
   - Verify data integrity post-migration
   - Check all document types
   - Validate relationships between documents

### Monitoring Migration

```rust
#[derive(Debug)]
pub struct MigrationMetrics {
    pub total_documents: usize,
    pub migrated_successfully: usize,
    pub migration_errors: usize,
    pub validation_errors: usize,
    pub processing_time: Duration,
}

pub async fn monitor_migration(ditto: &Ditto) -> Result<MigrationMetrics, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let mut metrics = MigrationMetrics::default();
    
    // Track migration progress
    let collections = ["map_items", "chat_messages", "files", "api_events"];
    
    for collection in &collections {
        let total_query = format!("SELECT COUNT(*) as count FROM {}", collection);
        let v2_query = format!("SELECT COUNT(*) as count FROM {} WHERE _v = 2", collection);
        
        let total_result = ditto.store().execute_v2((&total_query, serde_json::json!({}))).await?;
        let v2_result = ditto.store().execute_v2((&v2_query, serde_json::json!({}))).await?;
        
        if let (Some(total_item), Some(v2_item)) = (total_result.items().next(), v2_result.items().next()) {
            let total: usize = total_item.json_value()["count"].as_u64().unwrap_or(0) as usize;
            let migrated: usize = v2_item.json_value()["count"].as_u64().unwrap_or(0) as usize;
            
            metrics.total_documents += total;
            metrics.migrated_successfully += migrated;
            
            println!("Collection {}: {}/{} migrated", collection, migrated, total);
        }
    }
    
    metrics.processing_time = start_time.elapsed();
    metrics.migration_errors = metrics.total_documents - metrics.migrated_successfully;
    
    Ok(metrics)
}
```

### Rollback Procedures

```bash
#!/bin/bash
# rollback_migration.sh

BACKUP_DIR="$1"
COLLECTIONS=("map_items" "chat_messages" "files" "api_events")

if [ -z "$BACKUP_DIR" ]; then
    echo "Usage: $0 <backup_directory>"
    exit 1
fi

echo "Rolling back migration using backup from: $BACKUP_DIR"

for collection in "${COLLECTIONS[@]}"; do
    backup_file="$BACKUP_DIR/${collection}_v1.json"
    
    if [ -f "$backup_file" ]; then
        echo "Restoring $collection from $backup_file..."
        
        # Clear current collection
        ditto-cli execute "DELETE FROM $collection"
        
        # Restore from backup
        ditto-cli import \
            --collection "$collection" \
            --input "$backup_file"
        
        echo "$collection restored"
    else
        echo "Warning: Backup file not found for $collection"
    fi
done

echo "Rollback complete"
```

This migration guide provides comprehensive strategies for upgrading between versions and migrating from legacy systems to the Ditto CoT library while maintaining data integrity and system functionality.