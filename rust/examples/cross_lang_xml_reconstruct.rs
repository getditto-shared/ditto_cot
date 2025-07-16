//! Cross-language E2E test utility: Reconstruct XML from flattened JSON
//!
//! This utility is called by the Java cross-language E2E test to verify that
//! Rust can correctly reconstruct XML from a flattened JSON document created by Java.

use anyhow::{Context, Result};
use ditto_cot::ditto::cot_event_from_flattened_json;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <json_file>", args[0]);
        std::process::exit(1);
    }

    let json_file = &args[1];

    // Read the flattened JSON document
    let json_content = fs::read_to_string(json_file)
        .with_context(|| format!("Failed to read JSON file: {}", json_file))?;

    // Parse JSON
    let json_value: serde_json::Value =
        serde_json::from_str(&json_content).context("Failed to parse JSON")?;

    // Convert to CotEvent using the flattened JSON approach
    let cot_event = cot_event_from_flattened_json(&json_value);

    // Generate XML
    let xml = cot_event
        .to_xml()
        .context("Failed to generate XML from CotEvent")?;

    // Output the XML (this gets captured by the Java test)
    println!("{}", xml);

    Ok(())
}
