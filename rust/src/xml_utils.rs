//! Utility functions for XML normalization and whitespace handling.
//!
//! Use this to flatten XML for robust roundtrip comparison and detail field normalization.

/// Format a float for CoT XML: always include decimal point, preserve precision, and ensure .0 for whole numbers.
pub fn format_cot_float(val: f64) -> String {
    if val.fract() == 0.0 {
        format!("{:.1}", val)
    } else {
        // Use ryu for shortest representation, but always keep decimal point
        let s = ryu::Buffer::new().format(val).to_string();
        if s.contains('.') {
            s
        } else {
            format!("{:.1}", val)
        }
    }
}

/// Minimizes XML by removing unnecessary whitespace, newlines, and formatting for robust string comparison.
/// This function does not pretty-print; it produces a compact, normalized XML string.
pub fn minimize_xml(xml: &str) -> String {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use std::io::Cursor;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut out = String::new();
    use std::collections::BTreeMap;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                out.push('<');
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                out.push_str(&tag);
                // Sort attributes
                let mut attrs: Vec<_> = e.attributes().flatten().collect();
                attrs.sort_by_key(|a| String::from_utf8_lossy(a.key.as_ref()).to_string());
                for attr in attrs.iter() {
                    out.push(' ');
                    out.push_str(&String::from_utf8_lossy(attr.key.as_ref()));
                    out.push_str("=\"");
                    out.push_str(&String::from_utf8_lossy(&attr.value));
                    out.push('"');
                }
                out.push('>');
            }
            Ok(Event::Empty(ref e)) => {
                out.push('<');
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                out.push_str(&tag);
                // Sort attributes
                let mut attrs: Vec<_> = e.attributes().flatten().collect();
                attrs.sort_by_key(|a| String::from_utf8_lossy(a.key.as_ref()).to_string());
                for attr in attrs.iter() {
                    out.push(' ');
                    out.push_str(&String::from_utf8_lossy(attr.key.as_ref()));
                    out.push_str("=\"");
                    out.push_str(&String::from_utf8_lossy(&attr.value));
                    out.push('"');
                }
                out.push_str("/>");
            }
            Ok(Event::End(ref e)) => {
                out.push_str("</");
                out.push_str(&String::from_utf8_lossy(e.name().as_ref()));
                out.push('>');
            }
            Ok(Event::Text(e)) => {
                match std::str::from_utf8(e.as_ref()) {
                    Ok(text) => out.push_str(text),
                    Err(_) => out.push_str(&String::from_utf8_lossy(e.as_ref())),
                }
            }
            Ok(Event::CData(e)) => {
                match std::str::from_utf8(e.as_ref()) {
                    Ok(text) => out.push_str(text),
                    Err(_) => out.push_str(&String::from_utf8_lossy(e.as_ref())),
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break, // Ignore errors for minimization
            _ => (),
        }
        buf.clear();
    }
    out
}
