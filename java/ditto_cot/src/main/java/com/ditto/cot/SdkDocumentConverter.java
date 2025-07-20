package com.ditto.cot;

import com.ditto.cot.schema.*;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.DeserializationFeature;

import java.util.Map;
import java.util.Optional;

/**
 * SDK Document Conversion Utilities
 * 
 * This class provides utilities to convert documents from Ditto SDK observer callbacks
 * to typed schema objects and JSON representations with proper r-field reconstruction.
 * 
 * These utilities solve the limitation where observer callbacks could only extract document IDs
 * but couldn't access full document content or convert it to typed objects.
 */
public class SdkDocumentConverter {
    
    private final ObjectMapper objectMapper;
    private final CoTConverter cotConverter;
    
    public SdkDocumentConverter() throws Exception {
        this.objectMapper = new ObjectMapper();
        // Configure ObjectMapper to ignore unknown properties and not require type info
        this.objectMapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        this.objectMapper.configure(DeserializationFeature.FAIL_ON_INVALID_SUBTYPE, false);
        this.cotConverter = new CoTConverter();
    }
    
    /**
     * Convert observer document Map to a typed schema object based on document type
     * 
     * This function takes the Map&lt;String, Object&gt; from an observer document (via item.getValue())
     * and converts it to the appropriate schema class variant based on the document's
     * 'w' field (event type). This is the main function for getting typed access to observer documents.
     * 
     * @param observerDocumentMap Map&lt;String, Object&gt; from item.getValue() in observer callback
     * @return The converted schema object or null if conversion fails
     * 
     * Example usage:
     * <pre>
     * // In observer callback
     * store.registerObserver("SELECT * FROM map_items", (result, event) -&gt; {
     *     for (DittoQueryResultItem item : result.getItems()) {
     *         Map&lt;String, Object&gt; docMap = item.getValue();
     *         Object typedDoc = converter.observerMapToTypedDocument(docMap);
     *         
     *         if (typedDoc instanceof MapItemDocument) {
     *             MapItemDocument mapItem = (MapItemDocument) typedDoc;
     *             System.out.println("Received map item: " + mapItem.getId());
     *         } else if (typedDoc instanceof ChatDocument) {
     *             ChatDocument chat = (ChatDocument) typedDoc;
     *             System.out.println("Received chat: " + chat.getMessage());
     *         }
     *     }
     * });
     * </pre>
     */
    public Object observerMapToTypedDocument(Map<String, Object> observerDocumentMap) {
        if (observerDocumentMap == null) {
            return null;
        }
        
        try {
            // Unflatten r_* fields back to nested r field for proper parsing
            Map<String, Object> unflattenedMap = cotConverter.unflattenRField(observerDocumentMap);
            
            // Determine document type based on 'w' field
            String docType = getDocumentTypeFromMap(unflattenedMap);
            if (docType == null) {
                // Try to convert as GenericDocument if no type is found
                return convertMapToDocument(unflattenedMap, GenericDocument.class);
            }
            
            // Convert to appropriate schema class based on type
            if (isApiDocumentType(docType)) {
                return convertMapToDocument(unflattenedMap, ApiDocument.class);
            } else if (isChatDocumentType(docType)) {
                return convertMapToDocument(unflattenedMap, ChatDocument.class);
            } else if (isFileDocumentType(docType)) {
                return convertMapToDocument(unflattenedMap, FileDocument.class);
            } else if (isMapItemType(docType)) {
                return convertMapToDocument(unflattenedMap, MapItemDocument.class);
            } else {
                return convertMapToDocument(unflattenedMap, GenericDocument.class);
            }
            
        } catch (Exception e) {
            // Log the error for debugging
            System.err.println("Error converting observer document: " + e.getMessage());
            e.printStackTrace();
            return null;
        }
    }
    
