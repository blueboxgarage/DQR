#!/bin/bash
# API Examples for DQR
# Comprehensive examples for all API endpoints in the DQR service

# Define colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Define the base URL
API_HOST=${DQR_HOST:-127.0.0.1}
API_PORT=${DQR_PORT:-8080}
BASE_URL="http://${API_HOST}:${API_PORT}"

echo -e "${BLUE}DQR API Examples${NC}"
echo -e "Base URL: ${BASE_URL}"

# Create a temporary directory for example files
EXAMPLES_DIR=$(mktemp -d)
echo "Using directory for examples: ${EXAMPLES_DIR}"

# ===========================================================
# 1. HEALTH CHECK API
# ===========================================================
echo -e "\n${YELLOW}====== HEALTH CHECK API ======${NC}"
echo "GET ${BASE_URL}/health"
echo "Example Output:"
curl -s "${BASE_URL}/health" | jq .

# ===========================================================
# 2. GET ALL RULES API
# ===========================================================
echo -e "\n${YELLOW}====== GET ALL RULES API ======${NC}"
echo "GET ${BASE_URL}/api/rules"
echo "Example Output:"
curl -s "${BASE_URL}/api/rules" | jq .

# ===========================================================
# 3. CREATE RULE API - VARIOUS EXAMPLES
# ===========================================================
echo -e "\n${YELLOW}====== CREATE RULE API - VARIOUS EXAMPLES ======${NC}"

