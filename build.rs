fn main() {
    // Re-run this build script if the build script itself changes
    println!("cargo:rerun-if-changed=build.rs");

    // You can add any other build-time configuration here if needed
    // For example, setting up environment variables, generating build info, etc.
}
