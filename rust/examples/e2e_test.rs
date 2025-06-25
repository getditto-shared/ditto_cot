use anyhow::{Context, Result};
use chrono::Utc;
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{cot_to_document, from_ditto::cot_event_from_ditto_document, DittoDocument},
};
use dittolive_ditto::fs::PersistentRoot;
use dittolive_ditto::prelude::*;
use std::sync::Arc;

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
        return Err(e).context("Failed to load .env file");
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
    println!(
        "DITTO_PLAYGROUND_TOKEN: {}... (truncated)",
        &playground_token[..10]
    );

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
    let ditto = Ditto::builder()
        .with_root(root.clone())
        .with_identity(|_ditto_root| {
            // This closure is called with the Ditto root
            identity::OnlinePlayground::new(
                _ditto_root,
                app_id.clone(),
                playground_token.clone(),
                cloud_sync,
                custom_auth_url,
            )
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
        </event>
        "#,
        event_uid, start_time, start_time, stale_time
    );

    println!("Generated CoT XML with timestamps:");
    println!("--------------------------------");
    println!("{}", cot_xml);
    println!("--------------------------------");

    // 1. Parse a CoT XML event using the public library interface
    println!("1. Parsing CoT XML with CotEvent::from_xml");
    let cot_event = CotEvent::from_xml(&cot_xml)
        .with_context(|| format!("Failed to parse CoT XML: {}", cot_xml))?;
    println!("   Successfully parsed CoT XML into CotEvent: uid={}, type={}, time={:?}, start={:?}, stale={:?}", 
        cot_event.uid, cot_event.event_type, cot_event.time, cot_event.start, cot_event.stale);

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
            // Ensure _id is explicitly set if needed, but it should be part of the serialized map_item
            serde_json::to_value(map_item).unwrap()
        }
        _ => {
            println!("   Error: Expected MapItem document type");
            return Ok(());
        }
    };

    // Insert the document using DQL v2 with parameters
    let query = "INSERT INTO map_items VALUES (:document) ON ID CONFLICT DO MERGE";
    println!("Executing DQL: {}", query);

    // Execute the query with parameters
    let query_result = store
        .execute_v2((
            query,
            serde_json::json!({
                "document": doc_value
            }),
        ))
        .await?;

    // For INSERT queries, we don't expect any items in the result
    // Just log the number of items affected (if any)
    println!("DQL INSERT affected {} items", query_result.item_count());
    println!("Successfully executed DQL INSERT");

    println!("   Document inserted with ID: {}", doc_id);

    // 6. Query the document back from Ditto
    println!("6. Querying document from Ditto");
    let query = format!(
        "SELECT * FROM map_items WHERE _id = '{}'",
        doc_value["_id"].as_str().unwrap_or("")
    );
    println!("Executing DQL query: {}", query);
    let query_result = store.execute_v2(&query).await?;

    // Verify we got a result
    if query_result.item_count() == 0 {
        return Err(anyhow::anyhow!("No documents found matching the query"));
    }

    println!("Successfully retrieved document from Ditto");

    // Get the first document
    let doc = query_result
        .iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No document found with ID: {}", doc_id))?;
    println!("   Successfully retrieved document from Ditto");

    // Print the raw JSON string for debugging
    let json_str = doc.json_string();
    println!("   Raw JSON from Ditto: {}", json_str);

    // 7. Convert the Ditto document back to a CotEvent
    println!("7. Converting Ditto document back to CotEvent");

    // Deserialize the JSON into DittoDocument using the library function
    let retrieved_doc = DittoDocument::from_json_str(&json_str)?;

    let retrieved_cot_event = cot_event_from_ditto_document(&retrieved_doc);
    println!("   Successfully converted retrieved document back to CotEvent");

    // 8. Verify the round-trip conversion
    println!("8. Verifying round-trip conversion");

    let cot_xml_out = retrieved_cot_event
        .to_xml()
        .unwrap_or_else(|e| format!("Error generating XML: {}", e));
    if cot_xml.trim() == cot_xml_out.trim() {
        println!("SUCCESS: XML outputs match! Original and roundtripped XML are identical.");
        println!("\n✅ Full XML to XML Round-trip conversion successful!");
        println!("This example demonstrated a complete round-trip conversion:");
        println!("  - Parsed CoT XML into a FlatCotEvent");
        println!("  - Converted to a CotEvent and then to a Ditto document");
        println!("  - Stored in Ditto and retrieved back");
        println!("  - Converted back to a CotEvent and verified field preservation\n");
    } else {
        println!("❌ Round-trip conversion failed!");
        println!("Diff:\n-{}\n+{}", cot_xml, cot_xml_out);
    }

    println!("\nE2E test completed.");

    println!("Note: This test uses a real Ditto instance with the online playground.");
    println!(
        "      Make sure you have set the DITTO_APP_ID and DITTO_TOKEN environment variables."
    );

    // Gracefully shutdown Ditto instance
    ditto.close();

    Ok(())
}