# 3.1 Required Field Validation
echo -e "\n${BLUE}Example 1: Required Field Validation${NC}"
cat > "${EXAMPLES_DIR}/required-rule.json" << 'EOF'
{
  "field_path": "$.user.email",
  "validation_type": "required",
  "description": "User email is required",
  "journey": "registration",
  "system": "user-portal"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Request Body:"
cat "${EXAMPLES_DIR}/required-rule.json" | jq .
echo "Example Output:"
REQUIRED_RULE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/required-rule.json")
echo "${REQUIRED_RULE_RESPONSE}" | jq .
REQUIRED_RULE_ID=$(echo "${REQUIRED_RULE_RESPONSE}" | jq -r '.data // "unknown"')

# 3.2 Regex Pattern Validation
echo -e "\n${BLUE}Example 2: Regex Pattern Validation${NC}"
cat > "${EXAMPLES_DIR}/regex-rule.json" << 'EOF'
{
  "field_path": "$.user.email",
  "validation_type": "regex",
  "parameters": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
  "description": "User email must be in valid format",
  "journey": "registration",
  "system": "user-portal"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Request Body:"
cat "${EXAMPLES_DIR}/regex-rule.json" | jq .
echo "Example Output:"
REGEX_RULE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/regex-rule.json")
echo "${REGEX_RULE_RESPONSE}" | jq .
REGEX_RULE_ID=$(echo "${REGEX_RULE_RESPONSE}" | jq -r '.data // "unknown"')

# 3.3 Length Validation
echo -e "\n${BLUE}Example 3: Length Validation${NC}"
cat > "${EXAMPLES_DIR}/length-rule.json" << 'EOF'
{
  "field_path": "$.user.password",
  "validation_type": "length",
  "parameters": "min=8,max=64",
  "description": "Password must be between 8 and 64 characters",
  "journey": "registration",
  "system": "user-portal"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Request Body:"
cat "${EXAMPLES_DIR}/length-rule.json" | jq .
echo "Example Output:"
LENGTH_RULE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/length-rule.json")
echo "${LENGTH_RULE_RESPONSE}" | jq .
LENGTH_RULE_ID=$(echo "${LENGTH_RULE_RESPONSE}" | jq -r '.data // "unknown"')

# 3.4 Minimum Value Validation
echo -e "\n${BLUE}Example 4: Minimum Value Validation${NC}"
cat > "${EXAMPLES_DIR}/min-value-rule.json" << 'EOF'
{
  "field_path": "$.user.age",
  "validation_type": "min",
  "parameters": "18",
  "description": "User must be at least 18 years old",
  "journey": "age_verification",
  "system": "ALL"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Request Body:"
cat "${EXAMPLES_DIR}/min-value-rule.json" | jq .
echo "Example Output:"
MIN_VALUE_RULE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/min-value-rule.json")
echo "${MIN_VALUE_RULE_RESPONSE}" | jq .
MIN_VALUE_RULE_ID=$(echo "${MIN_VALUE_RULE_RESPONSE}" | jq -r '.data // "unknown"')

# 3.5 Maximum Value Validation
echo -e "\n${BLUE}Example 5: Maximum Value Validation${NC}"
cat > "${EXAMPLES_DIR}/max-value-rule.json" << 'EOF'
{
  "field_path": "$.payment.amount",
  "validation_type": "max",
  "parameters": "10000",
  "description": "Payment amount cannot exceed 10000",
  "journey": "payment_processing",
  "system": "checkout"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Request Body:"
cat "${EXAMPLES_DIR}/max-value-rule.json" | jq .
echo "Example Output:"
MAX_VALUE_RULE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/max-value-rule.json")
echo "${MAX_VALUE_RULE_RESPONSE}" | jq .
MAX_VALUE_RULE_ID=$(echo "${MAX_VALUE_RULE_RESPONSE}" | jq -r '.data // "unknown"')

# 3.6 Enum Validation
echo -e "\n${BLUE}Example 6: Enum Validation${NC}"
cat > "${EXAMPLES_DIR}/enum-rule.json" << 'EOF'
{
  "field_path": "$.payment.currency",
  "validation_type": "enum",
  "parameters": "USD,EUR,GBP,JPY,CAD",
  "description": "Currency must be one of the supported currencies",
  "journey": "payment_processing",
  "system": "checkout"
}
EOF

echo "POST ${BASE_URL}/api/rules"
echo "Request Body:"
cat "${EXAMPLES_DIR}/enum-rule.json" | jq .
echo "Example Output:"
ENUM_RULE_RESPONSE=$(curl -s -X POST "${BASE_URL}/api/rules" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/enum-rule.json")
echo "${ENUM_RULE_RESPONSE}" | jq .
ENUM_RULE_ID=$(echo "${ENUM_RULE_RESPONSE}" | jq -r '.data // "unknown"')

# ===========================================================
# 4. VALIDATION API - VARIOUS EXAMPLES
# ===========================================================
echo -e "\n${YELLOW}====== VALIDATION API - VARIOUS EXAMPLES ======${NC}"

# 4.1 Valid User Registration
echo -e "\n${BLUE}Example 1: Valid User Registration${NC}"
cat > "${EXAMPLES_DIR}/valid-registration.json" << 'EOF'
{
  "data": {
    "user": {
      "email": "john.doe@example.com",
      "password": "securePassword123",
      "age": 30
    }
  },
  "journey": "registration",
  "system": "user-portal"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Request Body:"
cat "${EXAMPLES_DIR}/valid-registration.json" | jq .
echo "Example Output:"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/valid-registration.json" | jq .

# 4.2 Invalid User Registration (Missing Email)
echo -e "\n${BLUE}Example 2: Invalid User Registration (Missing Email)${NC}"
cat > "${EXAMPLES_DIR}/invalid-registration-missing-email.json" << 'EOF'
{
  "data": {
    "user": {
      "password": "securePassword123",
      "age": 30
    }
  },
  "journey": "registration",
  "system": "user-portal"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Request Body:"
cat "${EXAMPLES_DIR}/invalid-registration-missing-email.json" | jq .
echo "Example Output:"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/invalid-registration-missing-email.json" | jq .

# 4.3 Invalid User Registration (Invalid Email Format)
echo -e "\n${BLUE}Example 3: Invalid User Registration (Invalid Email Format)${NC}"
cat > "${EXAMPLES_DIR}/invalid-registration-bad-email.json" << 'EOF'
{
  "data": {
    "user": {
      "email": "not-an-email",
      "password": "securePassword123",
      "age": 30
    }
  },
  "journey": "registration",
  "system": "user-portal"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Request Body:"
cat "${EXAMPLES_DIR}/invalid-registration-bad-email.json" | jq .
echo "Example Output:"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/invalid-registration-bad-email.json" | jq .

# 4.4 Invalid Age Verification
echo -e "\n${BLUE}Example 4: Invalid Age Verification${NC}"
cat > "${EXAMPLES_DIR}/invalid-age.json" << 'EOF'
{
  "data": {
    "user": {
      "email": "teen@example.com", 
      "age": 16
    }
  },
  "journey": "age_verification",
  "system": "ALL"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Request Body:"
cat "${EXAMPLES_DIR}/invalid-age.json" | jq .
echo "Example Output:"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/invalid-age.json" | jq .

# 4.5 Valid Payment Processing
echo -e "\n${BLUE}Example 5: Valid Payment Processing${NC}"
cat > "${EXAMPLES_DIR}/valid-payment.json" << 'EOF'
{
  "data": {
    "payment": {
      "amount": 5000,
      "currency": "USD"
    },
    "user": {
      "email": "john.doe@example.com"
    }
  },
  "journey": "payment_processing",
  "system": "checkout"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Request Body:"
cat "${EXAMPLES_DIR}/valid-payment.json" | jq .
echo "Example Output:"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/valid-payment.json" | jq .

# 4.6 Invalid Payment Processing (Exceeds Maximum)
echo -e "\n${BLUE}Example 6: Invalid Payment Processing (Exceeds Maximum)${NC}"
cat > "${EXAMPLES_DIR}/invalid-payment-amount.json" << 'EOF'
{
  "data": {
    "payment": {
      "amount": 15000,
      "currency": "USD"
    },
    "user": {
      "email": "john.doe@example.com"
    }
  },
  "journey": "payment_processing",
  "system": "checkout"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Request Body:"
cat "${EXAMPLES_DIR}/invalid-payment-amount.json" | jq .
echo "Example Output:"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/invalid-payment-amount.json" | jq .

# 4.7 Invalid Payment Processing (Invalid Currency)
echo -e "\n${BLUE}Example 7: Invalid Payment Processing (Invalid Currency)${NC}"
cat > "${EXAMPLES_DIR}/invalid-payment-currency.json" << 'EOF'
{
  "data": {
    "payment": {
      "amount": 5000,
      "currency": "BTC"
    },
    "user": {
      "email": "john.doe@example.com"
    }
  },
  "journey": "payment_processing",
  "system": "checkout"
}
EOF

echo "POST ${BASE_URL}/api/validate"
echo "Request Body:"
cat "${EXAMPLES_DIR}/invalid-payment-currency.json" | jq .
echo "Example Output:"
curl -s -X POST "${BASE_URL}/api/validate" \
  -H "Content-Type: application/json" \
  -d @"${EXAMPLES_DIR}/invalid-payment-currency.json" | jq .

# ===========================================================
# 5. DELETE RULE API
# ===========================================================
echo -e "\n${YELLOW}====== DELETE RULE API ======${NC}"

# Only delete if the rule IDs are valid (not "unknown")
for RULE_ID in "$REQUIRED_RULE_ID" "$REGEX_RULE_ID" "$LENGTH_RULE_ID" "$MIN_VALUE_RULE_ID" "$MAX_VALUE_RULE_ID" "$ENUM_RULE_ID"; do
  if [ "$RULE_ID" != "unknown" ]; then
    echo -e "\n${BLUE}Deleting Rule: ${RULE_ID}${NC}"
    echo "DELETE ${BASE_URL}/api/rules/${RULE_ID}"
    echo "Example Output:"
    curl -s -X DELETE "${BASE_URL}/api/rules/${RULE_ID}" | jq .
  fi
done

# ===========================================================
# Verify Cleanup
# ===========================================================
echo -e "\n${BLUE}Verifying rules after deletion${NC}"
echo "GET ${BASE_URL}/api/rules"
curl -s "${BASE_URL}/api/rules" | jq .

# Cleanup temporary files
echo -e "\n${BLUE}Cleaning up temporary files${NC}"
rm -rf "${EXAMPLES_DIR}"

echo -e "\n${GREEN}API examples completed!${NC}"
echo -e "These examples demonstrate all available DQR API endpoints with various use cases."