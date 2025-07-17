use std::collections::HashMap;
use serde_json::{json, Value};
use chrono::{DateTime, Utc};

pub mod test_utils;

/// Shared test fixtures for CoT library testing.
/// Provides consistent test data across all test suites.
pub struct CoTTestFixtures;

// Standard test timestamps
pub const STANDARD_TIME: &str = "2024-01-15T10:30:00.000Z";
pub const STANDARD_START: &str = "2024-01-15T10:30:00.000Z";
pub const STANDARD_STALE: &str = "2024-01-15T12:30:00.000Z";

// Standard test coordinates (San Francisco)
pub const STANDARD_LAT: f64 = 37.7749;
pub const STANDARD_LON: f64 = -122.4194;
pub const STANDARD_HAE: f64 = 100.5;
pub const STANDARD_CE: f64 = 10.0;
pub const STANDARD_LE: f64 = 5.0;

// Alternative coordinates (New York)
pub const ALT_LAT: f64 = 40.7128;
pub const ALT_LON: f64 = -74.0060;
pub const ALT_HAE: f64 = 50.0;
pub const ALT_CE: f64 = 15.0;
pub const ALT_LE: f64 = 8.0;

impl CoTTestFixtures {
    /// Creates a standard MapItem CoT XML
    pub fn create_map_item_xml(uid: &str) -> String {
        format!(
            r#"<?xml version='1.0' encoding='UTF-8'?>
<event version='2.0' uid='{}' type='a-f-G-U-C' 
       time='{}' 
       start='{}' 
       stale='{}' 
       how='h-g-i-g-o'>
    <point lat='{:.6}' lon='{:.6}' hae='{:.1}' ce='{:.1}' le='{:.1}'/>
    <detail>
        <contact callsign='ALPHA-1' endpoint='192.168.1.100:4242:tcp'/>
        <__group name='Alpha Team' role='Team Leader'/>
        <takv os='31' version='5.4.0' device='ANDROID-001' platform='ATAK'/>
        <status battery='85'/>
        <track course='270.5' speed='15.2'/>
        <precisionlocation geopointsrc='GPS' altsrc='GPS'/>
    </detail>
</event>"#,
            uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
            STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE
        )
    }

    /// Creates a Chat CoT XML
    pub fn create_chat_xml(uid: &str, message: &str) -> String {
        format!(
            r#"<?xml version='1.0' encoding='UTF-8'?>
<event version='2.0' uid='{}' type='b-t-f' 
       time='{}' 
       start='{}' 
       stale='{}' 
       how='h-g-i-g-o'>
    <point lat='{:.6}' lon='{:.6}' hae='{:.1}' ce='{:.1}' le='{:.1}'/>
    <detail>
        <__chat parent='RootContactGroup' groupOwner='false' 
               messageId='{}' chatroom='All Chat Rooms' 
               id='All Chat Rooms' senderCallsign='ALPHA-1'>
            <chatgrp uid0='{}' uid1='All Chat Rooms' id='All Chat Rooms'/>
        </__chat>
        <link uid='{}' type='a-f-G-U-C' relation='p-p'/>
        <remarks source='BAO.F.ATAK.{}' to='All Chat Rooms' time='{}'>{}</remarks>
        <__serverdestination destinations='192.168.1.100:8089:tcp:All Chat Rooms'/>
        <marti>
            <dest callsign='All Chat Rooms'/>
        </marti>
    </detail>
</event>"#,
            uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
            STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE,
            uid, uid, uid, uid, STANDARD_TIME, message
        )
    }

    /// Creates a File sharing CoT XML
    pub fn create_file_share_xml(uid: &str, filename: &str, size_in_bytes: u64) -> String {
        format!(
            r#"<?xml version='1.0' encoding='UTF-8'?>
<event version='2.0' uid='{}' type='b-f-t-f' 
       time='{}' 
       start='{}' 
       stale='{}' 
       how='h-g-i-g-o'>
    <point lat='{:.6}' lon='{:.6}' hae='{:.1}' ce='{:.1}' le='{:.1}'/>
    <detail>
        <fileshare filename='{}' 
                  senderUrl='http://192.168.1.100:8080/files/{}' 
                  sizeInBytes='{}' 
                  sha256hash='abc123def456789012345678901234567890' 
                  senderUid='{}' 
                  senderCallsign='ALPHA-1' 
                  name='Test File'/>
        <contact callsign='ALPHA-1' endpoint='192.168.1.100:4242:tcp'/>
    </detail>
</event>"#,
            uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
            STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE,
            filename, filename, size_in_bytes, uid
        )
    }

