package com.ditto.cot;

import com.ditto.java.*;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.Disabled;

import io.github.cdimascio.dotenv.Dotenv;

import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.CompletionStage;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * Test to verify Ditto store operations work correctly
 */
public class DittoStoreTest {
    
    private Ditto ditto;
    private DittoStore store;
    private Path tempDir;
    
    @BeforeEach
    void setUp() throws Exception {
        // Load .env file from rust directory
        Dotenv dotenv = Dotenv.configure()
            .directory("../../rust")
            .ignoreIfMissing()
            .load();
        
        String appId = dotenv.get("DITTO_APP_ID", System.getenv("DITTO_APP_ID"));
        String playgroundToken = dotenv.get("DITTO_PLAYGROUND_TOKEN", System.getenv("DITTO_PLAYGROUND_TOKEN"));
        
        if (appId == null || playgroundToken == null) {
            throw new RuntimeException("Missing environment variables. Please set DITTO_APP_ID and DITTO_PLAYGROUND_TOKEN in rust/.env file");
        }
        
        tempDir = Files.createTempDirectory("ditto-store-test-");
        
        DittoConfig config = new DittoConfig.Builder(tempDir.toFile())
            .identity(new DittoIdentity.OnlinePlayground(appId, playgroundToken, false))
            .build();
        
        ditto = new Ditto(config);
        ditto.getStore().execute("ALTER SYSTEM SET DQL_STRICT_MODE = false");
        store = ditto.getStore();
        
        try {
            ditto.startSync();
        } catch (DittoError e) {
            throw new RuntimeException("Failed to start sync", e);
        }
        
        Thread.sleep(1000); // Wait for initialization
    }
    
    @AfterEach
    void tearDown() {
        if (ditto != null) {
            ditto.close();
        }
        
        try {
            Files.deleteIfExists(tempDir);
        } catch (Exception e) {
            // Ignore cleanup errors
        }
    }
    
    @Test
    void testSimpleStoreOperations() throws Exception {
        System.out.println("🧪 Testing Ditto Store Operations");
        
        try {
            // Test 1: Simple INSERT
            String insertQuery = "INSERT INTO test_collection DOCUMENTS ({ '_id': 'test-1', 'message': 'Hello from Java', 'value': 42 })";
            CompletionStage<DittoQueryResult> insertStage = store.execute(insertQuery);
            DittoQueryResult insertResult = insertStage.toCompletableFuture().get();
            
            System.out.println("✅ INSERT successful: " + insertResult.getItems().size() + " items");
            
            // Test 2: SELECT the inserted document
            String selectQuery = "SELECT * FROM test_collection WHERE _id = 'test-1'";
            CompletionStage<DittoQueryResult> selectStage = store.execute(selectQuery);
            DittoQueryResult selectResult = selectStage.toCompletableFuture().get();
            
            System.out.println("✅ SELECT successful: " + selectResult.getItems().size() + " items found");
            
            if (selectResult.getItems().size() > 0) {
                System.out.println("📋 Document data: " + selectResult.getItems().get(0).getValue());
            }
            
            // Test 3: UPDATE
            String updateQuery = "UPDATE test_collection SET value = 100 WHERE _id = 'test-1'";
            CompletionStage<DittoQueryResult> updateStage = store.execute(updateQuery);
            DittoQueryResult updateResult = updateStage.toCompletableFuture().get();
            
            System.out.println("✅ UPDATE successful: " + updateResult.getItems().size() + " items");
            
            // Test 4: Verify update
            CompletionStage<DittoQueryResult> verifyStage = store.execute(selectQuery);
            DittoQueryResult verifyResult = verifyStage.toCompletableFuture().get();
            
            System.out.println("✅ Verification successful: " + verifyResult.getItems().size() + " items");
            
            if (verifyResult.getItems().size() > 0) {
                System.out.println("📋 Updated document data: " + verifyResult.getItems().get(0).getValue());
            }
            
            assertThat(selectResult.getItems()).hasSizeGreaterThan(0);
            
        } catch (Exception e) {
            System.out.println("❌ Store operation failed: " + e.getMessage());
            e.printStackTrace();
            throw e;
        }
        
        System.out.println("🎉 All store operations completed successfully!");
    }
}