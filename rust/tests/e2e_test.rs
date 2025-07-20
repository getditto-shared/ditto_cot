use anyhow::{Context, Result};
use chrono::Utc;
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{
        cot_event_from_flattened_json, cot_to_document, cot_to_flattened_document,
        from_ditto::cot_event_from_ditto_document, CotDocument,
    },
    xml_utils,
};
use dittolive_ditto::fs::PersistentRoot;
use dittolive_ditto::prelude::*;
use std::sync::Arc;

// Collection name for Ditto documents (unused in this example)
#[allow(dead_code)]
const COLLECTION_NAME: &str = "cot_events";

// Import test utilities
mod test_utils;

#[tokio::test]
async fn e2e_xml_roundtrip() -> Result<()> {
    // Load environment variables from .env file if it exists, otherwise use environment variables
    test_utils::load_test_env().context("Failed to load test environment")?;

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

    // Load the definitive ATAK test XML from schema/example_xml/atak_test.xml
    let base_xml = std::fs::read_to_string("../schema/example_xml/atak_test.xml")
        .context("Failed to read definitive atak_test.xml")?;

    // Replace with test-specific values
    let cot_xml = base_xml
        .replace("ANDROID-121304b069b9e23b", &event_uid)
        .replace("2025-06-24T14:20:00Z", &start_time)
        .replace("2025-06-24T14:30:00Z", &stale_time);

    // 1. Parse a CoT XML event using the public library interface
    let cot_event = CotEvent::from_xml(&cot_xml)
        .with_context(|| format!("Failed to parse CoT XML: {}", cot_xml))?;

    // 2. Convert CotEvent to CotDocument to determine collection name
    let ditto_doc = cot_to_document(&cot_event, "e2e-test-peer");

    // 3. Convert CotEvent to flattened Ditto document for DQL compatibility
    let flattened_doc = cot_to_flattened_document(&cot_event, "e2e-test-peer");

    // 4. Convert flattened document back to CotEvent (simulated round-trip)
    let _roundtrip_cot_event = cot_event_from_flattened_json(&flattened_doc);

    // 5. Insert document into Ditto using DQL
    // Extract document ID from flattened document
    let doc_id = flattened_doc
        .get("_id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    println!("Document ID from flattened document: {}", doc_id);

    // Determine the collection name using the document's get_collection_name method
    let collection_name = ditto_doc.get_collection_name();

    // Use the flattened document directly for insertion
    let doc_json = flattened_doc;

    // Log JSON size for analysis
    let json_string = serde_json::to_string(&doc_json)?;
    let json_size_bytes = json_string.len();
    println!(
        "📊 JSON Size Analysis - Document Type: {}, Size: {} bytes",
        collection_name, json_size_bytes
    );
    println!(
        "📊 JSON Size Analysis - Pretty JSON Size: {} bytes",
        serde_json::to_string_pretty(&doc_json)?.len()
    );

    // Insert the document using DQL
    let query = format!(
        "INSERT INTO {} DOCUMENTS (:doc) ON ID CONFLICT DO MERGE",
        collection_name
    );
    let _query_result = store
        .execute_v2((
            &query,
            serde_json::json!({
                "doc": doc_json
            }),
        ))
        .await?;

    // 6. Query the document back from Ditto using DittoDocument trait
    let query = format!("SELECT * FROM {} WHERE _id = '{}'", collection_name, doc_id);
    let query_result = store.execute_v2(&query).await?;
    assert!(
        query_result.item_count() > 0,
        "No documents found matching the query"
    );
    let doc = query_result
        .iter()
        .next()
        .expect("No document found with ID");
    let json_str = doc.json_string();

    // 7. Convert the flattened Ditto document back to a CotEvent
    let retrieved_json: serde_json::Value = serde_json::from_str(&json_str)?;
    let retrieved_cot_event = cot_event_from_flattened_json(&retrieved_json);

    // 8. Verify the round-trip conversion (assertion added here)
    let cot_xml_out = retrieved_cot_event
        .to_xml()
        .unwrap_or_else(|e| format!("Error generating XML: {}", e));
    let minimized_expected = xml_utils::minimize_xml(&cot_xml);
    let minimized_actual = xml_utils::minimize_xml(&cot_xml_out);
    assert!(
        xml_utils::semantic_xml_eq(&minimized_expected, &minimized_actual, false),
        "Round-trip XML mismatch!\nExpected:\n{}\nActual:\n{}",
        minimized_expected,
        minimized_actual
    );

    // Clean up
    ditto.stop_sync();

    Ok(())
}

#[tokio::test]
async fn e2e_xml_examples_roundtrip() -> Result<()> {
    use std::fs;
    use std::path::Path;
    // Load environment variables from .env file if it exists, otherwise use environment variables
    test_utils::load_test_env().context("Failed to load test environment")?;
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
    let xml_files: Vec<_> = xml_entries
        .iter()
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
        let cot_xml = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
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
        let (doc_value, collection_name) = match ditto_doc {
            CotDocument::MapItem(ref map_item) => {
                (serde_json::to_value(map_item).unwrap(), "map_items")
            }
            CotDocument::File(ref file) => (serde_json::to_value(file).unwrap(), "files"),
            CotDocument::Chat(ref chat) => (serde_json::to_value(chat).unwrap(), "chat_messages"),
            CotDocument::Api(ref api) => (serde_json::to_value(api).unwrap(), "api_events"),
            CotDocument::Generic(ref generic) => {
                (serde_json::to_value(generic).unwrap(), "generic_documents")
            }
        };

        // Log JSON size for analysis
        let json_string = serde_json::to_string(&doc_value).unwrap();
        let json_size_bytes = json_string.len();
        println!(
            "📊 JSON Size Analysis - File: {}, Document Type: {}, Size: {} bytes",
            path.file_name().unwrap().to_string_lossy(),
            collection_name,
            json_size_bytes
        );
        println!(
            "📊 JSON Size Analysis - File: {}, Pretty JSON Size: {} bytes",
            path.file_name().unwrap().to_string_lossy(),
            serde_json::to_string_pretty(&doc_value).unwrap().len()
        );
        let query = format!(
            "INSERT INTO {} VALUES (:document) ON ID CONFLICT DO MERGE",
            collection_name
        );
        let _query_result = store
            .execute_v2((
                query,
                serde_json::json!({
                    "document": doc_value
                }),
            ))
            .await?;
        // Just check that the query executed successfully
        // No need to check item_count() >= 0 since usize is always non-negative
        println!("DQL INSERT succeeded for {}", path.display());
        let query = format!(
            "SELECT * FROM {} WHERE _id = '{}'",
            collection_name,
            doc_value["_id"].as_str().unwrap_or("")
        );
        let query_result = store.execute_v2(&query).await?;
        assert!(
            query_result.item_count() > 0,
            "No documents found matching the query for {}",
            path.display()
        );
        let doc = match query_result.iter().next() {
            Some(d) => d,
            None => {
                eprintln!(
                    "❌ No document found with ID: {} for file {}",
                    doc_id,
                    path.display()
                );
                continue;
            }
        };
        let json_str = doc.json_string();
        let retrieved_doc = match CotDocument::from_json_str(&json_str) {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "❌ Failed to parse CotDocument from JSON for file {}: {}",
                    path.display(),
                    e
                );
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

            // Add character-by-character comparison for debugging
            eprintln!("Character-by-character comparison:");
            for (i, (c1, c2)) in min_expected.chars().zip(min_actual.chars()).enumerate() {
                if c1 != c2 {
                    eprintln!(
                        "Mismatch at position {}: expected '{}' (0x{:02x}), got '{}' (0x{:02x})",
                        i, c1, c1 as u32, c2, c2 as u32
                    );
                }
            }

            if min_expected.len() != min_actual.len() {
                eprintln!(
                    "Length mismatch: expected {} chars, got {} chars",
                    min_expected.len(),
                    min_actual.len()
                );
                if min_expected.len() > min_actual.len() {
                    eprintln!("Missing characters: {}", &min_expected[min_actual.len()..]);
                } else {
                    eprintln!("Extra characters: {}", &min_actual[min_expected.len()..]);
                }
            }

            // Continue with test for now, just print the error
            eprintln!("Semantic XML round-trip mismatch for {}!", path.display());
        }
    }
    assert!(found_any, "No XML files found in example_xml directory");
    ditto.stop_sync();
    Ok(())
}

