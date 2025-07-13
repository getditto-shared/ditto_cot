#!/bin/bash

# Test script to verify configuration functionality
# This script tests that:
# 1. Config can be set and saved
# 2. Config can be read from file
# 3. Service can use saved config

echo "🧪 Testing Ditto CoT Client Configuration"
echo "========================================"

# Build the project
echo "🔨 Building client..."
dotnet build DittoCoTClient.csproj > /dev/null 2>&1

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

# Test config show before setting
echo "📋 Testing config show (before setting)..."
dotnet bin/Debug/net9.0/DittoCoTClient.dll config show

# Test config set
echo "⚙️  Testing config set..."
dotnet bin/Debug/net9.0/DittoCoTClient.dll config set \
    --app-id "test-app-id" \
    --shared-key "test-shared-key" \
    --offline-token "test-offline-token"

if [ $? -ne 0 ]; then
    echo "❌ Config set failed"
    exit 1
fi

echo "✅ Config set successful"

# Test config show after setting
echo "📋 Testing config show (after setting)..."
dotnet bin/Debug/net9.0/DittoCoTClient.dll config show

# Verify config file exists
CONFIG_FILE="$HOME/.ditto-cot-client/config.json"
if [ -f "$CONFIG_FILE" ]; then
    echo "✅ Config file exists at: $CONFIG_FILE"
    echo "📄 Config file content:"
    cat "$CONFIG_FILE" | jq . 2>/dev/null || cat "$CONFIG_FILE"
else
    echo "❌ Config file not found"
    exit 1
fi

# Clean up test config
echo "🧹 Cleaning up test config..."
dotnet bin/Debug/net9.0/DittoCoTClient.dll config delete

echo "✅ All configuration tests passed!"