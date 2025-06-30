use ditto_cot::detail_parser::parse_detail_section;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detail_parsing() {
        let detail = r#"<detail><contact callsign='RAVEN'/><__group name='Blue'/></detail>"#;
        let extras = parse_detail_section(detail);
        assert_eq!(
            extras.get("contact").unwrap()["callsign"],
            serde_json::Value::String("RAVEN".to_string())
        );
        assert_eq!(
            extras.get("__group").unwrap()["name"],
            serde_json::Value::String("Blue".to_string())
        );
    }

    #[test]
    fn test_nested_detail_parsing() {
        let detail = r#"<detail><foo bar='baz'><child x='1'><subchild>abc</subchild></child></foo></detail>"#;
        let extras = parse_detail_section(detail);
        let foo = &extras["foo"];
        assert_eq!(foo["bar"], serde_json::Value::String("baz".to_string()));
        let child = &foo["child"];
        assert_eq!(child["x"], serde_json::Value::String("1".to_string()));
        let subchild = &child["subchild"];
        assert_eq!(subchild, "abc");
    }

    #[test]
    fn test_mixed_text_and_attributes() {
        let detail = r#"<detail><note importance='high'>Check this</note></detail>"#;
        let extras = parse_detail_section(detail);
        let note = &extras["note"];
        assert_eq!(
            note["importance"],
            serde_json::Value::String("high".to_string())
        );
        assert_eq!(
            note["_text"],
            serde_json::Value::String("Check this".to_string())
        );
    }

    #[test]
    fn test_repeated_elements() {
        let detail = r#"<detail><item id='1'/><item id='2'/><item id='3'/></detail>"#;
        let extras = parse_detail_section(detail);
        // All items are present, but since keys must be unique, only the last one is present
        assert_eq!(
            extras["item"]["id"],
            serde_json::Value::String("3".to_string())
        );
    }
}
