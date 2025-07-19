package com.ditto.cot.example;

import com.ditto.cot.CoTConverter;
import com.ditto.cot.CoTEvent;
import com.ditto.cot.SdkDocumentConverter;
import com.ditto.cot.schema.*;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.util.HashMap;
import java.util.Map;

/**
 * Example demonstrating SDK observer document to typed document conversion
 * 
 * This example shows how to use the new SdkDocumentConverter utilities to extract
 * full document content with proper r-field reconstruction in observer callbacks.
 * 
 * This solves the previous limitation where observer callbacks could only extract
 * document IDs but couldn't access full document content or convert to typed objects.
 * 
 * Note: This example uses mock observer documents to demonstrate the conversion 
 * utilities without requiring the actual Ditto SDK.
 */
public class SdkDocumentConversionExample {
    
    public static void main(String[] args) {
        System.out.println("🚀 SDK Document Conversion Example");
        
        try {
            // Initialize converters
            SdkDocumentConverter sdkConverter = new SdkDocumentConverter();
            CoTConverter cotConverter = new CoTConverter();
            ObjectMapper objectMapper = new ObjectMapper();
            
            // Create mock observer documents to demonstrate the conversion utilities
            System.out.println("📝 Creating mock observer documents...");
            
            // Mock location update document (as it would come from an observer callback)
            Map<String, Object> locationDoc = createMockLocationDocument();
            System.out.println("\n=== Processing Location Document ===");
            processObserverDocument(sdkConverter, cotConverter, objectMapper, locationDoc);
            
            // Mock chat document
            Map<String, Object> chatDoc = createMockChatDocument();
            System.out.println("\n=== Processing Chat Document ===");
            processObserverDocument(sdkConverter, cotConverter, objectMapper, chatDoc);
            
            // Mock file document
            Map<String, Object> fileDoc = createMockFileDocument();
            System.out.println("\n=== Processing File Document ===");
            processObserverDocument(sdkConverter, cotConverter, objectMapper, fileDoc);
            
            // Demonstrate direct conversion from JSON strings
            System.out.println("\n🔄 Testing direct JSON string conversion:");
            String testJson = """
                {
                  "_id": "test-json-conversion",
                  "w": "a-u-r-loc-g",
                  "j": 40.7128,
                  "l": -74.0060,
                  "r_contact_callsign": "JsonTestUnit",
                  "r_track_speed": "25.0"
                }""";
            
            System.out.println("Input JSON: " + testJson);
            
            Object typedFromJson = sdkConverter.observerJsonToTypedDocument(testJson);
            if (typedFromJson instanceof MapItemDocument) {
                MapItemDocument mapItem = (MapItemDocument) typedFromJson;
                System.out.println("Converted to MapItem: " + mapItem.getId());
                System.out.println("R-field content: " + mapItem.getR());
            }
            
            String reconstructedJson = sdkConverter.observerJsonToJsonWithRFields(testJson);
            System.out.println("Reconstructed JSON: " + reconstructedJson);
            
            System.out.println("\n🏁 Example completed successfully!");
            
        } catch (Exception e) {
            System.err.println("❌ Example failed: " + e.getMessage());
            e.printStackTrace();
        }
    }
    
