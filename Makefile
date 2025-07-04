# Ditto CoT Makefile
# Builds and cleans all language-specific libraries

# Default target
.PHONY: all
all: rust java csharp

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
		./gradlew build -x test; \
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

# Test targets
.PHONY: test
test: test-rust test-java test-csharp

.PHONY: test-rust
test-rust:
	@echo "Testing Rust library..."
	@cd rust && cargo nextest run

.PHONY: test-java
test-java:
	@echo "Testing Java library..."
	@if [ -f "java/build.gradle" ] || [ -f "java/build.gradle.kts" ]; then \
		cd java && ./gradlew :library:test -x checkstyleMain -x checkstyleTest -x jacocoTestCoverageVerification; \
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

# Clean all
.PHONY: clean
clean: clean-rust clean-java clean-csharp
	@echo "All libraries cleaned."

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
	@echo "  test          - Run tests for all libraries"
	@echo "  test-rust     - Run tests for Rust library"
	@echo "  test-java     - Run tests for Java library"
	@echo "  test-csharp   - Run tests for C# library"
	@echo "  clean         - Clean all libraries"
	@echo "  clean-rust    - Clean Rust library"
	@echo "  clean-java    - Clean Java library"
	@echo "  clean-csharp  - Clean C# library"
	@echo "  help          - Show this help message"
