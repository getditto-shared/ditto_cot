package com.ditto.cot;

import com.ditto.cot.schema.*;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for SDK Document Conversion Utilities
 */
class SdkDocumentConverterTest {
    
    private SdkDocumentConverter converter;
    private ObjectMapper objectMapper;
    
    @BeforeEach
    void setUp() throws Exception {
        converter = new SdkDocumentConverter();
        objectMapper = new ObjectMapper();
    }
    
    @Test
    void testObserverMapToTypedDocument_MapItem() {
        // Create a mock observer document map for a MapItem
        Map<String, Object> observerMap = createMapItemDocument();
        
        Object result = converter.observerMapToTypedDocument(observerMap);
        
        assertNotNull(result);
        assertTrue(result instanceof MapItemDocument);
        
        MapItemDocument mapItem = (MapItemDocument) result;
        assertEquals("test-map-001", mapItem.getId());
        assertEquals("a-u-r-loc-g", mapItem.getW());
        assertEquals(37.7749, mapItem.getJ(), 0.001);
        assertEquals(-122.4194, mapItem.getL(), 0.001);
        
        // Verify r-field reconstruction
        assertNotNull(mapItem.getR());
        assertTrue(mapItem.getR().containsKey("contact"));
        
        @SuppressWarnings("unchecked")
        Map<String, Object> contact = (Map<String, Object>) mapItem.getR().get("contact");
        assertEquals("TestUnit", contact.get("callsign"));
    }
    
    @Test
    void testObserverMapToTypedDocument_Chat() {
        // Create a mock observer document map for a ChatDocument
        Map<String, Object> observerMap = createChatDocument();
        
        Object result = converter.observerMapToTypedDocument(observerMap);
        
        assertNotNull(result);
        assertTrue(result instanceof ChatDocument);
        
        ChatDocument chat = (ChatDocument) result;
        assertEquals("test-chat-001", chat.getId());
        assertEquals("b-t-f", chat.getW());
    }
    
    @Test
    void testObserverMapToTypedDocument_Generic() {
        // Create a mock observer document map for unknown type
        Map<String, Object> observerMap = createGenericDocument();
        
        Object result = converter.observerMapToTypedDocument(observerMap);
        
        assertNotNull(result);
        assertTrue(result instanceof GenericDocument);
        
        GenericDocument generic = (GenericDocument) result;
        assertEquals("test-generic-001", generic.getId());
        assertEquals("unknown-type", generic.getW());
    }
    
    @Test
    void testObserverMapToTypedDocument_NullInput() {
        Object result = converter.observerMapToTypedDocument(null);
        assertNull(result);
    }
    
    @Test
    void testObserverMapToJsonWithRFields() {
        // Create a mock document with flattened r_* fields
        Map<String, Object> observerMap = createMapItemDocument();
        
        String result = converter.observerMapToJsonWithRFields(observerMap);
        
        assertNotNull(result);
        
        // Parse the result JSON to verify structure
        try {
            Map<String, Object> resultMap = objectMapper.readValue(result, 
                new TypeReference<Map<String, Object>>() {});
            
            // Verify basic fields are preserved
            assertEquals("test-map-001", resultMap.get("_id"));
            assertEquals("a-u-r-loc-g", resultMap.get("w"));
            
            // Verify r-field was reconstructed
            assertTrue(resultMap.containsKey("r"));
            
            @SuppressWarnings("unchecked")
            Map<String, Object> rField = (Map<String, Object>) resultMap.get("r");
            assertTrue(rField.containsKey("contact"));
            assertTrue(rField.containsKey("track"));
            
            @SuppressWarnings("unchecked")
            Map<String, Object> contact = (Map<String, Object>) rField.get("contact");
            assertEquals("TestUnit", contact.get("callsign"));
            
            @SuppressWarnings("unchecked")
            Map<String, Object> track = (Map<String, Object>) rField.get("track");
            assertEquals("15.0", track.get("speed"));
            assertEquals("90.0", track.get("course"));
            
            // Verify original r_* fields are not present
            assertFalse(resultMap.containsKey("r_contact_callsign"));
            assertFalse(resultMap.containsKey("r_track_speed"));
            assertFalse(resultMap.containsKey("r_track_course"));
            
        } catch (Exception e) {
            fail("Failed to parse result JSON: " + e.getMessage());
        }
    }
    
    @Test
    void testObserverJsonToTypedDocument() {
        // Create JSON string representation of a MapItem document
        Map<String, Object> docMap = createMapItemDocument();
        try {
            String jsonStr = objectMapper.writeValueAsString(docMap);
            
            Object result = converter.observerJsonToTypedDocument(jsonStr);
            
            assertNotNull(result);
            assertTrue(result instanceof MapItemDocument);
            
            MapItemDocument mapItem = (MapItemDocument) result;
            assertEquals("test-map-001", mapItem.getId());
            assertEquals("a-u-r-loc-g", mapItem.getW());
            
        } catch (Exception e) {
            fail("Failed to create test JSON: " + e.getMessage());
        }
    }
    
    @Test
    void testObserverJsonToTypedDocument_InvalidJson() {
        String invalidJson = "{ invalid json }";
        
        Object result = converter.observerJsonToTypedDocument(invalidJson);
        
        assertNull(result);
    }
    
