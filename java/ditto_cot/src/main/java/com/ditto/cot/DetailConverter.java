package com.ditto.cot;

import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.Node;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import java.util.HashMap;
import java.util.Map;

/**
 * Specialized converter for handling CoT Detail elements
 * Provides bidirectional conversion between XML DOM and Map structures
 */
public class DetailConverter {
    
    private final DocumentBuilderFactory documentBuilderFactory;
    
    public DetailConverter() {
        this.documentBuilderFactory = DocumentBuilderFactory.newInstance();
        this.documentBuilderFactory.setNamespaceAware(true);
    }
    
    /**
     * Convert a Map back to XML Element nodes for the detail section
     * This preserves the structure needed for proper CoT XML output
     */
    public Element convertMapToDetailElement(Map<String, Object> detailMap, Document document) {
        if (detailMap == null || detailMap.isEmpty()) {
            return null;
        }
        
        Element detailElement = document.createElement("detail");
        
        for (Map.Entry<String, Object> entry : detailMap.entrySet()) {
            String key = entry.getKey();
            Object value = entry.getValue();
            
            Element childElement = createElementFromMapEntry(document, key, value);
            if (childElement != null) {
                detailElement.appendChild(childElement);
            }
        }
        
        return detailElement;
    }
    
    /**
     * Create an XML element from a Map entry, handling different value types
     */
    private Element createElementFromMapEntry(Document document, String elementName, Object value) {
        Element element = document.createElement(elementName);
        
        if (value instanceof Map) {
            // Handle nested objects
            @SuppressWarnings("unchecked")
            Map<String, Object> nestedMap = (Map<String, Object>) value;
            
            // Check if this map represents an element with attributes
            if (hasAttributePattern(nestedMap)) {
                setElementFromAttributeMap(element, nestedMap);
            } else {
                // Create nested elements
                for (Map.Entry<String, Object> nestedEntry : nestedMap.entrySet()) {
                    String nestedKey = nestedEntry.getKey();
                    Object nestedValue = nestedEntry.getValue();
                    
                    if (nestedKey.equals("_text")) {
                        // Special case: text content
                        element.setTextContent(nestedValue.toString());
                    } else {
                        Element nestedElement = createElementFromMapEntry(document, nestedKey, nestedValue);
                        if (nestedElement != null) {
                            element.appendChild(nestedElement);
                        }
                    }
                }
            }
        } else {
            // Simple value - set as text content
            element.setTextContent(value.toString());
        }
        
        return element;
    }
    
    /**
     * Check if a Map represents an element with attributes (vs nested elements)
     * Heuristic: treat as attributes if it has typical attribute keys and no nested Maps
     */
    private boolean hasAttributePattern(Map<String, Object> map) {
        if (map.isEmpty()) {
            return false;
        }
        
        // Check if any values are Maps (indicating nested elements)
        boolean hasNestedMaps = map.entrySet().stream()
            .anyMatch(entry -> entry.getValue() instanceof Map);
        
        if (hasNestedMaps) {
            return false; // Has nested elements, not attributes
        }
        
        // Check for typical attribute patterns
        boolean hasText = map.containsKey("_text");
        boolean hasTypicalAttributes = map.entrySet().stream()
            .anyMatch(entry -> !entry.getKey().equals("_text") && 
                              entry.getValue() instanceof String &&
                              isTypicalAttributeName(entry.getKey()));
        
        // If it has _text or typical attributes, and all values are strings, treat as attributes
        return (hasText || hasTypicalAttributes) && map.entrySet().stream()
            .allMatch(entry -> entry.getValue() instanceof String);
    }
    
    /**
     * Check if a key name looks like a typical XML attribute
     */
    private boolean isTypicalAttributeName(String key) {
        // Common attribute patterns in CoT XML
        return key.equals("callsign") || key.equals("endpoint") || key.equals("battery") ||
               key.equals("version") || key.equals("device") || key.equals("platform") ||
               key.equals("os") || key.equals("ip") || key.equals("deviceName") ||
               key.equals("a") || key.equals("Droid") || key.length() <= 3; // Short keys often attributes
    }
    
