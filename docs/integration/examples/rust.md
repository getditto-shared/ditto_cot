# Rust Integration Examples

This guide provides comprehensive examples for integrating the Ditto CoT library in Rust applications, focusing on idiomatic Rust patterns and performance optimization.

## Table of Contents

- [Basic Integration](#basic-integration)
- [Advanced Builder Patterns](#advanced-builder-patterns)
- [Async Ditto Integration](#async-ditto-integration)
- [Observer Patterns](#observer-patterns)
- [Error Handling](#error-handling)
- [Performance Optimization](#performance-optimization)
- [Testing Patterns](#testing-patterns)

## Basic Integration

### Simple CoT Event Creation

```rust
use ditto_cot::{cot_events::CotEvent, ditto::cot_to_document};
use chrono::{DateTime, Utc, Duration};
use std::error::Error;

fn create_location_update() -> Result<CotEvent, Box<dyn Error>> {
    let event = CotEvent::builder()
        .uid("RUST-UNIT-001")
        .event_type("a-f-G-U-C")  // Friendly ground unit
        .location(34.052235, -118.243683, 100.0)  // Los Angeles
        .callsign("RUST-ALPHA")
        .team("Blue")
        .stale_in(Duration::minutes(5))
        .build();
    
    Ok(event)
}

fn convert_to_ditto_document(event: &CotEvent, peer_id: &str) -> Result<String, Box<dyn Error>> {
    let doc = cot_to_document(event, peer_id);
    let json = serde_json::to_string_pretty(&doc)?;
    Ok(json)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let event = create_location_update()?;
    let doc_json = convert_to_ditto_document(&event, "rust-peer-123")?;
    
    println!("Created Ditto document:\n{}", doc_json);
    Ok(())
}
```

### XML Processing

```rust
use ditto_cot::cot_events::CotEvent;

fn process_cot_xml(xml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse XML to CotEvent
    let event = CotEvent::from_xml(xml_content)?;
    
    println!("Parsed event:");
    println!("  UID: {}", event.uid);
    println!("  Type: {}", event.event_type);
    println!("  Time: {}", event.time);
    
    if let Some(point) = &event.point {
        println!("  Location: {}, {} at {}m", point.lat, point.lon, point.hae);
        println!("  Accuracy: CE={}m, LE={}m", point.ce, point.le);
    }
    
    // Convert back to XML
    let regenerated_xml = event.to_xml()?;
    println!("Regenerated XML:\n{}", regenerated_xml);
    
    Ok(())
}

// Example usage with complex CoT XML
fn example_complex_cot() -> Result<(), Box<dyn std::error::Error>> {
    let complex_xml = r#"
    <event version="2.0" uid="COMPLEX-001" type="a-f-G-U-C" 
           time="2024-01-15T10:30:00.000Z" 
           start="2024-01-15T10:30:00.000Z" 
           stale="2024-01-15T10:35:00.000Z" 
           how="h-g-i-g-o">
        <point lat="34.052235" lon="-118.243683" hae="100.0" ce="5.0" le="10.0"/>
        <detail>
            <contact callsign="RUST-ALPHA"/>
            <__group name="Blue" role="Team Leader"/>
            <status readiness="true"/>
            <track speed="15.0" course="90.0"/>
        </detail>
    </event>"#;
    
    process_cot_xml(complex_xml)
}
```

## Advanced Builder Patterns

### Tactical Event Creation

```rust
use ditto_cot::cot_events::{CotEvent, Point};
use chrono::{Duration, Utc};

fn create_tactical_events() -> Result<Vec<CotEvent>, Box<dyn std::error::Error>> {
    let mut events = Vec::new();
    
    // Sniper position with high accuracy
    let sniper_position = CotEvent::builder()
        .uid("SNIPER-007")
        .event_type("a-f-G-U-C-I")  // Infantry unit
        .location_with_accuracy(
            34.068921, -118.445181, 300.0,  // Position  
            2.0, 5.0  // CE: 2m horizontal, LE: 5m vertical
        )
        .callsign_and_team("OVERWATCH", "Green")
        .how("h-g-i-g-o")  // Human-generated GPS
        .stale_in(Duration::minutes(15))
        .detail(r#"<detail>
            <contact callsign="OVERWATCH"/>
            <__group name="Green" role="Sniper"/>
            <status readiness="true"/>
            <takv device="PRC-152" platform="Android" version="4.8.1"/>
        </detail>"#)
        .build();
    
    // Emergency beacon
    let emergency_beacon = CotEvent::builder()
        .uid("EMERGENCY-123")
        .event_type("b-a-o-can")  // Emergency beacon
        .location(34.073620, -118.240000, 50.0)
        .callsign("RESCUE-1")
        .stale_in(Duration::minutes(30))
        .detail(r#"<detail>
            <emergency type="Medical" priority="High"/>
            <contact callsign="RESCUE-1"/>
            <remarks>Medical emergency - request immediate assistance</remarks>
        </detail>"#)
        .build();
    
    // Moving vehicle with track data
    let vehicle_track = CotEvent::builder()
        .uid("VEHICLE-ALPHA-1")
        .event_type("a-f-G-E-V-C")  // Ground vehicle
        .location_with_accuracy(34.045000, -118.250000, 75.0, 8.0, 12.0)
        .callsign("ALPHA-ACTUAL")
        .team("Blue")
        .how("m-g")  // Machine GPS
        .stale_in(Duration::seconds(30))  // Fast-moving, frequent updates
        .detail(r#"<detail>
            <contact callsign="ALPHA-ACTUAL"/>
            <__group name="Blue" role="Team Leader"/>
            <track speed="45.0" course="135.0"/>
            <fuel level="75"/>
            <crew size="4"/>
        </detail>"#)
        .build();
    
    events.push(sniper_position);
    events.push(emergency_beacon);
    events.push(vehicle_track);
    
    Ok(events)
}

// Point construction variants
fn demonstrate_point_construction() -> Result<(), Box<dyn std::error::Error>> {
    // Method 1: Builder pattern
    let point1 = Point::builder()
        .lat(34.0526)
        .lon(-118.2437)
        .hae(100.0)
        .ce(5.0)
        .le(10.0)
        .build();
    
    // Method 2: Coordinates with accuracy
    let point2 = Point::builder()
        .coordinates(34.0526, -118.2437, 100.0)
        .accuracy(5.0, 10.0)
        .build();
    
    // Method 3: Direct constructors
    let point3 = Point::new(34.0526, -118.2437, 100.0);
    let point4 = Point::with_accuracy(34.0526, -118.2437, 100.0, 5.0, 10.0);
    
    println!("Created {} points with different construction methods", 4);
    Ok(())
}
```

### Chat and Communication Events

```rust
use ditto_cot::cot_events::CotEvent;

fn create_communication_events() -> Result<Vec<CotEvent>, Box<dyn std::error::Error>> {
    let mut events = Vec::new();
    
    // Method 1: Convenience function
    let simple_chat = CotEvent::new_chat_message(
        "USER-456",
        "BRAVO-2", 
        "Message received, moving to coordinates",
        "All Chat Rooms",
        "all-chat-room-id"
    );
    
    // Method 2: Builder with full control
    let tactical_chat = CotEvent::builder()
        .uid("CHAT-789")
        .event_type("b-t-f")
        .time(Utc::now())
        .callsign("CHARLIE-3")
        .detail(r#"<detail>
            <chat room="Command Net" id="cmd-net-001">
                <chatgrp uid="USER-789" id="cmd-net-001" senderCallsign="CHARLIE-3">
                    Enemy contact 200m north of checkpoint Alpha
                </chatgrp>
            </chat>
            <contact callsign="CHARLIE-3"/>
        </detail>"#)
        .build();
    
    // Group message with location
    let location_chat = CotEvent::builder()
        .uid("CHAT-LOCATION-001")
        .event_type("b-t-f")
        .location(34.052235, -118.243683, 100.0)  // Include sender location
        .callsign("DELTA-4")
        .detail(r#"<detail>
            <chat room="Patrol Net" id="patrol-net">
                <chatgrp uid="USER-DELTA-4" id="patrol-net" senderCallsign="DELTA-4">
                    Checkpoint clear, proceeding to next waypoint
                </chatgrp>
            </chat>
            <contact callsign="DELTA-4"/>
            <__group name="Red" role="Patrol Leader"/>
        </detail>"#)
        .build();
    
    events.push(simple_chat);
    events.push(tactical_chat);
    events.push(location_chat);
    
    Ok(events)
}
```

## Async Ditto Integration

### Complete Integration Example

```rust
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{cot_to_document, CotDocument},
};
use dittolive_ditto::prelude::*;
use tokio::time::{sleep, Duration};
use std::sync::Arc;

#[derive(Clone)]
pub struct CotSyncManager {
    ditto: Arc<Ditto>,
    peer_id: String,
}

impl CotSyncManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let app_id = std::env::var("DITTO_APP_ID")
            .map_err(|_| "DITTO_APP_ID environment variable required")?;
        let token = std::env::var("DITTO_PLAYGROUND_TOKEN")
            .map_err(|_| "DITTO_PLAYGROUND_TOKEN environment variable required")?;
        
        let ditto = Ditto::builder()
            .with_root(DittoRoot::from_current_exe()?)
            .with_identity(DittoIdentity::OnlinePlayground {
                app_id: app_id.clone(),
                token: token.clone(),
                enable_ditto_cloud_sync: true,
            })?
            .build()?;
        
        ditto.start_sync()?;
        
        let peer_id = format!("rust-peer-{}", uuid::Uuid::new_v4());
        
        Ok(Self {
            ditto: Arc::new(ditto),
            peer_id,
        })
    }
    
    pub async fn store_cot_event(&self, cot_xml: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Parse CoT XML
        let event = CotEvent::from_xml(cot_xml)?;
        let doc = cot_to_document(&event, &self.peer_id);
        
        // Determine collection based on document type
        let collection_name = match &doc {
            CotDocument::MapItem(_) => "map_items",
            CotDocument::Chat(_) => "chat_messages",
            CotDocument::File(_) => "files",
            CotDocument::Api(_) => "api_events",
        };
        
        // Store in Ditto
        let store = self.ditto.store();
        let doc_json = serde_json::to_value(&doc)?;
        let query = format!("INSERT INTO {} DOCUMENTS (:doc) ON ID CONFLICT DO MERGE", collection_name);
        let params = serde_json::json!({ "doc": doc_json });
        
        store.execute_v2((&query, params)).await?;
        
        let doc_id = match &doc {
            CotDocument::MapItem(item) => &item.id,
            CotDocument::Chat(chat) => &chat.id,
            CotDocument::File(file) => &file.id,
            CotDocument::Api(api) => &api.id,
        };
        
        println!("Stored {} document with ID: {}", collection_name, doc_id);
        Ok(doc_id.clone())
    }
    
    pub async fn query_nearby_units(&self, lat: f64, lon: f64, radius_degrees: f64) -> Result<Vec<CotDocument>, Box<dyn std::error::Error>> {
        let store = self.ditto.store();
        
        let query = "SELECT * FROM map_items WHERE 
            j BETWEEN ? AND ? AND 
            l BETWEEN ? AND ? AND 
            w LIKE 'a-f-%'";  // Only friendly units
            
        let lat_min = lat - radius_degrees;
        let lat_max = lat + radius_degrees;
        let lon_min = lon - radius_degrees;
        let lon_max = lon + radius_degrees;
        
        let params = serde_json::json!([lat_min, lat_max, lon_min, lon_max]);
        let results = store.execute_v2((query, params)).await?;
        
        let mut cot_documents = Vec::new();
        for item in results.items() {
            let doc_json = item.json_string();
            if let Ok(doc) = serde_json::from_str::<CotDocument>(&doc_json) {
                cot_documents.push(doc);
            }
        }
        
        Ok(cot_documents)
    }
}

// Usage example
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sync_manager = CotSyncManager::new().await?;
    
    // Create and store a location update
    let location_event = CotEvent::builder()
        .uid("RUST-DEMO-001")
        .event_type("a-f-G-U-C")
        .location(34.052235, -118.243683, 100.0)
        .callsign("RUST-DEMO")
        .team("Blue")
        .build();
    
    let xml = location_event.to_xml()?;
    let doc_id = sync_manager.store_cot_event(&xml).await?;
    
    // Wait for sync
    sleep(Duration::from_secs(2)).await;
    
    // Query nearby units
    let nearby = sync_manager.query_nearby_units(34.052235, -118.243683, 0.01).await?;
    println!("Found {} nearby units", nearby.len());
    
    Ok(())
}
```

## Observer Patterns

### SDK Observer Integration

```rust
use ditto_cot::ditto::sdk_conversion::{observer_json_to_cot_document, observer_json_to_json_with_r_fields};
use dittolive_ditto::prelude::*;
use tokio::sync::mpsc;

pub struct CotObserverManager {
    ditto: Arc<Ditto>,
    event_sender: mpsc::UnboundedSender<CotDocument>,
}

impl CotObserverManager {
    pub fn new(ditto: Arc<Ditto>) -> (Self, mpsc::UnboundedReceiver<CotDocument>) {
        let (tx, rx) = mpsc::unbounded_channel();
        
        (Self {
            ditto,
            event_sender: tx,
        }, rx)
    }
    
    pub async fn start_observers(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Location updates observer
        self.setup_location_observer().await?;
        
        // Chat messages observer
        self.setup_chat_observer().await?;
        
        // Emergency events observer
        self.setup_emergency_observer().await?;
        
        Ok(())
    }
    
    async fn setup_location_observer(&self) -> Result<(), Box<dyn std::error::Error>> {
        let store = self.ditto.store();
        let sender = self.event_sender.clone();
        
        let _subscription = store
            .collection("map_items")
            .find_all()
            .subscribe()
            .observe(move |docs, _event| {
                for doc in docs {
                    let boxed_doc = doc.value();
                    
                    match observer_json_to_cot_document(&boxed_doc) {
                        Ok(Some(cot_doc @ CotDocument::MapItem(_))) => {
                            if let Err(e) = sender.send(cot_doc) {
                                eprintln!("Failed to send location update: {}", e);
                            }
                        },
                        Ok(Some(other)) => {
                            eprintln!("Unexpected document type in map_items: {:?}", other);
                        },
                        Ok(None) => {
                            eprintln!("Failed to convert observer document to CotDocument");
                        },
                        Err(e) => {
                            eprintln!("Observer conversion error: {}", e);
                        }
                    }
                }
            })?;
        
        println!("Location observer started");
        Ok(())
    }
    
    async fn setup_chat_observer(&self) -> Result<(), Box<dyn std::error::Error>> {
        let store = self.ditto.store();
        let sender = self.event_sender.clone();
        
        let _subscription = store
            .collection("chat_messages")
            .find_all()
            .subscribe()
            .observe(move |docs, _event| {
                for doc in docs {
                    let boxed_doc = doc.value();
                    
                    if let Ok(Some(cot_doc @ CotDocument::Chat(_))) = observer_json_to_cot_document(&boxed_doc) {
                        if let Err(e) = sender.send(cot_doc) {
                            eprintln!("Failed to send chat message: {}", e);
                        }
                    }
                }
            })?;
        
        println!("Chat observer started");
        Ok(())
    }
    
    async fn setup_emergency_observer(&self) -> Result<(), Box<dyn std::error::Error>> {
        let store = self.ditto.store();
        let sender = self.event_sender.clone();
        
        // Filter for emergency events only
        let _subscription = store
            .collection("api_events")
            .find("w LIKE 'b-a-%'")  // Emergency event types
            .subscribe()
            .observe(move |docs, _event| {
                for doc in docs {
                    let boxed_doc = doc.value();
                    
                    if let Ok(Some(cot_doc @ CotDocument::Api(_))) = observer_json_to_cot_document(&boxed_doc) {
                        println!("🚨 EMERGENCY EVENT DETECTED 🚨");
                        if let Err(e) = sender.send(cot_doc) {
                            eprintln!("Failed to send emergency event: {}", e);
                        }
                    }
                }
            })?;
        
        println!("Emergency observer started");
        Ok(())
    }
}

// Application event handler
async fn handle_cot_events(mut receiver: mpsc::UnboundedReceiver<CotDocument>) {
    while let Some(cot_doc) = receiver.recv().await {
        match cot_doc {
            CotDocument::MapItem(map_item) => {
                println!("📍 Location: {} at {},{}", 
                    map_item.e,
                    map_item.j.unwrap_or(0.0),
                    map_item.l.unwrap_or(0.0)
                );
                
                // Process location data
                handle_location_update(&map_item);
            },
            CotDocument::Chat(chat) => {
                println!("💬 Chat from {}: {}", 
                    chat.author_callsign, 
                    chat.message
                );
                
                // Process chat message
                handle_chat_message(&chat);
            },
            CotDocument::Api(api) => {
                println!("🚨 Emergency from {}", api.e);
                
                // Handle emergency with priority
                handle_emergency_event(&api);
            },
            CotDocument::File(file) => {
                println!("📎 File shared: {}", 
                    file.file.clone().unwrap_or_default()
                );
                
                // Handle file sharing
                handle_file_share(&file);
            }
        }
    }
}

fn handle_location_update(map_item: &ditto_cot::ditto::MapItem) {
    // Extract r-field details if available
    if let Some(r_fields) = &map_item.r {
        println!("  Detail data: {:?}", r_fields);
        
        // Process specific detail fields
        if let Some(contact) = r_fields.get("contact") {
            println!("  Contact info: {:?}", contact);
        }
        
        if let Some(track) = r_fields.get("track") {
            println!("  Movement data: {:?}", track);
        }
    }
}

fn handle_chat_message(chat: &ditto_cot::ditto::Chat) {
    // Update UI, send notifications, etc.
    println!("  Room: {}", chat.room);
    if let Some(location) = &chat.location {
        println!("  Sender location: {}", location);
    }
}

fn handle_emergency_event(api: &ditto_cot::ditto::Api) {
    // High priority processing
    println!("  🚨 EMERGENCY PRIORITY 🚨");
    println!("  Callsign: {}", api.e);
    
    // Trigger alerts, notifications, etc.
}

fn handle_file_share(file: &ditto_cot::ditto::File) {
    println!("  MIME: {}", file.mime.clone().unwrap_or_default());
    println!("  Size: {} bytes", file.sz.unwrap_or_default());
}
```

## Error Handling

### Robust Error Management

```rust
use ditto_cot::{cot_events::CotEvent, ditto::cot_to_document};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CotIntegrationError {
    #[error("XML parsing failed: {0}")]
    XmlParse(#[from] ditto_cot::cot_events::CotEventError),
    
    #[error("Ditto operation failed: {0}")]
    DittoOperation(#[from] dittolive_ditto::DittoError),
    
    #[error("JSON serialization failed: {0}")]
    JsonSerialization(#[from] serde_json::Error),
    
    #[error("Invalid configuration: {0}")]
    Configuration(String),
    
    #[error("Network operation failed: {0}")]
    Network(String),
    
    #[error("Document conversion failed: {0}")]
    DocumentConversion(String),
}

pub type Result<T> = std::result::Result<T, CotIntegrationError>;

pub struct RobustCotManager {
    ditto: Arc<Ditto>,
    peer_id: String,
    retry_config: RetryConfig,
}

#[derive(Clone)]
pub struct RetryConfig {
    pub max_retries: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

impl RobustCotManager {
    pub async fn store_cot_with_retry(&self, cot_xml: &str) -> Result<String> {
        let mut retry_count = 0;
        let mut delay = self.retry_config.initial_delay;
        
        loop {
            match self.try_store_cot(cot_xml).await {
                Ok(doc_id) => return Ok(doc_id),
                Err(e) if retry_count < self.retry_config.max_retries => {
                    eprintln!("Store attempt {} failed: {}", retry_count + 1, e);
                    
                    // Exponential backoff with jitter
                    let jitter = Duration::from_millis(fastrand::u64(0..=100));
                    tokio::time::sleep(delay + jitter).await;
                    
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * self.retry_config.backoff_multiplier) as u64
                        ),
                        self.retry_config.max_delay
                    );
                    
                    retry_count += 1;
                },
                Err(e) => return Err(e),
            }
        }
    }
    
    async fn try_store_cot(&self, cot_xml: &str) -> Result<String> {
        // Validate XML first
        let event = CotEvent::from_xml(cot_xml)
            .map_err(CotIntegrationError::XmlParse)?;
        
        // Convert to document
        let doc = cot_to_document(&event, &self.peer_id);
        
        // Validate document structure
        self.validate_document(&doc)?;
        
        // Store in Ditto
        let collection_name = self.get_collection_name(&doc);
        let doc_id = self.store_document(&doc, collection_name).await?;
        
        Ok(doc_id)
    }
    
    fn validate_document(&self, doc: &CotDocument) -> Result<()> {
        match doc {
            CotDocument::MapItem(map_item) => {
                if map_item.id.is_empty() {
                    return Err(CotIntegrationError::DocumentConversion(
                        "MapItem document missing ID".to_string()
                    ));
                }
                
                // Validate coordinates if present
                if let (Some(lat), Some(lon)) = (map_item.j, map_item.l) {
                    if lat < -90.0 || lat > 90.0 {
                        return Err(CotIntegrationError::DocumentConversion(
                            format!("Invalid latitude: {}", lat)
                        ));
                    }
                    if lon < -180.0 || lon > 180.0 {
                        return Err(CotIntegrationError::DocumentConversion(
                            format!("Invalid longitude: {}", lon)
                        ));
                    }
                }
            },
            CotDocument::Chat(chat) => {
                if chat.message.is_empty() {
                    return Err(CotIntegrationError::DocumentConversion(
                        "Chat document missing message".to_string()
                    ));
                }
            },
            // Add validation for other document types
            _ => {}
        }
        
        Ok(())
    }
    
    fn get_collection_name(&self, doc: &CotDocument) -> &'static str {
        match doc {
            CotDocument::MapItem(_) => "map_items",
            CotDocument::Chat(_) => "chat_messages",
            CotDocument::File(_) => "files",
            CotDocument::Api(_) => "api_events",
        }
    }
    
    async fn store_document(&self, doc: &CotDocument, collection: &str) -> Result<String> {
        let store = self.ditto.store();
        let doc_json = serde_json::to_value(doc)?;
        
        let query = format!("INSERT INTO {} DOCUMENTS (:doc) ON ID CONFLICT DO MERGE", collection);
        let params = serde_json::json!({ "doc": doc_json });
        
        store.execute_v2((&query, params)).await
            .map_err(CotIntegrationError::DittoOperation)?;
        
        // Extract document ID
        let doc_id = match doc {
            CotDocument::MapItem(item) => item.id.clone(),
            CotDocument::Chat(chat) => chat.id.clone(),
            CotDocument::File(file) => file.id.clone(),
            CotDocument::Api(api) => api.id.clone(),
        };
        
        Ok(doc_id)
    }
}
```

## Performance Optimization

### High-Performance Patterns

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

pub struct PerformantCotProcessor {
    ditto: Arc<Ditto>,
    semaphore: Arc<Semaphore>,
    document_cache: Arc<Mutex<HashMap<String, CotDocument>>>,
    metrics: Arc<ProcessingMetrics>,
}

#[derive(Default)]
pub struct ProcessingMetrics {
    pub documents_processed: std::sync::atomic::AtomicU64,
    pub cache_hits: std::sync::atomic::AtomicU64,
    pub errors: std::sync::atomic::AtomicU64,
}

impl PerformantCotProcessor {
    pub fn new(ditto: Arc<Ditto>, max_concurrent: usize) -> Self {
        Self {
            ditto,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            document_cache: Arc::new(Mutex::new(HashMap::new())),
            metrics: Arc::new(ProcessingMetrics::default()),
        }
    }
    
    pub async fn batch_process(&self, cot_xml_list: Vec<String>, peer_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut handles = Vec::new();
        
        for xml in cot_xml_list {
            let permit = self.semaphore.clone().acquire_owned().await?;
            let processor = self.clone();
            let peer_id = peer_id.to_string();
            
            let handle = tokio::spawn(async move {
                let _permit = permit; // Hold permit for duration
                processor.process_single_with_cache(&xml, &peer_id).await
            });
            
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            match handle.await? {
                Ok(doc_id) => results.push(doc_id),
                Err(e) => {
                    self.metrics.errors.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    eprintln!("Batch processing error: {}", e);
                }
            }
        }
        
        Ok(results)
    }
    
    async fn process_single_with_cache(&self, cot_xml: &str, peer_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Check cache first
        let cache_key = self.generate_cache_key(cot_xml, peer_id);
        
        if let Ok(cache) = self.document_cache.lock() {
            if let Some(cached_doc) = cache.get(&cache_key) {
                self.metrics.cache_hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return Ok(self.extract_document_id(cached_doc));
            }
        }
        
        // Process new document
        let event = CotEvent::from_xml(cot_xml)?;
        let doc = cot_to_document(&event, peer_id);
        
        // Store in Ditto
        let doc_id = self.store_document_efficiently(&doc).await?;
        
        // Update cache
        if let Ok(mut cache) = self.document_cache.lock() {
            cache.insert(cache_key, doc);
        }
        
        self.metrics.documents_processed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(doc_id)
    }
    
    fn generate_cache_key(&self, cot_xml: &str, peer_id: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        cot_xml.hash(&mut hasher);
        peer_id.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    async fn store_document_efficiently(&self, doc: &CotDocument) -> Result<String, Box<dyn std::error::Error>> {
        let store = self.ditto.store();
        
        // Use prepared statements for better performance
        let collection = match doc {
            CotDocument::MapItem(_) => "map_items",
            CotDocument::Chat(_) => "chat_messages",
            CotDocument::File(_) => "files",
            CotDocument::Api(_) => "api_events",
        };
        
        let doc_json = serde_json::to_value(doc)?;
        let query = format!("INSERT INTO {} DOCUMENTS (:doc) ON ID CONFLICT DO MERGE", collection);
        let params = serde_json::json!({ "doc": doc_json });
        
        store.execute_v2((&query, params)).await?;
        
        Ok(self.extract_document_id(doc))
    }
    
    fn extract_document_id(&self, doc: &CotDocument) -> String {
        match doc {
            CotDocument::MapItem(item) => item.id.clone(),
            CotDocument::Chat(chat) => chat.id.clone(),
            CotDocument::File(file) => file.id.clone(),
            CotDocument::Api(api) => api.id.clone(),
        }
    }
    
    pub fn get_metrics(&self) -> (u64, u64, u64) {
        let processed = self.metrics.documents_processed.load(std::sync::atomic::Ordering::Relaxed);
        let cache_hits = self.metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed);
        let errors = self.metrics.errors.load(std::sync::atomic::Ordering::Relaxed);
        
        (processed, cache_hits, errors)
    }
}

impl Clone for PerformantCotProcessor {
    fn clone(&self) -> Self {
        Self {
            ditto: Arc::clone(&self.ditto),
            semaphore: Arc::clone(&self.semaphore),
            document_cache: Arc::clone(&self.document_cache),
            metrics: Arc::clone(&self.metrics),
        }
    }
}
```

## Testing Patterns

### Comprehensive Test Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_cot_event_creation() -> Result<(), Box<dyn std::error::Error>> {
        let event = CotEvent::builder()
            .uid("TEST-001")
            .event_type("a-f-G-U-C")
            .location(34.0, -118.0, 100.0)
            .callsign("TEST-UNIT")
            .build();
        
        assert_eq!(event.uid, "TEST-001");
        assert_eq!(event.event_type, "a-f-G-U-C");
        
        if let Some(point) = &event.point {
            assert_eq!(point.lat, 34.0);
            assert_eq!(point.lon, -118.0);
            assert_eq!(point.hae, 100.0);
        }
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_xml_round_trip() -> Result<(), Box<dyn std::error::Error>> {
        let original_xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C" 
                              time="2024-01-15T10:30:00.000Z" 
                              start="2024-01-15T10:30:00.000Z" 
                              stale="2024-01-15T10:35:00.000Z">
            <point lat="34.0" lon="-118.0" hae="100.0"/>
            <detail><contact callsign="TEST"/></detail>
        </event>"#;
        
        // Parse XML
        let event = CotEvent::from_xml(original_xml)?;
        
        // Convert back to XML
        let regenerated_xml = event.to_xml()?;
        
        // Parse again to verify
        let reparsed_event = CotEvent::from_xml(&regenerated_xml)?;
        
        assert_eq!(event.uid, reparsed_event.uid);
        assert_eq!(event.event_type, reparsed_event.event_type);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_ditto_document_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let event = CotEvent::builder()
            .uid("CONV-TEST-001")
            .event_type("a-f-G-U-C")
            .location(34.0, -118.0, 100.0)
            .callsign("CONV-TEST")
            .build();
        
        let doc = cot_to_document(&event, "test-peer");
        
        match doc {
            CotDocument::MapItem(map_item) => {
                assert_eq!(map_item.id, "CONV-TEST-001");
                assert_eq!(map_item.e, "CONV-TEST");
                assert_eq!(map_item.j, Some(34.0));
                assert_eq!(map_item.l, Some(-118.0));
            },
            _ => panic!("Expected MapItem document"),
        }
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_error_handling() {
        // Test invalid XML
        let invalid_xml = "<invalid><xml>";
        let result = CotEvent::from_xml(invalid_xml);
        assert!(result.is_err());
        
        // Test empty UID
        let event_result = CotEvent::builder()
            .uid("")  // Invalid empty UID
            .event_type("a-f-G-U-C")
            .build();
        
        // Should handle gracefully or validate
        // (depending on your validation strategy)
    }
    
    // Mock Ditto for testing
    async fn create_test_ditto() -> Result<Ditto, Box<dyn std::error::Error>> {
        // This would use test credentials or mock
        // Implementation depends on your test setup
        todo!("Implement test Ditto instance")
    }
}
```

These examples demonstrate comprehensive Rust integration patterns for the Ditto CoT library, focusing on performance, error handling, and idiomatic Rust code. For Java-specific patterns, see [Java Integration Examples](java.md).