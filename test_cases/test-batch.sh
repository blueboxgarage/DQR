#!/bin/bash
# Build the project but don't exit on errors in the validation
set -e

echo "Building DQR..."
cargo build

# Turn off exit on error for the validation part
set +e

echo -e "\nTesting batch validation..."
for file in examples/basic/*.json; do
  echo -e "\nValidating $file..."
  cargo run -- validate "$file"
  echo "Exit code $?"
done

echo -e "\nBatch validation complete."