package com.ditto.cot.schema;


import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;

class ApiDocumentTest {

    // No Moshi needed; use DittoDocument interface methods

    @Test
    void testSerializationWithAllFields() throws com.fasterxml.jackson.core.JsonProcessingException {
        // Given
        ApiDocument document = new ApiDocument();
        
        // Common fields
        document.setId("api-doc-123");
        document.setCounter(5);
        document.setVersion(2);
        document.setRemoved(false);
        document.setA("api-peer-key");
        document.setB(1672531200000.0);
        document.setD("api-tak-uid");
        document.setE("API Callsign");
        document.setJ(37.7749);
        document.setL(-122.4194);
        document.setW("b-m-p-s-p-i");
        
        Map<String, Object> detail = new HashMap<>();
        detail.put("api", "test");
        document.setR(detail);
        
        // API-specific fields
        document.setIsFile(true);
        document.setTitle("Test API Document");
        document.setMime("application/json");
        document.setContentType("application/json");
        document.setTag("test-tag");
        document.setData("test data content");
        document.setIsRemoved(false);
        document.setTimeMillis(1672531200);
        document.setSource("test-source");

        // When
        String json = ((DittoDocument) document).toJson();

        // Then
        // Verify Common fields are serialized with correct JSON names
        assertThat(json).contains("\"_id\":\"api-doc-123\"");
        assertThat(json).contains("\"_c\":5");
        assertThat(json).contains("\"_v\":2");
        assertThat(json).contains("\"_r\":false");
        assertThat(json).contains("\"a\":\"api-peer-key\"");
        assertThat(json).contains("\"d\":\"api-tak-uid\"");
        assertThat(json).contains("\"e\":\"API Callsign\"");
        assertThat(json).contains("\"j\":37.7749");
        assertThat(json).contains("\"l\":-122.4194");
        assertThat(json).contains("\"w\":\"b-m-p-s-p-i\"");
        
        // Verify API-specific fields
        assertThat(json).contains("\"isFile\":true");
        assertThat(json).contains("\"title\":\"Test API Document\"");
        assertThat(json).contains("\"mime\":\"application/json\"");
        assertThat(json).contains("\"contentType\":\"application/json\"");
        assertThat(json).contains("\"tag\":\"test-tag\"");
        assertThat(json).contains("\"data\":\"test data content\"");
        assertThat(json).contains("\"isRemoved\":false");
        assertThat(json).contains("\"timeMillis\":1672531200");
        assertThat(json).contains("\"source\":\"test-source\"");
    }

    @Test
    void testDeserializationWithAllFields() throws java.io.IOException {
        // Given
        String json = """
            {
                "_id": "api-doc-456",
                "_c": 10,
                "_v": 2,
                "_r": false,
                "a": "api-peer-key-2",
                "b": 1672534800000.0,
                "d": "api-tak-uid-2",
                "e": "API Callsign 2",
                "j": 40.7128,
                "l": -74.0060,
                "w": "b-m-p-s-p-i",
                "r": {
                    "api": "test-deserialize"
                },
                "isFile": false,
                "title": "Deserialized API Document",
                "mime": "text/plain",
                "contentType": "text/plain",
                "tag": "deserialize-tag",
                "data": "deserialized data",
                "isRemoved": true,
                "timeMillis": 1672534800,
                "source": "deserialize-source"
            }
            """;

        // When
        ApiDocument document = DittoDocument.fromJson(json, ApiDocument.class);

        // Then
        assertThat(document).isNotNull();
        
        // Verify Common fields are properly mapped
        assertThat(document.getId()).isEqualTo("api-doc-456");
        assertThat(document.getCounter()).isEqualTo(10);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(false);
        assertThat(document.getA()).isEqualTo("api-peer-key-2");
        assertThat(document.getB()).isEqualTo(1672534800000.0);
        assertThat(document.getD()).isEqualTo("api-tak-uid-2");
        assertThat(document.getE()).isEqualTo("API Callsign 2");
        assertThat(document.getJ()).isEqualTo(40.7128);
        assertThat(document.getL()).isEqualTo(-74.0060);
        assertThat(document.getW()).isEqualTo("b-m-p-s-p-i");
        assertThat(document.getR()).containsKey("api");
        
        // Verify API-specific fields
        assertThat(document.getIsFile()).isEqualTo(false);
        assertThat(document.getTitle()).isEqualTo("Deserialized API Document");
        assertThat(document.getMime()).isEqualTo("text/plain");
        assertThat(document.getContentType()).isEqualTo("text/plain");
        assertThat(document.getTag()).isEqualTo("deserialize-tag");
        assertThat(document.getData()).isEqualTo("deserialized data");
        assertThat(document.getIsRemoved()).isEqualTo(true);
        assertThat(document.getTimeMillis()).isEqualTo(1672534800);
        assertThat(document.getSource()).isEqualTo("deserialize-source");
    }

