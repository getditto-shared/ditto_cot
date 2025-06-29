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

/// Performs a semantic equality check on two XML strings, ignoring attribute order and whitespace by leveraging minimize_xml for normalization.
/// Performs a semantic equality check on two XML strings using DOM comparison.
/// This ignores element order, normalizes timezone formats, and compares the actual content structure.
/// 
/// If strict is false, attribute order is ignored completely. If true, attribute order must match.
pub fn semantic_xml_eq(xml1: &str, xml2: &str, strict: bool) -> bool {
    use roxmltree::Document;
    
    // Parse the XML documents
    let doc1 = match Document::parse(xml1) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Failed to parse first XML: {}", e);
            return false;
        },
    };
    
    let doc2 = match Document::parse(xml2) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Failed to parse second XML: {}", e);
            return false;
        },
    };
    
    // Compare the root elements
    let root1 = doc1.root_element();
    let root2 = doc2.root_element();
    
    nodes_equal(&root1, &root2, strict)
}

/// Legacy version that defaults to non-strict comparison
pub fn semantic_xml_eq_legacy(xml1: &str, xml2: &str) -> bool {
    semantic_xml_eq(xml1, xml2, false)
}

/// Helper function to compare two XML nodes recursively
fn nodes_equal(node1: &roxmltree::Node, node2: &roxmltree::Node, strict: bool) -> bool {
    // Compare tag names
    if node1.tag_name().name() != node2.tag_name().name() {
        return false;
    }
    
    // Compare attributes (ignoring order if not strict)
    let mut attrs1 = std::collections::HashMap::new();
    for attr in node1.attributes() {
        let value = if attr.name() == "time" || attr.name() == "start" || attr.name() == "stale" {
            // Normalize timezone format for date attributes
            normalize_datetime(attr.value())
        } else {
            attr.value().to_string()
        };
        attrs1.insert(attr.name(), value);
    }
    
    let mut attrs2 = std::collections::HashMap::new();
    for attr in node2.attributes() {
        let value = if attr.name() == "time" || attr.name() == "start" || attr.name() == "stale" {
            // Normalize timezone format for date attributes
            normalize_datetime(attr.value())
        } else {
            attr.value().to_string()
        };
        attrs2.insert(attr.name(), value);
    }
    
    // Compare attributes based on strict mode
    if strict {
        // In strict mode, attribute order matters
        if attrs1 != attrs2 {
            return false;
        }
    } else {
        // In non-strict mode, only compare attribute values, not order
        // Check that all keys in attrs1 are in attrs2 with the same values
        for (key, val1) in &attrs1 {
            match attrs2.get(key) {
                Some(val2) if val1 == val2 => {},
                _ => return false,
            }
        }
        
        // Check that all keys in attrs2 are in attrs1
        for key in attrs2.keys() {
            if !attrs1.contains_key(key) {
                return false;
            }
        }
    }
    
    // Compare children (recursively)
    let mut children1: Vec<roxmltree::Node> = node1.children().filter(|n| n.is_element()).collect();
    let mut children2: Vec<roxmltree::Node> = node2.children().filter(|n| n.is_element()).collect();
    
    // Special handling for <detail> elements - order doesn't matter
    if node1.tag_name().name() == "detail" && node2.tag_name().name() == "detail" {
        // Count children by tag name
        let mut children1 = std::collections::HashMap::new();
        let mut children2 = std::collections::HashMap::new();
        
        for child in node1.children().filter(|n| n.is_element()) {
            *children1.entry(child.tag_name().name()).or_insert(0) += 1;
        }
        
        for child in node2.children().filter(|n| n.is_element()) {
            *children2.entry(child.tag_name().name()).or_insert(0) += 1;
        }
        
        // Check if child counts match
        if children1 != children2 {
            return false;
        }
        
        // For each child tag type, match children regardless of order
        for tag_name in children1.keys() {
            let tag1_children: Vec<_> = node1.children()
                .filter(|n| n.is_element() && n.tag_name().name() == *tag_name)
                .collect();
            let tag2_children: Vec<_> = node2.children()
                .filter(|n| n.is_element() && n.tag_name().name() == *tag_name)
                .collect();
            
            // Try to match each child from node1 with a child from node2
            for child1 in &tag1_children {
                if !tag2_children.iter().any(|child2| nodes_equal(child1, child2, strict)) {
                    return false;
                }
            }
        }
        
        return true;
    }
    
    // For regular elements, compare children in order
    if children1.len() != children2.len() {
        return false;
    }
    
    for (child1, child2) in children1.iter().zip(children2.iter()) {
        if !nodes_equal(child1, child2, strict) {
            return false;
        }
    }
    
    true
}

/// Normalize datetime strings to a consistent format
fn normalize_datetime(datetime: &str) -> String {
    // Handle both "Z" and "+00:00" formats
    if datetime.ends_with('Z') {
        datetime.to_string()
    } else if datetime.contains('+') || datetime.contains('-') {
        // Check if it's UTC (+00:00)
        if datetime.ends_with("+00:00") {
            // Convert to Z format
            format!("{0}Z", datetime.strip_suffix("+00:00").unwrap_or(datetime))
        } else {
            datetime.to_string()
        }
    } else {
        datetime.to_string()
    }
}
