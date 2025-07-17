use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Shared test utilities for CoT library testing.
/// Provides common assertion helpers and test patterns.
pub struct TestUtils;

impl TestUtils {
    /// Performs a complete round-trip test: XML -> Document -> XML
    pub fn assert_round_trip_conversion(
        original_xml: &str,
        expected_uid: &str,
        expected_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Parse XML to Document
        let document = ditto_cot::from_xml(original_xml)?;
        
        // Verify document structure
        Self::assert_document_has_field(&document, "_id", expected_uid)?;
        
        // Convert back to XML
        let regenerated_xml = ditto_cot::to_xml(&document)?;
        assert!(!regenerated_xml.trim().is_empty(), "Regenerated XML should not be empty");
        
        // Parse regenerated XML to verify it's valid
        let regenerated_document = ditto_cot::from_xml(&regenerated_xml)?;
        Self::assert_document_has_field(&regenerated_document, "_id", expected_uid)?;
        
        Ok(())
    }
    
    /// Asserts that a document has a specific field with expected value
    pub fn assert_document_has_field(
        document: &Value,
        field: &str,
        expected_value: &str,
    ) -> Result<(), String> {
        match document.get(field) {
            Some(value) => {
                if value.as_str() == Some(expected_value) {
                    Ok(())
                } else {
                    Err(format!(
                        "Field '{}' has value {:?}, expected '{}'",
                        field, value, expected_value
                    ))
                }
            }
            None => Err(format!("Document missing required field '{}'", field)),
        }
    }
    
    /// Asserts that a document is a MapItem with required fields
    pub fn assert_map_item_document(
        document: &Value,
        expected_uid: &str,
        expected_lat: f64,
        expected_lon: f64,
    ) -> Result<(), String> {
        Self::assert_document_has_field(document, "_id", expected_uid)?;
        Self::assert_document_has_field(document, "w", "a-f-G-U-C")?;
        
        Self::assert_float_field(document, "h", expected_lat, 0.000001)?;
        Self::assert_float_field(document, "j", expected_lon, 0.000001)?;
        
        Self::assert_has_timestamp_field(document, "q")?; // start time
        Self::assert_has_timestamp_field(document, "r")?; // stale time
        
        Ok(())
    }
    
    /// Asserts that a document is a Chat with required fields
    pub fn assert_chat_document(
        document: &Value,
        expected_uid: &str,
        expected_message: &str,
        expected_sender: &str,
    ) -> Result<(), String> {
        Self::assert_document_has_field(document, "_id", expected_uid)?;
        Self::assert_document_has_field(document, "e", expected_message)?;
        Self::assert_document_has_field(document, "f", expected_sender)?;
        
        Self::assert_has_timestamp_field(document, "q")?; // start time
        Self::assert_has_timestamp_field(document, "r")?; // stale time
        
        Ok(())
    }
    
    /// Asserts that a document is a File with required fields
    pub fn assert_file_document(
        document: &Value,
        expected_uid: &str,
        expected_filename: &str,
        expected_size: Option<f64>,
    ) -> Result<(), String> {
        Self::assert_document_has_field(document, "_id", expected_uid)?;
        
        // Check if filename is contained in the file field
        match document.get("dd") {
            Some(file_value) => {
                if let Some(file_str) = file_value.as_str() {
                    if !file_str.contains(expected_filename) {
                        return Err(format!(
                            "File field '{}' does not contain expected filename '{}'",
                            file_str, expected_filename
                        ));
                    }
                } else {
                    return Err("File field is not a string".to_string());
                }
            }
            None => return Err("Document missing file field 'dd'".to_string()),
        }
        
        if let Some(size) = expected_size {
            Self::assert_float_field(document, "ee", size, 0.1)?;
        }
        
        Self::assert_has_timestamp_field(document, "q")?; // start time
        Self::assert_has_timestamp_field(document, "r")?; // stale time
        
        Ok(())
    }
    
    /// Asserts that a document is an API with required fields
    pub fn assert_api_document(
        document: &Value,
        expected_uid: &str,
        expected_endpoint: &str,
        expected_method: &str,
    ) -> Result<(), String> {
        Self::assert_document_has_field(document, "_id", expected_uid)?;
        Self::assert_document_has_field(document, "kk", expected_endpoint)?;
        Self::assert_document_has_field(document, "ll", expected_method)?;
        
        Self::assert_has_timestamp_field(document, "q")?; // start time
        Self::assert_has_timestamp_field(document, "r")?; // stale time
        
        Ok(())
    }
    
