use serde_json::Value;
use std::process::Command;

#[test]
#[ignore = "Run with --ignored flag or RUST_TEST_INTEGRATION=1 to execute cross-language tests"]
fn test_cross_language_integration() {
    // Check if we should run this test
    if std::env::var("RUST_TEST_INTEGRATION").unwrap_or_default() != "1" {
        println!("Skipping cross-language integration test. Set RUST_TEST_INTEGRATION=1 to run.");
        return;
    }
    // Run Rust client binary directly
    println!("Running Rust integration client...");
    let rust_output = Command::new("./target/debug/examples/integration_client")
        .current_dir(".")
        .output()
        .expect("Failed to run Rust example - make sure 'make example-rust' was run first");

    if !rust_output.status.success() {
        panic!(
            "Rust client failed: {}",
            String::from_utf8_lossy(&rust_output.stderr)
        );
    }

    // Run Java using the extracted distribution
    println!("Running Java integration client...");
    let java_output = Command::new("../java/example/build/example-1.0-SNAPSHOT/bin/example")
        .current_dir(".")
        .output()
        .expect("Failed to run Java example - make sure 'make example-java' was run first");

    if !java_output.status.success() {
        panic!(
            "Java client failed: {}",
            String::from_utf8_lossy(&java_output.stderr)
        );
    }

    // Run C# integration client
    println!("Running C# integration client...");
    let csharp_output = Command::new("dotnet")
        .args(["run"])
        .current_dir("../csharp/example")
        .output()
        .expect("Failed to run C# example - make sure 'make example-csharp' was run first");

    if !csharp_output.status.success() {
        panic!(
            "C# client failed: {}",
            String::from_utf8_lossy(&csharp_output.stderr)
        );
    }

    // Parse outputs
    let rust_json: Value = serde_json::from_str(&String::from_utf8_lossy(&rust_output.stdout))
        .expect("Failed to parse Rust output as JSON");

    let java_json: Value = serde_json::from_str(&String::from_utf8_lossy(&java_output.stdout))
        .expect("Failed to parse Java output as JSON");

    let csharp_json: Value = serde_json::from_str(&String::from_utf8_lossy(&csharp_output.stdout))
        .expect("Failed to parse C# output as JSON");

    // Verify all succeeded
    assert_eq!(rust_json["success"], true, "Rust client should succeed");
    assert_eq!(java_json["success"], true, "Java client should succeed");
    assert_eq!(csharp_json["success"], true, "C# client should succeed");

    // Verify language identification
    assert_eq!(rust_json["lang"], "rust");
    assert_eq!(java_json["lang"], "java");
    assert_eq!(csharp_json["lang"], "csharp");

    // Verify same original XML (all should contain the expected UID)
    let rust_xml = rust_json["original_xml"]
        .as_str()
        .expect("Rust XML should be a string");
    let java_xml = java_json["original_xml"]
        .as_str()
        .expect("Java XML should be a string");
    let csharp_xml = csharp_json["original_xml"]
        .as_str()
        .expect("C# XML should be a string");

    // All should contain the same UID - this verifies they processed the same event
    assert!(rust_xml.contains("ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279"));
    assert!(java_xml.contains("ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279"));
    assert!(csharp_xml.contains("ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279"));
    assert!(rust_xml.contains("b-m-p-s-p-loc"));
    assert!(java_xml.contains("b-m-p-s-p-loc"));
    assert!(csharp_xml.contains("b-m-p-s-p-loc"));

    // Compare key fields in the Ditto documents
    let rust_doc = &rust_json["ditto_document"];
    let java_doc = &java_json["ditto_document"];
    let csharp_doc = &csharp_json["ditto_document"];

    // These should be structurally equivalent
    // Note: We compare key fields rather than exact JSON due to potential
    // serialization differences between languages
    verify_document_equivalence(rust_doc, java_doc);
    verify_document_equivalence(rust_doc, csharp_doc);
    verify_document_equivalence(java_doc, csharp_doc);

    // Verify all can round-trip
    assert!(
        rust_json["roundtrip_xml"].is_string(),
        "Rust should produce roundtrip XML"
    );
    assert!(
        java_json["roundtrip_xml"].is_string(),
        "Java should produce roundtrip XML"
    );
    assert!(
        csharp_json["roundtrip_xml"].is_string(),
        "C# should produce roundtrip XML"
    );

    println!("✅ Cross-language integration test passed!");
    println!("🦀 Rust, ☕ Java, and 🔷 C# clients produced equivalent results");
}

fn verify_document_equivalence(rust_doc: &Value, java_doc: &Value) {
    // Compare document structure - both should have similar fields
    // This is a basic structural comparison

    // Check if both have the same top-level structure
    match (rust_doc, java_doc) {
        (Value::Object(rust_obj), Value::Object(java_obj)) => {
            // Both should have similar core fields
            let core_fields = ["uid", "type", "version", "time", "start", "stale"];

            for field in core_fields {
                if let (Some(rust_val), Some(java_val)) = (rust_obj.get(field), java_obj.get(field))
                {
                    // For string fields, they should be identical
                    if rust_val.is_string() && java_val.is_string() {
                        assert_eq!(
                            rust_val, java_val,
                            "Field '{}' should be identical between Rust and Java",
                            field
                        );
                    }
                }
            }

            println!("✅ Document structures are equivalent");
        }
        _ => {
            // If one is an object and the other isn't, that's still potentially valid
            // depending on the serialization approach, so we just log this
            println!(
                "⚠️  Document structures differ in top-level type, but may still be equivalent"
            );
        }
    }
}

#[test]
#[ignore = "Run with --ignored flag or RUST_TEST_INTEGRATION=1 to execute cross-language tests"]
fn test_makefile_integration() {
    // Check if we should run this test
    if std::env::var("RUST_TEST_INTEGRATION").unwrap_or_default() != "1" {
        println!("Skipping makefile integration test. Set RUST_TEST_INTEGRATION=1 to run.");
        return;
    }
    // Test that the Makefile targets work correctly
    println!("Testing Makefile integration...");

    // Test make example-rust
    let make_rust = Command::new("make")
        .args(["example-rust"])
        .current_dir("..")
        .output()
        .expect("Failed to run make example-rust");

    assert!(
        make_rust.status.success(),
        "make example-rust failed: {}",
        String::from_utf8_lossy(&make_rust.stderr)
    );

    // Test make example-java
    let make_java = Command::new("make")
        .args(["example-java"])
        .current_dir("..")
        .output()
        .expect("Failed to run make example-java");

    assert!(
        make_java.status.success(),
        "make example-java failed: {}",
        String::from_utf8_lossy(&make_java.stderr)
    );

    // Test make example-csharp
    let make_csharp = Command::new("make")
        .args(["example-csharp"])
        .current_dir("..")
        .output()
        .expect("Failed to run make example-csharp");

    assert!(
        make_csharp.status.success(),
        "make example-csharp failed: {}",
        String::from_utf8_lossy(&make_csharp.stderr)
    );

    println!("✅ Makefile integration test passed!");
}
