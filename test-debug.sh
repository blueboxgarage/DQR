#!/bin/bash

# Start the server in debug mode
RUST_LOG=debug cargo run &
SERVER_PID=$!

# Give the server time to start
sleep 2

# Test case with missing payment.type
echo "Testing with missing payment.type"
cat > test-debug.json << EOF
{
  "data": {
    "name": "John Doe",
    "payment": {}
  },
  "journey": "PAYMENT_FLOW",
  "system": "CHECKOUT"
}
EOF

# Send the request
curl -s -X POST -H "Content-Type: application/json" -d @test-debug.json http://127.0.0.1:8080/api/validate
echo ""

# Clean up
rm test-debug.json
kill $SERVER_PID

echo "Done!"