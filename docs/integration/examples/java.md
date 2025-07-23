# Java Integration Examples

This guide provides comprehensive examples for integrating the Ditto CoT library in Java applications, including Android development patterns and enterprise integration.

## Table of Contents

- [Basic Integration](#basic-integration)
- [Android Integration](#android-integration)
- [Enterprise Patterns](#enterprise-patterns)
- [Observer Integration](#observer-integration)
- [Concurrency and Threading](#concurrency-and-threading)
- [Error Handling](#error-handling)
- [Testing Patterns](#testing-patterns)

## Basic Integration

### Simple CoT Event Creation

```java
import com.ditto.cot.CotEvent;
import com.ditto.cot.SdkDocumentConverter;
import com.ditto.cot.schema.*;
import java.time.Instant;
import java.time.temporal.ChronoUnit;

public class BasicCotIntegration {
    
    public static void main(String[] args) {
        try {
            // Create a location update event
            CotEvent locationEvent = createLocationUpdate();
            
            // Convert to XML
            String xml = locationEvent.toXml();
            System.out.println("Generated XML:\n" + xml);
            
            // Convert to Ditto document
            SdkDocumentConverter converter = new SdkDocumentConverter();
            Map<String, Object> docMap = converter.convertToDocumentMap(locationEvent, "java-peer-123");
            
            // Process the document
            processDocument(docMap, converter);
            
        } catch (Exception e) {
            System.err.println("Integration failed: " + e.getMessage());
            e.printStackTrace();
        }
    }
    
    private static CotEvent createLocationUpdate() {
        return CotEvent.builder()
            .uid("JAVA-UNIT-001")
            .type("a-f-G-U-C")  // Friendly ground unit
            .time(Instant.now())
            .start(Instant.now())
            .stale(Instant.now().plus(5, ChronoUnit.MINUTES))
            .how("h-g-i-g-o")  // GPS
            .point(34.052235, -118.243683, 100.0, 5.0, 10.0)  // LA with accuracy
            .detail()
                .callsign("JAVA-ALPHA")
                .groupName("Blue")
                .add("platform", "Android")
                .add("version", "1.0.0")
                .build()
            .build();
    }
    
    private static void processDocument(Map<String, Object> docMap, SdkDocumentConverter converter) {
        // Extract basic info
        String docId = converter.getDocumentId(docMap);
        String docType = converter.getDocumentType(docMap);
        
        System.out.println("Document ID: " + docId);
        System.out.println("Document Type: " + docType);
        
        // Convert to typed document
        Object typedDoc = converter.observerMapToTypedDocument(docMap);
        
        if (typedDoc instanceof MapItemDocument) {
            MapItemDocument mapItem = (MapItemDocument) typedDoc;
            System.out.println("Location: " + mapItem.getE() + 
                             " at " + mapItem.getJ() + "," + mapItem.getL());
        }
    }
}
```

### XML Processing

```java
import com.ditto.cot.CotEvent;
import javax.xml.parsers.DocumentBuilderFactory;
import org.w3c.dom.Document;
import java.io.StringReader;
import javax.xml.parsers.DocumentBuilder;
import org.xml.sax.InputSource;

public class XmlProcessingExample {
    
    public void processComplexCotXml() {
        String complexXml = """
            <event version="2.0" uid="COMPLEX-001" type="a-f-G-U-C" 
                   time="2024-01-15T10:30:00.000Z" 
                   start="2024-01-15T10:30:00.000Z" 
                   stale="2024-01-15T10:35:00.000Z" 
                   how="h-g-i-g-o">
                <point lat="34.052235" lon="-118.243683" hae="100.0" ce="5.0" le="10.0"/>
                <detail>
                    <contact callsign="JAVA-ALPHA"/>
                    <__group name="Blue" role="Team Leader"/>
                    <status readiness="true"/>
                    <track speed="15.0" course="90.0"/>
                    <takv device="Samsung Galaxy" platform="Android" version="4.8.1"/>
                </detail>
            </event>
            """;
        
        try {
            // Parse XML to CotEvent
            CotEvent event = CotEvent.fromXml(complexXml);
            
            System.out.println("Parsed event:");
            System.out.println("  UID: " + event.getUid());
            System.out.println("  Type: " + event.getType());
            System.out.println("  Time: " + event.getTime());
            
            // Access point data
            if (event.getPoint() != null) {
                var point = event.getPoint();
                System.out.println("  Location: " + point.getLat() + ", " + 
                                 point.getLon() + " at " + point.getHae() + "m");
                System.out.println("  Accuracy: CE=" + point.getCe() + 
                                 "m, LE=" + point.getLe() + "m");
            }
            
            // Process detail section
            processDetailSection(event);
            
            // Round-trip test
            String regeneratedXml = event.toXml();
            validateRoundTrip(complexXml, regeneratedXml);
            
        } catch (Exception e) {
            System.err.println("XML processing failed: " + e.getMessage());
        }
    }
    
    private void processDetailSection(CotEvent event) {
        // Detail processing depends on your schema implementation
        // This is a conceptual example
        if (event.getDetail() != null) {
            System.out.println("  Detail section contains tactical information");
            
            // Extract specific fields if needed
            // Implementation depends on your detail parsing strategy
        }
    }
    
    private void validateRoundTrip(String original, String regenerated) {
        try {
            // Parse both as DOM for semantic comparison
            DocumentBuilder builder = DocumentBuilderFactory.newInstance().newDocumentBuilder();
            Document originalDoc = builder.parse(new InputSource(new StringReader(original)));
            Document regeneratedDoc = builder.parse(new InputSource(new StringReader(regenerated)));
            
            // Semantic comparison (simplified)
            String originalUid = originalDoc.getDocumentElement().getAttribute("uid");
            String regeneratedUid = regeneratedDoc.getDocumentElement().getAttribute("uid");
            
            if (originalUid.equals(regeneratedUid)) {
                System.out.println("✓ Round-trip validation successful");
            } else {
                System.out.println("✗ Round-trip validation failed");
            }
            
        } catch (Exception e) {
            System.err.println("Round-trip validation error: " + e.getMessage());
        }
    }
}
```

## Android Integration

### Android Service Integration

```java
import android.app.Service;
import android.content.Intent;
import android.os.Binder;
import android.os.IBinder;
import android.util.Log;
import androidx.annotation.Nullable;
import com.ditto.java.Ditto;
import com.ditto.java.DittoStore;
import com.ditto.cot.SdkDocumentConverter;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class CotSyncService extends Service {
    private static final String TAG = "CotSyncService";
    
    private Ditto ditto;
    private SdkDocumentConverter converter;
    private ExecutorService executorService;
    private final IBinder binder = new CotSyncBinder();
    
    public class CotSyncBinder extends Binder {
        public CotSyncService getService() {
            return CotSyncService.this;
        }
    }
    
    @Override
    public void onCreate() {
        super.onCreate();
        Log.d(TAG, "CotSyncService created");
        
        try {
            initializeDitto();
            converter = new SdkDocumentConverter();
            executorService = Executors.newFixedThreadPool(4);
            setupObservers();
            
        } catch (Exception e) {
            Log.e(TAG, "Failed to initialize service", e);
        }
    }
    
    private void initializeDitto() throws Exception {
        String appId = getApplicationContext().getString(R.string.ditto_app_id);
        String token = getApplicationContext().getString(R.string.ditto_token);
        
        DittoIdentity identity = new DittoIdentity.OnlinePlayground(appId, token, true);
        ditto = new Ditto(DittoRoot.fromContext(getApplicationContext()), identity);
        ditto.startSync();
        
        Log.d(TAG, "Ditto initialized and sync started");
    }
    
    private void setupObservers() {
        DittoStore store = ditto.getStore();
        
        // Location updates observer
        store.registerObserver("SELECT * FROM map_items", (result, event) -> {
            executorService.submit(() -> handleLocationUpdates(result));
        });
        
        // Chat messages observer
        store.registerObserver("SELECT * FROM chat_messages ORDER BY b DESC LIMIT 50", 
                              (result, event) -> {
            executorService.submit(() -> handleChatMessages(result));
        });
        
        // Emergency events observer (high priority)
        store.registerObserver("SELECT * FROM api_events WHERE w LIKE 'b-a-%'", 
                              (result, event) -> {
            executorService.submit(() -> handleEmergencyEvents(result));
        });
        
        Log.d(TAG, "Observers registered");
    }
    
    private void handleLocationUpdates(DittoQueryResult result) {
        for (DittoQueryResultItem item : result.getItems()) {
            try {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof MapItemDocument) {
                    MapItemDocument mapItem = (MapItemDocument) typedDoc;
                    
                    // Update UI via broadcast
                    Intent intent = new Intent("com.yourapp.LOCATION_UPDATE");
                    intent.putExtra("callsign", mapItem.getE());
                    intent.putExtra("lat", mapItem.getJ());
                    intent.putExtra("lon", mapItem.getL());
                    sendBroadcast(intent);
                    
                    Log.d(TAG, "Location update: " + mapItem.getE() + 
                         " at " + mapItem.getJ() + "," + mapItem.getL());
                }
                
            } catch (Exception e) {
                Log.e(TAG, "Error processing location update", e);
            }
        }
    }
    
    private void handleChatMessages(DittoQueryResult result) {
        for (DittoQueryResultItem item : result.getItems()) {
            try {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof ChatDocument) {
                    ChatDocument chat = (ChatDocument) typedDoc;
                    
                    // Send notification
                    showChatNotification(chat);
                    
                    // Update UI
                    Intent intent = new Intent("com.yourapp.CHAT_MESSAGE");
                    intent.putExtra("sender", chat.getAuthorCallsign());
                    intent.putExtra("message", chat.getMessage());
                    intent.putExtra("room", chat.getRoom());
                    sendBroadcast(intent);
                }
                
            } catch (Exception e) {
                Log.e(TAG, "Error processing chat message", e);
            }
        }
    }
    
    private void handleEmergencyEvents(DittoQueryResult result) {
        for (DittoQueryResultItem item : result.getItems()) {
            try {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof ApiDocument) {
                    ApiDocument emergency = (ApiDocument) typedDoc;
                    
                    Log.w(TAG, "🚨 EMERGENCY EVENT: " + emergency.getE());
                    
                    // High priority notification
                    showEmergencyNotification(emergency);
                    
                    // Alert all systems
                    Intent intent = new Intent("com.yourapp.EMERGENCY_EVENT");
                    intent.putExtra("callsign", emergency.getE());
                    intent.putExtra("emergency_data", docMap);
                    sendBroadcast(intent);
                }
                
            } catch (Exception e) {
                Log.e(TAG, "Error processing emergency event", e);
            }
        }
    }
    
    public void sendCotEvent(String cotXml) {
        executorService.submit(() -> {
            try {
                CotEvent event = CotEvent.fromXml(cotXml);
                Map<String, Object> docMap = converter.convertToDocumentMap(event, 
                    ditto.getIdentity().toString());
                
                String collection = determineCollection(docMap);
                String query = String.format("INSERT INTO %s DOCUMENTS (?) ON ID CONFLICT DO MERGE", collection);
                
                ditto.getStore().execute(query, docMap);
                Log.d(TAG, "CoT event stored in collection: " + collection);
                
            } catch (Exception e) {
                Log.e(TAG, "Failed to send CoT event", e);
            }
        });
    }
    
    private String determineCollection(Map<String, Object> docMap) {
        String docType = converter.getDocumentType(docMap);
        
        switch (docType) {
            case "MapItem": return "map_items";
            case "Chat": return "chat_messages";
            case "File": return "files";
            case "Api": return "api_events";
            default: return "api_events"; // Fallback
        }
    }
    
    private void showChatNotification(ChatDocument chat) {
        // Implementation depends on your notification strategy
        Log.d(TAG, "Chat notification: " + chat.getAuthorCallsign() + ": " + chat.getMessage());
    }
    
    private void showEmergencyNotification(ApiDocument emergency) {
        // High priority emergency notification
        Log.w(TAG, "Emergency notification: " + emergency.getE());
    }
    
    @Nullable
    @Override
    public IBinder onBind(Intent intent) {
        return binder;
    }
    
    @Override
    public void onDestroy() {
        super.onDestroy();
        if (executorService != null) {
            executorService.shutdown();
        }
        if (ditto != null) {
            ditto.stopSync();
        }
        Log.d(TAG, "CotSyncService destroyed");
    }
}
```

### Android Activity Integration

```java
import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.IntentFilter;
import android.os.Bundle;
import androidx.appcompat.app.AppCompatActivity;
import androidx.localbroadcastmanager.content.LocalBroadcastManager;
import com.google.android.gms.maps.GoogleMap;
import com.google.android.gms.maps.MapView;
import com.google.android.gms.maps.model.LatLng;
import com.google.android.gms.maps.model.Marker;
import com.google.android.gms.maps.model.MarkerOptions;
import java.util.HashMap;
import java.util.Map;

public class TacticalMapActivity extends AppCompatActivity {
    private MapView mapView;
    private GoogleMap googleMap;
    private Map<String, Marker> unitMarkers = new HashMap<>();
    private CotSyncService cotService;
    
    private BroadcastReceiver locationReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            String callsign = intent.getStringExtra("callsign");
            double lat = intent.getDoubleExtra("lat", 0.0);
            double lon = intent.getDoubleExtra("lon", 0.0);
            
            updateUnitLocation(callsign, lat, lon);
        }
    };
    
    private BroadcastReceiver chatReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            String sender = intent.getStringExtra("sender");
            String message = intent.getStringExtra("message");
            String room = intent.getStringExtra("room");
            
            displayChatMessage(sender, message, room);
        }
    };
    
    private BroadcastReceiver emergencyReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            String callsign = intent.getStringExtra("callsign");
            
            handleEmergencyAlert(callsign);
        }
    };
    
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_tactical_map);
        
        initializeMap(savedInstanceState);
        registerReceivers();
        
        // Start CoT sync service
        Intent serviceIntent = new Intent(this, CotSyncService.class);
        startService(serviceIntent);
    }
    
    private void initializeMap(Bundle savedInstanceState) {
        mapView = findViewById(R.id.map_view);
        mapView.onCreate(savedInstanceState);
        mapView.getMapAsync(map -> {
            googleMap = map;
            // Configure map settings
            googleMap.getUiSettings().setZoomControlsEnabled(true);
            googleMap.getUiSettings().setCompassEnabled(true);
        });
    }
    
    private void registerReceivers() {
        IntentFilter locationFilter = new IntentFilter("com.yourapp.LOCATION_UPDATE");
        registerReceiver(locationReceiver, locationFilter);
        
        IntentFilter chatFilter = new IntentFilter("com.yourapp.CHAT_MESSAGE");
        registerReceiver(chatReceiver, chatFilter);
        
        IntentFilter emergencyFilter = new IntentFilter("com.yourapp.EMERGENCY_EVENT");
        registerReceiver(emergencyReceiver, emergencyFilter);
    }
    
    private void updateUnitLocation(String callsign, double lat, double lon) {
        if (googleMap == null) return;
        
        runOnUiThread(() -> {
            LatLng position = new LatLng(lat, lon);
            
            Marker marker = unitMarkers.get(callsign);
            if (marker == null) {
                // Create new marker
                MarkerOptions options = new MarkerOptions()
                    .position(position)
                    .title(callsign)
                    .snippet("Friendly Unit");
                
                marker = googleMap.addMarker(options);
                unitMarkers.put(callsign, marker);
            } else {
                // Update existing marker
                marker.setPosition(position);
            }
        });
    }
    
    private void displayChatMessage(String sender, String message, String room) {
        runOnUiThread(() -> {
            // Update chat UI (implementation depends on your chat UI)
            Log.d("Chat", sender + " in " + room + ": " + message);
            
            // Could show toast, update chat window, etc.
            Toast.makeText(this, sender + ": " + message, Toast.LENGTH_SHORT).show();
        });
    }
    
    private void handleEmergencyAlert(String callsign) {
        runOnUiThread(() -> {
            // High priority UI update
            AlertDialog.Builder builder = new AlertDialog.Builder(this);
            builder.setTitle("🚨 EMERGENCY ALERT")
                   .setMessage("Emergency event from: " + callsign)
                   .setPositiveButton("Acknowledge", null)
                   .setCancelable(false)
                   .show();
            
            // Flash the marker or highlight on map
            Marker marker = unitMarkers.get(callsign);
            if (marker != null) {
                // Animate or highlight the emergency unit
                googleMap.animateCamera(CameraUpdateFactory.newLatLng(marker.getPosition()));
            }
        });
    }
    
    public void sendLocationUpdate() {
        // Example of sending own location
        if (cotService != null) {
            try {
                CotEvent locationEvent = CotEvent.builder()
                    .uid("ANDROID-" + Build.SERIAL)
                    .type("a-f-G-U-C")
                    .time(Instant.now())
                    .point(getCurrentLat(), getCurrentLon(), 0.0, 10.0, 15.0)
                    .detail()
                        .callsign("ANDROID-USER")
                        .groupName("Blue")
                        .add("platform", "Android")
                        .add("device", Build.MODEL)
                        .build()
                    .build();
                
                String xml = locationEvent.toXml();
                cotService.sendCotEvent(xml);
                
            } catch (Exception e) {
                Log.e("TacticalMap", "Failed to send location update", e);
            }
        }
    }
    
    @Override
    protected void onDestroy() {
        super.onDestroy();
        unregisterReceiver(locationReceiver);
        unregisterReceiver(chatReceiver);
        unregisterReceiver(emergencyReceiver);
        
        if (mapView != null) {
            mapView.onDestroy();
        }
    }
    
    // Lifecycle methods for MapView
    @Override
    protected void onResume() {
        super.onResume();
        mapView.onResume();
    }
    
    @Override
    protected void onPause() {
        super.onPause();
        mapView.onPause();
    }
}
```

## Enterprise Patterns

### Spring Boot Integration

```java
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.scheduling.annotation.EnableAsync;
import org.springframework.scheduling.annotation.EnableScheduling;
import org.springframework.stereotype.Service;
import org.springframework.scheduling.annotation.Async;
import org.springframework.scheduling.annotation.Scheduled;
import com.ditto.java.Ditto;
import com.ditto.cot.SdkDocumentConverter;
import java.util.concurrent.CompletableFuture;

@SpringBootApplication
@EnableAsync
@EnableScheduling
public class CotIntegrationApplication {
    
    public static void main(String[] args) {
        SpringApplication.run(CotIntegrationApplication.class, args);
    }
}

@Configuration
public class DittoConfiguration {
    
    @Bean
    public Ditto ditto() throws Exception {
        String appId = System.getenv("DITTO_APP_ID");
        String token = System.getenv("DITTO_PLAYGROUND_TOKEN");
        
        if (appId == null || token == null) {
            throw new IllegalStateException("Ditto credentials not configured");
        }
        
        DittoIdentity identity = new DittoIdentity.OnlinePlayground(appId, token, true);
        Ditto ditto = new Ditto(DittoRoot.fromCurrent(), identity);
        ditto.startSync();
        
        return ditto;
    }
    
    @Bean
    public SdkDocumentConverter sdkDocumentConverter() {
        return new SdkDocumentConverter();
    }
}

@Service
public class CotProcessingService {
    
    private final Ditto ditto;
    private final SdkDocumentConverter converter;
    private final Logger logger = LoggerFactory.getLogger(CotProcessingService.class);
    
    public CotProcessingService(Ditto ditto, SdkDocumentConverter converter) {
        this.ditto = ditto;
        this.converter = converter;
        setupObservers();
    }
    
    @Async
    public CompletableFuture<String> processCotEventAsync(String cotXml, String peerId) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                CotEvent event = CotEvent.fromXml(cotXml);
                Map<String, Object> docMap = converter.convertToDocumentMap(event, peerId);
                
                String collection = determineCollection(docMap);
                String query = String.format("INSERT INTO %s DOCUMENTS (?) ON ID CONFLICT DO MERGE", collection);
                
                ditto.getStore().execute(query, docMap);
                
                String docId = converter.getDocumentId(docMap);
                logger.info("Processed CoT event: {} in collection: {}", docId, collection);
                
                return docId;
                
            } catch (Exception e) {
                logger.error("Failed to process CoT event", e);
                throw new RuntimeException("CoT processing failed", e);
            }
        });
    }
    
    public List<Map<String, Object>> queryNearbyUnits(double lat, double lon, double radiusDegrees) {
        try {
            String query = """
                SELECT * FROM map_items 
                WHERE j BETWEEN ? AND ? 
                AND l BETWEEN ? AND ? 
                AND w LIKE 'a-f-%'
                ORDER BY b DESC
                """;
            
            double latMin = lat - radiusDegrees;
            double latMax = lat + radiusDegrees;
            double lonMin = lon - radiusDegrees;
            double lonMax = lon + radiusDegrees;
            
            DittoQueryResult result = ditto.getStore().execute(query, 
                latMin, latMax, lonMin, lonMax);
            
            List<Map<String, Object>> units = new ArrayList<>();
            for (DittoQueryResultItem item : result.getItems()) {
                units.add(item.getValue());
            }
            
            return units;
            
        } catch (Exception e) {
            logger.error("Failed to query nearby units", e);
            return Collections.emptyList();
        }
    }
    
    @Scheduled(fixedRate = 30000) // Every 30 seconds
    public void performHealthCheck() {
        try {
            // Simple query to check Ditto connectivity
            DittoQueryResult result = ditto.getStore().execute("SELECT COUNT(*) as count FROM map_items");
            logger.debug("Health check: {} map items in database", 
                result.getItems().get(0).getValue().get("count"));
            
        } catch (Exception e) {
            logger.warn("Health check failed", e);
        }
    }
    
    private void setupObservers() {
        DittoStore store = ditto.getStore();
        
        // Location updates
        store.registerObserver("SELECT * FROM map_items", (result, event) -> {
            processLocationUpdates(result);
        });
        
        // Chat messages
        store.registerObserver("SELECT * FROM chat_messages ORDER BY b DESC LIMIT 10", (result, event) -> {
            processChatMessages(result);
        });
        
        logger.info("Ditto observers registered");
    }
    
    private void processLocationUpdates(DittoQueryResult result) {
        for (DittoQueryResultItem item : result.getItems()) {
            try {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof MapItemDocument) {
                    MapItemDocument mapItem = (MapItemDocument) typedDoc;
                    // Process location update
                    logger.debug("Location update: {} at {},{}", 
                        mapItem.getE(), mapItem.getJ(), mapItem.getL());
                }
                
            } catch (Exception e) {
                logger.error("Error processing location update", e);
            }
        }
    }
    
    private void processChatMessages(DittoQueryResult result) {
        for (DittoQueryResultItem item : result.getItems()) {
            try {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof ChatDocument) {
                    ChatDocument chat = (ChatDocument) typedDoc;
                    logger.info("Chat message: {} in {}: {}", 
                        chat.getAuthorCallsign(), chat.getRoom(), chat.getMessage());
                }
                
            } catch (Exception e) {
                logger.error("Error processing chat message", e);
            }
        }
    }
    
    private String determineCollection(Map<String, Object> docMap) {
        String docType = converter.getDocumentType(docMap);
        
        return switch (docType) {
            case "MapItem" -> "map_items";
            case "Chat" -> "chat_messages";
            case "File" -> "files";
            case "Api" -> "api_events";
            default -> "api_events";
        };
    }
}

@RestController
@RequestMapping("/api/cot")
public class CotController {
    
    private final CotProcessingService cotService;
    
    public CotController(CotProcessingService cotService) {
        this.cotService = cotService;
    }
    
    @PostMapping("/events")
    public ResponseEntity<Map<String, String>> submitCotEvent(
            @RequestBody String cotXml,
            @RequestParam(defaultValue = "server-peer") String peerId) {
        
        try {
            CompletableFuture<String> future = cotService.processCotEventAsync(cotXml, peerId);
            String docId = future.get(5, TimeUnit.SECONDS); // 5 second timeout
            
            Map<String, String> response = Map.of(
                "status", "success",
                "documentId", docId
            );
            
            return ResponseEntity.ok(response);
            
        } catch (Exception e) {
            Map<String, String> response = Map.of(
                "status", "error",
                "message", e.getMessage()
            );
            
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(response);
        }
    }
    
    @GetMapping("/units/nearby")
    public ResponseEntity<List<Map<String, Object>>> getNearbyUnits(
            @RequestParam double lat,
            @RequestParam double lon,
            @RequestParam(defaultValue = "0.01") double radius) {
        
        List<Map<String, Object>> units = cotService.queryNearbyUnits(lat, lon, radius);
        return ResponseEntity.ok(units);
    }
}
```

## Observer Integration

### Advanced Observer Patterns

```java
import com.ditto.java.DittoStore;
import com.ditto.cot.SdkDocumentConverter;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

public class AdvancedObserverManager {
    private final DittoStore store;
    private final SdkDocumentConverter converter;
    private final ScheduledExecutorService scheduler;
    private final ConcurrentHashMap<String, ObserverRegistration> observers;
    
    public AdvancedObserverManager(Ditto ditto) {
        this.store = ditto.getStore();
        this.converter = new SdkDocumentConverter();
        this.scheduler = Executors.newScheduledThreadPool(4);
        this.observers = new ConcurrentHashMap<>();
    }
    
    public static class ObserverRegistration {
        private final String query;
        private final Consumer<Object> handler;
        private final Class<?> documentType;
        private volatile boolean active = true;
        
        public ObserverRegistration(String query, Consumer<Object> handler, Class<?> documentType) {
            this.query = query;
            this.handler = handler;
            this.documentType = documentType;
        }
        
        public void deactivate() {
            this.active = false;
        }
        
        public boolean isActive() {
            return active;
        }
    }
    
    public String registerLocationObserver(Consumer<MapItemDocument> handler) {
        return registerObserver(
            "SELECT * FROM map_items WHERE w LIKE 'a-f-%'",
            doc -> {
                if (doc instanceof MapItemDocument) {
                    handler.accept((MapItemDocument) doc);
                }
            },
            MapItemDocument.class
        );
    }
    
    public String registerChatObserver(String room, Consumer<ChatDocument> handler) {
        String query = room != null ? 
            "SELECT * FROM chat_messages WHERE room = ? ORDER BY b DESC LIMIT 20" :
            "SELECT * FROM chat_messages ORDER BY b DESC LIMIT 20";
            
        return registerObserver(
            query,
            doc -> {
                if (doc instanceof ChatDocument) {
                    ChatDocument chat = (ChatDocument) doc;
                    if (room == null || room.equals(chat.getRoom())) {
                        handler.accept(chat);
                    }
                }
            },
            ChatDocument.class
        );
    }
    
    public String registerEmergencyObserver(Consumer<ApiDocument> handler) {
        return registerObserver(
            "SELECT * FROM api_events WHERE w LIKE 'b-a-%'",
            doc -> {
                if (doc instanceof ApiDocument) {
                    handler.accept((ApiDocument) doc);
                }
            },
            ApiDocument.class
        );
    }
    
    public String registerFilteredObserver(String collection, String filter, 
                                         Consumer<Object> handler, Class<?> documentType) {
        String query = String.format("SELECT * FROM %s WHERE %s", collection, filter);
        return registerObserver(query, handler, documentType);
    }
    
    private String registerObserver(String query, Consumer<Object> handler, Class<?> documentType) {
        String observerId = "observer-" + System.currentTimeMillis() + "-" + 
                           Thread.currentThread().getId();
        
        ObserverRegistration registration = new ObserverRegistration(query, handler, documentType);
        observers.put(observerId, registration);
        
        // Register with Ditto
        store.registerObserver(query, (result, event) -> {
            if (!registration.isActive()) {
                return; // Observer has been deactivated
            }
            
            // Process in background thread to avoid blocking
            scheduler.submit(() -> {
                try {
                    for (DittoQueryResultItem item : result.getItems()) {
                        Map<String, Object> docMap = item.getValue();
                        Object typedDoc = converter.observerMapToTypedDocument(docMap);
                        
                        if (typedDoc != null && registration.documentType.isInstance(typedDoc)) {
                            registration.handler.accept(typedDoc);
                        }
                    }
                } catch (Exception e) {
                    System.err.println("Observer processing error: " + e.getMessage());
                }
            });
        });
        
        System.out.println("Registered observer: " + observerId + " for query: " + query);
        return observerId;
    }
    
    public void unregisterObserver(String observerId) {
        ObserverRegistration registration = observers.remove(observerId);
        if (registration != null) {
            registration.deactivate();
            System.out.println("Unregistered observer: " + observerId);
        }
    }
    
    public void registerPeriodicLocationQuery(double lat, double lon, double radius, 
                                            Duration interval, Consumer<List<MapItemDocument>> handler) {
        
        scheduler.scheduleAtFixedRate(() -> {
            try {
                String query = """
                    SELECT * FROM map_items 
                    WHERE j BETWEEN ? AND ? 
                    AND l BETWEEN ? AND ?
                    ORDER BY b DESC
                    """;
                
                double latMin = lat - radius;
                double latMax = lat + radius;
                double lonMin = lon - radius;
                double lonMax = lon + radius;
                
                DittoQueryResult result = store.execute(query, latMin, latMax, lonMin, lonMax);
                
                List<MapItemDocument> locations = new ArrayList<>();
                for (DittoQueryResultItem item : result.getItems()) {
                    Map<String, Object> docMap = item.getValue();
                    Object typedDoc = converter.observerMapToTypedDocument(docMap);
                    
                    if (typedDoc instanceof MapItemDocument) {
                        locations.add((MapItemDocument) typedDoc);
                    }
                }
                
                handler.accept(locations);
                
            } catch (Exception e) {
                System.err.println("Periodic location query error: " + e.getMessage());
            }
        }, 0, interval.toMillis(), TimeUnit.MILLISECONDS);
    }
    
    public void shutdown() {
        // Deactivate all observers
        observers.values().forEach(ObserverRegistration::deactivate);
        observers.clear();
        
        // Shutdown scheduler
        scheduler.shutdown();
        try {
            if (!scheduler.awaitTermination(5, TimeUnit.SECONDS)) {
                scheduler.shutdownNow();
            }
        } catch (InterruptedException e) {
            scheduler.shutdownNow();
            Thread.currentThread().interrupt();
        }
    }
}

// Usage example
public class ObserverUsageExample {
    public void demonstrateObservers() {
        AdvancedObserverManager observerManager = new AdvancedObserverManager(ditto);
        
        // Location observer
        String locationObserverId = observerManager.registerLocationObserver(mapItem -> {
            System.out.println("📍 " + mapItem.getE() + " moved to " + 
                             mapItem.getJ() + "," + mapItem.getL());
            
            // Update map display
            updateMapDisplay(mapItem);
        });
        
        // Chat observer for specific room
        String chatObserverId = observerManager.registerChatObserver("Command Net", chat -> {
            System.out.println("💬 [" + chat.getRoom() + "] " + 
                             chat.getAuthorCallsign() + ": " + chat.getMessage());
            
            // Update chat UI
            updateChatDisplay(chat);
        });
        
        // Emergency observer
        String emergencyObserverId = observerManager.registerEmergencyObserver(emergency -> {
            System.out.println("🚨 EMERGENCY: " + emergency.getE());
            
            // Trigger alerts
            handleEmergency(emergency);
        });
        
        // Periodic location tracking
        observerManager.registerPeriodicLocationQuery(
            34.0522, -118.2437, 0.01,  // LA area, ~1km radius
            Duration.ofSeconds(30),      // Every 30 seconds
            locations -> {
                System.out.println("Found " + locations.size() + " units in area");
                updateTacticalPicture(locations);
            }
        );
        
        // Later, unregister observers
        // observerManager.unregisterObserver(locationObserverId);
        // observerManager.shutdown();
    }
}
```

## Concurrency and Threading

### Thread-Safe CoT Processing

```java
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.locks.ReentrantReadWriteLock;

public class ThreadSafeCotProcessor {
    private final Ditto ditto;
    private final SdkDocumentConverter converter;
    private final ExecutorService processingPool;
    private final ExecutorService observerPool;
    private final ConcurrentHashMap<String, CotDocument> documentCache;
    private final ReentrantReadWriteLock cacheLock;
    private final AtomicLong processedCount;
    private final AtomicLong errorCount;
    
    public ThreadSafeCotProcessor(Ditto ditto, int processingThreads, int observerThreads) {
        this.ditto = ditto;
        this.converter = new SdkDocumentConverter();
        this.processingPool = Executors.newFixedThreadPool(processingThreads);
        this.observerPool = Executors.newFixedThreadPool(observerThreads);
        this.documentCache = new ConcurrentHashMap<>();
        this.cacheLock = new ReentrantReadWriteLock();
        this.processedCount = new AtomicLong(0);
        this.errorCount = new AtomicLong(0);
    }
    
    public CompletableFuture<ProcessingResult> processCotEventAsync(String cotXml, String peerId) {
        return CompletableFuture.supplyAsync(() -> {
            try {
                long startTime = System.currentTimeMillis();
                
                // Parse CoT event
                CotEvent event = CotEvent.fromXml(cotXml);
                
                // Check cache first
                String cacheKey = generateCacheKey(event, peerId);
                CotDocument cachedDoc = getCachedDocument(cacheKey);
                
                if (cachedDoc != null) {
                    return new ProcessingResult(true, extractDocumentId(cachedDoc), 
                                              System.currentTimeMillis() - startTime, true);
                }
                
                // Convert to document
                Map<String, Object> docMap = converter.convertToDocumentMap(event, peerId);
                
                // Store in Ditto
                String collection = determineCollection(docMap);
                String query = String.format("INSERT INTO %s DOCUMENTS (?) ON ID CONFLICT DO MERGE", collection);
                
                ditto.getStore().execute(query, docMap);
                
                // Cache the result
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                if (typedDoc instanceof CotDocument) {
                    cachePutDocument(cacheKey, (CotDocument) typedDoc);
                }
                
                String docId = converter.getDocumentId(docMap);
                long processingTime = System.currentTimeMillis() - startTime;
                
                processedCount.incrementAndGet();
                return new ProcessingResult(true, docId, processingTime, false);
                
            } catch (Exception e) {
                errorCount.incrementAndGet();
                return new ProcessingResult(false, null, 0, false, e.getMessage());
            }
        }, processingPool);
    }
    
    public CompletableFuture<List<ProcessingResult>> batchProcessAsync(List<String> cotXmlList, String peerId) {
        List<CompletableFuture<ProcessingResult>> futures = cotXmlList.stream()
            .map(xml -> processCotEventAsync(xml, peerId))
            .collect(Collectors.toList());
        
        return CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
            .thenApply(v -> futures.stream()
                .map(CompletableFuture::join)
                .collect(Collectors.toList()));
    }
    
    public void startObserverProcessing() {
        DittoStore store = ditto.getStore();
        
        // Process location updates in dedicated thread
        observerPool.submit(() -> {
            store.registerObserver("SELECT * FROM map_items", (result, event) -> {
                observerPool.submit(() -> processLocationUpdates(result));
            });
        });
        
        // Process chat messages in dedicated thread
        observerPool.submit(() -> {
            store.registerObserver("SELECT * FROM chat_messages ORDER BY b DESC LIMIT 50", (result, event) -> {
                observerPool.submit(() -> processChatMessages(result));
            });
        });
        
        // Process emergency events with high priority
        observerPool.submit(() -> {
            store.registerObserver("SELECT * FROM api_events WHERE w LIKE 'b-a-%'", (result, event) -> {
                // Use separate thread pool for high-priority emergency processing
                CompletableFuture.runAsync(() -> processEmergencyEvents(result));
            });
        });
    }
    
    private CotDocument getCachedDocument(String key) {
        cacheLock.readLock().lock();
        try {
            return documentCache.get(key);
        } finally {
            cacheLock.readLock().unlock();
        }
    }
    
    private void cachePutDocument(String key, CotDocument document) {
        cacheLock.writeLock().lock();
        try {
            documentCache.put(key, document);
            
            // Implement cache size limit
            if (documentCache.size() > 10000) {
                // Remove oldest entries (simple LRU-like behavior)
                documentCache.entrySet().stream()
                    .limit(1000)
                    .map(Map.Entry::getKey)
                    .forEach(documentCache::remove);
            }
        } finally {
            cacheLock.writeLock().unlock();
        }
    }
    
    private void processLocationUpdates(DittoQueryResult result) {
        try {
            for (DittoQueryResultItem item : result.getItems()) {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof MapItemDocument) {
                    MapItemDocument mapItem = (MapItemDocument) typedDoc;
                    
                    // Thread-safe processing
                    synchronized (this) {
                        // Update application state
                        updateLocationState(mapItem);
                    }
                }
            }
        } catch (Exception e) {
            System.err.println("Error processing location updates: " + e.getMessage());
        }
    }
    
    private void processChatMessages(DittoQueryResult result) {
        // Similar thread-safe processing for chat messages
        try {
            for (DittoQueryResultItem item : result.getItems()) {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof ChatDocument) {
                    ChatDocument chat = (ChatDocument) typedDoc;
                    
                    // Process chat message safely
                    processChatMessageSafely(chat);
                }
            }
        } catch (Exception e) {
            System.err.println("Error processing chat messages: " + e.getMessage());
        }
    }
    
    private void processEmergencyEvents(DittoQueryResult result) {
        // High-priority emergency processing
        try {
            for (DittoQueryResultItem item : result.getItems()) {
                Map<String, Object> docMap = item.getValue();
                Object typedDoc = converter.observerMapToTypedDocument(docMap);
                
                if (typedDoc instanceof ApiDocument) {
                    ApiDocument emergency = (ApiDocument) typedDoc;
                    
                    // High-priority processing
                    processEmergencyWithPriority(emergency);
                }
            }
        } catch (Exception e) {
            System.err.println("Error processing emergency events: " + e.getMessage());
        }
    }
    
    public ProcessingStats getStats() {
        return new ProcessingStats(
            processedCount.get(),
            errorCount.get(),
            documentCache.size(),
            ((ThreadPoolExecutor) processingPool).getActiveCount(),
            ((ThreadPoolExecutor) observerPool).getActiveCount()
        );
    }
    
    public void shutdown() {
        processingPool.shutdown();
        observerPool.shutdown();
        
        try {
            if (!processingPool.awaitTermination(10, TimeUnit.SECONDS)) {
                processingPool.shutdownNow();
            }
            if (!observerPool.awaitTermination(10, TimeUnit.SECONDS)) {
                observerPool.shutdownNow();
            }
        } catch (InterruptedException e) {
            processingPool.shutdownNow();
            observerPool.shutdownNow();
            Thread.currentThread().interrupt();
        }
    }
    
    // Helper classes
    public static class ProcessingResult {
        public final boolean success;
        public final String documentId;
        public final long processingTimeMs;
        public final boolean cacheHit;
        public final String errorMessage;
        
        public ProcessingResult(boolean success, String documentId, long processingTimeMs, boolean cacheHit) {
            this(success, documentId, processingTimeMs, cacheHit, null);
        }
        
        public ProcessingResult(boolean success, String documentId, long processingTimeMs, boolean cacheHit, String errorMessage) {
            this.success = success;
            this.documentId = documentId;
            this.processingTimeMs = processingTimeMs;
            this.cacheHit = cacheHit;
            this.errorMessage = errorMessage;
        }
    }
    
    public static class ProcessingStats {
        public final long processedCount;
        public final long errorCount;
        public final int cacheSize;
        public final int activeProcessingThreads;
        public final int activeObserverThreads;
        
        public ProcessingStats(long processedCount, long errorCount, int cacheSize, 
                             int activeProcessingThreads, int activeObserverThreads) {
            this.processedCount = processedCount;
            this.errorCount = errorCount;
            this.cacheSize = cacheSize;
            this.activeProcessingThreads = activeProcessingThreads;
            this.activeObserverThreads = activeObserverThreads;
        }
    }
}
```

## Error Handling

### Comprehensive Error Management

```java
import java.util.function.Supplier;
import java.util.function.Function;

public class RobustCotErrorHandler {
    
    public enum ErrorCategory {
        XML_PARSING,
        DITTO_OPERATION,
        NETWORK_ISSUE,
        VALIDATION_ERROR,
        CONVERSION_ERROR,
        UNKNOWN
    }
    
    public static class CotProcessingException extends Exception {
        private final ErrorCategory category;
        private final String cotXml;
        private final long timestamp;
        
        public CotProcessingException(ErrorCategory category, String message, String cotXml, Throwable cause) {
            super(message, cause);
            this.category = category;
            this.cotXml = cotXml;
            this.timestamp = System.currentTimeMillis();
        }
        
        public ErrorCategory getCategory() { return category; }
        public String getCotXml() { return cotXml; }
        public long getTimestamp() { return timestamp; }
    }
    
    public static class RetryConfig {
        public final int maxRetries;
        public final long initialDelayMs;
        public final long maxDelayMs;
        public final double backoffMultiplier;
        public final Set<ErrorCategory> retryableCategories;
        
        public RetryConfig(int maxRetries, long initialDelayMs, long maxDelayMs, 
                          double backoffMultiplier, Set<ErrorCategory> retryableCategories) {
            this.maxRetries = maxRetries;
            this.initialDelayMs = initialDelayMs;
            this.maxDelayMs = maxDelayMs;
            this.backoffMultiplier = backoffMultiplier;
            this.retryableCategories = retryableCategories;
        }
        
        public static RetryConfig defaultConfig() {
            return new RetryConfig(
                3, 1000, 30000, 2.0,
                Set.of(ErrorCategory.NETWORK_ISSUE, ErrorCategory.DITTO_OPERATION)
            );
        }
    }
    
    private final RetryConfig retryConfig;
    private final ConcurrentHashMap<ErrorCategory, AtomicLong> errorCounts;
    private final List<CotProcessingException> recentErrors;
    private final ReentrantReadWriteLock errorLogLock;
    
    public RobustCotErrorHandler(RetryConfig retryConfig) {
        this.retryConfig = retryConfig;
        this.errorCounts = new ConcurrentHashMap<>();
        this.recentErrors = new ArrayList<>();
        this.errorLogLock = new ReentrantReadWriteLock();
        
        // Initialize error counters
        for (ErrorCategory category : ErrorCategory.values()) {
            errorCounts.put(category, new AtomicLong(0));
        }
    }
    
    public <T> T executeWithRetry(Supplier<T> operation, String cotXml, String operationName) throws CotProcessingException {
        return executeWithRetry(operation, cotXml, operationName, Function.identity());
    }
    
    public <T> T executeWithRetry(Supplier<T> operation, String cotXml, String operationName, 
                                 Function<Exception, ErrorCategory> categoryMapper) throws CotProcessingException {
        
        Exception lastException = null;
        long delay = retryConfig.initialDelayMs;
        
        for (int attempt = 0; attempt <= retryConfig.maxRetries; attempt++) {
            try {
                return operation.get();
                
            } catch (Exception e) {
                lastException = e;
                ErrorCategory category = categoryMapper.apply(e);
                
                // Log error
                logError(category, operationName, e, cotXml);
                
                // Check if retryable
                if (attempt >= retryConfig.maxRetries || !retryConfig.retryableCategories.contains(category)) {
                    break;
                }
                
                // Wait before retry
                try {
                    Thread.sleep(delay + (long)(Math.random() * 1000)); // Add jitter
                    delay = Math.min((long)(delay * retryConfig.backoffMultiplier), retryConfig.maxDelayMs);
                } catch (InterruptedException ie) {
                    Thread.currentThread().interrupt();
                    throw new CotProcessingException(ErrorCategory.UNKNOWN, 
                        "Operation interrupted", cotXml, ie);
                }
            }
        }
        
        // All retries exhausted
        ErrorCategory category = categoryMapper.apply(lastException);
        throw new CotProcessingException(category, 
            String.format("Operation '%s' failed after %d attempts", operationName, retryConfig.maxRetries + 1),
            cotXml, lastException);
    }
    
    public String processCotEventWithErrorHandling(String cotXml, String peerId, Ditto ditto, SdkDocumentConverter converter) {
        try {
            return executeWithRetry(() -> {
                try {
                    // Parse CoT XML
                    CotEvent event = CotEvent.fromXml(cotXml);
                    
                    // Validate event
                    validateCotEvent(event);
                    
                    // Convert to document
                    Map<String, Object> docMap = converter.convertToDocumentMap(event, peerId);
                    
                    // Validate document
                    validateDocument(docMap);
                    
                    // Store in Ditto
                    String collection = determineCollection(docMap);
                    String query = String.format("INSERT INTO %s DOCUMENTS (?) ON ID CONFLICT DO MERGE", collection);
                    
                    ditto.getStore().execute(query, docMap);
                    
                    return converter.getDocumentId(docMap);
                    
                } catch (Exception e) {
                    throw new RuntimeException(e);
                }
            }, cotXml, "processCotEvent", this::categorizeException);
            
        } catch (CotProcessingException e) {
            // Handle different error categories
            switch (e.getCategory()) {
                case XML_PARSING:
                    // Log and possibly attempt XML repair
                    return handleXmlParsingError(e);
                    
                case VALIDATION_ERROR:
                    // Log validation issues but don't retry
                    return handleValidationError(e);
                    
                case DITTO_OPERATION:
                    // Network or Ditto-specific error
                    return handleDittoError(e);
                    
                case NETWORK_ISSUE:
                    // Network connectivity problems
                    return handleNetworkError(e);
                    
                default:
                    // Unknown error category
                    return handleUnknownError(e);
            }
        }
    }
    
    private ErrorCategory categorizeException(Exception e) {
        String message = e.getMessage().toLowerCase();
        
        if (e instanceof javax.xml.bind.JAXBException || 
            message.contains("xml") || message.contains("parse")) {
            return ErrorCategory.XML_PARSING;
        }
        
        if (message.contains("validation") || message.contains("invalid")) {
            return ErrorCategory.VALIDATION_ERROR;
        }
        
        if (message.contains("ditto") || message.contains("query") || message.contains("execute")) {
            return ErrorCategory.DITTO_OPERATION;
        }
        
        if (message.contains("network") || message.contains("connection") || 
            message.contains("timeout") || e instanceof java.net.ConnectException) {
            return ErrorCategory.NETWORK_ISSUE;
        }
        
        if (message.contains("conversion") || message.contains("document")) {
            return ErrorCategory.CONVERSION_ERROR;
        }
        
        return ErrorCategory.UNKNOWN;
    }
    
    private void validateCotEvent(CotEvent event) throws ValidationException {
        if (event.getUid() == null || event.getUid().trim().isEmpty()) {
            throw new ValidationException("CoT event UID is required");
        }
        
        if (event.getType() == null || event.getType().trim().isEmpty()) {
            throw new ValidationException("CoT event type is required");
        }
        
        if (event.getPoint() != null) {
            double lat = event.getPoint().getLat();
            double lon = event.getPoint().getLon();
            
            if (lat < -90.0 || lat > 90.0) {
                throw new ValidationException("Invalid latitude: " + lat);
            }
            
            if (lon < -180.0 || lon > 180.0) {
                throw new ValidationException("Invalid longitude: " + lon);
            }
        }
    }
    
    private void validateDocument(Map<String, Object> docMap) throws ValidationException {
        Object id = docMap.get("_id");
        if (id == null || id.toString().trim().isEmpty()) {
            throw new ValidationException("Document ID is required");
        }
        
        // Add more validation as needed
    }
    
    private String handleXmlParsingError(CotProcessingException e) {
        System.err.println("XML parsing failed: " + e.getMessage());
        
        // Could attempt XML repair or provide fallback
        // For now, just return error indicator
        return "ERROR_XML_PARSE";
    }
    
    private String handleValidationError(CotProcessingException e) {
        System.err.println("Validation failed: " + e.getMessage());
        return "ERROR_VALIDATION";
    }
    
    private String handleDittoError(CotProcessingException e) {
        System.err.println("Ditto operation failed: " + e.getMessage());
        return "ERROR_DITTO_OP";
    }
    
    private String handleNetworkError(CotProcessingException e) {
        System.err.println("Network error: " + e.getMessage());
        return "ERROR_NETWORK";
    }
    
    private String handleUnknownError(CotProcessingException e) {
        System.err.println("Unknown error: " + e.getMessage());
        return "ERROR_UNKNOWN";
    }
    
    private void logError(ErrorCategory category, String operation, Exception e, String cotXml) {
        errorCounts.get(category).incrementAndGet();
        
        CotProcessingException cotError = new CotProcessingException(category, 
            operation + " failed: " + e.getMessage(), cotXml, e);
        
        errorLogLock.writeLock().lock();
        try {
            recentErrors.add(cotError);
            
            // Keep only recent errors (last 100)
            if (recentErrors.size() > 100) {
                recentErrors.remove(0);
            }
        } finally {
            errorLogLock.writeLock().unlock();
        }
        
        System.err.printf("[%s] %s: %s%n", category, operation, e.getMessage());
    }
    
    public Map<ErrorCategory, Long> getErrorCounts() {
        return errorCounts.entrySet().stream()
            .collect(Collectors.toMap(
                Map.Entry::getKey,
                entry -> entry.getValue().get()
            ));
    }
    
    public List<CotProcessingException> getRecentErrors() {
        errorLogLock.readLock().lock();
        try {
            return new ArrayList<>(recentErrors);
        } finally {
            errorLogLock.readLock().unlock();
        }
    }
    
    public void clearErrorHistory() {
        errorLogLock.writeLock().lock();
        try {
            recentErrors.clear();
            errorCounts.values().forEach(counter -> counter.set(0));
        } finally {
            errorLogLock.writeLock().unlock();
        }
    }
}
```

## Testing Patterns

### Comprehensive Test Examples

```java
import org.junit.jupiter.api.*;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;
import static org.mockito.Mockito.*;
import static org.junit.jupiter.api.Assertions.*;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.TimeUnit;

@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class CotIntegrationTest {
    
    @Mock
    private Ditto mockDitto;
    
    @Mock
    private DittoStore mockStore;
    
    private SdkDocumentConverter converter;
    private CotProcessingService cotService;
    
    @BeforeAll
    void setup() {
        MockitoAnnotations.openMocks(this);
        when(mockDitto.getStore()).thenReturn(mockStore);
        
        converter = new SdkDocumentConverter();
        cotService = new CotProcessingService(mockDitto, converter);
    }
    
    @Test
    void testBasicCotEventCreation() {
        // Test builder pattern
        CotEvent event = CotEvent.builder()
            .uid("TEST-001")
            .type("a-f-G-U-C")
            .time(Instant.now())
            .point(34.0, -118.0, 100.0)
            .detail()
                .callsign("TEST-UNIT")
                .groupName("Blue")
                .build()
            .build();
        
        assertNotNull(event);
        assertEquals("TEST-001", event.getUid());
        assertEquals("a-f-G-U-C", event.getType());
        assertNotNull(event.getPoint());
        assertEquals(34.0, event.getPoint().getLat());
        assertEquals(-118.0, event.getPoint().getLon());
    }
    
    @Test
    void testXmlRoundTrip() throws Exception {
        String originalXml = """
            <event version="2.0" uid="TEST-123" type="a-f-G-U-C" 
                   time="2024-01-15T10:30:00.000Z" 
                   start="2024-01-15T10:30:00.000Z" 
                   stale="2024-01-15T10:35:00.000Z">
                <point lat="34.0" lon="-118.0" hae="100.0"/>
                <detail><contact callsign="TEST"/></detail>
            </event>
            """;
        
        // Parse XML
        CotEvent event = CotEvent.fromXml(originalXml);
        assertNotNull(event);
        assertEquals("TEST-123", event.getUid());
        
        // Convert back to XML
        String regeneratedXml = event.toXml();
        assertNotNull(regeneratedXml);
        
        // Parse again to verify
        CotEvent reparsedEvent = CotEvent.fromXml(regeneratedXml);
        assertEquals(event.getUid(), reparsedEvent.getUid());
        assertEquals(event.getType(), reparsedEvent.getType());
    }
    
    @Test
    void testDocumentConversion() throws Exception {
        CotEvent event = CotEvent.builder()
            .uid("CONV-TEST-001")
            .type("a-f-G-U-C")
            .point(34.0, -118.0, 100.0)
            .detail()
                .callsign("CONV-TEST")
                .build()
            .build();
        
        Map<String, Object> docMap = converter.convertToDocumentMap(event, "test-peer");
        assertNotNull(docMap);
        
        String docId = converter.getDocumentId(docMap);
        assertEquals("CONV-TEST-001", docId);
        
        String docType = converter.getDocumentType(docMap);
        assertEquals("MapItem", docType);
        
        Object typedDoc = converter.observerMapToTypedDocument(docMap);
        assertNotNull(typedDoc);
        assertTrue(typedDoc instanceof MapItemDocument);
        
        MapItemDocument mapItem = (MapItemDocument) typedDoc;
        assertEquals("CONV-TEST-001", mapItem.getId());
        assertEquals("CONV-TEST", mapItem.getE());
    }
    
    @Test
    void testAsyncProcessing() throws Exception {
        // Mock Ditto store behavior
        when(mockStore.execute(anyString(), any())).thenReturn(mock(DittoQueryResult.class));
        
        String cotXml = """
            <event version="2.0" uid="ASYNC-001" type="a-f-G-U-C">
                <point lat="34.0" lon="-118.0" hae="100.0"/>
                <detail><contact callsign="ASYNC-TEST"/></detail>
            </event>
            """;
        
        CompletableFuture<String> future = cotService.processCotEventAsync(cotXml, "test-peer");
        
        // Should complete within reasonable time
        String result = future.get(5, TimeUnit.SECONDS);
        assertNotNull(result);
        
        // Verify Ditto store was called
        verify(mockStore, times(1)).execute(anyString(), any());
    }
    
    @Test
    void testErrorHandling() {
        // Test invalid XML
        String invalidXml = "<invalid><xml>";
        
        assertThrows(Exception.class, () -> {
            CotEvent.fromXml(invalidXml);
        });
        
        // Test missing required fields
        assertThrows(Exception.class, () -> {
            CotEvent.builder()
                .uid("")  // Empty UID should fail
                .type("a-f-G-U-C")
                .build();
        });
    }
    
    @Test
    void testConcurrentProcessing() throws Exception {
        // Mock successful operations
        when(mockStore.execute(anyString(), any())).thenReturn(mock(DittoQueryResult.class));
        
        List<String> xmlList = Arrays.asList(
            createTestXml("CONCURRENT-001", "TEST-1"),
            createTestXml("CONCURRENT-002", "TEST-2"),
            createTestXml("CONCURRENT-003", "TEST-3")
        );
        
        ThreadSafeCotProcessor processor = new ThreadSafeCotProcessor(mockDitto, 4, 2);
        
        CompletableFuture<List<ThreadSafeCotProcessor.ProcessingResult>> future = 
            processor.batchProcessAsync(xmlList, "test-peer");
        
        List<ThreadSafeCotProcessor.ProcessingResult> results = future.get(10, TimeUnit.SECONDS);
        
        assertEquals(3, results.size());
        assertTrue(results.stream().allMatch(r -> r.success));
        
        processor.shutdown();
    }
    
    @Test
    void testObserverIntegration() {
        AdvancedObserverManager observerManager = new AdvancedObserverManager(mockDitto);
        
        // Test observer registration
        AtomicInteger locationUpdateCount = new AtomicInteger(0);
        String observerId = observerManager.registerLocationObserver(mapItem -> {
            locationUpdateCount.incrementAndGet();
        });
        
        assertNotNull(observerId);
        
        // Simulate observer callback
        // (In real test, you'd trigger actual Ditto observer)
        
        observerManager.unregisterObserver(observerId);
        observerManager.shutdown();
    }
    
    @Test
    void testRetryMechanism() throws Exception {
        RobustCotErrorHandler errorHandler = new RobustCotErrorHandler(
            RobustCotErrorHandler.RetryConfig.defaultConfig()
        );
        
        // Mock intermittent failures
        AtomicInteger attemptCount = new AtomicInteger(0);
        
        String result = errorHandler.executeWithRetry(() -> {
            int attempt = attemptCount.incrementAndGet();
            if (attempt < 3) {
                throw new RuntimeException("Simulated failure");
            }
            return "SUCCESS";
        }, "<test/>", "testOperation");
        
        assertEquals("SUCCESS", result);
        assertEquals(3, attemptCount.get()); // Should have retried twice
    }
    
    private String createTestXml(String uid, String callsign) {
        return String.format("""
            <event version="2.0" uid="%s" type="a-f-G-U-C">
                <point lat="34.0" lon="-118.0" hae="100.0"/>
                <detail><contact callsign="%s"/></detail>
            </event>
            """, uid, callsign);
    }
    
    @Test
    void testPerformanceMetrics() throws Exception {
        ThreadSafeCotProcessor processor = new ThreadSafeCotProcessor(mockDitto, 2, 1);
        
        // Mock successful operations
        when(mockStore.execute(anyString(), any())).thenReturn(mock(DittoQueryResult.class));
        
        // Process some events
        String testXml = createTestXml("PERF-001", "PERF-TEST");
        processor.processCotEventAsync(testXml, "test-peer").get();
        
        ThreadSafeCotProcessor.ProcessingStats stats = processor.getStats();
        
        assertTrue(stats.processedCount >= 1);
        assertEquals(0, stats.errorCount);
        
        processor.shutdown();
    }
}

// Integration test with actual Ditto (requires credentials)
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
@Disabled("Requires Ditto credentials")
class DittoIntegrationTest {
    
    private Ditto ditto;
    private SdkDocumentConverter converter;
    
    @BeforeAll
    void setupDitto() throws Exception {
        String appId = System.getenv("DITTO_APP_ID");
        String token = System.getenv("DITTO_PLAYGROUND_TOKEN");
        
        assumeTrue(appId != null && token != null, "Ditto credentials required");
        
        DittoIdentity identity = new DittoIdentity.OnlinePlayground(appId, token, true);
        ditto = new Ditto(DittoRoot.fromCurrent(), identity);
        ditto.startSync();
        
        converter = new SdkDocumentConverter();
        
        // Wait for initial sync
        Thread.sleep(2000);
    }
    
    @Test
    void testRealDittoIntegration() throws Exception {
        String testXml = createTestXml("REAL-DITTO-001", "REAL-TEST");
        
        CotEvent event = CotEvent.fromXml(testXml);
        Map<String, Object> docMap = converter.convertToDocumentMap(event, "integration-test-peer");
        
        String query = "INSERT INTO map_items DOCUMENTS (?) ON ID CONFLICT DO MERGE";
        ditto.getStore().execute(query, docMap);
        
        // Wait for sync
        Thread.sleep(1000);
        
        // Query back
        DittoQueryResult result = ditto.getStore().execute("SELECT * FROM map_items WHERE _id = ?", "REAL-DITTO-001");
        
        assertFalse(result.getItems().isEmpty());
        
        Map<String, Object> retrieved = result.getItems().get(0).getValue();
        assertEquals("REAL-DITTO-001", retrieved.get("_id"));
    }
    
    @AfterAll
    void teardownDitto() {
        if (ditto != null) {
            ditto.stopSync();
        }
    }
}
```

These comprehensive Java examples demonstrate enterprise-ready integration patterns for the Ditto CoT library, covering Android development, Spring Boot integration, advanced observer patterns, thread-safe processing, robust error handling, and thorough testing strategies.