    /**
     * Convert observer document Map to JSON with reconstructed r-fields
     * 
     * This function takes the Map&lt;String, Object&gt; from an observer document and reconstructs
     * the hierarchical r-field structure from flattened r_* fields. This gives you 
     * the full document structure as it would appear in the original CoT event.
     * 
     * @param observerDocumentMap Map&lt;String, Object&gt; from item.getValue() in observer callback
     * @return JSON string with r-field reconstruction or null if conversion fails
     * 
     * Example usage:
     * <pre>
     * // Example with flattened r_* fields
     * Map&lt;String, Object&gt; docMap = item.getValue(); // Contains r_contact_callsign, etc.
     * String jsonStr = converter.observerMapToJsonWithRFields(docMap);
     * 
     * // Result JSON will have nested structure:
     * // {
     * //   "_id": "test",
     * //   "w": "a-u-r-loc-g", 
     * //   "r": {
     * //     "contact": {
     * //       "callsign": "TestUnit"
     * //     }
     * //   }
     * // }
     * </pre>
     */
    public String observerMapToJsonWithRFields(Map<String, Object> observerDocumentMap) {
        if (observerDocumentMap == null) {
            return null;
        }
        
        try {
            // Unflatten r_* fields back to nested r field
            Map<String, Object> unflattenedMap = cotConverter.unflattenRField(observerDocumentMap);
            
            // Convert to JSON string
            return objectMapper.writeValueAsString(unflattenedMap);
        } catch (JsonProcessingException e) {
            return null;
        }
    }
    
    /**
     * Convert observer document JSON string to typed schema object
     * 
     * This function takes a JSON string representation of an observer document
     * and converts it to the appropriate schema class. Useful when you have
     * JSON from other sources or need to work with JSON representations.
     * 
     * @param observerDocumentJson JSON string representation of the document
     * @return The converted schema object or null if conversion fails
     * 
     * Example usage:
     * <pre>
     * String jsonStr = "{\"_id\": \"test\", \"w\": \"a-u-r-loc-g\", \"j\": 37.7749, \"l\": -122.4194}";
     * Object typedDoc = converter.observerJsonToTypedDocument(jsonStr);
     * 
     * if (typedDoc instanceof MapItemDocument) {
     *     MapItemDocument mapItem = (MapItemDocument) typedDoc;
     *     System.out.println("Map item ID: " + mapItem.getId());
     * }
     * </pre>
     */
    public Object observerJsonToTypedDocument(String observerDocumentJson) {
        if (observerDocumentJson == null || observerDocumentJson.trim().isEmpty()) {
            return null;
        }
        
        try {
            // Parse JSON to Map first
            Map<String, Object> docMap = objectMapper.readValue(observerDocumentJson, 
                new TypeReference<Map<String, Object>>() {});
            
            // Use the Map conversion method
            return observerMapToTypedDocument(docMap);
        } catch (JsonProcessingException e) {
            return null;
        }
    }
    
    /**
     * Convert observer document JSON string to JSON with reconstructed r-fields
     * 
     * @param observerDocumentJson JSON string from observer document
     * @return JSON string with r-field reconstruction or null if conversion fails
     */
    public String observerJsonToJsonWithRFields(String observerDocumentJson) {
        if (observerDocumentJson == null || observerDocumentJson.trim().isEmpty()) {
            return null;
        }
        
        try {
            // Parse JSON to Map first
            Map<String, Object> docMap = objectMapper.readValue(observerDocumentJson, 
                new TypeReference<Map<String, Object>>() {});
            
            // Use the Map conversion method
            return observerMapToJsonWithRFields(docMap);
        } catch (JsonProcessingException e) {
            return null;
        }
    }
    
    /**
     * Extract document ID from observer document Map or JSON
     * 
     * This is a convenience function that extracts just the document ID,
     * which is commonly needed in observer callbacks for logging or processing.
     * 
     * @param observerDocument Either Map&lt;String, Object&gt; or JSON string from observer
     * @return The document ID if present, null otherwise
     * 
     * Example usage:
     * <pre>
     * // From Map
     * Map&lt;String, Object&gt; docMap = item.getValue();
     * String id = converter.getDocumentId(docMap);
     * 
     * // From JSON string
     * String jsonStr = "{\"_id\": \"test-123\", \"w\": \"a-u-r-loc-g\"}";
     * String id = converter.getDocumentId(jsonStr);
     * </pre>
     */
    public String getDocumentId(Object observerDocument) {
        if (observerDocument == null) {
            return null;
        }
        
        if (observerDocument instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> docMap = (Map<String, Object>) observerDocument;
            return getDocumentIdFromMap(docMap);
        } else if (observerDocument instanceof String) {
            return getDocumentIdFromJson((String) observerDocument);
        }
        
        return null;
    }
    
