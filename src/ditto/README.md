# Ditto Rust Code Generation

This directory contains the Rust types and logic for Ditto document integration, including types generated automatically from the Ditto JSON schema.

## Code Generation Overview

- **Source of Truth:** The canonical data model is defined in the JSON schema at `src/schema/ditto_schemas/ditto.schema.json`.
- **Generated File:** Rust types are generated from this schema and written to `src/ditto/schema.rs`.
- **Build Script:** The code generation is handled by `build.rs` at the project root, which runs automatically during `cargo build`.
- **Codegen Tool:** The [`typify`](https://github.com/oxidecomputer/typify) crate is used to generate Rust types from JSON Schema.

## How it Works

1. **Edit the Schema:**
   - Make changes to the JSON schema (`src/schema/ditto_schemas/ditto.schema.json`).
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

---

For more information, see the root `README.md` or the documentation for `typify` and `schemars` crates.
