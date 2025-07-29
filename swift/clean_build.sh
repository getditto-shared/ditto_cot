#!/bin/bash
set -e

echo "🧹 Cleaning all build artifacts..."

# Clean Swift Package Manager
echo "  Cleaning Swift Package Manager..."
swift package clean

# Remove build directories
echo "  Removing .build directory..."
rm -rf .build

# Remove any built apps
echo "  Removing built apps..."
rm -rf CoTExampleApp.app
rm -rf Build/
rm -rf DerivedData/

# Clear module cache
echo "  Clearing module cache..."
rm -rf ~/Library/Caches/org.swift.swiftpm

# Clear Xcode derived data for anything CoT related
echo "  Clearing Xcode derived data..."
rm -rf ~/Library/Developer/Xcode/DerivedData/*CoT* 2>/dev/null || true

echo "✅ Clean complete!"
echo ""
echo "To rebuild the app, run:"
echo "  swift build"
echo "  or open in Xcode and build"