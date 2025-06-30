# Ditto Rust Code Generation

This directory contains the Rust types and logic for Ditto document integration, including types generated automatically from the Ditto JSON schema.

## Code Generation Overview

- **Source of Truth:** The canonical data model is defined in the JSON schema at `schema/ditto.schema.json`.
- **Generated File:** Rust types are generated from this schema and written to `src/ditto/schema.rs`.
- **Build Script:** The code generation is handled by `build.rs` at the project root, which runs automatically during `cargo build`.
- **Codegen Tool:** The [`typify`](https://github.com/oxidecomputer/typify) crate is used to generate Rust types from JSON Schema.

## How it Works

1. **Edit the Schema:**
   - Make changes to the JSON schema (`schema/ditto.schema.json`).
2. **Build or Test:**
   - Run `cargo build` or `cargo test`. The build script will:
     - Read the schema.
     - Generate Rust types with Serde and Schemars derives.
     - Write the output to `src/ditto/schema.rs`.
3. **Use the Types:**
   - Use the generated types (e.g., `Api`, `Chat`, `File`, `MapItem`) in your Rust code.

## Notes

- **Do Not Edit `schema.rs`:**
  - `src/ditto/schema.rs` is auto-generated and should never be edited by hand.
  - All changes should be made to the JSON schema and/or the build script.
- **Do Not Commit `schema.rs`:**
  - The generated file is excluded from version control via `.gitignore`.
- **Schema Descriptions:**
  - Field descriptions in the JSON schema are not currently propagated to Rust doc comments due to limitations in the codegen tool.
- **Custom Logic:**
  - Transformation functions in this directory convert CoT events into schema-compliant Ditto document types.

## Regenerating Code

If you update the schema, simply rerun `cargo build` or `cargo test` to regenerate the Rust types.

## Implementation Details

While `schema.rs` contains auto-generated types, the following files provide the manual implementation that uses these types:

## <detail> XML Parsing in Ditto

The Ditto Rust library provides robust parsing for the CoT `<detail>` element, supporting both flat and deeply nested XML structures. The parser is designed to be fully generic, with no special-case logic for specific keys or element names.

### Parsing Behavior
- **Recursive Parsing:**
  - All elements within `<detail>` are parsed recursively. Nested elements become nested `serde_json::Value::Object` maps in the output.
  - Example:
    ```xml
    <detail>
      <foo bar="baz"><child x="1"><subchild>abc</subchild></child></foo>
    </detail>
    ```
    becomes:
    ```json
    {
      "foo": {
        "bar": "baz",
        "child": {
          "x": "1",
          "subchild": "abc"
        }
      }
    }
    ```
- **Attributes:**
  - All XML attributes are included as key-value pairs in the corresponding map.
- **Text Content:**
  - If an element contains both attributes and text, the text is stored under the special key `_text`.
  - Example:
    ```xml
    <note importance="high">Check this</note>
    ```
    becomes:
    ```json
    {
      "note": {
        "importance": "high",
        "_text": "Check this"
      }
    }
    ```
- **Empty Elements:**
  - Empty elements (e.g., `<foo bar="baz"/>`) are parsed as objects with only attributes.
- **Repeated Elements:**
  - If multiple sibling elements share the same tag name, only the last one is retained in the resulting map (due to `HashMap` key semantics).

### Limitations
- **Key Collisions:**
  - Only the last occurrence of a repeated element is preserved. If you need to preserve all, consider extending the parser to collect repeated elements into a list.
- **Order Not Preserved:**
  - The output is a map, so XML element order is not preserved.

### Usage
- The main entry point is `parse_detail_section(&str) -> HashMap<String, Value>` in `src/detail_parser.rs`.
- The output can be used directly for CRDT map sync, round-trip conversion, or application logic.

See `tests/integration.rs` for examples and test coverage of nested, mixed, and repeated element parsing.

### `to_ditto.rs`

- **Core Transformation Logic**: Contains functions to transform CoT events into Ditto documents
- **Key Functions**:
  - `cot_to_document`: Main entry point that routes CoT events to appropriate transformers based on event type
  - `transform_location_event`: Converts location events to `MapItem` documents
  - `transform_chat_event`: Converts chat events to `Chat` documents
  - `transform_emergency_event`: Converts emergency events to `Api` documents
  - `transform_generic_event`: Fallback for other event types to `File` documents
- **DittoDocument Enum**: Defines the untagged enum that aggregates all document types (Api, Chat, File, MapItem)
- **Integration Point**: Bridges the gap between CoT event data and the schema-compliant Ditto document structure

### `from_ditto.rs`

- **Reverse Transformation**: Provides functionality to convert Ditto documents back to CoT events
- **Round-Trip Testing**: Primarily used to verify data integrity through complete conversion cycles
- **Key Function**: `cot_event_from_ditto_document` handles the conversion from any `DittoDocument` variant back to a `CotEvent`
- **Best-Effort Mapping**: Attempts to preserve as much information as possible during conversion, though some data loss may occur due to model differences

These implementation files demonstrate how the auto-generated schema types are used in practice, providing the business logic that connects the schema-defined data structures to the application's domain model.

---

For more information, see the root `README.md` or the documentation for `typify` and `schemars` crates.
