#!/bin/bash
set -e

# Build the project
cargo build

# Define custom rules file
RULES_FILE="rules/dependencies.csv"

# Print rules information
echo "=== CONDITIONAL VALIDATION WITH DEPENDS_ON ==="
echo ""
echo "In this example, we have the following rules:"
echo ""
echo "1. Rule 300: Requires employment status (always runs)"
echo "2. Rule 301: Requires employer name (only if status is 'employed')"
echo "3. Rule 302: Requires employer address (only if status is 'employed')"
echo "4. Rule 303: Requires income (only if status is 'employed')"
echo "5. Rule 304: Validates income is a number (only if status is 'employed')"
echo ""
echo "Rules 301-304 all have: "
echo "  - depends_on_selector: \$.application.individuals.data[*].employment.status"
echo "  - depends_on_condition: equals:employed"
echo ""
echo "This means these rules are only applied when status = 'employed'"
echo "==================================================="
echo ""

# Run validation with our initial example (status = employed, missing employer fields)
echo "Example 1: Status = 'employed' but employer fields are empty"
echo "(Should fail validation for employer name/address)"
echo ""
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/dependencies/depends-on-example.json

# Run validation with our second example (status = unemployed)
echo -e "\nExample 2: Status = 'unemployed' so employer fields not required"
echo "(Should pass validation even with empty employer fields)"
echo ""
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/dependencies/depends-on-unemployed.json

echo ""
echo "This demonstrates how the depends_on feature allows for conditional"
echo "validation rules that only apply when specific conditions are met."