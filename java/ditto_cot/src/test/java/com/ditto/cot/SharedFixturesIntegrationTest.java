package com.ditto.cot;

import com.ditto.cot.fixtures.CoTTestFixtures;
import com.ditto.cot.fixtures.TestUtils;
import com.ditto.cot.schema.*;
import jakarta.xml.bind.JAXBException;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.Map;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration test demonstrating the usage of shared test fixtures
 * and ensuring consistency across different document types.
 */
@DisplayName("Shared Fixtures Integration Tests")
public class SharedFixturesIntegrationTest {
    
    private CoTConverter converter;
    
    @BeforeEach
    void setUp() throws JAXBException {
        converter = new CoTConverter();
    }
    
    @Nested
    @DisplayName("MapItem Document Tests")
    class MapItemTests {
        
        @Test
        @DisplayName("Should convert MapItem using shared fixtures")
        void testMapItemWithSharedFixtures() throws JAXBException {
            String xml = CoTTestFixtures.createMapItemXml(CoTTestFixtures.TestUIDs.MAP_ITEM_1);
            
            TestUtils.assertRoundTripConversion(xml, 
                CoTTestFixtures.TestUIDs.MAP_ITEM_1, 
                CoTTestFixtures.CoTTypes.MAP_ITEM);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            TestUtils.assertMapItemDocument(document, 
                CoTTestFixtures.TestUIDs.MAP_ITEM_1,
                CoTTestFixtures.STANDARD_LAT,
                CoTTestFixtures.STANDARD_LON);
        }
        
        @ParameterizedTest
        @ValueSource(strings = {
            CoTTestFixtures.TestUIDs.MAP_ITEM_1,
            CoTTestFixtures.TestUIDs.MAP_ITEM_2,
            "DYNAMIC-MAP-001",
            "DYNAMIC-MAP-002"
        })
        @DisplayName("Should handle multiple MapItem UIDs")
        void testMultipleMapItemUIDs(String uid) throws JAXBException {
            String xml = CoTTestFixtures.createMapItemXml(uid);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            TestUtils.assertMapItemDocument(document, uid, 
                CoTTestFixtures.STANDARD_LAT, 
                CoTTestFixtures.STANDARD_LON);
        }
        
        @ParameterizedTest
        @ValueSource(strings = {
            "a-u-S",
            "a-u-A"
        })
        @DisplayName("Should handle unmanned system types as MapItem")
        void testUnmannedSystemTypes(String type) throws JAXBException {
            String uid = CoTTestFixtures.generateUID("UNMANNED");
            String xml = CoTTestFixtures.createGenericXml(uid, type);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            TestUtils.assertMapItemDocument(document, uid, 
                CoTTestFixtures.STANDARD_LAT, 
                CoTTestFixtures.STANDARD_LON);
        }
    }
    
    @Nested
    @DisplayName("Chat Document Tests")
    class ChatTests {
        
        @ParameterizedTest
        @CsvSource({
            "CHAT-001, Hello World",
            "CHAT-002, Emergency situation", 
            "CHAT-003, All clear",
            "CHAT-004, Mission briefing"
        })
        @DisplayName("Should convert Chat messages using shared fixtures")
        void testChatWithDifferentMessages(String uid, String message) throws JAXBException {
            String xml = CoTTestFixtures.createChatXml(uid, message);
            
            TestUtils.assertRoundTripConversion(xml, uid, CoTTestFixtures.CoTTypes.CHAT);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            TestUtils.assertChatDocument(document, uid, message, "ALPHA-1");
        }
    }
    
    @Nested
    @DisplayName("File Document Tests")
    class FileTests {
        
        @ParameterizedTest
        @CsvSource({
            "FILE-001, document.pdf, 1048576",
            "FILE-002, image.jpg, 2097152",
            "FILE-003, video.mp4, 104857600",
            "FILE-004, data.zip, 5242880"
        })
        @DisplayName("Should convert File shares using shared fixtures")
        void testFileShareWithDifferentFiles(String uid, String filename, long sizeInBytes) throws JAXBException {
            String xml = CoTTestFixtures.createFileShareXml(uid, filename, sizeInBytes);
            
            TestUtils.assertRoundTripConversion(xml, uid, CoTTestFixtures.CoTTypes.FILE_SHARE);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            TestUtils.assertFileDocument(document, uid, filename, (double) sizeInBytes);
        }
    }
    
