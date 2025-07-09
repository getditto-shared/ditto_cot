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
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test suite for verifying how our DetailConverter handles duplicate element names
 * in CoT XML detail sections. This demonstrates the challenge described in the issue.
 */
public class ComplexDetailTest {

    private DetailConverter detailConverter;
    private CoTConverter cotConverter;

    @BeforeEach
    void setUp() {
        detailConverter = new DetailConverter();
        try {
            cotConverter = new CoTConverter();
        } catch (Exception e) {
            throw new RuntimeException("Failed to create CoTConverter", e);
        }
    }

    @Test
    void testComplexDetailXmlParsing() throws Exception {
        // Load the complex_detail.xml file
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Parse the XML and examine the detail section
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document document = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        
        // Find the detail element
        NodeList detailNodes = document.getElementsByTagName("detail");
        assertEquals(1, detailNodes.getLength(), "Should have exactly one detail element");
        
        Element detailElement = (Element) detailNodes.item(0);
        
        // Count occurrences of each element type
        NodeList childNodes = detailElement.getChildNodes();
        int sensorCount = 0;
        int contactCount = 0;
        int trackCount = 0;
        int remarksCount = 0;
        
        for (int i = 0; i < childNodes.getLength(); i++) {
            Node node = childNodes.item(i);
            if (node instanceof Element) {
                String tagName = ((Element) node).getTagName();
                switch (tagName) {
                    case "sensor":
                        sensorCount++;
                        break;
                    case "contact":
                        contactCount++;
                        break;
                    case "track":
                        trackCount++;
                        break;
                    case "remarks":
                        remarksCount++;
                        break;
                }
            }
        }
        
        // Verify we have the expected number of duplicate elements
        assertEquals(3, sensorCount, "Should have 3 sensor elements");
        assertEquals(2, contactCount, "Should have 2 contact elements");
        assertEquals(3, trackCount, "Should have 3 track elements");
        assertEquals(3, remarksCount, "Should have 3 remarks elements");
        
        System.out.println("Original XML has:");
        System.out.println("  - " + sensorCount + " sensor elements");
        System.out.println("  - " + contactCount + " contact elements");
        System.out.println("  - " + trackCount + " track elements");
        System.out.println("  - " + remarksCount + " remarks elements");
    }

    @Test
    void testCurrentDetailConverterBehaviorWithDuplicates() throws Exception {
        // Load the complex_detail.xml file
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Parse the XML and extract detail element
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        Document document = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        
        Element detailElement = (Element) document.getElementsByTagName("detail").item(0);
        
        // Convert to Map using current DetailConverter
        Map<String, Object> detailMap = detailConverter.convertDetailElementToMap(detailElement);
        
        System.out.println("Current DetailConverter results:");
        System.out.println("Detail map keys: " + detailMap.keySet());
        
        // Test what happens with duplicate keys - they should be overwritten
        assertTrue(detailMap.containsKey("sensor"), "Should contain sensor key");
        assertTrue(detailMap.containsKey("contact"), "Should contain contact key");
        assertTrue(detailMap.containsKey("track"), "Should contain track key");
        assertTrue(detailMap.containsKey("remarks"), "Should contain remarks key");
        
        // Since the current implementation overwrites duplicates, we should only have
        // the LAST occurrence of each duplicate element
        Object sensorValue = detailMap.get("sensor");
        Object contactValue = detailMap.get("contact");
        Object trackValue = detailMap.get("track");
        Object remarksValue = detailMap.get("remarks");
        
        System.out.println("sensor value: " + sensorValue);
        System.out.println("contact value: " + contactValue);
        System.out.println("track value: " + trackValue);
        System.out.println("remarks value: " + remarksValue);
        
        // Check that we only got the last sensor (id="sensor-3")
        if (sensorValue instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> sensorMap = (Map<String, Object>) sensorValue;
            assertEquals("sensor-3", sensorMap.get("id"), "Should have last sensor (sensor-3)");
        }
        
        // Check that we only got the last contact (BRAVO-02)
        if (contactValue instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> contactMap = (Map<String, Object>) contactValue;
            assertEquals("BRAVO-02", contactMap.get("callsign"), "Should have last contact (BRAVO-02)");
        }
        
        // This demonstrates the problem: we've lost the first two sensors, first contact, etc.
        System.out.println("\nPROBLEM DEMONSTRATED: Only the last occurrence of each duplicate element is preserved!");
    }

