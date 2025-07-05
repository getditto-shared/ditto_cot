use schemars::schema::{RootSchema, Schema};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

// Helper function to resolve external references by creating a flattened schema
fn resolve_external_refs(schema_dir: &Path) -> Result<RootSchema, Box<dyn std::error::Error>> {
    use schemars::schema::*;
    use std::collections::BTreeMap;

    // Read the common schema first
    let common_path = schema_dir.join("common.schema.json");
    let common_content = fs::read_to_string(&common_path)
        .map_err(|e| format!("Failed to read {}: {}", common_path.display(), e))?;
    let common_schema: RootSchema = serde_json::from_str(&common_content)
        .map_err(|e| format!("Failed to parse {}: {}", common_path.display(), e))?;

    // Extract common properties
    let common_props = if let Some(object_validation) = &common_schema.schema.object {
        object_validation.properties.clone()
    } else {
        BTreeMap::new()
    };

    let common_required = if let Some(object_validation) = &common_schema.schema.object {
        object_validation.required.clone()
    } else {
        std::collections::BTreeSet::new()
    };

    // Start with a definitions map
    let mut definitions = BTreeMap::new();

    // Add Common definition
    definitions.insert(
        "Common".to_string(),
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties: common_props.clone(),
                required: common_required.clone(),
                ..Default::default()
            })),
            ..Default::default()
        }),
    );

    // Process each document type schema
    let doc_types = [
        ("api.schema.json", "Api"),
        ("chat.schema.json", "Chat"),
        ("file.schema.json", "File"),
        ("mapitem.schema.json", "MapItem"),
        ("generic.schema.json", "Generic"),
    ];

    let mut one_of_schemas = Vec::new();

    for (filename, type_name) in &doc_types {
        let doc_path = schema_dir.join(filename);
        let doc_content = fs::read_to_string(&doc_path)
            .map_err(|e| format!("Failed to read {}: {}", doc_path.display(), e))?;
        let doc_schema: RootSchema = serde_json::from_str(&doc_content)
            .map_err(|e| format!("Failed to parse {}: {}", doc_path.display(), e))?;

        // Create a flattened schema that combines common + specific properties
        let mut combined_props = common_props.clone();
        let mut combined_required = common_required.clone();

        // Extract document-specific properties from allOf
        if let Some(all_of) = doc_schema
            .schema
            .subschemas
            .as_ref()
            .and_then(|s| s.all_of.as_ref())
        {
            for all_of_item in all_of {
                if let Schema::Object(obj) = all_of_item {
                    // Skip external references, process inline objects
                    if obj.reference.is_none() {
                        if let Some(object_validation) = &obj.object {
                            for (prop_name, prop_schema) in &object_validation.properties {
                                combined_props.insert(prop_name.clone(), prop_schema.clone());
                            }
                            for req in &object_validation.required {
                                combined_required.insert(req.clone());
                            }
                        }
                    }
                }
            }
        }

        // Create the flattened document type
        let flattened_doc = Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                properties: combined_props,
                required: combined_required,
                ..Default::default()
            })),
            ..Default::default()
        });

        definitions.insert(type_name.to_string(), flattened_doc);

        // Add to oneOf
        one_of_schemas.push(Schema::Object(SchemaObject {
            reference: Some(format!("#/definitions/{}", type_name)),
            ..Default::default()
        }));
    }

    // Create the final combined schema
    Ok(RootSchema {
        meta_schema: Some("http://json-schema.org/draft-07/schema#".to_string()),
        schema: SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("Ditto Document Root Schema".to_string()),
                ..Default::default()
            })),
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(one_of_schemas),
                ..Default::default()
            })),
            ..Default::default()
        },
        definitions,
    })
}

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

    // Also process the renamed fields in all document types
    for doc_type in ["Api", "Chat", "File", "MapItem", "Generic"] {
        if let Some(Schema::Object(doc_obj)) = schema.definitions.get_mut(doc_type) {
            if let Some(props) = &mut doc_obj.object {
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
    let schema_dir = Path::new("../schema");
    let out_file = "src/ditto/schema.rs";

    // Instruct Cargo to rerun if the build script or any schema files change
    println!("cargo:rerun-if-changed=build.rs");

    // Watch all schema files
    for entry in fs::read_dir(schema_dir).expect("Failed to read schema directory") {
        if let Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    println!("cargo:rerun-if-changed={}", entry.path().display());
                }
            }
        }
    }

    // Resolve external references to create a flat schema
    let mut schema =
        resolve_external_refs(schema_dir).expect("Failed to resolve external references");

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
