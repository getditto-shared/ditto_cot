# API Reference

Complete API reference for the Ditto CoT library across all supported languages.

## Table of Contents

- [Core Types](#core-types)
- [Rust API](#rust-api)
- [Java API](#java-api)
- [Document Types](#document-types)
- [Error Types](#error-types)
- [Utility Functions](#utility-functions)

## Core Types

### CotEvent

Represents a Cursor-on-Target event with all standard fields and extensions.

**Fields:**
- `uid: String` - Unique identifier for the event
- `event_type: String` - CoT event type (e.g., "a-f-G-U-C")
- `time: DateTime` - Event timestamp
- `start: DateTime` - Event start time
- `stale: DateTime` - Event expiration time
- `how: String` - How the event was generated
- `point: Option<Point>` - Geographic coordinates
- `detail: String` - XML detail section

### Point

Geographic point with accuracy information.

**Fields:**
- `lat: f64` - Latitude in degrees
- `lon: f64` - Longitude in degrees
- `hae: f64` - Height above ellipsoid in meters
- `ce: f64` - Circular error in meters
- `le: f64` - Linear error in meters

### CotDocument

Enum representing different types of Ditto-compatible documents.

**Variants:**
- `MapItem` - Location updates and map graphics
- `Chat` - Chat messages
- `File` - File sharing events
- `Api` - API/emergency events

## Rust API

### CotEvent

#### Constructors

```rust
// Builder pattern (recommended)
CotEvent::builder() -> CotEventBuilder

// Convenience constructors
CotEvent::new_location_update(uid: &str, lat: f64, lon: f64, hae: f64) -> CotEvent
CotEvent::new_chat_message(uid: &str, callsign: &str, message: &str, room: &str, room_id: &str) -> CotEvent
```

#### Methods

```rust
// XML operations
fn from_xml(xml: &str) -> Result<CotEvent, CotEventError>
fn to_xml(&self) -> Result<String, CotEventError>

// Field access
fn uid(&self) -> &str
fn event_type(&self) -> &str
fn time(&self) -> DateTime<Utc>
fn point(&self) -> Option<&Point>
```

#### CotEventBuilder

```rust
impl CotEventBuilder {
    // Required fields
    fn uid(self, uid: &str) -> Self
    fn event_type(self, event_type: &str) -> Self
    
    // Optional fields
    fn time(self, time: DateTime<Utc>) -> Self
    fn start(self, start: DateTime<Utc>) -> Self
    fn stale(self, stale: DateTime<Utc>) -> Self
    fn stale_in(self, duration: Duration) -> Self
    fn how(self, how: &str) -> Self
    fn detail(self, detail: &str) -> Self
    
    // Point operations
    fn point(self, point: Point) -> Self
    fn location(self, lat: f64, lon: f64, hae: f64) -> Self
    fn location_with_accuracy(self, lat: f64, lon: f64, hae: f64, ce: f64, le: f64) -> Self
    
    // Convenience methods
    fn callsign(self, callsign: &str) -> Self
    fn team(self, team: &str) -> Self
    fn callsign_and_team(self, callsign: &str, team: &str) -> Self
    
    // Build
    fn build(self) -> CotEvent
}
```

### Point

#### Constructors

```rust
// Direct constructors
Point::new(lat: f64, lon: f64, hae: f64) -> Point
Point::with_accuracy(lat: f64, lon: f64, hae: f64, ce: f64, le: f64) -> Point

// Builder pattern
Point::builder() -> PointBuilder
```

#### PointBuilder

```rust
impl PointBuilder {
    fn lat(self, lat: f64) -> Self
    fn lon(self, lon: f64) -> Self
    fn hae(self, hae: f64) -> Self
    fn ce(self, ce: f64) -> Self
    fn le(self, le: f64) -> Self
    
    // Convenience methods
    fn coordinates(self, lat: f64, lon: f64, hae: f64) -> Self
    fn accuracy(self, ce: f64, le: f64) -> Self
    
    fn build(self) -> Point
}
```

### Ditto Integration

#### Document Conversion

```rust
// Convert CoT event to Ditto document
fn cot_to_document(event: &CotEvent, peer_id: &str) -> CotDocument

// Convert Ditto document back to CoT event
fn cot_event_from_ditto_document(doc: &CotDocument) -> CotEvent
```

#### SDK Observer Conversion

```rust
use ditto_cot::ditto::sdk_conversion;

// Convert observer document to typed CotDocument
fn observer_json_to_cot_document(boxed_doc: &BoxedDocument) -> Result<Option<CotDocument>, Box<dyn std::error::Error>>

// Reconstruct hierarchical JSON with r-fields
fn observer_json_to_json_with_r_fields(boxed_doc: &BoxedDocument) -> Result<String, Box<dyn std::error::Error>>

// Extract document metadata
fn extract_document_id(boxed_doc: &BoxedDocument) -> Result<String, Box<dyn std::error::Error>>
fn extract_document_type(boxed_doc: &BoxedDocument) -> Result<String, Box<dyn std::error::Error>>
```

### Error Types

```rust
#[derive(Debug, Error)]
pub enum CotEventError {
    #[error("XML parsing failed: {0}")]
    XmlParse(String),
    
    #[error("Invalid field value: {0}")]
    InvalidField(String),
    
    #[error("Required field missing: {0}")]
    MissingField(String),
    
    #[error("Serialization failed: {0}")]
    Serialization(String),
}
```

## Java API

### CotEvent

#### Constructors

```java
// Builder pattern (recommended)
public static CotEventBuilder builder()

// Direct constructor
public CotEvent(String uid, String type, Instant time, Point point, String detail)
```

#### Methods

```java
// XML operations
public static CotEvent fromXml(String xml) throws CotEventException
public String toXml() throws CotEventException

// Field access
public String getUid()
public String getType()
public Instant getTime()
public Point getPoint()
public String getDetail()

// Field modification
public void setUid(String uid)
public void setType(String type)
public void setTime(Instant time)
public void setPoint(Point point)
public void setDetail(String detail)
```

#### CotEventBuilder

```java
public class CotEventBuilder {
    // Required fields
    public CotEventBuilder uid(String uid)
    public CotEventBuilder type(String type)
    
    // Optional fields
    public CotEventBuilder time(Instant time)
    public CotEventBuilder start(Instant start)
    public CotEventBuilder stale(Instant stale)
    public CotEventBuilder staleIn(Duration duration)
    public CotEventBuilder how(String how)
    
    // Point operations
    public CotEventBuilder point(Point point)
    public CotEventBuilder point(double lat, double lon, double hae)
    public CotEventBuilder point(double lat, double lon, double hae, double ce, double le)
    
    // Detail builder
    public DetailBuilder detail()
    
    // Build
    public CotEvent build()
}
```

#### DetailBuilder

```java
public class DetailBuilder {
    // Common detail fields
    public DetailBuilder callsign(String callsign)
    public DetailBuilder groupName(String groupName)
    public DetailBuilder groupRole(String role)
    
    // Custom fields
    public DetailBuilder add(String key, String value)
    public DetailBuilder add(String key, Object value)
    
    // Chat-specific
    public DetailBuilder chat(String room, String message)
    public DetailBuilder chatGroup(String uid, String id, String senderCallsign, String message)
    
    // Status fields
    public DetailBuilder status(boolean readiness)
    public DetailBuilder battery(String level)
    
    // Track fields
    public DetailBuilder track(String speed, String course)
    
    // Build back to CotEventBuilder
    public CotEventBuilder build()
}
```

### Point

#### Constructors

```java
// Direct constructors
public Point(double lat, double lon, double hae)
public Point(double lat, double lon, double hae, double ce, double le)

// Builder pattern
public static PointBuilder builder()
```

#### Methods

```java
// Field access
public double getLat()
public double getLon()
public double getHae()
public double getCe()
public double getLe()

// Field modification
public void setLat(double lat)
public void setLon(double lon)
public void setHae(double hae)
public void setCe(double ce)
public void setLe(double le)

// Utility methods
public double distanceTo(Point other)
public boolean isValid()
```

### SdkDocumentConverter

Utility class for converting between CoT events and Ditto SDK documents.

```java
public class SdkDocumentConverter {
    // Constructor
    public SdkDocumentConverter()
    
    // Event to document conversion
    public Map<String, Object> convertToDocumentMap(CotEvent event, String peerId)
    
    // Observer document conversion
    public Object observerMapToTypedDocument(Map<String, Object> docMap)
    public String observerMapToJsonWithRFields(Map<String, Object> docMap)
    
    // Document metadata extraction
    public String getDocumentId(Map<String, Object> docMap)
    public String getDocumentType(Map<String, Object> docMap)
    
    // Validation
    public boolean validateDocument(Map<String, Object> docMap)
}
```

### Error Types

```java
public class CotEventException extends Exception {
    public enum ErrorType {
        XML_PARSING,
        INVALID_FIELD,
        MISSING_FIELD,
        SERIALIZATION
    }
    
    public CotEventException(ErrorType type, String message)
    public CotEventException(ErrorType type, String message, Throwable cause)
    
    public ErrorType getErrorType()
}

public class DocumentConversionException extends Exception {
    public DocumentConversionException(String message)
    public DocumentConversionException(String message, Throwable cause)
}
```

## Document Types

### MapItem Document

Represents location updates and map graphics.

#### Rust

```rust
pub struct MapItem {
    pub id: String,                    // _id: Document ID
    pub d_c: Option<i64>,             // _c: Counter
    pub d_v: Option<i64>,             // _v: Version
    pub d_r: Option<bool>,            // _r: Removed flag
    pub a: Option<String>,            // Peer ID
    pub b: Option<f64>,               // Timestamp
    pub d: Option<String>,            // Author UID
    pub e: String,                    // Callsign
    pub f: Option<bool>,              // Visible flag
    pub g: Option<String>,            // Version
    pub h: Option<f64>,               // CE (circular error)
    pub i: Option<f64>,               // HAE (height above ellipsoid)
    pub j: Option<f64>,               // Latitude
    pub k: Option<f64>,               // LE (linear error)
    pub l: Option<f64>,               // Longitude
    pub n: Option<i64>,               // Start time
    pub o: Option<i64>,               // Stale time
    pub p: Option<String>,            // How
    pub q: Option<String>,            // Access
    pub r: Option<HashMap<String, serde_json::Value>>, // Detail fields
    pub s: Option<String>,            // Opex
    pub t: Option<String>,            // QoS
    pub u: Option<String>,            // Caveat
    pub v: Option<String>,            // Releasable
    pub w: String,                    // Type
}
```

#### Java

```java
public class MapItemDocument {
    private String id;                    // _id
    private Long counter;                 // _c
    private Long version;                 // _v
    private Boolean removed;              // _r
    private String peerId;                // a
    private Double timestamp;             // b
    private String authorUid;             // d
    private String callsign;              // e
    private Boolean visible;              // f
    private String cotVersion;            // g
    private Double circularError;         // h
    private Double heightAboveEllipsoid;  // i
    private Double latitude;              // j
    private Double linearError;           // k
    private Double longitude;             // l
    private Long startTime;               // n
    private Long staleTime;               // o
    private String how;                   // p
    private String access;                // q
    private Map<String, Object> detail;   // r
    private String opex;                  // s
    private String qos;                   // t
    private String caveat;                // u
    private String releasable;            // v
    private String type;                  // w
    
    // Getters and setters for all fields
    // ...
}
```

### Chat Document

Represents chat messages.

#### Rust

```rust
pub struct Chat {
    pub id: String,                    // _id: Document ID
    pub d_c: Option<i64>,             // _c: Counter
    pub d_v: Option<i64>,             // _v: Version
    pub d_r: Option<bool>,            // _r: Removed flag
    pub message: String,               // Chat message content
    pub room: String,                  // Chat room name
    pub room_id: String,               // Chat room ID
    pub parent: Option<String>,        // Parent message ID
    pub author_callsign: String,       // Sender callsign
    pub author_uid: String,            // Sender UID
    pub author_type: Option<String>,   // Sender type
    pub time: String,                  // Message timestamp
    pub location: Option<String>,      // Sender location
    // Common CoT fields (a, b, d, e, etc.)
}
```

#### Java

```java
public class ChatDocument {
    private String id;
    private Long counter;
    private Long version;
    private Boolean removed;
    private String message;
    private String room;
    private String roomId;
    private String parent;
    private String authorCallsign;
    private String authorUid;
    private String authorType;
    private String time;
    private String location;
    
    // Getters and setters
    // ...
}
```

### File Document

Represents file sharing events.

#### Rust

```rust
pub struct File {
    pub id: String,                    // _id: Document ID
    pub d_c: Option<i64>,             // _c: Counter
    pub d_v: Option<i64>,             // _v: Version
    pub d_r: Option<bool>,            // _r: Removed flag
    pub file: Option<String>,          // Filename
    pub sz: Option<f64>,              // File size in bytes
    pub mime: Option<String>,          // MIME type
    pub content_type: Option<String>,  // Content type
    pub item_id: Option<String>,       // Associated item ID
    // Common CoT fields
}
```

#### Java

```java
public class FileDocument {
    private String id;
    private Long counter;
    private Long version;
    private Boolean removed;
    private String file;
    private Double size;
    private String mime;
    private String contentType;
    private String itemId;
    
    // Getters and setters
    // ...
}
```

### Api Document

Represents API/emergency events.

#### Rust

```rust
pub struct Api {
    pub id: String,                    // _id: Document ID
    pub d_c: Option<i64>,             // _c: Counter
    pub d_v: Option<i64>,             // _v: Version
    pub d_r: Option<bool>,            // _r: Removed flag
    pub e: String,                     // Callsign
    // Additional API-specific fields
    // Common CoT fields
}
```

#### Java

```java
public class ApiDocument {
    private String id;
    private Long counter;
    private Long version;
    private Boolean removed;
    private String callsign;
    
    // Getters and setters
    // ...
}
```

## Utility Functions

### Rust Utilities

```rust
// Validation functions
pub fn validate_coordinates(lat: f64, lon: f64) -> Result<(), String>
pub fn validate_cot_type(cot_type: &str) -> bool
pub fn validate_uid(uid: &str) -> bool

// Time utilities
pub fn parse_cot_time(time_str: &str) -> Result<DateTime<Utc>, chrono::ParseError>
pub fn format_cot_time(time: DateTime<Utc>) -> String

// Geographic utilities
pub fn calculate_distance(p1: &Point, p2: &Point) -> f64
pub fn calculate_bearing(p1: &Point, p2: &Point) -> f64

// Hash utilities
pub fn calculate_stable_key(document_id: &str, element_name: &str, index: u32) -> String
```

### Java Utilities

```java
// Validation utilities
public static boolean validateCoordinates(double lat, double lon)
public static boolean validateCotType(String cotType)
public static boolean validateUid(String uid)

// Time utilities
public static Instant parseCotTime(String timeStr) throws DateTimeParseException
public static String formatCotTime(Instant time)

// Geographic utilities
public static double calculateDistance(Point p1, Point p2)
public static double calculateBearing(Point p1, Point p2)

// Hash utilities
public static String calculateStableKey(String documentId, String elementName, int index)
```

## Constants

### CoT Event Types

```rust
// Rust constants
pub const COT_TYPE_FRIENDLY_GROUND: &str = "a-f-G-U-C";
pub const COT_TYPE_FRIENDLY_AIR: &str = "a-f-A-C";
pub const COT_TYPE_CHAT: &str = "b-t-f";
pub const COT_TYPE_EMERGENCY: &str = "b-a-o-can";
pub const COT_TYPE_FILE_SHARE: &str = "b-f-t-file";
```

```java
// Java constants
public static final String COT_TYPE_FRIENDLY_GROUND = "a-f-G-U-C";
public static final String COT_TYPE_FRIENDLY_AIR = "a-f-A-C";
public static final String COT_TYPE_CHAT = "b-t-f";
public static final String COT_TYPE_EMERGENCY = "b-a-o-can";
public static final String COT_TYPE_FILE_SHARE = "b-f-t-file";
```

### Collection Names

```rust
// Rust constants
pub const COLLECTION_MAP_ITEMS: &str = "map_items";
pub const COLLECTION_CHAT_MESSAGES: &str = "chat_messages";
pub const COLLECTION_FILES: &str = "files";
pub const COLLECTION_API_EVENTS: &str = "api_events";
```

```java
// Java constants
public static final String COLLECTION_MAP_ITEMS = "map_items";
public static final String COLLECTION_CHAT_MESSAGES = "chat_messages";
public static final String COLLECTION_FILES = "files";
public static final String COLLECTION_API_EVENTS = "api_events";
```

## Version Information

- **Current Version**: 1.0.0
- **Minimum Rust Version**: 1.70+
- **Minimum Java Version**: 17+
- **Schema Version**: 2

For detailed usage examples, see:
- [Rust Integration Examples](../integration/examples/rust.md)
- [Java Integration Examples](../integration/examples/java.md)
- [Ditto SDK Integration Guide](../integration/ditto-sdk.md)