use ditto_cot::cot_events::CotEvent;
use ditto_cot::detail_parser::parse_detail_section;
use ditto_cot::ditto::to_ditto::{cot_to_document, extract_callsign};

fn main() {
    // Create a chat event exactly like in the user's test
    let chat_event = CotEvent::new_chat_message(
        "CHAT-MAP-001",
        "DELTA-4",
        "Test message content",
        "Operations Room",
        "ops-room-001",
    );

    println!("=== CHAT EVENT DETAIL ===");
    println!("Detail string: {}", chat_event.detail);
    
    println!("\n=== PARSING DETAIL ===");
    let parsed_detail = parse_detail_section(&chat_event.detail);
    println!("Parsed detail map: {:#?}", parsed_detail);
    
    println!("\n=== EXTRACT CALLSIGN ===");
    // This is the private function, so let's simulate what it does
    
    // The function first checks for a "chat" key
    if let Some(chat_obj) = parsed_detail.get("chat") {
        println!("Found 'chat' object: {:?}", chat_obj);
        if let Some(chat_obj_map) = chat_obj.as_object() {
            if let Some(from_value) = chat_obj_map.get("from") {
                if let Some(cs) = from_value.as_str() {
                    println!("Successfully extracted callsign: {}", cs);
                } else {
                    println!("'from' value is not a string: {:?}", from_value);
                }
            } else {
                println!("No 'from' key found in chat object");
            }
        } else {
            println!("'chat' value is not an object: {:?}", chat_obj);
        }
    } else {
        println!("No 'chat' key found in parsed detail");
        println!("Available keys: {:?}", parsed_detail.keys().collect::<Vec<_>>());
    }
    
    println!("\n=== CONVERSION TO DITTO ===");
    let ditto_doc = cot_to_document(&chat_event, "test-peer");
    match ditto_doc {
        ditto_cot::ditto::to_ditto::CotDocument::Chat(chat_doc) => {
            println!("Chat document callsign (e field): {}", chat_doc.e);
            println!("Author callsign: {:?}", chat_doc.author_callsign);
        }
        _ => println!("Document was not converted to Chat type"),
    }
    
    println!("\n=== ANALYSIS ===");
    println!("The detail string '{}' is not valid XML because:", chat_event.detail);
    println!("1. The attributes don't have quoted values");
    println!("2. 'chat' should be a proper XML element, not text with attributes");
    println!("3. The XML parser can't parse unquoted attribute values");
    
    println!("\n=== EXPECTED FORMAT ===");
    println!("Should be: <detail><chat from=\"DELTA-4\" room=\"Operations Room\" roomId=\"ops-room-001\" msg=\"Test message content\"/></detail>");
}