# DQR - JSON Validation API

DQR is a configurable JSON validation backend service that allows teams to update validation rules without code changes. It provides a RESTful API for validating data and managing validation rules.

## Features

- RESTful API for validation and rule management
- Validate JSON payloads against configurable rules
- Rule-based validation using JSON Path selectors
- CSV configuration for easy rule management
- Detailed validation responses with error paths
- Efficient rule matching based on key fields
- Journey-based validation for different processing paths
- System-based filtering for multi-team usage
- Performance-optimized with multi-level caching

## Project Structure

- `src/` - Rust source code
  - `api.rs` - HTTP API implementation
  - `error.rs` - Error handling
  - `lib.rs` - Library exports
  - `main.rs` - Application entry point
  - `models.rs` - Data structures
  - `rules.rs` - Rule loading and management
  - `validation.rs` - Validation engine
  - `validation_test.rs` - Test cases for validation engine
- `rules/` - Validation rule files in CSV format
  - `default.csv` - Default validation rules
  - `examples/` - Example rule configurations
    - `advanced-key-fields.csv` - Rules demonstrating advanced key field usage
    - `conditionals.csv` - Rules demonstrating conditional validation
    - `dependencies.csv` - Rules demonstrating dependency-based validation
    - `journey-specific.csv` - Rules for specific validation journeys
    - `multiple-key-fields.csv` - Rules demonstrating multiple key fields
- `examples/` - Example JSON payloads and demonstration scripts
  - `basic/` - Simple validation examples
  - `conditionals/` - If/Then/Else validation examples
  - `dependencies/` - Dependent validation examples
  - `journeys/` - Journey-specific validation examples
  - `multiple-key-fields/` - Multiple key field validation examples
- `test_cases/` - Test scripts for various validation scenarios

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
- `DQR_RULES_PATH`: Path to the CSV rules file (default: rules/default.csv)
- `RUST_LOG`: Log level (default: info)

#### Setting Environment Variables

You can use this script to set up your environment:

```bash
#!/bin/bash
# Save this as setup-dqr.sh and run it with: source setup-dqr.sh

# Default configuration
export DQR_HOST=127.0.0.1
export DQR_PORT=8081
export DQR_RULES_PATH=rules/default.csv
export RUST_LOG=info

# Uncomment and modify these lines to customize your configuration
# export DQR_HOST=0.0.0.0        # Listen on all interfaces
# export DQR_PORT=9000           # Use custom port
# export DQR_RULES_PATH=rules/examples/conditionals.csv  # Use a different rule set
# export RUST_LOG=debug          # More detailed logging

echo "DQR environment configured:"
echo "  Host:      $DQR_HOST"
echo "  Port:      $DQR_PORT"
echo "  Rules:     $DQR_RULES_PATH"
echo "  Log Level: $RUST_LOG"
```

Save this script, make it executable with `chmod +x setup-dqr.sh`, and run it with `source setup-dqr.sh` before starting the server.

### Running the Server

```bash
cargo run
```

Or with custom configuration:

```bash
DQR_HOST=0.0.0.0 DQR_PORT=9000 DQR_RULES_PATH=rules/dependencies.csv cargo run
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
   curl -X POST -H "Content-Type: application/json" -d @examples/basic/valid-request.json http://localhost:8081/api/validate
   ```

   Test validation with invalid data:
   ```bash
   curl -X POST -H "Content-Type: application/json" -d @examples/basic/invalid-request.json http://localhost:8081/api/validate
   ```

   Or use the included test scripts in the test_cases directory:
   ```bash
   chmod +x test_cases/test-batch.sh
   ./test_cases/test-batch.sh
   ```

## API Reference

DQR provides a RESTful API for validation operations and rule management. All API endpoints return JSON responses.

For more comprehensive API documentation with examples, see the [API Reference Documentation](examples/api/api-reference.md) and try the [API Usage Examples](examples/api/api-usage.sh).

### API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/validate` | Validate JSON data against configured rules |
| GET | `/api/rules` | Get all validation rules |
| POST | `/api/rules` | Create a new validation rule |
| DELETE | `/api/rules/{id}` | Delete a rule by ID |
| GET | `/health` | Check API health status and cache statistics |

### 1. Validate JSON

