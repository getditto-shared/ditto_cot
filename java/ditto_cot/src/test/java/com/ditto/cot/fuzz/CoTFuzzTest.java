package com.ditto.cot.fuzz;

import com.ditto.cot.CoTConverter;
import com.ditto.cot.CoTEvent;
import jakarta.xml.bind.JAXBException;
import net.jqwik.api.*;
import net.jqwik.api.constraints.*;
import net.jqwik.api.lifecycle.BeforeTry;

import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.List;
import java.util.Random;

import static org.assertj.core.api.Assertions.*;

/**
 * Property-based fuzz testing for CoT library using jqwik.
 * Tests various edge cases and random inputs to find potential bugs.
 */
public class CoTFuzzTest {
    
    private CoTConverter converter;
    
    @BeforeTry
    void setUp() throws JAXBException {
        converter = new CoTConverter();
    }
    
    @Property
    void fuzzValidCoTXmlShouldAlwaysParse(
            @ForAll @AlphaChars @StringLength(min = 1, max = 50) String uid,
            @ForAll @From("validCoTTypes") String type,
            @ForAll @DoubleRange(min = -90.0, max = 90.0) double lat,
            @ForAll @DoubleRange(min = -180.0, max = 180.0) double lon,
            @ForAll @DoubleRange(min = -10000.0, max = 10000.0) double hae,
            @ForAll @DoubleRange(min = 0.0, max = 1000.0) double ce,
            @ForAll @DoubleRange(min = 0.0, max = 1000.0) double le
    ) {
        String xml = createValidXml(uid, type, lat, lon, hae, ce, le);
        
        assertThatCode(() -> {
            CoTEvent event = converter.parseCoTXml(xml);
            Object document = converter.convertCoTEventToDocument(event);
            assertThat(event).isNotNull();
            assertThat(document).isNotNull();
        }).doesNotThrowAnyException();
    }
    
    @Property
    void fuzzCoordinateEdgeCases(
            @ForAll @AlphaChars @StringLength(min = 1, max = 20) String uid,
            @ForAll @DoubleRange(min = -1000.0, max = 1000.0) double lat,
            @ForAll @DoubleRange(min = -1000.0, max = 1000.0) double lon
    ) {
        // Test with coordinates outside normal ranges
        String xml = createValidXml(uid, "a-f-G-U-C", lat, lon, 0.0, 10.0, 5.0);
        
        assertThatCode(() -> {
            CoTEvent event = converter.parseCoTXmlSafely(xml);
            if (event != null) {
                converter.convertCoTEventToDocument(event);
            }
        }).doesNotThrowAnyException();
    }
    
    @Property
    void fuzzSpecialDoubleValues(
            @ForAll @AlphaChars @StringLength(min = 1, max = 20) String uid,
            @ForAll @From("specialDoubles") double specialValue
    ) {
        String xml = createValidXml(uid, "a-f-G-U-C", specialValue, specialValue, specialValue, 10.0, 5.0);
        
        assertThatCode(() -> {
            CoTEvent event = converter.parseCoTXmlSafely(xml);
            if (event != null) {
                converter.convertCoTEventToDocument(event);
            }
        }).doesNotThrowAnyException();
    }
    
    @Property
    void fuzzLargeStrings(
            @ForAll @StringLength(min = 1000, max = 10000) String largeString
    ) {
        // Create XML with very large string values
        String safeString = largeString.replaceAll("[<>&\"']", "_");
        String uid = "FUZZ-" + safeString.substring(0, Math.min(50, safeString.length()));
        
        String xml = String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='a-f-G-U-C' 
                   time='2024-01-15T10:30:00.000Z' 
                   start='2024-01-15T10:30:00.000Z' 
                   stale='2024-01-15T11:30:00.000Z' 
                   how='h-g-i-g-o'>
                <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                <detail>
                    <contact callsign='%s'/>
                    <remarks>%s</remarks>
                </detail>
            </event>
            """, uid, safeString.substring(0, Math.min(100, safeString.length())), 
                    safeString.substring(0, Math.min(500, safeString.length())));
        
        assertThatCode(() -> {
            CoTEvent event = converter.parseCoTXmlSafely(xml);
            if (event != null) {
                converter.convertCoTEventToDocument(event);
            }
        }).doesNotThrowAnyException();
    }
    
    @Property
    void fuzzRandomXmlStructures(
            @ForAll @AlphaChars @StringLength(min = 1, max = 20) String uid,
            @ForAll @From("randomDetailElements") List<String> detailElements
    ) {
        StringBuilder detailBuilder = new StringBuilder("<detail>");
        for (String element : detailElements) {
            detailBuilder.append(element);
        }
        detailBuilder.append("</detail>");
        
        String xml = String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='a-f-G-U-C' 
                   time='2024-01-15T10:30:00.000Z' 
                   start='2024-01-15T10:30:00.000Z' 
                   stale='2024-01-15T11:30:00.000Z' 
                   how='h-g-i-g-o'>
                <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                %s
            </event>
            """, uid, detailBuilder.toString());
        
