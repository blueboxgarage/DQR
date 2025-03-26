# DQR - JSON Validation API

DQR is a configurable JSON validation service that allows teams to update validation rules without code changes.

## Features

- Validate JSON payloads against configurable rules
- Rule-based validation using JSON Path selectors
- CSV configuration for easy rule management
- Detailed error responses with path and error message
- Efficient rule matching based on key fields
- Journey-based validation for different processing paths
- System-based filtering for multi-team usage

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
  },
  "journey": "DEFAULT",
  "system": "CUSTOMER"
}
```

The request includes two optional parameters:

- `journey`: The validation journey to use (e.g., "DEFAULT", "FAST_CHECK", "ALL_CHECKS", "PAYMENT_FLOW")
- `system`: The system identifier for rule filtering (e.g., "CUSTOMER", "INVENTORY", "CHECKOUT")

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
- `journey`: The validation journey where this rule applies (e.g., "DEFAULT", "ALL_CHECKS", "FAST_CHECK")
- `system`: The system this rule belongs to (e.g., "CUSTOMER", "INVENTORY", "CHECKOUT")

Example rules:

```csv
id,selector,condition,key_fields,error_message,journey,system
rule1,$.name,required,name,"Name field is required",DEFAULT,CUSTOMER
rule2,$.age,is_number,age,"Age must be a number",DEFAULT,CUSTOMER
rule3,$.email,min_length:5,email,"Email must be at least 5 characters long",ALL_CHECKS,CUSTOMER
rule4,$.items[*].quantity,is_number,items,"Item quantity must be a number",FAST_CHECK,INVENTORY
rule5,$.payment.type,required,payment,"Payment type is required",PAYMENT_FLOW,CHECKOUT
```

### Journey and System Filtering

- **Journey**: Controls which validation rules are applied based on the validation context:
  - `DEFAULT`: Standard validation rules for everyday usage
  - `FAST_CHECK`: Minimal validation for high-performance needs
  - `ALL_CHECKS`: Extensive validation for critical operations
  - Custom journeys: Define your own validation paths

- **System**: Allows multiple systems to use the same API with different validation rules:
  - Each system can define its own rules for the same fields
  - Rules with system="ALL" apply to all systems
  - Enables shared validation infrastructure with team-specific rules

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
- Support conditional validation rules (if/then/else)
- Improve error messages with more context
- Add journey-specific error handling and reporting
- Support multiple languages for error messages