    /**
     * Extract document ID from observer document Map
     */
    public String getDocumentIdFromMap(Map<String, Object> observerDocumentMap) {
        if (observerDocumentMap == null) {
            return null;
        }
        
        // Try _id first, then id
        Object id = observerDocumentMap.get("_id");
        if (id == null) {
            id = observerDocumentMap.get("id");
        }
        
        return id != null ? id.toString() : null;
    }
    
    /**
     * Extract document ID from observer document JSON string
     */
    public String getDocumentIdFromJson(String observerDocumentJson) {
        if (observerDocumentJson == null || observerDocumentJson.trim().isEmpty()) {
            return null;
        }
        
        try {
            Map<String, Object> docMap = objectMapper.readValue(observerDocumentJson, 
                new TypeReference<Map<String, Object>>() {});
            return getDocumentIdFromMap(docMap);
        } catch (JsonProcessingException e) {
            return null;
        }
    }
    
    /**
     * Extract document type from observer document Map or JSON
     * 
     * This is a convenience function that extracts the document type (w field),
     * which determines the schema class variant. Useful for filtering or routing
     * different document types in observer callbacks.
     * 
     * @param observerDocument Either Map&lt;String, Object&gt; or JSON string from observer
     * @return The document type if present (e.g., "a-u-r-loc-g", "b-t-f"), null otherwise
     * 
     * Example usage:
     * <pre>
     * Map&lt;String, Object&gt; docMap = item.getValue();
     * String docType = converter.getDocumentType(docMap);
     * 
     * switch (docType) {
     *     case "a-u-r-loc-g":
     *         System.out.println("Received location update");
     *         break;
     *     case "b-t-f":
     *         System.out.println("Received chat message");
     *         break;
     *     default:
     *         System.out.println("Received " + docType);
     * }
     * </pre>
     */
    public String getDocumentType(Object observerDocument) {
        if (observerDocument == null) {
            return null;
        }
        
        if (observerDocument instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> docMap = (Map<String, Object>) observerDocument;
            return getDocumentTypeFromMap(docMap);
        } else if (observerDocument instanceof String) {
            return getDocumentTypeFromJson((String) observerDocument);
        }
        
        return null;
    }
    
    /**
     * Extract document type from observer document Map
     */
    public String getDocumentTypeFromMap(Map<String, Object> observerDocumentMap) {
        if (observerDocumentMap == null) {
            return null;
        }
        
        Object type = observerDocumentMap.get("w");
        return type != null ? type.toString() : null;
    }
    
    /**
     * Extract document type from observer document JSON string
     */
    public String getDocumentTypeFromJson(String observerDocumentJson) {
        if (observerDocumentJson == null || observerDocumentJson.trim().isEmpty()) {
            return null;
        }
        
        try {
            Map<String, Object> docMap = objectMapper.readValue(observerDocumentJson, 
                new TypeReference<Map<String, Object>>() {});
            return getDocumentTypeFromMap(docMap);
        } catch (JsonProcessingException e) {
            return null;
        }
    }
    
    /**
     * Get the appropriate Ditto collection name for this document
     */
    public String getCollectionName(Object document) {
        if (document instanceof MapItemDocument) {
            MapItemDocument mapItem = (MapItemDocument) document;
            // Check if this is a track (PLI/location with track data) or map item (persistent graphics)
            if (isTrackDocument(mapItem)) {
                return "track";
            } else {
                return "map_items";
            }
        } else if (document instanceof ChatDocument) {
            return "chat_messages";
        } else if (document instanceof FileDocument) {
            return "files";
        } else if (document instanceof ApiDocument) {
            return "api_events";
        } else {
            return "generic";
        }
    }

