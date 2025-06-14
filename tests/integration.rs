use ditto_cot::detail_parser::parse_detail_section;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detail_parsing() {
        let detail = r#"<detail><contact callsign='RAVEN'/><__group name='Blue'/></detail>"#;
        let (callsign, group_name, extras) = parse_detail_section(detail);

        assert_eq!(callsign, Some("RAVEN".to_string()));
        assert_eq!(group_name, Some("Blue".to_string()));
        assert!(extras.contains_key("contact"));
        assert!(extras.contains_key("__group"));
    }
}