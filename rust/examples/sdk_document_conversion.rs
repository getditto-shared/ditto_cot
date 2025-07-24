//! Example demonstrating observer document to CotDocument/JSON conversion
//!
//! This example shows how to use the new observer document conversion utilities to extract
//! full document content with proper r-field reconstruction in observer callbacks.

use ditto_cot::cot_events::CotEvent;
use ditto_cot::ditto::cot_to_document;
use ditto_cot::ditto::{
    observer_json_to_cot_document, observer_json_to_json_with_r_fields, CotDocument,
};
use dittolive_ditto::fs::PersistentRoot;
use dittolive_ditto::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 SDK Document Conversion Example");

    // Load environment variables for Ditto credentials
    dotenv::dotenv().ok();

    // Get Ditto credentials from environment
    let app_id = AppId::from_env("DITTO_APP_ID").unwrap_or_else(|_| AppId::generate()); // Use generated ID if env not set
    let playground_token =
        env::var("DITTO_PLAYGROUND_TOKEN").unwrap_or_else(|_| "demo-token".to_string()); // Use demo token if env not set

    // Initialize Ditto with proper setup
    let temp_dir = tempfile::tempdir()?;
    let ditto_path = temp_dir.path().join("ditto_data");
    let root = Arc::new(PersistentRoot::new(ditto_path)?);

    let ditto = Ditto::builder()
        .with_root(root.clone())
        .with_minimum_log_level(LogLevel::Warning)
        .with_identity(|ditto_root| {
            identity::OnlinePlayground::new(
                ditto_root,
                app_id.clone(),
                playground_token,
                false,        // Use peer-to-peer sync for local example
                None::<&str>, // No custom auth URL
            )
        })?
        .build()?;

    let store = ditto.store();

    // Set up an observer that demonstrates the new conversion utilities
    let _observer = store.register_observer_v2("SELECT * FROM map_items", move |result| {
        println!("📄 Observer received {} documents", result.item_count());

        for observer_doc in result.iter() {
            // Get JSON string from the document
            let json_str = observer_doc.json_string();

            // Demonstrate document ID extraction
            if let Some(id) = ditto_cot::ditto::get_document_id_from_json(&json_str) {
                println!("  📋 Document ID: {}", id);
            }

            // Demonstrate document type extraction
            if let Some(doc_type) = ditto_cot::ditto::get_document_type_from_json(&json_str) {
                println!("  🏷️  Document type: {}", doc_type);
            }

            // Convert observer document JSON to JSON with r-field reconstruction
            match observer_json_to_json_with_r_fields(&json_str) {
                Ok(json_value) => {
                    println!("  📋 Full JSON representation (with reconstructed r-field):");
                    println!(
                        "     {}",
                        serde_json::to_string_pretty(&json_value).unwrap_or_default()
                    );
                }
                Err(e) => {
                    println!("  ❌ Failed to convert to JSON: {}", e);
                }
            }

            // Convert observer document JSON to CotDocument
            match observer_json_to_cot_document(&json_str) {
                Ok(cot_doc) => {
                    println!("  🎯 Successfully converted to CotDocument:");
                    match &cot_doc {
                        CotDocument::MapItem(item) => {
                            println!(
                                "     MapItem - ID: {}, Lat: {:?}, Lon: {:?}",
                                item.id, item.j, item.l
                            );
                        }
                        CotDocument::Chat(chat) => {
                            println!(
                                "     Chat - Message: {:?}, Author: {:?}",
                                chat.message, chat.author_callsign
                            );
                        }
                        CotDocument::File(file) => {
                            println!("     File - Name: {:?}, MIME: {:?}", file.file, file.mime);
                        }
                        CotDocument::Api(api) => {
                            println!("     API - Content Type: {:?}", api.content_type);
                        }
                        CotDocument::Generic(generic) => {
                            println!("     Generic - ID: {}, Type: {}", generic.id, generic.w);
                        }
                    }

                    // Demonstrate round-trip conversion: CotDocument -> CotEvent
                    let cot_event = cot_doc.to_cot_event();
                    println!(
                        "  🔄 Round-trip to CotEvent - UID: {}, Type: {}",
                        cot_event.uid, cot_event.event_type
                    );
                }
                Err(e) => {
                    println!("  ❌ Failed to convert to CotDocument: {}", e);
                }
            }

            println!(); // Add spacing between documents
        }
    })?;

    // Set up a subscription for sync
    let _subscription = ditto
        .sync()
        .register_subscription_v2("SELECT * FROM map_items")?;

    println!("🔗 Setting up observer and subscription...");

    // Create some test documents to demonstrate the conversion
    println!("📝 Creating test documents...");

    // Create a sample CoT XML for a location update
    let location_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<event version="2.0" uid="test-location-001" type="a-u-r-loc-g" time="2024-01-15T10:30:00.000Z" start="2024-01-15T10:30:00.000Z" stale="2024-01-15T11:30:00.000Z" how="h-g-i-g-o">
  <point lat="37.7749" lon="-122.4194" hae="10.0" ce="5.0" le="2.0"/>
  <detail>
    <contact callsign="TestUnit001" endpoint="192.168.1.100:4242:tcp"/>
    <track speed="15.0" course="90.0"/>
  </detail>
</event>"#;

    // Convert XML to CotEvent and then to CotDocument
    if let Ok(cot_event) = CotEvent::from_xml(location_xml) {
        let cot_doc = cot_to_document(&cot_event, "example-peer");

        // Insert into Ditto store using the correct execute_v2 pattern
        if let Ok(flattened) = serde_json::to_value(&cot_doc) {
            let query = "INSERT INTO map_items DOCUMENTS (:doc) ON ID CONFLICT DO MERGE";
            let params = serde_json::json!({ "doc": flattened });
            match store.execute_v2((query, params)).await {
                Ok(_) => println!("✅ Inserted test location document"),
                Err(e) => println!("❌ Failed to insert document: {}", e),
            }
        }
    }

    println!("⏳ Waiting for observer callbacks...");
    sleep(Duration::from_secs(3)).await;

    println!("🏁 Example completed");
    Ok(())
}
