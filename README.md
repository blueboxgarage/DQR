# DQR - JSON Validation API

DQR is a configurable JSON validation service that allows teams to update validation rules without code changes.

## Features

- Validate JSON payloads against configurable rules
- Rule-based validation using JSON Path selectors
- CSV configuration for easy rule management
- Detailed  responses
- Efficient rule matching based on key fields
- Journey-based validation for different processing paths
- System-based filtering for multi-team usage

## Project Structure

- `src/` - Rust source code
  - `api.rs` - HTTP API implementation
  - `.rs` -  handling
  - `lib.rs` - Library exports
  - `main.rs` - Application entry point
  - `models.rs` - Data structures
  - `rules.rs` - Rule loading and management
  - `validation.rs` - Validation engine
- `rules.csv` - Sample validation rules
- `examples/` - Example JSON payloads for testing

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

DQR uses environment variables for configuration:

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

   Or use the included test scripts in the test_cases directory:
   ```bash
   chmod +x test_cases/test-batch.sh
   ./test_cases/test-batch.sh
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
- `depends_on_selector`: Optional JSON Path selector for a field this rule depends on
- `depends_on_condition`: Optional condition that must be met by the dependency field for this rule to apply

Example rules:

```csv
id,selector,condition,key_fields,journey,system,depends_on_selector,depends_on_condition
rule1,$.name,required,name,"Name field is required",DEFAULT,CUSTOMER,,
rule2,$.age,is_number,age,"Age must be a number",DEFAULT,CUSTOMER,,
01_applicant_name_length,$.applicants.names.first,min_length:3,applicants,"First name must be at least 3 characters long",DEFAULT_TEST,ACQ_TEST,$.applicants.number,equals:1
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
- `equals:VALUE` - Field must equal the specified value (string, number, boolean)
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

## Understanding Key Fields

Key fields in the DQR system serve as an efficient indexing mechanism for validation rules:

1. **Purpose**: 
   - They identify which fields in your JSON data a particular rule applies to
   - They create a mapping that allows the system to quickly look up relevant rules when validating specific parts of your data

2. **Implementation**:
   - In the rule definition, key_fields are comma-separated strings (e.g., `application.individuals.names.name`)
   - The system splits these into individual fields and creates a HashMap where each field points to applicable rules
   - This creates an efficient index for rule retrieval

3. **Example**:
   If you have a rule with `key_fields: "name,email"`, it means this rule applies when validating either the name or email fields in your JSON data.

4. **Performance Benefit**:
   Instead of checking every rule against every field, the system can quickly retrieve only the rules that are relevant to the fields present in your data structure, making validation more efficient.

## Requirements for Rule Nesting and Dependencies

To support rule nesting and dependencies, the following changes would be needed:

1. **Enhanced Rule Structure**:
   - Expand the ValidationRule struct to support multiple dependencies
   - Add support for complex dependency types beyond simple equality checks
   - Implement a logical structure for nested rules (parent-child relationships)

2. **Dependency Resolution Engine**:
   - Create a general dependency resolution mechanism
   - Add logic to determine rule evaluation order based on dependencies
   - Implement cycle detection to prevent circular dependencies

3. **Rule Condition Evaluation**:
   - Develop a more robust condition evaluation system
   - Support boolean logic (AND, OR, NOT) for complex conditions
   - Allow conditions to reference other rule results

4. **Data Structure Changes**:
   - Create a hierarchical rule structure for nesting
   - Implement a dependency graph to track relationships
   - Store evaluation results for dependency checking

5. **Validation Process Updates**:
   - Modify the validation flow to handle rule dependencies
   - Implement conditional rule execution based on parent rule results
   - Add proper error propagation for dependency failures

## Future Enhancements

- Add more validation conditions (regex, numeric ranges, enum values)
- Add caching for improved performance
- Support multiple rule sources (database, API)
- Add admin interface for rule management
- Support conditional validation rules (if/then/else)
- Improve error messages with more context
- Add journey-specific  handling and reporting
- Support multiple languages for  messages
