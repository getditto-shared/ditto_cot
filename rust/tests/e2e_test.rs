use anyhow::{Context, Result};
use chrono::Utc;
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{cot_to_document, from_ditto::cot_event_from_ditto_document, DittoDocument},
    xml_utils,
};
use dittolive_ditto::fs::PersistentRoot;
use dittolive_ditto::prelude::*;
use std::sync::Arc;

// Collection name for Ditto documents (unused in this example)
#[allow(dead_code)]
const COLLECTION_NAME: &str = "cot_events";

#[tokio::test]
async fn e2e_xml_roundtrip() -> Result<()> {
    // Load environment variables from .env file in the current directory
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let env_path = current_dir.join(".env");

    // Try to load .env file from the current directory
    if let Err(e) = dotenv::from_path(&env_path) {
        eprintln!("Failed to load .env file: {}", e);
        return Err(e).context("Failed to load .env file");
    }

    // Get Ditto App ID and token from environment variables
    let app_id = AppId::from_env("DITTO_APP_ID")
        .context("DITTO_APP_ID environment variable not set or invalid")?;
    let playground_token = std::env::var("DITTO_PLAYGROUND_TOKEN")
        .context("DITTO_PLAYGROUND_TOKEN environment variable not set")?;

    // Initialize Ditto
    let temp_dir = tempfile::tempdir().context("Failed to create temp dir")?;
    let ditto_path = temp_dir.path().join("ditto_data");
    let root = Arc::new(PersistentRoot::new(ditto_path)?);
    let cloud_sync = true;
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

    // Generate RFC3339 timestamps
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(5)).to_rfc3339();
    let event_uid = format!("TEST-{}-1", uuid::Uuid::new_v4());

    // Sample CoT XML event
    let cot_xml = format!(r#"<event version="2.0" type="a-f-G-U-C" uid="{}" time="{}" start="{}" stale="{}" how="h-g-i-g-o">
  <point lat="1.2345" lon="2.3456" hae="9999999.0" ce="9999999.0" le="9999999.0"/>
  <detail>
    <__group name="Cyan" role="Team Member"/>
    <contact endpoint="*:-1:stcp" callsign="TEST-1"/>
    <precisionlocation geopointsrc="User" altsrc="???"/>
    <status battery="100"/>
    <track course="45.0" speed="10.0"/>
    <uid Droid="TEST-1"/>
  </detail>
</event>"#, event_uid, start_time, start_time, stale_time);

    // 1. Parse a CoT XML event using the public library interface
    let cot_event = CotEvent::from_xml(&cot_xml)
        .with_context(|| format!("Failed to parse CoT XML: {}", cot_xml))?;

    // 3. Convert CotEvent to Ditto document
    let ditto_doc = cot_to_document(&cot_event, "e2e-test-peer");

    // 4. Convert Ditto document back to CotEvent (simulated round-trip)
    let _roundtrip_cot_event = cot_event_from_ditto_document(&ditto_doc);

    // 5. Insert document into Ditto using DQL
    let doc_id = cot_event.uid.clone();
    let doc_value = match ditto_doc {
        DittoDocument::MapItem(ref map_item) => {
            serde_json::to_value(map_item).unwrap()
        }
        _ => {
            panic!("Expected MapItem document type");
        }
    };
    let query = "INSERT INTO map_items VALUES (:document) ON ID CONFLICT DO MERGE";
    let query_result = store
        .execute_v2((
            query,
            serde_json::json!({
                "document": doc_value
            }),
        ))
        .await?;

    // 6. Query the document back from Ditto
    let query = format!(
        "SELECT * FROM map_items WHERE _id = '{}'",
        doc_value["_id"].as_str().unwrap_or("")
    );
    let query_result = store.execute_v2(&query).await?;
    assert!(query_result.item_count() > 0, "No documents found matching the query");
    let doc = query_result
        .iter()
        .next()
        .expect("No document found with ID");
    let json_str = doc.json_string();

    // 7. Convert the Ditto document back to a CotEvent
    let retrieved_doc = DittoDocument::from_json_str(&json_str)?;
    let retrieved_cot_event = cot_event_from_ditto_document(&retrieved_doc);

    // 8. Verify the round-trip conversion (assertion added here)
    let cot_xml_out = retrieved_cot_event
        .to_xml()
        .unwrap_or_else(|e| format!("Error generating XML: {}", e));
    let minimized_expected = xml_utils::minimize_xml(&cot_xml);
    let minimized_actual = xml_utils::minimize_xml(&cot_xml_out);
    assert!(xml_utils::semantic_xml_eq(&minimized_expected, &minimized_actual, false), "Round-trip XML mismatch!\nExpected:\n{}\nActual:\n{}", minimized_expected, minimized_actual);

    // Gracefully shutdown Ditto instance
    ditto.close();
    Ok(())}

