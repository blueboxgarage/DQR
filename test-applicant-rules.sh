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

echo -e "\nTesting valid applicant data (passing both rules):"
cat > test-valid-applicant.json << EOF
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

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-valid-applicant.json http://localhost:$PORT/api/validate $FORMAT"
echo

echo -e "\nTesting invalid first name (too short):"
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

echo -e "\nTesting invalid number (not equal to 1):"
cat > test-invalid-number.json << EOF
{
  "data": {
    "applicants": {
      "names": {
        "first": "John"
      },
      "number": 2
    }
  },
  "journey": "DEFAULT_TEST",
  "system": "ACQ_TEST"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-invalid-number.json http://localhost:$PORT/api/validate $FORMAT"
echo

echo -e "\nTesting with different journey (rules should not apply):"
cat > test-different-journey.json << EOF
{
  "data": {
    "applicants": {
      "names": {
        "first": "Jo"
      },
      "number": 2
    }
  },
  "journey": "DEFAULT",
  "system": "ACQ_TEST"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-different-journey.json http://localhost:$PORT/api/validate $FORMAT"
echo

echo -e "\nTesting with different system (rules should not apply):"
cat > test-different-system.json << EOF
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
  "system": "CUSTOMER"
}
EOF

eval "curl -s -X POST -H \"Content-Type: application/json\" -d @test-different-system.json http://localhost:$PORT/api/validate $FORMAT"
echo

# Clean up
rm test-valid-applicant.json test-invalid-name.json test-invalid-number.json test-different-journey.json test-different-system.json
kill $SERVER_PID

echo -e "\nTest completed!"