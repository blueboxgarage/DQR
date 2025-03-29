#!/bin/bash
set -e

# Build the project
cargo build

# Export environment variables for rules path
export DQR_RULES_PATH=rules/examples/advanced-key-fields.csv

# Print rules information
echo "=== ADVANCED KEY_FIELDS USAGE ==="
echo ""
echo "This example shows unusual but powerful key_fields patterns:"
echo ""
echo "1. Validator targets a different field than its key_field:"
echo "   - KEY_FIELD: user.profile.type"
echo "   - SELECTOR: $.user.security.mfa_enabled" 
echo "   - PURPOSE: When processing profile.type, validate security.mfa_enabled"
echo ""
echo "2. One rule triggered by completely unrelated fields:"
echo "   - KEY_FIELDS: user.profile.type, user.security.mfa_enabled, user.contact"
echo "   - SELECTOR: $.user.contact.email"
echo "   - PURPOSE: Email validation is triggered by multiple unrelated fields" 
echo ""
echo "This demonstrates that key_fields (WHEN to validate) and"
echo "selectors (WHAT to validate) can be completely different."
echo "==================================================="
echo ""

# Run validation with our advanced example
echo "Running validation with advanced key_fields example:"
echo ""
cargo run -- validate examples/multiple-key-fields/advanced-key-fields-example.json

echo ""
echo "Note: The validation errors occur because:"
echo "1. Premium users should have MFA enabled (mfa_enabled is false)"
echo "2. Phone is required for premium users (phone is provided but this shows the concept)"
echo ""
echo "This demonstrates how key_fields can trigger validation of entirely different"
echo "parts of your data structure, enabling complex, relationship-based validation rules."