Validates a JSON payload against configured rules.

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

**Parameters:**
- `data` (required): The JSON data to validate
- `journey` (optional): The validation journey to use (default: "DEFAULT")
- `system` (optional): The system identifier for rule filtering (default: "ALL")

**Response:**
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

### 2. Get All Rules

Retrieves all configured validation rules.

```
GET /api/rules
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "rule1",
      "field_path": "$.name",
      "validation_type": "required",
      "parameters": null,
      "description": "Name field is required",
      "journey": "DEFAULT",
      "system": "ALL"
    },
    ...
  ],
  "error": null
}
```

### 3. Create Rule

Creates a new validation rule.

```
POST /api/rules
Content-Type: application/json

{
  "field_path": "$.name",
  "validation_type": "required",
  "parameters": null,
  "description": "Name field is required",
  "journey": "onboarding",
  "system": "registration"
}
```

**Parameters:**
- `field_path` (required): JSON path to the field (e.g., "$.name")
- `validation_type` (required): Type of validation (e.g., "required", "min_length")
- `parameters` (optional): Additional parameters for the validation
- `description` (optional): Human-readable description of the rule
- `journey` (optional): Validation journey (default: "DEFAULT")
- `system` (optional): System identifier (default: "ALL")

**Response:**
```json
{
  "success": true,
  "data": "generated-rule-id",
  "error": null
}
```

### 4. Delete Rule

Deletes a validation rule by ID.

```
DELETE /api/rules/{id}
```

**Parameters:**
- `id` (required): The unique identifier of the rule to delete

**Response:**
```json
{
  "success": true,
  "data": null,
  "error": null
}
```

### 5. Health Check

Checks the API's health status and returns cache statistics.

