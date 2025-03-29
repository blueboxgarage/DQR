#!/bin/bash
set -e

# Build the project
cargo build

# Define custom rules file
RULES_FILE="rules/examples/journey-specific.csv"

# Print rules information
echo "=== JOURNEY-SPECIFIC VALIDATION RULES ==="
echo ""
echo "This example demonstrates two different validation journeys:"
echo ""
echo "1. ONBOARDING Journey:"
echo "   - First and last name are required"
echo "   - Name length is only checked if the name is provided"
echo "   - Empty name fails 'required' validation but skips length check"
echo ""
echo "2. VALIDATION_CHECK Journey:"
echo "   - If names section exists (if condition):"
echo "     - THEN check if all names are provided"
echo "     - ELSE check name length and require emergency contact"
echo ""
echo "These journeys show how validation can adapt to different contexts"
echo "and how dependencies can create conditional validation flows."
echo "==================================================="
echo ""

# Test ONBOARDING journey with complete data
echo "Example 1: ONBOARDING Journey - Complete Data"
echo "Expected: All validations pass"
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/journeys/onboarding-complete.json

# Test ONBOARDING journey with empty first name
echo -e "\nExample 2: ONBOARDING Journey - Empty First Name"
echo "Expected: First name required validation fails, but length check is skipped"
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/journeys/onboarding-empty-first.json

# Test VALIDATION_CHECK journey with complete data
echo -e "\nExample 3: VALIDATION_CHECK Journey - Complete Data"
echo "Expected: All validations pass (names section exists, so names are checked)"
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/journeys/validation-check-complete.json

# Test VALIDATION_CHECK journey with incomplete data
echo -e "\nExample 4: VALIDATION_CHECK Journey - Incomplete Data (Short Name & No Emergency Contact)"
echo "Expected: Name length validation fails, emergency contact required fails"
DQR_RULES_PATH=$RULES_FILE cargo run -- validate examples/journeys/validation-check-incomplete.json

echo ""
echo "These examples demonstrate how validation logic can differ by journey,"
echo "and how the same data can be validated differently in different contexts."
echo "This is useful for progressive validation across a multi-step process."