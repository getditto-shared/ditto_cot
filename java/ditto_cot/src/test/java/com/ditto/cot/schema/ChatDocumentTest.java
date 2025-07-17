package com.ditto.cot.schema;


import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;

class ChatDocumentTest {

    // No Moshi needed; use DittoDocument interface methods

    @Test
    void testChatDocumentSerialization() throws com.fasterxml.jackson.core.JsonProcessingException {
        // Given
        ChatDocument document = new ChatDocument();
        
        // Common fields
        document.setId("chat-doc-123");
        document.setCounter(3);
        document.setVersion(2);
        document.setRemoved(false);
        document.setA("chat-peer-key");
        document.setB(1672531200000.0);
        document.setD("chat-tak-uid");
        document.setE("Chat User");
        
        // Chat-specific fields
        document.setMessage("Hello from chat test!");
        document.setRoom("Test Room");
        document.setParent("parent-msg-id");
        document.setRoomId("room-123");
        document.setAuthorCallsign("TESTER");
        document.setAuthorUid("author-uid-456");
        document.setAuthorType("a-f-G-U-C");
        document.setTime("2023-01-01T12:00:00Z");
        document.setLocation("37.7749,-122.4194");
        document.setSource("chat-client");

        // When
        String json = ((DittoDocument) document).toJson();

        // Then
        // Verify Common fields
        assertThat(json).contains("\"_id\":\"chat-doc-123\"");
        assertThat(json).contains("\"_c\":3");
        assertThat(json).contains("\"_v\":2");
        assertThat(json).contains("\"_r\":false");
        
        // Verify Chat-specific fields
        assertThat(json).contains("\"message\":\"Hello from chat test!\"");
        assertThat(json).contains("\"room\":\"Test Room\"");
        assertThat(json).contains("\"parent\":\"parent-msg-id\"");
        assertThat(json).contains("\"roomId\":\"room-123\"");
        assertThat(json).contains("\"authorCallsign\":\"TESTER\"");
        assertThat(json).contains("\"authorUid\":\"author-uid-456\"");
        assertThat(json).contains("\"authorType\":\"a-f-G-U-C\"");
        assertThat(json).contains("\"time\":\"2023-01-01T12:00:00Z\"");
        assertThat(json).contains("\"location\":\"37.7749,-122.4194\"");
        assertThat(json).contains("\"source\":\"chat-client\"");
    }

    @Test
    void testChatDocumentDeserialization() throws java.io.IOException {
        // Given
        String json = """
            {
                "_id": "chat-deserialize-456",
                "_c": 7,
                "_v": 2,
                "_r": false,
                "a": "chat-peer-2",
                "b": 1672534800000.0,
                "d": "chat-tak-2",
                "e": "Chat User 2",
                "message": "Deserialized chat message",
                "room": "Deserialize Room",
                "parent": "parent-deserialize",
                "roomId": "room-deserialize-456",
                "authorCallsign": "DESERIALIZER",
                "authorUid": "deserialize-uid",
                "authorType": "a-f-G-U-C",
                "time": "2023-01-01T14:30:00Z",
                "location": "40.7128,-74.0060",
                "source": "deserialize-client"
            }
            """;

        // When
        ChatDocument document = DittoDocument.fromJson(json, ChatDocument.class);

        // Then
        assertThat(document).isNotNull();
        
        // Verify Common fields
        assertThat(document.getId()).isEqualTo("chat-deserialize-456");
        assertThat(document.getCounter()).isEqualTo(7);
        assertThat(document.getVersion()).isEqualTo(2);
        assertThat(document.getRemoved()).isEqualTo(false);
        assertThat(document.getA()).isEqualTo("chat-peer-2");
        assertThat(document.getB()).isEqualTo(1672534800000.0);
        assertThat(document.getD()).isEqualTo("chat-tak-2");
        assertThat(document.getE()).isEqualTo("Chat User 2");
        
        // Verify Chat-specific fields
        assertThat(document.getMessage()).isEqualTo("Deserialized chat message");
        assertThat(document.getRoom()).isEqualTo("Deserialize Room");
        assertThat(document.getParent()).isEqualTo("parent-deserialize");
        assertThat(document.getRoomId()).isEqualTo("room-deserialize-456");
        assertThat(document.getAuthorCallsign()).isEqualTo("DESERIALIZER");
        assertThat(document.getAuthorUid()).isEqualTo("deserialize-uid");
        assertThat(document.getAuthorType()).isEqualTo("a-f-G-U-C");
        assertThat(document.getTime()).isEqualTo("2023-01-01T14:30:00Z");
        assertThat(document.getLocation()).isEqualTo("40.7128,-74.0060");
        assertThat(document.getSource()).isEqualTo("deserialize-client");
    }

    @Test
    void testChatWithoutOptionalFields() throws Exception {
        // Given - minimal chat document
        String json = """
            {
                "_id": "minimal-chat",
                "_c": 1,
                "_v": 2,
                "_r": false,
                "a": "minimal-peer",
                "b": 1672531200000.0,
                "d": "minimal-tak",
                "e": "Minimal User",
                "message": "Minimal message",
                "room": "Minimal Room"
            }
            """;

        // When
        ChatDocument document = DittoDocument.fromJson(json, ChatDocument.class);

        // Then
        assertThat(document).isNotNull();
        assertThat(document.getMessage()).isEqualTo("Minimal message");
        assertThat(document.getRoom()).isEqualTo("Minimal Room");
        assertThat(document.getParent()).isNull();
        assertThat(document.getRoomId()).isNull();
        assertThat(document.getSource()).isNull();
    }

    @Test
    void testRoundTripChatSerialization() throws Exception {
        // Given
        ChatDocument original = new ChatDocument();
        original.setId("round-trip-chat");
        original.setCounter(15);
        original.setVersion(2);
        original.setRemoved(false);
        original.setA("round-trip-peer");
        original.setB(1672531200000.0);
        original.setD("round-trip-tak");
        original.setE("Round Trip User");
        original.setMessage("Round trip chat message");
        original.setRoom("Round Trip Room");
        original.setAuthorCallsign("ROUNDTRIP");
        original.setTime("2023-01-01T10:00:00Z");

        // When
        String json = ((DittoDocument) original).toJson();
        ChatDocument roundTrip = DittoDocument.fromJson(json, ChatDocument.class);

        // Then
        assertThat(roundTrip).isNotNull();
        assertThat(roundTrip.getId()).isEqualTo(original.getId());
        assertThat(roundTrip.getCounter()).isEqualTo(original.getCounter());
        assertThat(roundTrip.getMessage()).isEqualTo(original.getMessage());
        assertThat(roundTrip.getRoom()).isEqualTo(original.getRoom());
        assertThat(roundTrip.getAuthorCallsign()).isEqualTo(original.getAuthorCallsign());
        assertThat(roundTrip.getTime()).isEqualTo(original.getTime());
        assertThat(roundTrip.getA()).isEqualTo(original.getA());
        assertThat(roundTrip.getE()).isEqualTo(original.getE());
    }
}