    @Test
    void testCoTConverterRoundTripWithComplexDetail() throws Exception {
        // Load the complex_detail.xml file
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Convert XML to CoT Event
        CoTEvent event = cotConverter.parseCoTXml(xmlContent);
        
        // Convert back to XML
        String regeneratedXml = cotConverter.convertCoTEventToXml(event);
        
        System.out.println("Original XML detail section:");
        extractDetailSection(xmlContent);
        
        System.out.println("\nRegenerated XML detail section:");
        extractDetailSection(regeneratedXml);
        
        // Parse both to compare detail content
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        
        Document originalDoc = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Document regeneratedDoc = builder.parse(new ByteArrayInputStream(regeneratedXml.getBytes()));
        
        Element originalDetail = (Element) originalDoc.getElementsByTagName("detail").item(0);
        Element regeneratedDetail = (Element) regeneratedDoc.getElementsByTagName("detail").item(0);
        
        // Count child elements in both
        int originalChildCount = countChildElements(originalDetail);
        int regeneratedChildCount = countChildElements(regeneratedDetail);
        
        System.out.println("Original detail child count: " + originalChildCount);
        System.out.println("Regenerated detail child count: " + regeneratedChildCount);
        
        // JAXB preserves all XML elements during roundtrip - this is expected behavior
        assertEquals(originalChildCount, regeneratedChildCount, 
                   "JAXB should preserve all XML elements during roundtrip");
        
        System.out.println("\nKey finding: JAXB preserves all duplicate elements during XML roundtrip");
        System.out.println("The data loss occurs only when converting to/from Map representation for Ditto storage");
    }

    private void extractDetailSection(String xmlContent) {
        try {
            DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
            DocumentBuilder builder = factory.newDocumentBuilder();
            Document document = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
            
            Element detailElement = (Element) document.getElementsByTagName("detail").item(0);
            NodeList childNodes = detailElement.getChildNodes();
            
            for (int i = 0; i < childNodes.getLength(); i++) {
                Node node = childNodes.item(i);
                if (node instanceof Element) {
                    Element child = (Element) node;
                    System.out.println("  <" + child.getTagName() + " " + getAttributesString(child) + ">");
                }
            }
        } catch (Exception e) {
            System.out.println("Error extracting detail section: " + e.getMessage());
        }
    }

    private String getAttributesString(Element element) {
        StringBuilder attrs = new StringBuilder();
        for (int i = 0; i < element.getAttributes().getLength(); i++) {
            Node attr = element.getAttributes().item(i);
            if (i > 0) attrs.append(" ");
            attrs.append(attr.getNodeName()).append("=\"").append(attr.getNodeValue()).append("\"");
        }
        return attrs.toString();
    }

    @Test
    void testDittoDocumentConversionDataLoss() throws Exception {
        // Load the complex_detail.xml file
        Path xmlPath = Paths.get("../../schema/example_xml/complex_detail.xml");
        String xmlContent = Files.readString(xmlPath);
        
        // Convert XML to CoT Event (preserves all elements)
        CoTEvent event = cotConverter.parseCoTXml(xmlContent);
        
        // Convert CoT Event to Ditto document (causes data loss)
        Object dittoDocument = cotConverter.convertCoTEventToDocument(event);
        
        // Convert back to CoT Event (data is lost)
        CoTEvent reconstructedEvent = cotConverter.convertDocumentToCoTEvent(dittoDocument);
        
        // Convert back to XML
        String reconstructedXml = cotConverter.convertCoTEventToXml(reconstructedEvent);
        
        System.out.println("=== DITTO DOCUMENT CONVERSION DATA LOSS TEST ===");
        System.out.println("Original XML detail section:");
        extractDetailSection(xmlContent);
        
        System.out.println("\nAfter Ditto document conversion and back to XML:");
        extractDetailSection(reconstructedXml);
        
        // Count elements in both
        DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
        DocumentBuilder builder = factory.newDocumentBuilder();
        
        Document originalDoc = builder.parse(new ByteArrayInputStream(xmlContent.getBytes()));
        Document reconstructedDoc = builder.parse(new ByteArrayInputStream(reconstructedXml.getBytes()));
        
        Element originalDetail = (Element) originalDoc.getElementsByTagName("detail").item(0);
        Element reconstructedDetail = (Element) reconstructedDoc.getElementsByTagName("detail").item(0);
        
        int originalChildCount = countChildElements(originalDetail);
        int reconstructedChildCount = countChildElements(reconstructedDetail);
        
        System.out.println("Original detail child count: " + originalChildCount);
        System.out.println("Reconstructed detail child count: " + reconstructedChildCount);
        
        // This is where the real data loss occurs - during Ditto document conversion
        assertTrue(originalChildCount > reconstructedChildCount, 
                  "Original XML should have more detail elements than reconstructed due to Map conversion data loss");
        
        // Verify specific data loss
        Map<String, Object> originalDetailMap = event.getDetailMap();
        Map<String, Object> reconstructedDetailMap = reconstructedEvent.getDetailMap();
        
        System.out.println("\nOriginal detail map keys: " + originalDetailMap.keySet());
        System.out.println("Reconstructed detail map keys: " + reconstructedDetailMap.keySet());
        
        // Should be same keys but different values (only last occurrence preserved)
        assertEquals(originalDetailMap.keySet(), reconstructedDetailMap.keySet(), 
                   "Map keys should be the same");
        
        System.out.println("\nCONCLUSION: Data loss occurs during CoT -> Ditto Document -> CoT conversion");
        System.out.println("This is due to the Map-based storage losing duplicate elements with same tag names");
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