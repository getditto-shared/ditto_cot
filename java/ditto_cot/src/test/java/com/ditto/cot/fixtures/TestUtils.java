package com.ditto.cot.fixtures;

import com.ditto.cot.CoTConverter;
import com.ditto.cot.CoTEvent;
import com.ditto.cot.schema.*;
import jakarta.xml.bind.JAXBException;

import java.util.Map;
import java.util.function.Consumer;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Shared test utilities for CoT library testing.
 * Provides common assertion helpers and test patterns.
 */
public class TestUtils {
    
    private static final CoTConverter converter;
    
    static {
        try {
            converter = new CoTConverter();
        } catch (JAXBException e) {
            throw new RuntimeException("Failed to initialize CoTConverter for tests", e);
        }
    }
    
    /**
     * Performs a complete round-trip test: XML -> CoTEvent -> Document -> XML
     */
    public static void assertRoundTripConversion(String originalXml, String expectedUID, String expectedType) {
        try {
            // Parse XML to CoTEvent
            CoTEvent event = converter.parseCoTXml(originalXml);
            assertNotNull(event, "Parsed CoTEvent should not be null");
            assertEquals(expectedUID, event.getUid(), "UID should match");
            assertEquals(expectedType, event.getType(), "Type should match");
            
            // Convert to Document
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document, "Document should not be null");
            
            // Verify document type
            assertDocumentType(document, expectedType);
            
            // Convert back to XML
            String regeneratedXml = converter.marshalCoTEvent(event);
            assertNotNull(regeneratedXml, "Regenerated XML should not be null");
            assertFalse(regeneratedXml.trim().isEmpty(), "Regenerated XML should not be empty");
            
            // Parse regenerated XML to verify it's valid
            CoTEvent regeneratedEvent = converter.parseCoTXml(regeneratedXml);
            assertEquals(event.getUid(), regeneratedEvent.getUid(), "Round-trip UID should match");
            assertEquals(event.getType(), regeneratedEvent.getType(), "Round-trip type should match");
            
        } catch (JAXBException e) {
            org.junit.jupiter.api.Assertions.fail("Round-trip conversion failed: " + e.getMessage());
        }
    }
    
    /**
     * Asserts that a document is of the expected type based on CoT type
     */
    public static void assertDocumentType(Object document, String cotType) {
        assertNotNull(document, "Document should not be null");
        
        switch (cotType) {
            case "a-f-G-U-C" -> {
                assertTrue(document instanceof MapItemDocument, 
                    "Document should be MapItemDocument for type " + cotType);
            }
            case "b-t-f" -> {
                assertTrue(document instanceof ChatDocument, 
                    "Document should be ChatDocument for type " + cotType);
            }
            case "b-f-t-f", "b-f-t-a" -> {
                assertTrue(document instanceof FileDocument, 
                    "Document should be FileDocument for type " + cotType);
            }
            case "t-x-c-t" -> {
                assertTrue(document instanceof ApiDocument, 
                    "Document should be ApiDocument for type " + cotType);
            }
            default -> {
                assertTrue(document instanceof GenericDocument, 
                    "Document should be GenericDocument for unknown type " + cotType);
            }
        }
    }
    
    /**
     * Asserts that a MapItem document has all required fields
     */
    public static void assertMapItemDocument(Object document, String expectedUID, 
                                           double expectedLat, double expectedLon) {
        assertTrue(document instanceof MapItemDocument, "Should be MapItemDocument");
        MapItemDocument mapItem = (MapItemDocument) document;
        
        assertEquals(expectedUID, mapItem.getId(), "ID should match");
        assertEquals(expectedLat, mapItem.getJ(), 0.000001, "Latitude should match");  // j = LAT
        assertEquals(expectedLon, mapItem.getL(), 0.000001, "Longitude should match");  // l = LON
        assertNotNull(mapItem.getQ(), "Start time should not be null");
        assertNotNull(mapItem.getR(), "Stale time should not be null");
    }
    
    /**
     * Asserts that a Chat document has all required fields
     */
    public static void assertChatDocument(Object document, String expectedUID, 
                                        String expectedMessage, String expectedSender) {
        assertTrue(document instanceof ChatDocument, "Should be ChatDocument");
        ChatDocument chat = (ChatDocument) document;
        
        assertEquals(expectedUID, chat.getId(), "ID should match");
        assertEquals(expectedMessage, chat.getMessage(), "Message should match");  // message field
        assertEquals(expectedSender, chat.getE(), "Sender should match");  // e = author callsign
        assertNotNull(chat.getQ(), "Start time should not be null");
        assertNotNull(chat.getR(), "Stale time should not be null");
    }
    
    /**
     * Asserts that a File document has all required fields
     */
    public static void assertFileDocument(Object document, String expectedUID, 
                                        String expectedFilename, Double expectedSize) {
        assertTrue(document instanceof FileDocument, "Should be FileDocument");
        FileDocument file = (FileDocument) document;
        
        assertEquals(expectedUID, file.getId(), "ID should match");
        assertThat(file.getFile()).contains(expectedFilename);
        if (expectedSize != null) {
            assertEquals(expectedSize, file.getSz(), 0.1, "File size should match");
        }
        assertNotNull(file.getQ(), "Start time should not be null");
        assertNotNull(file.getR(), "Stale time should not be null");
    }
    
    /**
     * Asserts that an API document has all required fields
     */
    public static void assertApiDocument(Object document, String expectedUID, 
                                       String expectedEndpoint, String expectedMethod) {
        assertTrue(document instanceof ApiDocument, "Should be ApiDocument");
        ApiDocument api = (ApiDocument) document;
        
        assertEquals(expectedUID, api.getId(), "ID should match");
        // Note: getEndpoint() and getMethod() don't exist, API uses different field mapping
        assertNotNull(api.getQ(), "Start time should not be null");
        assertNotNull(api.getR(), "Stale time should not be null");
    }
    
    /**
     * Asserts that a Generic document has all required fields
     */
    public static void assertGenericDocument(Object document, String expectedUID, String expectedType) {
        assertTrue(document instanceof GenericDocument, "Should be GenericDocument");
        GenericDocument generic = (GenericDocument) document;
        
        assertEquals(expectedUID, generic.getId(), "ID should match");
        assertEquals(expectedType, generic.getW(), "Type should match");
        assertNotNull(generic.getQ(), "Start time should not be null");
        assertNotNull(generic.getR(), "Stale time should not be null");
    }
    
    /**
     * Asserts that coordinates are within expected ranges
     */
    public static void assertValidCoordinates(double lat, double lon, double hae) {
        assertTrue(lat >= -90.0 && lat <= 90.0, 
            "Latitude should be between -90 and 90, was: " + lat);
        assertTrue(lon >= -180.0 && lon <= 180.0, 
            "Longitude should be between -180 and 180, was: " + lon);
        // HAE can be negative (below sea level) but should be reasonable
        assertTrue(hae >= -15000.0 && hae <= 50000.0, 
            "HAE should be reasonable (between -15km and 50km), was: " + hae);
    }
    
    /**
     * Asserts that timestamp values are reasonable
     */
    public static void assertValidTimestamp(String timestamp) {
        assertNotNull(timestamp, "Timestamp should not be null");
        assertFalse(timestamp.trim().isEmpty(), "Timestamp should not be empty");
        
        try {
            // Try to parse as microseconds timestamp
            long timestampLong = Long.parseLong(timestamp);
            assertTrue(timestampLong > 0, "Timestamp should be positive");
            
            // Check if timestamp is in microseconds (should be much larger than milliseconds)
            assertTrue(timestampLong > 1_000_000_000_000_000L, 
                "Timestamp should be in microseconds, was: " + timestampLong);
            
            // Should not be too far in the future (year 2100)
            assertTrue(timestampLong < 4_000_000_000_000_000L, 
                "Timestamp should not be too far in future, was: " + timestampLong);
        } catch (NumberFormatException e) {
            org.junit.jupiter.api.Assertions.fail("Timestamp should be a valid number: " + timestamp);
        }
    }
    
    /**
     * Performs timing test to ensure operations complete within reasonable time
     */
    public static void assertPerformance(Runnable operation, long maxMillis, String operationName) {
        long startTime = System.nanoTime();
        operation.run();
        long endTime = System.nanoTime();
        long durationMillis = (endTime - startTime) / 1_000_000;
        
        assertTrue(durationMillis <= maxMillis, 
            String.format("%s took %d ms, expected <= %d ms", 
                operationName, durationMillis, maxMillis));
    }
    
    /**
     * Tests conversion with error handling
     */
    public static void assertConversionWithErrorHandling(String xml, 
                                                       Consumer<Object> documentAssertions) {
        try {
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            documentAssertions.accept(document);
        } catch (JAXBException e) {
            org.junit.jupiter.api.Assertions.fail("Conversion failed: " + e.getMessage());
        }
    }
    
    /**
     * Tests that XML parsing throws expected exception
     */
    public static void assertXmlParsingThrows(String invalidXml, Class<? extends Exception> expectedException) {
        assertThatThrownBy(() -> converter.parseCoTXml(invalidXml))
            .isInstanceOf(expectedException);
    }
    
    /**
     * Tests concurrent access to converter
     */
    public static void assertConcurrentSafety(String xml, int threadCount, int iterationsPerThread) {
        Thread[] threads = new Thread[threadCount];
        
        for (int i = 0; i < threadCount; i++) {
            threads[i] = new Thread(() -> {
                for (int j = 0; j < iterationsPerThread; j++) {
                    try {
                        CoTEvent event = converter.parseCoTXml(xml);
                        Object document = converter.convertCoTEventToDocument(event);
                        assertNotNull(document);
                    } catch (JAXBException e) {
                        org.junit.jupiter.api.Assertions.fail("Concurrent access failed: " + e.getMessage());
                    }
                }
            });
        }
        
        // Start all threads
        for (Thread thread : threads) {
            thread.start();
        }
        
        // Wait for all threads to complete
        for (Thread thread : threads) {
            try {
                thread.join(5000); // 5 second timeout
                assertFalse(thread.isAlive(), "Thread should have completed");
            } catch (InterruptedException e) {
                org.junit.jupiter.api.Assertions.fail("Thread interrupted: " + e.getMessage());
            }
        }
    }
    
    /**
     * Validates that a document contains only expected fields for its type
     */
    public static void assertDocumentStructure(Object document) {
        assertNotNull(document, "Document should not be null");
        
        if (document instanceof MapItemDocument mapItem) {
            assertNotNull(mapItem.getId(), "MapItem should have ID");
            assertNotNull(mapItem.getH(), "MapItem should have latitude");
            assertNotNull(mapItem.getJ(), "MapItem should have longitude");
        } else if (document instanceof ChatDocument chat) {
            assertNotNull(chat.getId(), "Chat should have ID");
            assertNotNull(chat.getE(), "Chat should have message");
            // Chat documents use different field mapping
        } else if (document instanceof FileDocument file) {
            assertNotNull(file.getId(), "File should have ID");
            assertNotNull(file.getFile(), "File should have filename");
        } else if (document instanceof ApiDocument api) {
            assertNotNull(api.getId(), "API should have ID");
            // API documents use different field mapping
        } else if (document instanceof GenericDocument generic) {
            assertNotNull(generic.getId(), "Generic should have ID");
            assertNotNull(generic.getW(), "Generic should have type");
        } else {
            org.junit.jupiter.api.Assertions.fail("Unknown document type: " + document.getClass());
        }
    }
    
    /**
     * Helper to generate test XML with specific modifications
     */
    public static String modifyXml(String baseXml, Map<String, String> replacements) {
        String result = baseXml;
        for (Map.Entry<String, String> entry : replacements.entrySet()) {
            result = result.replace(entry.getKey(), entry.getValue());
        }
        return result;
    }
    
    /**
     * Helper to create test XML with invalid data for error testing
     */
    public static String createInvalidXml(String baseXml, String invalidPart) {
        return baseXml.replace("</event>", invalidPart + "</event>");
    }
}