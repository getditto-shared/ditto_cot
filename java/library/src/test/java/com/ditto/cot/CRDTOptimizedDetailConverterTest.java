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
        
        // Multiple occurrence elements with stable keys
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_sensor_0"), "sensor_0");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_sensor_1"), "sensor_1");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_sensor_2"), "sensor_2");
        
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_contact_0"), "contact_0");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_contact_1"), "contact_1");
        
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_track_0"), "track_0");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_track_1"), "track_1");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_track_2"), "track_2");
        
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_remarks_0"), "remarks_0");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_remarks_1"), "remarks_1");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_remarks_2"), "remarks_2");
        
        // Total: 2 single + 11 with stable keys = 13 elements preserved
        assertEquals(13, detailMap.size(), "All 13 detail elements should be preserved");
        
        // Verify attributes are preserved
        @SuppressWarnings("unchecked")
        Map<String, Object> sensor1 = (Map<String, Object>) detailMap.get(TEST_DOC_ID + "_sensor_1");
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
        
        // Node A: Update sensor_1 zoom attribute
        @SuppressWarnings("unchecked")
        Map<String, Object> sensorA = (Map<String, Object>) nodeA.get(TEST_DOC_ID + "_sensor_1");
        sensorA.put("zoom", "20x"); // Changed from 5x
        System.out.println("Node A: Updated sensor_1 zoom to 20x");
        
        // Node B: Remove contact_0, add new track
        nodeB.remove(TEST_DOC_ID + "_contact_0");
        System.out.println("Node B: Removed contact_0");
        
        int nextTrackIndex = converter.getNextAvailableIndex(nodeB, TEST_DOC_ID, "track");
        Map<String, Object> newTrack = new java.util.HashMap<>();
        newTrack.put("_tag", "track");
        newTrack.put("_docId", TEST_DOC_ID);
        newTrack.put("_elementIndex", nextTrackIndex);
        newTrack.put("course", "60.0");
        newTrack.put("speed", "3.5");
        newTrack.put("timestamp", "2025-07-05T21:05:00Z");
        
        nodeB.put(TEST_DOC_ID + "_track_" + nextTrackIndex, newTrack);
        System.out.println("Node B: Added track_3");
        
        // Simulate CRDT merge (simplified)
        Map<String, Object> merged = new java.util.HashMap<>(nodeA);
        merged.remove(TEST_DOC_ID + "_contact_0"); // Apply removal
        merged.put(TEST_DOC_ID + "_track_3", newTrack); // Apply addition
        
        System.out.println("\nAfter convergence:");
        System.out.println("- sensor_1 has zoom=20x (from Node A)");
        System.out.println("- contact_0 removed (from Node B)");  
        System.out.println("- track_3 added (from Node B)");
        System.out.println("- All other elements unchanged");
        
        // Verify convergence
        @SuppressWarnings("unchecked")
        Map<String, Object> mergedSensor = (Map<String, Object>) merged.get(TEST_DOC_ID + "_sensor_1");
        assertEquals("20x", mergedSensor.get("zoom"));
        assertFalse(merged.containsKey(TEST_DOC_ID + "_contact_0"));
        assertTrue(merged.containsKey(TEST_DOC_ID + "_track_3"));
        
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
}