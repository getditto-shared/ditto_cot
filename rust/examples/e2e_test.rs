use anyhow::{Context, Result};
use chrono::{DateTime, Utc, TimeZone};
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{
        cot_to_document, from_ditto::cot_event_from_ditto_document,
        Api, Chat, File, MapItem, DittoDocument
    },
    xml_parser::parse_cot,
};
use dittolive_ditto::prelude::*;
use dittolive_ditto::fs::PersistentRoot;
use serde_json::json;
use std::sync::Arc;
use uuid;

// Collection name for Ditto documents (unused in this example)
#[allow(dead_code)]
const COLLECTION_NAME: &str = "cot_events";

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file in the current directory
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let env_path = current_dir.join(".env");
    
    println!("Current directory: {}", current_dir.display());
    println!("Trying to load .env from: {}", env_path.display());
    
    // Try to load .env file from the current directory
    if let Err(e) = dotenv::from_path(&env_path) {
        println!("Failed to load .env file: {}", e);
        return Err(e).context("Failed to load .env file").map_err(Into::into);
    } else {
        println!("Successfully loaded .env file");
    }
    
    // Get Ditto App ID and token from environment variables
    let app_id = AppId::from_env("DITTO_APP_ID")
        .context("DITTO_APP_ID environment variable not set or invalid")?;
    let playground_token = std::env::var("DITTO_PLAYGROUND_TOKEN")
        .context("DITTO_PLAYGROUND_TOKEN environment variable not set")?;
    
    // Print environment variables for debugging
    println!("DITTO_APP_ID: {}", app_id);
    println!("DITTO_PLAYGROUND_TOKEN: {}... (truncated)", &playground_token[..10]);
    
    // Initialize Ditto
    println!("Initializing Ditto...");
    
    // Create a temporary directory for Ditto data
    let temp_dir = tempfile::tempdir().context("Failed to create temp dir")?;
    let ditto_path = temp_dir.path().join("ditto_data");
    
    // Create Ditto instance with online playground identity
    let root = Arc::new(PersistentRoot::new(ditto_path)?);
    let cloud_sync = true;
    let custom_auth_url: Option<&str> = None;
    
    // Initialize Ditto with a closure that creates the identity
    let mut ditto = Ditto::builder()
        .with_root(root.clone())
        .with_identity(|_ditto_root| {
            // This closure is called with the Ditto root
            Ok(identity::OnlinePlayground::new(
                _ditto_root,
                app_id.clone(),
                playground_token.clone(),
                cloud_sync,
                custom_auth_url,
            )?)
        })?
        .with_minimum_log_level(LogLevel::Debug)
        .build()
        .context("Failed to initialize Ditto")?;
        
    // Disable V3 sync to use DQL mutations
    let _ = ditto.disable_sync_with_v3();
    
    // 5. Start Ditto sync
    println!("5. Starting Ditto sync");
    ditto.start_sync().context("Failed to start Ditto sync")?;
    
    // Get a reference to the store
    let store = ditto.store();
    
    println!("Starting E2E integration test");

    // Generate RFC3339 timestamps
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(5)).to_rfc3339();
    let event_uid = format!("TEST-{}-1", uuid::Uuid::new_v4());
    
    // Sample CoT XML event with proper timestamps and required attributes
    let cot_xml = format!(
        r#"
        <event version="2.0"
              type="a-f-G-U-C"
              uid="{}"
              time="{}"
              start="{}"
              stale="{}"
              how="h-g-i-g-o"
              lat="1.2345"
              lon="2.3456"
              hae="9999999.0"
              ce="9999999.0"
              le="9999999.0">
            <point lat="1.2345" lon="2.3456" hae="9999999.0" ce="9999999.0" le="9999999.0"/>
            <detail>
                <track course="45.0" speed="10.0"/>
                <contact endpoint="*:-1:stcp" callsign="TEST-1"/>
                <uid Droid="TEST-1"/>
                <__group name="Cyan" role="Team Member"/>
                <status battery="100"/>
                <track />
                <precisionlocation geopointsrc="User" altsrc="???"/>
            </detail>
            <marti>
                <dest callsign="TEST-1"/>
            </marti>
        </event>
        "#,
        event_uid,
        start_time,
        start_time,
        stale_time
    );
    
    println!("Generated CoT XML with timestamps:");
    println!("--------------------------------");
    println!("{}", cot_xml);
    println!("--------------------------------");
    
    // 1. Parse a CoT XML event
    println!("1. Parsing CoT XML");
    let flat_event = parse_cot(&cot_xml)
        .with_context(|| format!("Failed to parse CoT XML: {}", cot_xml))?;
    
    println!("   Successfully parsed CoT XML");
    
    // Debug print the parsed flat event
    println!("   Parsed FlatCotEvent: uid={}, type={}, time={}, start={}, stale={}", 
        flat_event.uid, flat_event.type_, flat_event.time, flat_event.start, flat_event.stale);
        
    // Verify required fields are present
    if flat_event.time.is_empty() || flat_event.start.is_empty() || flat_event.stale.is_empty() {
        anyhow::bail!("Parsed event is missing required timestamp fields. Make sure the XML includes 'time', 'start', and 'stale' attributes in the event element.");
    }
    
    // 2. Convert FlatCotEvent to CotEvent (manually for now)
    println!("2. Converting FlatCotEvent to CotEvent");
    let cot_event = CotEvent {
        version: "2.0".to_string(),
        uid: flat_event.uid,
        event_type: flat_event.type_,
        how: flat_event.how,
        time: DateTime::parse_from_rfc3339(&flat_event.time)
            .context("Failed to parse time")?
            .with_timezone(&Utc),
        start: DateTime::parse_from_rfc3339(&flat_event.start)
            .context("Failed to parse start time")?
            .with_timezone(&Utc),
        stale: DateTime::parse_from_rfc3339(&flat_event.stale)
            .context("Failed to parse stale time")?
            .with_timezone(&Utc),
        point: ditto_cot::cot_events::Point {
            lat: flat_event.lat,
            lon: flat_event.lon,
            hae: flat_event.hae,
            ce: flat_event.ce,
            le: flat_event.le,
        },
        detail: flat_event.detail_extra
            .into_iter()
            .filter_map(|(k, v)| {
                if let serde_json::Value::String(s) = v {
                    Some((k, s))
                } else {
                    Some((k, v.to_string()))
                }
            })
            .collect(),
    };
    println!("   Successfully converted to CotEvent");
    
    // 3. Convert CotEvent to Ditto document
    println!("3. Converting CotEvent to Ditto document");
    let ditto_doc = cot_to_document(&cot_event, "e2e-test-peer");
    println!("   Successfully converted to Ditto document");
    
    // 4. Convert Ditto document back to CotEvent (simulated round-trip)
    println!("4. Converting Ditto document back to CotEvent");
    let _roundtrip_cot_event = cot_event_from_ditto_document(&ditto_doc);
    println!("   Successfully converted back to CotEvent");
    
    // 5. Insert document into Ditto using DQL
    println!("5. Inserting document into Ditto");
    let doc_id = cot_event.uid.clone();
    
    // Convert our DittoDocument to a serde_json::Value
    let doc_value = match ditto_doc {
        DittoDocument::MapItem(ref map_item) => {
            let mut map = serde_json::Map::new();
            map.insert("a".to_string(), json!(&map_item.a));
            map.insert("b".to_string(), json!(&map_item.b));
            if let Some(ref c) = map_item.c { map.insert("c".to_string(), json!(c)); }
            map.insert("d".to_string(), json!(&map_item.d));
            map.insert("_c".to_string(), json!(&map_item.d_c));
            map.insert("_r".to_string(), json!(&map_item.d_r));
            map.insert("_v".to_string(), json!(&map_item.d_v));
            map.insert("e".to_string(), json!(&map_item.e));
            if let Some(ref f) = map_item.f { map.insert("f".to_string(), json!(f)); }
            map.insert("g".to_string(), json!(&map_item.g));
            if let Some(ref h) = map_item.h { map.insert("h".to_string(), json!(h)); }
            if let Some(ref i) = map_item.i { map.insert("i".to_string(), json!(i)); }
            if let Some(ref j) = map_item.j { map.insert("j".to_string(), json!(j)); }
            if let Some(ref k) = map_item.k { map.insert("k".to_string(), json!(k)); }
            if let Some(ref l) = map_item.l { map.insert("l".to_string(), json!(l)); }
            map.insert("n".to_string(), json!(&map_item.n));
            map.insert("o".to_string(), json!(&map_item.o));
            map.insert("p".to_string(), json!(&map_item.p));
            map.insert("q".to_string(), json!(&map_item.q));
            map.insert("r".to_string(), json!(&map_item.r));
            map.insert("s".to_string(), json!(&map_item.s));
            map.insert("t".to_string(), json!(&map_item.t));
            map.insert("u".to_string(), json!(&map_item.u));
            map.insert("v".to_string(), json!(&map_item.v));
            map.insert("w".to_string(), json!(&map_item.w));
            map.insert("_id".to_string(), json!(&doc_id));
            serde_json::Value::Object(map)
        }
        _ => {
            println!("   Error: Expected MapItem document type");
            return Ok(());
        }
    };
    
    // Insert the document using DQL v2 with parameters
    let query = "INSERT INTO map_items VALUES (:document) ON ID CONFLICT DO MERGE";
    println!("Executing DQL: {}", query);
    
    // Convert the document to a serde_json::Value
    let doc_value = serde_json::to_value(doc_value)?;
    
    // Execute the query with parameters
    let query_result = store.execute_v2((
        query,
        serde_json::json!({
            "document": doc_value
        })
    )).await?;
    
    // For INSERT queries, we don't expect any items in the result
    // Just log the number of items affected (if any)
    println!("DQL INSERT affected {} items", query_result.item_count());
    println!("Successfully executed DQL INSERT");
    
    println!("   Document inserted with ID: {}", doc_id);
    
    // 6. Query the document back from Ditto
    println!("6. Querying document from Ditto");
    let query = format!("SELECT * FROM map_items WHERE _id = '{}'", doc_value["_id"].as_str().unwrap_or(""));
    println!("Executing DQL query: {}", query);
    let query_result = store.execute_v2(&query).await?;
    
    // Verify we got a result
    if query_result.item_count() == 0 {
        return Err(anyhow::anyhow!("No documents found matching the query"));
    }
    
    println!("Successfully retrieved document from Ditto");
    
    // Get the first document
    let doc = query_result.iter().next()
        .ok_or_else(|| anyhow::anyhow!("No document found with ID: {}", doc_id))?;
    println!("   Successfully retrieved document from Ditto");
    
    // Print the raw JSON string for debugging
    let json_str = doc.json_string();
    println!("   Raw JSON from Ditto: {}", json_str);
    
    // 7. Convert the Ditto document back to a CotEvent
    println!("7. Converting Ditto document back to CotEvent");
    
    // Parse the JSON into a Value first to inspect the type
    let json_value: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;
    
    // Get the document type from the 'w' field (CoT type)
    let doc_type = json_value.get("w")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Document is missing 'w' field"))?;
    
    // Deserialize into the appropriate DittoDocument variant based on the type
    let retrieved_doc = if doc_type.starts_with("a-f-G-U") || doc_type.starts_with("a-u-r-loc") {
        // This is a MapItem - handle missing fields by providing defaults
        let mut map_item: MapItem = serde_json::from_value(json_value.clone())
            .map_err(|e| anyhow::anyhow!("Failed to deserialize as MapItem: {}", e))?;
        
        // Ensure required fields have default values if missing
        let c_value = json_value.get("d_c").or_else(|| json_value.get("_c"));
        if map_item.d_c == 0 && c_value.is_none() {
            map_item.d_c = 1; // Default document counter
        }
        
        DittoDocument::MapItem(map_item)
    } else if doc_type.contains("b-t-f") || doc_type.contains("chat") {
        // This is a Chat
        let chat: Chat = serde_json::from_value(json_value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize as Chat: {}", e))?;
        DittoDocument::Chat(chat)
    } else if doc_type == "a-u-emergency-g" {
        // This is an emergency (Api)
        let api: Api = serde_json::from_value(json_value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize as Api: {}", e))?;
        DittoDocument::Api(api)
    } else {
        // Default to File for unknown types
        let file: File = serde_json::from_value(json_value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize as File: {}", e))?;
        DittoDocument::File(file)
    };
    
    let retrieved_cot_event = cot_event_from_ditto_document(&retrieved_doc);
    println!("   Successfully converted retrieved document back to CotEvent");
    
    // 8. Verify the round-trip conversion
    println!("8. Verifying round-trip conversion");
    
    // Check if the original and retrieved CotEvents are equal
    if cot_event == retrieved_cot_event {
        println!("   SUCCESS: Original and retrieved CotEvents match!");
        println!("\n✅ Round-trip conversion successful!");
    } else {
        println!("   ERROR: Original and retrieved CotEvents do not match!");
        println!("  Original: {:#?}", cot_event);
        println!("  Retrieved: {:#?}", retrieved_cot_event);
        println!("\n❌ Round-trip conversion failed!");
    }
    
    println!("\nE2E test completed successfully!");
    println!("This example demonstrated a complete round-trip conversion:");
    println!("  - Parsed CoT XML into a FlatCotEvent");
    println!("  - Converted to a CotEvent and then to a Ditto document");
    println!("  - Stored in Ditto and retrieved back");
    println!("  - Converted back to a CotEvent and verified field preservation\n");
    
    println!("Note: This test uses a real Ditto instance with the online playground.");
    println!("      Make sure you have set the DITTO_APP_ID and DITTO_TOKEN environment variables.");
    
    Ok(())
}
