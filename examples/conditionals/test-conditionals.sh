#!/bin/bash
set -e

# Build the project
cargo build

# Define custom rules file
RULES_FILE="rules/examples/conditionals.csv"

# Print rules information
echo "=== TESTING CONDITIONAL VALIDATION RULES (IF/THEN/ELSE) ==="
echo ""
echo "In this example, we have two conditional validation scenarios:"
echo ""
echo "1. Age Check Conditional Rules:"
echo "   - IF user.age is a number:"
echo "     - THEN check if age is at least 18"
echo "     - ELSE require guardian name"
echo ""
echo "2. Payment Type Conditional Rules:"
echo "   - IF payment.type equals 'credit_card':"
echo "     - THEN validate credit card fields (number, expiry, cvv)"
echo "     - ELSE validate bank account fields (routing, account)"
echo ""
echo "==================================================="
echo ""

# Test with the credit card example
echo "Example 1: User is minor (age 15) and payment type is credit card"
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/conditionals/user-payment-example.json

# Create modified examples for different scenarios
echo -e "\nCreating bank account example..."
sed 's/"type": "credit_card"/"type": "bank_account"/' examples/conditionals/user-payment-example.json > examples/conditionals/bank-payment-example.json

echo -e "\nExample 2: User is minor but payment type is bank account"
echo "(Should require bank account fields instead of credit card fields)"
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/conditionals/bank-payment-example.json

echo ""
echo "This demonstrates how conditional validation rules can be used to:"
echo "1. Create branching logic with if/then/else conditions"
echo "2. Validate different fields based on the values in the data"
echo "3. Handle complex dependencies between different parts of the data"