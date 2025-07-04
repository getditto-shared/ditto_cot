package com.ditto.cot.example;

import com.ditto.cot.CoTEvent;
import com.ditto.cot.CoTConverter;

import java.lang.reflect.Method;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
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
            // Initialize the converter
            CoTConverter converter = new CoTConverter();
            
            // 1. Create a simple CoT event
            System.out.println("=== Creating a CoT Event ===");
            CoTEvent event = createSampleCotEvent();
            System.out.println("Original CoT Event:");
            printEventDetails(event);
            
            // 2. Convert to Ditto document
            System.out.println("\n=== Converting to Ditto Document ===");
            Object document = converter.convertCoTEventToDocument(event);
            System.out.println("Ditto Document Type: " + document.getClass().getSimpleName());
            System.out.println("Document toString(): " + document);
            
            // 3. Convert back to CoT event
            System.out.println("\n=== Converting back to CoT Event ===");
            CoTEvent roundTripEvent = converter.convertDocumentToCoTEvent(document);
            System.out.println("Round-tripped CoT Event:");
            printEventDetails(roundTripEvent);
            
            // 4. Verify the round trip by comparing XML representations
            System.out.println("\n=== Verification ===");
            String originalXml = converter.convertCoTEventToXml(event);
            String roundTripXml = converter.convertCoTEventToXml(roundTripEvent);
            boolean isEqual = originalXml.equals(roundTripXml);
            System.out.println("Original and round-tripped XML are equal: " + isEqual);
            
            if (!isEqual) {
                System.out.println("\nOriginal XML:");
                System.out.println(originalXml);
                System.out.println("\nRound-tripped XML:");
                System.out.println(roundTripXml);
            }
            
        } catch (Exception e) {
            System.err.println("Error in example: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }
    
    private static Class<?> findNestedClass(Class<?> outerClass, String simpleName) {
        for (Class<?> nestedClass : outerClass.getDeclaredClasses()) {
            if (nestedClass.getSimpleName().equals(simpleName)) {
                return nestedClass;
            }
        }
        throw new RuntimeException("Could not find nested class: " + simpleName);
    }
    
    private static CoTEvent createSampleCotEvent() throws Exception {
        try {
            Instant now = Instant.now();
            
            // Create a new CoTEvent
            CoTEvent event = new CoTEvent();
            event.setVersion("2.0");
            event.setUid("USER-" + System.currentTimeMillis());
            event.setType("a-f-G-U-C");
            event.setTime(now.toString());
            event.setStart(now.toString());
            event.setStale(now.plus(5, ChronoUnit.MINUTES).toString());
            event.setHow("h-g-i-gdo");
            
            // Find the CoTPoint class using getDeclaredClasses()
            Class<?> pointClass = findNestedClass(CoTEvent.class, "CoTPoint");
            
            // Make the constructor accessible and create an instance
            java.lang.reflect.Constructor<?> pointConstructor = pointClass.getDeclaredConstructor();
            pointConstructor.setAccessible(true);
            Object point = pointConstructor.newInstance();
            
            // Set point properties using reflection
            Method setLat = pointClass.getMethod("setLat", String.class);
            Method setLon = pointClass.getMethod("setLon", String.class);
            Method setHae = pointClass.getMethod("setHae", String.class);
            Method setCe = pointClass.getMethod("setCe", String.class);
            Method setLe = pointClass.getMethod("setLe", String.class);
            
            setLat.invoke(point, "34.12345");
            setLon.invoke(point, "-118.12345");
            setHae.invoke(point, "150.0");
            setCe.invoke(point, "10.0");
            setLe.invoke(point, "25.0");
            
            // Set the point on the event using reflection
            Method setPoint = CoTEvent.class.getMethod("setPoint", pointClass);
            setPoint.invoke(event, point);
            
            // Create detail map
            Map<String, Object> detailMap = new HashMap<>();
            detailMap.put("callsign", "ALPHA-1");
            detailMap.put("groupName", "BLUE");
            detailMap.put("original_type", "a-f-G-U-C");
            
            // Find the CoTDetail class using getDeclaredClasses()
            Class<?> detailClass = findNestedClass(CoTEvent.class, "CoTDetail");
            
            // Make the constructor accessible and create an instance
            java.lang.reflect.Constructor<?> detailConstructor = detailClass.getDeclaredConstructor();
            detailConstructor.setAccessible(true);
            Object detail = detailConstructor.newInstance();
            
            // Set detail properties using reflection
            Method setFromMap = detailClass.getMethod("setFromMap", Map.class, Object.class);
            setFromMap.invoke(detail, detailMap, null);
            
            // Set the detail on the event using reflection
            Method setDetail = CoTEvent.class.getMethod("setDetail", detailClass);
            setDetail.invoke(event, detail);
            
            return event;
            
        } catch (Exception e) {
            System.err.println("Error creating CoT event: " + e);
            e.printStackTrace();
            throw e;
        }
    }
    
    private static void printEventDetails(CoTEvent event) throws Exception {
        System.out.println("UID: " + event.getUid());
        System.out.println("Type: " + event.getType());
        System.out.println("Time: " + event.getTime());
        System.out.println("Start: " + event.getStart());
        System.out.println("Stale: " + event.getStale());
        System.out.println("How: " + event.getHow());
        
        // Get point using reflection
        Method getPoint = CoTEvent.class.getMethod("getPoint");
        Object point = getPoint.invoke(event);
        
        if (point != null) {
            Class<?> pointClass = point.getClass();
            Method getLat = pointClass.getMethod("getLat");
            Method getLon = pointClass.getMethod("getLon");
            Method getHae = pointClass.getMethod("getHae");
            Method getCe = pointClass.getMethod("getCe");
            Method getLe = pointClass.getMethod("getLe");
            
            System.out.println("Point: " + 
                getLat.invoke(point) + ", " + 
                getLon.invoke(point) + ", " + 
                getHae.invoke(point) + " (HAE), " +
                getCe.invoke(point) + " CE, " +
                getLe.invoke(point) + " LE");
        }
        
        // Get detail using reflection
        Method getDetail = CoTEvent.class.getMethod("getDetail");
        Object detail = getDetail.invoke(event);
        
        if (detail != null) {
            Method toMap = detail.getClass().getMethod("toMap");
            System.out.println("Detail: " + toMap.invoke(detail));
        }
    }
}
