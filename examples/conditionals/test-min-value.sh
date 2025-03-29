#!/bin/bash

# Set the rules path to use our conditionals rules
export DQR_RULES_PATH=rules/examples/conditionals.csv

# Build the project
echo "Building DQR..."
cargo build

# Test under 18 example (should require guardian name)
echo -e "\nTesting under 18 example (should pass with guardian name):"
cargo run -- validate examples/conditionals/under-18-example.json

# Test over 18 example (should pass min_value validation)
echo -e "\nTesting over 18 example (should pass min_value validation):"
cargo run -- validate examples/conditionals/over-18-example.json

# Create a failing example for min_value validation (age = 17)
echo -e "\nCreating temporary file for failing min_value test..."
cat > /tmp/age-17-example.json << EOF
{
  "data": {
    "user": {
      "age": 17,
      "guardian": {
        "name": "John Doe"
      }
    },
    "payment": {
      "type": "credit_card",
      "credit_card": {
        "number": "4111111111111111",
        "expiry": "12/25",
        "cvv": "123"
      }
    }
  },
  "journey": "DEFAULT",
  "system": "ALL"
}
EOF

# Test the failing example (should fail min_value validation)
echo -e "\nTesting age 17 example (should fail min_value validation):"
cargo run -- validate /tmp/age-17-example.json

# Clean up
rm /tmp/age-17-example.json