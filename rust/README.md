# Rust Implementation

[![Crates.io](https://img.shields.io/crates/v/cotditto)](https://crates.io/crates/cotditto)
[![Documentation](https://docs.rs/cotditto/badge.svg)](https://docs.rs/cotditto)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust library for translating between [Cursor-on-Target (CoT)](https://www.mitre.org/sites/default/files/pdf/09_4937.pdf) XML events and Ditto-compatible CRDT documents.

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cotditto = { git = "https://github.com/yourusername/ditto_cot" }
```

## 🚀 Usage

### Basic Conversion

Convert between CoT XML and Ditto documents:

```rust
use cotditto::{
    cot_events::CotEvent,
    ditto::{cot_to_document, DittoDocument},
};

// Parse CoT XML to CotEvent
let cot_xml = r#"<event version="2.0" ...></event>"#;
let event = CotEvent::from_xml(cot_xml)?;

// Convert to Ditto document
let doc = cot_to_document(&event, "peer-123");

// Serialize to JSON
let json = serde_json::to_string_pretty(&doc)?;
println!("{}", json);
```

## 📚 Documentation

Full API documentation is available on [docs.rs](https://docs.rs/cotditto).

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please see the [main CONTRIBUTING guide](../../CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.
