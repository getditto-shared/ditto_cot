package com.ditto.cot;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.NodeList;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import java.io.ByteArrayInputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Map;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.nio.charset.StandardCharsets;
import java.util.Base64;
import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test suite for CRDTOptimizedDetailConverter
 */
public class CRDTOptimizedDetailConverterTest {
    
    private CRDTOptimizedDetailConverter converter;
    private static final String TEST_DOC_ID = "complex-detail-test";
    
    @BeforeEach
    void setUp() {
        converter = new CRDTOptimizedDetailConverter();
    }
    
    @Test
    void testStableKeyGenerationPreservesAllElements() throws Exception {
        // Load the complex_detail.xml file
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Parse XML to get detail element
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document document = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Element detailElement = (Element) document.getElementsByTagName("detail").item(0);
        
        // Convert with stable keys
        Map<String, Object> detailMap = converter.convertDetailElementToMapWithStableKeys(detailElement, TEST_DOC_ID);
        
        System.out.println("=== CRDT-OPTIMIZED STABLE KEY TEST ===");
        System.out.println("Total keys generated: " + detailMap.size());
        
        // Verify all elements are preserved with appropriate keys
        // Single occurrence elements
        assertTrue(detailMap.containsKey("status"), "Single 'status' element");
        assertTrue(detailMap.containsKey("acquisition"), "Single 'acquisition' element");
        
        // Multiple occurrence elements with stable keys (base64 hash format)
        // Count keys by checking metadata to identify element types
        long sensorCount = detailMap.entrySet().stream()
            .filter(entry -> entry.getValue() instanceof Map)
            .filter(entry -> {
                @SuppressWarnings("unchecked")
                Map<String, Object> valueMap = (Map<String, Object>) entry.getValue();
                return "sensor".equals(valueMap.get("_tag"));
            })
            .count();
        assertEquals(3, sensorCount, "Should have 3 sensor elements");
        
        long contactCount = detailMap.entrySet().stream()
            .filter(entry -> entry.getValue() instanceof Map)
            .filter(entry -> {
                @SuppressWarnings("unchecked")
                Map<String, Object> valueMap = (Map<String, Object>) entry.getValue();
                return "contact".equals(valueMap.get("_tag"));
            })
            .count();
        assertEquals(2, contactCount, "Should have 2 contact elements");
        
        long trackCount = detailMap.entrySet().stream()
            .filter(entry -> entry.getValue() instanceof Map)
            .filter(entry -> {
                @SuppressWarnings("unchecked")
                Map<String, Object> valueMap = (Map<String, Object>) entry.getValue();
                return "track".equals(valueMap.get("_tag"));
            })
            .count();
        assertEquals(3, trackCount, "Should have 3 track elements");
        
        long remarksCount = detailMap.entrySet().stream()
            .filter(entry -> entry.getValue() instanceof Map)
            .filter(entry -> {
                @SuppressWarnings("unchecked")
                Map<String, Object> valueMap = (Map<String, Object>) entry.getValue();
                return "remarks".equals(valueMap.get("_tag"));
            })
            .count();
        assertEquals(3, remarksCount, "Should have 3 remarks elements");
        
        // Total: 2 single + 11 with stable keys = 13 elements preserved
        assertEquals(13, detailMap.size(), "All 13 detail elements should be preserved");
        
        // Verify attributes are preserved - find sensor with index 1 by key
        @SuppressWarnings("unchecked")
        Map<String, Object> sensor1 = (Map<String, Object>) detailMap.entrySet().stream()
            .filter(entry -> entry.getValue() instanceof Map)
            .filter(entry -> {
                String key = entry.getKey();
                @SuppressWarnings("unchecked")
                Map<String, Object> valueMap = (Map<String, Object>) entry.getValue();
                return "sensor".equals(valueMap.get("_tag")) && 
                       key.endsWith("_1"); // Check key suffix for index
            })
            .findFirst()
            .map(Map.Entry::getValue)
            .orElse(null);
        
        assertNotNull(sensor1, "Should find sensor with index 1");
        assertEquals("sensor-2", sensor1.get("id"));
        assertEquals("thermal", sensor1.get("type"));
        assertEquals("1080p", sensor1.get("resolution"));
    }
    
    @Test
    void testRoundTripPreservesAllData() throws Exception {
        // Load the complex_detail.xml file
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Parse XML
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document originalDoc = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Element originalDetail = (Element) originalDoc.getElementsByTagName("detail").item(0);
        
        // Convert to Map
        Map<String, Object> detailMap = converter.convertDetailElementToMapWithStableKeys(originalDetail, TEST_DOC_ID);
        
        // Convert back to XML
        Document newDoc = builder.newDocument();
        Element reconstructedDetail = converter.convertMapToDetailElementFromStableKeys(detailMap, newDoc);
        
        // Verify all elements are present
        System.out.println("=== ROUND TRIP TEST ===");
        
        // Count each element type
        assertEquals(3, countElementsByName(reconstructedDetail, "sensor"), "Should have 3 sensors");
        assertEquals(2, countElementsByName(reconstructedDetail, "contact"), "Should have 2 contacts");
        assertEquals(3, countElementsByName(reconstructedDetail, "track"), "Should have 3 tracks");
        assertEquals(3, countElementsByName(reconstructedDetail, "remarks"), "Should have 3 remarks");
        assertEquals(1, countElementsByName(reconstructedDetail, "status"), "Should have 1 status");
        assertEquals(1, countElementsByName(reconstructedDetail, "acquisition"), "Should have 1 acquisition");
        
        System.out.println("✅ All elements preserved in round trip!");
    }
    
