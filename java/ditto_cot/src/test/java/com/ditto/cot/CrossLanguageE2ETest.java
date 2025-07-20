package com.ditto.cot;

import com.ditto.java.*;
import com.ditto.cot.schema.*;
import org.junit.jupiter.api.*;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Instant;
import java.util.*;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.CompletionStage;

import io.github.cdimascio.dotenv.Dotenv;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.type.TypeReference;
import jakarta.xml.bind.JAXBException;

import static org.junit.jupiter.api.Assertions.*;
import static org.assertj.core.api.Assertions.assertThat;

/**
 * Cross-language E2E test that verifies full compatibility between Java and Rust implementations.
 * 
 * This test validates:
 * 1. Java → Ditto → Rust (Android creates document, Rust client reads)
 * 2. Rust → Ditto → Java (Rust creates document, Java client reads) 
 * 3. Full detail element preservation in both directions
 */
public class CrossLanguageE2ETest {

    // Load the definitive ATAK test XML dynamically
    private String getDefinitiveCoTXml(String uid) throws IOException {
        Path xmlPath = Paths.get("../../schema/example_xml/atak_test.xml");
        String baseXml = Files.readString(xmlPath);
        
        // Replace with test-specific uid if provided
        if (uid != null && !uid.equals("ANDROID-121304b069b9e23b")) {
            baseXml = baseXml.replace("ANDROID-121304b069b9e23b", uid);
        }
        
        return baseXml;
    }

    private Ditto ditto;
    private DittoStore store;
    private CoTConverter converter;
    private Path tempDir;

    @BeforeEach
    void setUp() throws Exception {
        // Load test environment variables
        loadTestEnv();
        
        // Create temp directory for this test
        tempDir = Files.createTempDirectory("cross_lang_e2e_test");
        
        // Initialize CoT converter
        converter = new CoTConverter();
        
        // Initialize Ditto with unique app ID for isolation
        String appId = System.getProperty("DITTO_APP_ID");
        String token = System.getProperty("DITTO_PLAYGROUND_TOKEN");
        
        assertNotNull(appId, "DITTO_APP_ID environment variable must be set");
        assertNotNull(token, "DITTO_PLAYGROUND_TOKEN environment variable must be set");
        
        DittoConfig config = new DittoConfig.Builder(tempDir.toFile())
            .identity(new DittoIdentity.OnlinePlayground(appId, token, false))
            .build();
        
        ditto = new Ditto(config);
        ditto.getStore().execute("ALTER SYSTEM SET DQL_STRICT_MODE = false");
        try {
            ditto.startSync();
        } catch (DittoError e) {
            throw new RuntimeException("Failed to start Ditto sync", e);
        }
        store = ditto.getStore();
        
        // Subscribe to all collections to enable sync using DQL  
        try {
            // Subscribe to both track and map_items collections since tests may use either
            store.registerObserver("SELECT * FROM track", (result, event) -> {
                // Observer callback - just needed to establish subscription
            });
            store.registerObserver("SELECT * FROM map_items", (result, event) -> {
                // Observer callback - just needed to establish subscription
            });
        } catch (DittoError e) {
            throw new RuntimeException("Failed to register observer", e);
        }
        
        System.out.println("🚀 Cross-Language E2E Test Setup Complete");
        System.out.println("===========================================");
    }
    
    private void loadTestEnv() {
        // Try to load .env file from project root
        try {
            String userDir = System.getProperty("user.dir");
            Path envPath = Paths.get(userDir).getParent().getParent().resolve(".env");
            if (Files.exists(envPath)) {
                Dotenv dotenv = Dotenv.configure()
                    .directory(envPath.getParent().toString())
                    .load();
                dotenv.entries().forEach(entry -> 
                    System.setProperty(entry.getKey(), entry.getValue()));
            }
        } catch (Exception e) {
            System.out.println("Could not load .env file: " + e.getMessage());
        }
        
        // Fallback to system environment variables
        if (System.getenv("DITTO_APP_ID") != null) {
            System.setProperty("DITTO_APP_ID", System.getenv("DITTO_APP_ID"));
        }
        if (System.getenv("DITTO_PLAYGROUND_TOKEN") != null) {
            System.setProperty("DITTO_PLAYGROUND_TOKEN", System.getenv("DITTO_PLAYGROUND_TOKEN"));
        }
    }

