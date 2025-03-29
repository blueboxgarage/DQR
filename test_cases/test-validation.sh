#!/bin/bash
# Don't fail on errors, so we can see all the test results
set +e

echo "Building DQR..."
cargo build

echo -e "\nTesting valid request..."
cargo run -- validate examples/basic/valid-request.json
echo -e "\nExpected: No validation errors"

echo -e "\nTesting invalid request..."
cargo run -- validate examples/basic/invalid-request.json
echo -e "\nExpected: First name is required (001_first_name_required) and First name must be at least 2 characters (002_idiv_00_frst_nm_lngth)"

# Create and test a file with multiple potential validation failures
echo -e "\nCreating test file with multiple potential validation failures..."
cat > examples/basic/test_multiple_validations.json << 'EOF'
{
  "data": {
    "application": {
      "individuals": {
        "number": 1,
        "data": [
          {
            "names": [
              {
                "name": {
                  "first": "",
                  "last": ""
                }
              }
            ],
            "age": "thirty",
            "contact": {
              "email": "not-an-email",
              "phone": 12345
            },
            "address": {
              "street": "",
              "city": null,
              "zip": "ABCDEF",
              "coordinates": "not-an-object"
            },
            "preferences": "not-an-array",
            "active": "not-a-boolean",
            "scores": [1, "two", 3],
            "tags": ["tag1", "", "tag3"]
          }
        ]
      },
      "metadata": {
        "createdAt": "123",
        "requestId": "a"
      }
    }
  },
  "journey": "DEFAULT",
  "system": "ALL"
}
EOF

echo -e "\nTesting with multiple validation failures..."
cargo run -- validate examples/basic/test_multiple_validations.json
echo -e "\nNote: This should now trigger all validation types (required, is_*, min/max_length, equals, regex)"