    /// Creates an API request CoT XML
    pub fn create_api_xml(uid: &str, endpoint: &str, method: &str) -> String {
        format!(
            r#"<?xml version='1.0' encoding='UTF-8'?>
<event version='2.0' uid='{}' type='t-x-c-t' 
       time='{}' 
       start='{}' 
       stale='{}' 
       how='h-g-i-g-o'>
    <point lat='{:.6}' lon='{:.6}' hae='{:.1}' ce='{:.1}' le='{:.1}'/>
    <detail>
        <TakControl>
            <TakRequest>
                <TakMessage>
                    <TakResponse status='true'/>
                </TakMessage>
            </TakRequest>
        </TakControl>
        <contact callsign='API-CLIENT' endpoint='{}'/>
        <__group name='API Group' role='Client'/>
        <api>
            <endpoint>{}</endpoint>
            <method>{}</method>
            <timestamp>{}</timestamp>
        </api>
    </detail>
</event>"#,
            uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
            STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE,
            endpoint, endpoint, method, STANDARD_TIME
        )
    }

    /// Creates a Generic CoT XML with custom type
    pub fn create_generic_xml(uid: &str, cot_type: &str) -> String {
        format!(
            r#"<?xml version='1.0' encoding='UTF-8'?>
<event version='2.0' uid='{}' type='{}' 
       time='{}' 
       start='{}' 
       stale='{}' 
       how='h-g-i-g-o'>
    <point lat='{:.6}' lon='{:.6}' hae='{:.1}' ce='{:.1}' le='{:.1}'/>
    <detail>
        <contact callsign='GENERIC-1'/>
        <remarks>Generic event for testing</remarks>
        <custom>
            <field1>value1</field1>
            <field2>42</field2>
            <field3>true</field3>
        </custom>
    </detail>
</event>"#,
            uid, cot_type, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
            STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE
        )
    }

    /// Creates expected MapItem document structure
    pub fn create_expected_map_item_document(uid: &str) -> Value {
        json!({
            "_id": uid,
            "w": "a-f-G-U-C",
            "h": STANDARD_LAT,
            "j": STANDARD_LON,
            "k": STANDARD_HAE,
            "l": STANDARD_CE,
            "m": STANDARD_LE,
            "n": "ALPHA-1",
            "o": "Alpha Team",
            "p": "Team Leader",
            "q": Self::parse_timestamp_to_micros(STANDARD_START),
            "r": Self::parse_timestamp_to_micros(STANDARD_STALE),
            "s": 85.0,
            "t": 270.5,
            "u": 15.2,
            "v": "5.4.0",
            "x": 31.0,
            "y": "ANDROID-001",
            "z": "ATAK",
            "aa": "192.168.1.100:4242:tcp",
            "bb": "GPS",
            "cc": "GPS"
        })
    }

    /// Creates expected Chat document structure
    pub fn create_expected_chat_document(uid: &str, message: &str) -> Value {
        json!({
            "_id": uid,
            "e": message,
            "f": "ALPHA-1",
            "g": "All Chat Rooms",
            "h": STANDARD_LAT,
            "j": STANDARD_LON,
            "k": STANDARD_HAE,
            "l": STANDARD_CE,
            "m": STANDARD_LE,
            "q": Self::parse_timestamp_to_micros(STANDARD_START),
            "r": Self::parse_timestamp_to_micros(STANDARD_STALE)
        })
    }

    /// Creates expected File document structure
    pub fn create_expected_file_document(uid: &str, filename: &str, size_in_bytes: u64) -> Value {
        json!({
            "_id": uid,
            "dd": filename,
            "ee": size_in_bytes as f64,
            "ff": "abc123def456789012345678901234567890",
            "gg": format!("http://192.168.1.100:8080/files/{}", filename),
            "hh": uid,
            "ii": "ALPHA-1",
            "jj": "Test File",
            "h": STANDARD_LAT,
            "j": STANDARD_LON,
            "k": STANDARD_HAE,
            "l": STANDARD_CE,
            "m": STANDARD_LE,
            "q": Self::parse_timestamp_to_micros(STANDARD_START),
            "r": Self::parse_timestamp_to_micros(STANDARD_STALE)
        })
    }

    /// Creates expected API document structure
    pub fn create_expected_api_document(uid: &str, endpoint: &str, method: &str) -> Value {
        json!({
            "_id": uid,
            "kk": endpoint,
            "ll": method,
            "mm": Self::parse_timestamp_to_micros(STANDARD_TIME),
            "h": STANDARD_LAT,
            "j": STANDARD_LON,
            "k": STANDARD_HAE,
            "l": STANDARD_CE,
            "m": STANDARD_LE,
            "q": Self::parse_timestamp_to_micros(STANDARD_START),
            "r": Self::parse_timestamp_to_micros(STANDARD_STALE)
        })
    }

    /// Creates expected Generic document structure
    pub fn create_expected_generic_document(uid: &str, cot_type: &str) -> Value {
        json!({
            "_id": uid,
            "w": cot_type,
            "h": STANDARD_LAT,
            "j": STANDARD_LON,
            "k": STANDARD_HAE,
            "l": STANDARD_CE,
            "m": STANDARD_LE,
            "q": Self::parse_timestamp_to_micros(STANDARD_START),
            "r": Self::parse_timestamp_to_micros(STANDARD_STALE),
            "nn": {
                "contact": {"callsign": "GENERIC-1"},
                "remarks": "Generic event for testing",
                "custom": {
                    "field1": "value1",
                    "field2": "42",
                    "field3": "true"
                }
            }
        })
    }