    /// Asserts that a document is Generic with required fields
    pub fn assert_generic_document(
        document: &Value,
        expected_uid: &str,
        expected_type: &str,
    ) -> Result<(), String> {
        Self::assert_document_has_field(document, "_id", expected_uid)?;
        Self::assert_document_has_field(document, "w", expected_type)?;
        
        Self::assert_has_timestamp_field(document, "q")?; // start time
        Self::assert_has_timestamp_field(document, "r")?; // stale time
        
        Ok(())
    }
    
    /// Asserts that coordinates are within expected ranges
    pub fn assert_valid_coordinates(lat: f64, lon: f64, hae: f64) -> Result<(), String> {
        if lat < -90.0 || lat > 90.0 {
            return Err(format!("Latitude should be between -90 and 90, was: {}", lat));
        }
        if lon < -180.0 || lon > 180.0 {
            return Err(format!("Longitude should be between -180 and 180, was: {}", lon));
        }
        if hae < -15000.0 || hae > 50000.0 {
            return Err(format!("HAE should be reasonable (between -15km and 50km), was: {}", hae));
        }
        Ok(())
    }
    
    /// Asserts that timestamp values are in microseconds and reasonable
    pub fn assert_valid_timestamp(timestamp: u64) -> Result<(), String> {
        if timestamp == 0 {
            return Err("Timestamp should not be zero".to_string());
        }
        
        // Check if timestamp is in microseconds (should be much larger than milliseconds)
        // 2024 timestamps in milliseconds start around 1.7e12, in microseconds around 1.7e15
        if timestamp <= 1_000_000_000_000_000 {
            return Err(format!("Timestamp should be in microseconds, was: {}", timestamp));
        }
        
        // Should not be too far in the future (year 2100)
        if timestamp >= 4_000_000_000_000_000 {
            return Err(format!("Timestamp should not be too far in future, was: {}", timestamp));
        }
        
        Ok(())
    }
    
    /// Performs timing test to ensure operations complete within reasonable time
    pub fn assert_performance<F>(operation: F, max_millis: u64, operation_name: &str) -> Result<(), String>
    where
        F: FnOnce(),
    {
        let start_time = Instant::now();
        operation();
        let duration = start_time.elapsed();
        let duration_millis = duration.as_millis() as u64;
        
        if duration_millis > max_millis {
            Err(format!(
                "{} took {} ms, expected <= {} ms",
                operation_name, duration_millis, max_millis
            ))
        } else {
            Ok(())
        }
    }
    
    /// Tests conversion with error handling
    pub fn assert_conversion_with_error_handling<F>(
        xml: &str,
        document_assertions: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&Value) -> Result<(), String>,
    {
        let document = ditto_cot::from_xml(xml)?;
        document_assertions(&document).map_err(|e| e.into())
    }
    
    /// Tests that XML parsing returns an error
    pub fn assert_xml_parsing_fails(invalid_xml: &str) -> Result<(), String> {
        match ditto_cot::from_xml(invalid_xml) {
            Ok(_) => Err("Expected XML parsing to fail, but it succeeded".to_string()),
            Err(_) => Ok(()),
        }
    }
    
    /// Tests concurrent access to conversion functions
    pub fn assert_concurrent_safety(
        xml: &str,
        thread_count: usize,
        iterations_per_thread: usize,
    ) -> Result<(), String> {
        let xml_arc = Arc::new(xml.to_string());
        let mut handles = Vec::new();
        
        for _ in 0..thread_count {
            let xml_clone = Arc::clone(&xml_arc);
            let handle = thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    match ditto_cot::from_xml(&xml_clone) {
                        Ok(document) => {
                            if document.is_null() {
                                return Err("Document should not be null".to_string());
                            }
                        }
                        Err(e) => return Err(format!("Conversion failed: {}", e)),
                    }
                }
                Ok(())
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            match handle.join() {
                Ok(result) => result?,
                Err(_) => return Err("Thread panicked".to_string()),
            }
        }
        