        assertThatCode(() -> {
            CoTEvent event = converter.parseCoTXmlSafely(xml);
            if (event != null) {
                converter.convertCoTEventToDocument(event);
            }
        }).doesNotThrowAnyException();
    }
    
    @Property
    void fuzzUnicodeStrings(
            @ForAll @StringLength(min = 1, max = 100) String unicodeString
    ) {
        // Test with various Unicode characters
        String safeUid = unicodeString.replaceAll("[^\\p{IsAlphabetic}\\p{IsDigit}]", "_")
                                     .substring(0, Math.min(50, unicodeString.length()));
        if (safeUid.isEmpty()) {
            safeUid = "UNICODE-TEST";
        }
        
        String xml = String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='a-f-G-U-C' 
                   time='2024-01-15T10:30:00.000Z' 
                   start='2024-01-15T10:30:00.000Z' 
                   stale='2024-01-15T11:30:00.000Z' 
                   how='h-g-i-g-o'>
                <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                <detail>
                    <remarks>%s</remarks>
                </detail>
            </event>
            """, safeUid, escapeXml(unicodeString));
        
        assertThatCode(() -> {
            CoTEvent event = converter.parseCoTXmlSafely(xml);
            if (event != null) {
                converter.convertCoTEventToDocument(event);
            }
        }).doesNotThrowAnyException();
    }
    
    @Property
    void fuzzTimestampFormats(
            @ForAll @From("timestampStrings") String timestamp,
            @ForAll @AlphaChars @StringLength(min = 1, max = 20) String uid
    ) {
        String xml = String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='a-f-G-U-C' 
                   time='%s' 
                   start='%s' 
                   stale='%s' 
                   how='h-g-i-g-o'>
                <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                <detail/>
            </event>
            """, uid, timestamp, timestamp, timestamp);
        
        // Some timestamp formats may be invalid, but should not crash
        try {
            CoTEvent event = converter.parseCoTXml(xml);
            converter.convertCoTEventToDocument(event);
        } catch (JAXBException e) {
            // Expected for invalid timestamps
        } catch (Exception e) {
            // Unexpected exceptions should not occur
            fail("Unexpected exception: " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }
    
    @Property
    void fuzzDeepNesting(
            @ForAll @IntRange(min = 1, max = 20) int nestingDepth,
            @ForAll @AlphaChars @StringLength(min = 1, max = 10) String tagName
    ) {
        StringBuilder nestedXml = new StringBuilder();
        
        // Create deeply nested structure
        for (int i = 0; i < nestingDepth; i++) {
            nestedXml.append("<").append(tagName).append(i).append(">");
        }
        nestedXml.append("deep content");
        for (int i = nestingDepth - 1; i >= 0; i--) {
            nestedXml.append("</").append(tagName).append(i).append(">");
        }
        
        String xml = String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='DEEP-NEST-%d' type='a-f-G-U-C' 
                   time='2024-01-15T10:30:00.000Z' 
                   start='2024-01-15T10:30:00.000Z' 
                   stale='2024-01-15T11:30:00.000Z' 
                   how='h-g-i-g-o'>
                <point lat='37.7749' lon='-122.4194' hae='100.5' ce='10.0' le='5.0'/>
                <detail>%s</detail>
            </event>
            """, nestingDepth, nestedXml.toString());
        
        assertThatCode(() -> {
            CoTEvent event = converter.parseCoTXmlSafely(xml);
            if (event != null) {
                converter.convertCoTEventToDocument(event);
            }
        }).doesNotThrowAnyException();
    }
    
    @Property
    void fuzzMaliciousXmlPatterns(
            @ForAll @From("maliciousPatterns") String pattern
    ) {
        // Test known malicious XML patterns (XXE, billion laughs, etc.)
        // These should be handled safely without causing security issues
        try {
            converter.parseCoTXml(pattern);
        } catch (Exception e) {
            // Expected - malicious patterns should be rejected
            assertThat(e).isInstanceOf(JAXBException.class);
        }
    }
    
    @Property
    void fuzzConcurrentAccess(
            @ForAll @AlphaChars @StringLength(min = 1, max = 20) String uid1,
            @ForAll @AlphaChars @StringLength(min = 1, max = 20) String uid2
    ) {
        String xml1 = createValidXml(uid1, "a-f-G-U-C", 37.7749, -122.4194, 100.5, 10.0, 5.0);
        String xml2 = createValidXml(uid2, "b-t-f", 40.7128, -74.0060, 50.0, 15.0, 8.0);
        
        // Test concurrent parsing
        assertThatCode(() -> {
            Thread t1 = new Thread(() -> {
                try {
                    for (int i = 0; i < 10; i++) {
                        CoTEvent event = converter.parseCoTXml(xml1);
                        converter.convertCoTEventToDocument(event);
                    }
                } catch (JAXBException e) {
                    throw new RuntimeException(e);
                }
            });
            
            Thread t2 = new Thread(() -> {
                try {
                    for (int i = 0; i < 10; i++) {
                        CoTEvent event = converter.parseCoTXml(xml2);
                        converter.convertCoTEventToDocument(event);
                    }
                } catch (JAXBException e) {
                    throw new RuntimeException(e);
                }
            });
            
            t1.start();
            t2.start();
            t1.join();
            t2.join();
        }).doesNotThrowAnyException();
    }
    
    // Generators for test data
    
    @Provide
    Arbitrary<String> validCoTTypes() {
        return Arbitraries.of(
            "a-f-G-U-C",   // MapItem
            "a-u-r-loc-g", // Location
            "b-t-f",       // Chat
            "b-f-t-f",     // File
            "a-u-emergency-g", // Emergency
            "a-u-S",       // Sensor
            "a-u-A",       // Aircraft
            "a-u-G",       // Ground
            "x-custom-type" // Generic
        );
    }
    
    @Provide
    Arbitrary<Double> specialDoubles() {
        return Arbitraries.of(
            Double.NaN,
            Double.POSITIVE_INFINITY,
            Double.NEGATIVE_INFINITY,
            Double.MAX_VALUE,
            Double.MIN_VALUE,
            0.0,
            -0.0,
            1.0,
            -1.0
        );
    }
    
    @Provide
    Arbitrary<List<String>> randomDetailElements() {
        return Arbitraries.of(
            "<contact callsign='ALPHA-1'/>",
            "<__group name='Team Alpha'/>",
            "<takv os='31' version='5.4.0'/>",
            "<status battery='85'/>",
            "<track course='270.5' speed='15.2'/>",
            "<remarks>Test message</remarks>",
            "<custom attr='value'>content</custom>",
            "<empty/>",
            "<nested><inner>value</inner></nested>"
        ).list().ofMinSize(0).ofMaxSize(5);
    }
    
    @Provide
    Arbitrary<String> timestampStrings() {
        return Arbitraries.of(
            "2024-01-15T10:30:00.000Z",        // Valid ISO format
            "2024-01-15T10:30:00Z",            // Valid without milliseconds
            "2024-13-45T25:99:99.999Z",        // Invalid date/time
            "not-a-timestamp",                 // Invalid format
            "2024/01/15 10:30:00",             // Different format
            "",                                // Empty
            "1970-01-01T00:00:00.000Z",        // Epoch
            "2038-01-19T03:14:07.000Z",        // Unix timestamp limit
            "9999-12-31T23:59:59.999Z"         // Far future
        );
    }
    
    @Provide
    Arbitrary<String> maliciousPatterns() {
        return Arbitraries.of(
            // XXE injection attempts
            "<?xml version='1.0'?><!DOCTYPE foo [<!ENTITY xxe SYSTEM 'file:///etc/passwd'>]><event>&xxe;</event>",
            
            // Billion laughs attack
            "<?xml version='1.0'?><!DOCTYPE lolz [<!ENTITY lol 'lol'><!ENTITY lol2 '&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;'>]><event>&lol2;</event>",
            
            // Malformed XML
            "<event><unclosed>",
            "<event attr='unclosed>content</event>",
            
            // Script injection attempts
            "<event><script>alert('xss')</script></event>",
            "<event onclick='alert(1)'>content</event>",
            
            // Very large content
            "<event>" + "A".repeat(1000000) + "</event>",
            
            // Null bytes and control characters
            "<event>\u0000\u0001\u0002</event>",
            
            // Empty and minimal cases
            "",
            "<>",
            "</>",
            "<event/>"
        );
    }
    
    // Helper methods
    
    private String createValidXml(String uid, String type, double lat, double lon, double hae, double ce, double le) {
        return String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='%s' 
                   time='2024-01-15T10:30:00.000Z' 
                   start='2024-01-15T10:30:00.000Z' 
                   stale='2024-01-15T11:30:00.000Z' 
                   how='h-g-i-g-o'>
                <point lat='%f' lon='%f' hae='%f' ce='%f' le='%f'/>
                <detail>
                    <contact callsign='FUZZ-TEST'/>
                </detail>
            </event>
            """, uid, type, lat, lon, hae, ce, le);
    }
    
    private String escapeXml(String input) {
        return input.replace("&", "&amp;")
                   .replace("<", "&lt;")
                   .replace(">", "&gt;")
                   .replace("\"", "&quot;")
                   .replace("'", "&apos;");
    }
}