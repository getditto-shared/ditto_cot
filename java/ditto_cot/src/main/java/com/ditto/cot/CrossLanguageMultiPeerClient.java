package com.ditto.cot;

import com.ditto.java.*;
import com.ditto.cot.schema.MapItemDocument;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Map;
import java.util.Scanner;
import java.util.concurrent.CompletionStage;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;

/**
 * Cross-language multi-peer client for communication with Rust test.
 * 
 * This Java client runs as a subprocess and communicates with the Rust test
 * via stdin/stdout commands. It handles:
 * - INIT: Initialize Ditto client
 * - QUERY {id}: Query for document by ID
 * - MODIFY {id} {changes}: Modify document with specified changes
 * - SHUTDOWN: Clean shutdown
 */
public class CrossLanguageMultiPeerClient {

    private Ditto ditto;
    private DittoStore store;
    private CoTConverter converter;
    private Path tempDir;
    private ObjectMapper objectMapper;

    public static void main(String[] args) {
        CrossLanguageMultiPeerClient client = new CrossLanguageMultiPeerClient();
        client.run();
    }

    public void run() {
        System.out.println("📱 Java Cross-Language Multi-Peer Client Started");
        System.out.println("================================================");
        
        try {
            objectMapper = new ObjectMapper();
            converter = new CoTConverter();
            
            Scanner scanner = new Scanner(System.in);
            String line;
            
            while (scanner.hasNextLine() && (line = scanner.nextLine()) != null) {
                line = line.trim();
                
                if (line.isEmpty()) {
                    continue;
                }
                
                String[] parts = line.split(" ", 2);
                String command = parts[0].toUpperCase();
                
                try {
                    switch (command) {
                        case "INIT":
                            handleInit();
                            break;
                        case "QUERY":
                            if (parts.length < 2) {
                                System.out.println("❌ QUERY command requires document ID");
                                break;
                            }
                            handleQuery(parts[1]);
                            break;
                        case "PEERS":
                            handlePeers();
                            break;
                        case "MODIFY":
                            if (parts.length < 2) {
                                System.out.println("❌ MODIFY command requires document ID and changes");
                                break;
                            }
                            handleModify(parts[1]);
                            break;
                        case "SHUTDOWN":
                            handleShutdown();
                            return;
                        default:
                            System.out.println("❌ Unknown command: " + command);
                            break;
                    }
                } catch (Exception e) {
                    System.out.println("❌ Error executing command '" + command + "': " + e.getMessage());
                    e.printStackTrace();
                }
            }
        } catch (Exception e) {
            System.out.println("❌ Fatal error in Java client: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private void handleInit() throws Exception {
        System.out.println("🔌 Initializing Java Ditto client...");
        
        // Load environment variables
        String appId = System.getenv("DITTO_APP_ID");
        String playgroundToken = System.getenv("DITTO_PLAYGROUND_TOKEN");
        
        // Environment variables should be passed directly from the Rust test
        
        if (appId == null || playgroundToken == null) {
            throw new RuntimeException("Missing DITTO_APP_ID or DITTO_PLAYGROUND_TOKEN environment variables");
        }
        
        // Create temp directory
        tempDir = Files.createTempDirectory("cross_lang_java_client");
        
        // Initialize Ditto
        DittoConfig config = new DittoConfig.Builder(tempDir.toFile())
            .identity(new DittoIdentity.OnlinePlayground(appId, playgroundToken, false)) // Disable cloud sync for peer-to-peer
            .build();
        
        ditto = new Ditto(config);
        ditto.getStore().execute("ALTER SYSTEM SET DQL_STRICT_MODE = false");
        
        try {
            ditto.startSync();
        } catch (DittoError e) {
            throw new RuntimeException("Failed to start Ditto sync", e);
        }
        
        store = ditto.getStore();
        
        System.out.println("🔗 Setting up DQL sync subscriptions and observers for map_items collection...");
        
        // Set up sync subscription to enable peer-to-peer replication (like working Rust test)
        try {
            var syncSubscription = ditto.getSync().registerSubscription("SELECT * FROM map_items");
            System.out.println("✅ Java sync subscription registered");
        } catch (DittoError e) {
            System.out.println("⚠️ Could not register sync subscription, trying alternative approach: " + e.getMessage());
            // Try alternative approach if the API is different
            try {
                ditto.getSync().registerSubscription("map_items");
                System.out.println("✅ Java sync subscription registered (alternative)");
            } catch (DittoError e2) {
                System.out.println("⚠️ Sync subscription failed, continuing with observer only: " + e2.getMessage());
            }
        }
        
        // Subscribe to map_items collection to observe changes
        try {
            store.registerObserver("SELECT * FROM map_items", (result, event) -> {
                System.out.println("🔔 Java observer: received " + result.getItems().size() + " documents");
            });
        } catch (DittoError e) {
            throw new RuntimeException("Failed to register observer", e);
        }
        
        System.out.println("✅ Java Ditto client initialized and syncing");
        System.out.println("   App ID: " + appId.substring(0, Math.min(8, appId.length())) + "...");
    }

    private void handleQuery(String docId) throws Exception {
        System.out.println("🔍 Querying for document: " + docId);
        
        if (store == null) {
            System.out.println("❌ Ditto not initialized. Call INIT first.");
            return;
        }
        
        // Wait a moment for potential sync
        Thread.sleep(1000);
        
        String selectQuery = String.format("SELECT * FROM map_items WHERE _id = '%s'", docId);
        CompletionStage<DittoQueryResult> selectStage = store.execute(selectQuery);
        DittoQueryResult result = selectStage.toCompletableFuture().get();
        
        if (result.getItems().size() == 0) {
            System.out.println("📋 Document not found: " + docId);
            
            // Try querying all documents for debugging
            String debugQuery = "SELECT * FROM map_items";
            CompletionStage<DittoQueryResult> debugStage = store.execute(debugQuery);
            DittoQueryResult debugResult = debugStage.toCompletableFuture().get();
            
            System.out.println("📋 Total documents in map_items: " + debugResult.getItems().size());
            for (int i = 0; i < Math.min(3, debugResult.getItems().size()); i++) {
                Object item = debugResult.getItems().get(i).getValue();
                System.out.println("📋 Document " + i + ": " + item.toString().substring(0, Math.min(100, item.toString().length())) + "...");
            }
        } else {
            System.out.println("✅ Document found: " + docId);
            Object docData = result.getItems().get(0).getValue();
            System.out.println("📋 Document data preview: " + docData.toString().substring(0, Math.min(200, docData.toString().length())) + "...");
        }
    }

    private void handleModify(String commandArgs) throws Exception {
        System.out.println("✏️ Modifying document: " + commandArgs);
        
        if (store == null) {
            System.out.println("❌ Ditto not initialized. Call INIT first.");
            return;
        }
        
        // Parse command: "MODIFY <doc_id> lat=38.0 lon=-122.0"
        String[] parts = commandArgs.split(" ");
        if (parts.length < 2) {
            System.out.println("❌ MODIFY command format: <doc_id> lat=<value> lon=<value>");
            return;
        }
        
        String docId = parts[0];
        
        // Parse lat and lon values
        Double newLat = null;
        Double newLon = null;
        
        for (int i = 1; i < parts.length; i++) {
            String part = parts[i];
            if (part.startsWith("lat=")) {
                newLat = Double.parseDouble(part.substring(4));
            } else if (part.startsWith("lon=")) {
                newLon = Double.parseDouble(part.substring(4));
            }
        }
        
        if (newLat == null || newLon == null) {
            System.out.println("❌ Both lat and lon values are required");
            return;
        }
        
        System.out.println("📋 Modifying document " + docId + " to lat=" + newLat + ", lon=" + newLon);
        
        // First, query the existing document
        String selectQuery = String.format("SELECT * FROM map_items WHERE _id = '%s'", docId);
        CompletionStage<DittoQueryResult> selectStage = store.execute(selectQuery);
        DittoQueryResult result = selectStage.toCompletableFuture().get();
        
        if (result.getItems().size() == 0) {
            System.out.println("❌ Document not found for modification: " + docId);
            return;
        }
        
        // Update the document using DQL UPDATE
        String updateQuery = String.format(
            "UPDATE map_items SET j = %f, l = %f WHERE _id = '%s'", 
            newLat, newLon, docId
        );
        
        CompletionStage<DittoQueryResult> updateStage = store.execute(updateQuery);
        DittoQueryResult updateResult = updateStage.toCompletableFuture().get();
        
        System.out.println("✅ Document modified successfully");
        System.out.println("📋 Update result: " + updateResult.getItems().size() + " documents affected");
        
        // Wait a moment for sync
        Thread.sleep(500);
    }

    private void handlePeers() throws Exception {
        System.out.println("🔍 Checking Java client peer connectivity...");
        
        if (ditto == null) {
            System.out.println("❌ Ditto not initialized. Call INIT first.");
            return;
        }
        
        try {
            // Get peer connectivity info
            var presence = ditto.getPresence();
            var graph = presence.getGraph();
            
            System.out.println("🔍 Java client sees " + graph.getRemotePeers().size() + " peers");
            for (var peer : graph.getRemotePeers()) {
                System.out.println("🔍 Java connected to peer: " + peer.toString());
            }
        } catch (Exception e) {
            System.out.println("❌ Error checking peers: " + e.getMessage());
            e.printStackTrace();
        }
    }

    private void handleShutdown() throws Exception {
        System.out.println("🧹 Shutting down Java client...");
        
        if (ditto != null) {
            ditto.close();
        }
        
        if (tempDir != null) {
            try {
                Files.walk(tempDir)
                    .sorted(java.util.Comparator.reverseOrder())
                    .map(Path::toFile)
                    .forEach(File::delete);
            } catch (Exception e) {
                System.out.println("⚠️ Error cleaning up temp directory: " + e.getMessage());
            }
        }
        
        System.out.println("✅ Java client shutdown complete");
    }
}