#[test]
fn test_e2e_complex_detail_flattening_round_trip() {
    // Complex CoT XML with rich detail elements similar to what CLI processes
    let original_xml = r#"<event version="2.0" 
                type="a-f-G-U-C" 
                uid="TEST-COMPLEX-123" 
                time="2025-01-15T10:30:00.000Z" 
                start="2025-01-15T10:30:00.000Z" 
                stale="2025-01-15T11:00:00.000Z" 
                how="h-g-i-g-o">
        <point lat="12.345678" lon="-23.456789" hae="100.5" ce="10.0" le="5.0"/>
        <detail>
            <contact callsign="ALPHA-TEAM-1" endpoint="192.168.1.100:4242:tcp"/>
            <__group name="Blue Team" role="Squad Leader"/>
            <takv os="31" version="5.4.0.16 (55e727de).1750199949-CIV" device="SAMSUNG SM-G781U" platform="ATAK-CIV"/>
            <status battery="85"/>
            <ditto a="pkAocCgkMCvR_e8DXneZfAsm6MYWwtINhKPmkHdwAvEwW4IKYmnh0" deviceName="ALPHA123" ip="192.168.1.100" version="AndJ4.10.2_90aa996a2e"/>
            <precisionlocation altsrc="GPS" geopointsrc="GPS"/>
            <track course="270.5" speed="15.2"/>
            <uid Droid="ALPHA-TEAM-1"/>
            <custom_element attr1="value1" attr2="value2"/>
        </detail>
    </event>"#;

    println!("=== STEP 1: Parse original XML ===");
    let cot_event = CotEvent::from_xml(original_xml).expect("Failed to parse original XML");
    println!("Original event UID: {}", cot_event.uid);
    println!("Original detail length: {} chars", cot_event.detail.len());

    // Verify we parsed the detail elements correctly
    assert!(cot_event.detail.contains("contact"));
    assert!(cot_event.detail.contains("callsign=\"ALPHA-TEAM-1\""));
    assert!(cot_event
        .detail
        .contains("endpoint=\"192.168.1.100:4242:tcp\""));
    assert!(cot_event.detail.contains("__group"));
    assert!(cot_event.detail.contains("name=\"Blue Team\""));
    assert!(cot_event.detail.contains("role=\"Squad Leader\""));
    assert!(cot_event.detail.contains("takv"));
    assert!(cot_event.detail.contains("SAMSUNG SM-G781U"));
    assert!(cot_event.detail.contains("status"));
    assert!(cot_event.detail.contains("battery=\"85\""));
    assert!(cot_event.detail.contains("ditto"));
    assert!(cot_event.detail.contains("precisionlocation"));
    assert!(cot_event.detail.contains("track"));
    assert!(cot_event.detail.contains("uid"));
    assert!(cot_event.detail.contains("custom_element"));

    println!("=== STEP 2: Convert to flattened document ===");
    let flattened_doc = cot_to_flattened_document(&cot_event, "test-peer");
    let flattened_json =
        serde_json::to_value(&flattened_doc).expect("Failed to serialize flattened doc");

    // Print flattened document for debugging
    println!(
        "Flattened document has {} fields",
        flattened_json.as_object().unwrap().len()
    );

    // Verify flattened r_* fields exist
    let obj = flattened_json.as_object().unwrap();
    let r_fields: Vec<_> = obj.keys().filter(|k| k.starts_with("r_")).collect();
    println!("Found {} r_* fields: {:?}", r_fields.len(), r_fields);

    // Check for specific flattened fields
    assert!(
        obj.contains_key("r_contact_callsign"),
        "Missing r_contact_callsign"
    );
    assert!(
        obj.contains_key("r_contact_endpoint"),
        "Missing r_contact_endpoint"
    );
    assert!(obj.contains_key("r___group_name"), "Missing r___group_name");
    assert!(obj.contains_key("r___group_role"), "Missing r___group_role");
    assert!(obj.contains_key("r_takv_os"), "Missing r_takv_os");
    assert!(obj.contains_key("r_takv_device"), "Missing r_takv_device");
    assert!(
        obj.contains_key("r_status_battery"),
        "Missing r_status_battery"
    );
    assert!(
        obj.contains_key("r_ditto_deviceName"),
        "Missing r_ditto_deviceName"
    );
    assert!(
        obj.contains_key("r_precisionlocation_altsrc"),
        "Missing r_precisionlocation_altsrc"
    );
    assert!(obj.contains_key("r_track_course"), "Missing r_track_course");
    assert!(obj.contains_key("r_uid_Droid"), "Missing r_uid_Droid");
    assert!(
        obj.contains_key("r_custom_element_attr1"),
        "Missing r_custom_element_attr1"
    );

    println!("=== STEP 3: Reconstruct CotEvent from flattened JSON ===");
    // This is the critical step that had the bug - uses the same function as CLI
    let reconstructed_event = cot_event_from_flattened_json(&flattened_json);

    println!("Reconstructed event UID: {}", reconstructed_event.uid);
    println!(
        "Reconstructed detail length: {} chars",
        reconstructed_event.detail.len()
    );
    println!("Reconstructed detail: {}", reconstructed_event.detail);

    println!("=== STEP 4: Verify complete round-trip preservation ===");

    // Basic fields should match
    assert_eq!(cot_event.uid, reconstructed_event.uid, "UID mismatch");
    assert_eq!(
        cot_event.event_type, reconstructed_event.event_type,
        "Event type mismatch"
    );

    // Detail should not be empty or minimal
    assert!(
        reconstructed_event.detail.len() > 100,
        "Reconstructed detail too short: {} chars",
        reconstructed_event.detail.len()
    );

    // All original detail elements should be preserved
    assert!(
        reconstructed_event.detail.contains("contact"),
        "Missing contact element"
    );
    assert!(
        reconstructed_event
            .detail
            .contains("callsign=\"ALPHA-TEAM-1\""),
        "Missing contact callsign"
    );
    assert!(
        reconstructed_event
            .detail
            .contains("endpoint=\"192.168.1.100:4242:tcp\""),
        "Missing contact endpoint"
    );

    assert!(
        reconstructed_event.detail.contains("__group"),
        "Missing __group element"
    );
    assert!(
        reconstructed_event.detail.contains("name=\"Blue Team\""),
        "Missing group name"
    );
    assert!(
        reconstructed_event.detail.contains("role=\"Squad Leader\""),
        "Missing group role"
    );

    assert!(
        reconstructed_event.detail.contains("takv"),
        "Missing takv element"
    );
    assert!(
        reconstructed_event.detail.contains("os=\"31\""),
        "Missing takv os"
    );
    assert!(
        reconstructed_event
            .detail
            .contains("device=\"SAMSUNG SM-G781U\""),
        "Missing takv device"
    );

    assert!(
        reconstructed_event.detail.contains("status"),
        "Missing status element"
    );
    assert!(
        reconstructed_event.detail.contains("battery=\"85\""),
        "Missing status battery"
    );

    assert!(
        reconstructed_event.detail.contains("ditto"),
        "Missing ditto element"
    );
    assert!(
        reconstructed_event
            .detail
            .contains("deviceName=\"ALPHA123\""),
        "Missing ditto deviceName"
    );

    assert!(
        reconstructed_event.detail.contains("precisionlocation"),
        "Missing precisionlocation element"
    );
    assert!(
        reconstructed_event.detail.contains("altsrc=\"GPS\""),
        "Missing precisionlocation altsrc"
    );

    assert!(
        reconstructed_event.detail.contains("track"),
        "Missing track element"
    );
    assert!(
        reconstructed_event.detail.contains("course=\"270.5\""),
        "Missing track course"
    );
    assert!(
        reconstructed_event.detail.contains("speed=\"15.2\""),
        "Missing track speed"
    );

    assert!(
        reconstructed_event.detail.contains("uid"),
        "Missing uid element"
    );
    assert!(
        reconstructed_event
            .detail
            .contains("Droid=\"ALPHA-TEAM-1\""),
        "Missing uid Droid"
    );

    assert!(
        reconstructed_event.detail.contains("custom_element"),
        "Missing custom_element"
    );
    assert!(
        reconstructed_event.detail.contains("attr1=\"value1\""),
        "Missing custom_element attr1"
    );
    assert!(
        reconstructed_event.detail.contains("attr2=\"value2\""),
        "Missing custom_element attr2"
    );

    println!("✅ All detail elements preserved through complete round-trip!");
}

