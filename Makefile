# Ditto CoT Makefile
# Builds and cleans all language-specific libraries

# Default target - show help when no command is given
.DEFAULT_GOAL := help

# Build all languages
.PHONY: all
all: rust java csharp swift

# Rust targets
.PHONY: rust
rust:
	@echo "Building Rust library..."
	@cd rust && cargo build

.PHONY: clean-rust
clean-rust:
	@echo "Cleaning Rust library..."
	@cd rust && cargo clean

# Java targets
.PHONY: java
java:
	@echo "Cleaning previous build and generated sources..."
	@if [ -f "java/build.gradle" ] || [ -f "java/build.gradle.kts" ]; then \
		cd java && \
		rm -rf build/generated-src build/classes build/resources build/tmp build/libs build/reports build/test-results && \
		mkdir -p src/main/java/com/ditto/cot/schema && \
		find src/main/java/com/ditto/cot/schema -type f -name '*.java' -delete; \
		echo "Generating Java classes from schema..."; \
		./gradlew generateSchemaClasses; \
		echo "Building Java library..."; \
		./gradlew build -x test -x jacocoTestCoverageVerification; \
	else \
		echo "Java build files not found. Skipping."; \
	fi

.PHONY: java-test-client
java-test-client:
	@echo "Building Java test client for cross-language tests..."
	@if [ -f "java/build.gradle" ] || [ -f "java/build.gradle.kts" ]; then \
		cd java && \
		echo "Building test client with all dependencies..."; \
		./gradlew :ditto_cot:fatJar; \
	else \
		echo "Java build files not found. Skipping."; \
	fi

.PHONY: clean-java
clean-java:
	@echo "Cleaning Java library..."
	@if [ -f "java/build.gradle" ] || [ -f "java/build.gradle.kts" ]; then \
		cd java && ./gradlew clean; \
	else \
		echo "Java build files not found. Skipping."; \
	fi

# C# targets
.PHONY: csharp
csharp:
	@echo "Building C# library..."
	@if [ -f "csharp/*.sln" ] || [ -f "csharp/*.csproj" ]; then \
		cd csharp && dotnet build; \
	else \
		echo "C# build files not found. Skipping."; \
	fi

.PHONY: clean-csharp
clean-csharp:
	@echo "Cleaning C# library..."
	@if [ -f "csharp/*.sln" ] || [ -f "csharp/*.csproj" ]; then \
		cd csharp && dotnet clean; \
	else \
		echo "C# build files not found. Skipping."; \
	fi

# Swift targets
.PHONY: swift
swift:
	@echo "Generating Swift types from schema..."
	@if [ -f "swift/Package.swift" ]; then \
		cd swift && \
		swift build --product ditto-cot-codegen && \
		.build/debug/ditto-cot-codegen --schema-path ../schema --output-path Sources/DittoCoTCore/Generated; \
		echo "Building Swift library..."; \
		swift build; \
	else \
		echo "Swift Package.swift not found. Skipping."; \
	fi

.PHONY: clean-swift
clean-swift:
	@echo "Cleaning Swift library..."
	@if [ -f "swift/Package.swift" ]; then \
		cd swift && swift package clean && rm -rf Sources/DittoCoTCore/Generated/*.swift; \
	else \
		echo "Swift Package.swift not found. Skipping."; \
	fi

# Test targets
.PHONY: test
test: test-rust test-java test-csharp test-swift

.PHONY: test-cross-lang
test-cross-lang: java-test-client
	@echo "Running cross-language multi-peer test..."
	@cd rust && cargo test e2e_cross_lang_multi_peer_test

.PHONY: test-rust
test-rust:
	@echo "Testing Rust library..."
	@cd rust && cargo nextest run

.PHONY: test-java
test-java:
	@echo "Testing Java library and example..."
	@if [ -f "java/build.gradle" ] || [ -f "java/build.gradle.kts" ]; then \
		cd java && ./gradlew :ditto_cot:test :ditto_cot_example:test --info --console=rich --rerun-tasks; \
	else \
		echo "Java build files not found. Skipping tests."; \
	fi

.PHONY: test-csharp
test-csharp:
	@echo "Testing C# library..."
	@if [ -f "csharp/*.sln" ] || [ -f "csharp/*.csproj" ]; then \
		cd csharp && dotnet test; \
	else \
		echo "C# build files not found. Skipping tests."; \
	fi

.PHONY: test-swift
test-swift:
	@echo "Testing Swift library..."
	@if [ -f "swift/Package.swift" ]; then \
		cd swift && swift test; \
	else \
		echo "Swift Package.swift not found. Skipping tests."; \
	fi

# Clean all
.PHONY: clean
clean: clean-rust clean-java clean-csharp clean-swift
	@echo "All libraries cleaned."

# Example targets
.PHONY: example-rust
example-rust:
	@echo "Running Rust example..."
	@cd rust && cargo run --example integration_client

.PHONY: example-java
example-java:
	@echo "Running Java example..."
	@cd java && ./gradlew :example:runIntegrationClient

# Integration test target
.PHONY: test-integration
test-integration: example-rust example-java
	@echo "Running cross-language integration test..."
	@cd rust && cargo test --test integration_test

# Help target
.PHONY: help
help:
	@echo "Ditto CoT Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  all           - Build all language libraries"
	@echo "  rust          - Build Rust library"
	@echo "  java          - Build Java library"
	@echo "  csharp        - Build C# library"
	@echo "  swift         - Build Swift library"
	@echo "  test          - Run tests for all libraries"
	@echo "  test-rust     - Run tests for Rust library"
	@echo "  test-java     - Run tests for Java library"
	@echo "  test-csharp   - Run tests for C# library"
	@echo "  test-swift    - Run tests for Swift library"
	@echo "  test-cross-lang - Run cross-language multi-peer test"
	@echo "  example-rust  - Run Rust example client"
	@echo "  example-java  - Run Java example client"
	@echo "  test-integration - Run cross-language integration test"
	@echo "  clean         - Clean all libraries"
	@echo "  clean-rust    - Clean Rust library"
	@echo "  clean-java    - Clean Java library"
	@echo "  clean-csharp  - Clean C# library"
	@echo "  clean-swift   - Clean Swift library"
	@echo "  java-test-client - Build Java test client for cross-language tests"
	@echo "  help          - Show this help message"
