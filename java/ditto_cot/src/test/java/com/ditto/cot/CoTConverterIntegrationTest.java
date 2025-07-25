package com.ditto.cot;

import com.ditto.cot.schema.*;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import jakarta.xml.bind.JAXBException;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * Integration tests that validate full XML-to-Document-to-JSON conversion pipeline
 * Tests use real CoT XML examples from schema/example_xml
 */
class CoTConverterIntegrationTest {

    private CoTConverter converter;

    @BeforeEach
    void setUp() throws JAXBException {
        converter = new CoTConverter();
    }

    @Test
    void testFriendlyUnitConversion() throws Exception {
        // Given
        String xmlContent = readExampleXml("friendly_unit.xml");
        
        // When
        Object document = converter.convertToDocument(xmlContent);
        
        // Then
        assertThat(document).isInstanceOf(MapItemDocument.class);
        
        MapItemDocument mapItem = (MapItemDocument) document;
        assertThat(mapItem.getId()).isEqualTo("Alpha1");
        assertThat(mapItem.getW()).isEqualTo("a-f-G-U-C");
        assertThat(mapItem.getJ()).isEqualTo(34.052235); // lat
        assertThat(mapItem.getL()).isEqualTo(-118.243683); // lon
        assertThat(mapItem.getI()).isEqualTo(100.0); // hae
        assertThat(mapItem.getH()).isEqualTo(10.0); // ce
        assertThat(mapItem.getK()).isEqualTo(5.0); // le
        assertThat(mapItem.getP()).isEqualTo("m-g"); // how
        assertThat(mapItem.getE()).isEqualTo("Alpha1"); // callsign
        assertThat(mapItem.getC()).isEqualTo("Alpha1"); // name
        assertThat(mapItem.getF()).isTrue(); // visible
        
        // Verify detail conversion
        assertThat(mapItem.getR()).isNotNull();
        assertThat(mapItem.getR()).containsKey("contact");
    }

    @Test
    void testEmergencyBeaconConversion() throws Exception {
        // Given
        String xmlContent = readExampleXml("emergency_beacon.xml");
        
        // When
        Object document = converter.convertToDocument(xmlContent);
        
        // Then
        assertThat(document).isInstanceOf(GenericDocument.class);
        
        GenericDocument generic = (GenericDocument) document;
        assertThat(generic.getId()).isEqualTo("EMERGENCY-001");
        assertThat(generic.getW()).isEqualTo("b-m-p-s-r");
        assertThat(generic.getJ()).isEqualTo(40.712776); // lat
        assertThat(generic.getL()).isEqualTo(-74.005974); // lon
        assertThat(generic.getH()).isEqualTo(20.0); // ce
        assertThat(generic.getK()).isEqualTo(10.0); // le
        
        // Verify detail contains status
        assertThat(generic.getR()).containsKey("status");
    }

    @Test
    void testAtakTestConversion() throws Exception {
        // Given
        String xmlContent = readExampleXml("atak_test.xml");
        
        // When
        Object document = converter.convertToDocument(xmlContent);
        
        // Then
        assertThat(document).isInstanceOf(MapItemDocument.class);
        
        MapItemDocument mapItem = (MapItemDocument) document;
        assertThat(mapItem.getId()).isEqualTo("ANDROID-121304b069b9e23b");
        assertThat(mapItem.getW()).isEqualTo("a-f-G-U-C");
        assertThat(mapItem.getJ()).isEqualTo(1.2345); // lat
        assertThat(mapItem.getL()).isEqualTo(2.3456); // lon
        
        // Verify complex detail structure
        assertThat(mapItem.getR()).isNotNull();
        assertThat(mapItem.getR()).containsKey("contact");
        assertThat(mapItem.getR()).containsKey("ditto");
        assertThat(mapItem.getR()).containsKey("status");
    }

