package com.ditto.cot;

import com.ditto.cot.schema.*;
import jakarta.xml.bind.JAXBException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import org.junit.jupiter.params.provider.NullSource;
import org.junit.jupiter.params.provider.EmptySource;
import org.junit.jupiter.params.provider.MethodSource;

import java.time.Instant;
import java.time.format.DateTimeParseException;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Stream;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Comprehensive error handling tests for the CoT library.
 * Tests various error conditions and edge cases.
 */
@DisplayName("CoT Library Error Handling Tests")
public class ErrorHandlingTest {
    
    private CoTConverter converter;
    
    @BeforeEach
    void setUp() throws JAXBException {
        converter = new CoTConverter();
    }
    
    @Nested
    @DisplayName("XML Parsing Error Tests")
    class XmlParsingErrors {
        
        @Test
        @DisplayName("Should throw exception for null XML")
        void testNullXml() {
            assertThatThrownBy(() -> converter.parseCoTXml(null))
                    .isInstanceOf(IllegalArgumentException.class)
                    .hasMessageContaining("XML content cannot be null");
        }
        
        @Test
        @DisplayName("Should throw exception for empty XML")
        void testEmptyXml() {
            assertThatThrownBy(() -> converter.parseCoTXml(""))
                    .isInstanceOf(JAXBException.class);
        }
        
        @Test
        @DisplayName("Should throw exception for malformed XML")
        void testMalformedXml() {
            String malformedXml = "<event><point lat='37.7749' lon='-122.4194'></event>";
            assertThatThrownBy(() -> converter.parseCoTXml(malformedXml))
                    .isInstanceOf(JAXBException.class);
        }
        
        @Test
        @DisplayName("Should throw exception for XML with missing required attributes")
        void testMissingRequiredAttributes() {
            String xmlMissingUid = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' type='a-f-G-U-C' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                </event>
                """;
            
            assertThatThrownBy(() -> converter.parseCoTXml(xmlMissingUid))
                    .isInstanceOf(JAXBException.class);
        }
        
        @ParameterizedTest
        @ValueSource(strings = {
            "<event>not valid xml",
            "<?xml version='1.0'?><event><unclosed>",
            "<event>&invalid_entity;</event>",
            "plain text, not xml"
        })
        @DisplayName("Should throw exception for various invalid XML formats")
        void testInvalidXmlFormats(String invalidXml) {
            assertThatThrownBy(() -> converter.parseCoTXml(invalidXml))
                    .isInstanceOf(JAXBException.class);
        }
        
        @Test
        @DisplayName("Should handle XML with invalid coordinate values")
        void testInvalidCoordinates() {
            String xmlWithInvalidCoords = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='invalid' lon='not-a-number' hae='100.5' ce='10.0' le='5.0'/>
                </event>
                """;
            
            assertThatThrownBy(() -> converter.parseCoTXml(xmlWithInvalidCoords))
                    .isInstanceOf(JAXBException.class);
        }
    }
    
    @Nested
    @DisplayName("Date/Time Parsing Error Tests")
    class DateTimeParsingErrors {
        
