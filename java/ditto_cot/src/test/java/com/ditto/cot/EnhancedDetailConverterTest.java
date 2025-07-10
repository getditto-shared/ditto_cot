package com.ditto.cot;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.Node;
import org.w3c.dom.NodeList;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import java.io.ByteArrayInputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test suite for EnhancedDetailConverter with stable key generation
 */
public class EnhancedDetailConverterTest {
    
    private EnhancedDetailConverter converter;
    private static final String TEST_DOC_ID = "test-doc-123";
    
    @BeforeEach
    void setUp() {
        converter = new EnhancedDetailConverter();
    }
    
    @Test
    void testStableKeyGenerationForDuplicates() throws Exception {
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
        
        System.out.println("=== STABLE KEY CONVERSION TEST ===");
        System.out.println("Generated keys: " + detailMap.keySet());
        
        // Verify single occurrence elements use direct keys
        assertTrue(detailMap.containsKey("status"), "Single occurrence 'status' should use direct key");
        assertTrue(detailMap.containsKey("acquisition"), "Single occurrence 'acquisition' should use direct key");
        
        // Verify duplicate elements use stable keys
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_sensor_0"), "First sensor should have stable key");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_sensor_1"), "Second sensor should have stable key");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_sensor_2"), "Third sensor should have stable key");
        
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_contact_0"), "First contact should have stable key");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_contact_1"), "Second contact should have stable key");
        
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_track_0"), "First track should have stable key");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_track_1"), "Second track should have stable key");
        assertTrue(detailMap.containsKey(TEST_DOC_ID + "_track_2"), "Third track should have stable key");
        
        // Verify metadata is added
        Map<String, Object> firstSensor = (Map<String, Object>) detailMap.get(TEST_DOC_ID + "_sensor_0");
        assertEquals("sensor", firstSensor.get("_tag"), "Should have _tag metadata");
        assertEquals(TEST_DOC_ID, firstSensor.get("_docId"), "Should have _docId metadata");
        assertEquals(0, firstSensor.get("_elementIndex"), "Should have _elementIndex metadata");
        
        // Verify original attributes are preserved
        assertEquals("sensor-1", firstSensor.get("id"), "Should preserve original id attribute");
        assertEquals("optical", firstSensor.get("type"), "Should preserve original type attribute");
        
        System.out.println("\nFirst sensor details: " + firstSensor);
    }
    
    @Test
    void testRoundTripConversionWithStableKeys() throws Exception {
        // Load the complex_detail.xml file
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Parse XML to get detail element
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document originalDoc = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Element originalDetail = (Element) originalDoc.getElementsByTagName("detail").item(0);
        
        // Convert to Map with stable keys
        Map<String, Object> detailMap = converter.convertDetailElementToMapWithStableKeys(originalDetail, TEST_DOC_ID);
        
        // Convert back to XML
        Document newDoc = builder.newDocument();
        Element reconstructedDetail = converter.convertMapToDetailElementFromStableKeys(detailMap, newDoc);
        
        // Count elements in both
        int originalCount = countChildElements(originalDetail);
        int reconstructedCount = countChildElements(reconstructedDetail);
        
        System.out.println("=== ROUND TRIP TEST ===");
        System.out.println("Original element count: " + originalCount);
        System.out.println("Reconstructed element count: " + reconstructedCount);
        
        assertEquals(originalCount, reconstructedCount, "Should preserve all elements in round trip");
        
        // Verify all element types are present (order doesn't matter)
        NodeList reconstructedChildren = reconstructedDetail.getChildNodes();
        Map<String, Integer> reconstructedCounts = new HashMap<>();
        
        for (int i = 0; i < reconstructedChildren.getLength(); i++) {
            Node node = reconstructedChildren.item(i);
            if (node instanceof Element) {
                Element elem = (Element) node;
                String tagName = elem.getTagName();
                reconstructedCounts.put(tagName, reconstructedCounts.getOrDefault(tagName, 0) + 1);
                System.out.println("Reconstructed element: " + tagName);
            }
        }
        
        // Verify expected element counts
        assertEquals(3, (int) reconstructedCounts.getOrDefault("sensor", 0), "Should have 3 sensors");
        assertEquals(2, (int) reconstructedCounts.getOrDefault("contact", 0), "Should have 2 contacts");
        assertEquals(3, (int) reconstructedCounts.getOrDefault("track", 0), "Should have 3 tracks");
        assertEquals(3, (int) reconstructedCounts.getOrDefault("remarks", 0), "Should have 3 remarks");
        assertEquals(1, (int) reconstructedCounts.getOrDefault("status", 0), "Should have 1 status");
        assertEquals(1, (int) reconstructedCounts.getOrDefault("acquisition", 0), "Should have 1 acquisition");
    }
    
    @Test
    void testCRDTUpdateScenario() throws Exception {
        // Simulate a P2P update scenario
        
        // Initial state from complex_detail.xml
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document document = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Element detailElement = (Element) document.getElementsByTagName("detail").item(0);
        
        // Node A converts to stable keys
        Map<String, Object> nodeAMap = converter.convertDetailElementToMapWithStableKeys(detailElement, TEST_DOC_ID);
        
        // Node B also has the same initial state
        Map<String, Object> nodeBMap = converter.convertDetailElementToMapWithStableKeys(detailElement, TEST_DOC_ID);
        
        System.out.println("=== CRDT UPDATE SCENARIO ===");
        System.out.println("Initial state keys: " + nodeAMap.keySet().size() + " elements");
        
        // Node A updates second sensor's resolution
        String sensor1Key = TEST_DOC_ID + "_sensor_1";
        Map<String, Object> sensor1Data = (Map<String, Object>) nodeAMap.get(sensor1Key);
        sensor1Data.put("resolution", "4K"); // Changed from 1080p to 4K
        System.out.println("Node A: Updated sensor_1 resolution to 4K");
        
        // Node B removes first contact and adds a new sensor
        nodeBMap.remove(TEST_DOC_ID + "_contact_0");
        System.out.println("Node B: Removed contact_0");
        
        // Node B adds new sensor
        int nextSensorIndex = converter.getNextAvailableIndex(nodeBMap, TEST_DOC_ID, "sensor");
        assertEquals(3, nextSensorIndex, "Next sensor index should be 3");
        
        Map<String, Object> newSensor = new java.util.HashMap<>();
        newSensor.put("_tag", "sensor");
        newSensor.put("_order", 13); // After all original elements
        newSensor.put("_docId", TEST_DOC_ID);
        newSensor.put("_elementIndex", nextSensorIndex);
        newSensor.put("type", "lidar");
        newSensor.put("range", "100m");
        
        String newSensorKey = TEST_DOC_ID + "_sensor_" + nextSensorIndex;
        nodeBMap.put(newSensorKey, newSensor);
        System.out.println("Node B: Added new sensor_3 (lidar)");
        
        // Simulate CRDT merge (simplified - just showing the concept)
        // In real Ditto, this would be handled by CRDT merge logic
        Map<String, Object> mergedMap = new java.util.HashMap<>(nodeAMap);
        
        // Apply Node B's removal
        mergedMap.remove(TEST_DOC_ID + "_contact_0");
        
        // Apply Node B's addition
        mergedMap.put(newSensorKey, newSensor);
        
        System.out.println("\nAfter CRDT merge:");
        System.out.println("- sensor_1 has updated resolution (from Node A)");
        System.out.println("- contact_0 is removed (from Node B)");
        System.out.println("- sensor_3 is added (from Node B)");
        System.out.println("Final element count: " + mergedMap.keySet().size());
        
        // Verify the merge maintains all changes
        Map<String, Object> mergedSensor1 = (Map<String, Object>) mergedMap.get(sensor1Key);
        assertEquals("4K", mergedSensor1.get("resolution"), "Node A's update should be preserved");
        
        assertFalse(mergedMap.containsKey(TEST_DOC_ID + "_contact_0"), "Node B's removal should be applied");
        
        assertTrue(mergedMap.containsKey(newSensorKey), "Node B's addition should be present");
        Map<String, Object> mergedNewSensor = (Map<String, Object>) mergedMap.get(newSensorKey);
        assertEquals("lidar", mergedNewSensor.get("type"), "New sensor attributes should be preserved");
    }
    
    @Test
    void testGetNextAvailableIndex() {
        Map<String, Object> detailMap = new java.util.HashMap<>();
        
        // Add some existing sensors
        detailMap.put(TEST_DOC_ID + "_sensor_0", new java.util.HashMap<>());
        detailMap.put(TEST_DOC_ID + "_sensor_1", new java.util.HashMap<>());
        detailMap.put(TEST_DOC_ID + "_sensor_4", new java.util.HashMap<>()); // Gap in numbering
        
        // Test getting next index
        int nextIndex = converter.getNextAvailableIndex(detailMap, TEST_DOC_ID, "sensor");
        assertEquals(5, nextIndex, "Should return 5 (after highest existing index 4)");
        
        // Test with no existing elements
        int nextContactIndex = converter.getNextAvailableIndex(detailMap, TEST_DOC_ID, "contact");
        assertEquals(0, nextContactIndex, "Should return 0 for non-existing element type");
    }
    
    private int countChildElements(Element element) {
        int count = 0;
        NodeList childNodes = element.getChildNodes();
        for (int i = 0; i < childNodes.getLength(); i++) {
            if (childNodes.item(i) instanceof Element) {
                count++;
            }
        }
        return count;
    }
}