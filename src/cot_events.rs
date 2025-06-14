//! Common TAK CoT (Cursor on Target) event templates and utilities.
//! 
//! This module provides pre-defined templates for common CoT message types used in the TAK ecosystem.
//! Each template includes the standard fields and can be customized as needed.

use std::collections::HashMap;
use std::io::Cursor;

use chrono::{DateTime, Utc};
use quick_xml::events::{BytesStart, BytesEnd, Event, BytesText};
use quick_xml::{Reader, Writer};

use crate::error::CotError;
use uuid::Uuid;

/// Represents a basic CoT event with common fields
#[derive(Debug, Clone)]
pub struct CotEvent {
    pub version: String,
    pub uid: String,
    pub event_type: String,
    pub time: DateTime<Utc>,
    pub start: DateTime<Utc>,
    pub stale: DateTime<Utc>,
    pub how: String,
    pub point: Point,
    pub detail: HashMap<String, String>,
}

/// Represents a geographic point with elevation and accuracy information
#[derive(Debug, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
    pub hae: f64,  // Height Above Ellipsoid
    pub ce: f64,   // Circular Error
    pub le: f64,   // Linear Error
}

impl Default for CotEvent {
    fn default() -> Self {
        let now = Utc::now();
        let uid = Uuid::new_v4().to_string();
        
        Self {
            version: "2.0".to_string(),
            uid: uid.clone(),
            event_type: "a-f-G-U-C".to_string(),  // Default: Military Ground Unit
            time: now,
            start: now,
            stale: now + chrono::Duration::minutes(5),  // Default 5 minute staleness
            how: "h-g-i-g-o".to_string(),  // Human-input GPS general
            point: Point {
                lat: 0.0,
                lon: 0.0,
                hae: 0.0,
                ce: 999999.0,  // Default high error values
                le: 999999.0,
            },
            detail: HashMap::new(),
        }
    }
}

impl CotEvent {
    /// Converts the CotEvent to an XML string
    pub fn to_xml(&self) -> Result<String, CotError> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        
        // Start event element
        let mut event_start = BytesStart::new("event");
        event_start.push_attribute(("version", self.version.as_str()));
        event_start.push_attribute(("uid", self.uid.as_str()));
        event_start.push_attribute(("type", self.event_type.as_str()));
        event_start.push_attribute(("time", self.time.to_rfc3339().as_str()));
        event_start.push_attribute(("start", self.start.to_rfc3339().as_str()));
        event_start.push_attribute(("stale", self.stale.to_rfc3339().as_str()));
        event_start.push_attribute(("how", self.how.as_str()));
        
        writer.write_event(Event::Start(event_start))?;
        
        // Write point
        let mut point_start = BytesStart::new("point");
        point_start.push_attribute(("lat", self.point.lat.to_string().as_str()));
        point_start.push_attribute(("lon", self.point.lon.to_string().as_str()));
        point_start.push_attribute(("hae", self.point.hae.to_string().as_str()));
        point_start.push_attribute(("ce", self.point.ce.to_string().as_str()));
        point_start.push_attribute(("le", self.point.le.to_string().as_str()));
        writer.write_event(Event::Empty(point_start))?;
        