    @Nested
    @DisplayName("API Document Tests")
    class ApiTests {
        
        @ParameterizedTest
        @CsvSource({
            "API-001, /api/status, GET",
            "API-002, /api/users, POST",
            "API-003, /api/data, PUT",
            "API-004, /api/logs, DELETE"
        })
        @DisplayName("Should convert API requests using shared fixtures")
        void testApiWithDifferentEndpoints(String uid, String endpoint, String method) throws JAXBException {
            String xml = CoTTestFixtures.createApiXml(uid, endpoint, method);
            
            TestUtils.assertRoundTripConversion(xml, uid, CoTTestFixtures.CoTTypes.API);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            TestUtils.assertApiDocument(document, uid, endpoint, method);
        }
    }
    
    @Nested
    @DisplayName("Generic Document Tests")
    class GenericTests {
        
        @ParameterizedTest
        @ValueSource(strings = {
            "x-custom-type",
            "a-u-emergency-g",
            "unknown-type"
        })
        @DisplayName("Should convert Generic events using shared fixtures")
        void testGenericWithDifferentTypes(String type) throws JAXBException {
            String uid = CoTTestFixtures.generateUID("GENERIC");
            String xml = CoTTestFixtures.createGenericXml(uid, type);
            
            TestUtils.assertRoundTripConversion(xml, uid, type);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            TestUtils.assertGenericDocument(document, uid, type);
        }
    }
    
    @Nested
    @DisplayName("Performance Tests")
    class PerformanceTests {
        
        @Test
        @DisplayName("Should convert documents within performance limits")
        void testConversionPerformance() {
            String xml = CoTTestFixtures.createMapItemXml("PERF-TEST-001");
            
            TestUtils.assertPerformance(
                () -> {
                    try {
                        for (int i = 0; i < 100; i++) {
                            CoTEvent event = converter.parseCoTXml(xml);
                            converter.convertCoTEventToDocument(event);
                        }
                    } catch (JAXBException e) {
                        org.junit.jupiter.api.Assertions.fail("Performance test failed: " + e.getMessage());
                    }
                },
                1000, // 1 second for 100 conversions
                "100 MapItem conversions"
            );
        }
        
        @Test
        @DisplayName("Should handle concurrent access safely")
        void testConcurrentAccess() {
            String xml = CoTTestFixtures.createChatXml("CONCURRENT-001", "Test message");
            
            TestUtils.assertConcurrentSafety(xml, 10, 50); // 10 threads, 50 iterations each
        }
    }
    
    @Nested
    @DisplayName("Coordinate Validation Tests")
    class CoordinateTests {
        
        @Test
        @DisplayName("Should validate standard coordinates")
        void testStandardCoordinates() {
            TestUtils.assertValidCoordinates(
                CoTTestFixtures.STANDARD_LAT,
                CoTTestFixtures.STANDARD_LON,
                CoTTestFixtures.STANDARD_HAE
            );
        }
        
        @Test
        @DisplayName("Should validate alternative coordinates")
        void testAlternativeCoordinates() {
            TestUtils.assertValidCoordinates(
                CoTTestFixtures.ALT_LAT,
                CoTTestFixtures.ALT_LON,
                CoTTestFixtures.ALT_HAE
            );
        }
        
        @Test
        @DisplayName("Should reject invalid coordinates")
        void testInvalidCoordinates() {
            assertThatThrownBy(() -> TestUtils.assertValidCoordinates(91.0, 0.0, 0.0))
                .isInstanceOf(AssertionError.class)
                .hasMessageContaining("Latitude should be between -90 and 90");
            
            assertThatThrownBy(() -> TestUtils.assertValidCoordinates(0.0, 181.0, 0.0))
                .isInstanceOf(AssertionError.class)
                .hasMessageContaining("Longitude should be between -180 and 180");
        }
    }
    
    @Nested
    @DisplayName("Timestamp Tests")
    class TimestampTests {
        
        @Test
        @DisplayName("Should validate timestamp conversion to microseconds")
        void testTimestampConversion() throws JAXBException {
            String xml = CoTTestFixtures.createMapItemXml("TIMESTAMP-TEST");
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertTrue(document instanceof MapItemDocument);
            MapItemDocument mapItem = (MapItemDocument) document;
            
            TestUtils.assertValidTimestamp(mapItem.getQ()); // start time
            // Note: getR() returns Map<String, Object>, not timestamp for MapItemDocument
        }
        
