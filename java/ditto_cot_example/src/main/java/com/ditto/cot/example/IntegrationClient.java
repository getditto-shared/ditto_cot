package com.ditto.cot.example;

import com.ditto.cot.CoTConverter;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;

/**
 * Integration client that outputs structured JSON for cross-language testing.
 * This client processes the same CoT XML as the Rust client and outputs
 * comparable JSON results for integration testing.
 */
public class IntegrationClient {
    public static void main(String[] args) {
        try {
            // Create the same sample CoT XML as Rust client
            String cotXml = """
                <?xml version="1.0" encoding="UTF-8"?>
                <event version="2.0" uid="ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279" type="b-m-p-s-p-loc" how="h-e" start="2023-10-15T10:30:00.000Z" time="2023-10-15T10:30:00.000Z" stale="2023-10-15T10:35:00.000Z">
                    <point lat="35.091" lon="-106.558" hae="1618.8" ce="3.2" le="5.8"/>
                    <detail>
                        <contact callsign="PINKY" endpoint="192.168.1.10:4242:tcp"/>
                        <__group name="Blue" role="Team Member"/>
                        <color argb="-1"/>
                        <usericon iconsetpath="COT_MAPPING_SPOTMAP/b-m-p-s-p-loc/spy.png"/>
                        <link uid="ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279" type="b-m-p-s-p-loc" relation="p-p"/>
                        <remarks>Equipment check complete</remarks>
                        <status readiness="true"/>
                        <track speed="12.5" course="45.0"/>
                        <precisionlocation altsrc="GPS"/>
                    </detail>
                </event>
                """;

            // Initialize the converter
            CoTConverter converter = new CoTConverter();
            
            // Convert XML to Ditto Document
            Object dittoDocument = converter.convertToDocument(cotXml);
            
            // Convert back to XML
            String roundtripXml = converter.convertDocumentToXml(dittoDocument);
            
            // Create structured output using Jackson
            ObjectMapper mapper = new ObjectMapper();
            ObjectNode output = mapper.createObjectNode();
            
            output.put("lang", "java");
            output.put("original_xml", cotXml);
            output.set("ditto_document", mapper.valueToTree(dittoDocument));
            output.put("roundtrip_xml", roundtripXml);
            output.put("success", true);
            
            // Output JSON to stdout
            System.out.println(mapper.writerWithDefaultPrettyPrinter().writeValueAsString(output));
            
        } catch (Exception e) {
            try {
                // Output error in same JSON format
                ObjectMapper mapper = new ObjectMapper();
                ObjectNode output = mapper.createObjectNode();
                output.put("lang", "java");
                output.put("success", false);
                output.put("error", e.getMessage());
                System.out.println(mapper.writerWithDefaultPrettyPrinter().writeValueAsString(output));
            } catch (Exception jsonError) {
                System.err.println("Error in Java integration client: " + e.getMessage());
                e.printStackTrace();
            }
            System.exit(1);
        }
    }
}