    /**
     * Determine if a MapItemDocument should be considered a track (transient location/movement)
     * vs a map item (persistent graphics)
     */
    private boolean isTrackDocument(MapItemDocument mapItem) {
        // Check if document contains track data
        boolean hasTrackData = mapItem.getR() != null && mapItem.getR().containsKey("track");
        
        // Check if the CoT type indicates this is a moving entity (track/PLI)
        String cotType = mapItem.getW() != null ? mapItem.getW() : "";
        boolean isTrackType = cotType.contains("a-f-S") ||  // Friendly surface units (like USVs)
                             cotType.contains("a-f-A") ||  // Friendly air units  
                             cotType.contains("a-f-G") ||  // Friendly ground units
                             cotType.contains("a-u-S") ||  // Unknown surface units
                             cotType.contains("a-u-A") ||  // Unknown air units
                             cotType.contains("a-u-G") ||  // Unknown ground units
                             cotType.contains("a-h-S") ||  // Hostile surface units
                             cotType.contains("a-h-A") ||  // Hostile air units
                             cotType.contains("a-h-G") ||  // Hostile ground units
                             cotType.contains("a-n-") ||   // Neutral units
                             cotType.contains("a-u-r-loc"); // Location reports
        
        return hasTrackData || isTrackType;
    }

    // Private helper methods for document type determination
    // These mirror the logic in CoTConverter
    
    private boolean isApiDocumentType(String cotType) {
        return cotType != null && (
            cotType.equals("t-x-c-t") ||         // Standard CoT API/control type
            cotType.equals("b-m-p-s-p-i") ||     // Sensor point of interest
            cotType.contains("api") ||
            cotType.contains("data")
        );
    }
    
    private boolean isChatDocumentType(String cotType) {
        return cotType != null && (
            cotType.equals("b-t-f") ||           // Standard CoT chat type
            cotType.contains("chat") ||
            cotType.contains("message")
        );
    }
    
    private boolean isFileDocumentType(String cotType) {
        return cotType != null && (
            cotType.equals("b-f-t-f") ||         // Standard CoT file share type
            cotType.equals("b-f-t-a") ||         // Standard CoT file attachment type
            cotType.contains("file") ||
            cotType.contains("attachment")
        );
    }
    
    private boolean isMapItemType(String cotType) {
        return cotType != null && (
            cotType.startsWith("a-f-") || // Friendly units
            cotType.startsWith("a-h-") || // Hostile units  
            cotType.startsWith("a-n-") || // Neutral units
            cotType.equals("a-u-G") ||    // Ground units (specific MapItem type)
            cotType.equals("a-u-S") ||    // Sensor unmanned system
            cotType.equals("a-u-A") ||    // Airborne unmanned system
            cotType.contains("a-u-r-loc") // Location reports
        );
    }
    
    /**
     * Convert a Map to a typed document using the existing CoTConverter's method
     */
    private <T> T convertMapToDocument(Map<String, Object> map, Class<T> documentClass) {
        // Create a copy of the map and add the @type field required by Jackson's polymorphic deserialization
        Map<String, Object> mapWithType = new java.util.HashMap<>(map);
        
        // Add the @type field based on the document class
        // Use the correct type IDs that Jackson expects: [Common, api, chat, file, generic, mapitem]
        if (documentClass == ApiDocument.class) {
            mapWithType.put("@type", "api");
        } else if (documentClass == ChatDocument.class) {
            mapWithType.put("@type", "chat");
        } else if (documentClass == FileDocument.class) {
            mapWithType.put("@type", "file");
        } else if (documentClass == MapItemDocument.class) {
            mapWithType.put("@type", "mapitem");
        } else if (documentClass == GenericDocument.class) {
            mapWithType.put("@type", "generic");
        }
        
        // Use the existing CoTConverter's convertMapToDocument method
        return cotConverter.convertMapToDocument(mapWithType, documentClass);
    }
}