        @Test
        @DisplayName("Should handle invalid time format")
        void testInvalidTimeFormat() {
            String xmlWithInvalidTime = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                       time='not-a-date' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                </event>
                """;
            
            assertThatThrownBy(() -> converter.parseCoTXml(xmlWithInvalidTime))
                    .isInstanceOf(JAXBException.class);
        }
        
        @ParameterizedTest
        @ValueSource(strings = {
            "2024-13-01T10:30:00.000Z",  // Invalid month
            "2024-01-32T10:30:00.000Z",  // Invalid day
            "2024-01-15T25:30:00.000Z",  // Invalid hour
            "2024/01/15 10:30:00",       // Wrong format
            "January 15, 2024"           // Human readable format
        })
        @DisplayName("Should reject various invalid date formats")
        void testInvalidDateFormats(String invalidDate) {
            String xml = String.format("""
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                       time='%s' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                </event>
                """, invalidDate);
            
            assertThatThrownBy(() -> converter.parseCoTXml(xml))
                    .isInstanceOf(JAXBException.class);
        }
    }
    
    @Nested
    @DisplayName("Null and Empty Value Handling")
    class NullAndEmptyHandling {
        
        @Test
        @DisplayName("Should handle null CoTEvent")
        void testNullCoTEvent() {
            assertThatThrownBy(() -> converter.convertCoTEventToDocument(null))
                    .isInstanceOf(NullPointerException.class);
        }
        
        @Test
        @DisplayName("Should handle CoTEvent with null fields")
        void testCoTEventWithNullFields() {
            CoTEvent event = new CoTEvent();
            // Leave all fields null
            
            assertThatThrownBy(() -> converter.convertCoTEventToDocument(event))
                    .isInstanceOf(NullPointerException.class);
        }
        
        @Test
        @DisplayName("Should handle empty detail section")
        void testEmptyDetailSection() throws JAXBException {
            String xmlWithEmptyDetail = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail></detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(xmlWithEmptyDetail);
            assertNotNull(event);
            
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
        }
        
        @ParameterizedTest
        @NullSource
        @EmptySource
        @ValueSource(strings = {"   ", "\t", "\n"})
        @DisplayName("Should handle various empty/whitespace UIDs")
        void testEmptyOrWhitespaceUid(String uid) {
            CoTEvent event = createValidCoTEvent();
            event.setUid(uid);
            
            if (uid == null) {
                assertThatThrownBy(() -> converter.convertCoTEventToDocument(event))
                        .isInstanceOf(NullPointerException.class);
            } else {
                // Should handle empty/whitespace UIDs gracefully
                Object doc = converter.convertCoTEventToDocument(event);
                assertNotNull(doc);
            }
        }
    }
    
    @Nested
    @DisplayName("Document Type Error Handling")
    class DocumentTypeErrors {
        
        @Test
        @DisplayName("Should handle unknown CoT type")
        void testUnknownCoTType() throws JAXBException {
            String xmlWithUnknownType = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='x-unknown-type' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(xmlWithUnknownType);
            Object document = converter.convertCoTEventToDocument(event);
            
            // Should fallback to Generic document
            assertNotNull(document);
            assertTrue(document instanceof GenericDocument);
        }
        
        @Test
        @DisplayName("Should handle malformed CoT type")
        void testMalformedCoTType() {
            CoTEvent event = createValidCoTEvent();
            event.setType("!@#$%^&*()");
            
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
            assertTrue(document instanceof GenericDocument);
        }
    }
    
    @Nested
    @DisplayName("Numeric Value Error Handling")
    class NumericValueErrors {
        
        @Test
        @DisplayName("Should handle NaN and Infinity values")
        void testNaNAndInfinityValues() {
            CoTEvent event = createValidCoTEvent();
            CoTPoint point = new CoTPoint();
            point.setLat(String.valueOf(Double.NaN));
            point.setLon(String.valueOf(Double.POSITIVE_INFINITY));
            point.setHae(String.valueOf(Double.NEGATIVE_INFINITY));
            event.setPoint(point);
            
            // Should handle gracefully - converter might replace with defaults
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
        }
        
        @Test
        @DisplayName("Should validate coordinate ranges")
        void testCoordinateRangeValidation() {
            CoTEvent event = createValidCoTEvent();
            CoTPoint point = new CoTPoint();
            point.setLat("200.0");  // Invalid latitude (should be -90 to 90)
            point.setLon("400.0");  // Invalid longitude (should be -180 to 180)
            point.setHae("-50000.0");  // Extreme altitude
            event.setPoint(point);
            
            // Converter should handle invalid coordinates
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
        }
    }
    
    @Nested
    @DisplayName("Detail Parsing Error Tests")
    class DetailParsingErrors {
        
        @Test
        @DisplayName("Should handle malformed detail XML")
        void testMalformedDetailXml() {
            CoTEvent event = createValidCoTEvent();
            CoTDetail detail = new CoTDetail();
            detail.setContent(new Object[]{"<invalid>unclosed tag"});
            event.setDetail(detail);
            
            // Should handle gracefully
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
        }
        
        @Test
        @DisplayName("Should handle circular references in detail")
        void testCircularReferencesInDetail() {
            // This would require more complex setup with actual circular refs
            // For now, test deeply nested structure
            String deeplyNestedXml = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <level1>
                            <level2>
                                <level3>
                                    <level4>
                                        <level5>
                                            <level6>
                                                <level7>
                                                    <level8>
                                                        <level9>
                                                            <level10>Deep nesting test</level10>
                                                        </level9>
                                                    </level8>
                                                </level7>
                                            </level6>
                                        </level5>
                                    </level4>
                                </level3>
                            </level2>
                        </level1>
                    </detail>
                </event>
                """;
            
            assertDoesNotThrow(() -> {
                CoTEvent event = converter.parseCoTXml(deeplyNestedXml);
                converter.convertCoTEventToDocument(event);
            });
        }
    }
    
