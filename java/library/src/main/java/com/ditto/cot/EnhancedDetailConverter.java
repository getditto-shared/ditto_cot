package com.ditto.cot;

import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.Node;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import java.util.*;
import java.util.stream.Collectors;

/**
 * Enhanced DetailConverter that handles duplicate elements with stable keys
 * for CRDT optimization in P2P networks
 */
public class EnhancedDetailConverter extends DetailConverter {
    
    private static final String TAG_METADATA = "_tag";
    private static final String DOC_ID_METADATA = "_docId";
    private static final String INDEX_METADATA = "_elementIndex";
    private static final String KEY_SEPARATOR = "_";
    
    /**
     * Convert detail element to Map with stable keys for duplicate elements
     * @param detailElement The detail DOM element
     * @param documentId The document ID to use in stable key generation
     * @return Map with CRDT-optimized keys
     */
    public Map<String, Object> convertDetailElementToMapWithStableKeys(Element detailElement, String documentId) {
        Map<String, Object> result = new HashMap<>();
        
        if (detailElement == null) {
            return result;
        }
        
        // Track element occurrences for duplicate detection
        Map<String, Integer> elementCounts = new HashMap<>();
        Map<String, Integer> elementIndices = new HashMap<>();
        
        // First pass: count occurrences of each element type
        Node countChild = detailElement.getFirstChild();
        while (countChild != null) {
            if (countChild instanceof Element) {
                Element childElement = (Element) countChild;
                String tagName = childElement.getTagName();
                elementCounts.put(tagName, elementCounts.getOrDefault(tagName, 0) + 1);
            }
            countChild = countChild.getNextSibling();
        }
        
        // Second pass: convert elements with appropriate keys
        Node child = detailElement.getFirstChild();
        
        while (child != null) {
            if (child instanceof Element) {
                Element childElement = (Element) child;
                String tagName = childElement.getTagName();
                
                // Determine if this element type has duplicates
                boolean hasDuplicates = elementCounts.get(tagName) > 1;
                
                if (hasDuplicates) {
                    // Use stable key format: docId_elementName_index
                    int currentIndex = elementIndices.getOrDefault(tagName, 0);
                    String stableKey = generateStableKey(documentId, tagName, currentIndex);
                    
                    // Extract element value and add metadata
                    Object baseValue = extractElementValue(childElement);
                    Map<String, Object> enhancedValue = enhanceWithMetadata(
                        baseValue, tagName, documentId, currentIndex
                    );
                    
                    result.put(stableKey, enhancedValue);
                    elementIndices.put(tagName, currentIndex + 1);
                } else {
                    // Single occurrence - use direct key mapping
                    Object value = extractElementValue(childElement);
                    result.put(tagName, value);
                }
            }
            child = child.getNextSibling();
        }
        
        return result;
    }
    
    /**
     * Convert Map with stable keys back to detail element
     * @param detailMap Map with CRDT-optimized keys
     * @param document The DOM document for creating elements
     * @return Reconstructed detail element
     */
    public Element convertMapToDetailElementFromStableKeys(Map<String, Object> detailMap, Document document) {
        if (detailMap == null || detailMap.isEmpty()) {
            return null;
        }
        
        Element detailElement = document.createElement("detail");
        
        // Separate direct elements from stable key elements
        Map<String, Object> directElements = new HashMap<>();
        Map<String, List<StableElement>> stableElements = new HashMap<>();
        
        for (Map.Entry<String, Object> entry : detailMap.entrySet()) {
            String key = entry.getKey();
            Object value = entry.getValue();
            
            if (isStableKey(key)) {
                // Extract stable key info
                if (value instanceof Map) {
                    @SuppressWarnings("unchecked")
                    Map<String, Object> valueMap = (Map<String, Object>) value;
                    
                    String originalTag = (String) valueMap.get(TAG_METADATA);
                    Integer elementIndex = (Integer) valueMap.get(INDEX_METADATA);
                    
                    if (originalTag != null && elementIndex != null) {
                        Map<String, Object> cleanedValue = removeMetadata(valueMap);
                        
                        stableElements.computeIfAbsent(originalTag, k -> new ArrayList<>())
                            .add(new StableElement(elementIndex, cleanedValue));
                    }
                }
            } else {
                // Direct key mapping (single elements)
                directElements.put(key, value);
            }
        }
        
        // Add direct elements first
        for (Map.Entry<String, Object> entry : directElements.entrySet()) {
            Element childElement = createElementFromValue(document, entry.getKey(), entry.getValue());
            if (childElement != null) {
                detailElement.appendChild(childElement);
            }
        }
        
        // Add stable key elements, sorted by index within each group
        for (Map.Entry<String, List<StableElement>> entry : stableElements.entrySet()) {
            String tagName = entry.getKey();
            List<StableElement> elements = entry.getValue();
            
            // Sort by element index
            elements.sort(Comparator.comparingInt(e -> e.index));
            
            for (StableElement element : elements) {
                Element childElement = createElementFromValue(document, tagName, element.value);
                if (childElement != null) {
                    detailElement.appendChild(childElement);
                }
            }
        }
        
        return detailElement;
    }
    
