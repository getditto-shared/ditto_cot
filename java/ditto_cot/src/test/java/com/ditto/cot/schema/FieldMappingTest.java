package com.ditto.cot.schema;


import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * Tests to verify that JSON field names are correctly mapped to Java field names
 * This is critical for ensuring compatibility with the Rust implementation
 */
class FieldMappingTest {

    // No Moshi needed; use DittoDocument interface methods

    /**
     * Test data for underscore field mapping verification
     */
    static Stream<Arguments> underscoreFieldMappings() {
        return Stream.of(
            Arguments.of("_id", "test-id", "getId"),
            Arguments.of("_c", 42, "getCounter"),
            Arguments.of("_v", 2, "getVersion"),
            Arguments.of("_r", true, "getRemoved")
        );
    }

    @ParameterizedTest
    @MethodSource("underscoreFieldMappings")
    void testUnderscoreFieldDeserialization(String jsonField, Object value, String getterMethod) throws Exception {
        // Given
        String json = String.format("{\"%s\": %s}", jsonField, 
            value instanceof String ? "\"" + value + "\"" : value);

        // When
        Common document = DittoDocument.fromJson(json, Common.class);

        // Then
        assertThat(document).isNotNull();
        
        Object actualValue = Common.class.getMethod(getterMethod).invoke(document);
        assertThat(actualValue).isEqualTo(value);
    }

    @Test
    void testCriticalFieldMappings() throws Exception {
        // Given - JSON with all underscore fields that need special mapping
        String json = """
            {
                "_id": "mapping-test-123",
                "_c": 999,
                "_v": 2,
                "_r": false,
                "a": "peer-key-test",
                "b": 1672531200000.0,
                "d": "tak-uid-test",
                "e": "Mapping Test User"
            }
            """;

        // When
        Common document = DittoDocument.fromJson(json, Common.class);

        // Then - verify all critical mappings work correctly
        assertThat(document.getId()).isEqualTo("mapping-test-123");
        assertThat(document.getCounter()).isEqualTo(999);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(false);
        assertThat(document.getA()).isEqualTo("peer-key-test");
        assertThat(document.getB()).isEqualTo(1672531200000.0);
        assertThat(document.getD()).isEqualTo("tak-uid-test");
        assertThat(document.getE()).isEqualTo("Mapping Test User");
    }

    @Test
    void testReverseFieldMapping() throws java.io.IOException {
        // Given
        Common document = new Common();
        document.setId("reverse-test");
        document.setCounter(123);
        document.setVersion(2);
        document.setRemoved(true);

        // When
        String json = ((DittoDocument) document).toJson();

        // Then - verify Java fields are serialized to correct JSON field names
        assertThat(json).contains("\"_id\":\"reverse-test\"");
        assertThat(json).contains("\"_c\":123");
        assertThat(json).contains("\"_v\":2");
        assertThat(json).contains("\"_r\":true");
    }

    @Test
    void testInheritedFieldMappingInApiDocument() throws java.io.IOException {
        // Given - API document JSON with underscore fields
        String json = """
            {
                "_id": "api-mapping-test",
                "_c": 55,
                "_v": 2,
                "_r": false,
                "a": "api-peer",
                "b": 1672531200000.0,
                "d": "api-tak",
                "e": "API User",
                "title": "API Mapping Test",
                "isFile": true
            }
            """;

        // When
        ApiDocument document = DittoDocument.fromJson(json, ApiDocument.class);

        // Then - verify inherited field mappings work in API document
        assertThat(document.getId()).isEqualTo("api-mapping-test");
        assertThat(document.getCounter()).isEqualTo(55);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(false);
        
        // And API-specific fields work normally
        assertThat(document.getTitle()).isEqualTo("API Mapping Test");
        assertThat(document.getIsFile()).isEqualTo(true);
    }

    @Test
    void testFieldMappingConsistencyAcrossTypes() throws java.io.IOException {
        // Given - same document data for both Common and ApiDocument
        Common commonDoc = new Common();
        commonDoc.setId("consistency-test");
        commonDoc.setCounter(777);
        commonDoc.setVersion(2);
        commonDoc.setRemoved(false);

        ApiDocument apiDoc = new ApiDocument();
        apiDoc.setId("consistency-test");
        apiDoc.setCounter(777);
        apiDoc.setVersion(2);
        apiDoc.setRemoved(false);

        // When
        String commonJson = ((DittoDocument) commonDoc).toJson();
        String apiJson = ((DittoDocument) apiDoc).toJson();

        // Then - verify both produce the same JSON for common fields
        assertThat(commonJson).contains("\"_id\":\"consistency-test\"");
        assertThat(commonJson).contains("\"_c\":777");
        assertThat(commonJson).contains("\"_v\":2");
        assertThat(commonJson).contains("\"_r\":false");
        
        assertThat(apiJson).contains("\"_id\":\"consistency-test\"");
        assertThat(apiJson).contains("\"_c\":777");
        assertThat(apiJson).contains("\"_v\":2");
        assertThat(apiJson).contains("\"_r\":false");
    }