        Ok(())
    }
    
    /// Validates that a document contains expected structure for its type
    pub fn assert_document_structure(document: &Value) -> Result<(), String> {
        if !document.is_object() {
            return Err("Document should be an object".to_string());
        }
        
        // All documents should have an ID
        if !document.get("_id").is_some() {
            return Err("Document should have '_id' field".to_string());
        }
        
        // Check document type based on fields present
        if document.get("w").is_some() {
            // Has type field - could be MapItem, Generic, etc.
            Self::assert_has_timestamp_field(document, "q")?;
            Self::assert_has_timestamp_field(document, "r")?;
        } else if document.get("e").is_some() {
            // Has message field - likely Chat
            if !document.get("f").is_some() {
                return Err("Chat document should have sender field 'f'".to_string());
            }
        } else if document.get("dd").is_some() {
            // Has filename field - likely File
            if !document.get("ee").is_some() {
                return Err("File document should have size field 'ee'".to_string());
            }
        } else if document.get("kk").is_some() {
            // Has endpoint field - likely API
            if !document.get("ll").is_some() {
                return Err("API document should have method field 'll'".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Helper to modify XML with specific replacements for testing
    pub fn modify_xml(base_xml: &str, replacements: &HashMap<&str, &str>) -> String {
        let mut result = base_xml.to_string();
        for (from, to) in replacements {
            result = result.replace(from, to);
        }
        result
    }
    
    /// Helper to create test XML with invalid data for error testing
    pub fn create_invalid_xml(base_xml: &str, invalid_part: &str) -> String {
        base_xml.replace("</event>", &format!("{}</event>", invalid_part))
    }
    
    // Private helper methods
    
    fn assert_float_field(
        document: &Value,
        field: &str,
        expected: f64,
        tolerance: f64,
    ) -> Result<(), String> {
        match document.get(field) {
            Some(value) => {
                if let Some(actual) = value.as_f64() {
                    if (actual - expected).abs() <= tolerance {
                        Ok(())
                    } else {
                        Err(format!(
                            "Field '{}' has value {}, expected {} (tolerance {})",
                            field, actual, expected, tolerance
                        ))
                    }
                } else {
                    Err(format!("Field '{}' is not a number: {:?}", field, value))
                }
            }
            None => Err(format!("Document missing field '{}'", field)),
        }
    }
    
    fn assert_has_timestamp_field(document: &Value, field: &str) -> Result<(), String> {
        match document.get(field) {
            Some(value) => {
                if let Some(timestamp) = value.as_u64() {
                    Self::assert_valid_timestamp(timestamp)
                } else {
                    Err(format!("Field '{}' is not a valid timestamp: {:?}", field, value))
                }
            }
            None => Err(format!("Document missing timestamp field '{}'", field)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::CoTTestFixtures;
    
    #[test]
    fn test_document_field_assertion() {
        let document = serde_json::json!({
            "_id": "TEST-001",
            "w": "a-f-G-U-C"
        });
        
        assert!(TestUtils::assert_document_has_field(&document, "_id", "TEST-001").is_ok());
        assert!(TestUtils::assert_document_has_field(&document, "w", "a-f-G-U-C").is_ok());
        assert!(TestUtils::assert_document_has_field(&document, "_id", "WRONG").is_err());
        assert!(TestUtils::assert_document_has_field(&document, "missing", "value").is_err());
    }
    
    #[test]
    fn test_coordinate_validation() {
        assert!(TestUtils::assert_valid_coordinates(37.7749, -122.4194, 100.5).is_ok());
        assert!(TestUtils::assert_valid_coordinates(0.0, 0.0, 0.0).is_ok());
        assert!(TestUtils::assert_valid_coordinates(91.0, 0.0, 0.0).is_err()); // Invalid latitude
        assert!(TestUtils::assert_valid_coordinates(0.0, 181.0, 0.0).is_err()); // Invalid longitude
        assert!(TestUtils::assert_valid_coordinates(0.0, 0.0, 60000.0).is_err()); // Invalid altitude
    }
    
    #[test]
    fn test_timestamp_validation() {
        // Valid microsecond timestamp (around 2024)
        assert!(TestUtils::assert_valid_timestamp(1_705_320_600_000_000).is_ok());
        
        // Invalid timestamps
        assert!(TestUtils::assert_valid_timestamp(0).is_err()); // Zero
        assert!(TestUtils::assert_valid_timestamp(1_705_320_600_000).is_err()); // Milliseconds, not microseconds
        assert!(TestUtils::assert_valid_timestamp(5_000_000_000_000_000).is_err()); // Too far in future
    }
    
    #[test]
    fn test_xml_modification() {
        let base_xml = "<event uid='OLD-ID' type='OLD-TYPE'></event>";
        let mut replacements = HashMap::new();
        replacements.insert("OLD-ID", "NEW-ID");
        replacements.insert("OLD-TYPE", "NEW-TYPE");
        
        let modified = TestUtils::modify_xml(base_xml, &replacements);
        assert!(modified.contains("NEW-ID"));
        assert!(modified.contains("NEW-TYPE"));
        assert!(!modified.contains("OLD-ID"));
        assert!(!modified.contains("OLD-TYPE"));
    }
    
    #[test]
    fn test_performance_assertion() {
        // Fast operation should pass
        assert!(TestUtils::assert_performance(
            || std::thread::sleep(Duration::from_millis(10)),
            50, // 50ms limit
            "fast operation"
        ).is_ok());
        
        // Slow operation should fail
        assert!(TestUtils::assert_performance(
            || std::thread::sleep(Duration::from_millis(100)),
            50, // 50ms limit
            "slow operation"
        ).is_err());
    }
}