    @AfterEach 
    void tearDown() throws Exception {
        if (ditto != null) {
            ditto.close();
        }
        
        // Clean up temp directory
        if (tempDir != null) {
            Files.walk(tempDir)
                .sorted(Comparator.reverseOrder())
                .map(Path::toFile)
                .forEach(File::delete);
        }
    }

    @Test
    @Disabled("Cross-language tests are disabled by default in CI. Enable with system property or IDE for manual testing.")
    @DisplayName("Java→Ditto→Rust: Verify Java document can be correctly read by Rust")
    void testJavaToDittoToRust() throws Exception {
        System.out.println("📤 Testing Java → Ditto → Rust flow");
        System.out.println("====================================");
        
        // Step 1: Parse CoT XML with Java
        System.out.println("Step 1: Parsing CoT XML with Java...");
        String cotXml = getDefinitiveCoTXml("CROSS-LANG-TEST");
        Object document = converter.convertToDocument(cotXml);
        assertThat(document).isInstanceOf(MapItemDocument.class);
        MapItemDocument mapItem = (MapItemDocument) document;
        System.out.println("✅ Java parsing successful");
        
        // Step 2: Convert to flattened Ditto document using Java
        System.out.println("Step 2: Converting to flattened Ditto document...");
        ObjectMapper objectMapper = new ObjectMapper();
        Map<String, Object> dittoDoc = converter.convertDocumentToMap(mapItem);
        
        // Determine the correct collection for this document type
        String collectionName = converter.getCollectionName(mapItem);
        System.out.println("📁 Using collection: " + collectionName);
        
        // Count and log r_* fields
        int rFieldCount = (int) dittoDoc.entrySet().stream()
            .filter(entry -> entry.getKey().startsWith("r_"))
            .count();
        System.out.println("📊 Java created " + rFieldCount + " flattened r_* fields");
        
        // Step 3: Store in Ditto using DQL
        System.out.println("Step 3: Storing document in Ditto...");
        String docId = (String) dittoDoc.get("_id");
        
        // Use the flattened insertion approach with dynamic collection
        String basicInsert = String.format(
            "INSERT INTO %s DOCUMENTS ({ '_id': '%s', 'w': '%s', 'j': %f, 'l': %f, 'c': '%s', '@type': 'mapitem' })",
            collectionName, docId, dittoDoc.get("w"), dittoDoc.get("j"), dittoDoc.get("l"), dittoDoc.get("c")
        );
        
        CompletionStage<DittoQueryResult> insertStage = store.execute(basicInsert);
        DittoQueryResult insertResult = insertStage.toCompletableFuture().get();
        
        // Now add the flattened r_* fields using UPDATE statements
        for (Map.Entry<String, Object> entry : dittoDoc.entrySet()) {
            String key = entry.getKey();
            if (key.startsWith("r_") && entry.getValue() != null) {
                String updateQuery;
                Object value = entry.getValue();
                if (value instanceof String) {
                    updateQuery = String.format("UPDATE %s SET `%s` = '%s' WHERE _id = '%s'", 
                        collectionName, key, value.toString().replace("'", "''"), docId);
                } else if (value instanceof Number) {
                    updateQuery = String.format("UPDATE %s SET `%s` = %s WHERE _id = '%s'", 
                        collectionName, key, value, docId);
                } else {
                    updateQuery = String.format("UPDATE %s SET `%s` = '%s' WHERE _id = '%s'", 
                        collectionName, key, value.toString().replace("'", "''"), docId);
                }
                
                CompletionStage<DittoQueryResult> updateStage = store.execute(updateQuery);
                updateStage.toCompletableFuture().get(); // Wait for each update
            }
        }
        
        System.out.println("✅ Document stored with ID: " + docId);
        
        // Step 4: Retrieve document from Ditto
        System.out.println("Step 4: Retrieving document from Ditto...");
        String selectQuery = "SELECT * FROM " + collectionName + " WHERE _id = '" + docId + "'";
        CompletionStage<DittoQueryResult> selectStage = store.execute(selectQuery);
        DittoQueryResult queryResult = selectStage.toCompletableFuture().get();
        assertEquals(1, queryResult.getItems().size(), "Should find exactly one document");
        
        Object retrievedDoc = queryResult.getItems().get(0);
        System.out.println("✅ Document retrieved from Ditto");
        
        // Step 5: Simulate Rust reconstruction by calling Rust binary
        System.out.println("Step 5: Simulating Rust reconstruction...");
        
        // Extract proper JSON from Ditto query result
        Object docData = queryResult.getItems().get(0).getValue();
        
        // Convert Dictionary to plain Map (same approach as E2EMultiPeerTest)
        Map<String, Object> documentMap = convertDittoObjectToMap(docData);
        String retrievedJson = objectMapper.writeValueAsString(documentMap);
        
        // Debug the JSON format
        System.out.println("📋 Retrieved JSON format: " + retrievedJson.getClass().getSimpleName());
        System.out.println("📋 Retrieved JSON preview (first 200 chars): " + 
            (retrievedJson.length() > 200 ? retrievedJson.substring(0, 200) + "..." : retrievedJson));
        
        String rustXml = callRustXmlReconstruction(retrievedJson);
        
        if (rustXml.startsWith("ERROR:")) {
            fail("Rust reconstruction failed: " + rustXml);
        }
        
        System.out.println("✅ Rust reconstruction successful");
        
        // Step 6: Compare original and reconstructed XML
        System.out.println("Step 6: Comparing XML results...");
        boolean xmlMatches = compareCoTXml(cotXml, rustXml);
        
        if (!xmlMatches) {
            System.out.println("❌ XML comparison failed!");
            System.out.println("--- Original XML ---");
            System.out.println(cotXml);
            System.out.println("--- Rust Reconstructed XML ---"); 
            System.out.println(rustXml);
            fail("XML round-trip through Rust failed - detail elements lost");
        }
        
        System.out.println("✅ Java → Ditto → Rust flow completed successfully!");
    }


