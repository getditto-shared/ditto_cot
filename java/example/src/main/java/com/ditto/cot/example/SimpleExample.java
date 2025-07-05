package com.ditto.cot.example;

import com.ditto.cot.CoTConverter;
import com.ditto.cot.CoTEvent;
import com.ditto.cot.schema.DittoDocument;

import java.time.Instant;
import java.util.HashMap;
import java.util.Map;

/**
 * A simple example demonstrating the basic usage of the Ditto CoT library.
 * This example shows how to:
 * 1. Parse CoT XML from a sample
 * 2. Convert it to a Ditto document
 * 3. Serialize to JSON
 * 4. Print the results
 */
public class SimpleExample {
    public static void main(String[] args) {
        try {
            // Initialize the converter
            CoTConverter converter = new CoTConverter();
            
            // 1. Create a simple CoT XML
            System.out.println("=== Creating Sample CoT XML ===");
            String cotXml = createSampleCoTXml();
            System.out.println("Sample CoT XML:");
            System.out.println(cotXml);
            
            // 2. Parse the XML into a CoTEvent
            System.out.println("\n=== Parsing CoT XML ===");
            CoTEvent cotEvent = converter.parseCoTXml(cotXml);
            System.out.println("Parsed CoT Event:");
            printEventDetails(cotEvent);
            
            // 3. Convert to Ditto document
            System.out.println("\n=== Converting to Ditto Document ===");
            Object dittoDocument = converter.convertToDocument(cotXml);
            System.out.println("Ditto Document Type: " + dittoDocument.getClass().getSimpleName());
            
            // 4. Serialize to JSON
            if (dittoDocument instanceof DittoDocument) {
                System.out.println("\n=== Serializing to JSON ===");
                String json = ((DittoDocument) dittoDocument).toJson();
                System.out.println("Ditto Document JSON:");
                System.out.println(json);
                
                // 5. Deserialize back from JSON
                System.out.println("\n=== Round-trip Test ===");
                @SuppressWarnings("unchecked")
                Class<? extends DittoDocument> docClass = (Class<? extends DittoDocument>) dittoDocument.getClass();
                DittoDocument roundTripDoc = DittoDocument.fromJson(json, docClass);
                System.out.println("Round-trip successful: " + (roundTripDoc != null));
                if (roundTripDoc != null) {
                    System.out.println("Round-trip document type: " + roundTripDoc.getClass().getSimpleName());
                }
            }
            
        } catch (Exception e) {
            System.err.println("Error in example: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }
    
    private static String createSampleCoTXml() {
        // Return a sample CoT XML string - this would normally come from external source
        return """
            <?xml version="1.0" encoding="UTF-8"?>
            <event version="2.0" uid="EXAMPLE-001" type="a-f-G-U-C" 
                   time="2023-01-01T12:00:00.000Z" 
                   start="2023-01-01T12:00:00.000Z" 
                   stale="2023-01-01T12:05:00.000Z" 
                   how="h-g-i-gdo">
                <point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="25.0"/>
                <detail>
                    <contact callsign="ALPHA-1"/>
                    <group name="BLUE"/>
                    <platform original_type="a-f-G-U-C"/>
                </detail>
            </event>
            """;
    }
    
    private static void printEventDetails(CoTEvent event) {
        System.out.println("  UID: " + event.getUid());
        System.out.println("  Type: " + event.getType());
        System.out.println("  Version: " + event.getVersion());
        System.out.println("  Time: " + event.getTime());
        System.out.println("  Start: " + event.getStart());
        System.out.println("  Stale: " + event.getStale());
        System.out.println("  How: " + event.getHow());
        
        if (event.getPoint() != null) {
            System.out.println("  Point: " + 
                event.getPointLatitude() + ", " + 
                event.getPointLongitude() + ", " + 
                event.getPointHae() + " (HAE)");
        }
        
        if (event.getDetail() != null) {
            System.out.println("  Detail: " + event.getDetailMap());
        }
    }
}