    @Test
    void testSensorSpiConversion() throws Exception {
        // Given
        String xmlContent = readExampleXml("sensor_spi.xml");
        
        // When
        Object document = converter.convertToDocument(xmlContent);
        
        // Then
        assertThat(document).isInstanceOf(ApiDocument.class);
        
        ApiDocument apiDoc = (ApiDocument) document;
        assertThat(apiDoc.getId()).isEqualTo("SENSOR-001");
        assertThat(apiDoc.getW()).isEqualTo("b-m-p-s-p-i");
        assertThat(apiDoc.getJ()).isEqualTo(35.689487); // lat
        assertThat(apiDoc.getL()).isEqualTo(139.691711); // lon
        assertThat(apiDoc.getTitle()).isEqualTo("CoT Event: SENSOR-001");
        assertThat(apiDoc.getMime()).isEqualTo("application/xml");
        
        // Verify sensor detail
        assertThat(apiDoc.getR()).containsKey("sensor");
    }

    @Test
    void testCustomTypeConversion() throws Exception {
        // Given
        String xmlContent = readExampleXml("custom_type.xml");
        
        // When
        Object document = converter.convertToDocument(xmlContent);
        
        // Then
        assertThat(document).isInstanceOf(GenericDocument.class);
        
        GenericDocument generic = (GenericDocument) document;
        assertThat(generic.getId()).isEqualTo("generic-test-123456789");
        assertThat(generic.getW()).isEqualTo("x-custom-generic-type");
        assertThat(generic.getJ()).isEqualTo(37.7749); // lat
        assertThat(generic.getL()).isEqualTo(-122.4194); // lon
        
        // Verify complex detail structure
        assertThat(generic.getR()).isNotNull();
        assertThat(generic.getR()).containsKey("custom_field");
        assertThat(generic.getR()).containsKey("nested");
        assertThat(generic.getR()).containsKey("numeric_field");
        assertThat(generic.getR()).containsKey("boolean_field");
    }

    @Test
    void testUsvTrackConversion() throws Exception {
        // Given
        String xmlContent = readExampleXml("usv_track.xml");
        
        // When
        Object document = converter.convertToDocument(xmlContent);
        
        // Then
        assertThat(document).isInstanceOf(MapItemDocument.class);
        
        MapItemDocument mapItem = (MapItemDocument) document;
        assertThat(mapItem.getId()).isEqualTo("00000000-0000-0000-0000-333333333333");
        assertThat(mapItem.getW()).isEqualTo("a-f-S-C-U"); // USV track event type
        assertThat(mapItem.getJ()).isEqualTo(-22.553768); // lat
        assertThat(mapItem.getL()).isEqualTo(150.818542); // lon
        assertThat(mapItem.getI()).isEqualTo(48.4075); // hae
        assertThat(mapItem.getH()).isEqualTo(50.0); // ce
        assertThat(mapItem.getK()).isEqualTo(10.0); // le
        assertThat(mapItem.getP()).isEqualTo("m-g"); // how
        
        // Verify callsign extraction - this is the key test
        System.out.println("ACTUAL Java USV Track Callsign (e): '" + mapItem.getE() + "'");
        System.out.println("EXPECTED: 'USV-4'");
        assertThat(mapItem.getE()).isEqualTo("USV-4"); // callsign should be extracted to 'e' field
        
        // Verify detail fields contain USV track information
        assertThat(mapItem.getR()).isNotNull();
        assertThat(mapItem.getR()).containsKey("contact");
        assertThat(mapItem.getR()).containsKey("track");
        assertThat(mapItem.getR()).containsKey("status");
        
        System.out.println("Java USV Track Callsign (e): " + mapItem.getE());
        System.out.println("Java USV Track Detail (r): " + mapItem.getR());
    }