```
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "cache_stats": {
    "validation_cache_size": 12,
    "journey_system_cache_size": 5
  }
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
- `min_value:N` - Number must be greater than or equal to N
- `max_value:N` - Number must be less than or equal to N
- `equals:VALUE` - Field must equal the specified value (string, number, boolean)
- `regex:PATTERN` - String must match the regular expression

### Validation Approaches

DQR supports three different approaches to validation, each with different levels of complexity:

1. **Standard Rules**: Basic field validation without dependencies
   ```csv
   id,selector,condition,key_fields
   email_format,$.user.email,"regex:^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",user.email
   ```

2. **Dependent Rules**: Rules that only apply when another field meets a condition
   ```csv
   id,selector,condition,key_fields,depends_on_selector,depends_on_condition
   shipping_address,$.order.shipping_address,required,order.shipping_address,$.order.delivery_method,equals:physical
   ```

3. **Conditional Rules (If/Then/Else)**: Branching logic for complex validation
   ```csv
   id,selector,condition,key_fields,logic_type,parent_rule_id
   payment_type_check,$.payment.type,equals:credit_card,payment.type,if,
   credit_card_rules,$.payment.credit_card.number,required,payment.credit_card.number,then,payment_type_check
   bank_account_rules,$.payment.bank_account.routing,required,payment.bank_account.routing,else,payment_type_check
   ```

Choose the approach that best matches your validation needs:
- Use **standard rules** for simple, independent validations
- Use **dependent rules** when one field depends on another
- Use **conditional rules** for complex branching logic


## Understanding Key Fields

Key fields in the DQR system serve as an indexing and lookup mechanism for validation rules:

1. **Purpose**: 
   - They determine when a rule should be triggered in the validation process
   - They create an efficient index for quickly finding relevant rules
   - They enable a single rule to apply to multiple related fields

2. **How They Work**:
   - When rules are loaded, they're indexed by each of their key_fields
   - During validation, the system looks up all rules that match the fields being validated
   - A rule is applied when ANY of its key_fields matches data in the document

3. **Relation to Selectors**:
   - The `selector` determines WHAT the rule validates (the actual JSON path)
   - The `key_fields` determine WHEN the rule is triggered (the indexing)
   - These can be different, allowing flexible validation patterns

4. **Practical Examples**:

   **Example 1: Single Key Field**
   ```csv
   id,selector,condition,key_fields
   email_format,$.user.email,"regex:^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",user.email
   ```
   This rule validates the email format and is only triggered when processing the user.email field.

   **Example 2: Multiple Key Fields**
   ```csv
   id,selector,condition,key_fields
   email_format,$.user.primaryEmail,"regex:^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$","user.primaryEmail,user.secondaryEmail,user.workEmail"
   ```
   This rule is triggered when ANY of the three email fields is present, but only validates the primaryEmail (based on selector).

   **Example 3: Different Selector and Key Fields**
   ```csv
   id,selector,condition,key_fields
   contact_required,$.user.contactInfo,required,"user.type,user.role"
   ```
   This rule checks if contactInfo exists but is triggered when processing either user.type or user.role.

5. **Multiple Key Fields Use Case**:
   - Validate related fields with the same rule (all email fields use same validation)
   - Create category-based rules (all address fields trigger address validation rules)
   - Optimize rule organization (fewer rules to manage)

6. **Performance Benefit**:
   Instead of checking every rule against every field, the system can quickly retrieve only the rules that are relevant to the fields present in your data, making validation more efficient.

## Rule Nesting and Dependencies

DQR already supports rule nesting and dependencies through the following features:

1. **Rule Dependencies**:
   - Rules can depend on other fields using `depends_on_selector` and `depends_on_condition`
   - A rule is only evaluated if its dependency condition is met
   - Supported dependency checks include equality and not_empty

2. **Conditional Logic (If/Then/Else)**:
   - Rules can be organized in conditional branches using `logic_type` and `parent_rule_id`
   - The `if` branch determines which set of rules to run (`then` or `else`)
   - Conditional rules can be nested to create complex logic trees

3. **Rule Evaluation Flow**:
   - The validation engine first checks if a rule should be applied based on dependencies
   - For conditional rules, only the relevant branch is executed based on the parent rule result
   - Error messages include the rule ID and the path that failed validation

### Future Enhancements for Rule Dependencies

To further improve rule nesting and dependencies, we could add:

1. **Enhanced Dependency Types**:
   - Support for more complex dependency conditions beyond equality and not_empty
   - Multiple dependencies per rule (AND/OR relationships between dependencies)
   - Support for numeric comparisons, pattern matching, and custom functions

2. **Advanced Conditional Logic**:
   - Support for more complex boolean expressions (AND, OR, NOT)
   - Allow conditions to reference results of other rules
   - Add support for switch/case style multi-branch conditionals

3. **Performance Optimizations**:
   - Implement a dependency graph for more efficient rule evaluation order
   - Add caching of intermediate validation results
   - Add cycle detection to prevent circular dependencies

## Examples

The project includes organized examples demonstrating different features:

- **Basic Validation**: Simple validation examples
  ```bash
  cargo run -- validate examples/basic/valid-request.json
  cargo run -- validate examples/basic/invalid-request.json
  ```

- **Multiple Key Fields**: How a single rule can validate multiple related fields
  ```bash
  ./examples/multiple-key-fields/test-multiple-key-fields.sh
  ```

- **Dependent Validation**: Rules that only apply when certain conditions are met
  ```bash
  ./examples/dependencies/test-depends-on.sh
  ```

- **If/Then/Else Validation**: Branching validation logic based on data values
  ```bash
  ./examples/conditionals/test-conditionals.sh
  ```

For more details, see the [Examples README](examples/README.md).

## Caching Implementation

DQR includes a multi-level caching system to maximize performance:

1. **Rule Repository Caching**:
   - Rules filtered by journey and system are cached to avoid re-filtering
   - Caches are automatically cleared when rules are updated
   - Significantly improves performance for repeated journey/system combinations

2. **Validation Result Caching**:
   - Results of validation operations are cached using content hashing
   - Identical validation requests return cached results immediately
   - Provides dramatic performance improvements for repeated validations

3. **Cache Management**:
   - Cache statistics are available via the health check endpoint
   - `GET /health` returns current cache sizes and health information
   - Caches are cleared automatically when rule definitions change

The caching system is designed to be thread-safe and requires no explicit configuration.

## Future Enhancements

- Add more validation conditions (enum values, array validations, custom validators)
- Support for date/time validation (date format, before/after comparisons)
- Support multiple rule sources (database, API)
- Add admin interface for rule management
- Improve error messages with more context
- Support for custom error messages and localization
- Add cache size limits and time-based expiration
