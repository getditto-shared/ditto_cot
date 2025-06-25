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