    /**
     * Process a mock observer document to demonstrate the conversion utilities
     */
    private static void processObserverDocument(SdkDocumentConverter sdkConverter, 
                                              CoTConverter cotConverter, 
                                              ObjectMapper objectMapper, 
                                              Map<String, Object> docMap) {
        // Demonstrate document ID extraction
        String docId = sdkConverter.getDocumentId(docMap);
        if (docId != null) {
            System.out.println("  📋 Document ID: " + docId);
        }
        
        // Demonstrate document type extraction
        String docType = sdkConverter.getDocumentType(docMap);
        if (docType != null) {
            System.out.println("  🏷️  Document type: " + docType);
        }
        
        // Convert observer document Map to JSON with r-field reconstruction
        String jsonWithRFields = sdkConverter.observerMapToJsonWithRFields(docMap);
        if (jsonWithRFields != null) {
            System.out.println("  📋 Full JSON representation (with reconstructed r-field):");
            try {
                // Pretty print the JSON
                Object jsonObj = objectMapper.readValue(jsonWithRFields, Object.class);
                String prettyJson = objectMapper.writerWithDefaultPrettyPrinter()
                    .writeValueAsString(jsonObj);
                System.out.println("     " + prettyJson.replace("\n", "\n     "));
            } catch (Exception e) {
                System.out.println("     " + jsonWithRFields);
            }
        }
        
        // Convert observer document Map to typed schema object
        Object typedDoc = sdkConverter.observerMapToTypedDocument(docMap);
        if (typedDoc != null) {
            System.out.println("  🎯 Successfully converted to typed document:");
            
            if (typedDoc instanceof MapItemDocument) {
                MapItemDocument mapItem = (MapItemDocument) typedDoc;
                System.out.println("     MapItem - ID: " + mapItem.getId() + 
                                 ", Lat: " + mapItem.getJ() + 
                                 ", Lon: " + mapItem.getL());
                
                // Show r-field content if present
                if (mapItem.getR() != null && !mapItem.getR().isEmpty()) {
                    System.out.println("     Detail (r-field): " + mapItem.getR());
                }
                
            } else if (typedDoc instanceof ChatDocument) {
                ChatDocument chat = (ChatDocument) typedDoc;
                System.out.println("     Chat - Message: " + chat.getMessage() + 
                                 ", Author: " + chat.getAuthorCallsign());
                
            } else if (typedDoc instanceof FileDocument) {
                FileDocument file = (FileDocument) typedDoc;
                System.out.println("     File - Name: " + file.getFile() + 
                                 ", MIME: " + file.getMime());
                
            } else if (typedDoc instanceof ApiDocument) {
                ApiDocument api = (ApiDocument) typedDoc;
                System.out.println("     API - Content Type: " + api.getContentType());
                
            } else if (typedDoc instanceof GenericDocument) {
                GenericDocument generic = (GenericDocument) typedDoc;
                System.out.println("     Generic - ID: " + generic.getId() + 
                                 ", Type: " + generic.getW());
            }
            
            // Demonstrate round-trip conversion: typed document -> CoTEvent
            try {
                CoTEvent cotEvent = cotConverter.convertDocumentToCoTEvent(typedDoc);
                System.out.println("  🔄 Round-trip to CoTEvent - UID: " + cotEvent.getUid() + 
                                 ", Type: " + cotEvent.getType());
            } catch (Exception e) {
                System.out.println("  ⚠️  Round-trip conversion failed: " + e.getMessage());
            }
            
        } else {
            System.out.println("  ❌ Failed to convert to typed document");
        }
    }
    
    /**
     * Create a mock location update document as it would appear in an observer callback
     */
    private static Map<String, Object> createMockLocationDocument() {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", "test-location-001");
        doc.put("w", "a-u-r-loc-g");
        doc.put("a", "test-peer");
        doc.put("b", 1642248600000000.0);
        doc.put("d", "test-location-001");
        doc.put("_c", 1);
        doc.put("_r", false);
        doc.put("_v", 2);
        doc.put("e", "TestUnit001");
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
        
        // Flattened r_* fields as they would appear in observer documents
        doc.put("r_contact_callsign", "TestUnit001");
        doc.put("r_contact_endpoint", "192.168.1.100:4242:tcp");
        doc.put("r_track_speed", "15.0");
        doc.put("r_track_course", "90.0");
        
        return doc;
    }
    
    /**
     * Create a mock chat document as it would appear in an observer callback
     */
    private static Map<String, Object> createMockChatDocument() {
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
        doc.put("r_remarks", "Hello from SDK conversion example!");
        doc.put("r___chat_chatroom", "test-room");
        doc.put("r___chat_senderCallsign", "TestUser");
        
        return doc;
    }
    
    /**
     * Create a mock file document as it would appear in an observer callback
     */
    private static Map<String, Object> createMockFileDocument() {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", "test-file-001");
        doc.put("w", "b-f-t-f");
        doc.put("a", "test-peer");
        doc.put("b", 1642248600000000.0);
        doc.put("d", "test-file-001");
        doc.put("_c", 1);
        doc.put("_r", false);
        doc.put("_v", 2);
        
        // File-specific flattened r_* fields
        doc.put("r___file_filename", "example.pdf");
        doc.put("r___file_mime", "application/pdf");
        doc.put("r___file_size", "1024000");
        
        return doc;
    }
}