    @Nested
    @DisplayName("JSON Conversion Error Tests")
    class JsonConversionErrors {
        
        @Test
        @DisplayName("Should handle invalid JSON in Ditto document")
        void testInvalidJsonInDittoDocument() {
            Map<String, Object> invalidDoc = new HashMap<>();
            invalidDoc.put("_id", "TEST-123");
            invalidDoc.put("w", "a-f-G-U-C");
            invalidDoc.put("circular", invalidDoc); // Circular reference
            
            // This would typically cause issues during JSON serialization
            assertThatCode(() -> {
                // Simulate what would happen in real usage
                new com.fasterxml.jackson.databind.ObjectMapper()
                        .writeValueAsString(invalidDoc);
            }).isInstanceOf(com.fasterxml.jackson.databind.JsonMappingException.class);
        }
        
        @Test
        @DisplayName("Should handle special characters in strings")
        void testSpecialCharactersInStrings() throws JAXBException {
            String xmlWithSpecialChars = """
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    <detail>
                        <contact callsign='Special"Characters&lt;&gt;&amp;' endpoint='192.168.1.100'/>
                        <remarks>Unicode: 你好 العربية 🚀</remarks>
                    </detail>
                </event>
                """;
            
            CoTEvent event = converter.parseCoTXml(xmlWithSpecialChars);
            Object document = converter.convertCoTEventToDocument(event);
            assertNotNull(document);
        }
    }
    
    @Nested
    @DisplayName("Resource and Memory Error Tests")
    class ResourceErrors {
        
        @Test
        @DisplayName("Should handle very large XML documents")
        void testVeryLargeXmlDocument() {
            // Create a large detail section
            StringBuilder largeDetail = new StringBuilder("<detail>");
            for (int i = 0; i < 1000; i++) {
                largeDetail.append(String.format("<item%d>Value %d</item%d>", i, i, i));
            }
            largeDetail.append("</detail>");
            
            String largeXml = String.format("""
                <?xml version='1.0' encoding='UTF-8'?>
                <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                       time='2024-01-15T10:30:00.000Z' 
                       start='2024-01-15T10:30:00.000Z' 
                       stale='2024-01-15T11:00:00.000Z' 
                       how='h-g-i-g-o'>
                    <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                    %s
                </event>
                """, largeDetail.toString());
            
            assertDoesNotThrow(() -> {
                CoTEvent event = converter.parseCoTXml(largeXml);
                converter.convertCoTEventToDocument(event);
            });
        }
        
        @Test
        @DisplayName("Should handle concurrent conversions")
        void testConcurrentConversions() {
            String xml = createValidXml();
            
            assertDoesNotThrow(() -> {
                Stream.generate(() -> xml)
                        .limit(100)
                        .parallel()
                        .forEach(xmlContent -> {
                            try {
                                CoTEvent event = converter.parseCoTXml(xmlContent);
                                converter.convertCoTEventToDocument(event);
                            } catch (JAXBException e) {
                                org.junit.jupiter.api.Assertions.fail("Concurrent conversion failed: " + e.getMessage());
                            }
                        });
            });
        }
    }
    
    // Helper methods
    
    private CoTEvent createValidCoTEvent() {
        CoTEvent event = new CoTEvent();
        event.setVersion("2.0");
        event.setUid("TEST-" + System.currentTimeMillis());
        event.setType("a-f-G-U-C");
        event.setTime(Instant.now().toString());
        event.setStart(Instant.now().toString());
        event.setStale(Instant.now().plusSeconds(1800).toString());
        event.setHow("h-g-i-g-o");
        
        CoTPoint point = new CoTPoint();
        point.setLat("37.7749");
        point.setLon("-122.4194");
        point.setHae("100.5");
        point.setCe("10.0");
        point.setLe("5.0");
        event.setPoint(point);
        
        return event;
    }
    
    private String createValidXml() {
        return """
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='TEST-123' type='a-f-G-U-C' 
                   time='2024-01-15T10:30:00.000Z' 
                   start='2024-01-15T10:30:00.000Z' 
                   stale='2024-01-15T11:00:00.000Z' 
                   how='h-g-i-g-o'>
                <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                <detail>
                    <contact callsign='ALPHA-1'/>
                </detail>
            </event>
            """;
    }
    
    private static Stream<String> invalidXmlProvider() {
        return Stream.of(
            null,
            "",
            "   ",
            "not xml",
            "<invalid/>",
            "<?xml version='1.0'?><root>",
            "<event><point></event>"
        );
    }
}