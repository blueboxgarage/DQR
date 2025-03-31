#!/bin/bash
# Test script for rule management API endpoints
# Don't fail on errors, so we can see all the test results
set +e

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Define the base URL
API_HOST=${DQR_HOST:-127.0.0.1}
API_PORT=${DQR_PORT:-8080}
BASE_URL="http://${API_HOST}:${API_PORT}"

echo -e "${BLUE}Testing DQR Rule Management API at ${BASE_URL}${NC}"

# Create a temporary directory for test files
TEMP_DIR=$(mktemp -d)
echo "Using temporary directory: ${TEMP_DIR}"

# Get initial rules
echo -e "\n${BLUE}Getting Initial Rules${NC}"
echo "GET ${BASE_URL}/api/rules"
curl -s "${BASE_URL}/api/rules" | jq .

# Create a test rule with required validation
echo -e "\n${BLUE}Creating Required Field Rule${NC}"
cat > "${TEMP_DIR}/required-rule.json" << 'EOF'
{
  "field_path": "$.customer.id",
  "validation_type": "required",
  "parameters": null,
  "description": "Customer ID is required",
  "journey": "onboarding",
  "system": "customer-service"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Content: $(cat ${TEMP_DIR}/required-rule.json)"
REQUIRED_RULE_ID=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/required-rule.json" | jq -r '.data')

echo "Created required rule with ID: ${REQUIRED_RULE_ID}"

# Create a test rule with regex validation
echo -e "\n${BLUE}Creating Regex Validation Rule${NC}"
cat > "${TEMP_DIR}/regex-rule.json" << 'EOF'
{
  "field_path": "$.customer.phone",
  "validation_type": "regex",
  "parameters": "^\\+?[0-9]{10,15}$",
  "description": "Phone number must be valid (10-15 digits, optional + prefix)",
  "journey": "onboarding",
  "system": "customer-service"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Content: $(cat ${TEMP_DIR}/regex-rule.json)"
REGEX_RULE_ID=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/regex-rule.json" | jq -r '.data')

echo "Created regex rule with ID: ${REGEX_RULE_ID}"

# Create a test rule with minimum length validation
echo -e "\n${BLUE}Creating Min Length Validation Rule${NC}"
cat > "${TEMP_DIR}/min-length-rule.json" << 'EOF'
{
  "field_path": "$.customer.name",
  "validation_type": "min_length",
  "parameters": "2",
  "description": "Customer name must be at least 2 characters",
  "journey": "onboarding",
  "system": "customer-service"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Content: $(cat ${TEMP_DIR}/min-length-rule.json)"
MIN_LENGTH_RULE_ID=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/min-length-rule.json" | jq -r '.data')

echo "Created min length rule with ID: ${MIN_LENGTH_RULE_ID}"

# Get all rules to verify creation
echo -e "\n${BLUE}Getting All Rules After Creation${NC}"
echo "GET ${BASE_URL}/api/rules"
curl -s "${BASE_URL}/api/rules" | jq .

# Test validation with all rules
echo -e "\n${BLUE}Testing Validation with All Rules${NC}"
cat > "${TEMP_DIR}/validation-test.json" << 'EOF'
{
  "data": {
    "customer": {
      "id": "12345",
      "name": "John Doe",
      "phone": "+12345678901"
    }
  },
  "journey": "onboarding",
  "system": "customer-service"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Content: $(cat ${TEMP_DIR}/validation-test.json)"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/validation-test.json" | jq .

# Test validation with invalid data
echo -e "\n${BLUE}Testing Validation with Invalid Data${NC}"
cat > "${TEMP_DIR}/invalid-test.json" << 'EOF'
{
  "data": {
    "customer": {
      "name": "J",
      "phone": "123"
    }
  },
  "journey": "onboarding",
  "system": "customer-service"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Content: $(cat ${TEMP_DIR}/invalid-test.json)"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/invalid-test.json" | jq .

# Delete the test rules
echo -e "\n${BLUE}Deleting Test Rules${NC}"

echo "DELETE ${BASE_URL}/api/rules/${REQUIRED_RULE_ID}"
curl -s -X DELETE "${BASE_URL}/api/rules/${REQUIRED_RULE_ID}" | jq .

echo "DELETE ${BASE_URL}/api/rules/${REGEX_RULE_ID}"
curl -s -X DELETE "${BASE_URL}/api/rules/${REGEX_RULE_ID}" | jq .

echo "DELETE ${BASE_URL}/api/rules/${MIN_LENGTH_RULE_ID}"
curl -s -X DELETE "${BASE_URL}/api/rules/${MIN_LENGTH_RULE_ID}" | jq .

# Get all rules to verify deletion
echo -e "\n${BLUE}Getting All Rules After Deletion${NC}"
echo "GET ${BASE_URL}/api/rules"
curl -s "${BASE_URL}/api/rules" | jq .

# Clean up
echo -e "\n${BLUE}Cleaning up temporary files${NC}"
rm -rf "${TEMP_DIR}"

echo -e "\n${GREEN}Rule API Tests Completed${NC}"