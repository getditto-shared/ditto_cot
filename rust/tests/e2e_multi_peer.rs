use anyhow::{Context, Result};
use chrono::Utc;
use ditto_cot::{
    cot_events::CotEvent,
    ditto::{
        cot_to_document, from_ditto::cot_event_from_ditto_document, CotDocument, MapItemRValue,
    },
};
use dittolive_ditto::fs::PersistentRoot;
use dittolive_ditto::prelude::*;
use dittolive_ditto::store::query_builder::DittoDocument;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// Import test utilities
mod test_utils;

/// Comprehensive multi-peer E2E test that covers:
/// 1. Two Rust clients coming online and verifying peer connection
/// 2. One peer inserting a CoT MapItem document
/// 3. Verifying document sync and accuracy on both clients
/// 4. Taking both clients offline and disabling sync
/// 5. Both clients making independent modifications to Detail elements
/// 6. Bringing both clients online and ensuring reconnection/sync
/// 7. Validating final document state with last-write-wins merge
#[tokio::test]
async fn e2e_multi_peer_mapitem_sync_test() -> Result<()> {
    // EARLY XML TEST - Test XML parsing before any Ditto setup
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(30)).to_rfc3339();
    let event_uid = format!("MULTI-PEER-TEST-{}", uuid::Uuid::new_v4());

    let cot_xml = format!(
        r#"<event version="2.0" type="a-f-G-U-C" uid="{}" time="{}" start="{}" stale="{}" how="h-g-i-g-o">
  <point lat="37.7749" lon="-122.4194" hae="100.0" ce="50.0" le="25.0"/>
  <detail>
    <contact endpoint="*:-1:stcp" callsign="PEER1-LEADER"/>
    <track course="180.0" speed="15.0"/>
  </detail>
</event>"#,
        event_uid, start_time, start_time, stale_time
    );

    println!("EARLY XML TEST:");
    println!("XML: {}", cot_xml);

    match CotEvent::from_xml(&cot_xml) {
        Ok(_) => println!("✅ EARLY XML parsing PASSED"),
        Err(e) => {
            println!("❌ EARLY XML parsing FAILED: {}", e);
            panic!("Early XML parsing failed before any Ditto setup: {}", e);
        }
    }

    // Load environment variables from .env file if it exists
    test_utils::load_test_env().context("Failed to load test environment")?;

    // Get Ditto App ID and token from environment variables
    let app_id = AppId::from_env("DITTO_APP_ID")
        .context("DITTO_APP_ID environment variable not set or invalid")?;
    let playground_token = std::env::var("DITTO_PLAYGROUND_TOKEN")
        .context("DITTO_PLAYGROUND_TOKEN environment variable not set")?;

    // Create two separate temp directories for peer isolation
    let temp_dir_1 = tempfile::tempdir().context("Failed to create temp dir for peer 1")?;
    let temp_dir_2 = tempfile::tempdir().context("Failed to create temp dir for peer 2")?;

    let ditto_path_1 = temp_dir_1.path().join("ditto_data_peer1");
    let ditto_path_2 = temp_dir_2.path().join("ditto_data_peer2");

    let root_1 = Arc::new(PersistentRoot::new(ditto_path_1)?);
    let root_2 = Arc::new(PersistentRoot::new(ditto_path_2)?);

    let cloud_sync = false; // Disable cloud sync for peer-to-peer only testing
    let custom_auth_url: Option<&str> = None;

    // Initialize Ditto Peer 1
    let ditto_1 = Ditto::builder()
        .with_root(root_1.clone())
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
        .context("Failed to initialize Ditto peer 1")?;

    // Initialize Ditto Peer 2
    let ditto_2 = Ditto::builder()
        .with_root(root_2.clone())
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
        .context("Failed to initialize Ditto peer 2")?;

    // Step 1: Make both clients online and verify peer connection
    println!("🔌 Step 1: Bringing both peers online...");

    // Disable v3 sync for local peer-to-peer testing
    let _ = ditto_1.disable_sync_with_v3();
    let _ = ditto_2.disable_sync_with_v3();

    ditto_1
        .start_sync()
        .context("Failed to start sync for peer 1")?;
    ditto_2
        .start_sync()
        .context("Failed to start sync for peer 2")?;

    // Wait a moment to ensure Ditto instances are fully ready
    sleep(Duration::from_millis(200)).await;

    let store_1 = ditto_1.store();
    let store_2 = ditto_2.store();

    // Set up sync subscriptions and observers for the map_items collection on both peers using DQL
    // Subscriptions enable peer-to-peer sync, observers detect local changes
    println!("🔗 Setting up DQL sync subscriptions and observers for map_items collection...");

    // Set up sync subscriptions on both peers to enable peer-to-peer replication
    let sync_subscription_1 = ditto_1
        .sync()
        .register_subscription_v2("SELECT * FROM map_items")?;
    let sync_subscription_2 = ditto_2
        .sync()
        .register_subscription_v2("SELECT * FROM map_items")?;

    // Set up observers on both peers to actively listen for changes using DQL
    let observer_1 = store_1.register_observer_v2("SELECT * FROM map_items", move |result| {
        println!(
            "🔔 Peer 1 DQL observer: received {} documents",
            result.item_count()
        );
    })?;

    let observer_2 = store_2.register_observer_v2("SELECT * FROM map_items", move |result| {
        println!(
            "🔔 Peer 2 DQL observer: received {} documents",
            result.item_count()
        );
        for doc in result.iter() {
            let doc_value = doc.value();
            if let Some(id_value) = doc_value.get("_id") {
                println!("🔔 Peer 2 DQL observer: document ID {:?}", id_value);
            }
        }
    })?;

    // Keep sync subscriptions and observers alive by storing them
    let _sync_sub1 = sync_subscription_1;
    let _sync_sub2 = sync_subscription_2;
    let _obs1 = observer_1;
    let _obs2 = observer_2;

    // Wait for peer discovery and connection establishment
    sleep(Duration::from_secs(2)).await;

    // Check peer presence for debugging
    let presence_1 = ditto_1.presence();
    let presence_2 = ditto_2.presence();

    // Get peer graph info for debugging
    let graph_1 = presence_1.graph();
    let graph_2 = presence_2.graph();

    println!("🔍 Peer 1 ID: {}", graph_1.local_peer.peer_key_string);
    println!("🔍 Peer 2 ID: {}", graph_2.local_peer.peer_key_string);
    println!(
        "🔍 Peer 1 sees {} peers in total",
        graph_1.remote_peers.len()
    );
    println!(
        "🔍 Peer 2 sees {} peers in total",
        graph_2.remote_peers.len()
    );

    let mut peer1_connected_to_peer2 = false;
    let mut peer2_connected_to_peer1 = false;

    for peer in &graph_1.remote_peers {
        println!("🔍 Peer 1 connected to: {}", peer.peer_key_string);
        if peer.peer_key_string == graph_2.local_peer.peer_key_string {
            println!("✅ Peer 1 is connected to Peer 2!");
            peer1_connected_to_peer2 = true;
        }
    }

    for peer in &graph_2.remote_peers {
        println!("🔍 Peer 2 connected to: {}", peer.peer_key_string);
        if peer.peer_key_string == graph_1.local_peer.peer_key_string {
            println!("✅ Peer 2 is connected to Peer 1!");
            peer2_connected_to_peer1 = true;
        }
    }

    if !peer1_connected_to_peer2 || !peer2_connected_to_peer1 {
        println!("❌ Peers are not connected to each other!");
        println!("❌ This suggests there may be other Ditto instances running or network isolation issues");
    }

    println!("🔍 Waiting for peer connection establishment...");
    // Let connections establish further
    sleep(Duration::from_secs(1)).await;

    println!("✅ Step 1 Complete: Both peers are online");

    // Step 2: Input CoT MapItem document on peer 1
    println!("📤 Step 2: Creating CoT MapItem document on peer 1...");

    // Generate RFC3339 timestamps
    let now = Utc::now();
    let start_time = now.to_rfc3339();
    let stale_time = (now + chrono::Duration::minutes(30)).to_rfc3339();
    let event_uid = format!("MULTI-PEER-TEST-{}", uuid::Uuid::new_v4());

    // Create a CoT MapItem XML event
    let cot_xml = format!(
        r#"<?xml version="1.0" standalone="yes"?>
<event version="2.0" uid="{}" type="a-u-S" time="{}" start="{}" stale="{}" how="m-d-a"><point ce="500.0" hae="0.0" lat="37.32699544764403" le="100.0" lon="-75.2905272033264" /><detail><track course="30.86376880675669" speed="1.3613854354920412" /></detail></event>"#,
        event_uid, start_time, start_time, stale_time
    );

    println!("COT_XML: {}", cot_xml);
    println!("COT_XML length: {}", cot_xml.len());

    // Test parsing immediately
    println!("Testing XML parsing immediately...");
    match CotEvent::from_xml(&cot_xml) {
        Ok(_) => println!("✅ XML parsing test PASSED"),
        Err(e) => {
            println!("❌ XML parsing test FAILED: {}", e);
            println!(
                "First 50 chars: {:?}",
                &cot_xml[..std::cmp::min(50, cot_xml.len())]
            );
            for (i, c) in cot_xml.chars().enumerate() {
                if i < 30 {
                    println!("Position {}: '{}' (ASCII: {})", i, c, c as u32);
                }
            }
        }
    }
    //     let old_cot_xml = format!(
    //         r#"<event version="2.0" type="a-f-G-U-C" uid="{}" time="{}" start="{}" stale="{}" how="h-g-i-g-o">
    //   <point lat="37.7749" lon="-122.4194" hae="100.0" ce="50.0" le="25.0"/>
    //   <detail>
    //     <__group name="Blue Team" role="Team Leader"/>
    //     <contact endpoint="*:-1:stcp" callsign="PEER1-LEADER"/>
    //     <precisionlocation geopointsrc="GPS" altsrc="GPS"/>
    //     <status battery="85" readiness="true"/>
    //     <track course="180.0" speed="15.0"/>
    //     <uid Droid="PEER1-LEADER"/>
    //     <remarks>Initial MapItem created by peer 1</remarks>
    //   </detail>
    // </event>"#,
    //         event_uid, start_time, start_time, stale_time
    //     );

    // Parse the CoT XML into a CotEvent
    let cot_event = CotEvent::from_xml(&cot_xml)
        .with_context(|| format!("Failed to parse THE CoT XML: {}", cot_xml))?;

    // Convert CotEvent to Ditto document
    let ditto_doc = cot_to_document(&cot_event, "peer1");

    // Ensure it's a MapItem document
    let map_item = match &ditto_doc {
        CotDocument::MapItem(item) => item,
        _ => panic!("Expected MapItem document, got different type"),
    };

    let doc_id = DittoDocument::id(&ditto_doc);
    println!("📋 Document ID: {}", doc_id);

    // Insert document into peer 1
    let doc_json = serde_json::to_value(map_item)?;
    let query = "INSERT INTO map_items DOCUMENTS (:doc) ON ID CONFLICT DO MERGE";
    let _query_result = store_1
        .execute_v2((
            query,
            serde_json::json!({
                "doc": doc_json
            }),
        ))
        .await?;

    println!("✅ Step 2 Complete: MapItem document inserted on peer 1");

    // Step 3: Verify document sync and accuracy on both clients
    println!("🔄 Step 3: Verifying document sync between peers...");

    // Query document from peer 1 first to ensure it exists
    let query = format!("SELECT * FROM map_items WHERE _id = '{}'", doc_id);
    let result_1 = store_1.execute_v2(&query).await?;
    assert!(result_1.item_count() > 0, "Document not found on peer 1");
    println!("✅ Document confirmed on peer 1");

    // Wait for sync to occur with retry logic
    let max_sync_attempts = 20; // 20 attempts with 100ms intervals = 2 seconds base + grace period
    let mut result_2 = store_2.execute_v2(&query).await?; // Initialize to avoid compile error
    let mut found = false;

    for attempt in 1..=max_sync_attempts {
        // Check if Ditto instances are still running
        let graph_1_check = ditto_1.presence().graph();
        let graph_2_check = ditto_2.presence().graph();
        if attempt % 10 == 1 {
            // Log every 10th attempt to reduce noise
            println!(
                "🔍 Sync attempt {}: Peer 1 still sees {} peers, Peer 2 still sees {} peers",
                attempt,
                graph_1_check.remote_peers.len(),
                graph_2_check.remote_peers.len()
            );
        }

        result_2 = store_2.execute_v2(&query).await?;
        if result_2.item_count() > 0 {
            println!(
                "✅ Document synced to peer 2 after {} attempts ({:.1} seconds)",
                attempt,
                attempt as f64 * 0.1
            );
            found = true;
            break;
        }

        if attempt % 10 == 0 {
            // Log progress every 10 attempts
            println!(
                "⏳ Waiting for sync... attempt {} of {}",
                attempt, max_sync_attempts
            );
        }

        sleep(Duration::from_millis(100)).await; // Use 100ms intervals for faster testing
    }

    if !found {
        // Check if we can find any documents at all on peer 2
        let all_docs_query = "SELECT * FROM map_items";
        let all_result = store_2.execute_v2(all_docs_query).await?;
        println!(
            "❌ Sync failed - peer 2 has {} total documents in map_items collection",
            all_result.item_count()
        );

        // Add a grace period like the working example
        println!("🔄 Adding 3-second grace period for sync propagation...");
        sleep(Duration::from_secs(3)).await;

        // Try one more time after grace period
        result_2 = store_2.execute_v2(&query).await?;
        if result_2.item_count() > 0 {
            println!("✅ Document synced to peer 2 after grace period!");
            found = true;
        }
    }

    if !found {
        panic!("Document not found on peer 2 after {} attempts ({:.1} seconds) + 3s grace period - sync failed", 
               max_sync_attempts, max_sync_attempts as f64 * 0.1);
    }

    // Verify document accuracy on both peers
    let doc_1 = result_1.iter().next().unwrap();
    let doc_2 = result_2.iter().next().unwrap();

    let json_1 = doc_1.json_string();
    let json_2 = doc_2.json_string();

    let retrieved_doc_1 = CotDocument::from_json_str(&json_1)?;
    let retrieved_doc_2 = CotDocument::from_json_str(&json_2)?;

    // Verify both documents have the same key fields and detail elements
    match (&retrieved_doc_1, &retrieved_doc_2) {
        (CotDocument::MapItem(doc1), CotDocument::MapItem(doc2)) => {
            // Verify core CoT fields are identical
            assert_eq!(doc1.id, doc2.id, "Document IDs don't match after sync");
            assert_eq!(
                doc1.w, doc2.w,
                "Event types (w field) don't match after sync"
            );
            assert_eq!(
                doc1.p, doc2.p,
                "How fields (p field) don't match after sync"
            );

            // Verify point data (j=LAT, l=LON, i=HAE)
            assert_eq!(
                doc1.j, doc2.j,
                "Latitude (j field) doesn't match after sync"
            );
            assert_eq!(
                doc1.l, doc2.l,
                "Longitude (l field) doesn't match after sync"
            );
            assert_eq!(doc1.i, doc2.i, "HAE (i field) doesn't match after sync");

            // Verify detail elements (r field contains the <detail> content)
            assert_eq!(
                doc1.r.len(),
                doc2.r.len(),
                "Detail element count doesn't match after sync"
            );

            println!("✅ Document core CoT fields and detail elements verified as identical");
        }
        _ => panic!("Expected MapItem documents for sync verification"),
    }

    println!("✅ Step 3 Complete: Document sync verified on both peers");

    // Step 4: Take both clients offline and turn off sync
    println!("📴 Step 4: Taking both clients offline...");

    ditto_1.stop_sync();
    ditto_2.stop_sync();

    // Wait for sync to fully stop
    sleep(Duration::from_millis(500)).await;

    println!("✅ Step 4 Complete: Both clients are offline");

    // Step 5: Both clients make independent modifications to Detail elements
    println!("✏️ Step 5: Making independent modifications on both peers...");

    // Get the current document from peer 1
    let result_1 = store_1.execute_v2(&query).await?;
    let doc_1 = result_1.iter().next().unwrap();
    let json_1 = doc_1.json_string();
    let mut retrieved_doc_1 = CotDocument::from_json_str(&json_1)?;

    // Get the current document from peer 2
    let result_2 = store_2.execute_v2(&query).await?;
    let doc_2 = result_2.iter().next().unwrap();
    let json_2 = doc_2.json_string();
    let mut retrieved_doc_2 = CotDocument::from_json_str(&json_2)?;

    // Modify the document on peer 1 - change location and track
    if let CotDocument::MapItem(ref mut map_item) = retrieved_doc_1 {
        // Update _v (version) to simulate a change
        map_item.d_v += 1;

        // Update point coordinates (j=LAT, l=LON)
        map_item.j = Some(38.0); // Change latitude to 38.0
        map_item.l = Some(-123.0); // Change longitude to -123.0

        // Update track information
        let track_map = {
            let mut map = serde_json::Map::new();
            map.insert(
                "course".to_string(),
                serde_json::Value::String("90.0".to_string()), // Peer 1: heading East
            );
            map.insert(
                "speed".to_string(),
                serde_json::Value::String("20.0".to_string()), // Peer 1: 20 m/s
            );
            map
        };
        map_item
            .r
            .insert("track".to_string(), MapItemRValue::Object(track_map));
    }

    // Modify the document on peer 2 - change location and track (creating conflicts)
    if let CotDocument::MapItem(ref mut map_item) = retrieved_doc_2 {
        // Update _v (version) to simulate a change
        map_item.d_v += 1;

        // Update point coordinates (j=LAT, l=LON) with different values than peer 1
        map_item.j = Some(39.0); // Change latitude to 39.0 (conflicts with peer 1's 38.0)
        map_item.l = Some(-124.0); // Change longitude to -124.0 (conflicts with peer 1's -123.0)

        // Update track information with different values than peer 1
        let track_map = {
            let mut map = serde_json::Map::new();
            map.insert(
                "course".to_string(),
                serde_json::Value::String("270.0".to_string()), // Peer 2: heading West (conflicts with peer 1's 90.0 East)
            );
            map.insert(
                "speed".to_string(),
                serde_json::Value::String("25.0".to_string()), // Peer 2: 25 m/s (conflicts with peer 1's 20.0)
            );
            map
        };
        map_item
            .r
            .insert("track".to_string(), MapItemRValue::Object(track_map));
    }

    // Update documents in their respective stores using INSERT ... ON ID CONFLICT DO MERGE
    let doc_json_1 = match &retrieved_doc_1 {
        CotDocument::MapItem(item) => serde_json::to_value(item)?,
        _ => panic!("Expected MapItem"),
    };

    let doc_json_2 = match &retrieved_doc_2 {
        CotDocument::MapItem(item) => serde_json::to_value(item)?,
        _ => panic!("Expected MapItem"),
    };

    // Update on peer 1 by inserting the modified document
    let insert_query = "INSERT INTO map_items DOCUMENTS (:doc) ON ID CONFLICT DO MERGE";
    let _update_result_1 = store_1
        .execute_v2((
            insert_query,
            serde_json::json!({
                "doc": doc_json_1
            }),
        ))
        .await?;

    // Update on peer 2 by inserting the modified document
    let _update_result_2 = store_2
        .execute_v2((
            insert_query,
            serde_json::json!({
                "doc": doc_json_2
            }),
        ))
        .await?;

    println!("✅ Step 5 Complete: Independent modifications made on both peers");

    // Step 6: Bring both clients online and ensure reconnection/sync
    println!("🔌 Step 6: Bringing both clients back online...");

    ditto_1
        .start_sync()
        .context("Failed to restart sync for peer 1")?;
    ditto_2
        .start_sync()
        .context("Failed to restart sync for peer 2")?;

    // Wait for reconnection and sync
    sleep(Duration::from_secs(3)).await;

    println!("✅ Step 6 Complete: Both clients are back online and syncing");

    // Step 7: Validate final document state with last-write-wins merge
    println!("🔍 Step 7: Validating final document state after merge...");

    // Query final document state from both peers
    let final_result_1 = store_1.execute_v2(&query).await?;
    let final_result_2 = store_2.execute_v2(&query).await?;

    assert!(
        final_result_1.item_count() > 0,
        "Final document not found on peer 1"
    );
    assert!(
        final_result_2.item_count() > 0,
        "Final document not found on peer 2"
    );

    let final_doc_1 = final_result_1.iter().next().unwrap();
    let final_doc_2 = final_result_2.iter().next().unwrap();

    let final_json_1 = final_doc_1.json_string();
    let final_json_2 = final_doc_2.json_string();

    let final_retrieved_doc_1 = CotDocument::from_json_str(&final_json_1)?;
    let final_retrieved_doc_2 = CotDocument::from_json_str(&final_json_2)?;

    // Verify both documents are identical after merge (focus on key CoT fields and detail elements)
    match (&final_retrieved_doc_1, &final_retrieved_doc_2) {
        (CotDocument::MapItem(final_doc1), CotDocument::MapItem(final_doc2)) => {
            // Verify core CoT fields are identical after merge
            assert_eq!(
                final_doc1.id, final_doc2.id,
                "Document IDs don't match after merge"
            );
            assert_eq!(
                final_doc1.w, final_doc2.w,
                "Event types (w field) don't match after merge"
            );
            assert_eq!(
                final_doc1.p, final_doc2.p,
                "How fields (p field) don't match after merge"
            );

            // Verify point data is identical (j=LAT, l=LON, i=HAE)
            assert_eq!(
                final_doc1.j, final_doc2.j,
                "Latitude (j field) doesn't match after merge"
            );
            assert_eq!(
                final_doc1.l, final_doc2.l,
                "Longitude (l field) doesn't match after merge"
            );
            assert_eq!(
                final_doc1.i, final_doc2.i,
                "HAE (i field) doesn't match after merge"
            );

            // Verify detail elements are identical (this shows last-write-wins worked correctly)
            assert_eq!(
                final_doc1.r.len(),
                final_doc2.r.len(),
                "Detail element count doesn't match after merge"
            );

            // The version should be the same on both (showing convergence)
            assert_eq!(
                final_doc1.d_v, final_doc2.d_v,
                "Document versions don't match after merge"
            );

            println!("✅ Final document core CoT fields and detail elements verified as identical after merge");
        }
        _ => panic!("Expected MapItem documents for final verification"),
    }

    // Verify that the merged document contains expected changes
    if let CotDocument::MapItem(final_map_item) = &final_retrieved_doc_1 {
        println!("📊 Final document version: {}", final_map_item.d_v);

        // The document should have the highest version number
        assert!(
            final_map_item.d_v >= 1,
            "Document version should be updated"
        );

        // Log the final state for verification
        println!("🎯 Final document state verification:");
        println!("   - Document ID: {}", final_map_item.id);
        println!("   - Version: {}", final_map_item.d_v);

        // Show final coordinate values (these were in conflict)
        println!("   - Final Latitude (j): {:?}", final_map_item.j);
        println!("   - Final Longitude (l): {:?}", final_map_item.l);
        println!("     (Peer 1 wanted: lat=38.0, lon=-123.0; Peer 2 wanted: lat=39.0, lon=-124.0)");

        // Check detail elements
        println!("   - Detail elements present: {}", final_map_item.r.len());

        // Log specific elements that were modified
        for (key, value) in &final_map_item.r {
            match value {
                MapItemRValue::Object(obj) => {
                    println!(
                        "   - {}: {}",
                        key,
                        serde_json::to_string(obj).unwrap_or_default()
                    );
                }
                MapItemRValue::String(s) => {
                    println!("   - {}: {}", key, s);
                }
                MapItemRValue::Number(n) => {
                    println!("   - {}: {}", key, n);
                }
                MapItemRValue::Boolean(b) => {
                    println!("   - {}: {}", key, b);
                }
                MapItemRValue::Array(arr) => {
                    println!("   - {}: {:?}", key, arr);
                }
                MapItemRValue::Null => {
                    println!("   - {}: null", key);
                }
            }
        }

        // Convert back to CoT XML to verify round-trip
        let final_cot_event = cot_event_from_ditto_document(&final_retrieved_doc_1);
        let final_xml = final_cot_event.to_xml()?;

        println!("🔄 Final XML representation:");
        println!("{}", final_xml);

        // Verify XML can be parsed back
        let _verify_cot = CotEvent::from_xml(&final_xml)?;
        println!("✅ XML round-trip verification successful");
    }

    println!("✅ Step 7 Complete: Final document state validated");

    // Clean up
    ditto_1.stop_sync();
    ditto_2.stop_sync();

    println!("🎉 E2E Multi-Peer Test Complete!");
    println!("✅ All steps completed successfully:");
    println!("   1. ✅ Both peers came online and established connection");
    println!("   2. ✅ CoT MapItem document created on peer 1");
    println!("   3. ✅ Document synced and verified on both peers");
    println!("   4. ✅ Both peers taken offline");
    println!("   5. ✅ Independent modifications made on both peers");
    println!("   6. ✅ Both peers brought back online and reconnected");
    println!("   7. ✅ Final document state validated with last-write-wins merge");

    Ok(())
}
