#!/bin/bash
set -e

# Build the project
cargo build

# Export environment variables for rules path
export DQR_RULES_PATH=rules/examples/multiple-key-fields.csv

# Print rules information
echo "=== RULES WITH MULTIPLE KEY_FIELDS ==="
echo ""
echo "In this example, we have the following rules:"
echo ""
echo "1. Rule 100: Validates primary email format (single key_field)"
echo "2. Rule 101: Validates alternate email format (single key_field)"
echo "3. Rule 200: Validates email format with MULTIPLE key_fields:"
echo "   - application.individuals.contact.email"
echo "   - application.individuals.contact.alternate.email"
echo ""
echo "4. Rule 201: Validates phone format with MULTIPLE key_fields:"
echo "   - application.individuals.contact.phone"
echo "   - application.individuals.contact.alternate.phone"
echo ""
echo "When multiple key_fields are provided in a rule,"
echo "the rule is triggered for EITHER field that matches."
echo ""
echo "This means that rule #200 will be applied to both primary and"
echo "alternate email fields even though it only specifies one selector."
echo "==================================================="
echo ""

# Run validation with our custom examples
echo "Running validation with multiple key_fields example:"
echo "(The data has invalid primary email but valid alternate email)"
echo ""
cargo run -- validate examples/multiple-key-fields/multiple-key-fields-example.json

echo ""
echo "NOTE: In the output above, rule 200_combined_email_format_check"
echo "was triggered for the primary email even though it has the selector"
echo "for the primary email only. This is because it lists BOTH email fields"
echo "in its key_fields value, which means it will validate BOTH fields."
echo ""
echo "This demonstrates how a single rule can be applied to multiple related fields"
echo "by specifying multiple key_fields as a comma-separated list."