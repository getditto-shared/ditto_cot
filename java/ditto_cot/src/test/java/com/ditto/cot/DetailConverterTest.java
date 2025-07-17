package com.ditto.cot;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.NodeList;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import java.io.ByteArrayInputStream;
import java.util.HashMap;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * Tests for the enhanced Detail element conversion
 * Validates complex XML structure preservation through Map conversion
 */
class DetailConverterTest {

    private DetailConverter converter;
    private DocumentBuilderFactory documentBuilderFactory;

    @BeforeEach
    void setUp() {
        converter = new DetailConverter();
        documentBuilderFactory = DocumentBuilderFactory.newInstance();
        documentBuilderFactory.setNamespaceAware(true);
    }

    @Test
    void testSimpleElementConversion() throws Exception {
        // Given - Simple XML element
        String xml = "<detail><contact>ALPHA</contact></detail>";
        Element detailElement = parseXmlToElement(xml);
        
        // When - Convert to Map and back
        Map<String, Object> detailMap = converter.convertDetailElementToMap(detailElement);
        Document doc = createDocument();
        Element reconstructedDetail = converter.convertMapToDetailElement(detailMap, doc);
        
        // Then - Verify simple element is preserved
        assertThat(detailMap).containsKey("contact");
        assertThat(detailMap.get("contact")).isEqualTo("ALPHA");
        
        assertThat(reconstructedDetail).isNotNull();
        NodeList contactNodes = reconstructedDetail.getElementsByTagName("contact");
        assertThat(contactNodes.getLength()).isEqualTo(1);
        assertThat(contactNodes.item(0).getTextContent()).isEqualTo("ALPHA");
    }

    @Test
    void testElementWithAttributesConversion() throws Exception {
        // Given - Element with attributes
        String xml = "<detail><contact callsign='BRAMA' endpoint='192.168.1.1:4242:tcp'/></detail>";
        Element detailElement = parseXmlToElement(xml);
        
        // When
        Map<String, Object> detailMap = converter.convertDetailElementToMap(detailElement);
        Document doc = createDocument();
        Element reconstructedDetail = converter.convertMapToDetailElement(detailMap, doc);
        
        // Then - Verify attributes are preserved
        assertThat(detailMap).containsKey("contact");
        Object contactValue = detailMap.get("contact");
        assertThat(contactValue).isInstanceOf(Map.class);
        
        @SuppressWarnings("unchecked")
        Map<String, Object> contactMap = (Map<String, Object>) contactValue;
        assertThat(contactMap).containsEntry("callsign", "BRAMA");
        assertThat(contactMap).containsEntry("endpoint", "192.168.1.1:4242:tcp");
        
        // Verify reconstruction
        assertThat(reconstructedDetail).isNotNull();
        Element contactElement = (Element) reconstructedDetail.getElementsByTagName("contact").item(0);
        assertThat(contactElement.getAttribute("callsign")).isEqualTo("BRAMA");
        assertThat(contactElement.getAttribute("endpoint")).isEqualTo("192.168.1.1:4242:tcp");
    }

    @Test
    void testNestedElementConversion() throws Exception {
        // Given - Nested elements
        String xml = """
            <detail>
                <nested>
                    <field1>value1</field1>
                    <field2>value2</field2>
                </nested>
            </detail>
            """;
        Element detailElement = parseXmlToElement(xml);
        
        // When
        Map<String, Object> detailMap = converter.convertDetailElementToMap(detailElement);
        Document doc = createDocument();
        Element reconstructedDetail = converter.convertMapToDetailElement(detailMap, doc);
        
        // Then - Verify nested structure is preserved
        assertThat(detailMap).containsKey("nested");
        Object nestedValue = detailMap.get("nested");
        assertThat(nestedValue).isInstanceOf(Map.class);
        
        @SuppressWarnings("unchecked")
        Map<String, Object> nestedMap = (Map<String, Object>) nestedValue;
        assertThat(nestedMap).containsEntry("field1", "value1");
        assertThat(nestedMap).containsEntry("field2", "value2");
        
        // Verify reconstruction
        assertThat(reconstructedDetail).isNotNull();
        Element nestedElement = (Element) reconstructedDetail.getElementsByTagName("nested").item(0);
        assertThat(nestedElement).isNotNull();
        NodeList field1Nodes = nestedElement.getElementsByTagName("field1");
        assertThat(field1Nodes.getLength()).isGreaterThan(0);
        assertThat(field1Nodes.item(0)).isNotNull();
        assertThat(field1Nodes.item(0).getTextContent()).isEqualTo("value1");
        NodeList field2Nodes = nestedElement.getElementsByTagName("field2");
        assertThat(field2Nodes.getLength()).isGreaterThan(0);
        assertThat(field2Nodes.item(0)).isNotNull();
        assertThat(field2Nodes.item(0).getTextContent()).isEqualTo("value2");
    }