    /**
     * Generate a stable key for duplicate elements
     */
    private String generateStableKey(String documentId, String elementName, int index) {
        return documentId + KEY_SEPARATOR + elementName + KEY_SEPARATOR + index;
    }
    
    /**
     * Check if a key is a stable key (contains separators)
     */
    private boolean isStableKey(String key) {
        String[] parts = key.split(KEY_SEPARATOR);
        return parts.length >= 3 && isNumeric(parts[parts.length - 1]);
    }
    
    /**
     * Check if string is numeric
     */
    private boolean isNumeric(String str) {
        try {
            Integer.parseInt(str);
            return true;
        } catch (NumberFormatException e) {
            return false;
        }
    }
    
    /**
     * Enhance value with metadata for reconstruction
     */
    private Map<String, Object> enhanceWithMetadata(Object baseValue, String tagName, 
                                                   String docId, int elementIndex) {
        Map<String, Object> enhanced = new HashMap<>();
        
        // Add metadata
        enhanced.put(TAG_METADATA, tagName);
        enhanced.put(DOC_ID_METADATA, docId);
        enhanced.put(INDEX_METADATA, elementIndex);
        
        // Add original value content
        if (baseValue instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> baseMap = (Map<String, Object>) baseValue;
            enhanced.putAll(baseMap);
        } else if (baseValue instanceof String) {
            enhanced.put("_text", baseValue);
        }
        
        return enhanced;
    }
    
    /**
     * Remove metadata fields from a value map
     */
    private Map<String, Object> removeMetadata(Map<String, Object> valueMap) {
        Map<String, Object> cleaned = new HashMap<>(valueMap);
        cleaned.remove(TAG_METADATA);
        cleaned.remove(DOC_ID_METADATA);
        cleaned.remove(INDEX_METADATA);
        return cleaned;
    }
    
    /**
     * Create an XML element from a value object
     */
    private Element createElementFromValue(Document document, String elementName, Object value) {
        Element element = document.createElement(elementName);
        
        if (value instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> valueMap = (Map<String, Object>) value;
            
            // Set attributes and text content
            for (Map.Entry<String, Object> entry : valueMap.entrySet()) {
                String key = entry.getKey();
                Object val = entry.getValue();
                
                if (key.equals("_text")) {
                    element.setTextContent(val.toString());
                } else if (!key.startsWith("_")) { // Skip metadata fields
                    element.setAttribute(key, val.toString());
                }
            }
        } else {
            // Simple text content
            element.setTextContent(value.toString());
        }
        
        return element;
    }
    
    /**
     * Helper class to store stable key elements with their index
     */
    private static class StableElement {
        final int index;
        final Object value;
        
        StableElement(int index, Object value) {
            this.index = index;
            this.value = value;
        }
    }
    
    /**
     * Get the next available index for a given element type
     * This is useful when adding new elements in a P2P network
     */
    public int getNextAvailableIndex(Map<String, Object> detailMap, String documentId, String elementName) {
        int maxIndex = -1;
        
        String keyPrefix = documentId + KEY_SEPARATOR + elementName + KEY_SEPARATOR;
        
        for (String key : detailMap.keySet()) {
            if (key.startsWith(keyPrefix)) {
                String indexStr = key.substring(keyPrefix.length());
                try {
                    int index = Integer.parseInt(indexStr);
                    maxIndex = Math.max(maxIndex, index);
                } catch (NumberFormatException e) {
                    // Ignore malformed keys
                }
            }
        }
        
        return maxIndex + 1;
    }
}