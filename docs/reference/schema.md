# Schema Reference

Complete reference for the Ditto CoT library schemas, including JSON schemas, document structures, and field mappings.

## Table of Contents

- [Schema Overview](#schema-overview)
- [Ditto Document Schema](#ditto-document-schema)
- [CoT Event Schema](#cot-event-schema)
- [Document Types](#document-types)
- [Field Mappings](#field-mappings)
- [CRDT Optimization](#crdt-optimization)
- [Schema Validation](#schema-validation)

## Schema Overview

The Ditto CoT library uses two primary schemas:

1. **JSON Schema** (`schema/ditto.schema.json`) - Defines Ditto document structure
2. **XML Schema** (`schema/cot_event.xsd`) - Defines CoT XML event structure

### Schema Version

- **Current Version**: 2
- **Backward Compatibility**: Version 1 (deprecated)
- **Schema Evolution**: Managed through version fields

## Ditto Document Schema

### Common Properties

All Ditto documents share these common properties:

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `_id` | String | Unique document identifier | `"ANDROID-123"` |
| `_c` | Counter | Document update counter | `14365` |
| `_v` | Integer | Schema version | `2` |
| `_r` | Boolean | Soft-delete flag | `false` |
| `a` | String | Ditto peer key | `"pkAocCgkMDQ1..."` |
| `b` | Double | Timestamp (millis since epoch) | `1748370358459` |
| `d` | String | Author UID | `"ANDROID-123"` |
| `e` | String | Author callsign | `"GATOR"` |

### CoT-Specific Properties

Additional properties for CoT events:

| Field | Type | Description | Default | Example |
|-------|------|-------------|---------|---------|
| `g` | String | CoT version | `""` | `"2.0"` |
| `h` | Double | Circular error (CE) in meters | `0.0` | `27.5` |
| `i` | Double | Height above ellipsoid (HAE) | `0.0` | `-30.741` |
| `j` | Double | Latitude | `0.0` | `27.020123` |
| `k` | Double | Linear error (LE) in meters | `0.0` | `9999999` |
| `l` | Double | Longitude | `0.0` | `-81.261311` |
| `n` | Long | Start time (millis) | `0` | `1748370358459` |
| `o` | Long | Stale time (millis) | `0` | `1748370433459` |
| `p` | String | How generated | `""` | `"m-g"` |
| `q` | String | Access control | `""` | `"Undefined"` |
| `r` | Object/String | Detail section | `{}` | See [Detail Schema](#detail-schema) |
| `s` | String | Operational exercise | `""` | `""` |
| `t` | String | Quality of service | `""` | `""` |
| `u` | String | Caveat | `""` | `""` |
| `v` | String | Releasable to | `""` | `""` |
| `w` | String | CoT event type | `""` | `"a-f-G-U-C"` |

## CoT Event Schema

### Root Element: `<event>`

```xml
<event version="2.0" 
       uid="ANDROID-123" 
       type="a-f-G-U-C" 
       time="2021-02-27T20:32:24.913Z" 
       start="2021-02-27T20:32:24.913Z" 
       stale="2021-02-27T20:38:39.913Z" 
       how="h-g-i-g-o">
    <point lat="1.234567" lon="3.456789" hae="9999999.0" ce="9999999.0" le="9999999.0"/>
    <detail>
        <!-- Detail content -->
    </detail>
</event>
```

#### Event Attributes

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `version` | String | Yes | CoT schema version |
| `uid` | String | Yes | Unique identifier |
| `type` | String | Yes | Event type hierarchy |
| `time` | DateTime | Yes | Event timestamp |
| `start` | DateTime | Yes | Event start time |
| `stale` | DateTime | Yes | Event expiration |
| `how` | String | No | Generation method |
| `access` | String | No | Access control |
| `opex` | String | No | Operational exercise |
| `qos` | String | No | Quality of service |
| `caveat` | String | No | Caveat information |
| `releaseableTo` | String | No | Release authorization |

### Point Element

```xml
<point lat="34.052235" 
       lon="-118.243683" 
       hae="100.0" 
       ce="5.0" 
       le="10.0"/>
```

#### Point Attributes

| Attribute | Type | Required | Description | Units |
|-----------|------|----------|-------------|-------|
| `lat` | Double | Yes | Latitude | Degrees |
| `lon` | Double | Yes | Longitude | Degrees |
| `hae` | Double | No | Height above ellipsoid | Meters |
| `ce` | Double | No | Circular error | Meters |
| `le` | Double | No | Linear error | Meters |

### Detail Element

The detail element contains tactical information specific to the event type.

```xml
<detail>
    <contact callsign="ALPHA-1"/>
    <__group name="Blue" role="Team Leader"/>
    <status readiness="true"/>
    <track speed="15.0" course="90.0"/>
    <takv device="Android" platform="ATAK" version="4.8.1"/>
</detail>
```

## Document Types

### MapItem Document

Used for location updates and map graphics.

#### Schema Structure

```json
{
  "type": "object",
  "properties": {
    "_id": {"type": "string"},
    "_c": {"type": "integer"},
    "_v": {"type": "integer"},
    "_r": {"type": "boolean"},
    "a": {"type": "string"},
    "b": {"type": "number"},
    "c": {"type": "string"},
    "d": {"type": "string"},
    "e": {"type": "string"},
    "f": {"type": "boolean"},
    "g": {"type": "string"},
    "h": {"type": "number"},
    "i": {"type": "number"},
    "j": {"type": "number"},
    "k": {"type": "number"},
    "l": {"type": "number"},
    "n": {"type": "integer"},
    "o": {"type": "integer"},
    "p": {"type": "string"},
    "q": {"type": "string"},
    "r": {"type": "object"},
    "s": {"type": "string"},
    "t": {"type": "string"},
    "u": {"type": "string"},
    "v": {"type": "string"},
    "w": {"type": "string"}
  },
  "required": ["_id", "e", "w"]
}
```

#### Example Document

```json
{
  "_c": 14365,
  "_id": "ANDROID-6d2198a6271bca69",
  "_r": false,
  "_v": 2,
  "a": "pkAocCgkMDQ1_BWQXXkjEah7pV_2rvS4TTwwkJ6qeUpBPRYrAlphs",
  "b": 1748370358459,
  "c": "GATOR",
  "d": "ANDROID-6d2198a6271bca69",
  "e": "GATOR",
  "f": true,
  "g": "2.0",
  "h": 27.5,
  "i": -30.741204952759624,
  "j": 27.020123,
  "k": 9999999,
  "l": -81.261311,
  "n": 1748370358459,
  "o": 1748370433459,
  "p": "m-g",
  "q": "Undefined",
  "r": {
    "takv": {
      "os": "34",
      "version": "4.10.0.57",
      "device": "GOOGLE PIXEL 8A",
      "platform": "ATAK-CIV"
    },
    "contact": {
      "endpoint": "192.168.1.116:4242:tcp",
      "callsign": "GATOR"
    },
    "group": {
      "role": "Team Member",
      "name": "Cyan"
    }
  },
  "w": "a-f-G-U-C"
}
```

### Chat Document

Used for chat messages and communications.

#### Schema Structure

```json
{
  "type": "object",
  "properties": {
    "_id": {"type": "string"},
    "_c": {"type": "integer"},
    "_v": {"type": "integer"},
    "_r": {"type": "boolean"},
    "message": {"type": "string"},
    "room": {"type": "string"},
    "roomId": {"type": "string"},
    "parent": {"type": "string"},
    "authorCallsign": {"type": "string"},
    "authorUid": {"type": "string"},
    "authorType": {"type": "string"},
    "time": {"type": "string"},
    "location": {"type": "string"}
  },
  "required": ["_id", "message", "authorCallsign"]
}
```

#### Example Document

```json
{
  "_id": "chat-message-001",
  "_c": 0,
  "_v": 2,
  "_r": false,
  "message": "Moving to checkpoint Alpha",
  "room": "Command Net",
  "roomId": "cmd-net-001",
  "authorCallsign": "ALPHA-1",
  "authorUid": "ANDROID-123",
  "authorType": "user",
  "time": "2024-01-15T10:30:00.000Z",
  "location": "34.0522,-118.2437,100"
}
```

### File Document

Used for file sharing and attachments.

#### Schema Structure

```json
{
  "type": "object",
  "properties": {
    "_id": {"type": "string"},
    "_c": {"type": "integer"},
    "_v": {"type": "integer"},
    "_r": {"type": "boolean"},
    "file": {"type": "string"},
    "sz": {"type": "number"},
    "mime": {"type": "string"},
    "contentType": {"type": "string"},
    "itemId": {"type": "string"}
  },
  "required": ["_id", "file"]
}
```

#### Example Document

```json
{
  "_id": "file-share-001",
  "_c": 0,
  "_v": 2,
  "_r": false,
  "file": "tactical-map.png",
  "sz": 1048576,
  "mime": "image/png",
  "contentType": "image/png",
  "itemId": "map-item-001"
}
```

### Api Document

Used for API events and emergency situations.

#### Schema Structure

```json
{
  "type": "object",
  "properties": {
    "_id": {"type": "string"},
    "_c": {"type": "integer"},
    "_v": {"type": "integer"},
    "_r": {"type": "boolean"},
    "e": {"type": "string"},
    "emergencyType": {"type": "string"},
    "priority": {"type": "string"},
    "status": {"type": "string"}
  },
  "required": ["_id", "e"]
}
```

## Field Mappings

### CoT XML to Ditto Document

| CoT XML | Ditto Field | Type | Description |
|---------|-------------|------|-------------|
| `@uid` | `_id` | String | Document identifier |
| `@type` | `w` | String | Event type |
| `@time` | `b` | Number | Timestamp (converted to millis) |
| `@start` | `n` | Number | Start time (converted to millis) |
| `@stale` | `o` | Number | Stale time (converted to millis) |
| `@how` | `p` | String | Generation method |
| `@version` | `g` | String | CoT version |
| `point@lat` | `j` | Number | Latitude |
| `point@lon` | `l` | Number | Longitude |
| `point@hae` | `i` | Number | Height above ellipsoid |
| `point@ce` | `h` | Number | Circular error |
| `point@le` | `k` | Number | Linear error |
| `detail/*` | `r` | Object | Detail elements |

### Underscore Field Mapping

The library handles underscore-prefixed fields specially:

| JSON Field | Rust Field | Description |
|------------|------------|-------------|
| `_id` | `id` | Document ID |
| `_c` | `d_c` | Counter |
| `_v` | `d_v` | Version |
| `_r` | `d_r` | Removed flag |

### R-Field Flattening

For DQL compatibility, detail fields are flattened with `r_` prefix:

**Hierarchical Structure:**
```json
{
  "r": {
    "contact": {
      "callsign": "ALPHA-1"
    },
    "track": {
      "speed": "15.0",
      "course": "90.0"
    }
  }
}
```

**Flattened for DQL:**
```json
{
  "r_contact_callsign": "ALPHA-1",
  "r_track_speed": "15.0",
  "r_track_course": "90.0"
}
```

## CRDT Optimization

### Stable Key Generation

The library uses CRDT-optimized stable keys for duplicate elements:

#### Key Format

```
Format: base64(hash(documentId + elementName))_index
Example: "aG1k_0", "aG1k_1", "aG1k_2"
```

#### Example: Duplicate Sensors

**Original XML:**
```xml
<detail>
  <sensor type="optical" id="sensor-1"/>
  <sensor type="thermal" id="sensor-2"/>
  <sensor type="radar" id="sensor-3"/>
</detail>
```

**CRDT-Optimized Storage:**
```json
{
  "r": {
    "aG1k_0": {
      "type": "optical",
      "id": "sensor-1",
      "_tag": "sensor"
    },
    "aG1k_1": {
      "type": "thermal", 
      "id": "sensor-2",
      "_tag": "sensor"
    },
    "aG1k_2": {
      "type": "radar",
      "id": "sensor-3", 
      "_tag": "sensor"
    }
  }
}
```

### Benefits

- **100% Data Preservation**: All duplicate elements maintained
- **Differential Updates**: Only changed fields sync
- **Cross-Language Compatibility**: Identical key generation
- **Bandwidth Efficiency**: ~74% reduction in key size

## Schema Validation

### JSON Schema Validation

The library provides validation against the JSON schema:

#### Rust

```rust
use ditto_cot::schema::validate_document;

let document = /* ... */;
match validate_document(&document) {
    Ok(_) => println!("Document is valid"),
    Err(e) => eprintln!("Validation error: {}", e),
}
```

#### Java

```java
import com.ditto.cot.schema.DocumentValidator;

DocumentValidator validator = new DocumentValidator();
boolean isValid = validator.validate(documentMap);
if (!isValid) {
    List<String> errors = validator.getErrors();
    // Handle validation errors
}
```

### XML Schema Validation

Basic XML well-formedness checking is provided:

#### Rust

```rust
use ditto_cot::schema::validate_cot_xml;

match validate_cot_xml(xml_content) {
    Ok(_) => println!("XML is well-formed"),
    Err(e) => eprintln!("XML error: {}", e),
}
```

#### Java

```java
import com.ditto.cot.schema.XmlValidator;

try {
    XmlValidator.validateCotXml(xmlContent);
    System.out.println("XML is valid");
} catch (ValidationException e) {
    System.err.println("XML error: " + e.getMessage());
}
```

### Validation Rules

#### Required Fields

**All Documents:**
- `_id` - Must be non-empty string
- `_v` - Must be integer ≥ 2

**MapItem Documents:**
- `e` - Callsign must be non-empty
- `w` - Type must be valid CoT hierarchy

**Chat Documents:**
- `message` - Must be non-empty
- `authorCallsign` - Must be non-empty

#### Coordinate Validation

```rust
// Latitude: -90.0 to 90.0
// Longitude: -180.0 to 180.0
// HAE: Any valid f64
// CE/LE: Non-negative
```

#### Type Validation

**CoT Event Types:**
- Must follow CoT hierarchy (e.g., "a-f-G-U-C")
- First character: a=atom, b=bit, c=capability
- Valid affiliation codes: f=friendly, h=hostile, n=neutral, u=unknown

## Schema Evolution

### Version 1 to Version 2 Migration

Major changes in version 2:

1. **Detail Storage**: Changed from Ditto map to string representation
2. **Property Names**: Shortened field names
3. **Counter Addition**: Added `_c` field for update tracking
4. **Underscore Prefixes**: Common properties prefixed with `_`

### Migration Strategy

```json
{
  "v1": {
    "version": 1,
    "detail": {
      "contact": {"callsign": "ALPHA-1"}
    }
  },
  "v2": {
    "_v": 2,
    "r": "<detail><contact callsign=\"ALPHA-1\"/></detail>"
  }
}
```

### Backward Compatibility

The library maintains read compatibility with version 1 documents but writes only version 2 format.

## Usage Examples

### Creating Schema-Compliant Documents

#### Rust

```rust
use ditto_cot::{cot_events::CotEvent, ditto::cot_to_document};

let event = CotEvent::builder()
    .uid("SCHEMA-TEST-001")
    .event_type("a-f-G-U-C")
    .location(34.0522, -118.2437, 100.0)
    .callsign("SCHEMA-TEST")
    .build();

let doc = cot_to_document(&event, "peer-123");
// Document automatically conforms to schema
```

#### Java

```java
import com.ditto.cot.CotEvent;
import com.ditto.cot.SdkDocumentConverter;

CotEvent event = CotEvent.builder()
    .uid("SCHEMA-TEST-001")
    .type("a-f-G-U-C")
    .point(34.0522, -118.2437, 100.0)
    .detail()
        .callsign("SCHEMA-TEST")
        .build()
    .build();

SdkDocumentConverter converter = new SdkDocumentConverter();
Map<String, Object> doc = converter.convertToDocumentMap(event, "peer-123");
// Document automatically conforms to schema
```

This schema reference provides the complete specification for all document types and validation rules used by the Ditto CoT library. For implementation details, see the [API Reference](api-reference.md) and [Integration Examples](../integration/).