    /// Helper method to parse timestamp to microseconds
    fn parse_timestamp_to_micros(timestamp: &str) -> u64 {
        match DateTime::parse_from_rfc3339(timestamp) {
            Ok(dt) => {
                let utc_dt = dt.with_timezone(&Utc);
                (utc_dt.timestamp() as u64) * 1_000_000 + (utc_dt.timestamp_subsec_micros() as u64)
            }
            Err(_) => 0,
        }
    }

    /// Creates a timestamp that is X seconds in the future from the standard time
    pub fn create_future_timestamp(seconds_from_now: i64) -> String {
        let base = DateTime::parse_from_rfc3339(STANDARD_TIME).unwrap();
        let future = base + chrono::Duration::seconds(seconds_from_now);
        future.to_rfc3339()
    }

    /// Creates a timestamp that is X seconds in the past from the standard time
    pub fn create_past_timestamp(seconds_ago: i64) -> String {
        let base = DateTime::parse_from_rfc3339(STANDARD_TIME).unwrap();
        let past = base - chrono::Duration::seconds(seconds_ago);
        past.to_rfc3339()
    }

    /// Generates a unique UID with a given prefix
    pub fn generate_uid(prefix: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() % 1000;
        format!("{}-{}-{}", prefix, now, nanos)
    }
}

/// Common CoT types for testing
pub mod cot_types {
    pub const MAP_ITEM: &str = "a-f-G-U-C";
    pub const CHAT: &str = "b-t-f";
    pub const FILE_SHARE: &str = "b-f-t-f";
    pub const API: &str = "t-x-c-t";
    pub const EMERGENCY: &str = "a-u-emergency-g";
    pub const SENSOR: &str = "a-u-S";
    pub const AIRCRAFT: &str = "a-u-A";
    pub const GROUND: &str = "a-u-G";
    pub const GENERIC: &str = "x-custom-type";
}

/// Common UIDs for testing
pub mod test_uids {
    pub const MAP_ITEM_1: &str = "MAP-ITEM-001";
    pub const MAP_ITEM_2: &str = "MAP-ITEM-002";
    pub const CHAT_1: &str = "CHAT-MESSAGE-001";
    pub const CHAT_2: &str = "CHAT-MESSAGE-002";
    pub const FILE_1: &str = "FILE-SHARE-001";
    pub const FILE_2: &str = "FILE-SHARE-002";
    pub const API_1: &str = "API-REQUEST-001";
    pub const API_2: &str = "API-REQUEST-002";
    pub const GENERIC_1: &str = "GENERIC-EVENT-001";
    pub const GENERIC_2: &str = "GENERIC-EVENT-002";
}

/// Test data sets for parameterized tests
pub mod test_data_sets {
    use super::*;

    pub const COORDINATE_TEST_DATA: &[(f64, f64, f64, f64, f64)] = &[
        (STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE),
        (ALT_LAT, ALT_LON, ALT_HAE, ALT_CE, ALT_LE),
        (0.0, 0.0, 0.0, 0.0, 0.0),
        (90.0, 180.0, 9999.9, 999.9, 999.9),
        (-90.0, -180.0, -9999.9, 0.1, 0.1),
    ];

    pub const CALLSIGN_TEST_DATA: &[&str] = &[
        "ALPHA-1", "BRAVO-2", "CHARLIE-3", "DELTA-4", "ECHO-5"
    ];

    pub const MESSAGE_TEST_DATA: &[&str] = &[
        "Hello World", "Test message", "Emergency situation",
        "All clear", "Status update", "Mission briefing"
    ];

    pub const FILENAME_TEST_DATA: &[&str] = &[
        "document.pdf", "image.jpg", "video.mp4", "audio.wav", "data.zip"
    ];

    pub const FILESIZE_TEST_DATA: &[u64] = &[
        1024, 1048576, 5242880, 10485760, 104857600
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_creation() {
        let xml = CoTTestFixtures::create_map_item_xml("TEST-001");
        assert!(xml.contains("TEST-001"));
        assert!(xml.contains("a-f-G-U-C"));
        assert!(xml.contains(&STANDARD_LAT.to_string()));
    }

    #[test]
    fn test_expected_document_creation() {
        let doc = CoTTestFixtures::create_expected_map_item_document("TEST-001");
        assert_eq!(doc["_id"], "TEST-001");
        assert_eq!(doc["w"], "a-f-G-U-C");
        assert_eq!(doc["h"], STANDARD_LAT);
    }

    #[test]
    fn test_timestamp_parsing() {
        let micros = CoTTestFixtures::parse_timestamp_to_micros(STANDARD_TIME);
        assert!(micros > 0);
    }

    #[test]
    fn test_uid_generation() {
        let uid1 = CoTTestFixtures::generate_uid("TEST");
        let uid2 = CoTTestFixtures::generate_uid("TEST");
        assert_ne!(uid1, uid2);
        assert!(uid1.starts_with("TEST-"));
        assert!(uid2.starts_with("TEST-"));
    }
}