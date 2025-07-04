package com.ditto.cot.schema;


import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;

class CommonTest {

    // No ObjectMapper needed; use DittoDocument interface methods

    @Test
    void testSerializationWithAllFields() throws com.fasterxml.jackson.core.JsonProcessingException {
        // Given
        Common document = new Common();
        document.setId("test-id-123");
        document.setCounter(42);
        document.setVersion(2);
        document.setRemoved(false);
        document.setA("peer-key-abc");
        document.setB(1672531200000.0); // 2023-01-01T00:00:00Z
        document.setD("tak-uid-456");
        document.setE("Test Callsign");
        document.setG("1.0.0");
        document.setH(10.5);
        document.setI(100.0);
        document.setJ(37.7749);
        document.setK(5.2);
        document.setL(-122.4194);
        document.setN(1672531200);
        document.setO(1672534800);
        document.setP("h-e");
        document.setQ("unclassified");
        
        Map<String, Object> detail = new HashMap<>();
        detail.put("contact", Map.of("callsign", "ALPHA"));
        detail.put("remarks", "Test detail");
        document.setR(detail);
        
        document.setS("exercise");
        document.setT("r-d");
        document.setU("restricted");
        document.setV("USA");
        document.setW("a-f-G-U-C");

        // When
        String json = ((DittoDocument) document).toJson();

        // Then
        assertThat(json).contains("\"_id\":\"test-id-123\"");
        assertThat(json).contains("\"_c\":42");
        assertThat(json).contains("\"_v\":2");
        assertThat(json).contains("\"_r\":false");
        assertThat(json).contains("\"a\":\"peer-key-abc\"");
        assertThat(json).contains("\"b\":1.6725312E12");
        assertThat(json).contains("\"d\":\"tak-uid-456\"");
        assertThat(json).contains("\"e\":\"Test Callsign\"");
        assertThat(json).contains("\"j\":37.7749");
        assertThat(json).contains("\"l\":-122.4194");
        assertThat(json).contains("\"w\":\"a-f-G-U-C\"");
    }

    @Test
    void testDeserializationWithAllFields() throws com.fasterxml.jackson.core.JsonProcessingException, java.io.IOException {
        // Given
        String json = """
            {
                "_id": "test-id-123",
                "_c": 42,
                "_v": 2,
                "_r": false,
                "a": "peer-key-abc",
                "b": 1672531200000.0,
                "d": "tak-uid-456",
                "e": "Test Callsign",
                "g": "1.0.0",
                "h": 10.5,
                "i": 100.0,
                "j": 37.7749,
                "k": 5.2,
                "l": -122.4194,
                "n": 1672531200,
                "o": 1672534800,
                "p": "h-e",
                "q": "unclassified",
                "r": {
                    "contact": {"callsign": "ALPHA"},
                    "remarks": "Test detail"
                },
                "s": "exercise",
                "t": "r-d",
                "u": "restricted",
                "v": "USA",
                "w": "a-f-G-U-C"
            }
            """;

        // When
        Common document = DittoDocument.fromJson(json, Common.class);

        // Then
        assertThat(document).isNotNull();
        assertThat(document.getId()).isEqualTo("test-id-123");
        assertThat(document.getCounter()).isEqualTo(42);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(false);
        assertThat(document.getA()).isEqualTo("peer-key-abc");
        assertThat(document.getB()).isEqualTo(1672531200000.0);
        assertThat(document.getD()).isEqualTo("tak-uid-456");
        assertThat(document.getE()).isEqualTo("Test Callsign");
        assertThat(document.getG()).isEqualTo("1.0.0");
        assertThat(document.getH()).isEqualTo(10.5);
        assertThat(document.getI()).isEqualTo(100.0);
        assertThat(document.getJ()).isEqualTo(37.7749);
        assertThat(document.getK()).isEqualTo(5.2);
        assertThat(document.getL()).isEqualTo(-122.4194);
        assertThat(document.getN()).isEqualTo(1672531200);
        assertThat(document.getO()).isEqualTo(1672534800);
        assertThat(document.getP()).isEqualTo("h-e");
        assertThat(document.getQ()).isEqualTo("unclassified");
        assertThat(document.getR()).isNotNull();
        assertThat(document.getR()).containsKey("contact");
        assertThat(document.getR()).containsKey("remarks");
        assertThat(document.getS()).isEqualTo("exercise");
        assertThat(document.getT()).isEqualTo("r-d");
        assertThat(document.getU()).isEqualTo("restricted");
        assertThat(document.getV()).isEqualTo("USA");
        assertThat(document.getW()).isEqualTo("a-f-G-U-C");
    }

