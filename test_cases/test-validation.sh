#!/bin/bash
set -e

echo "Building DQR..."
cargo build

echo -e "\nTesting valid request..."
cargo run -- validate examples/valid-request.json
echo -e "\nExpected: No validation errors"

echo -e "\nTesting invalid request..."
cargo run -- validate examples/invalid-request.json
echo -e "\nExpected: First name is required (001_first_name_required) and First name must be at least 2 characters (002_idiv_00_frst_nm_lngth)"