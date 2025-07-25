use ditto_cot::cot_events::CotEvent;
use ditto_cot::detail_parser::parse_detail_section;
use ditto_cot::ditto::to_ditto::cot_to_document;

#[test]
fn test_chat_callsign_extraction() {
    // Create a chat event exactly like in the user's test
    let chat_event = CotEvent::new_chat_message(
        "CHAT-MAP-001",
        "DELTA-4",
        "Test message content",
        "Operations Room",
        "ops-room-001",
    );

    println!("Detail string: {}", chat_event.detail);

    // Parse the detail section
    let parsed_detail = parse_detail_section(&chat_event.detail);
    println!("Parsed detail: {:#?}", parsed_detail);

    // Verify that the detail was parsed correctly and has a 'chat' object
    assert!(
        parsed_detail.contains_key("chat"),
        "Detail should contain 'chat' key"
    );

    let chat_obj = parsed_detail.get("chat").unwrap();
    assert!(chat_obj.is_object(), "Chat value should be an object");

    let chat_map = chat_obj.as_object().unwrap();
    assert!(
        chat_map.contains_key("from"),
        "Chat object should contain 'from' key"
    );

    let from_value = chat_map.get("from").unwrap();
    assert_eq!(
        from_value.as_str().unwrap(),
        "DELTA-4",
        "From value should be 'DELTA-4'"
    );

    // Test the conversion to Ditto document
    let ditto_doc = cot_to_document(&chat_event, "test-peer");

    match ditto_doc {
        ditto_cot::ditto::to_ditto::CotDocument::Chat(chat_doc) => {
            // The callsign should be extracted correctly into the 'e' field
            assert_eq!(
                chat_doc.e, "DELTA-4",
                "Chat document should have correct callsign in 'e' field"
            );
            println!("✓ Successfully extracted callsign: {}", chat_doc.e);
        }
        _ => panic!("Document should be converted to Chat type"),
    }
}

#[test]
fn test_chat_detail_parsing_with_spaces() {
    // Test with a chat message that has spaces in the room name
    let chat_event = CotEvent::new_chat_message(
        "CHAT-001",
        "BRAVO-2",
        "Hello world",
        "Command Center Alpha",
        "cmd-center-001",
    );

    let parsed_detail = parse_detail_section(&chat_event.detail);

    // Verify all fields are parsed correctly
    let chat_obj = parsed_detail.get("chat").unwrap().as_object().unwrap();
    assert_eq!(chat_obj.get("from").unwrap().as_str().unwrap(), "BRAVO-2");
    assert_eq!(
        chat_obj.get("room").unwrap().as_str().unwrap(),
        "Command Center Alpha"
    );
    assert_eq!(
        chat_obj.get("roomId").unwrap().as_str().unwrap(),
        "cmd-center-001"
    );
    assert_eq!(
        chat_obj.get("msg").unwrap().as_str().unwrap(),
        "Hello world"
    );
}