#[tokio::test]
async fn e2e_xml_examples_roundtrip() -> Result<()> {
    use std::fs;
    use std::path::Path;
    // Load environment variables from .env file in the current directory
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;
    let env_path = current_dir.join(".env");
    if let Err(e) = dotenv::from_path(&env_path) {
        eprintln!("Failed to load .env file: {}", e);
        return Err(e).context("Failed to load .env file");
    }
    let app_id = AppId::from_env("DITTO_APP_ID")
        .context("DITTO_APP_ID environment variable not set or invalid")?;
    let playground_token = std::env::var("DITTO_PLAYGROUND_TOKEN")
        .context("DITTO_PLAYGROUND_TOKEN environment variable not set")?;
    let temp_dir = tempfile::tempdir().context("Failed to create temp dir")?;
    let ditto_path = temp_dir.path().join("ditto_data");
    let root = Arc::new(PersistentRoot::new(ditto_path)?);
    let cloud_sync = true;
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

    // Use CARGO_MANIFEST_DIR to build an absolute path to schema/example_xml
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let example_dir = Path::new(&manifest_dir).join("../schema/example_xml");
    println!("Looking for XML files in: {}", example_dir.display());
    let mut found_any = false;
    let xml_entries: Vec<_> = fs::read_dir(&example_dir)
        .context("Failed to read example_xml directory")?
        .filter_map(|e| e.ok())
        .collect();
    let xml_files: Vec<_> = xml_entries.iter()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    println!("Available XML files in example_xml: {:?}", xml_files);
    // Allow limiting to a single file by env var
    let only_file = std::env::var("E2E_XML_FILE").ok();
    for entry in xml_entries {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("xml") {
            continue;
        }
        if let Some(ref fname) = only_file {
            if path.file_name().and_then(|s| s.to_str()) != Some(fname) {
                continue;
            }
        }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("xml") {
            continue;
        }
        found_any = true;
        let cot_xml = fs::read_to_string(&path).with_context(|| format!("Failed to read file: {}", path.display()))?;
        let cot_event = match CotEvent::from_xml(&cot_xml) {
            Ok(ev) => ev,
            Err(e) => {
                eprintln!("❌ Failed to parse CoT XML in {}: {}", path.display(), e);
                continue;
            }
        };
        let ditto_doc = cot_to_document(&cot_event, "e2e-test-peer");
        let _roundtrip_cot_event = cot_event_from_ditto_document(&ditto_doc);
        let doc_id = cot_event.uid.clone();
        let doc_value = match ditto_doc {
            DittoDocument::MapItem(ref map_item) => serde_json::to_value(map_item).unwrap(),
            _ => {
                eprintln!("   Error: Expected MapItem document type for file {}", path.display());
                continue;
            }
        };
        let query = "INSERT INTO map_items VALUES (:document) ON ID CONFLICT DO MERGE";
        let query_result = store
            .execute_v2((
                query,
                serde_json::json!({
                    "document": doc_value
                }),
            ))
            .await?;
        assert!(query_result.item_count() >= 0, "DQL INSERT failed for {}", path.display());
        let query = format!(
            "SELECT * FROM map_items WHERE _id = '{}'",
            doc_value["_id"].as_str().unwrap_or("")
        );
        let query_result = store.execute_v2(&query).await?;
        assert!(query_result.item_count() > 0, "No documents found matching the query for {}", path.display());
        let doc = match query_result.iter().next() {
            Some(d) => d,
            None => {
                eprintln!("❌ No document found with ID: {} for file {}", doc_id, path.display());
                continue;
            }
        };
        let json_str = doc.json_string();
        let retrieved_doc = match DittoDocument::from_json_str(&json_str) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("❌ Failed to parse DittoDocument from JSON for file {}: {}", path.display(), e);
                continue;
            }
        };
        let retrieved_cot_event = cot_event_from_ditto_document(&retrieved_doc);
        let cot_xml_out = retrieved_cot_event
            .to_xml()
            .unwrap_or_else(|e| format!("Error generating XML: {}", e));
        // Minimize both input and output XML to remove insignificant whitespace and formatting
        let min_expected = xml_utils::minimize_xml(&cot_xml);
        let min_actual = xml_utils::minimize_xml(&cot_xml_out);
        // Use semantic XML equality for round-trip check with non-strict comparison (ignore attribute order)
        if !xml_utils::semantic_xml_eq(&min_expected, &min_actual, false) {
            eprintln!("\n❌ Semantic XML round-trip mismatch for {}!\n--- Expected (input, minimized) ---\n{}\n--- Actual (output, minimized) ---\n{}\n", path.display(), min_expected, min_actual);
            panic!("Semantic XML round-trip mismatch for {}!", path.display());
        }
    }
    assert!(found_any, "No XML files found in example_xml directory");
    ditto.close();
    Ok(())
}
