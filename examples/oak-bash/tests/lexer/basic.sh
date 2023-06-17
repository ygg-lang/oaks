#!/bin/bash

# Basic bash script for testing
set -euo pipefail

# Function to print a greeting
greet() {
    local name="$1"
    echo "Hello, $name!"
}

# Variable declarations
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly LOG_FILE="/tmp/test.log"

# Arrays and loops
fruits=("apple" "banana" "orange")
for fruit in "${fruits[@]}"; do
    echo "Processing $fruit"
done

# Conditional statements
if [[ -f "$LOG_FILE" ]]; then
    echo "Log file exists"
elif [[ -d "/tmp" ]]; then
    echo "Temp directory exists"
else
    echo "Neither file nor directory found"
fi

# Case statement
case "$1" in
    start)
        echo "Starting service..."
        ;;
    stop)
        echo "Stopping service..."
        ;;
    restart)
        echo "Restarting service..."
        ;;
    *)
        echo "Unknown command: $1"
        exit 1
        ;;
esac

# Command substitution
files_count=$(ls -1 "$SCRIPT_DIR" | wc -l)
echo "Found $files_count files in script directory"

# Redirections and pipes
echo "Test message" > "$LOG_FILE" 2>&1
cat "$LOG_FILE" | grep "Test" || true