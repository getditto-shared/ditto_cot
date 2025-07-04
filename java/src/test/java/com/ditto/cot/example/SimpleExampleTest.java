package com.ditto.cot.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Test class for SimpleExample
 */
public class SimpleExampleTest {

    @Test
    public void testExampleExecution() {
        try {
            // Redirect System.out to capture output
            java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream();
            System.setOut(new java.io.PrintStream(out));
            
            // Run the example
            SimpleExample.main(new String[]{});
            
            // Verify output contains expected strings
            String output = out.toString();
            assertTrue(output.contains("=== Creating a CoT Event ==="), "Missing creation message");
            assertTrue(output.contains("=== Converting to Ditto Document ==="), "Missing conversion message");
            assertTrue(output.contains("=== Converting back to CoT Event ==="), "Missing round-trip message");
            assertTrue(output.contains("=== Verification ==="), "Missing verification message");
            
            // Check for successful round-trip
            assertTrue(output.contains("Original and round-tripped XML are equal: true"), 
                "Round-trip conversion failed");
                
        } catch (Exception e) {
            fail("Example execution failed: " + e.getMessage(), e);
        }
    }
}