    @Test
    void testSingleCharacterFieldNames() throws Exception {
        // Given - JSON with single character field names (common in this schema)
        String json = """
            {
                "_id": "single-char-test",
                "_c": 1,
                "_v": 2,
                "_r": false,
                "a": "single-a",
                "b": 1672531200000.0,
                "d": "single-d",
                "e": "single-e",
                "g": "single-g",
                "h": 1.5,
                "i": 2.5,
                "j": 3.5,
                "k": 4.5,
                "l": 5.5,
                "n": 10,
                "o": 20,
                "p": "single-p",
                "q": "single-q",
                "s": "single-s",
                "t": "single-t",
                "u": "single-u",
                "v": "single-v",
                "w": "single-w"
            }
            """;

        // When
        Common document = DittoDocument.fromJson(json, Common.class);

        // Then - verify all single character fields are mapped correctly
        assertThat(document.getA()).isEqualTo("single-a");
        assertThat(document.getB()).isEqualTo(1672531200000.0);
        assertThat(document.getD()).isEqualTo("single-d");
        assertThat(document.getE()).isEqualTo("single-e");
        assertThat(document.getG()).isEqualTo("single-g");
        assertThat(document.getH()).isEqualTo(1.5);
        assertThat(document.getI()).isEqualTo(2.5);
        assertThat(document.getJ()).isEqualTo(3.5);
        assertThat(document.getK()).isEqualTo(4.5);
        assertThat(document.getL()).isEqualTo(5.5);
        assertThat(document.getN()).isEqualTo(10);
        assertThat(document.getO()).isEqualTo(20);
        assertThat(document.getP()).isEqualTo("single-p");
        assertThat(document.getQ()).isEqualTo("single-q");
        assertThat(document.getS()).isEqualTo("single-s");
        assertThat(document.getT()).isEqualTo("single-t");
        assertThat(document.getU()).isEqualTo("single-u");
        assertThat(document.getV()).isEqualTo("single-v");
        assertThat(document.getW()).isEqualTo("single-w");
    }

    @Test
    void testCompleteRoundTripFieldMapping() throws Exception {
        // Given - document with all possible fields set
        Common original = new Common();
        original.setId("complete-round-trip");
        original.setCounter(12345);
        original.setVersion(2);
        original.setRemoved(true);
        original.setA("complete-a");
        original.setB(1672531200000.0);
        original.setD("complete-d");
        original.setE("complete-e");
        original.setG("complete-g");
        original.setH(10.1);
        original.setI(20.2);
        original.setJ(30.3);
        original.setK(40.4);
        original.setL(50.5);
        original.setN(100);
        original.setO(200);
        original.setP("complete-p");
        original.setQ("complete-q");
        original.setS("complete-s");
        original.setT("complete-t");
        original.setU("complete-u");
        original.setV("complete-v");
        original.setW("complete-w");

        // When - serialize and deserialize
        String json = ((DittoDocument) original).toJson();
        Common roundTrip = DittoDocument.fromJson(json, Common.class);

        // Then - verify perfect round trip for all fields
        assertThat(roundTrip.getId()).isEqualTo(original.getId());
        assertThat(roundTrip.getCounter()).isEqualTo(original.getCounter());
        assertThat(roundTrip.getVersion()).isEqualTo(original.getVersion());
        assertThat(roundTrip.getRemoved()).isEqualTo(original.getRemoved());
        assertThat(roundTrip.getA()).isEqualTo(original.getA());
        assertThat(roundTrip.getB()).isEqualTo(original.getB());
        assertThat(roundTrip.getD()).isEqualTo(original.getD());
        assertThat(roundTrip.getE()).isEqualTo(original.getE());
        assertThat(roundTrip.getG()).isEqualTo(original.getG());
        assertThat(roundTrip.getH()).isEqualTo(original.getH());
        assertThat(roundTrip.getI()).isEqualTo(original.getI());
        assertThat(roundTrip.getJ()).isEqualTo(original.getJ());
        assertThat(roundTrip.getK()).isEqualTo(original.getK());
        assertThat(roundTrip.getL()).isEqualTo(original.getL());
        assertThat(roundTrip.getN()).isEqualTo(original.getN());
        assertThat(roundTrip.getO()).isEqualTo(original.getO());
        assertThat(roundTrip.getP()).isEqualTo(original.getP());
        assertThat(roundTrip.getQ()).isEqualTo(original.getQ());
        assertThat(roundTrip.getS()).isEqualTo(original.getS());
        assertThat(roundTrip.getT()).isEqualTo(original.getT());
        assertThat(roundTrip.getU()).isEqualTo(original.getU());
        assertThat(roundTrip.getV()).isEqualTo(original.getV());
        assertThat(roundTrip.getW()).isEqualTo(original.getW());
    }
}