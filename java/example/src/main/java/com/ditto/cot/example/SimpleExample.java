package com.ditto.cot.example;

import com.ditto.cot.CoTEvent;
import com.ditto.cot.CoTEvent.CoTDetail;
import com.ditto.cot.CoTEvent.CoTPoint;
import com.ditto.cot.DittoDocument;

import java.time.Instant;
import java.util.HashMap;
import java.util.Map;

/**
 * A simple example demonstrating the basic usage of the Ditto CoT library.
 * This example shows how to:
 * 1. Create a CoT event from scratch
 * 2. Convert it to a Ditto document
 * 3. Convert it back to a CoT event
 * 4. Print the results
 */
public class SimpleExample {
    public static void main(String[] args) {
        try {
            // 1. Create a simple CoT event
            System.out.println("=== Creating a CoT Event ===");
            CoTEvent event = createSampleCotEvent();
            System.out.println("Original CoT Event XML:");
            System.out.println(event.toXml());
            
            // 2. Convert to Ditto document
            System.out.println("\n=== Converting to Ditto Document ===");
            DittoDocument doc = event.toDittoDocument();
            System.out.println("Ditto Document JSON:");
            System.out.println(doc.toJson());
            
            // 3. Convert back to CoT event
            System.out.println("\n=== Converting back to CoT Event ===");
            CoTEvent roundTripEvent = CoTEvent.fromDittoDocument(doc);
            System.out.println("Round-tripped CoT Event XML:");
            System.out.println(roundTripEvent.toXml());
            
            // 4. Verify the round trip
            System.out.println("\n=== Verification ===");
            boolean isEqual = event.toXml().equals(roundTripEvent.toXml());
            System.out.println("Original and round-tripped XML are equal: " + isEqual);
            
            if (!isEqual) {
                System.err.println("Warning: The round-trip conversion did not produce identical XML.");
            }
            
        } catch (Exception e) {
            System.err.println("Error in example: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }
    
    private static CoTEvent createSampleCotEvent() {
        // Create a new CoTEvent
        CoTEvent event = new CoTEvent();
        event.setVersion("2.0");
        event.setUid("USER-" + System.currentTimeMillis());
        event.setType("a-f-G-U-C");
        
        // Set timestamps
        String now = Instant.now().toString();
        event.setTime(now);
        event.setStart(now);
        event.setStale(Instant.now().plusSeconds(300).toString());
        
        event.setHow("h-g-i-gdo");
        
        // Create and set the point
        CoTPoint point = new CoTPoint();
        point.setLat("34.12345");
        point.setLon("-118.12345");
        point.setHae("150.0");
        point.setCe("10.0");
        point.setLe("25.0");
        event.setPoint(point);
        
        // Create and set the detail
        CoTDetail detail = new CoTDetail();
        Map<String, Object> detailMap = new HashMap<>();
        detailMap.put("callsign", "ALPHA-1");
        detailMap.put("groupName", "BLUE");
        detailMap.put("original_type", "a-f-G-U-C");
        detail.setFromMap(detailMap, null);
        event.setDetail(detail);
        
        return event;
    }
}
