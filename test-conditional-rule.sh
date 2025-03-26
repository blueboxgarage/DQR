#!/bin/bash

PORT=${1:-8080}

# Use jq if available, otherwise just print the response
if command -v jq &> /dev/null; then
  FORMAT="| jq"
else
  FORMAT=""
  echo "Note: Install jq for prettier JSON output"
fi

# Start the server in the background
cargo run &
SERVER_PID=$!

# Give the server time to start
sleep 2

echo -e "\nTest 1: applicants.number=1 and valid name (should pass):"
cat > test-valid.json << EOF
{
  "data": {
    "applicants": {
      "names": {
        "first": "John"
      },
      "number": 1
    }
  },
  "journey": "DEFAULT_TEST",
  "system": "ACQ_TEST"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-valid.json http://localhost:$PORT/api/validate $FORMAT"
echo

echo -e "\nTest 2: applicants.number=1 and invalid name (too short):"
cat > test-invalid-name.json << EOF
{
  "data": {
    "applicants": {
      "names": {
        "first": "Jo"
      },
      "number": 1
    }
  },
  "journey": "DEFAULT_TEST",
  "system": "ACQ_TEST"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-invalid-name.json http://localhost:$PORT/api/validate $FORMAT"
echo

echo -e "\nTest 3: applicants.number=2 and invalid name (name check should be skipped):"
cat > test-number-2.json << EOF
{
  "data": {
    "applicants": {
      "names": {
        "first": "Jo"
      },
      "number": 2
    }
  },
  "journey": "DEFAULT_TEST",
  "system": "ACQ_TEST"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-number-2.json http://localhost:$PORT/api/validate $FORMAT"
echo

echo -e "\nTest 4: applicants.number=1 but different journey (rule should not apply):"
cat > test-different-journey.json << EOF
{
  "data": {
    "applicants": {
      "names": {
        "first": "Jo"
      },
      "number": 1
    }
  },
  "journey": "DEFAULT",
  "system": "ACQ_TEST"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-different-journey.json http://localhost:$PORT/api/validate $FORMAT"
echo

# Clean up
rm test-valid.json test-invalid-name.json test-number-2.json test-different-journey.json
kill $SERVER_PID

echo -e "\nTests completed!"