    @ParameterizedTest
    @ValueSource(strings = {
        "friendly_unit.xml",
        "emergency_beacon.xml", 
        "atak_test.xml",
        "sensor_spi.xml",
        "custom_type.xml",
        "usv_track.xml"
    })
    void testFullRoundTripConversion(String xmlFile) throws Exception {
        // Given
        String originalXml = readExampleXml(xmlFile);
        
        // When - Convert XML to Document
        Object document = converter.convertToDocument(originalXml);
        
        // Then - Verify document was created
        assertThat(document).isNotNull();
        
        // And - Convert document to JSON
        String json = convertDocumentToJson(document);
        assertThat(json).isNotNull();
        assertThat(json).isNotEmpty();
        
        // And - Parse JSON back to document
        DittoDocument roundTripDocument = parseJsonToDocument(json, document.getClass());
        assertThat(roundTripDocument).isNotNull();
        
        // And - Verify critical fields match
        verifyCriticalFieldsMatch(document, roundTripDocument);
    }

    @Test 
    void testJacksonSerializationWithMapItemDocument() throws Exception {
        // Given
        String xmlContent = readExampleXml("friendly_unit.xml");
        MapItemDocument document = (MapItemDocument) converter.convertToDocument(xmlContent);
        
        // When
        String json = ((DittoDocument) document).toJson();
        
        // Then
        assertThat(json).contains("\"_id\":\"Alpha1\"");
        assertThat(json).contains("\"_c\":1");
        assertThat(json).contains("\"_v\":2");
        assertThat(json).contains("\"_r\":false");
        assertThat(json).contains("\"w\":\"a-f-G-U-C\"");
        assertThat(json).contains("\"j\":34.052235");
        assertThat(json).contains("\"l\":-118.243683");
        
        // And - Deserialize back
        MapItemDocument deserialized = DittoDocument.fromJson(json, MapItemDocument.class);
        assertThat(deserialized).isNotNull();
        assertThat(deserialized.getId()).isEqualTo(document.getId());
        assertThat(deserialized.getW()).isEqualTo(document.getW());
        assertThat(deserialized.getJ()).isEqualTo(document.getJ());
        assertThat(deserialized.getL()).isEqualTo(document.getL());
    }

    @Test
    void testJacksonSerializationWithApiDocument() throws Exception {
        // Given
        String xmlContent = readExampleXml("sensor_spi.xml");
        ApiDocument document = (ApiDocument) converter.convertToDocument(xmlContent);
        
        // When
        String json = ((DittoDocument) document).toJson();
        
        // Then
        assertThat(json).contains("\"_id\":\"SENSOR-001\"");
        assertThat(json).contains("\"w\":\"b-m-p-s-p-i\"");
        assertThat(json).contains("\"title\":\"CoT Event: SENSOR-001\"");
        assertThat(json).contains("\"mime\":\"application/xml\"");
        
        // And - Deserialize back
        ApiDocument deserialized = DittoDocument.fromJson(json, ApiDocument.class);
        assertThat(deserialized).isNotNull();
        assertThat(deserialized.getId()).isEqualTo(document.getId());
        assertThat(deserialized.getTitle()).isEqualTo(document.getTitle());
        assertThat(deserialized.getMime()).isEqualTo(document.getMime());
    }

    @Test
    void testDetailConversionAccuracy() throws Exception {
        // Given
        String xmlContent = readExampleXml("atak_test.xml");
        
        // When
        MapItemDocument document = (MapItemDocument) converter.convertToDocument(xmlContent);
        
        // Then - Verify detail structure is preserved
        Map<String, Object> detail = document.getR();
        assertThat(detail).isNotNull();
        
        // Verify contact information
        assertThat(detail).containsKey("contact");
        Object contact = detail.get("contact");
        assertThat(contact).isInstanceOf(Map.class);
        @SuppressWarnings("unchecked")
        Map<String, Object> contactMap = (Map<String, Object>) contact;
        assertThat(contactMap).containsKey("callsign");
        assertThat(contactMap.get("callsign")).isEqualTo("BRAMA");
        
        // Verify ditto information
        assertThat(detail).containsKey("ditto");
        Object ditto = detail.get("ditto");
        assertThat(ditto).isInstanceOf(Map.class);
        @SuppressWarnings("unchecked")
        Map<String, Object> dittoMap = (Map<String, Object>) ditto;
        assertThat(dittoMap).containsKey("deviceName");
        assertThat(dittoMap.get("deviceName")).isEqualTo("T9b9e23b");
    }

