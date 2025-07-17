//! Cross-language E2E test utility: Create flattened document from XML
//!
//! This utility is called by the Java cross-language E2E test to verify that
//! Rust can create a flattened document that Java can correctly read.

use anyhow::{Context, Result};
use ditto_cot::{cot_events::CotEvent, ditto::cot_to_flattened_document};
use dittolive_ditto::fs::PersistentRoot;
use dittolive_ditto::prelude::*;
use std::env;
use std::fs;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <xml_file> <doc_id>", args[0]);
        std::process::exit(1);
    }

    let xml_file = &args[1];
    let doc_id = &args[2];

    // Load environment variables for Ditto
    // When called from Java tests, we need to find the .env file relative to where cargo is run
    let current_exe = env::current_exe().context("Failed to get current exe path")?;
    let rust_dir = current_exe
        .parent() // target/debug/examples
        .and_then(|p| p.parent()) // target/debug
        .and_then(|p| p.parent()) // target
        .and_then(|p| p.parent()) // rust directory
        .context("Failed to find rust directory")?;

    let env_file = rust_dir.join(".env");
    if env_file.exists() {
        dotenv::from_path(&env_file).context("Failed to load .env file")?;
    } else {
        // Fallback to standard dotenv loading
        dotenv::dotenv().ok();
    }

    // Get Ditto credentials
    let app_id = AppId::from_env("DITTO_APP_ID")
        .context("DITTO_APP_ID environment variable not set or invalid")?;
    let playground_token = env::var("DITTO_PLAYGROUND_TOKEN")
        .context("DITTO_PLAYGROUND_TOKEN environment variable not set")?;

    // Read the XML file
    let xml_content = fs::read_to_string(xml_file)
        .with_context(|| format!("Failed to read XML file: {}", xml_file))?;

    // Parse XML to CotEvent
    let cot_event = CotEvent::from_xml(&xml_content).context("Failed to parse CoT XML")?;

    // Convert to flattened document
    let mut flattened_doc = cot_to_flattened_document(&cot_event, "rust-peer");

    // Override the document ID to match what Java expects
    if let serde_json::Value::Object(ref mut map) = flattened_doc {
        map.insert("_id".to_string(), serde_json::Value::String(doc_id.clone()));
    }

    // Initialize Ditto
    let temp_dir = tempfile::tempdir().context("Failed to create temp dir")?;
    let ditto_path = temp_dir.path().join("ditto_data");
    let root = Arc::new(PersistentRoot::new(ditto_path)?);
    let cloud_sync = false; // Use peer-to-peer sync to match Java test
    let custom_auth_url: Option<&str> = None;

    let ditto = Ditto::builder()
        .with_root(root.clone())
        .with_identity(|_ditto_root| {
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

    let _ = ditto.disable_sync_with_v3();
    ditto.start_sync().context("Failed to start Ditto sync")?;
    let store = ditto.store();

    // Subscribe to map_items collection to enable sync using DQL
    let _subscription = store.register_observer_v2("SELECT * FROM map_items", move |_result| {
        // Observer callback - just needed to establish subscription
    })?;

    // Insert document using DQL
    let query = "INSERT INTO map_items DOCUMENTS (:doc) ON ID CONFLICT DO MERGE";
    let _query_result = store
        .execute_v2((
            &query,
            serde_json::json!({
                "doc": flattened_doc
            }),
        ))
        .await?;

    // Wait a moment for the document to be written
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Verify the document was stored
    let verify_query = format!("SELECT * FROM map_items WHERE _id = '{}'", doc_id);
    let verify_result = store.execute_v2(&verify_query).await?;

    if verify_result.item_count() == 0 {
        anyhow::bail!("Document was not stored successfully");
    }

    // Count r_* fields for verification
    let r_field_count = if let serde_json::Value::Object(ref map) = flattened_doc {
        map.keys().filter(|k| k.starts_with("r_")).count()
    } else {
        0
    };

    println!(
        "SUCCESS: Document '{}' created with {} r_* fields",
        doc_id, r_field_count
    );

    // Keep the Ditto client running for peer-to-peer sync
    // The Java test process needs to discover this peer and sync the document
    println!("Keeping Ditto client running for peer-to-peer sync...");
    println!("Press Ctrl+C or send SIGTERM to exit");

    // Set up signal handler to gracefully shutdown
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

    // Handle Ctrl+C signal
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl-c");
        let _ = tx.send(()).await;
    });

    // Keep running until signal received
    rx.recv().await;

    println!("Shutting down Rust client...");
    ditto.stop_sync();

    Ok(())
}
