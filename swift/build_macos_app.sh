#!/bin/bash

# Build script for creating a proper macOS CoTExampleApp.app bundle

set -e

echo "🔨 Building CoTExampleApp for macOS..."

# Clean any previous builds
rm -rf CoTExampleApp.app

# Build the executable (force rebuild)
echo "🧹 Cleaning previous build..."
swift package clean
rm -rf .build
echo "🔨 Building fresh..."
swift build --product CoTExampleApp --configuration release

# Get the built executable path
EXECUTABLE_PATH=$(swift build --show-bin-path --configuration release)/CoTExampleApp

echo "📦 Creating app bundle..."

# Create the app bundle structure
APP_NAME="CoTExampleApp"
APP_BUNDLE="${APP_NAME}.app"
CONTENTS_DIR="${APP_BUNDLE}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy the executable
cp "${EXECUTABLE_PATH}" "${MACOS_DIR}/${APP_NAME}"

# Copy .env file if it exists
if [[ -f ".env" ]]; then
    echo "📄 Copying .env file..."
    cp ".env" "${RESOURCES_DIR}/"
    echo "   ✅ Copied .env to app bundle"
else
    echo "   ⚠️  No .env file found - app may require manual configuration"
fi

# Find and copy required frameworks
echo "📚 Copying required frameworks..."
FRAMEWORKS_DIR="${CONTENTS_DIR}/Frameworks"
mkdir -p "${FRAMEWORKS_DIR}"

# Find the DittoSwift framework in the Swift Package Manager cache
SWIFT_BUILD_DIR=$(swift build --show-bin-path --configuration release)
SWIFT_BUILD_BASE=$(dirname "${SWIFT_BUILD_DIR}")

# Look for required frameworks
REQUIRED_FRAMEWORKS=("DittoSwift.framework" "DittoObjC.framework")

for framework_name in "${REQUIRED_FRAMEWORKS[@]}"; do
    echo "   Looking for $framework_name..."
    
    FRAMEWORK_PATHS=(
        "${SWIFT_BUILD_BASE}/release/$framework_name"
        "${SWIFT_BUILD_BASE}/debug/$framework_name"
        "${SWIFT_BUILD_BASE}/artifacts/ditto-swift-package/DittoSwift/DittoSwift.xcframework/macos-arm64_x86_64/$framework_name"
        "${SWIFT_BUILD_BASE}/checkouts/DittoSwiftPackage/DittoSwift.xcframework/macos-arm64_x86_64/$framework_name"
        "$(find ~/.swiftpm -name "$framework_name" -path "*/macos-arm64_x86_64/*" 2>/dev/null | head -1)"
    )

    FOUND_FRAMEWORK=""
    for framework_path in "${FRAMEWORK_PATHS[@]}"; do
        if [[ -d "$framework_path" ]]; then
            FOUND_FRAMEWORK="$framework_path"
            echo "   Found $framework_name at: $framework_path"
            break
        fi
    done

    if [[ -z "$FOUND_FRAMEWORK" ]]; then
        echo "   ❌ Could not find $framework_name"
        echo "   Searching for frameworks..."
        find ~/.swiftpm -name "$framework_name" 2>/dev/null || true
        find "${SWIFT_BUILD_BASE}" -name "$framework_name" 2>/dev/null || true
        echo "   Please ensure DittoSwift package is resolved: swift package resolve"
        exit 1
    fi

    # Copy the framework
    cp -R "${FOUND_FRAMEWORK}" "${FRAMEWORKS_DIR}/"
    echo "   ✅ Copied $framework_name to app bundle"
done

# Create Info.plist
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>live.ditto.cot.example</string>
    <key>CFBundleName</key>
    <string>CoT Example</string>
    <key>CFBundleDisplayName</key>
    <string>CoT Example</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>14.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2025 Ditto. All rights reserved.</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.developer-tools</string>
    <key>NSBluetoothAlwaysUsageDescription</key>
    <string>This app uses Bluetooth to discover and sync with nearby devices running Ditto.</string>
    <key>NSLocationWhenInUseUsageDescription</key>
    <string>This app uses your location to display CoT events on the map.</string>
    <key>NSLocationAlwaysAndWhenInUseUsageDescription</key>
    <string>This app uses your location to display and share CoT events with other users.</string>
</dict>
</plist>
EOF

# Create entitlements file for modern macOS compatibility
cat > "${CONTENTS_DIR}/entitlements.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <false/>
    <key>com.apple.security.network.client</key>
    <true/>
    <key>com.apple.security.network.server</key>
    <true/>
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>
</dict>
</plist>
EOF

# Make the executable actually executable
chmod +x "${MACOS_DIR}/${APP_NAME}"

# Fix the @rpath in the executable to point to the bundled frameworks
echo "🔧 Fixing framework paths..."
install_name_tool -add_rpath "@executable_path/../Frameworks" "${MACOS_DIR}/${APP_NAME}"

# Also fix the frameworks themselves to ensure they have correct install names
for framework_name in "${REQUIRED_FRAMEWORKS[@]}"; do
    framework_path="${FRAMEWORKS_DIR}/$framework_name"
    if [[ -d "$framework_path" ]]; then
        # Get the binary name (remove .framework extension)
        binary_name="${framework_name%.framework}"
        binary_path="$framework_path/$binary_name"
        
        # If it's a versioned framework, update the main binary path
        if [[ -f "$framework_path/Versions/A/$binary_name" ]]; then
            binary_path="$framework_path/Versions/A/$binary_name"
        fi
        
        if [[ -f "$binary_path" ]]; then
            echo "   Updating install name for $binary_name"
            install_name_tool -id "@rpath/$framework_name/$binary_name" "$binary_path" 2>/dev/null || true
        fi
    fi
done

# Try to sign the app (this will work if you have a developer certificate, otherwise it will fail gracefully)
echo "🔐 Attempting to sign the application..."
if codesign --sign - --force --deep "${APP_BUNDLE}" 2>/dev/null; then
    echo "   ✅ App signed successfully"
else
    echo "   ⚠️  App signing failed - app may require manual approval in System Preferences"
fi

echo "✅ Created ${APP_BUNDLE}"
echo "🚀 You can now run: open ${APP_BUNDLE}"
echo ""
echo "Or double-click the app in Finder to launch it properly as a macOS application."

# Check if we should kill any existing instances first
if [[ "$1" == "--relaunch" || "$2" == "--relaunch" ]]; then
    echo "🔄 Killing any existing instances..."
    pkill -f "CoTExampleApp" || true
    sleep 1
fi

# Optionally launch it
if [[ "$1" == "--launch" || "$1" == "--relaunch" ]]; then
    echo "🚀 Launching ${APP_BUNDLE}..."
    open "${APP_BUNDLE}"
fi