    /**
     * Call Rust binary to reconstruct XML from flattened JSON
     */
    private String callRustXmlReconstruction(String flattenedJson) {
        try {
            // Write JSON to temp file
            Path jsonFile = Files.createTempFile("cross_lang_test", ".json");
            Files.write(jsonFile, flattenedJson.getBytes());
            
            // Call Rust binary
            ProcessBuilder pb = new ProcessBuilder(
                "cargo", "run", "--example", "cross_lang_xml_reconstruct", 
                "--", jsonFile.toString()
            );
            pb.directory(new File("../../rust")); // Navigate to Rust project from java/ditto_cot
            pb.redirectErrorStream(true);
            
            // Pass environment variables to Rust process
            Map<String, String> env = pb.environment();
            env.put("DITTO_APP_ID", System.getProperty("DITTO_APP_ID"));
            env.put("DITTO_PLAYGROUND_TOKEN", System.getProperty("DITTO_PLAYGROUND_TOKEN"));
            
            Process process = pb.start();
            StringBuilder output = new StringBuilder();
            
            try (BufferedReader reader = new BufferedReader(
                    new InputStreamReader(process.getInputStream()))) {
                String line;
                while ((line = reader.readLine()) != null) {
                    output.append(line).append("\n");
                }
            }
            
            int exitCode = process.waitFor();
            if (exitCode != 0) {
                return "ERROR: Rust process exited with code " + exitCode + ": " + output.toString();
            }
            
            // Clean up
            Files.deleteIfExists(jsonFile);
            
            return output.toString().trim();
            
        } catch (Exception e) {
            return "ERROR: " + e.getMessage();
        }
    }


