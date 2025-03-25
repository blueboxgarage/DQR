#!/bin/bash

echo "Testing health endpoint..."
curl -s http://localhost:8081/health | jq

echo -e "\nTesting validation with valid data..."
curl -s -X POST -H "Content-Type: application/json" -d @examples/valid-request.json http://localhost:8081/api/validate | jq

echo -e "\nTesting validation with invalid data..."
curl -s -X POST -H "Content-Type: application/json" -d @examples/invalid-request.json http://localhost:8081/api/validate | jq