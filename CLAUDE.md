# Claude Configuration for Ditto CoT Library

This document provides specific instructions for Claude when working with the Ditto CoT library project.

## Project Context

Multi-language libraries (starting from a single managed JSON Schema) for translating between Cursor-on-Target (CoT) XML events and Ditto-compatible CRDT documents.

## Linear Integration Guidelines

**IMPORTANT:** When working with Linear tickets:

- **NEVER** automatically change the status or state of Linear issues
- **NEVER** transition issues between states (e.g., from "In Progress" to "Done")
- **DO** read and reference Linear tickets for context
- **DO** add comments to issues when explicitly requested
- **DO NOT** modify any issue properties (assignee, labels, priority, etc.)
- All status transitions should be handled manually by the development team

## Development Guidelines

### Testing Requirements

- Always run tests before suggesting code completion
- For Ditto CoT library development:
  - All tests: `make test`
- Suggest running lint and type checking if available
- Verify that all tests pass before marking any task as complete

### Build Commands

- Build debug: `make clean`

## Code Style Guidelines

### General

- Follow existing code conventions in the codebase
- Use meaningful variable and function names
- Maintain consistent indentation (check existing files)
- Avoid adding debug prints or logs unless specifically requested

### Java/Android/Kotlin Specific

- Follow Java/Androind/Kotlin coding conventions
- Use proper null safety patterns
- Prefer data classes for data models
- Use appropriate visibility modifiers

### Rust Specific

- Follow Rust coding conventions and idioms

### C# Specific

- Follow C# and .NET coding conventions and idioms

### Swift/SwiftUI Specific

- Follow Swift API Design Guidelines
- Use protocol-oriented design patterns
- Prefer value types where appropriate
- Use modern concurrency (async/await) patterns
- Follow SwiftUI best practices for state management

### Documentation

- Do not create documentation files unless explicitly requested
- Keep code comments minimal and meaningful
- Update existing documentation when making related changes

## Important Reminders

1. **Security**: Never commit sensitive information like API keys, passwords, or tokens
2. **Dependencies**: Check existing dependencies before suggesting new ones
3. **File Creation**: Prefer modifying existing files over creating new ones
4. **Breaking Changes**: Always highlight potential breaking changes
5. **Error Handling**: Implement proper error handling for all new features

## Learning More About Ditto

When you need more context about Ditto's architecture, conventions, or specific implementations:

https://docs.ditto.live

For Rust SDK: https://software.ditto.live/rust/Ditto/4.11.0/x86_64-unknown-linux-gnu/docs/dittolive_ditto/index.html
For Java SDK: https://software.ditto.live/java/ditto-java/4.11.0-preview.1/api-reference/
For C# SDK: https://software.ditto.live/dotnet/Ditto/4.11.0/api-reference/
For Swift SDK: https://software.ditto.live/swift/Ditto/4.11.0/index.html

## Swift/SwiftUI Integration Plan

The Swift implementation for Ditto CoT library follows a 7-phase approach:

### Phase 1: Foundation & Schema Integration (2-3 weeks)
- Swift Package Manager setup with proper module structure
- Schema code generation (JSON Schema → Swift Codable types)
- Build integration with existing Makefile system
- CI/CD pipeline extension for Swift builds and tests

### Phase 2: Core CoT Event Handling (3-4 weeks)
- CoTEvent model with builder pattern (similar to Rust implementation)
- XML parsing/serialization using XMLCoder or custom parser
- Event validation against XSD schema
- Type-safe event builders for each CoT event type

### Phase 3: Ditto SDK Integration (2-3 weeks)
- Document conversion following established CRDT optimization patterns
- Observer integration with Combine framework
- Async/await support for modern Swift concurrency
- R-field reconstruction handling

### Phase 4: SwiftUI Integration Layer (2-3 weeks)
- ObservableObject wrappers for CoT event streams
- View models for common CoT event display patterns
- Data binding utilities for real-time updates
- SwiftUI-specific convenience APIs

### Phase 5: Testing Infrastructure (3-4 weeks)
- Unit tests using XCTest framework
- Integration tests with mock Ditto instances
- Cross-language validation tests against Java/Rust implementations
- SwiftUI UI tests for interface components
- Performance benchmarks using XCTMetric

### Phase 6: Documentation & Examples (1-2 weeks)
- Swift-specific documentation following existing patterns
- Example iOS/macOS app demonstrating full integration
- Migration guides from other Ditto SDK patterns
- API reference generation using Swift-DocC

### Phase 7: Advanced Features & Optimization (2-3 weeks)
- CRDT optimization refinements for Swift-specific patterns
- Memory management optimization for iOS constraints
- Background processing support for iOS/macOS apps
- SwiftUI performance optimizations for large event lists

### Implementation Principles
- Protocol-oriented design following Swift best practices
- Value types where appropriate for thread safety
- Reference types for stateful components (stores, converters)
- Modern concurrency with async/await and actors where beneficial
- Platform-specific considerations for iOS/macOS/watchOS/tvOS

### When in Doubt, Ask First

If you don't know how to do something, and you can't find accurate and up-to-date information from sources such as online documentation, content in Notion or Linear, or a tool's help output or man pages, then ask about an approach before doing it instead of guessing.
