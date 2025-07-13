# Ditto CoT C# Library

This C# library provides functionality to convert between Cursor on Target (CoT) XML events and Ditto-compatible CRDT documents, following the same patterns as the Rust and Java implementations.

## Features

- Parse CoT XML messages into strongly-typed objects
- Convert CoT events to appropriate Ditto document types (MapItem, Chat, Api, File, Generic)
- Convert Ditto documents back to CoT events
- Full round-trip conversion support
- Comprehensive unit tests
- Schema-based document models

## Dependencies

- .NET 9.0
- Ditto SDK 4.7.2
- Newtonsoft.Json for JSON serialization
- NUnit for testing

## Document Types

The library supports conversion between CoT events and the following Ditto document types based on the CoT event type:

- **MapItemDocument**: For location/position events (a-f-G-U-C, a-u-r-loc-g, etc.)
- **ChatDocument**: For chat/messaging events (b-t-f, chat types)
- **ApiDocument**: For emergency events (a-u-emergency-g)
- **FileDocument**: For file sharing events (file, attachment, b-f-t-a types)
- **GenericDocument**: For all other event types

## Usage Examples

### Convert CoT XML to Ditto Document

```csharp
using Ditto.Cot;

// Parse CoT XML and convert to appropriate Ditto document
string cotXml = @"<event version=\"2.0\" uid=\"UNIT-123\" type=\"a-f-G-U-C\" 
                         time=\"2023-01-15T10:30:00.000Z\" 
                         start=\"2023-01-15T10:30:00.000Z\" 
                         stale=\"2023-01-15T10:35:00.000Z\" 
                         how=\"h-g-i-g-o\">
                    <point lat=\"34.12345\" lon=\"-118.12345\" hae=\"150.0\" ce=\"10.0\" le=\"20.0\"/>
                    <detail>
                        <contact callsign=\"ALPHA-1\" endpoint=\"*:-1:stcp\"/>
                    </detail>
                  </event>";

var dittoDoc = DocumentConverter.ConvertXmlToDocument(cotXml, "my-peer-id");
// Returns a MapItemDocument for this friendly unit event

// Convert to JSON for Ditto storage
string json = DocumentConverter.ConvertDocumentToJson(dittoDoc);
```

### Convert Ditto Document back to CoT

```csharp
using Ditto.Cot.Models;

// Create or retrieve a Ditto document
var mapItem = new MapItemDocument
{
    Id = "UNIT-456",
    EventType = "a-f-G-U-C",
    PeerKey = "peer-123",
    AuthorUid = "UNIT-456",
    Latitude = 35.0,
    Longitude = -119.0,
    HeightAboveEllipsoid = 200.0,
    CircularError = 5.0,
    LinearError = 10.0,
    // ... other fields
};

// Convert back to CoT event
var cotEvent = DocumentConverter.ConvertMapItemDocumentToCoTEvent(mapItem);

// Generate XML
string xml = DocumentConverter.ConvertCoTEventToXml(cotEvent);
```

### Working with Different Document Types

```csharp
// Chat event
var chatDoc = DocumentConverter.ConvertXmlToDocument(chatXml, peerKey) as ChatDocument;

// Emergency event  
var apiDoc = DocumentConverter.ConvertXmlToDocument(emergencyXml, peerKey) as ApiDocument;

// File sharing event
var fileDoc = DocumentConverter.ConvertXmlToDocument(fileXml, peerKey) as FileDocument;
```

## Schema Compliance

The document models are generated from the JSON schemas in the `/schema` directory and follow the Ditto CRDT document structure:

- **Common fields**: All documents inherit from `CommonDocument` with shared fields like `_id`, `_c`, `_v`, `_r`, `a`, `b`, etc.
- **Type-specific fields**: Each document type adds its own specific fields (e.g., `message` for Chat, `isFile` for Api)
- **Detail mapping**: CoT detail sections are converted to the `r` field as a dictionary for CRDT support

## Field Mappings

Key field mappings from CoT to Ditto documents:

| CoT Field | Ditto Field | Description |
|-----------|-------------|-------------|
| uid | \_id, d | Document ID and author UID |
| type | w | Event type |
| time | b, n | Time in millis and microseconds |
| start | n | Start time in microseconds |
| stale | o | Stale time in microseconds |
| how | p | How field |
| point.lat | j | Latitude |
| point.lon | l | Longitude |
| point.hae | i | Height above ellipsoid |
| point.ce | h | Circular error |
| point.le | k | Linear error |
| detail | r | Detail map for CRDT support |

## Testing

The library includes comprehensive unit tests covering:

- CoT to Ditto document conversion for all document types
- Ditto document to CoT event conversion 
- Round-trip conversion fidelity
- Error handling for invalid inputs
- Edge cases and null value handling

Run tests with:
```bash
dotnet test
```

## Build

```bash
dotnet restore
dotnet build
```

## Integration with Ditto

This library is designed to work with the Ditto SDK 4.7.2. The generated documents can be stored directly in Ditto collections and will benefit from CRDT synchronization, especially for the detail fields stored in the `r` map.

Example Ditto integration:
```csharp
// Convert CoT to Ditto document
var dittoDoc = DocumentConverter.ConvertXmlToDocument(cotXml, ditto.Auth.UserId);

// Store in Ditto collection
var collection = ditto.Store.Collection("cot_events");
await collection.Upsert(dittoDoc);
```