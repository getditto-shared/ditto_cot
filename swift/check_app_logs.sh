#!/bin/bash

# Script to check if the app is running and view its logs

echo "🔍 Checking if CoTExampleApp is running..."

# Check if the app is running
if pgrep -f "CoTExampleApp" > /dev/null; then
    echo "✅ CoTExampleApp is running"
    PID=$(pgrep -f "CoTExampleApp")
    echo "   Process ID: $PID"
else
    echo "❌ CoTExampleApp is not running"
fi

echo ""
echo "📋 Options:"
echo "1. Default: Show recent logs from last 5 minutes"
echo "2. 'stream' or 'live': Stream live logs (press Ctrl+C to stop)"
echo "3. 'terminal': Run app directly in terminal to see console output"
echo ""

if [[ "$1" == "stream" || "$1" == "live" ]]; then
    echo "🔴 Streaming live logs for CoTExampleApp (press Ctrl+C to stop)..."
    echo ""
    log stream --predicate 'process == "CoTExampleApp"' --level debug
elif [[ "$1" == "terminal" ]]; then
    echo "🚀 Running app directly in terminal to see console output..."
    echo ""
    cd "$(dirname "$0")"
    if [[ -d "CoTExampleApp.app" ]]; then
        ./CoTExampleApp.app/Contents/MacOS/CoTExampleApp
    else
        echo "❌ CoTExampleApp.app not found. Run './build_macos_app.sh --launch' first."
        exit 1
    fi
else
    echo "📋 Recent app logs (last 5 minutes):"
    echo "======================================="
    
    # Show recent logs from the app
    log show --predicate 'process == "CoTExampleApp"' --last 5m --style syslog | tail -50
    
    echo ""
    echo "To view real-time logs, run:"
    echo "./check_app_logs.sh stream"
    echo ""
    echo "To run app in terminal with direct console output:"
    echo "./check_app_logs.sh terminal"
fi