    @Test
    void testSerializationWithMinimalRequiredFields() throws com.fasterxml.jackson.core.JsonProcessingException {
        // Given
        Common document = new Common();
        document.setId("minimal-id");
        document.setCounter(1);
        document.setVersion(2);
        document.setRemoved(false);
        document.setA("peer-key");
        document.setB(1672531200000.0);
        document.setD("tak-uid");
        document.setE("Callsign");

        // When
        String json = ((DittoDocument) document).toJson();

        // Then
        assertThat(json).contains("\"_id\":\"minimal-id\"");
        assertThat(json).contains("\"_c\":1");
        assertThat(json).contains("\"_v\":2");
        assertThat(json).contains("\"_r\":false");
        assertThat(json).contains("\"a\":\"peer-key\"");
        assertThat(json).contains("\"d\":\"tak-uid\"");
        assertThat(json).contains("\"e\":\"Callsign\"");
    }

    @Test
    void testDeserializationWithMinimalRequiredFields() throws com.fasterxml.jackson.core.JsonProcessingException, java.io.IOException {
        // Given
        String json = """
            {
                "_id": "minimal-id",
                "_c": 1,
                "_v": 2,
                "_r": false,
                "a": "peer-key",
                "b": 1672531200000.0,
                "d": "tak-uid",
                "e": "Callsign"
            }
            """;

        // When
        Common document = DittoDocument.fromJson(json, Common.class);

        // Then
        assertThat(document).isNotNull();
        assertThat(document.getId()).isEqualTo("minimal-id");
        assertThat(document.getCounter()).isEqualTo(1);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(false);
        assertThat(document.getA()).isEqualTo("peer-key");
        assertThat(document.getB()).isEqualTo(1672531200000.0);
        assertThat(document.getD()).isEqualTo("tak-uid");
        assertThat(document.getE()).isEqualTo("Callsign");
    }

    @Test
    void testDefaultValues() {
        // Given/When
        Common document = new Common();

        // Then
        assertThat(document.getG()).isEqualTo("");
        assertThat(document.getH()).isEqualTo(0.0);
        assertThat(document.getI()).isEqualTo(0.0);
        assertThat(document.getJ()).isEqualTo(0.0);
        assertThat(document.getK()).isEqualTo(0.0);
        assertThat(document.getL()).isEqualTo(0.0);
        assertThat(document.getN()).isEqualTo(0);
        assertThat(document.getO()).isEqualTo(0);
        assertThat(document.getP()).isEqualTo("");
        assertThat(document.getQ()).isEqualTo("");
        assertThat(document.getS()).isEqualTo("");
        assertThat(document.getT()).isEqualTo("");
        assertThat(document.getU()).isEqualTo("");
        assertThat(document.getV()).isEqualTo("");
        assertThat(document.getW()).isEqualTo("");
    }

    @Test
    void testRoundTripSerialization() throws com.fasterxml.jackson.core.JsonProcessingException, java.io.IOException {
        // Given
        Common original = new Common();
        original.setId("round-trip-test");
        original.setCounter(99);
        original.setVersion(2);
        original.setRemoved(true);
        original.setA("peer-key-round-trip");
        original.setB(1672531200000.0);
        original.setD("tak-uid-round-trip");
        original.setE("Round Trip Callsign");
        original.setJ(40.7128);
        original.setL(-74.0060);
        
        Map<String, Object> detail = new HashMap<>();
        detail.put("test", "round-trip");
        original.setR(detail);

        // When
        String json = ((DittoDocument) original).toJson();
        Common roundTrip = DittoDocument.fromJson(json, Common.class);

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
        assertThat(roundTrip.getJ()).isEqualTo(original.getJ());
        assertThat(roundTrip.getL()).isEqualTo(original.getL());
        assertThat(roundTrip.getR()).containsKey("test");
        assertThat(roundTrip.getR().get("test")).isEqualTo("round-trip");
    }
}