    @Test
    void testCoTEventParsingAccuracy() throws Exception {
        // Given
        String xmlContent = readExampleXml("friendly_unit.xml");
        
        // When
        CoTEvent cotEvent = converter.parseCoTXml(xmlContent);
        
        // Then
        assertThat(cotEvent).isNotNull();
        assertThat(cotEvent.getVersion()).isEqualTo("2.0");
        assertThat(cotEvent.getUid()).isEqualTo("Alpha1");
        assertThat(cotEvent.getType()).isEqualTo("a-f-G-U-C");
        assertThat(cotEvent.getTime()).isEqualTo("2025-06-24T14:10:00Z");
        assertThat(cotEvent.getStart()).isEqualTo("2025-06-24T14:10:00Z");
        assertThat(cotEvent.getStale()).isEqualTo("2025-06-24T14:20:00Z");
        assertThat(cotEvent.getHow()).isEqualTo("m-g");
        
        // Verify point data
        assertThat(cotEvent.getPoint()).isNotNull();
        assertThat(cotEvent.getPoint().getLatDouble()).isEqualTo(34.052235);
        assertThat(cotEvent.getPoint().getLonDouble()).isEqualTo(-118.243683);
        assertThat(cotEvent.getPoint().getHaeDouble()).isEqualTo(100.0);
        assertThat(cotEvent.getPoint().getCeDouble()).isEqualTo(10.0);
        assertThat(cotEvent.getPoint().getLeDouble()).isEqualTo(5.0);
        
        // Verify detail parsing
        assertThat(cotEvent.getDetail()).isNotNull();
        Map<String, Object> detailMap = cotEvent.getDetail().toMap();
        assertThat(detailMap).containsKey("contact");
    }

    @Test
    void testAndroidCoTConverterUnflattenRField() throws Exception {
        // Given - create an AndroidCoTConverter
        AndroidCoTConverter androidConverter = new AndroidCoTConverter();
        
        // Given - a flattened map like ATAK would receive from Ditto
        Map<String, Object> flattenedMap = new java.util.HashMap<>();
        flattenedMap.put("_id", "00000000-0000-0000-0000-333333333333");
        flattenedMap.put("e", "USV-4"); // callsign field
        flattenedMap.put("w", "a-f-S-C-U"); // event type
        flattenedMap.put("j", -22.553768); // lat
        flattenedMap.put("l", 150.818542); // lon
        // Flattened detail fields
        flattenedMap.put("r_contact_callsign", "USV-4");
        flattenedMap.put("r_contact_endpoint", "*:-1:stcp");
        flattenedMap.put("r_track_speed", "2.5");
        flattenedMap.put("r_track_course", "275.0");
        flattenedMap.put("r_status_battery", "85");
        
        System.out.println("Input flattened map: " + flattenedMap);
        
        // When - ATAK calls the unflatten method
        Map<String, Object> unflattenedMap = androidConverter.unflattenRField(flattenedMap);
        
        System.out.println("Output unflattened map: " + unflattenedMap);
        
        // Then - verify the r field is properly reconstructed
        assertThat(unflattenedMap).containsKey("r");
        
        @SuppressWarnings("unchecked")
        Map<String, Object> rField = (Map<String, Object>) unflattenedMap.get("r");
        assertThat(rField).isNotNull();
        
        // Verify contact structure
        assertThat(rField).containsKey("contact");
        @SuppressWarnings("unchecked")
        Map<String, Object> contactMap = (Map<String, Object>) rField.get("contact");
        assertThat(contactMap.get("callsign")).isEqualTo("USV-4");
        assertThat(contactMap.get("endpoint")).isEqualTo("*:-1:stcp");
        
        // Verify track structure
        assertThat(rField).containsKey("track");
        @SuppressWarnings("unchecked")
        Map<String, Object> trackMap = (Map<String, Object>) rField.get("track");
        assertThat(trackMap.get("speed")).isEqualTo("2.5");
        assertThat(trackMap.get("course")).isEqualTo("275.0");
        
        // Verify status structure
        assertThat(rField).containsKey("status");
        @SuppressWarnings("unchecked")
        Map<String, Object> statusMap = (Map<String, Object>) rField.get("status");
        assertThat(statusMap.get("battery")).isEqualTo("85");
        
        // Verify other fields are preserved
        assertThat(unflattenedMap.get("e")).isEqualTo("USV-4");
        assertThat(unflattenedMap.get("w")).isEqualTo("a-f-S-C-U");
        assertThat(unflattenedMap.get("_id")).isEqualTo("00000000-0000-0000-0000-333333333333");
        
        // Verify flattened r_* fields are removed
        assertThat(unflattenedMap).doesNotContainKey("r_contact_callsign");
        assertThat(unflattenedMap).doesNotContainKey("r_contact_endpoint");
        assertThat(unflattenedMap).doesNotContainKey("r_track_speed");
        
        System.out.println("✓ AndroidCoTConverter.unflattenRField() works correctly for ATAK");
    }

