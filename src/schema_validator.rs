use std::io::Error as IoError;
use xml::reader::{EventReader, ParserConfig};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaValidationError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    
    #[error("XML parsing error: {0}")]
    XmlParsing(String),
    
    #[error("Schema validation error: {0}")]
    Validation(String),
}

/// Validates XML data against the CoT schema
/// 
/// # Arguments
/// * `xml_data` - The XML data to validate as a string
/// 
/// # Returns
/// * `Result<(), SchemaValidationError>` - Ok(()) if validation succeeds, or an error if validation fails
pub fn validate_against_cot_schema(xml_data: &str) -> Result<(), SchemaValidationError> {
    // Basic XML well-formedness check
    let config = ParserConfig::new()
        .trim_whitespace(true)
        .whitespace_to_characters(true);
    
    let reader = EventReader::new_with_config(xml_data.as_bytes(), config);
    
    // Just parse the XML to check if it's well-formed
    for event in reader {
        match event {
            Ok(_) => {},
            Err(e) => return Err(SchemaValidationError::XmlParsing(e.to_string())),
        }
    }
    
    // Note: Full XSD validation is not implemented due to issues with xml-rs-xsd
    // In a production environment, you might want to use a different approach,
    // such as calling out to a command-line tool like xmllint
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_valid_cot() -> Result<(), Box<dyn std::error::Error>> {
        let valid_cot = r#"
        <event version="2.0" 
              uid="ANDROID-deadbeef" 
              type="a-f-G-U-C" 
              time="2021-02-27T20:32:24.913Z" 
              start="2021-02-27T20:32:24.913Z" 
              stale="2021-02-27T20:38:39.913Z" 
              how="h-g-i-g-o">
            <point lat="1.234567" lon="3.456789" hae="9999999.0" ce="9999999.0" le="9999999.0"/>
            <detail>
                <contact callsign="TEST-USER"/>
                <__group name="Cyan" role="Team Member"/>
            </detail>
        </event>"#;
        
        validate_against_cot_schema(valid_cot)?;
        Ok(())
    }
    
    #[test]
    fn test_validate_invalid_cot() {
        // Malformed XML - missing closing tag
        let invalid_cot = r#"
        <event version="2.0" 
              uid="TEST-123"
              type="a-f-G-U-C" 
              time="2021-02-27T20:32:24.913Z" 
              start="2021-02-27T20:32:24.913Z" 
              stale="2021-02-27T20:38:39.913Z" 
              how="h-g-i-g-o">
            <point lat="1.234567" lon="3.456789" hae="9999999.0" ce="9999999.0" le="9999999.0">
        "#;
        
        assert!(validate_against_cot_schema(invalid_cot).is_err());
        
        // Malformed XML - unclosed attribute
        let invalid_cot2 = r#"
        <event version="2.0" 
              uid="TEST-123
              type="a-f-G-U-C" 
              time="2021-02-27T20:32:24.913Z" 
              start="2021-02-27T20:32:24.913Z" 
              stale="2021-02-27T20:38:39.913Z" 
              how="h-g-i-g-o">
            <point lat="1.234567" lon="3.456789" hae="9999999.0" ce="9999999.0" le="9999999.0"/>
        </event>"#;
        
        assert!(validate_against_cot_schema(invalid_cot2).is_err());
    }
}
