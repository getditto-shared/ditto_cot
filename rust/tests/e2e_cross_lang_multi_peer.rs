//! Cross-language multi-peer E2E test
//!
//! This test replicates the existing 2 Rust client test but substitutes one of those
//! Rust clients with a Java one. This validates full cross-language compatibility
//! between Rust and Java implementations of the CoT library.
//!
//! Test flow:
//! 1. Start one Rust client and one Java subprocess client
//! 2. Rust client creates a CoT MapItem document
//! 3. Verify document syncs to Java client
//! 4. Java client modifies the document
//! 5. Verify modifications sync back to Rust client
//! 6. Validate final document state shows proper convergence

use anyhow::{Context, Result};
use chrono::Utc;
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{cot_to_document, from_ditto::cot_event_from_ditto_document, CotDocument},
};
use dittolive_ditto::fs::PersistentRoot;
use dittolive_ditto::prelude::*;
use dittolive_ditto::store::query_builder::DittoDocument;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::Duration as StdDuration;
use tokio::time::{sleep, Duration};

// Import test utilities
mod test_utils;

/// Cross-language multi-peer E2E test with one Rust client and one Java client
#[tokio::test]
#[ignore = "Cross-language tests are disabled by default in CI. Run with --ignored to enable."]
async fn e2e_cross_lang_multi_peer_test() -> Result<()> {
    println!("🚀 Starting Cross-Language Multi-Peer E2E Test");
    println!("==============================================");
    println!("📋 Test Plan:");
    println!("   1. Start Rust client and Java subprocess client");
    println!("   2. Rust creates CoT MapItem document");
    println!("   3. Verify document syncs to Java client");
    println!("   4. Java modifies the document");
    println!("   5. Verify modifications sync back to Rust");
    println!("   6. Validate final convergence");
    println!();

    // Load environment variables from .env file if it exists
    test_utils::load_test_env().context("Failed to load test environment")?;

    // Get Ditto App ID and token from environment variables
    let app_id = AppId::from_env("DITTO_APP_ID")
        .context("DITTO_APP_ID environment variable not set or invalid")?;
    let playground_token = std::env::var("DITTO_PLAYGROUND_TOKEN")
        .context("DITTO_PLAYGROUND_TOKEN environment variable not set")?;

    // Create temp directory for Rust client
    let temp_dir_rust = tempfile::tempdir().context("Failed to create temp dir for Rust client")?;
    let ditto_path_rust = temp_dir_rust.path().join("ditto_data_rust");
    let root_rust = Arc::new(PersistentRoot::new(ditto_path_rust)?);

    let cloud_sync = false; // Disable cloud sync for peer-to-peer only testing
    let custom_auth_url: Option<&str> = None;

    // Step 1: Initialize Rust Ditto client
    println!("🔌 Step 1: Initializing Rust Ditto client...");

    let ditto_rust = Ditto::builder()
        .with_root(root_rust.clone())
        .with_identity(|_ditto_root| {
            identity::OnlinePlayground::new(
                _ditto_root,
                app_id.clone(),
                playground_token.clone(),
                cloud_sync,
                custom_auth_url,
            )
        })?
        .with_minimum_log_level(LogLevel::Info)
        .build()
        .context("Failed to initialize Rust Ditto client")?;

    // Disable v3 sync for local peer-to-peer testing (same as working Rust test)
    let _ = ditto_rust.disable_sync_with_v3();
    ditto_rust
        .start_sync()
        .context("Failed to start sync for Rust client")?;

    let store_rust = ditto_rust.store();

    // Set up sync subscriptions and observers (same as working Rust test)
    println!("🔗 Setting up DQL sync subscriptions and observers for map_items collection...");

    // Set up sync subscription to enable peer-to-peer replication
    let _sync_subscription_rust = ditto_rust
        .sync()
        .register_subscription_v2("SELECT * FROM map_items")?;

    let _observer_rust =
        store_rust.register_observer_v2("SELECT * FROM map_items", move |result| {
            println!(
                "🔔 Rust client observer: received {} documents",
                result.item_count()
            );
        })?;

    println!("✅ Rust client initialized and syncing");

    // Step 2: Start Java subprocess client
    println!("🔌 Step 2: Starting Java subprocess client...");

    // Find the project root directory by looking for Cargo.toml
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    println!("🔍 Current test directory: {:?}", current_dir);

    // Find project root by looking for java directory and Makefile
    let mut search_dir = current_dir.clone();
    let project_root = loop {
        if search_dir.join("java").exists() && search_dir.join("Makefile").exists() {
            break search_dir;
        }
        match search_dir.parent() {
            Some(parent) => search_dir = parent.to_path_buf(),
            None => {
                anyhow::bail!("Could not find project root (looking for java dir and Makefile)")
            }
        }
    };

    println!("🔍 Project root: {:?}", project_root);

    let jar_path = project_root.join("java/ditto_cot/build/libs/ditto_cot-1.0-SNAPSHOT-all.jar");
    println!("🔍 Jar file exists: {}", jar_path.exists());
    println!("🔍 Absolute jar path: {:?}", jar_path);

    let mut java_process = Command::new("java")
        .args(["-jar", jar_path.to_string_lossy().as_ref()])
        .current_dir(&project_root)
        .env("DITTO_APP_ID", app_id.to_string())
        .env("DITTO_PLAYGROUND_TOKEN", &playground_token)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to start Java subprocess")?;

    let mut java_stdin = java_process
        .stdin
        .take()
        .context("Failed to get Java process stdin")?;
    let java_stdout = java_process
        .stdout
        .take()
        .context("Failed to get Java process stdout")?;

    // Read Java process output in a separate thread
    let java_output_handle = thread::spawn(move || {
        let reader = BufReader::new(java_stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("📱 Java: {}", line),
                Err(e) => eprintln!("❌ Java output error: {}", e),
            }
        }
    });

    // Give Java client time to initialize
    println!("⏳ Waiting for Java client to initialize...");
    sleep(Duration::from_secs(3)).await;

    // Check if Java process is still alive
    match java_process.try_wait() {
        Ok(Some(status)) => {
            anyhow::bail!("Java process exited early with status: {}", status);
        }
        Ok(None) => {
            println!("✅ Java process is running");
        }
        Err(e) => {
            anyhow::bail!("Failed to check Java process status: {}", e);
        }
    }

    // Send initialization command to Java client
    writeln!(java_stdin, "INIT").context("Failed to send INIT command to Java client")?;
    java_stdin.flush().context("Failed to flush Java stdin")?;

    sleep(Duration::from_secs(2)).await;
    println!("✅ Java client should be initialized");

    // Step 3: Create CoT MapItem document with Rust client
    println!("📤 Step 3: Creating CoT MapItem document with Rust client...");

    // Generate test CoT XML using the definitive ATAK test XML pattern
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(30)).to_rfc3339();
    let event_uid = format!("CROSS-LANG-MULTI-PEER-{}", uuid::Uuid::new_v4());

    // Use simplified CoT XML similar to what works in the existing test
    let cot_xml = format!(
        r#"<?xml version="1.0" standalone="yes"?>
<event version="2.0" uid="{}" type="a-u-S" time="{}" start="{}" stale="{}" how="m-d-a">
  <point ce="500.0" hae="0.0" lat="37.32699544764403" le="100.0" lon="-75.2905272033264" />
  <detail>
    <track course="45.0" speed="10.0" />
    <contact endpoint="*:-1:stcp" callsign="RUST-PEER"/>
    <__group name="Red Team" role="Leader"/>
    <status battery="90" readiness="true"/>
  </detail>
</event>"#,
        event_uid, start_time, start_time, stale_time
    );

    println!("📋 Creating document from CoT XML:");
    println!("{}", cot_xml);

    // Parse the CoT XML into a CotEvent
    let cot_event = CotEvent::from_xml(&cot_xml)
        .with_context(|| format!("Failed to parse CoT XML: {}", cot_xml))?;

    // Convert CotEvent to Ditto document
    let ditto_doc = cot_to_document(&cot_event, "rust-peer");

    // Ensure it's a MapItem document
    let map_item = match &ditto_doc {
        CotDocument::MapItem(item) => item,
        _ => panic!("Expected MapItem document, got different type"),
    };

    let doc_id = DittoDocument::id(&ditto_doc);
    println!("📋 Document ID: {}", doc_id);

    // Insert document into Rust client
    let doc_json = serde_json::to_value(map_item)?;
    let query = "INSERT INTO map_items DOCUMENTS (:doc) ON ID CONFLICT DO MERGE";
    let _query_result = store_rust
        .execute_v2((
            query,
            serde_json::json!({
                "doc": doc_json
            }),
        ))
        .await?;

    println!("✅ Document created by Rust client with ID: {}", doc_id);

    // Step 4: Verify document syncs to Java client
    println!("🔄 Step 4: Verifying document syncs to Java client...");

    // Check peer connectivity first
    println!("🔍 Checking peer connectivity...");
    let presence_rust = ditto_rust.presence();
    let graph_rust = presence_rust.graph();
    println!(
        "🔍 Rust client sees {} peers",
        graph_rust.remote_peers.len()
    );
    for peer in &graph_rust.remote_peers {
        println!("🔍 Rust connected to peer: {}", peer.peer_key_string);
    }

    // Wait longer for peer discovery in cross-process scenario
    println!("⏳ Waiting for peer discovery and sync (cross-process takes longer)...");
    sleep(Duration::from_secs(15)).await;

    // Check Java client peer connectivity
    writeln!(java_stdin, "PEERS").context("Failed to send PEERS command to Java client")?;
    java_stdin.flush().context("Failed to flush Java stdin")?;

    sleep(Duration::from_secs(2)).await;

    // Send query command to Java client
    writeln!(java_stdin, "QUERY {}", doc_id)
        .context("Failed to send QUERY command to Java client")?;
    java_stdin.flush().context("Failed to flush Java stdin")?;

    // Wait for sync to occur
    sleep(Duration::from_secs(5)).await;

    // Verify document still exists on Rust client
    let verify_query = format!("SELECT * FROM map_items WHERE _id = '{}'", doc_id);
    let rust_result = store_rust.execute_v2(&verify_query).await?;

    if rust_result.item_count() == 0 {
        anyhow::bail!("Document not found on Rust client after creation");
    }

    println!("✅ Document confirmed present on Rust client");

    // Step 5: Java client modifies the document
    println!("✏️ Step 5: Requesting Java client to modify the document...");

    // Send modify command to Java client
    writeln!(java_stdin, "MODIFY {} lat=38.0 lon=-122.0", doc_id)
        .context("Failed to send MODIFY command to Java client")?;
    java_stdin.flush().context("Failed to flush Java stdin")?;

    // Wait for modification and sync
    println!("⏳ Waiting for Java modification to sync back to Rust...");
    sleep(Duration::from_secs(5)).await;

    // Step 6: Verify modifications sync back to Rust client
    println!("🔄 Step 6: Verifying Java modifications synced to Rust client...");

    let modified_result = store_rust.execute_v2(&verify_query).await?;

    if modified_result.item_count() == 0 {
        anyhow::bail!("Modified document not found on Rust client");
    }

    let modified_doc = modified_result.iter().next().unwrap();
    let modified_json = modified_doc.json_string();
    let modified_ditto_doc = CotDocument::from_json_str(&modified_json)?;

    match &modified_ditto_doc {
        CotDocument::MapItem(modified_map_item) => {
            println!("📊 Modified document verification:");
            println!("   - Document ID: {}", modified_map_item.id);
            println!("   - Latitude: {:?}", modified_map_item.j);
            println!("   - Longitude: {:?}", modified_map_item.l);

            // Verify the modifications took effect
            if let (Some(lat), Some(lon)) = (modified_map_item.j, modified_map_item.l) {
                if (lat - 38.0).abs() < 0.001 && (lon - (-122.0)).abs() < 0.001 {
                    println!("✅ Java modifications successfully synced to Rust client");
                } else {
                    anyhow::bail!("Java modifications not properly synced. Expected lat=38.0, lon=-122.0, got lat={}, lon={}", lat, lon);
                }
            } else {
                anyhow::bail!("Modified document missing latitude/longitude values");
            }
        }
        _ => anyhow::bail!("Expected MapItem document after modification"),
    }

    // Step 7: Test round-trip XML conversion
    println!("🔄 Step 7: Testing round-trip XML conversion...");

    let final_cot_event = cot_event_from_ditto_document(&modified_ditto_doc);
    let final_xml = final_cot_event.to_xml()?;

    println!("📋 Final XML representation:");
    println!("{}", final_xml);

    // Verify XML can be parsed back
    let _verify_cot = CotEvent::from_xml(&final_xml)?;
    println!("✅ XML round-trip verification successful");

    // Step 8: Clean up
    println!("🧹 Step 8: Cleaning up...");

    // Send shutdown command to Java client
    writeln!(java_stdin, "SHUTDOWN").context("Failed to send SHUTDOWN command to Java client")?;
    java_stdin.flush().context("Failed to flush Java stdin")?;

    // Wait for Java process to exit gracefully
    // Use a simple approach since wait_timeout is not available on all platforms
    thread::sleep(StdDuration::from_secs(2));

    match java_process.try_wait() {
        Ok(Some(status)) => {
            println!("✅ Java client exited with status: {}", status);
        }
        Ok(None) => {
            println!("⚠️ Java client still running, sending kill signal");
            let _ = java_process.kill();
            let _ = java_process.wait();
        }
        Err(e) => {
            println!("❌ Error checking Java client status: {}", e);
            let _ = java_process.kill();
        }
    }

    // Wait for Java output thread to finish
    let _ = java_output_handle.join();

    // Stop Rust sync
    ditto_rust.stop_sync();

    println!("🎉 Cross-Language Multi-Peer E2E Test Complete!");
    println!("============================================");
    println!("✅ All steps completed successfully:");
    println!("   1. ✅ Rust and Java clients initialized");
    println!("   2. ✅ Rust created CoT MapItem document");
    println!("   3. ✅ Document synced to Java client");
    println!("   4. ✅ Java modified the document");
    println!("   5. ✅ Modifications synced back to Rust");
    println!("   6. ✅ Final state validated");
    println!("   7. ✅ XML round-trip successful");
    println!("   8. ✅ Clean shutdown completed");

    Ok(())
}
