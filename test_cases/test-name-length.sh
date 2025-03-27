#!/bin/bash
set -e

echo "Building DQR..."
cargo build

echo -e "\nTesting valid name length..."
cargo run -- validate examples/valid-name-length.json
echo -e "\nExpected: No validation errors"

echo -e "\nTesting invalid name length..."
cargo run -- validate examples/invalid-name-length.json
echo -e "\nExpected: First name must be at least 2 characters (rule 002_idiv_00_frst_nm_lngth)"