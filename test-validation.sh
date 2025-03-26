#!/bin/bash
set -e

echo "Building DQR..."
cargo build

echo -e "\nTesting valid request..."
cargo run -- validate examples/valid-request.json
echo -e "\nExpected: No validation errors"

echo -e "\nTesting invalid request..."
cargo run -- validate examples/invalid-request.json
echo -e "\nExpected: First name is required"