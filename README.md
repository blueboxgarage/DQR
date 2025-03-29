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
- `rules/` - Validation rule files in CSV format
  - `default.csv` - Default validation rules
  - `multiple-key-fields.csv` - Rules demonstrating multiple key fields
  - `dependencies.csv` - Rules demonstrating conditional validation
- `examples/` - Example JSON payloads and demonstration scripts

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
- `depends_on_selector`: Optional JSON Path selector for a field this rule depends on
- `depends_on_condition`: Optional condition that must be met by the dependency field for this rule to apply

Example rules:

```csv
id,selector,condition,key_fields,error_message,journey,system,depends_on_selector,depends_on_condition
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

## Future Enhancements

- Add more validation conditions (numeric ranges, enum values, custom validators)
- Add caching for improved performance
- Support multiple rule sources (database, API)
- Add admin interface for rule management
- Improve error messages with more context
- Add journey-specific error handling and reporting
- Support multiple languages for error messages
- Add custom user-defined functions for validation
- Support cross-field validation (comparing multiple fields)
- Implement validation result caching for faster repeated validations
- Add rule versioning and rule deployment pipelines