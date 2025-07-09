# Java E2E Multi-Peer Test Flow

The Java E2E multi-peer test (`e2eMultiPeerMapItemSyncTest`) is a comprehensive test that validates Cursor-on-Target (CoT) sensor document synchronization between two Ditto peers. Here's the detailed flow:

## Test Overview
**Purpose**: Verify that CoT sensor XML events can be converted to Ditto documents, synchronized between peers, and handle sensor data fusion with conflict resolution in a distributed environment.

**Duration**: ~10 seconds  
**Peers**: 2 Ditto instances running locally with peer-to-peer networking

## Detailed Step-by-Step Flow

### **Step 0: Early XML Test (Pre-Ditto Validation)**
```
📍 EARLY XML TEST (Java):
```
- **Purpose**: Verify XML parsing works before any Ditto setup
- **Action**: Creates a test CoT XML message and parses it using `CoTConverter`
- **Validation**: Ensures XML → MapItemDocument conversion succeeds
- **Why Important**: Isolates XML parsing issues from Ditto connectivity issues

### **Step 1: Initialize Two Ditto Peers**
```
🔌 Step 1: Bringing both peers online...
```
- **Environment Loading**: Loads `.env` file from `rust/` directory using dotenv-java
- **Identity Setup**: Both peers use OnlinePlayground identity with:
  - App ID from `DITTO_APP_ID` env var
  - Playground token from `DITTO_PLAYGROUND_TOKEN` env var
  - Cloud sync disabled (local peer-to-peer only)
- **Peer Configuration**:
  - Peer 1: Temporary directory `/tmp/ditto-peer1-*`
  - Peer 2: Temporary directory `/tmp/ditto-peer2-*`
- **Transport Protocols**: Enables TCP, AWDL (Apple Wireless Direct Link), mDNS discovery
- **Connection**: Peers discover and connect to each other automatically
- **Wait Time**: 2 seconds for peer discovery

### **Step 2: Create CoT Sensor Document on Peer 1**
```
📤 Step 2: Creating CoT MapItem document on peer 1...
```
- **XML Generation**: Creates a realistic sensor CoT XML with:
  - Event type: `a-u-S` (Unknown Sensor)
  - UID: `MULTI-PEER-TEST-{UUID}`
  - Location: Norfolk, VA coordinates (37.32699544764403, -75.2905272033264)
  - How: `m-d-a` (Machine-to-machine Data Automatic)
  - Track data: course 30.86°, speed 1.36 m/s

- **Full CoT XML Message Used**:
  ```xml
  <?xml version="1.0" standalone="yes"?>
  <event version="2.0" 
         uid="MULTI-PEER-TEST-{UUID}" 
         type="a-u-S" 
         time="{ISO-8601-timestamp}" 
         start="{ISO-8601-timestamp}" 
         stale="{ISO-8601-timestamp + 30 minutes}" 
         how="m-d-a">
    <point ce="500.0" 
           hae="0.0" 
           lat="37.32699544764403" 
           le="100.0" 
           lon="-75.2905272033264" />
    <detail>
      <track course="30.86376880675669" 
             speed="1.3613854354920412" />
    </detail>
  </event>
  ```
  
  **Note for Developers**: This XML represents a sensor (`a-u-S`) reporting its position and movement. The sensor is located in Norfolk, VA, moving at ~1.36 m/s on a course of ~31°. Is this the appropriate message type and data for testing multi-peer sensor synchronization? Consider:
  - Should we use a different CoT type (e.g., `a-f-G-U-C` for ground unit)?
  - Should we include additional detail elements (e.g., `<contact>`, `<uid>`, `<precisionlocation>`)?
  - Are the CE/LE error values (500m/100m) appropriate for the test scenario?

- **Conversion Process**:
  1. XML → CoTEvent (JAXB parsing)
  2. CoTEvent → MapItemDocument (schema-based conversion)
  3. MapItemDocument → JSON (full document serialization)
- **Storage**: Uses DQL (Ditto Query Language) to insert the complete document:
  ```sql
  INSERT INTO map_items DOCUMENTS ({ full_json_document })
  ```