    @Test
    void testInheritanceFromCommon() {
        // Given/When
        ApiDocument document = new ApiDocument();
        
        // Set Common fields
        document.setId("inheritance-test");
        document.setCounter(1);
        document.setVersion(2);
        document.setRemoved(false);
        document.setA("inheritance-peer");
        document.setB(1672531200000.0);
        document.setD("inheritance-tak");
        document.setE("Inheritance Test");
        
        // Set API-specific fields
        document.setTitle("Inheritance Test Document");
        document.setMime("application/xml");

        // Then - verify all Common methods are available and working
        assertThat(document.getId()).isEqualTo("inheritance-test");
        assertThat(document.getCounter()).isEqualTo(1);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(false);
        assertThat(document.getA()).isEqualTo("inheritance-peer");
        assertThat(document.getB()).isEqualTo(1672531200000.0);
        assertThat(document.getD()).isEqualTo("inheritance-tak");
        assertThat(document.getE()).isEqualTo("Inheritance Test");
        
        // And API-specific methods work
        assertThat(document.getTitle()).isEqualTo("Inheritance Test Document");
        assertThat(document.getMime()).isEqualTo("application/xml");
    }

    @Test
    void testFieldMappingCorrectness() throws java.io.IOException {
        // Given - JSON with underscore field names
        String json = """
            {
                "_id": "field-mapping-test",
                "_c": 123,
                "_v": 2,
                "_r": true
            }
            """;

        // When
        ApiDocument document = DittoDocument.fromJson(json, ApiDocument.class);

        // Then - verify the underscore fields map to correct Java field names
        assertThat(document.getId()).isEqualTo("field-mapping-test");
        assertThat(document.getCounter()).isEqualTo(123);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(true);
    }

    @Test
    void testRoundTripSerialization() throws Exception {
        // Given
        ApiDocument original = new ApiDocument();
        original.setId("round-trip-api");
        original.setCounter(42);
        original.setVersion(2);
        original.setRemoved(false);
        original.setA("round-trip-peer");
        original.setB(1672531200000.0);
        original.setD("round-trip-tak");
        original.setE("Round Trip API");
        original.setTitle("Round Trip Test");
        original.setMime("application/json");
        original.setData("round trip data");
        original.setIsFile(true);
        original.setTimeMillis(1672531200);

        // When
        String json = ((DittoDocument) original).toJson();
        ApiDocument roundTrip = DittoDocument.fromJson(json, ApiDocument.class);

        // Then
        assertThat(roundTrip).isNotNull();
        assertThat(roundTrip.getId()).isEqualTo(original.getId());
        assertThat(roundTrip.getCounter()).isEqualTo(original.getCounter());
        assertThat(roundTrip.getVersion()).isEqualTo(original.getVersion());
        assertThat(roundTrip.getRemoved()).isEqualTo(original.getRemoved());
        assertThat(roundTrip.getA()).isEqualTo(original.getA());
        assertThat(roundTrip.getB()).isEqualTo(original.getB());
        assertThat(roundTrip.getD()).isEqualTo(original.getD());
        assertThat(roundTrip.getE()).isEqualTo(original.getE());
        assertThat(roundTrip.getTitle()).isEqualTo(original.getTitle());
        assertThat(roundTrip.getMime()).isEqualTo(original.getMime());
        assertThat(roundTrip.getData()).isEqualTo(original.getData());
        assertThat(roundTrip.getIsFile()).isEqualTo(original.getIsFile());
        assertThat(roundTrip.getTimeMillis()).isEqualTo(original.getTimeMillis());
    }

    @Test
    void testOptionalSourceField() throws Exception {
        // Given - JSON without source field
        String jsonWithoutSource = """
            {
                "_id": "no-source-test",
                "_c": 1,
                "_v": 2,
                "_r": false,
                "a": "peer",
                "b": 1672531200000.0,
                "d": "tak",
                "e": "Test"
            }
            """;

        // When
        ApiDocument document = DittoDocument.fromJson(jsonWithoutSource, ApiDocument.class);

        // Then
        assertThat(document).isNotNull();
        assertThat(document.getSource()).isNull();

        // Given - JSON with source field
        String jsonWithSource = """
            {
                "_id": "with-source-test",
                "_c": 1,
                "_v": 2,
                "_r": false,
                "a": "peer",
                "b": 1672531200000.0,
                "d": "tak",
                "e": "Test",
                "source": "test-origin"
            }
            """;

        // When
        ApiDocument documentWithSource = DittoDocument.fromJson(jsonWithSource, ApiDocument.class);

        // Then
        assertThat(documentWithSource).isNotNull();
        assertThat(documentWithSource.getSource()).isEqualTo("test-origin");
    }
}