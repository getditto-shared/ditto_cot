//! XML parsing utilities for CoT (Cursor on Target) messages.
//!
//! This module provides functionality to parse CoT XML messages into
//! structured Rust types.

use crate::detail_parser::parse_detail_section;
use crate::error::CotError;
use crate::model::FlatCotEvent;
use quick_xml::events::Event;
use quick_xml::Reader;

/// Parses a CoT XML string into a `FlatCotEvent`.
///
/// This function takes a CoT XML message as input and returns a structured
/// `FlatCotEvent` containing all the parsed fields. It handles both the main
/// event attributes and the detail section.
///
/// # Arguments
/// * `xml` - A string slice containing the CoT XML message
///
/// # Returns
/// * `Result<FlatCotEvent, CotError>` - The parsed event on success, or an error if parsing fails
///
/// # Examples
/// ```no_run
/// use ditto_cot::xml_parser::parse_cot;
///
/// let xml = r#"<event version="2.0" ...></event>"#;
/// match parse_cot(xml) {
///     Ok(event) => println!("Parsed event: {:?}", event),
///     Err(e) => eprintln!("Failed to parse CoT: {}", e),
/// }
/// ```
pub fn parse_cot(xml: &str) -> Result<FlatCotEvent, CotError> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut flat = FlatCotEvent {
        uid: String::new(),
        type_: String::new(),
        time: String::new(),
        start: String::new(),
        stale: String::new(),
        how: String::new(),
        lat: 0.0,
        lon: 0.0,
        hae: 0.0,
        ce: 0.0,
        le: 0.0,
        callsign: None,
        group_name: None,
        detail_extra: Default::default(),
    };

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            Event::Start(ref e) if e.name().as_ref() == b"event" => {
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = attr.unescape_value().unwrap_or_default().to_string();
                    match key.as_str() {
                        "uid" => flat.uid = val,
                        "type" => flat.type_ = val,
                        "time" => flat.time = val,
                        "start" => flat.start = val,
                        "stale" => flat.stale = val,
                        "how" => flat.how = val,
                        "lat" => {
                            flat.lat = val.parse::<f64>().map_err(|e| CotError::InvalidNumeric {
                                field: "lat".to_string(),
                                value: val.clone(),
                                source: Box::new(e),
                            })?
                        }
                        "lon" => {
                            flat.lon = val.parse::<f64>().map_err(|e| CotError::InvalidNumeric {
                                field: "lon".to_string(),
                                value: val.clone(),
                                source: Box::new(e),
                            })?
                        }
                        "hae" => {
                            flat.hae = val.parse::<f64>().map_err(|e| CotError::InvalidNumeric {
                                field: "hae".to_string(),
                                value: val.clone(),
                                source: Box::new(e),
                            })?
                        }
                        "ce" => {
                            flat.ce = val.parse::<f64>().map_err(|e| CotError::InvalidNumeric {
                                field: "ce".to_string(),
                                value: val.clone(),
                                source: Box::new(e),
                            })?
                        }
                        "le" => {
                            flat.le = val.parse::<f64>().map_err(|e| CotError::InvalidNumeric {
                                field: "le".to_string(),
                                value: val.clone(),
                                source: Box::new(e),
                            })?
                        }
                        _ => {}
                    }
                }
            }
            Event::Start(ref e) if e.name().as_ref() == b"detail" => {
                let mut detail_buf = Vec::new();
                let mut depth = 1;

                // Read until we find the matching end tag
                loop {
                    match reader.read_event_into(&mut detail_buf) {
                        Ok(Event::Start(_)) => depth += 1,
                        Ok(Event::End(_)) => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        Ok(Event::Eof) => break,
                        _ => {}
                    }
                    detail_buf.clear();
                }

                // Get the inner XML as a string
                let inner_xml = String::from_utf8_lossy(&detail_buf);
                let extras = parse_detail_section(&inner_xml);
                flat.detail_extra = extras;
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(flat)
}
