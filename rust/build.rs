use schemars::schema::{RootSchema, Schema};
use std::fs::{self, File};
use std::io::Write;
use typify::{TypeSpace, TypeSpaceSettings};

// Helper function to process underscore-prefixed keys in the Common schema
fn process_underscore_keys(schema: &mut RootSchema) {
    if let Some(Schema::Object(common_obj)) = schema.definitions.get_mut("Common") {
        if let Some(props) = &mut common_obj.object {
            // Special handling for _id - add serde rename attribute but don't rename in schema
            if let Some(Schema::Object(id_obj)) = props.properties.get_mut("_id") {
                id_obj.extensions.insert(
                    "x-rust-type-attributes".to_string(),
                    serde_json::json!([
                        "#[serde(rename = \"_id\")]",
                        "#[schemars(rename = \"_id\")]"
                    ]),
                );
            }

            // Step 2: Rename _c, _v, _r to d_c, d_v, d_r in schema (without adding x-rust-type-attributes)
            let fields_to_rename = [("_c", "d_c"), ("_v", "d_v"), ("_r", "d_r")];

            for (old_name, new_name) in &fields_to_rename {
                if let Some(property_schema) = props.properties.remove(*old_name) {
                    props
                        .properties
                        .insert(new_name.to_string(), property_schema);

                    // Update required fields
                    let old_name_str = old_name.to_string();
                    if props.required.contains(&old_name_str) {
                        props.required.remove(&old_name_str);
                        props.required.insert(new_name.to_string());
                    }
                }
            }
        }
    }
}

// Helper function to enhance the r field schema to generate proper RValue enums
fn enhance_r_field_schema(schema: &mut RootSchema) {
    // Define the document types that need RValue enums
    let doc_types = ["Api", "Chat", "File", "MapItem", "Generic"];

    for doc_type in &doc_types {
        if let Some(Schema::Object(doc_obj)) = schema.definitions.get_mut(*doc_type) {
            if let Some(props) = &mut doc_obj.object {
                // Find the r field and enhance its schema
                if let Some(Schema::Object(r_obj)) = props.properties.get_mut("r") {
                    // Create a unique name for this document type's RValue
                    let rvalue_name = format!("{}{}", doc_type, "RValue");

                    // Add a custom type name to generate an enum
                    r_obj.extensions.insert(
                        "x-rust-type-name".to_string(),
                        serde_json::json!(rvalue_name),
                    );

                    // Add custom derives and attributes
                    r_obj.extensions.insert(
                        "x-rust-type-attributes".to_string(),
                        serde_json::json!(["#[derive(Debug, Clone)]", "#[serde(untagged)]"]),
                    );
                }
            }
        }
    }
}

fn main() {
    // Directory containing the JSON schema files
    let schema_path = "../schema/ditto.schema.json";
    let out_file = "src/ditto/schema.rs";

    // Instruct Cargo to rerun if the schema or build script changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", schema_path);

    // Read the unified schema file
    let schema_str = fs::read_to_string(schema_path).expect("Failed to read schema file");
    let mut schema: RootSchema = serde_json::from_str(&schema_str).expect("Invalid JSON schema");

    // Process underscore-prefixed keys in the Common object
    process_underscore_keys(&mut schema);

    // Enhance the r field schema to generate proper RValue enums
    enhance_r_field_schema(&mut schema);

    // Generate Rust code from the schema
    let mut settings = TypeSpaceSettings::default();
    settings.with_derive("schemars::JsonSchema".to_string());
    let mut type_space = TypeSpace::new(&settings);

    // Add the schema to the type space
    type_space
        .add_root_schema(schema)
        .expect("Failed to add schema");

    // Generate the Rust code
    let mut generated = type_space.to_stream().to_string();

    // Manually patch the generated code to add serde rename attributes for d_c, d_v, d_r
    generated = generated
        .replace(
            "pub d_c : i64 ,",
            "#[serde(rename = \"_c\")] pub d_c : i64 ,",
        )
        .replace(
            "pub d_r : bool ,",
            "#[serde(rename = \"_r\")] pub d_r : bool ,",
        )
        .replace(
            "pub d_v : i64 ,",
            "#[serde(rename = \"_v\")] pub d_v : i64 ,",
        );

    // Add helper functions to convert between HashMap and serde_json::Map
    let helper_functions = r#"
// Helper functions to convert between HashMap and serde_json::Map
pub mod map_helpers {
    use std::collections::HashMap;
    use serde_json::{Map, Value};

    pub fn hashmap_to_json_map(map: HashMap<String, Value>) -> Map<String, Value> {
        map.into_iter().collect()
    }

    pub fn json_map_to_hashmap(map: Map<String, Value>) -> HashMap<String, Value> {
        map.into_iter().collect()
    }
}
"#;

    // Write the generated code to the output file
    let mut file = File::create(out_file).expect("Failed to create output file");
    writeln!(
        file,
        "// This file is @generated by build.rs; do not edit by hand.\n"
    )
    .unwrap();
    writeln!(file, "#![allow(missing_docs)]\n").unwrap();
    writeln!(file, "use serde::{{Serialize, Deserialize}};\n").unwrap();
    // writeln!(file, "use std::collections::HashMap;\n").unwrap();
    file.write_all(generated.as_bytes())
        .expect("Failed to write generated code");
    file.write_all(helper_functions.as_bytes())
        .expect("Failed to write helper functions");
}
