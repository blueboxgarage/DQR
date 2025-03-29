# DQR Examples

This directory contains example files and scripts demonstrating the different features of the DQR validation engine.

## Directory Structure

- `basic/`: Basic validation examples
  - `valid-request.json`: Example of a valid JSON payload
  - `invalid-request.json`: Example of an invalid JSON payload
  - `valid-name-length.json`: Example with valid name length validation
  - `invalid-name-length.json`: Example with invalid name length validation
  - `test_multiple_validations.json`: Example with multiple validation errors

- `multiple-key-fields/`: Examples for the multiple key fields feature
  - `multiple-key-fields-example.json`: Sample data with primary and alternate contact information
  - `test-multiple-key-fields.sh`: Script to demonstrate multiple key_fields validation

- `dependencies/`: Examples for conditional validation with depends_on
  - `depends-on-example.json`: Sample data with "employed" status
  - `depends-on-unemployed.json`: Sample data with "unemployed" status
  - `test-depends-on.sh`: Script to demonstrate conditional validation
  - `README-dependencies.md`: Detailed explanation of dependencies feature

- `conditionals/`: Examples for if/then/else conditional validation
  - `user-payment-example.json`: Sample data with user and payment information
  - `test-conditionals.sh`: Script to demonstrate if/then/else validation rules

- `journeys/`: Examples for journey-specific validation
  - `onboarding-complete.json`: Complete data for the onboarding journey
  - `onboarding-empty-first.json`: Data with empty first name for onboarding
  - `validation-check-complete.json`: Complete data for validation check journey
  - `validation-check-incomplete.json`: Incomplete data for validation check
  - `test-journey-validation.sh`: Script demonstrating journey-specific validation

## Running the Examples

### Basic Validation

```bash
# Validate a valid request
cargo run -- validate examples/basic/valid-request.json

# Validate an invalid request
cargo run -- validate examples/basic/invalid-request.json
```

### Multiple Key Fields

```bash
# Run the multiple key fields example
./examples/multiple-key-fields/test-multiple-key-fields.sh
```

This example demonstrates how a single rule can be applied to multiple fields by specifying multiple comma-separated key_fields in the rule definition.

### Conditional Validation (depends_on)

```bash
# Run the depends_on example
./examples/dependencies/test-depends-on.sh
```

This example demonstrates how rules can be conditionally applied based on the values of other fields in the data.

### If/Then/Else Conditional Validation

```bash
# Run the if/then/else conditionals example
./examples/conditionals/test-conditionals.sh
```

This example shows how to use if/then/else branching logic to create sophisticated validation rules that follow different paths based on the data values.

### Journey-Specific Validation

```bash
# Run the journey-specific validation example
./examples/journeys/test-journey-validation.sh
```

This example demonstrates how to create different validation rules for different journeys (ONBOARDING vs VALIDATION_CHECK) and how to conditionally validate fields only when certain conditions are met.