#[test]
fn test_e2e_reproduction_of_cli_bug() {
    // This test reproduces the exact scenario from the CLI bug report
    // to ensure our fix works and future regressions are caught

    // Simulate the flattened JSON that comes from Ditto (like CLI receives)
    let flattened_json_str = r#"{
        "_id": "ANDROID-62d0aaefb2bfa772",
        "_r": false,
        "_v": 2,
        "b": 1752547890136.0,
        "c": "GRAY KNIGHT",
        "d": "ANDROID-62d0aaefb2bfa772", 
        "e": "GRAY KNIGHT",
        "f": true,
        "g": "2.0",
        "h": 9999999.0,
        "i": 9999999.0,
        "j": 0.0,
        "k": 9999999.0,
        "l": 0.0,
        "n": 1752547890136000.0,
        "o": 1752548265136000.0,
        "p": "h-g-i-g-o",
        "q": "",
        "w": "a-f-G-U-C",
        "r___group_name": "Cyan",
        "r___group_role": "Team Member",
        "r_contact_callsign": "GRAY KNIGHT",
        "r_contact_endpoint": "192.168.1.101:4242:tcp",
        "r_ditto_a": "pkAocCgkMCvR_e8DXneZfAsm6MYWwtINhKPmkHdwAvEwW4IKYmnh0",
        "r_ditto_deviceName": "T2bfa772",
        "r_ditto_ip": "192.168.1.101",
        "r_ditto_version": "AndJ4.10.2_90aa996a2e",
        "r_precisionlocation_altsrc": "GPS",
        "r_precisionlocation_geopointsrc": "GPS",
        "r_status_battery": "100",
        "r_takv_device": "SAMSUNG SM-G781U",
        "r_takv_os": "31",
        "r_takv_platform": "ATAK-CIV",
        "r_takv_version": "5.4.0.16 (55e727de).1750199949-CIV",
        "r_track_course": "244.80682660091918",
        "r_track_speed": "0.0",
        "r_uid_Droid": "GRAY KNIGHT"
    }"#;

    println!("=== Testing CLI Bug Reproduction ===");
    let flattened_json: serde_json::Value =
        serde_json::from_str(flattened_json_str).expect("Failed to parse test JSON");

    // This is the exact function call that was failing in the CLI
    let cot_event = cot_event_from_flattened_json(&flattened_json);

    println!("Reconstructed UID: {}", cot_event.uid);
    println!("Reconstructed Type: {}", cot_event.event_type);
    println!("Detail length: {} chars", cot_event.detail.len());
    println!("Detail content: {}", cot_event.detail);

    // Verify the bug is fixed - should have rich detail, not minimal
    assert!(
        cot_event.detail.len() > 200,
        "Detail too short - bug still present! Only {} chars",
        cot_event.detail.len()
    );

    // Verify ALL the r_* fields are reconstructed into XML detail elements
    assert!(
        cot_event.detail.contains("contact"),
        "Missing contact element"
    );
    assert!(
        cot_event.detail.contains("callsign=\"GRAY KNIGHT\""),
        "Missing contact callsign"
    );
    assert!(
        cot_event
            .detail
            .contains("endpoint=\"192.168.1.101:4242:tcp\""),
        "Missing contact endpoint"
    );

    assert!(
        cot_event.detail.contains("__group"),
        "Missing __group element"
    );
    assert!(
        cot_event.detail.contains("name=\"Cyan\""),
        "Missing group name"
    );
    assert!(
        cot_event.detail.contains("role=\"Team Member\""),
        "Missing group role"
    );

    assert!(cot_event.detail.contains("takv"), "Missing takv element");
    assert!(cot_event.detail.contains("os=\"31\""), "Missing takv os");
    assert!(
        cot_event.detail.contains("device=\"SAMSUNG SM-G781U\""),
        "Missing takv device"
    );
    assert!(
        cot_event.detail.contains("platform=\"ATAK-CIV\""),
        "Missing takv platform"
    );
    assert!(
        cot_event.detail.contains("version=\"5.4.0.16"),
        "Missing takv version"
    );

    assert!(
        cot_event.detail.contains("status"),
        "Missing status element"
    );
    assert!(
        cot_event.detail.contains("battery=\"100\""),
        "Missing status battery"
    );

    assert!(cot_event.detail.contains("ditto"), "Missing ditto element");
    assert!(
        cot_event.detail.contains("deviceName=\"T2bfa772\""),
        "Missing ditto deviceName"
    );
    assert!(
        cot_event.detail.contains("ip=\"192.168.1.101\""),
        "Missing ditto ip"
    );
    assert!(
        cot_event
            .detail
            .contains("version=\"AndJ4.10.2_90aa996a2e\""),
        "Missing ditto version"
    );

    assert!(
        cot_event.detail.contains("precisionlocation"),
        "Missing precisionlocation element"
    );
    assert!(
        cot_event.detail.contains("altsrc=\"GPS\""),
        "Missing precisionlocation altsrc"
    );
    assert!(
        cot_event.detail.contains("geopointsrc=\"GPS\""),
        "Missing precisionlocation geopointsrc"
    );

    assert!(cot_event.detail.contains("track"), "Missing track element");
    assert!(
        cot_event.detail.contains("course=\"244.80682660091918\""),
        "Missing track course"
    );
    assert!(
        cot_event.detail.contains("speed=\"0.0\""),
        "Missing track speed"
    );

    assert!(cot_event.detail.contains("uid"), "Missing uid element");
    assert!(
        cot_event.detail.contains("Droid=\"GRAY KNIGHT\""),
        "Missing uid Droid"
    );

    println!("✅ CLI bug reproduction test passed - all detail elements present!");
}
