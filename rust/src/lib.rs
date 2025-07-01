//! # Ditto CoT
//!
//! A high-performance Rust library for working with Cursor on Target (CoT) messages and Ditto.
//!
//! ## Features
//! - Parse and generate CoT XML messages
//! - Transform between CoT and Ditto document formats
//! - Validate CoT messages against XML schemas
//! - Asynchronous Ditto integration
//! - Support for chat, location, and emergency message types
//! - Ergonomic builder patterns for event creation
//! - Fluent APIs for coordinate and accuracy specification
//!
//! ## Quick Start
//!
//! ### Creating CoT Events with Builder Pattern
//!
//! The library provides ergonomic builder patterns for creating CoT events:
//!
//! ```rust
//! use ditto_cot::cot_events::CotEvent;
//! use chrono::Duration;
//!
//! // Create a simple location update
//! let event = CotEvent::builder()
//!     .uid("USER-123")
//!     .event_type("a-f-G-U-C")
//!     .location(34.12345, -118.12345, 150.0)
//!     .callsign("ALPHA-1")
//!     .stale_in(Duration::minutes(10))
//!     .build();
//!
//! // Create with team and accuracy information
//! let tactical_event = CotEvent::builder()
//!     .uid("BRAVO-2")
//!     .location_with_accuracy(35.0, -119.0, 200.0, 5.0, 10.0)
//!     .callsign_and_team("BRAVO-2", "Blue")
//!     .build();
//! ```
//!
//! ### Point Construction with Fluent API
//!
//! Create geographic points with builder pattern:
//!
//! ```rust
//! use ditto_cot::cot_events::Point;
//!
//! // Simple coordinate specification
//! let point = Point::builder()
//!     .lat(34.0526)
//!     .lon(-118.2437)
//!     .hae(100.0)
//!     .build();
//!
//! // Coordinates with accuracy in one call
//! let accurate_point = Point::builder()
//!     .coordinates(34.0526, -118.2437, 100.0)
//!     .accuracy(3.0, 5.0)
//!     .build();
//!
//! // Alternative constructors
//! let point1 = Point::new(34.0, -118.0, 100.0);
//! let point2 = Point::with_accuracy(34.0, -118.0, 100.0, 5.0, 10.0);
//! ```
//!
//! ### XML Parsing and Generation
//!
//! ```rust
//! use ditto_cot::cot_events::CotEvent;
//!
//! // Parse CoT XML
//! let xml = r#"<event version="2.0" uid="TEST-123" type="a-f-G-U-C"
//!              time="2023-01-01T12:00:00Z" start="2023-01-01T12:00:00Z"
//!              stale="2023-01-01T12:05:00Z" how="h-g-i-g-o">
//!     <point lat="34.12345" lon="-118.12345" hae="150.0" ce="10.0" le="20.0"/>
//!     <detail></detail>
//! </event>"#;
//!
//! let event = CotEvent::from_xml(xml)?;
//!
//! // Generate XML from event
//! let xml_output = event.to_xml()?;
//! # Ok::<(), ditto_cot::error::CotError>(())
//! ```
//!
//! ### Ditto Integration
//!
//! ```rust
//! use ditto_cot::{cot_events::CotEvent, ditto::cot_to_document};
//!
//! let event = CotEvent::builder()
//!     .uid("USER-456")
//!     .callsign("CHARLIE-3")
//!     .location(36.0, -120.0, 250.0)
//!     .build();
//!
//! // Convert to Ditto document
//! let ditto_doc = cot_to_document(&event, "my-peer-id");
//! ```
//!
//! ## Modules
//! - `cot_events`: Core CoT event types and parsing
//! - `ditto`: Ditto document types and transformations
//! - `error`: Error types and utilities
//! - `model`: Data models and serialization
//! - `schema_validator`: XML schema validation
//! - `xml_parser`: XML parsing utilities
//! - `xml_writer`: XML generation utilities

#![warn(missing_docs)]

/// Core CoT event types and parsing
pub mod cot_events;

/// Detail section parsing utilities
pub mod detail_parser;

/// Ditto document types and transformations
pub mod ditto;

/// Error types and utilities
pub mod error;

/// Data models and serialization
pub mod model;

/// XML schema validation
pub mod schema_validator;

/// XML normalization utilities
pub mod xml_utils;

/// XML parsing utilities
pub mod xml_parser;

/// XML generation utilities
pub mod xml_writer;

// Re-export commonly used types
pub use ditto::{cot_to_document, CotDocument};
