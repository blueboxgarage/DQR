#!/bin/bash
# DQR API Usage Examples

# Define the base URL
API_HOST=${DQR_HOST:-127.0.0.1}
API_PORT=${DQR_PORT:-8080}
BASE_URL="http://${API_HOST}:${API_PORT}"

# Check health status
echo "Checking DQR service health..."
curl -s "${BASE_URL}/health" | jq .

# Get all rules
echo -e "\nGetting all validation rules..."
curl -s "${BASE_URL}/api/rules" | jq .

# Create a rule (required field)
echo -e "\nCreating a required field rule..."
REQUIRED_RULE_ID=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-required.json | jq -r '.data // "unknown"')
echo "Created rule with ID: ${REQUIRED_RULE_ID}"

# Create a rule (regex validation)
echo -e "\nCreating a regex validation rule..."
REGEX_RULE_ID=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-regex.json | jq -r '.data // "unknown"')
echo "Created rule with ID: ${REGEX_RULE_ID}"

# Validate data
echo -e "\nValidating data..."
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @validation-request.json | jq .

# Delete rules
echo -e "\nDeleting rules..."
curl -s -X DELETE "${BASE_URL}/api/rules/${REQUIRED_RULE_ID}" | jq .
curl -s -X DELETE "${BASE_URL}/api/rules/${REGEX_RULE_ID}" | jq .

echo -e "\nAPI operations completed!"