        // Write detail elements
        if !self.detail.is_empty() {
            let detail_start = BytesStart::new("detail");
            writer.write_event(Event::Start(detail_start))?;
            
            // Special handling for chat messages
            if self.event_type == "b-t-f" {
                if let Some(chat) = self.detail.get("chat") {
                    let mut chat_elem = BytesStart::new("chat");
                    chat_elem.push_attribute(("message", chat.as_str()));
                    if let Some(chatroom) = self.detail.get("chatroom") {
                        chat_elem.push_attribute(("chatroom", chatroom.as_str()));
                    }
                    if let Some(chatgrp) = self.detail.get("chatgrp") {
                        chat_elem.push_attribute(("chatgrp", chatgrp.as_str()));
                    }
                    if let Some(sender) = self.detail.get("senderCallsign") {
                        chat_elem.push_attribute(("senderCallsign", sender.as_str()));
                    }
                    writer.write_event(Event::Empty(chat_elem))?;
                }
            } 
            // Special handling for emergency events
            else if self.event_type == "b-a-o-can" {
                if let Some(emergency) = self.detail.get("emergency") {
                    let mut emergency_elem = BytesStart::new("emergency");
                    emergency_elem.push_attribute(("type", emergency.as_str()));
                    writer.write_event(Event::Empty(emergency_elem))?;
                }
                
                if let Some(remarks) = self.detail.get("remarks") {
                    let remarks_elem = BytesStart::new("remarks");
                    writer.write_event(Event::Start(remarks_elem))?;
                    writer.write_event(Event::Text(BytesText::new(remarks)))?;
                    writer.write_event(Event::End(BytesEnd::new("remarks")))?;
                }
            } else {
                // Handle other detail elements
                for (key, value) in &self.detail {
                    if key.contains('.') {
                        // Handle nested elements with dot notation
                        let parts: Vec<&str> = key.splitn(2, '.').collect();
                        if parts.len() == 2 {
                            let elem_name = parts[0];
                            let attr_name = parts[1];
                            
                            let mut elem = BytesStart::new(elem_name);
                            elem.push_attribute((attr_name, value.as_str()));
                            writer.write_event(Event::Empty(elem))?;
                        }
                    } else {
                        // Handle top-level elements
                        let mut elem = BytesStart::new(key.as_str());
                        elem.push_attribute(("value", value.as_str()));
                        writer.write_event(Event::Empty(elem))?;
                    }
                }
            }
            
            writer.write_event(Event::End(BytesEnd::new("detail")))?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("event")))?;
        
        let result = writer.into_inner().into_inner();
        String::from_utf8(result).map_err(Into::into)
    }
    
    /// Parses a CoT XML string into a CotEvent
    pub fn from_xml(xml: &str) -> Result<Self, CotError> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        
        let mut event = CotEvent::default();
        let mut buf = Vec::new();
        let mut current_element: Option<String> = None;
        let mut in_detail = false;
        
        // Parse the XML document
        loop {
            buf.clear();
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"event" => {
                        // Parse event attributes
                        for attr in e.attributes() {
                            let attr = attr?;
                            let value = attr.unescape_value()?;
                            match attr.key.0 {
                                b"version" => event.version = value.into_owned(),
                                b"uid" => event.uid = value.into_owned(),
                                b"type" => event.event_type = value.into_owned(),
                                b"time" => event.time = Self::parse_datetime(&value)?,
                                b"start" => event.start = Self::parse_datetime(&value)?,
                                b"stale" => event.stale = Self::parse_datetime(&value)?,
                                b"how" => event.how = value.into_owned(),
                                _ => {}
                            }
                        }
                    }
                    b"point" => {
                        // Parse point attributes
                        println!("Found point element");
                        for attr in e.attributes() {
                            let attr = attr?;
                            let value = attr.unescape_value()?;
                            match attr.key.0 {
                                b"lat" => {
                                    let lat_val = value.parse().map_err(|_| CotError::InvalidFormat("Invalid lat".to_string()))?;
                                    println!("Parsed lat: {}", lat_val);
                                    event.point.lat = lat_val;
                                },
                                b"lon" => {
                                    let lon_val = value.parse().map_err(|_| CotError::InvalidFormat("Invalid lon".to_string()))?;
                                    println!("Parsed lon: {}", lon_val);
                                    event.point.lon = lon_val;
                                },
                                b"hae" => event.point.hae = value.parse().map_err(|_| CotError::InvalidFormat("Invalid hae".to_string()))?,
                                b"ce" => event.point.ce = value.parse().map_err(|_| CotError::InvalidFormat("Invalid ce".to_string()))?,
                                b"le" => event.point.le = value.parse().map_err(|_| CotError::InvalidFormat("Invalid le".to_string()))?,
                                _ => {}
                            }
                        }
                        println!("After parsing point: lat={}, lon={}", event.point.lat, event.point.lon);
                    }
                    b"detail" => {
                        in_detail = true;
                        // Process detail attributes if any
                        for attr in e.attributes() {
                            let attr = attr?;
                            let key = format!("detail.{}", String::from_utf8(attr.key.0.to_vec())?);
                            let value = attr.unescape_value()?.to_string();
                            event.detail.insert(key, value);
                        }
                    }
                    b"chat" => {
                        // Special handling for chat messages
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"message" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("chat".to_string(), value);
                                },
                                b"chatroom" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("chatroom".to_string(), value);
                                },
                                b"chatgrp" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("chatgrp".to_string(), value);
                                },
                                b"senderCallsign" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("senderCallsign".to_string(), value);
                                },
                                _ => {}
                            }
                        }
                    },
                    b"emergency" => {
                        // Special handling for emergency elements
                        println!("Found emergency element");
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"type" => {
                                    let value = attr.unescape_value()?.to_string();
                                    println!("Parsed emergency type: {}", value);
                                    event.detail.insert("emergency".to_string(), value);
                                },
                                _ => {}
                            }
                        }
                    }
                    _ if in_detail => {
                        // Inside detail element, handle nested elements
                        let name = String::from_utf8(e.name().as_ref().to_vec())?;
                        current_element = Some(name.clone());
                        
                        // Process attributes of this element
                        for attr in e.attributes() {
                            let attr = attr?;
                            let key = format!("{}.{}", name, String::from_utf8(attr.key.0.to_vec())?);
                            let value = attr.unescape_value()?.to_string();
                            event.detail.insert(key, value);
                        }
                    }
                    _ => {
                        // Handle other start elements outside detail
                        current_element = Some(String::from_utf8(e.name().as_ref().to_vec())?);
                    }
                },
                Ok(Event::Empty(e)) => {
                    // Handle self-closing elements
                    let name = String::from_utf8(e.name().as_ref().to_vec())?;
                    
                    // Special handling for point element
                    if name == "point" {
                        println!("Found point element (Empty)");
                        for attr in e.attributes() {
                            let attr = attr?;
                            let value = attr.unescape_value()?;
                            match attr.key.0 {
                                b"lat" => {
                                    let lat_val = value.parse().map_err(|_| CotError::InvalidFormat("Invalid lat".to_string()))?;
                                    println!("Parsed lat (Empty): {}", lat_val);
                                    event.point.lat = lat_val;
                                },
                                b"lon" => {
                                    let lon_val = value.parse().map_err(|_| CotError::InvalidFormat("Invalid lon".to_string()))?;
                                    println!("Parsed lon (Empty): {}", lon_val);
                                    event.point.lon = lon_val;
                                },
                                b"hae" => event.point.hae = value.parse().map_err(|_| CotError::InvalidFormat("Invalid hae".to_string()))?,
                                b"ce" => event.point.ce = value.parse().map_err(|_| CotError::InvalidFormat("Invalid ce".to_string()))?,
                                b"le" => event.point.le = value.parse().map_err(|_| CotError::InvalidFormat("Invalid le".to_string()))?,
                                _ => {}
                            }
                        }
                        println!("After parsing point (Empty): lat={}, lon={}", event.point.lat, event.point.lon);
                    }
                    // Special handling for chat messages
                    else if name == "chat" {
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"message" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("chat".to_string(), value);
                                },
                                b"chatroom" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("chatroom".to_string(), value);
                                },
                                b"chatgrp" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("chatgrp".to_string(), value);
                                },
                                b"senderCallsign" => {
                                    let value = attr.unescape_value()?.to_string();
                                    event.detail.insert("senderCallsign".to_string(), value);
                                },
                                _ => {}
                            }
                        }
                    }
                    // Special handling for emergency elements
                    else if name == "emergency" {
                        println!("Found emergency element (Empty)");
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.0 {
                                b"type" => {
                                    let value = attr.unescape_value()?.to_string();
                                    println!("Parsed emergency type (Empty): {}", value);
                                    event.detail.insert("emergency".to_string(), value);
                                },
                                _ => {}
                            }
                        }
                    } else {
                        // Handle other self-closing elements
                        for attr in e.attributes() {
                            let attr = attr?;
                            let key = if in_detail {
                                format!("{}.{}", name, String::from_utf8(attr.key.0.to_vec())?)
                            } else {
                                format!("detail.{}.{}", name, String::from_utf8(attr.key.0.to_vec())?)
                            };
                            let value = attr.unescape_value()?.to_string();
                            event.detail.insert(key, value);
                        }
                    }
                },
                Ok(Event::End(e)) if e.name().as_ref() == b"detail" => {
                    // End of detail element
                    in_detail = false;
                },
                Ok(Event::End(e)) if e.name().as_ref() == b"event" => {
                    // End of event element, we're done
                    return Ok(event);
                },
                Ok(Event::End(_)) => {
                    // End of some other element
                    current_element = None;
                },
                Ok(Event::Text(e)) => {
                    if let Some(name) = &current_element {
                        let value = e.unescape()?.to_string();
                        if !value.trim().is_empty() {
                            // For elements inside detail, store with proper namespacing
                            if in_detail {
                                event.detail.insert(name.clone(), value);
                            } else {
                                event.detail.insert(format!("detail.{}", name), value);
                            }
                        }
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(CotError::XmlError(e.to_string())),
                _ => {}
            }
        }
        
        Ok(event)
    }
    
    /// Helper function to parse ISO 8601 datetime strings
    fn parse_datetime(s: &str) -> Result<DateTime<Utc>, CotError> {
        // First try parsing as RFC 3339 format
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .or_else(|_| {
                // Try with different formats if needed
                DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
                    .or_else(|_| DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%SZ"))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .map_err(|_| CotError::InvalidFormat(format!("Invalid datetime format: {}", s)))
    }
    
    /// Creates a new location update event
    pub fn new_location_update(
        uid: &str,
        callsign: &str,
        team: &str,
        lat: f64,
        lon: f64,
        hae: f64,
    ) -> Self {
        let now = Utc::now();
        let mut event = Self {
            version: "2.0".to_string(),
            uid: uid.to_string(),
            event_type: "a-f-G-U-C".to_string(),  // Default location update type
            time: now,
            start: now,
            stale: now + chrono::Duration::minutes(5),
            how: "h-g-i-g-o".to_string(),  // Human-input GPS general
            point: Point {
                lat,
                lon,
                hae,
                ce: 10.0,  // Default circular error in meters
                le: 10.0,  // Default linear error in meters
            },
            detail: HashMap::new(),
        };
        
        // Add contact and group details
        event.detail.insert("contact.callsign".to_string(), callsign.to_string());
        event.detail.insert("__group.name".to_string(), team.to_string());
        event.detail.insert("__group.role".to_string(), "Team Member".to_string());
        
        event
    }
    
    /// Creates a new chat message event
    pub fn new_chat_message(
        sender_uid: &str,
        sender_callsign: &str,
        message: &str,
        chatroom: &str,
        chat_group_uid: &str,
    ) -> Self {
        let now = Utc::now();
        let uid = format!("Chat-{}-{}-{}", sender_uid, uuid::Uuid::new_v4(), now.timestamp_millis());
        
        let mut detail = HashMap::new();
        detail.insert("chat".to_string(), message.to_string());
        detail.insert("chatroom".to_string(), chatroom.to_string());
        detail.insert("chatgrp".to_string(), chat_group_uid.to_string());
        detail.insert("senderCallsign".to_string(), sender_callsign.to_string());
        
        CotEvent {
            version: "2.0".to_string(),
            uid,
            event_type: "b-t-f".to_string(),
            time: now,
            start: now,
            stale: now + chrono::Duration::minutes(5),
            how: "h-g-i-g-o".to_string(),
            point: Point {
                lat: 0.0,
                lon: 0.0,
                hae: 0.0,
                ce: 999999.0,
                le: 999999.0,
            },
            detail,
        }
    }
    
    /// Creates a new emergency signal (911) event
    pub fn new_emergency(
        uid: &str,
        callsign: &str,
        lat: f64,
        lon: f64,
        emergency_type: &str,
        message: &str,
    ) -> Self {
        let mut event = Self::default();
        event.uid = uid.to_string();
        event.event_type = "b-a-o-can".to_string();  // Emergency
        event.point.lat = lat;
        event.point.lon = lon;
        
        let mut detail = HashMap::new();
        detail.insert("emergency".to_string(), emergency_type.to_string());
        detail.insert("contact.callsign".to_string(), callsign.to_string());
        detail.insert("remarks".to_string(), message.to_string());
        
        event.detail = detail;
        event
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_location_update_creation() {
        let event = CotEvent::new_location_update(
            "USER-123",
            "ALPHA-1",
            "Cyan",
            34.12345,
            -118.12345,
            150.0
        );
        
        assert_eq!(event.uid, "USER-123");
        assert_eq!(event.event_type, "a-f-G-U-C");
        assert_eq!(event.point.lat, 34.12345);
        assert_eq!(event.point.lon, -118.12345);
        assert_eq!(event.point.hae, 150.0);
        assert_eq!(event.detail.get("contact.callsign").unwrap(), "ALPHA-1");
        assert_eq!(event.detail.get("__group.name").unwrap(), "Cyan");
    }
    
    #[test]
    fn test_chat_message_creation() {
        let event = CotEvent::new_chat_message(
            "USER-123",
            "ALPHA-1",
            "Test message",
            "All Chat Rooms",
            "All Chat Rooms"
        );
        
        assert_eq!(event.event_type, "b-t-f");
        assert!(event.uid.contains("USER-123"));
        assert_eq!(event.detail.get("chat").unwrap(), "Test message");
        assert_eq!(event.detail.get("chatroom").unwrap(), "All Chat Rooms");
    }
    
    #[test]
    fn test_emergency_creation() {
        let event = CotEvent::new_emergency(
            "USER-123",
            "ALPHA-1",
            34.12345,
            -118.12345,
            "Emergency-911",
            "Need immediate assistance!"
        );
        
        assert_eq!(event.uid, "USER-123");
        assert_eq!(event.event_type, "b-a-o-can");
        assert_eq!(event.detail.get("emergency").unwrap(), "Emergency-911");
        assert_eq!(event.detail.get("remarks").unwrap(), "Need immediate assistance!");
    }
}