    @Test
    void testObserverJsonToJsonWithRFields() {
        // Create JSON string with flattened r_* fields
        Map<String, Object> docMap = createMapItemDocument();
        try {
            String inputJson = objectMapper.writeValueAsString(docMap);
            
            String result = converter.observerJsonToJsonWithRFields(inputJson);
            
            assertNotNull(result);
            
            // Parse and verify the reconstructed JSON
            Map<String, Object> resultMap = objectMapper.readValue(result, 
                new TypeReference<Map<String, Object>>() {});
            
            assertTrue(resultMap.containsKey("r"));
            assertFalse(resultMap.containsKey("r_contact_callsign"));
            
        } catch (Exception e) {
            fail("Failed to test JSON conversion: " + e.getMessage());
        }
    }
    
    @Test
    void testGetDocumentId_FromMap() {
        Map<String, Object> docMap = createMapItemDocument();
        
        String result = converter.getDocumentId(docMap);
        
        assertEquals("test-map-001", result);
    }
    
    @Test
    void testGetDocumentId_FromJson() {
        String jsonStr = "{\"_id\": \"test-doc-123\", \"w\": \"a-u-r-loc-g\"}";
        
        String result = converter.getDocumentId(jsonStr);
        
        assertEquals("test-doc-123", result);
    }
    
    @Test
    void testGetDocumentId_FallbackToId() {
        Map<String, Object> docMap = new HashMap<>();
        docMap.put("id", "fallback-id");  // No _id, should use id
        docMap.put("w", "a-u-r-loc-g");
        
        String result = converter.getDocumentId(docMap);
        
        assertEquals("fallback-id", result);
    }
    
    @Test
    void testGetDocumentId_NullInput() {
        String result = converter.getDocumentId(null);
        assertNull(result);
    }
    
    @Test
    void testGetDocumentType_FromMap() {
        Map<String, Object> docMap = createMapItemDocument();
        
        String result = converter.getDocumentType(docMap);
        
        assertEquals("a-u-r-loc-g", result);
    }
    
    @Test
    void testGetDocumentType_FromJson() {
        String jsonStr = "{\"_id\": \"test-doc-123\", \"w\": \"b-t-f\"}";
        
        String result = converter.getDocumentType(jsonStr);
        
        assertEquals("b-t-f", result);
    }
    
    @Test
    void testGetDocumentType_NullInput() {
        String result = converter.getDocumentType(null);
        assertNull(result);
    }
    
    @Test
    void testGetDocumentIdFromMap_NullMap() {
        String result = converter.getDocumentIdFromMap(null);
        assertNull(result);
    }
    
    @Test
    void testGetDocumentIdFromJson_EmptyString() {
        String result = converter.getDocumentIdFromJson("");
        assertNull(result);
    }
    
    @Test
    void testGetDocumentTypeFromMap_NullMap() {
        String result = converter.getDocumentTypeFromMap(null);
        assertNull(result);
    }
    
    @Test
    void testGetDocumentTypeFromJson_EmptyString() {
        String result = converter.getDocumentTypeFromJson("");
        assertNull(result);
    }
    
    @Test
    void testDocumentTypeDetection_ApiDocument() {
        Map<String, Object> apiDoc = new HashMap<>();
        apiDoc.put("_id", "test-api");
        apiDoc.put("w", "t-x-c-t");  // API type
        
        Object result = converter.observerMapToTypedDocument(apiDoc);
        
        assertTrue(result instanceof ApiDocument);
    }
    
    @Test
    void testDocumentTypeDetection_FileDocument() {
        Map<String, Object> fileDoc = new HashMap<>();
        fileDoc.put("_id", "test-file");
        fileDoc.put("w", "b-f-t-f");  // File share type
        
        Object result = converter.observerMapToTypedDocument(fileDoc);
        
        assertTrue(result instanceof FileDocument);
    }
    
    // Helper methods to create test documents
    
    private Map<String, Object> createMapItemDocument() {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", "test-map-001");
        doc.put("w", "a-u-r-loc-g");
        doc.put("a", "test-peer");
        doc.put("b", 1642248600000000.0);
        doc.put("d", "test-map-001");
        doc.put("_c", 1);
        doc.put("_r", false);
        doc.put("_v", 2);
        doc.put("e", "TestUnit");
        doc.put("g", "2.0");
        doc.put("h", 5.0);
        doc.put("i", 10.0);
        doc.put("j", 37.7749);
        doc.put("k", 2.0);
        doc.put("l", -122.4194);
        doc.put("n", 1642248600000000.0);
        doc.put("o", 1642252200000000.0);
        doc.put("p", "h-g-i-g-o");
        doc.put("q", "");
        doc.put("s", "");
        doc.put("t", "");
        doc.put("u", "");
        doc.put("v", "");
        
        // Flattened r_* fields
        doc.put("r_contact_callsign", "TestUnit");
        doc.put("r_track_speed", "15.0");
        doc.put("r_track_course", "90.0");
        
        return doc;
    }
    
    private Map<String, Object> createChatDocument() {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", "test-chat-001");
        doc.put("w", "b-t-f");
        doc.put("a", "test-peer");
        doc.put("b", 1642248600000000.0);
        doc.put("d", "test-chat-001");
        doc.put("_c", 1);
        doc.put("_r", false);
        doc.put("_v", 2);
        
        // Chat-specific flattened r_* fields
        doc.put("r_remarks", "Hello from test");
        doc.put("r_chat_chatroom", "test-room");
        doc.put("r_chat_senderCallsign", "TestUser");
        
        return doc;
    }
    
    private Map<String, Object> createGenericDocument() {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", "test-generic-001");
        doc.put("w", "unknown-type");
        doc.put("a", "test-peer");
        doc.put("b", 1642248600000000.0);
        doc.put("d", "test-generic-001");
        doc.put("_c", 1);
        doc.put("_r", false);
        doc.put("_v", 2);
        
        return doc;
    }
}