    @Test
    void testComplexAtakDetailConversion() throws Exception {
        // Given - Real ATAK-style detail structure
        String xml = """
            <detail>
                <takv os='35' version='5.4.0.11' device='GOOGLE PIXEL 7' platform='ATAK-CIV'/>
                <contact endpoint='192.168.5.241:4242:tcp' callsign='BRAMA'/>
                <uid Droid='BRAMA'/>
                <status battery='100'/>
                <ditto a='pkAocCgk' ip='192.168.5.241' version='AndJ4.10.2' deviceName='T9b9e23b'/>
            </detail>
            """;
        Element detailElement = parseXmlToElement(xml);
        
        // When
        Map<String, Object> detailMap = converter.convertDetailElementToMap(detailElement);
        Document doc = createDocument();
        Element reconstructedDetail = converter.convertMapToDetailElement(detailMap, doc);
        
        // Then - Verify complex ATAK structure
        assertThat(detailMap).containsKeys("takv", "contact", "uid", "status", "ditto");
        
        // Verify takv element with multiple attributes
        Object takvValue = detailMap.get("takv");
        assertThat(takvValue).isInstanceOf(Map.class);
        @SuppressWarnings("unchecked")
        Map<String, Object> takvMap = (Map<String, Object>) takvValue;
        assertThat(takvMap).containsEntry("os", "35");
        assertThat(takvMap).containsEntry("version", "5.4.0.11");
        assertThat(takvMap).containsEntry("device", "GOOGLE PIXEL 7");
        assertThat(takvMap).containsEntry("platform", "ATAK-CIV");
        
        // Verify contact element
        Object contactValue = detailMap.get("contact");
        assertThat(contactValue).isInstanceOf(Map.class);
        @SuppressWarnings("unchecked")
        Map<String, Object> contactMap = (Map<String, Object>) contactValue;
        assertThat(contactMap).containsEntry("callsign", "BRAMA");
        assertThat(contactMap).containsEntry("endpoint", "192.168.5.241:4242:tcp");
        
        // Verify reconstruction preserves all elements
        assertThat(reconstructedDetail).isNotNull();
        assertThat(reconstructedDetail.getElementsByTagName("takv").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("contact").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("uid").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("status").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("ditto").getLength()).isEqualTo(1);
        
        // Verify specific attributes are preserved
        Element reconstructedContact = (Element) reconstructedDetail.getElementsByTagName("contact").item(0);
        assertThat(reconstructedContact.getAttribute("callsign")).isEqualTo("BRAMA");
        assertThat(reconstructedContact.getAttribute("endpoint")).isEqualTo("192.168.5.241:4242:tcp");
    }

    @Test
    void testElementWithAttributesAndTextContent() throws Exception {
        // Given - Element with both attributes and text content
        String xml = "<detail><status battery='100'>Online</status></detail>";
        Element detailElement = parseXmlToElement(xml);
        
        // When
        Map<String, Object> detailMap = converter.convertDetailElementToMap(detailElement);
        Document doc = createDocument();
        Element reconstructedDetail = converter.convertMapToDetailElement(detailMap, doc);
        
        // Then - Verify both attributes and text are preserved
        assertThat(detailMap).containsKey("status");
        Object statusValue = detailMap.get("status");
        assertThat(statusValue).isInstanceOf(Map.class);
        
        @SuppressWarnings("unchecked")
        Map<String, Object> statusMap = (Map<String, Object>) statusValue;
        assertThat(statusMap).containsEntry("battery", "100");
        assertThat(statusMap).containsEntry("_text", "Online");
        
        // Verify reconstruction
        Element statusElement = (Element) reconstructedDetail.getElementsByTagName("status").item(0);
        assertThat(statusElement.getAttribute("battery")).isEqualTo("100");
        assertThat(statusElement.getTextContent()).isEqualTo("Online");
    }

    @Test
    void testEmptyDetailHandling() throws Exception {
        // Given - Empty detail
        Map<String, Object> emptyMap = new HashMap<>();
        Document doc = createDocument();
        
        // When
        Element detailElement = converter.convertMapToDetailElement(emptyMap, doc);
        
        // Then
        assertThat(detailElement).isNull();
    }

