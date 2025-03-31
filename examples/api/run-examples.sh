#!/bin/bash
# Run all DQR API examples

# Define colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Define the base URL
API_HOST=${DQR_HOST:-127.0.0.1}
API_PORT=${DQR_PORT:-8080}
BASE_URL="http://${API_HOST}:${API_PORT}"

echo -e "${BLUE}DQR API Examples Runner${NC}"
echo -e "Testing against server: ${BASE_URL}"

# Check if server is running
echo -e "\n${YELLOW}Checking if DQR service is running...${NC}"
if curl -s "${BASE_URL}/health" > /dev/null; then
  echo -e "${GREEN}DQR service is running!${NC}"
else
  echo -e "${RED}ERROR: DQR service does not appear to be running at ${BASE_URL}${NC}"
  echo "Please start the DQR service and try again."
  echo "You can start the service with:"
  echo "  cargo run -- --host ${API_HOST} --port ${API_PORT}"
  exit 1
fi

# Create rules
echo -e "\n${YELLOW}Creating rules...${NC}"

echo -e "${BLUE}Creating required field rule...${NC}"
REQUIRED_RULE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-required.json)
echo "${REQUIRED_RULE}" | jq .
REQUIRED_RULE_ID=$(echo "${REQUIRED_RULE}" | jq -r '.data // "unknown"')

echo -e "\n${BLUE}Creating regex validation rule...${NC}"
REGEX_RULE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-regex.json)
echo "${REGEX_RULE}" | jq .
REGEX_RULE_ID=$(echo "${REGEX_RULE}" | jq -r '.data // "unknown"')

echo -e "\n${BLUE}Creating length validation rule...${NC}"
LENGTH_RULE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-length.json)
echo "${LENGTH_RULE}" | jq .
LENGTH_RULE_ID=$(echo "${LENGTH_RULE}" | jq -r '.data // "unknown"')

echo -e "\n${BLUE}Creating min value rule...${NC}"
MIN_RULE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-min.json)
echo "${MIN_RULE}" | jq .
MIN_RULE_ID=$(echo "${MIN_RULE}" | jq -r '.data // "unknown"')

echo -e "\n${BLUE}Creating max value rule...${NC}"
MAX_RULE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-max.json)
echo "${MAX_RULE}" | jq .
MAX_RULE_ID=$(echo "${MAX_RULE}" | jq -r '.data // "unknown"')

echo -e "\n${BLUE}Creating enum validation rule...${NC}"
ENUM_RULE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @create-rule-enum.json)
echo "${ENUM_RULE}" | jq .
ENUM_RULE_ID=$(echo "${ENUM_RULE}" | jq -r '.data // "unknown"')

# Get all rules
echo -e "\n${YELLOW}Getting all rules...${NC}"
curl -s "${BASE_URL}/api/rules" | jq .

# Testing valid data
echo -e "\n${YELLOW}Validating data with valid request...${NC}"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @validation-request.json | jq .

# Testing invalid data (create a temporary file)
echo -e "\n${YELLOW}Validating data with invalid request...${NC}"

# Create an invalid validation request file
TMP_FILE=$(mktemp)
cat > "${TMP_FILE}" << 'EOF'
{
  "data": {
    "user": {
      "name": "John Doe",
      "email": "not-an-email",
      "age": 16,
      "password": "short"
    },
    "payment": {
      "amount": 15000,
      "currency": "BTC"
    }
  },
  "journey": "registration",
  "system": "user-portal"
}
EOF

curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${TMP_FILE}" | jq .

# Delete rules
echo -e "\n${YELLOW}Deleting rules...${NC}"

for RULE_ID in "$REQUIRED_RULE_ID" "$REGEX_RULE_ID" "$LENGTH_RULE_ID" "$MIN_RULE_ID" "$MAX_RULE_ID" "$ENUM_RULE_ID"; do
  if [ "$RULE_ID" != "unknown" ]; then
    echo -e "${BLUE}Deleting rule: ${RULE_ID}${NC}"
    curl -s -X DELETE "${BASE_URL}/api/rules/${RULE_ID}" | jq .
  fi
done

# Clean up
rm -f "${TMP_FILE}"

echo -e "\n${GREEN}API examples completed!${NC}"
echo "You have successfully run through all API examples for the DQR service."