    /**
     * Set element attributes and text content from a Map
     */
    private void setElementFromAttributeMap(Element element, Map<String, Object> attributeMap) {
        for (Map.Entry<String, Object> entry : attributeMap.entrySet()) {
            String key = entry.getKey();
            Object value = entry.getValue();
            
            if (key.equals("_text")) {
                element.setTextContent(value.toString());
            } else {
                element.setAttribute(key, value.toString());
            }
        }
    }
    
    /**
     * Enhanced XML to Map conversion that preserves more structural information
     */
    public Map<String, Object> convertDetailElementToMap(Element detailElement) {
        Map<String, Object> result = new HashMap<>();
        
        if (detailElement == null) {
            return result;
        }
        
        Node child = detailElement.getFirstChild();
        while (child != null) {
            if (child instanceof Element) {
                Element childElement = (Element) child;
                String tagName = childElement.getTagName();
                Object value = extractElementValue(childElement);
                result.put(tagName, value);
            }
            child = child.getNextSibling();
        }
        
        return result;
    }
    
    /**
     * Enhanced element value extraction that better preserves XML structure
     */
    public Object extractElementValue(Element element) {
        if (element == null) {
            return "";
        }
        
        boolean hasAttributes = element.hasAttributes();
        boolean hasChildElements = hasChildElements(element);
        String textContent = element.getTextContent();
        
        if (hasChildElements) {
            // Has child elements - create nested map
            Map<String, Object> nestedMap = new HashMap<>();
            
            // Add attributes if present
            if (hasAttributes) {
                for (int i = 0; i < element.getAttributes().getLength(); i++) {
                    Node attr = element.getAttributes().item(i);
                    if (attr != null) {
                        nestedMap.put(attr.getNodeName(), attr.getNodeValue());
                    }
                }
            }
            
            // Add child elements
            Node child = element.getFirstChild();
            while (child != null) {
                if (child instanceof Element) {
                    Element childElement = (Element) child;
                    String childTagName = childElement.getTagName();
                    Object childValue = extractElementValue(childElement);
                    nestedMap.put(childTagName, childValue);
                }
                child = child.getNextSibling();
            }
            
            return nestedMap;
        } else if (hasAttributes) {
            // Has attributes but no child elements
            Map<String, Object> attributeMap = new HashMap<>();
            
            // Add attributes
            for (int i = 0; i < element.getAttributes().getLength(); i++) {
                Node attr = element.getAttributes().item(i);
                if (attr != null) {
                    attributeMap.put(attr.getNodeName(), attr.getNodeValue());
                }
            }
            
            // Add text content if present
            if (textContent != null && !textContent.trim().isEmpty()) {
                attributeMap.put("_text", textContent.trim());
            }
            
            return attributeMap;
        } else {
            // Simple element with just text content
            return textContent != null ? textContent.trim() : "";
        }
    }
    
    /**
     * Check if element has child elements (not just text nodes)
     */
    private boolean hasChildElements(Element element) {
        if (element == null) {
            return false;
        }
        Node child = element.getFirstChild();
        while (child != null) {
            if (child instanceof Element) {
                return true;
            }
            child = child.getNextSibling();
        }
        return false;
    }
    
    /**
     * Create a complete DOM Document with a detail element from a Map
     * Useful for standalone testing and debugging
     */
    public Document createDetailDocument(Map<String, Object> detailMap) throws Exception {
        DocumentBuilder builder = documentBuilderFactory.newDocumentBuilder();
        Document document = builder.newDocument();
        
        Element detailElement = convertMapToDetailElement(detailMap, document);
        if (detailElement != null) {
            document.appendChild(detailElement);
        }
        
        return document;
    }
}