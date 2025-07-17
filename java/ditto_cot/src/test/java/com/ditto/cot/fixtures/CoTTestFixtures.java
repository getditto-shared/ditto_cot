package com.ditto.cot.fixtures;

import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.HashMap;
import java.util.Map;

/**
 * Shared test fixtures for CoT library testing.
 * Provides consistent test data across all test suites.
 */
public class CoTTestFixtures {
    
    // Standard test timestamps
    public static final String STANDARD_TIME = "2024-01-15T10:30:00.000Z";
    public static final String STANDARD_START = "2024-01-15T10:30:00.000Z";
    public static final String STANDARD_STALE = "2024-01-15T12:30:00.000Z";
    
    // Standard test coordinates (San Francisco)
    public static final double STANDARD_LAT = 37.7749;
    public static final double STANDARD_LON = -122.4194;
    public static final double STANDARD_HAE = 100.5;
    public static final double STANDARD_CE = 10.0;
    public static final double STANDARD_LE = 5.0;
    
    // Alternative coordinates (New York)
    public static final double ALT_LAT = 40.7128;
    public static final double ALT_LON = -74.0060;
    public static final double ALT_HAE = 50.0;
    public static final double ALT_CE = 15.0;
    public static final double ALT_LE = 8.0;
    
    /**
     * Creates a standard MapItem CoT XML
     */
    public static String createMapItemXml(String uid) {
        return String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='a-f-G-U-C' 
                   time='%s' 
                   start='%s' 
                   stale='%s' 
                   how='h-g-i-g-o'>
                <point lat='%.6f' lon='%.6f' hae='%.1f' ce='%.1f' le='%.1f'/>
                <detail>
                    <contact callsign='ALPHA-1' endpoint='192.168.1.100:4242:tcp'/>
                    <__group name='Alpha Team' role='Team Leader'/>
                    <takv os='31' version='5.4.0' device='ANDROID-001' platform='ATAK'/>
                    <status battery='85'/>
                    <track course='270.5' speed='15.2'/>
                    <precisionlocation geopointsrc='GPS' altsrc='GPS'/>
                </detail>
            </event>
            """, uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE, 
                STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE);
    }
    
    /**
     * Creates a Chat CoT XML
     */
    public static String createChatXml(String uid, String message) {
        return String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='b-t-f' 
                   time='%s' 
                   start='%s' 
                   stale='%s' 
                   how='h-g-i-g-o'>
                <point lat='%.6f' lon='%.6f' hae='%.1f' ce='%.1f' le='%.1f'/>
                <detail>
                    <__chat parent='RootContactGroup' groupOwner='false' 
                           messageId='%s' chatroom='All Chat Rooms' 
                           id='All Chat Rooms' senderCallsign='ALPHA-1'>
                        <chatgrp uid0='%s' uid1='All Chat Rooms' id='All Chat Rooms'/>
                    </__chat>
                    <link uid='%s' type='a-f-G-U-C' relation='p-p'/>
                    <remarks source='BAO.F.ATAK.%s' to='All Chat Rooms' time='%s'>%s</remarks>
                    <__serverdestination destinations='192.168.1.100:8089:tcp:All Chat Rooms'/>
                    <marti>
                        <dest callsign='All Chat Rooms'/>
                    </marti>
                </detail>
            </event>
            """, uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
                STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE,
                uid, uid, uid, uid, STANDARD_TIME, message);
    }
    
    /**
     * Creates a File sharing CoT XML
     */
    public static String createFileShareXml(String uid, String filename, long sizeInBytes) {
        return String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='b-f-t-f' 
                   time='%s' 
                   start='%s' 
                   stale='%s' 
                   how='h-g-i-g-o'>
                <point lat='%.6f' lon='%.6f' hae='%.1f' ce='%.1f' le='%.1f'/>
                <detail>
                    <fileshare filename='%s' 
                              senderUrl='http://192.168.1.100:8080/files/%s' 
                              sizeInBytes='%d' 
                              sha256hash='abc123def456789012345678901234567890' 
                              senderUid='%s' 
                              senderCallsign='ALPHA-1' 
                              name='Test File'/>
                    <contact callsign='ALPHA-1' endpoint='192.168.1.100:4242:tcp'/>
                </detail>
            </event>
            """, uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
                STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE,
                filename, filename, sizeInBytes, uid);
    }
    
    /**
     * Creates an API request CoT XML
     */
    public static String createApiXml(String uid, String endpoint, String method) {
        return String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='t-x-c-t' 
                   time='%s' 
                   start='%s' 
                   stale='%s' 
                   how='h-g-i-g-o'>
                <point lat='%.6f' lon='%.6f' hae='%.1f' ce='%.1f' le='%.1f'/>
                <detail>
                    <TakControl>
                        <TakRequest>
                            <TakMessage>
                                <TakResponse status='true'/>
                            </TakMessage>
                        </TakRequest>
                    </TakControl>
                    <contact callsign='API-CLIENT' endpoint='%s'/>
                    <__group name='API Group' role='Client'/>
                    <api>
                        <endpoint>%s</endpoint>
                        <method>%s</method>
                        <timestamp>%s</timestamp>
                    </api>
                </detail>
            </event>
            """, uid, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
                STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE,
                endpoint, endpoint, method, STANDARD_TIME);
    }
    
    /**
     * Creates a Generic CoT XML with custom type
     */
    public static String createGenericXml(String uid, String type) {
        return String.format("""
            <?xml version='1.0' encoding='UTF-8'?>
            <event version='2.0' uid='%s' type='%s' 
                   time='%s' 
                   start='%s' 
                   stale='%s' 
                   how='h-g-i-g-o'>
                <point lat='%.6f' lon='%.6f' hae='%.1f' ce='%.1f' le='%.1f'/>
                <detail>
                    <contact callsign='GENERIC-1'/>
                    <remarks>Generic event for testing</remarks>
                    <custom>
                        <field1>value1</field1>
                        <field2>42</field2>
                        <field3>true</field3>
                    </custom>
                </detail>
            </event>
            """, uid, type, STANDARD_TIME, STANDARD_START, STANDARD_STALE,
                STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE);
    }
    
    /**
     * Creates expected MapItem document structure
     */
    public static Map<String, Object> createExpectedMapItemDocument(String uid) {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", uid);
        doc.put("w", "a-f-G-U-C");
        doc.put("h", STANDARD_CE);   // h = CE (circular error)
        doc.put("i", STANDARD_HAE);  // i = HAE (height above ellipsoid)
        doc.put("j", STANDARD_LAT);  // j = LAT (latitude)
        doc.put("k", STANDARD_LE);   // k = LE (linear error)
        doc.put("l", STANDARD_LON);  // l = LON (longitude)
        doc.put("n", parseTimestampToMicros(STANDARD_START));  // n = Start time
        doc.put("o", parseTimestampToMicros(STANDARD_STALE));  // o = Stale time
        // Additional fields would be mapped as they're actually used
        return doc;
    }
    
    /**
     * Creates expected Chat document structure
     */
    public static Map<String, Object> createExpectedChatDocument(String uid, String message) {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", uid);
        doc.put("message", message);  // message = chat message content
        doc.put("e", "ALPHA-1");      // e = author callsign
        doc.put("room", "All Chat Rooms");  // room = chat room name
        doc.put("j", STANDARD_LAT);   // j = LAT
        doc.put("l", STANDARD_LON);   // l = LON
        doc.put("h", STANDARD_CE);    // h = CE
        doc.put("i", STANDARD_HAE);   // i = HAE
        doc.put("k", STANDARD_LE);    // k = LE
        doc.put("n", parseTimestampToMicros(STANDARD_START));  // n = Start time
        doc.put("o", parseTimestampToMicros(STANDARD_STALE));  // o = Stale time
        return doc;
    }
    
    /**
     * Creates expected File document structure
     */
    public static Map<String, Object> createExpectedFileDocument(String uid, String filename, long sizeInBytes) {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", uid);
        doc.put("dd", filename);
        doc.put("ee", (double) sizeInBytes);
        doc.put("ff", "abc123def456789012345678901234567890");
        doc.put("gg", "http://192.168.1.100:8080/files/" + filename);
        doc.put("hh", uid);
        doc.put("ii", "ALPHA-1");
        doc.put("jj", "Test File");
        doc.put("h", STANDARD_LAT);
        doc.put("j", STANDARD_LON);
        doc.put("k", STANDARD_HAE);
        doc.put("l", STANDARD_CE);
        doc.put("m", STANDARD_LE);
        doc.put("q", parseTimestampToMicros(STANDARD_START));
        doc.put("r", parseTimestampToMicros(STANDARD_STALE));
        return doc;
    }
    
    /**
     * Creates expected API document structure
     */
    public static Map<String, Object> createExpectedApiDocument(String uid, String endpoint, String method) {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", uid);
        doc.put("kk", endpoint);
        doc.put("ll", method);
        doc.put("mm", parseTimestampToMicros(STANDARD_TIME));
        doc.put("h", STANDARD_LAT);
        doc.put("j", STANDARD_LON);
        doc.put("k", STANDARD_HAE);
        doc.put("l", STANDARD_CE);
        doc.put("m", STANDARD_LE);
        doc.put("q", parseTimestampToMicros(STANDARD_START));
        doc.put("r", parseTimestampToMicros(STANDARD_STALE));
        return doc;
    }
    
    /**
     * Creates expected Generic document structure
     */
    public static Map<String, Object> createExpectedGenericDocument(String uid, String type) {
        Map<String, Object> doc = new HashMap<>();
        doc.put("_id", uid);
        doc.put("w", type);
        doc.put("h", STANDARD_LAT);
        doc.put("j", STANDARD_LON);
        doc.put("k", STANDARD_HAE);
        doc.put("l", STANDARD_CE);
        doc.put("m", STANDARD_LE);
        doc.put("q", parseTimestampToMicros(STANDARD_START));
        doc.put("r", parseTimestampToMicros(STANDARD_STALE));
        
        // Generic documents include all detail content
        Map<String, Object> detail = new HashMap<>();
        detail.put("contact", Map.of("callsign", "GENERIC-1"));
        detail.put("remarks", "Generic event for testing");
        Map<String, Object> custom = new HashMap<>();
        custom.put("field1", "value1");
        custom.put("field2", "42");
        custom.put("field3", "true");
        detail.put("custom", custom);
        doc.put("nn", detail);
        
        return doc;
    }
    
    /**
     * Common CoT types for testing
     */
    public static class CoTTypes {
        public static final String MAP_ITEM = "a-f-G-U-C";
        public static final String CHAT = "b-t-f";
        public static final String FILE_SHARE = "b-f-t-f";
        public static final String API = "t-x-c-t";
        public static final String EMERGENCY = "a-u-emergency-g";
        public static final String SENSOR = "a-u-S";
        public static final String AIRCRAFT = "a-u-A";
        public static final String GROUND = "a-u-G";
        public static final String GENERIC = "x-custom-type";
    }
    
    /**
     * Common UIDs for testing
     */
    public static class TestUIDs {
        public static final String MAP_ITEM_1 = "MAP-ITEM-001";
        public static final String MAP_ITEM_2 = "MAP-ITEM-002";
        public static final String CHAT_1 = "CHAT-MESSAGE-001";
        public static final String CHAT_2 = "CHAT-MESSAGE-002";
        public static final String FILE_1 = "FILE-SHARE-001";
        public static final String FILE_2 = "FILE-SHARE-002";
        public static final String API_1 = "API-REQUEST-001";
        public static final String API_2 = "API-REQUEST-002";
        public static final String GENERIC_1 = "GENERIC-EVENT-001";
        public static final String GENERIC_2 = "GENERIC-EVENT-002";
    }
    
    /**
     * Test data sets for parameterized tests
     */
    public static class TestDataSets {
        public static final Object[][] COORDINATE_TEST_DATA = {
            {STANDARD_LAT, STANDARD_LON, STANDARD_HAE, STANDARD_CE, STANDARD_LE},
            {ALT_LAT, ALT_LON, ALT_HAE, ALT_CE, ALT_LE},
            {0.0, 0.0, 0.0, 0.0, 0.0},
            {90.0, 180.0, 9999.9, 999.9, 999.9},
            {-90.0, -180.0, -9999.9, 0.1, 0.1}
        };
        
        public static final String[] CALLSIGN_TEST_DATA = {
            "ALPHA-1", "BRAVO-2", "CHARLIE-3", "DELTA-4", "ECHO-5"
        };
        
        public static final String[] MESSAGE_TEST_DATA = {
            "Hello World", "Test message", "Emergency situation", 
            "All clear", "Status update", "Mission briefing"
        };
        
        public static final String[] FILENAME_TEST_DATA = {
            "document.pdf", "image.jpg", "video.mp4", "audio.wav", "data.zip"
        };
        
        public static final long[] FILESIZE_TEST_DATA = {
            1024L, 1048576L, 5242880L, 10485760L, 104857600L
        };
    }
    
    /**
     * Helper method to parse timestamp to microseconds
     */
    private static long parseTimestampToMicros(String timestamp) {
        try {
            Instant instant = Instant.parse(timestamp);
            return instant.getEpochSecond() * 1_000_000L + instant.getNano() / 1_000L;
        } catch (Exception e) {
            return 0L;
        }
    }
    
    /**
     * Creates a timestamp that is X seconds in the future from the standard time
     */
    public static String createFutureTimestamp(int secondsFromNow) {
        Instant future = Instant.parse(STANDARD_TIME).plusSeconds(secondsFromNow);
        return future.toString();
    }
    
    /**
     * Creates a timestamp that is X seconds in the past from the standard time
     */
    public static String createPastTimestamp(int secondsAgo) {
        Instant past = Instant.parse(STANDARD_TIME).minusSeconds(secondsAgo);
        return past.toString();
    }
    
    /**
     * Generates a unique UID with a given prefix
     */
    public static String generateUID(String prefix) {
        return prefix + "-" + System.currentTimeMillis() + "-" + Math.abs(System.nanoTime() % 1000);
    }
}