# DQR Conditional Validation with depends_on

This example demonstrates how to use the `depends_on` feature of the DQR validation engine to create conditional validation rules.

## How depends_on Works

The `depends_on` feature allows you to specify a dependency between rules, where a rule is only applied when a specific condition is met. This is useful for validating fields that are only required in certain contexts.

Each rule can have two dependency-related fields:
1. `depends_on_selector`: A JSON path that selects the field(s) to check
2. `depends_on_condition`: The condition that must be met for the rule to be applied

The validation engine evaluates the dependency condition first, and only applies the rule if the condition is met.

## Example

In this example, we have rules for validating employment information:

1. Rule 300: Requires employment status (always runs)
2. Rule 301: Requires employer name (only if status is "employed")
3. Rule 302: Requires employer address (only if status is "employed")
4. Rule 303: Requires income (only if status is "employed")
5. Rule 304: Validates income is a number (only if status is "employed")

Rules 301-304 all have:
- `depends_on_selector`: `$.application.individuals.data[*].employment.status`
- `depends_on_condition`: `equals:employed`

This means these rules are only applied when the employment status is "employed".

## Test Cases

The example includes two test cases:

1. `depends-on-example.json`: Status = "employed" but employer fields are empty
   - Result: Validation fails because employer name and address are required

2. `depends-on-unemployed.json`: Status = "unemployed" and employer fields are empty
   - Result: Validation passes because the dependency condition is not met, so employer fields are not required

## Running the Example

```bash
./examples/test-depends-on.sh
```

This script will run the validation engine with both test cases and show the results.

## Implementation Details

The dependency check is implemented in the `apply_rule` method of the `ValidationEngine` class. Before applying a rule, it checks if there's a dependency condition. If there is, it evaluates the condition and only proceeds if it's met.

This feature allows for complex validation scenarios where the validity of data depends on the values of other fields.