    /**
     * Compare two CoT XML documents semantically, ignoring formatting differences
     */
    private boolean compareCoTXml(String xml1, String xml2) {
        try {
            // Use the existing XML comparison utilities if available
            // For now, do a simple detail element count comparison
            
            int detailCount1 = countDetailElements(xml1);
            int detailCount2 = countDetailElements(xml2);
            
            System.out.println("📊 Original XML detail elements: " + detailCount1);
            System.out.println("📊 Reconstructed XML detail elements: " + detailCount2);
            
            if (detailCount1 != detailCount2) {
                System.out.println("❌ Detail element count mismatch!");
                return false;
            }
            
            // Check for key detail elements
            String[] requiredElements = {
                "takv", "contact", "uid", "precisionlocation", 
                "__group", "status", "track", "ditto"
            };
            
            for (String element : requiredElements) {
                boolean inOriginal = xml1.contains("<" + element);
                boolean inReconstructed = xml2.contains("<" + element);
                
                if (inOriginal != inReconstructed) {
                    System.out.println("❌ Element '" + element + "' missing in reconstruction");
                    return false;
                }
            }
            
            return true;
            
        } catch (Exception e) {
            System.err.println("Error comparing XML: " + e.getMessage());
            return false;
        }
    }

    /**
     * Count the number of detail child elements in a CoT XML
     */
    private int countDetailElements(String xml) {
        // Simple approach: count opening tags inside <detail>
        int detailStart = xml.indexOf("<detail>");
        int detailEnd = xml.indexOf("</detail>");
        
        if (detailStart == -1 || detailEnd == -1) {
            return 0;
        }
        
        String detailContent = xml.substring(detailStart + 8, detailEnd);
        
        // Count opening tags (excluding self-closing)
        int count = 0;
        int pos = 0;
        while ((pos = detailContent.indexOf('<', pos)) != -1) {
            if (pos + 1 < detailContent.length() && detailContent.charAt(pos + 1) != '/') {
                // This is an opening tag
                count++;
            }
            pos++;
        }
        
        return count;
    }

    /**
     * Convert Ditto Dictionary object to plain Map (same approach as E2EMultiPeerTest)
     */
    private Map<String, Object> convertDittoObjectToMap(Object documentData) {
        if (documentData instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> docMap = (Map<String, Object>) documentData;
            return docMap;
        } else {
            // Parse Dictionary string representation (same logic as E2EMultiPeerTest)
            String docString = documentData.toString();
            System.out.println("📋 Converting Dictionary to Map from: " + documentData.getClass().getSimpleName());
            
            Map<String, Object> document = new java.util.HashMap<>();
            
            if (docString.contains("Dictionary(value={") && docString.endsWith("})")) {
                // Extract content inside Dictionary(value={...})
                String content = docString.substring(
                    docString.indexOf("Dictionary(value={") + 18,
                    docString.lastIndexOf("})")
                );
                
                // Use regex to parse Ditto's internal format
                java.util.regex.Pattern pattern = java.util.regex.Pattern.compile(
                    "(\\w+)\\(value=([^)]+)\\)=(\\w+)\\(value=([^)]+)\\)"
                );
                
                java.util.regex.Matcher matcher = pattern.matcher(content);
                while (matcher.find()) {
                    String key = matcher.group(2);
                    String value = matcher.group(4);
                    
                    // Convert numeric values
                    try {
                        if (value.contains(".")) {
                            document.put(key, Double.parseDouble(value));
                        } else {
                            document.put(key, Long.parseLong(value));
                        }
                    } catch (NumberFormatException e) {
                        document.put(key, value);
                    }
                }
            } else {
                throw new RuntimeException("Unable to parse Ditto object: " + documentData.getClass().getSimpleName());
            }
            
            return document;
        }
    }
}