    @Test
    void testCompleteRoundTripWithCustomTypeExample() throws Exception {
        // Given - Complex nested structure from custom_type.xml
        String xml = """
            <detail>
                <custom_field>custom value</custom_field>
                <test_field>test value</test_field>
                <nested>
                    <field1>value1</field1>
                    <field2>value2</field2>
                </nested>
                <numeric_field>123</numeric_field>
                <boolean_field>true</boolean_field>
            </detail>
            """;
        Element originalDetail = parseXmlToElement(xml);
        
        // When - Complete round trip
        Map<String, Object> detailMap = converter.convertDetailElementToMap(originalDetail);
        Document doc = createDocument();
        Element reconstructedDetail = converter.convertMapToDetailElement(detailMap, doc);
        
        // Then - Verify all elements and structure are preserved
        assertThat(detailMap).containsKeys("custom_field", "test_field", "nested", "numeric_field", "boolean_field");
        
        // Verify simple fields
        assertThat(detailMap.get("custom_field")).isEqualTo("custom value");
        assertThat(detailMap.get("test_field")).isEqualTo("test value");
        assertThat(detailMap.get("numeric_field")).isEqualTo("123");
        assertThat(detailMap.get("boolean_field")).isEqualTo("true");
        
        // Verify nested structure
        Object nestedValue = detailMap.get("nested");
        assertThat(nestedValue).isInstanceOf(Map.class);
        @SuppressWarnings("unchecked")
        Map<String, Object> nestedMap = (Map<String, Object>) nestedValue;
        assertThat(nestedMap).containsEntry("field1", "value1");
        assertThat(nestedMap).containsEntry("field2", "value2");
        
        // Verify reconstruction has all elements
        assertThat(reconstructedDetail).isNotNull();
        assertThat(reconstructedDetail.getElementsByTagName("custom_field").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("test_field").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("nested").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("numeric_field").getLength()).isEqualTo(1);
        assertThat(reconstructedDetail.getElementsByTagName("boolean_field").getLength()).isEqualTo(1);
        
        // Verify nested structure in reconstruction
        Element nestedElement = (Element) reconstructedDetail.getElementsByTagName("nested").item(0);
        assertThat(nestedElement).isNotNull();
        NodeList field1Nodes = nestedElement.getElementsByTagName("field1");
        assertThat(field1Nodes.getLength()).isGreaterThan(0);
        assertThat(field1Nodes.item(0)).isNotNull();
        assertThat(field1Nodes.item(0).getTextContent()).isEqualTo("value1");
        
        NodeList field2Nodes = nestedElement.getElementsByTagName("field2");
        assertThat(field2Nodes.getLength()).isGreaterThan(0);
        assertThat(field2Nodes.item(0)).isNotNull();
        assertThat(field2Nodes.item(0).getTextContent()).isEqualTo("value2");
    }

    @Test
    void testNumericAndBooleanValuePreservation() throws Exception {
        // Given - Different data types as text
        String xml = """
            <detail>
                <string_field>text value</string_field>
                <numeric_field>42</numeric_field>
                <decimal_field>3.14159</decimal_field>
                <boolean_field>true</boolean_field>
                <empty_field></empty_field>
            </detail>
            """;
        Element detailElement = parseXmlToElement(xml);
        
        // When
        Map<String, Object> detailMap = converter.convertDetailElementToMap(detailElement);
        Document doc = createDocument();
        Element reconstructedDetail = converter.convertMapToDetailElement(detailMap, doc);
        
        // Then - Values are preserved as strings (XML behavior)
        assertThat(detailMap.get("string_field")).isEqualTo("text value");
        assertThat(detailMap.get("numeric_field")).isEqualTo("42");
        assertThat(detailMap.get("decimal_field")).isEqualTo("3.14159");
        assertThat(detailMap.get("boolean_field")).isEqualTo("true");
        assertThat(detailMap.get("empty_field")).isEqualTo("");
        
        // Verify values in reconstruction
        Element numericElement = (Element) reconstructedDetail.getElementsByTagName("numeric_field").item(0);
        assertThat(numericElement.getTextContent()).isEqualTo("42");
        
        Element booleanElement = (Element) reconstructedDetail.getElementsByTagName("boolean_field").item(0);
        assertThat(booleanElement.getTextContent()).isEqualTo("true");
    }

    // Helper methods

    private Element parseXmlToElement(String xml) throws Exception {
        DocumentBuilder builder = documentBuilderFactory.newDocumentBuilder();
        Document document = builder.parse(new ByteArrayInputStream(xml.getBytes()));
        return document.getDocumentElement();
    }

    private Document createDocument() throws Exception {
        DocumentBuilder builder = documentBuilderFactory.newDocumentBuilder();
        return builder.newDocument();
    }
}