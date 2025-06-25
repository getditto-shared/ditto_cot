//! Common TAK CoT (Cursor on Target) event templates and utilities.
//!
//! This module provides pre-defined templates for common CoT message types used in the TAK ecosystem.
//! Each template includes the standard fields and can be customized as needed.

use chrono::{DateTime, Utc};

use crate::error::CotError;

use crate::xml_utils::format_cot_float;
use quick_xml::events::Event;
use quick_xml::Reader;
use uuid::Uuid;

/// Represents a Cursor on Target (CoT) event with all standard fields.
///
/// A CoT event represents a unit's status, location, or communication in a tactical network.
/// It contains metadata about the event, location information, and a flexible detail section.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CotEvent {
    /// CoT protocol version (e.g., "2.0")
    pub version: String,

    /// Unique identifier for the event
    pub uid: String,

    /// Event type code (e.g., "a-f-G-U-C" for military ground unit)
    pub event_type: String,

    /// When the event was generated
    pub time: DateTime<Utc>,

    /// When the event becomes valid
    pub start: DateTime<Utc>,

    /// When the event expires
    pub stale: DateTime<Utc>,

    /// How the event was generated (e.g., "h-g-i-g-o" for human-generated)
    pub how: String,

    /// Geographic location and accuracy information
    pub point: Point,

    /// Raw XML for the <detail> element
    pub detail: String,
}

/// Represents a geographic point with elevation and accuracy information.
///
/// This is used to specify locations in the CoT protocol with associated
/// accuracy metrics for different dimensions.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Point {
    /// Latitude in decimal degrees (WGS84)
    pub lat: f64,

    /// Longitude in decimal degrees (WGS84)
    pub lon: f64,

    /// Height Above Ellipsoid in meters
    pub hae: f64,

    /// Circular Error in meters (horizontal accuracy)
    pub ce: f64,

    /// Linear Error in meters (vertical accuracy)
    pub le: f64,
}

impl Default for CotEvent {
    fn default() -> Self {
        let now = Utc::now();
        let uid = Uuid::new_v4().to_string();

        Self {
            version: "2.0".to_string(),
            uid: uid.clone(),
            event_type: "a-f-G-U-C".to_string(), // Default: Military Ground Unit
            time: now,
            start: now,
            stale: now + chrono::Duration::minutes(5), // Default 5 minute staleness
            how: "h-g-i-g-o".to_string(),              // Human-input GPS general
            point: Point {
                lat: 0.0,
                lon: 0.0,
                hae: 0.0,
                ce: 999999.0, // Default high error values
                le: 999999.0,
            },
            detail: String::new(),
        }
    }
}

impl CotEvent {
    /// Returns a reference to the event's point data
    pub fn point(&self) -> &Point {
        &self.point
    }

    /// Returns a reference to the event's UID
    pub fn uid(&self) -> &str {
        &self.uid
    }

    /// Converts the CotEvent to an XML string
    pub fn to_xml(&self) -> Result<String, CotError> {
        // Pretty-print XML by manual string construction to match the expected format
        let lat = format_cot_float(self.point.lat);
        let lon = format_cot_float(self.point.lon);
        let hae = format_cot_float(self.point.hae);
        let ce = format_cot_float(self.point.ce);
        let le = format_cot_float(self.point.le);
        let mut xml = String::new();
        xml.push_str("        <event version=\"");
        xml.push_str(self.version.as_str());
        xml.push_str("\"\n");
        xml.push_str("              type=\"");
        xml.push_str(self.event_type.as_str());
        xml.push_str("\"\n");
        xml.push_str("              uid=\"");
        xml.push_str(self.uid.as_str());
        xml.push_str("\"\n");
        xml.push_str("              time=\"");
        xml.push_str(self.time.to_rfc3339().as_str());
        xml.push_str("\"\n");
        xml.push_str("              start=\"");
        xml.push_str(self.start.to_rfc3339().as_str());
        xml.push_str("\"\n");
        xml.push_str("              stale=\"");
        xml.push_str(self.stale.to_rfc3339().as_str());
        xml.push_str("\"\n");
        xml.push_str("              how=\"");
        xml.push_str(self.how.as_str());
        xml.push_str("\"\n");
        xml.push_str("              lat=\"");
        xml.push_str(&lat);
        xml.push_str("\"\n");
        xml.push_str("              lon=\"");
        xml.push_str(&lon);
        xml.push_str("\"\n");
        xml.push_str("              hae=\"");
        xml.push_str(&hae);
        xml.push_str("\"\n");
        xml.push_str("              ce=\"");
        xml.push_str(&ce);
        xml.push_str("\"\n");
        xml.push_str("              le=\"");
        xml.push_str(&le);
        xml.push_str("\">\n");
        xml.push_str("            <point lat=\"");
        xml.push_str(&lat);
        xml.push_str("\" lon=\"");
        xml.push_str(&lon);
        xml.push_str("\" hae=\"");
        xml.push_str(&hae);
        xml.push_str("\" ce=\"");
        xml.push_str(&ce);
        xml.push_str("\" le=\"");
        xml.push_str(&le);
        xml.push_str("\"/>\n");
        if !self.detail.is_empty() {
            if self.detail.trim_start().starts_with("<detail") {
                // Insert detail block at correct indentation
                let detail_lines: Vec<&str> = self.detail.lines().collect();
                for line in detail_lines {
                    xml.push_str("    ");
                    xml.push_str(line.trim());
                    xml.push('\n');
                }
            } else {
                xml.push_str("    <detail>\n");
                xml.push_str(&self.detail);
                xml.push_str("\n    </detail>\n");
            }
        }
        xml.push_str("        </event>\n");
        Ok(xml)
    }