        @Test
        @DisplayName("Should handle future and past timestamps")
        void testRelativeTimestamps() {
            String futureTime = CoTTestFixtures.createFutureTimestamp(3600); // 1 hour future
            String pastTime = CoTTestFixtures.createPastTimestamp(3600); // 1 hour past
            
            assertNotNull(futureTime);
            assertNotNull(pastTime);
            assertNotEquals(futureTime, pastTime);
            assertNotEquals(futureTime, CoTTestFixtures.STANDARD_TIME);
            assertNotEquals(pastTime, CoTTestFixtures.STANDARD_TIME);
        }
    }
    
    @Nested
    @DisplayName("Expected Document Structure Tests")
    class ExpectedDocumentTests {
        
        @Test
        @DisplayName("Should match expected MapItem document structure")
        void testExpectedMapItemStructure() throws JAXBException {
            String uid = "EXPECTED-MAP-001";
            String xml = CoTTestFixtures.createMapItemXml(uid);
            Map<String, Object> expected = CoTTestFixtures.createExpectedMapItemDocument(uid);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertTrue(document instanceof MapItemDocument);
            MapItemDocument mapItem = (MapItemDocument) document;
            
            // Verify key fields match expected structure
            assertEquals(expected.get("_id"), mapItem.getId());
            assertEquals(expected.get("w"), mapItem.getW());
            assertEquals(expected.get("h"), mapItem.getH());  // h = CE
            assertEquals(expected.get("j"), mapItem.getJ());  // j = LAT  
            assertEquals(expected.get("l"), mapItem.getL());  // l = LON
        }
        
        @Test
        @DisplayName("Should match expected Chat document structure")
        void testExpectedChatStructure() throws JAXBException {
            String uid = "EXPECTED-CHAT-001";
            String message = "Expected test message";
            String xml = CoTTestFixtures.createChatXml(uid, message);
            Map<String, Object> expected = CoTTestFixtures.createExpectedChatDocument(uid, message);
            
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            
            assertTrue(document instanceof ChatDocument);
            ChatDocument chat = (ChatDocument) document;
            
            // Verify key fields match expected structure
            assertEquals(expected.get("_id"), chat.getId());
            assertEquals(expected.get("message"), chat.getMessage());  // message field
            assertEquals(expected.get("e"), chat.getE());              // e = author callsign
            assertEquals(expected.get("room"), chat.getRoom());        // room field
        }
    }
    
    @Test
    @DisplayName("Should generate unique UIDs consistently")
    void testUIDGeneration() {
        String uid1 = CoTTestFixtures.generateUID("TEST");
        String uid2 = CoTTestFixtures.generateUID("TEST");
        String uid3 = CoTTestFixtures.generateUID("DIFFERENT");
        
        // All UIDs should be unique
        assertNotEquals(uid1, uid2);
        assertNotEquals(uid1, uid3);
        assertNotEquals(uid2, uid3);
        
        // UIDs should start with the specified prefix
        assertTrue(uid1.startsWith("TEST-"));
        assertTrue(uid2.startsWith("TEST-"));
        assertTrue(uid3.startsWith("DIFFERENT-"));
    }
    
    @Test
    @DisplayName("Should provide consistent test data across fixture methods")
    void testFixtureConsistency() {
        // All fixture methods should use the same standard coordinates
        String mapXml = CoTTestFixtures.createMapItemXml("CONSISTENCY-MAP");
        String chatXml = CoTTestFixtures.createChatXml("CONSISTENCY-CHAT", "message");
        String fileXml = CoTTestFixtures.createFileShareXml("CONSISTENCY-FILE", "file.txt", 1024);
        
        // All should contain the same coordinate values
        assertTrue(mapXml.contains(String.valueOf(CoTTestFixtures.STANDARD_LAT)));
        assertTrue(chatXml.contains(String.valueOf(CoTTestFixtures.STANDARD_LAT)));
        assertTrue(fileXml.contains(String.valueOf(CoTTestFixtures.STANDARD_LAT)));
        
        // All should contain the same timestamp values
        assertTrue(mapXml.contains(CoTTestFixtures.STANDARD_TIME));
        assertTrue(chatXml.contains(CoTTestFixtures.STANDARD_TIME));
        assertTrue(fileXml.contains(CoTTestFixtures.STANDARD_TIME));
    }
}