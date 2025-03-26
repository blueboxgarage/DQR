#!/bin/bash

PORT=${1:-8080}

# Use jq if available, otherwise just print the response
if command -v jq &> /dev/null; then
  FORMAT="| jq"
else
  FORMAT=""
  echo "Note: Install jq for prettier JSON output"
fi

echo "Testing health endpoint..."
eval "curl -s http://localhost:$PORT/health $FORMAT"
echo

echo -e "\nTesting validation with valid data (DEFAULT journey, CUSTOMER system)..."
eval "curl -s -X POST -H \"Content-Type: application/json\" -d @examples/valid-request.json http://localhost:$PORT/api/validate $FORMAT"
echo

echo -e "\nTesting validation with invalid data (ALL_CHECKS journey, INVENTORY system)..."
eval "curl -s -X POST -H \"Content-Type: application/json\" -d @examples/invalid-request.json http://localhost:$PORT/api/validate $FORMAT"
echo

# Create a test case for PAYMENT_FLOW and CHECKOUT system
echo -e "\nTesting PAYMENT_FLOW journey with CHECKOUT system..."
cat > payment-test.json << EOF
{
  "data": {
    "name": "John Doe",
    "email": "john@example.com",
    "payment": {}
  },
  "journey": "PAYMENT_FLOW",
  "system": "CHECKOUT"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @payment-test.json http://localhost:$PORT/api/validate $FORMAT"
echo

# Fix the payment data and try again
echo -e "\nTesting PAYMENT_FLOW journey with CHECKOUT system (with payment.type)..."
cat > payment-fixed.json << EOF
{
  "data": {
    "name": "John Doe",
    "email": "john@example.com",
    "payment": {
      "type": "credit_card"
    }
  },
  "journey": "PAYMENT_FLOW",
  "system": "CHECKOUT"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @payment-fixed.json http://localhost:$PORT/api/validate $FORMAT"
echo

# Clean up temporary files
rm payment-test.json payment-fixed.json