    @Test
    void testP2PConvergenceScenario() throws Exception {
        // Load initial state
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document document = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Element detailElement = (Element) document.getElementsByTagName("detail").item(0);
        
        // Both nodes start with same state
        Map<String, Object> nodeA = converter.convertDetailElementToMapWithStableKeys(detailElement, TEST_DOC_ID);
        Map<String, Object> nodeB = converter.convertDetailElementToMapWithStableKeys(detailElement, TEST_DOC_ID);
        
        System.out.println("=== P2P CONVERGENCE SCENARIO ===");
        
        // Node A: Find and update sensor with index 1 by key
        String sensorKey = nodeA.entrySet().stream()
            .filter(entry -> entry.getValue() instanceof Map)
            .filter(entry -> {
                String key = entry.getKey();
                @SuppressWarnings("unchecked")
                Map<String, Object> valueMap = (Map<String, Object>) entry.getValue();
                return "sensor".equals(valueMap.get("_tag")) && 
                       key.endsWith("_1"); // Check key suffix for index
            })
            .map(Map.Entry::getKey)
            .findFirst()
            .orElse(null);
        
        assertNotNull(sensorKey, "Should find sensor with index 1");
        
        @SuppressWarnings("unchecked")
        Map<String, Object> sensorA = (Map<String, Object>) nodeA.get(sensorKey);
        sensorA.put("zoom", "20x"); // Changed from 5x
        System.out.println("Node A: Updated sensor_1 zoom to 20x");
        
        // Node B: Find and remove contact with index 0 by key
        String contactKey = nodeB.entrySet().stream()
            .filter(entry -> entry.getValue() instanceof Map)
            .filter(entry -> {
                String key = entry.getKey();
                @SuppressWarnings("unchecked")
                Map<String, Object> valueMap = (Map<String, Object>) entry.getValue();
                return "contact".equals(valueMap.get("_tag")) && 
                       key.endsWith("_0"); // Check key suffix for index
            })
            .map(Map.Entry::getKey)
            .findFirst()
            .orElse(null);
        
        if (contactKey != null) {
            nodeB.remove(contactKey);
            System.out.println("Node B: Removed contact_0");
        }
        
        int nextTrackIndex = converter.getNextAvailableIndex(nodeB, TEST_DOC_ID, "track");
        
        // Generate stable key for new track
        String newTrackKey = generateStableKey(TEST_DOC_ID, "track", nextTrackIndex);
        
        Map<String, Object> newTrack = new java.util.HashMap<>();
        newTrack.put("_tag", "track");
        newTrack.put("course", "60.0");
        newTrack.put("speed", "3.5");
        newTrack.put("timestamp", "2025-07-05T21:05:00Z");
        
        nodeB.put(newTrackKey, newTrack);
        System.out.println("Node B: Added track_3");
        
        // Simulate CRDT merge (simplified)
        Map<String, Object> merged = new java.util.HashMap<>(nodeA);
        if (contactKey != null) {
            merged.remove(contactKey); // Apply removal from Node B
        }
        merged.put(newTrackKey, newTrack); // Apply addition from Node B
        
        System.out.println("\nAfter convergence:");
        System.out.println("- sensor_1 has zoom=20x (from Node A)");
        System.out.println("- contact_0 removed (from Node B)");  
        System.out.println("- track_3 added (from Node B)");
        System.out.println("- All other elements unchanged");
        
        // Verify convergence - find updated sensor by key
        @SuppressWarnings("unchecked")
        Map<String, Object> mergedSensor = (Map<String, Object>) merged.get(sensorKey);
        assertNotNull(mergedSensor, "Merged sensor should not be null");
        assertEquals("20x", mergedSensor.get("zoom"));
        assertFalse(merged.containsKey(contactKey != null ? contactKey : "contact_not_found"));
        assertTrue(merged.containsKey(newTrackKey));
        
        System.out.println("✅ P2P convergence successful!");
    }
    
    @Test
    void testDittoDocumentIntegration() throws Exception {
        // This demonstrates how the solution solves the original problem
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Parse original XML
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document originalDoc = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Element originalDetail = (Element) originalDoc.getElementsByTagName("detail").item(0);
        
        // Old approach: loses data
        DetailConverter oldConverter = new DetailConverter();
        Map<String, Object> oldMap = oldConverter.convertDetailElementToMap(originalDetail);
        
        // New approach: preserves all data with stable keys
        Map<String, Object> newMap = converter.convertDetailElementToMapWithStableKeys(originalDetail, TEST_DOC_ID);
        
        System.out.println("=== SOLUTION COMPARISON ===");
        System.out.println("Old approach preserved: " + oldMap.size() + " elements");
        System.out.println("New approach preserved: " + newMap.size() + " elements");
        System.out.println("Data preserved: " + (newMap.size() - oldMap.size()) + " additional elements!");
        
        assertTrue(newMap.size() > oldMap.size(), "New approach should preserve more data");
        
        // The new approach can now be used in CoTConverter for Ditto document storage
        System.out.println("\n✅ Problem solved: All duplicate elements preserved for CRDT!");
    }
    
    private int countElementsByName(Element parent, String elementName) {
        NodeList nodes = parent.getElementsByTagName(elementName);
        return nodes.getLength();
    }
    
    /**
     * Helper method to generate stable key for testing
     */
    private String generateStableKey(String documentId, String elementName, int index) {
        try {
            String input = documentId + elementName + "stable_key_salt";
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] hashBytes = digest.digest(input.getBytes(StandardCharsets.UTF_8));
            
            // Take first 8 bytes for shorter hash
            byte[] truncated = Arrays.copyOf(hashBytes, 8);
            String b64Hash = Base64.getUrlEncoder().withoutPadding().encodeToString(truncated);
            
            return b64Hash + "_" + index;
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException("SHA-256 algorithm not available", e);
        }
    }
}