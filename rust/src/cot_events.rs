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

impl Point {
    /// Creates a new builder for constructing Point instances.
    pub fn builder() -> PointBuilder {
        PointBuilder::new()
    }

    /// Creates a new Point with specified coordinates.
    pub fn new(lat: f64, lon: f64, hae: f64) -> Self {
        Self {
            lat,
            lon,
            hae,
            ce: 999999.0,
            le: 999999.0,
        }
    }

    /// Creates a new Point with coordinates and accuracy information.
    pub fn with_accuracy(lat: f64, lon: f64, hae: f64, ce: f64, le: f64) -> Self {
        Self {
            lat,
            lon,
            hae,
            ce,
            le,
        }
    }
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
    /// Creates a new builder for constructing CotEvent instances.
    ///
    /// # Examples
    ///
    /// ```
    /// use ditto_cot::cot_events::CotEvent;
    ///
    /// let event = CotEvent::builder()
    ///     .uid("USER-123")
    ///     .event_type("a-f-G-U-C")
    ///     .location(34.12345, -118.12345, 150.0)
    ///     .callsign("ALPHA-1")
    ///     .team("Cyan")
    ///     .build();
    ///
    /// assert_eq!(event.uid, "USER-123");
    /// assert_eq!(event.point.lat, 34.12345);
    /// ```
    pub fn builder() -> CotEventBuilder {
        CotEventBuilder::new()
    }

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
        // Pretty-print XML by manual string construction
        let lat = format_cot_float(self.point.lat);
        let lon = format_cot_float(self.point.lon);
        let hae = format_cot_float(self.point.hae);
        let ce = format_cot_float(self.point.ce);
        let le = format_cot_float(self.point.le);
        let mut xml = String::new();
        xml.push_str("<event version=\"");
        xml.push_str(self.version.as_str());
        xml.push('"');
        xml.push('\n');
        xml.push_str("              type=\"");
        xml.push_str(self.event_type.as_str());
        xml.push('"');
        xml.push('\n');
        xml.push_str("              uid=\"");
        xml.push_str(self.uid.as_str());
        xml.push('"');
        xml.push('\n');
        xml.push_str("              time=\"");
        // Format UTC timestamps with Z suffix instead of +00:00
        xml.push_str(&self.time.to_rfc3339().replace("+00:00", "Z"));
        xml.push('"');
        xml.push('\n');
        xml.push_str("              start=\"");
        xml.push_str(&self.start.to_rfc3339().replace("+00:00", "Z"));
        xml.push('"');
        xml.push('\n');
        xml.push_str("              stale=\"");
        xml.push_str(&self.stale.to_rfc3339().replace("+00:00", "Z"));
        xml.push('"');
        xml.push('\n');
        xml.push_str("              how=\"");
        xml.push_str(self.how.as_str());
        xml.push('"');
        xml.push('>');
        xml.push('\n');
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
        xml.push_str("\"/>");
        xml.push('\n');
        xml.push_str("            ");
        xml.push_str(&self.detail);
        xml.push('\n');
        xml.push_str("        </event>");
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
                        log::trace!("Found point element");
                        for attr in e.attributes() {
                            let attr = attr?;
                            let value = attr.unescape_value()?;
                            match attr.key.0 {
                                b"lat" => {
                                    let lat_val: f64 =
                                        value.parse().map_err(|e| CotError::InvalidNumeric {
                                            field: "lat".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        })?;
                                    log::trace!("Parsed lat: {}", lat_val);
                                    event.point.lat = lat_val;
                                }
                                b"lon" => {
                                    let lon_val: f64 =
                                        value.parse().map_err(|e| CotError::InvalidNumeric {
                                            field: "lon".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        })?;
                                    log::trace!("Parsed lon: {}", lon_val);
                                    event.point.lon = lon_val;
                                }
                                b"hae" => {
                                    event.point.hae = value.parse::<f64>().map_err(|e| {
                                        CotError::InvalidNumeric {
                                            field: "hae".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        }
                                    })?
                                }
                                b"ce" => {
                                    event.point.ce = value.parse::<f64>().map_err(|e| {
                                        CotError::InvalidNumeric {
                                            field: "ce".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        }
                                    })?
                                }
                                b"le" => {
                                    event.point.le = value.parse::<f64>().map_err(|e| {
                                        CotError::InvalidNumeric {
                                            field: "le".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        }
                                    })?
                                }
                                _ => {}
                            }
                        }
                        log::trace!(
                            "After parsing point: lat={}, lon={}",
                            event.point.lat,
                            event.point.lon
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
                        log::trace!("Found point element (Empty)");
                        for attr in e.attributes() {
                            let attr = attr?;
                            let value = attr.unescape_value()?;
                            match attr.key.0 {
                                b"lat" => {
                                    let lat_val: f64 =
                                        value.parse().map_err(|e| CotError::InvalidNumeric {
                                            field: "lat".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        })?;
                                    log::trace!("Parsed lat (Empty): {}", lat_val);
                                    event.point.lat = lat_val;
                                }
                                b"lon" => {
                                    let lon_val: f64 =
                                        value.parse().map_err(|e| CotError::InvalidNumeric {
                                            field: "lon".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        })?;
                                    log::trace!("Parsed lon (Empty): {}", lon_val);
                                    event.point.lon = lon_val;
                                }
                                b"hae" => {
                                    event.point.hae = value.parse::<f64>().map_err(|e| {
                                        CotError::InvalidNumeric {
                                            field: "hae".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        }
                                    })?
                                }
                                b"ce" => {
                                    event.point.ce = value.parse::<f64>().map_err(|e| {
                                        CotError::InvalidNumeric {
                                            field: "ce".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        }
                                    })?
                                }
                                b"le" => {
                                    event.point.le = value.parse::<f64>().map_err(|e| {
                                        CotError::InvalidNumeric {
                                            field: "le".to_string(),
                                            value: value.to_string(),
                                            source: Box::new(e),
                                        }
                                    })?
                                }
                                _ => {}
                            }
                        }
                        log::trace!(
                            "After parsing point (Empty): lat={}, lon={}",
                            event.point.lat,
                            event.point.lon
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
            .map_err(|_| CotError::InvalidDateTime {
                field: "datetime".to_string(),
                value: s.to_string(),
            })
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

/// Builder for constructing CotEvent instances with a fluent API.
///
/// This builder provides an ergonomic way to create CotEvent instances
/// with method chaining and sensible defaults.
#[derive(Debug, Clone)]
pub struct CotEventBuilder {
    event: CotEvent,
}

impl CotEventBuilder {
    /// Creates a new CotEventBuilder with default values.
    pub fn new() -> Self {
        Self {
            event: CotEvent::default(),
        }
    }

    /// Sets the unique identifier for the event.
    pub fn uid<S: Into<String>>(mut self, uid: S) -> Self {
        self.event.uid = uid.into();
        self
    }

    /// Sets the event type (e.g., "a-f-G-U-C" for military ground unit).
    pub fn event_type<S: Into<String>>(mut self, event_type: S) -> Self {
        self.event.event_type = event_type.into();
        self
    }

    /// Sets the location coordinates.
    pub fn location(mut self, lat: f64, lon: f64, hae: f64) -> Self {
        self.event.point.lat = lat;
        self.event.point.lon = lon;
        self.event.point.hae = hae;
        self
    }

    /// Sets the location with accuracy information.
    pub fn location_with_accuracy(
        mut self,
        lat: f64,
        lon: f64,
        hae: f64,
        ce: f64,
        le: f64,
    ) -> Self {
        self.event.point.lat = lat;
        self.event.point.lon = lon;
        self.event.point.hae = hae;
        self.event.point.ce = ce;
        self.event.point.le = le;
        self
    }

    /// Sets event timing with explicit timestamps.
    pub fn timing(
        mut self,
        time: DateTime<Utc>,
        start: DateTime<Utc>,
        stale: DateTime<Utc>,
    ) -> Self {
        self.event.time = time;
        self.event.start = start;
        self.event.stale = stale;
        self
    }

    /// Sets the stale time as duration from now.
    pub fn stale_in(mut self, duration: chrono::Duration) -> Self {
        self.event.stale = Utc::now() + duration;
        self
    }

    /// Sets how the event was generated.
    pub fn how<S: Into<String>>(mut self, how: S) -> Self {
        self.event.how = how.into();
        self
    }

    /// Sets the detail XML content.
    pub fn detail<S: Into<String>>(mut self, detail: S) -> Self {
        self.event.detail = detail.into();
        self
    }

    /// Convenience method to set callsign in detail section.
    pub fn callsign<S: Into<String>>(mut self, callsign: S) -> Self {
        let callsign = callsign.into();
        self.event.detail = format!("<detail><contact callsign=\"{}\"/></detail>", callsign);
        self
    }

    /// Convenience method to set team in detail section.
    pub fn team<S: Into<String>>(mut self, team: S) -> Self {
        let team = team.into();
        self.event.detail = format!("<detail><__group name=\"{}\"/></detail>", team);
        self
    }

    /// Convenience method to set both callsign and team.
    pub fn callsign_and_team<S1: Into<String>, S2: Into<String>>(
        mut self,
        callsign: S1,
        team: S2,
    ) -> Self {
        let callsign = callsign.into();
        let team = team.into();
        self.event.detail = format!(
            "<detail><contact callsign=\"{}\"/><__group name=\"{}\"/></detail>",
            callsign, team
        );
        self
    }

    /// Builds the final CotEvent instance.
    pub fn build(self) -> CotEvent {
        self.event
    }
}

impl Default for CotEventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing Point instances with a fluent API.
///
/// This builder provides an ergonomic way to create Point instances
/// with method chaining and sensible defaults.
#[derive(Debug, Clone)]
pub struct PointBuilder {
    point: Point,
}

impl PointBuilder {
    /// Creates a new PointBuilder with default values.
    pub fn new() -> Self {
        Self {
            point: Point::default(),
        }
    }

    /// Sets the latitude in decimal degrees (WGS84).
    pub fn lat(mut self, lat: f64) -> Self {
        self.point.lat = lat;
        self
    }

    /// Sets the longitude in decimal degrees (WGS84).
    pub fn lon(mut self, lon: f64) -> Self {
        self.point.lon = lon;
        self
    }

    /// Sets the Height Above Ellipsoid in meters.
    pub fn hae(mut self, hae: f64) -> Self {
        self.point.hae = hae;
        self
    }

    /// Sets the Circular Error in meters (horizontal accuracy).
    pub fn ce(mut self, ce: f64) -> Self {
        self.point.ce = ce;
        self
    }

    /// Sets the Linear Error in meters (vertical accuracy).
    pub fn le(mut self, le: f64) -> Self {
        self.point.le = le;
        self
    }

    /// Sets coordinates in one call.
    pub fn coordinates(mut self, lat: f64, lon: f64, hae: f64) -> Self {
        self.point.lat = lat;
        self.point.lon = lon;
        self.point.hae = hae;
        self
    }

    /// Sets accuracy information in one call.
    pub fn accuracy(mut self, ce: f64, le: f64) -> Self {
        self.point.ce = ce;
        self.point.le = le;
        self
    }

    /// Builds the final Point instance.
    pub fn build(self) -> Point {
        self.point
    }
}

impl Default for PointBuilder {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_builder_basic_usage() {
        let event = CotEvent::builder()
            .uid("TEST-123")
            .event_type("a-f-G-U-C")
            .location(34.12345, -118.12345, 150.0)
            .callsign("ALPHA-1")
            .build();

        assert_eq!(event.uid, "TEST-123");
        assert_eq!(event.event_type, "a-f-G-U-C");
        assert_eq!(event.point.lat, 34.12345);
        assert_eq!(event.point.lon, -118.12345);
        assert_eq!(event.point.hae, 150.0);
        assert_eq!(
            event.detail,
            "<detail><contact callsign=\"ALPHA-1\"/></detail>"
        );
    }

    #[test]
    fn test_builder_with_team() {
        let event = CotEvent::builder()
            .uid("TEST-456")
            .callsign_and_team("BRAVO-2", "Blue")
            .location_with_accuracy(35.0, -119.0, 200.0, 5.0, 10.0)
            .build();

        assert_eq!(event.uid, "TEST-456");
        assert_eq!(event.point.ce, 5.0);
        assert_eq!(event.point.le, 10.0);
        assert_eq!(
            event.detail,
            "<detail><contact callsign=\"BRAVO-2\"/><__group name=\"Blue\"/></detail>"
        );
    }

    #[test]
    fn test_builder_stale_duration() {
        let event = CotEvent::builder()
            .uid("TEST-789")
            .stale_in(chrono::Duration::minutes(10))
            .build();

        assert_eq!(event.uid, "TEST-789");
        assert!(event.stale > event.time);
        let duration = event.stale - event.time;
        assert!(duration >= chrono::Duration::minutes(9)); // Allow for some execution time
        assert!(duration <= chrono::Duration::minutes(11));
    }

    #[test]
    fn test_builder_custom_detail() {
        let event = CotEvent::builder()
            .uid("TEST-CUSTOM")
            .detail("<detail><custom field=\"value\"/></detail>")
            .build();

        assert_eq!(event.uid, "TEST-CUSTOM");
        assert_eq!(event.detail, "<detail><custom field=\"value\"/></detail>");
    }

    #[test]
    fn test_point_builder() {
        let point = Point::builder()
            .lat(34.12345)
            .lon(-118.12345)
            .hae(150.0)
            .ce(5.0)
            .le(10.0)
            .build();

        assert_eq!(point.lat, 34.12345);
        assert_eq!(point.lon, -118.12345);
        assert_eq!(point.hae, 150.0);
        assert_eq!(point.ce, 5.0);
        assert_eq!(point.le, 10.0);
    }

    #[test]
    fn test_point_builder_coordinates_and_accuracy() {
        let point = Point::builder()
            .coordinates(35.0, -119.0, 200.0)
            .accuracy(3.0, 8.0)
            .build();

        assert_eq!(point.lat, 35.0);
        assert_eq!(point.lon, -119.0);
        assert_eq!(point.hae, 200.0);
        assert_eq!(point.ce, 3.0);
        assert_eq!(point.le, 8.0);
    }

    #[test]
    fn test_point_constructors() {
        let point1 = Point::new(34.0, -118.0, 100.0);
        assert_eq!(point1.lat, 34.0);
        assert_eq!(point1.lon, -118.0);
        assert_eq!(point1.hae, 100.0);
        assert_eq!(point1.ce, 999999.0);
        assert_eq!(point1.le, 999999.0);

        let point2 = Point::with_accuracy(35.0, -119.0, 200.0, 5.0, 10.0);
        assert_eq!(point2.lat, 35.0);
        assert_eq!(point2.lon, -119.0);
        assert_eq!(point2.hae, 200.0);
        assert_eq!(point2.ce, 5.0);
        assert_eq!(point2.le, 10.0);
    }
}
