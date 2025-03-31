#!/bin/bash
# Test script for all API endpoints in the DQR system
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

echo -e "${BLUE}Testing DQR API endpoints at ${BASE_URL}${NC}"

# Create a temporary directory for test files
TEMP_DIR=$(mktemp -d)
echo "Using temporary directory: ${TEMP_DIR}"

# Health Check
echo -e "\n${BLUE}Testing Health Check Endpoint${NC}"
echo "GET ${BASE_URL}/health"
curl -s "${BASE_URL}/health" | jq .

# Get All Rules
echo -e "\n${BLUE}Testing Get All Rules Endpoint${NC}"
echo "GET ${BASE_URL}/api/rules"
curl -s "${BASE_URL}/api/rules" | jq .

# Create a test rule
echo -e "\n${BLUE}Testing Create Rule Endpoint${NC}"
cat > "${TEMP_DIR}/new-rule.json" << 'EOF'
{
  "field_path": "$.user.email",
  "validation_type": "regex",
  "parameters": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
  "description": "Email must be in a valid format",
  "journey": "registration",
  "system": "user-api"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Content: $(cat ${TEMP_DIR}/new-rule.json)"
RESPONSE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/new-rule.json")

echo "Response: ${RESPONSE}"

# Extract rule ID from the response for later deletion
RULE_ID=$(echo $RESPONSE | jq -r '.data')
echo "Created rule with ID: ${RULE_ID}"

# Verify the rule was created by getting all rules again
echo -e "\n${BLUE}Verifying Rule Creation${NC}"
echo "GET ${BASE_URL}/api/rules"
curl -s "${BASE_URL}/api/rules" | jq .

# Test validation with a valid payload
echo -e "\n${BLUE}Testing Validation Endpoint with Valid Data${NC}"
cat > "${TEMP_DIR}/valid-payload.json" << 'EOF'
{
  "data": {
    "user": {
      "email": "test@example.com",
      "name": "Test User"
    }
  },
  "journey": "registration",
  "system": "user-api"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Content: $(cat ${TEMP_DIR}/valid-payload.json)"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/valid-payload.json" | jq .

# Test validation with an invalid payload
echo -e "\n${BLUE}Testing Validation Endpoint with Invalid Data${NC}"
cat > "${TEMP_DIR}/invalid-payload.json" << 'EOF'
{
  "data": {
    "user": {
      "email": "not-an-email",
      "name": "Test User"
    }
  },
  "journey": "registration",
  "system": "user-api"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Content: $(cat ${TEMP_DIR}/invalid-payload.json)"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${TEMP_DIR}/invalid-payload.json" | jq .

# Delete the test rule
echo -e "\n${BLUE}Testing Delete Rule Endpoint${NC}"
echo "DELETE ${BASE_URL}/api/rules/${RULE_ID}"
curl -s -X DELETE "${BASE_URL}/api/rules/${RULE_ID}" | jq .

# Verify the rule was deleted by getting all rules again
echo -e "\n${BLUE}Verifying Rule Deletion${NC}"
echo "GET ${BASE_URL}/api/rules"
curl -s "${BASE_URL}/api/rules" | jq .

# Clean up
echo -e "\n${BLUE}Cleaning up temporary files${NC}"
rm -rf "${TEMP_DIR}"

echo -e "\n${GREEN}API Tests Completed${NC}"