    // Helper methods

    private String readExampleXml(String filename) throws IOException {
        Path xmlPath = Paths.get("../../schema/example_xml/" + filename);
        return Files.readString(xmlPath);
    }

    private String convertDocumentToJson(Object document) throws Exception {
        if (document instanceof DittoDocument) {
            return ((DittoDocument) document).toJson();
        }
        throw new IllegalArgumentException("Document must implement DittoDocument interface: " + document.getClass());
    }

    @SuppressWarnings("unchecked")
    private DittoDocument parseJsonToDocument(String json, Class<?> documentClass) throws Exception {
        if (documentClass == ApiDocument.class) {
            return DittoDocument.fromJson(json, ApiDocument.class);
        } else if (documentClass == ChatDocument.class) {
            return DittoDocument.fromJson(json, ChatDocument.class);
        } else if (documentClass == FileDocument.class) {
            return DittoDocument.fromJson(json, FileDocument.class);
        } else if (documentClass == MapItemDocument.class) {
            return DittoDocument.fromJson(json, MapItemDocument.class);
        } else if (documentClass == GenericDocument.class) {
            return DittoDocument.fromJson(json, GenericDocument.class);
        } else if (documentClass == Common.class) {
            return DittoDocument.fromJson(json, Common.class);
        }
        throw new IllegalArgumentException("Unknown document class: " + documentClass);
    }

    private void verifyCriticalFieldsMatch(Object original, Object roundTrip) {
        // Use reflection or type-specific checks to verify critical fields
        if (original instanceof MapItemDocument && roundTrip instanceof MapItemDocument) {
            MapItemDocument orig = (MapItemDocument) original;
            MapItemDocument rt = (MapItemDocument) roundTrip;
            assertThat(rt.getId()).isEqualTo(orig.getId());
            assertThat(rt.getW()).isEqualTo(orig.getW());
            assertThat(rt.getJ()).isEqualTo(orig.getJ());
            assertThat(rt.getL()).isEqualTo(orig.getL());
        } else if (original instanceof ApiDocument && roundTrip instanceof ApiDocument) {
            ApiDocument orig = (ApiDocument) original;
            ApiDocument rt = (ApiDocument) roundTrip;
            assertThat(rt.getId()).isEqualTo(orig.getId());
            assertThat(rt.getW()).isEqualTo(orig.getW());
            assertThat(rt.getTitle()).isEqualTo(orig.getTitle());
        } else if (original instanceof GenericDocument && roundTrip instanceof GenericDocument) {
            GenericDocument orig = (GenericDocument) original;
            GenericDocument rt = (GenericDocument) roundTrip;
            assertThat(rt.getId()).isEqualTo(orig.getId());
            assertThat(rt.getW()).isEqualTo(orig.getW());
        }
        // Add more type checks as needed
    }
}