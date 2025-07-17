package com.ditto.cot.example;

import com.ditto.cot.CoTConverter;
import com.ditto.cot.CoTEvent;
import com.ditto.cot.schema.DittoDocument;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.BeforeEach;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * Tests for the SimpleExample to ensure the example code works correctly
 */
class SimpleExampleTest {

    private CoTConverter converter;
    private String sampleXml;

    @BeforeEach
    void setUp() throws Exception {
        converter = new CoTConverter();
        sampleXml = """
            <?xml version="1.0" encoding="UTF-8"?>
            <event version="2.0" uid="TEST-001" type="a-f-G-U-C" 
                   time="2023-01-01T12:00:00.000Z" 
                   start="2023-01-01T12:00:00.000Z" 
                   stale="2023-01-01T12:05:00.000Z" 
                   how="h-g-i-gdo">
                <point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="25.0"/>
                <detail>
                    <contact callsign="TEST-UNIT"/>
                    <group name="BLUE"/>
                    <platform original_type="a-f-G-U-C"/>
                </detail>
            </event>
            """;
    }

    @Test
    void testParseCoTXml() throws Exception {
        // When
        CoTEvent event = converter.parseCoTXml(sampleXml);
        
        // Then
        assertThat(event).isNotNull();
        assertThat(event.getUid()).isEqualTo("TEST-001");
        assertThat(event.getType()).isEqualTo("a-f-G-U-C");
        assertThat(event.getVersion()).isEqualTo("2.0");
        assertThat(event.getTime()).isEqualTo("2023-01-01T12:00:00.000Z");
        assertThat(event.getStart()).isEqualTo("2023-01-01T12:00:00.000Z");
        assertThat(event.getStale()).isEqualTo("2023-01-01T12:05:00.000Z");
        assertThat(event.getHow()).isEqualTo("h-g-i-gdo");
    }

    @Test
    void testPointParsing() throws Exception {
        // When
        CoTEvent event = converter.parseCoTXml(sampleXml);
        
        // Then
        assertThat(event.getPoint()).isNotNull();
        assertThat(event.getPointLatitude()).isEqualTo("34.12345");
        assertThat(event.getPointLongitude()).isEqualTo("-118.12345");
        assertThat(event.getPointHae()).isEqualTo("150.0");
        assertThat(event.getPointCe()).isEqualTo("10.0");
        assertThat(event.getPointLe()).isEqualTo("25.0");
    }

    @Test
    void testDetailParsing() throws Exception {
        // When
        CoTEvent event = converter.parseCoTXml(sampleXml);
        
        // Then
        assertThat(event.getDetail()).isNotNull();
        assertThat(event.getDetailMap()).isNotEmpty();
    }

    @Test
    void testConvertToDocument() throws Exception {
        // When
        Object document = converter.convertToDocument(sampleXml);
        
        // Then
        assertThat(document).isNotNull();
        assertThat(document).isInstanceOf(DittoDocument.class);
    }

    @Test
    void testJsonSerialization() throws Exception {
        // Given
        Object document = converter.convertToDocument(sampleXml);
        
        // When
        String json = ((DittoDocument) document).toJson();
        
        // Then
        assertThat(json).isNotNull();
        assertThat(json).isNotEmpty();
        assertThat(json).contains("\"_id\":\"TEST-001\"");
        assertThat(json).contains("\"w\":\"a-f-G-U-C\"");
    }

    @Test
    void testJsonRoundTrip() throws Exception {
        // Given
        Object originalDocument = converter.convertToDocument(sampleXml);
        String json = ((DittoDocument) originalDocument).toJson();
        
        // When
        @SuppressWarnings("unchecked")
        Class<? extends DittoDocument> docClass = (Class<? extends DittoDocument>) originalDocument.getClass();
        DittoDocument roundTripDocument = DittoDocument.fromJson(json, docClass);
        
        // Then
        assertThat(roundTripDocument).isNotNull();
        assertThat(roundTripDocument.getClass()).isEqualTo(originalDocument.getClass());
    }

    @Test
    void testExampleRunsWithoutErrors() {
        // This test ensures the main method doesn't throw exceptions
        // When/Then - should not throw any exceptions
        org.assertj.core.api.Assertions.assertThatCode(() -> SimpleExample.main(new String[]{})).doesNotThrowAnyException();
    }
}