    /// Parses a CoT XML string into a CotEvent
    pub fn from_xml(xml: &str) -> Result<Self, CotError> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut event = CotEvent::default();
        let mut buf = Vec::new();
        let mut in_detail = false;
        let mut detail_buf = Vec::new();

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
                                    let lat_val = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid lat".to_string())
                                    })?;
                                    println!("Parsed lat: {}", lat_val);
                                    event.point.lat = lat_val;
                                }
                                b"lon" => {
                                    let lon_val = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid lon".to_string())
                                    })?;
                                    println!("Parsed lon: {}", lon_val);
                                    event.point.lon = lon_val;
                                }
                                b"hae" => {
                                    event.point.hae = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid hae".to_string())
                                    })?
                                }
                                b"ce" => {
                                    event.point.ce = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid ce".to_string())
                                    })?
                                }
                                b"le" => {
                                    event.point.le = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid le".to_string())
                                    })?
                                }
                                _ => {}
                            }
                        }
                        println!(
                            "After parsing point: lat={}, lon={}",
                            event.point.lat, event.point.lon
                        );
                    }
                    b"detail" => {
                        // Capture the entire <detail>...</detail> block as a string
                        let detail_start = reader.buffer_position() - e.name().0.len() - 2; // -2 for < and >
                        let mut depth = 1;
                        let mut detail_end = detail_start;

                        loop {
                            buf.clear();
                            match reader.read_event_into(&mut buf) {
                                Ok(Event::Start(ref e2)) if e2.name().as_ref() == b"detail" => {
                                    depth += 1
                                }
                                Ok(Event::End(ref e2)) if e2.name().as_ref() == b"detail" => {
                                    depth -= 1;
                                    if depth == 0 {
                                        detail_end = reader.buffer_position();
                                        break;
                                    }
                                }
                                Ok(Event::Eof) => break,
                                Ok(_) => {}
                                Err(_) => break,
                            }
                        }
                        let xml_bytes = xml.as_bytes();
                        let detail_xml = &xml_bytes[detail_start..detail_end];
                        // Normalize whitespace: remove newlines, carriage returns, and trim
                        let detail_str = String::from_utf8_lossy(detail_xml).to_string();
                        event.detail = detail_str;
                    }
                    _ => {}
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
                                    let lat_val = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid lat".to_string())
                                    })?;
                                    println!("Parsed lat (Empty): {}", lat_val);
                                    event.point.lat = lat_val;
                                }
                                b"lon" => {
                                    let lon_val = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid lon".to_string())
                                    })?;
                                    println!("Parsed lon (Empty): {}", lon_val);
                                    event.point.lon = lon_val;
                                }
                                b"hae" => {
                                    event.point.hae = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid hae".to_string())
                                    })?
                                }
                                b"ce" => {
                                    event.point.ce = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid ce".to_string())
                                    })?
                                }
                                b"le" => {
                                    event.point.le = value.parse().map_err(|_| {
                                        CotError::InvalidFormat("Invalid le".to_string())
                                    })?
                                }
                                _ => {}
                            }
                        }
                        println!(
                            "After parsing point (Empty): lat={}, lon={}",
                            event.point.lat, event.point.lon
                        );
                    }
                }
                Ok(Event::End(e)) => match e.name().as_ref() {
                    b"detail" => {
                        in_detail = false;
                        event.detail = String::from_utf8_lossy(&detail_buf)
                            .replace('\n', "")
                            .replace("\r", "")
                            .trim()
                            .to_string();
                    }
                    b"event" => {
                        // End of event element, we're done
                        return Ok(event);
                    }
                    _ => {}
                },
                Ok(Event::Text(e)) => {
                    if in_detail {
                        detail_buf.extend_from_slice(e.unescape()?.as_bytes());
                    }
                }
                Ok(Event::CData(e)) => {
                    if in_detail {
                        detail_buf.extend_from_slice(e.as_ref());
                    }
                }
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
        Self {
            version: "2.0".to_string(),
            uid: uid.to_string(),
            event_type: "a-f-G-U-C".to_string(),
            time: now,
            start: now,
            stale: now + chrono::Duration::minutes(5),
            how: "h-g-i-g-o".to_string(),
            point: Point {
                lat,
                lon,
                hae,
                ce: 10.0,
                le: 10.0,
            },
            detail: format!("location update: callsign={}, team={}", callsign, team),
        }
    }

    /// Creates a new chat message event
    pub fn new_chat_message(
        sender_uid: &str,
        sender_callsign: &str,
        message: &str,
        chatroom: &str,
        _chat_group_uid: &str,
    ) -> Self {
        let now = Utc::now();
        let uid = format!("Chat-{}-", sender_uid);
        Self {
            version: "2.0".to_string(),
            uid,
            event_type: "b-t-f".to_string(),
            time: now,
            start: now,
            stale: now + chrono::Duration::minutes(5),
            how: "h-g-i-g-o".to_string(),
            point: Point::default(),
            detail: format!(
                "<detail>chat from={} room={} msg={}</detail>",
                sender_callsign, chatroom, message
            ),
        }
    }

    /// Creates a new emergency event
    pub fn new_emergency(
        uid: &str,
        _callsign: &str,
        lat: f64,
        lon: f64,
        emergency_type: &str,
        message: &str,
    ) -> Self {
        let now = Utc::now();
        Self {
            version: "2.0".to_string(),
            uid: uid.to_string(),
            event_type: "b-a-o-can".to_string(),
            time: now,
            start: now,
            stale: now + chrono::Duration::minutes(5),
            how: "h-g-i-g-o".to_string(),
            point: Point {
                lat,
                lon,
                hae: 0.0,
                ce: 10.0,
                le: 10.0,
            },
            detail: format!(
                "<detail>emergency: type={} msg={}</detail>",
                emergency_type, message
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_update_creation() {
        let event = CotEvent::new_location_update(
            "USER-123", "ALPHA-1", "Cyan", 34.12345, -118.12345, 150.0,
        );

        assert_eq!(event.uid, "USER-123");
        assert_eq!(event.event_type, "a-f-G-U-C");
        assert_eq!(event.point.lat, 34.12345);
        assert_eq!(event.point.lon, -118.12345);
        assert_eq!(event.point.hae, 150.0);
        assert_eq!(event.detail, "location update: callsign=ALPHA-1, team=Cyan");
    }

    #[test]
    fn test_chat_message_creation() {
        let event = CotEvent::new_chat_message(
            "USER-123",
            "ALPHA-1",
            "Test message",
            "All Chat Rooms",
            "All Chat Rooms",
        );

        assert_eq!(event.uid, "Chat-USER-123-");
        assert_eq!(event.event_type, "b-t-f");
        assert_eq!(event.point.lat, 0.0);
        assert_eq!(event.point.lon, 0.0);
        assert_eq!(event.point.hae, 0.0);
        assert_eq!(
            event.detail,
            "<detail>chat from=ALPHA-1 room=All Chat Rooms msg=Test message</detail>"
        );
    }

    #[test]
    fn test_emergency_creation() {
        let event = CotEvent::new_emergency(
            "USER-123",
            "ALPHA-1",
            34.12345,
            -118.12345,
            "Emergency-911",
            "Need immediate assistance!",
        );

        assert_eq!(event.uid, "USER-123");
        assert_eq!(event.event_type, "b-a-o-can");
        assert_eq!(
            event.detail,
            "<detail>emergency: type=Emergency-911 msg=Need immediate assistance!</detail>"
        );
    }
}
