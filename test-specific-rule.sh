#!/bin/bash

# Start the server in the background
cargo run &
SERVER_PID=$!

# Give the server time to start
sleep 2

# Colors for better readability
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Testing validation with different system/journey combinations${NC}"
echo "------------------------------------------------------------"

# Case 1: Data with missing payment type, but using DEFAULT journey and CUSTOMER system
# Should pass because rule9 requires PAYMENT_FLOW and CHECKOUT
echo -e "\n${BLUE}Case 1: Data with missing payment.type but DEFAULT journey and CUSTOMER system${NC}"
cat > test-case1.json << EOF
{
  "data": {
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com",
    "payment": {}
  },
  "journey": "DEFAULT",
  "system": "CUSTOMER"
}
EOF

RESULT1=$(curl -s -X POST -H "Content-Type: application/json" -d @test-case1.json http://127.0.0.1:8080/api/validate)
echo "Result: $RESULT1"
if [[ $RESULT1 == *"valid\":true"* ]]; then
  echo -e "${GREEN}✓ PASSED: Validation passed as expected (rule9 not applied)${NC}"
else
  echo -e "${RED}✗ FAILED: Validation should have passed${NC}"
fi

# Case 2: Same data but using PAYMENT_FLOW journey and DEFAULT system
# Should pass because rule9 requires CHECKOUT system
echo -e "\n${BLUE}Case 2: Data with missing payment.type but PAYMENT_FLOW journey and DEFAULT system${NC}"
cat > test-case2.json << EOF
{
  "data": {
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com",
    "payment": {}
  },
  "journey": "PAYMENT_FLOW",
  "system": "DEFAULT"
}
EOF

RESULT2=$(curl -s -X POST -H "Content-Type: application/json" -d @test-case2.json http://127.0.0.1:8080/api/validate)
echo "Result: $RESULT2"
if [[ $RESULT2 == *"valid\":true"* ]]; then
  echo -e "${GREEN}✓ PASSED: Validation passed as expected (rule9 not applied)${NC}"
else
  echo -e "${RED}✗ FAILED: Validation should have passed${NC}"
fi

# Case 3: Same data but using PAYMENT_FLOW journey and CHECKOUT system
# Should fail because rule9 applies and payment.type is missing
echo -e "\n${BLUE}Case 3: Data with missing payment.type with PAYMENT_FLOW journey and CHECKOUT system${NC}"
cat > test-case3.json << EOF
{
  "data": {
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com",
    "payment": {}
  },
  "journey": "PAYMENT_FLOW",
  "system": "CHECKOUT"
}
EOF

RESULT3=$(curl -s -X POST -H "Content-Type: application/json" -d @test-case3.json http://127.0.0.1:8080/api/validate)
echo "Result: $RESULT3"
if [[ $RESULT3 == *"valid\":false"* && $RESULT3 == *"rule9"* ]]; then
  echo -e "${GREEN}✓ PASSED: Validation failed as expected (rule9 applied)${NC}"
else
  echo -e "${RED}✗ FAILED: Validation should have failed with rule9 error${NC}"
fi

# Case 4: Fixed data with payment.type included
# Should pass even with PAYMENT_FLOW journey and CHECKOUT system
echo -e "\n${BLUE}Case 4: Data with payment.type included with PAYMENT_FLOW journey and CHECKOUT system${NC}"
cat > test-case4.json << EOF
{
  "data": {
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com",
    "payment": {
      "type": "credit_card"
    }
  },
  "journey": "PAYMENT_FLOW",
  "system": "CHECKOUT"
}
EOF

RESULT4=$(curl -s -X POST -H "Content-Type: application/json" -d @test-case4.json http://127.0.0.1:8080/api/validate)
echo "Result: $RESULT4"
if [[ $RESULT4 == *"valid\":true"* ]]; then
  echo -e "${GREEN}✓ PASSED: Validation passed as expected (rule9 satisfied)${NC}"
else
  echo -e "${RED}✗ FAILED: Validation should have passed${NC}"
fi

# Clean up
rm test-case1.json test-case2.json test-case3.json test-case4.json

# Stop the server
kill $SERVER_PID

echo -e "\n${BLUE}Test completed!${NC}"