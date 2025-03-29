# Journey-Specific Validation Rules

This example demonstrates how to create different validation rules for different journeys, as well as conditional validation that only checks certain fields under specific circumstances.

## Key Concepts

### 1. Journey-Specific Validation

In many applications, validation requirements differ depending on which part of the user journey the data comes from:

- During **onboarding**, you might only validate required fields
- During **verification**, you might apply stricter validation rules
- During **review**, you might check for data consistency

The DQR system allows you to specify a `journey` value for each rule, enabling you to apply different validation rules to the same data based on its context.

### 2. Conditional Field Validation

Some fields should only be validated if they're present or if other fields meet certain conditions. This example demonstrates two approaches to conditional validation:

1. **Dependency-Based Validation**: Only validate a field if it depends on another field that meets a condition
2. **If/Then/Else Validation**: Use branching logic to apply different validations based on the data

## Example Journeys

This directory contains examples for two journeys:

### ONBOARDING Journey

During onboarding, we:
- Require names to be provided
- Only check name length if a name is actually provided (using `not_empty` dependency)
- This prevents overwhelming users with too many validation errors initially

### VALIDATION_CHECK Journey 

During validation checking, we:
- Check if the names section exists (if condition)
- If it does, we validate that all names are complete
- If not, we check for minimum name length and require emergency contact
- This demonstrates more complex business logic with if/then/else branching

## Running the Example

```bash
./examples/journeys/test-journey-validation.sh
```

This script demonstrates how the same data is validated differently depending on the journey context, and how conditional validation is applied.

## Implementation Details

The key implementation aspects are:

1. **not_empty Dependency Condition**:
   ```csv
   j002_first_name_length,$.application.individuals.data[*].names[*].name.first,min_length:2,application.individuals.names.name.first,ONBOARDING,ALL,$.application.individuals.data[*].names[*].name.first,not_empty,standard,
   ```
   This rule only checks name length if the depends_on_selector value is not empty.

2. **If/Then/Else Validation**:
   ```csv
   j005_name_validation_checker,$.application.individuals.data[*].names[*],required,application.individuals.names,VALIDATION_CHECK,ALL,,,if,
   j006_all_names_check,$.application.individuals.data[*].names[*].name,required,application.individuals.names.name,VALIDATION_CHECK,ALL,,,then,j005_name_validation_checker
   j007_all_names_length_check,$.application.individuals.data[*].names[*].name.first,min_length:3,application.individuals.names.name.first,VALIDATION_CHECK,ALL,,,else,j005_name_validation_checker
   ```
   This creates a branching validation flow based on whether names exist.