### **Step 3: Verify Document Sync Between Peers**
```
🔄 Step 3: Verifying document sync between peers...
```
- **Peer 1 Verification**: 
  ```sql
  SELECT * FROM map_items WHERE _id = 'document-id'
  ```
- **Sync Wait Logic**: Polls Peer 2 with retry mechanism:
  - Max attempts: 20
  - Interval: 100ms (like optimized Rust version)
  - Total max wait: 2 seconds
- **Success Criteria**: Document appears on both peers with identical data
- **Validation**: Compares core CoT fields (ID, type, lat, lon)

### **Step 4: Take Both Peers Offline**
```
📴 Step 4: Taking both clients offline...
```
- **Action**: Calls `ditto1.stopSync()` and `ditto2.stopSync()`
- **Purpose**: Simulates network partition for conflict testing
- **Wait Time**: 500ms for sync to fully stop

### **Step 5: Make Independent Sensor Modifications**
```
✏️ Step 5: Making independent modifications on both peers...
```
- **Sensor Fusion Conflict Setup**: Each peer modifies the same sensor document independently:

**Peer 1 Changes** (Sensor Update):
- Latitude: 37.32699544764403 → 38.0
- Longitude: -75.2905272033264 → -123.0  
- Track: `{course: "90.0", speed: "20.0"}` (heading East)

**Peer 2 Changes** (Conflicting Sensor Data):
- Latitude: 37.32699544764403 → 39.0
- Longitude: -75.2905272033264 → -124.0
- Track: `{course: "270.0", speed: "25.0"}` (heading West)

- **Storage**: Each peer updates its local copy using DQL UPDATE statements
- **Result**: Two divergent versions of the same document

### **Step 6: Bring Peers Back Online**
```
🔌 Step 6: Bringing both clients back online...
```
- **Reconnection**: Calls `ditto1.startSync()` and `ditto2.startSync()`
- **Sync Process**: Peers re-establish connection and begin conflict resolution
- **Wait Time**: 3 seconds for full reconnection and synchronization

### **Step 7: Validate Final Document State**
```
🔍 Step 7: Validating final document state after merge...
```
- **Convergence Check**: Queries both peers to ensure identical final state
- **Conflict Resolution**: Ditto's last-write-wins CRDT algorithm resolves conflicts
- **Expected Winner**: Peer 2 (typically, due to timestamp ordering)
- **Final State Validation**:
  - Both peers have identical document
  - Final coordinates: lat=39.0, lon=-124.0
  - Final track: course=270.0, speed=25.0
- **Round-trip Test**: Converts final document back to XML and re-parses

## Network Behavior Observed

The test logs show realistic peer-to-peer behavior:
- **Transport Protocols**: TCP and AWDL connections established
- **Peer Discovery**: Automatic discovery via mDNS and multicast
- **Connection Management**: Active transport switching between protocols
- **Resilience**: Graceful handling of auth server connectivity issues

## Test Success Criteria

✅ **All steps complete without errors**  
✅ **Document synchronization works between peers**  
✅ **Conflict resolution produces deterministic results**  
✅ **XML round-trip conversion succeeds**  
✅ **Total execution time < 15 seconds**

This test validates the entire CoT sensor-to-Ditto pipeline works correctly in Java, demonstrating realistic sensor data fusion scenarios and matching the functionality and reliability of the Rust implementation. The sensor message type (`a-u-S`) is particularly relevant for multi-peer scenarios where different sensors might be reporting conflicting or complementary data that needs to be synchronized and resolved across a distributed network.

## File Locations

- **Test Source**: `E2EMultiPeerTest.java` (same directory)
- **Environment**: `../../rust/.env` (loaded via dotenv-java)
- **Rust Equivalent**: `../../rust/tests/e2e_multi_peer.rs`

## Running the Test

```bash
# From java/ directory
./gradlew test --tests "com.ditto.cot.E2EMultiPeerTest"
```

**Prerequisites**:
- Ditto environment variables set in `rust/.env`
- macOS with AWDL support (or other supported platform)
- Java 17+
- Network connectivity for initial auth (falls back to local P2P)