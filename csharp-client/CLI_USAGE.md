# Ditto CoT Client CLI Usage

## Easy Usage with Wrapper Script

Instead of typing `dotnet run --project DittoCoTClient.csproj --` every time, use the wrapper script:

```bash
./ditto-cot [command] [options]
```

The wrapper script automatically:
- Builds the project if needed
- Uses the correct project file
- Passes all arguments to the client

## Configuration Commands

### Set Configuration
```bash
./ditto-cot config set --app-id "your-app-id" --shared-key "your-shared-key" --offline-token "your-offline-token"
```

### Show Current Configuration
```bash
./ditto-cot config show
```

### Delete Configuration
```bash
./ditto-cot config delete
```

## Service Commands

### Start Service (uses saved config)
```bash
./ditto-cot service start
```

### Check Service Status
```bash
./ditto-cot service status
```

### Stop Service
```bash
./ditto-cot service stop
```

## Query Commands

### List Documents
```bash
./ditto-cot list --collection cot_events --limit 10
```

### Query Documents
```bash
./ditto-cot query --collection cot_events --query 'w == "a-f-G-U-C"' --limit 5
```

## Configuration File

The configuration is automatically saved to:
`~/.ditto-cot-client/config.json`

This file contains:
- App ID
- Authentication credentials (shared key, offline token, playground token)
- Default collection name
- Default query limit
- Last updated timestamp

## Testing

Run the test script to verify configuration functionality:
```bash
./test_config.sh
```

This tests:
1. Configuration setting and saving
2. Configuration reading from file  
3. Service startup with saved config