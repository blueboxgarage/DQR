# DQR - JSON Validation API

DQR is a configurable JSON validation service that allows teams to update validation rules without code changes.

## Features

- Validate JSON payloads against configurable rules
- Rule-based validation using JSON Path selectors
- CSV configuration for easy rule management
- Detailed error responses with path and error message
- Efficient rule matching based on key fields

## Project Structure

- `src/` - Rust source code
  - `api.rs` - HTTP API implementation
  - `error.rs` - Error handling
  - `lib.rs` - Library exports
  - `main.rs` - Application entry point
  - `models.rs` - Data structures
  - `rules.rs` - Rule loading and management
  - `validation.rs` - Validation engine
  - `validation_test.rs` - Unit tests
- `rules.csv` - Sample validation rules
- `examples/` - Example JSON payloads for testing
- `.env` - Environment configuration

## Getting Started

### Prerequisites

- Rust toolchain (1.60+)
- Cargo

### Installation

```bash
git clone https://github.com/yourusername/dqr.git
cd dqr
cargo build
```

### Configuration

DQR uses environment variables for configuration, which can be set in the `.env` file:

- `DQR_HOST`: Host to bind the server to (default: 127.0.0.1)
- `DQR_PORT`: Port to listen on (default: 8081)
- `DQR_RULES_PATH`: Path to the CSV rules file (default: rules.csv)
- `RUST_LOG`: Log level (default: info)

### Running the Server

```bash
cargo run
```

Or with custom configuration:

```bash
DQR_HOST=0.0.0.0 DQR_PORT=9000 DQR_RULES_PATH=/path/to/custom-rules.csv cargo run
```

## Testing

### Running Unit Tests

```bash
cargo test
```

### Testing the API

1. Start the server:
   ```bash
   cargo run
   ```

2. In another terminal, test the API endpoints:

   Test the health endpoint:
   ```bash
   curl http://localhost:8081/health
   ```

   Test validation with valid data:
   ```bash
   curl -X POST -H "Content-Type: application/json" -d @examples/valid-request.json http://localhost:8081/api/validate
   ```

   Test validation with invalid data:
   ```bash
   curl -X POST -H "Content-Type: application/json" -d @examples/invalid-request.json http://localhost:8081/api/validate
   ```

   Or use the included test script:
   ```bash
   chmod +x test-api.sh
   ./test-api.sh
   ```

## API Usage

### Validate JSON

```
POST /api/validate
Content-Type: application/json

{
  "data": {
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com",
    "address": {
      "city": "New York",
      "zipcode": "10001"
    },
    "items": [
      {"id": "item1", "quantity": 5},
      {"id": "item2", "quantity": 10}
    ],
    "metadata": {
      "tags": ["personal", "customer"]
    }
  }
}
```

### Response Format

```json
{
  "valid": true|false,
  "errors": [
    {
      "path": "$.field.path",
      "message": "Error message",
      "rule_id": "rule_id"
    }
  ]
}
```

## Rule Configuration

Rules are defined in a CSV file with the following columns:

- `id`: Unique rule identifier
- `selector`: JSON Path selector for the field to validate (e.g., `$.name`, `$.items[*].id`)
- `condition`: Validation condition (e.g., `required`, `is_number`, `min_length:5`)
- `key_fields`: Fields that trigger this rule (comma-separated)
- `error_message`: Human-readable error message

Example rules:

```csv
id,selector,condition,key_fields,error_message
rule1,$.name,required,name,"Name field is required"
rule2,$.age,is_number,age,"Age must be a number"
rule3,$.email,min_length:5,email,"Email must be at least 5 characters long"
```

### Supported Validation Conditions

- `required` - Field must exist and not be null
- `is_number` - Field must be a number
- `is_string` - Field must be a string
- `is_boolean` - Field must be a boolean
- `is_array` - Field must be an array
- `is_object` - Field must be an object
- `min_length:N` - String must have at least N characters
- `max_length:N` - String must have at most N characters
- `regex:PATTERN` - String must match the regular expression (placeholder implementation)

## Health Check

```
GET /health
```

Response:

```json
{
  "status": "healthy"
}
```

## Future Enhancements

- Add more validation conditions (regex, numeric ranges, enum values)
- Support rule nesting and dependencies
- Add caching for improved performance
- Support multiple rule sources (database, API)
- Add admin interface for rule management