#!/bin/bash

# Base64 key validation tool for Ditto shared keys

KEY="$1"

if [ -z "$KEY" ]; then
    echo "Usage: $0 <base64-key>"
    echo "Example: $0 'MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg0r0Wrcs46zDjpSUAo98MumurAGX98VpSLKNffLRL0qhRANCAAQ4mhBlCrwlhz3ClWmr98bdHYUNXJawYj0fbU6wYySMKvIRx9o/L6AqBvih4Cd7+0fDKRbCpZtMvDTVPKFU60'"
    exit 1
fi

echo "🔍 Analyzing shared key..."
echo "Key length: ${#KEY} characters"
echo "Key: $KEY"
echo

# Check for invalid characters
echo "🔍 Checking for invalid base64 characters..."
INVALID_CHARS=$(echo "$KEY" | tr -d 'A-Za-z0-9+/=' | wc -c)
if [ $INVALID_CHARS -gt 0 ]; then
    echo "❌ Found $INVALID_CHARS invalid base64 characters"
    echo "Invalid characters: $(echo "$KEY" | tr -d 'A-Za-z0-9+/=')"
else
    echo "✅ All characters are valid base64"
fi

# Check padding
echo
echo "🔍 Checking base64 padding..."
REMAINDER=$((${#KEY} % 4))
if [ $REMAINDER -eq 0 ]; then
    echo "✅ No padding needed (length is multiple of 4)"
elif [ $REMAINDER -eq 2 ]; then
    echo "⚠️  May need 2 padding characters (==)"
    PADDED_KEY="${KEY}=="
    echo "Suggested key with padding: $PADDED_KEY"
elif [ $REMAINDER -eq 3 ]; then
    echo "⚠️  May need 1 padding character (=)"
    PADDED_KEY="${KEY}="
    echo "Suggested key with padding: $PADDED_KEY"
else
    echo "❌ Invalid length for base64 (remainder: $REMAINDER)"
fi

# Test base64 decoding
echo
echo "🔍 Testing base64 decoding..."
if echo "$KEY" | base64 -d > /dev/null 2>&1; then
    echo "✅ Base64 decoding successful"
    DECODED_LENGTH=$(echo "$KEY" | base64 -d | wc -c)
    echo "Decoded length: $DECODED_LENGTH bytes"
    echo "First 20 bytes (hex): $(echo "$KEY" | base64 -d | xxd -l 20 -p)"
else
    echo "❌ Base64 decoding failed"
    
    # Try with padding if needed
    if [ ! -z "$PADDED_KEY" ]; then
        echo "🔍 Trying with padding..."
        if echo "$PADDED_KEY" | base64 -d > /dev/null 2>&1; then
            echo "✅ Base64 decoding successful with padding!"
            echo "Use this key: $PADDED_KEY"
        else
            echo "❌ Still fails with padding"
        fi
    fi
fi