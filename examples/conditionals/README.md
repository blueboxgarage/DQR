# Conditional Validation with If/Then/Else

This example demonstrates how to use conditional if/then/else validation rules in the DQR system to create branching validation logic.

## How If/Then/Else Validation Works

The if/then/else validation pattern allows you to:

1. Check a condition using an "if" rule
2. Apply one set of rules if the condition passes (the "then" branch)
3. Apply a different set of rules if the condition fails (the "else" branch)

This creates flexible, data-driven validation that can adapt to different scenarios.

## Rule Structure

Conditional rules use three new fields in the rule definition:

1. `logic_type`: Specifies whether this is an "if", "then", or "else" rule
2. `parent_rule_id`: For "then" and "else" rules, identifies which "if" rule they belong to 
3. The standard fields (selector, condition, etc.) still apply to each rule

### Example CSV Structure

```csv
id,selector,condition,key_fields,logic_type,parent_rule_id
payment_type_check,$.payment.type,equals:credit_card,payment.type,if,
credit_card_rules,$.payment.credit_card.number,required,payment.credit_card.number,then,payment_type_check
bank_account_rules,$.payment.bank_account.routing,required,payment.bank_account.routing,else,payment_type_check
```

## Use Cases

Conditional validation is useful for:

1. **Different Required Fields**: When different fields are required based on a selection
   - Example: Credit card fields required for credit card payments, bank account for bank transfers

2. **Age-Based Requirements**: Different requirements for adults vs. minors
   - Example: Adults need ID verification, minors need guardian information

3. **Form Validation**: Different validation rules based on selected options
   - Example: International addresses need different format validation than domestic ones

4. **Complex Business Rules**: Implement sophisticated validation logic
   - Example: Different product types trigger different validation requirements

## Key Features

1. **Nested Conditions**: Conditional rules can be nested for complex logic trees
2. **Different Selectors**: Each branch can validate completely different parts of the data
3. **Efficient Processing**: Only the relevant branch is processed, improving performance
4. **Clear Organization**: Logic is represented clearly in rule definitions

## Example Scenarios

This directory includes examples demonstrating:

1. **Age Verification**:
   - If user is a minor (age < 18), guardian information is required
   - If user is an adult, guardian information is not required

2. **Payment Method Validation**:
   - If payment type is "credit_card", validate credit card fields
   - If payment type is "bank_account", validate bank account fields

## Running the Example

```bash
./examples/conditionals/test-conditionals.sh
```

## How It Works Internally

1. The system first processes all "if" rules
2. For each "if" rule:
   - The condition is evaluated
   - Based on the result, either the "then" or "else" branch is processed
   - Any nested conditional rules in those branches are processed recursively

This enables powerful, context-aware validation that adapts to the data being validated.