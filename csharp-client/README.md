# Ditto CoT Client with Background Service

A service-based command-line client for querying Ditto documents and converting them to Cursor on Target (CoT) XML format. The architecture separates the Ditto SDK into a persistent background service that maintains connections and sync, while the CLI communicates with it via IPC.

## Architecture

The solution consists of two components:

1. **DittoCoTService**: Background service that runs the Ditto SDK continuously
2. **DittoCoTClient**: CLI that communicates with the service via Named Pipes

This architecture allows:
- Persistent Ditto connections and sync
- Fast CLI responses (no SDK initialization delay)
- Multiple CLI instances sharing one service
- Better resource management

## Prerequisites

- .NET 9.0 SDK
- Ditto SDK 4.7.2 (included as dependency)
- Ditto license/credentials:
  - Shared Key for production use
  - Playground token for development
  - Or offline-only mode

## Build

From the `csharp-client` directory:

```bash
# Build both projects
dotnet build DittoCoTClient.csproj
dotnet build DittoCoTService.csproj
```

## Usage

### Service Management

#### Start the Service

```bash
# Start with shared key authentication (production)
dotnet bin/Debug/net9.0/DittoCoTClient.dll service start --app-id "your-app-id" --shared-key "your-shared-key"

# Start with playground token (development) 
dotnet bin/Debug/net9.0/DittoCoTClient.dll service start --app-id "your-app-id" --playground-token "your-token"

# Start in offline-only mode (no sync)
dotnet bin/Debug/net9.0/DittoCoTClient.dll service start --offline
```

#### Check Service Status

```bash
dotnet bin/Debug/net9.0/DittoCoTClient.dll service status
```

#### Stop the Service

```bash
dotnet bin/Debug/net9.0/DittoCoTClient.dll service stop
```

### Querying Documents

Once the service is running, you can query documents:

#### List All Documents

```bash
dotnet bin/Debug/net9.0/DittoCoTClient.dll list
dotnet bin/Debug/net9.0/DittoCoTClient.dll list --collection my_collection --limit 20
```

#### Query with Filters

```bash
# Query by event type
dotnet bin/Debug/net9.0/DittoCoTClient.dll query --query "w == \"a-f-G-U-C\""

# Query by document ID
dotnet bin/Debug/net9.0/DittoCoTClient.dll query --query "_id == \"UNIT-123\""

# Query by callsign
dotnet bin/Debug/net9.0/DittoCoTClient.dll query --query "e == \"ALPHA-1\""

# Complex queries
dotnet bin/Debug/net9.0/DittoCoTClient.dll query --query "w == \"a-f-G-U-C\" && j > 34.0"
```

## Authentication Options

### Shared Key Authentication (Production)

For production environments with a Ditto shared key:

```bash
dotnet bin/Debug/net9.0/DittoCoTClient.dll service start \
  --app-id "your-app-id" \
  --shared-key "your-shared-key"
```

### Playground Token (Development)

For development with a playground token:

```bash
dotnet bin/Debug/net9.0/DittoCoTClient.dll service start \
  --app-id "your-app-id" \
  --playground-token "your-playground-token"
```

### Offline Mode

For local testing without network connectivity:

```bash
dotnet bin/Debug/net9.0/DittoCoTClient.dll service start --offline
```

## Output Format

The client displays each document in two formats:

1. **Ditto Document (JSON)**: The raw document as stored in Ditto
2. **CoT XML**: The document converted to CoT format

Example output:

```
================================================================================
Document #1
================================================================================

📋 Ditto Document (JSON) - ID: ANDROID-359975090805611
{
  "_id": "ANDROID-359975090805611",
  "_c": 1,
  "_v": 2,
  "_r": false,
  "a": "peer-123",
  "b": 1673780400000,
  "d": "ANDROID-359975090805611",
  "e": "ALPHA-1",
  "w": "a-f-G-U-C",
  "j": 34.12345,
  "l": -118.12345,
  ...
}

🎯 CoT Conversion:
✅ Successfully converted to CoT XML:
<event version="2.0" uid="ANDROID-359975090805611" type="a-f-G-U-C" 
       time="2023-01-15T10:30:00.000Z" start="2023-01-15T10:30:00.000Z" 
       stale="2023-01-15T10:35:00.000Z" how="h-g-i-g-o">
  <point lat="34.12345" lon="-118.12345" hae="150" ce="10" le="20" />
  <detail>
    <contact callsign="ALPHA-1" endpoint="*:-1:stcp" />
  </detail>
</event>
```

## Command Reference

### Service Commands

| Command | Description |
|---------|-------------|
| `service start` | Start the Ditto background service |
| `service stop` | Stop the Ditto background service |
| `service status` | Check if service is running and responding |

### Query Commands

| Command | Description |
|---------|-------------|
| `list` | List all documents in a collection |
| `query` | Query documents with DQL filter |

### Common Options

| Option | Description | Default |
|--------|-------------|---------|
| `--app-id` | Ditto App ID | "ditto-cot-client" |
| `--shared-key` | Shared key for production auth | - |
| `--playground-token` | Playground token for dev auth | - |
| `--offline` | Run in offline-only mode | false |
| `--collection` | Collection to query | "cot_events" |
| `--limit` | Max documents to retrieve | 10 |
| `--query` | DQL query string | "true" |

## Error Handling

### Service Not Running

If you see "Cannot connect to Ditto service", start the service first:

```bash
dotnet bin/Debug/net9.0/DittoCoTClient.dll service start
```

### Authentication Errors

- **Shared Key**: Verify your app ID and shared key are correct
- **Playground Token**: Check token validity and network connectivity
- **Offline**: Some operations require valid licensing

### Query Errors

Query syntax errors are passed through from the Ditto SDK:

- Escape quotes in string values: `\"value\"`
- Use proper field names: `_id`, `w`, `e`, etc.
- Valid operators: `==`, `!=`, `<`, `>`, `CONTAINS`, etc.

## Architecture Details

### Inter-Process Communication

- **Named Pipes**: Used for IPC between client and service
- **JSON Protocol**: Request/response messages in JSON format
- **Connection Pooling**: Service handles multiple client connections

### Service Lifecycle

1. Service starts and initializes Ditto SDK
2. Begins sync (if not offline mode)
3. Listens for client connections on named pipe
4. Processes query requests and returns results
5. Maintains persistent connections until stopped

### Benefits

- **Performance**: No SDK initialization delay for each query
- **Resource Efficiency**: Single Ditto instance shared across clients
- **Sync Continuity**: Background service maintains sync state
- **Scalability**: Multiple CLI instances can use same service

## Troubleshooting

### Service Won't Start

1. Check if another instance is running: `service status`
2. Verify credentials are correct
3. Ensure network connectivity (for online modes)
4. Check Ditto licensing requirements

### IPC Connection Issues

1. Verify service is running: `service status`
2. Check named pipe permissions
3. Try restarting the service: `service stop && service start`

### Query Issues

1. Test with simple query: `--query "true"`
2. Verify collection exists and has data
3. Check field names match Ditto document structure
4. Use `service status` to verify Ditto is initialized

## Development

To extend the client:

1. **Add new commands**: Extend the `Program.cs` command structure
2. **New service operations**: Add handlers in `DittoBackgroundService.cs`
3. **IPC protocol**: Update `ServiceRequest`/`ServiceResponse` classes
4. **Authentication**: Add new identity methods in service initialization