# Understanding Multiple Key Fields in DQR

This example demonstrates how multiple key_fields work in the DQR validation system.

## What are Key Fields?

Key fields serve as an indexing mechanism for validation rules. They determine when a rule should be triggered during validation.

## Key Fields vs. Selectors

It's important to understand the distinction between key_fields and selectors:

- **Selector**: The JSON path that determines WHAT data the rule validates
- **Key Fields**: The fields that determine WHEN the rule is triggered
- **Condition**: The actual validation to perform on the selected data

## Multiple Key Fields in Action

When a rule has multiple key_fields (comma-separated), the rule is indexed under EACH of those fields in the rule repository.

```csv
id,selector,condition,key_fields
200_combined_email_format_check,$.application.individuals.data[*].contact.email,"regex:^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$","application.individuals.contact.email,application.individuals.contact.alternate.email"
```

In this example:

1. The rule has a selector that targets the primary email field: `$.application.individuals.data[*].contact.email`
2. It has key_fields for BOTH primary and alternate email fields: `application.individuals.contact.email,application.individuals.contact.alternate.email`
3. When the system loads rules, this single rule is added to the index under BOTH key fields
4. During validation, the rule is triggered when EITHER field exists in the data
5. However, the actual validation (regex pattern matching) is only applied to the primary email field because that's what the selector specifies

## Benefits of Multiple Key Fields

1. **Reduced Duplication**: Apply the same validation to multiple related fields without duplicating rules
2. **Category-Based Rules**: Create rules that apply to all fields of a certain category
3. **Simplified Maintenance**: Fewer rules to manage and update
4. **Consistent Validation**: Ensure all related fields are validated with the same logic

## Examples

This directory contains two examples demonstrating key_fields:

### 1. Basic Multiple Key Fields Example

This example demonstrates a rule that validates email format. It's configured with multiple key_fields so it's triggered for both primary and alternate email addresses, but only validates the primary email.

```bash
./examples/multiple-key-fields/test-multiple-key-fields.sh
```

### 2. Advanced Key Fields Usage

This example shows more unusual patterns where the key_fields and selectors are completely different, demonstrating how you can create complex validation relationships.

```bash
./examples/multiple-key-fields/test-advanced-key-fields.sh
```

In this advanced example:
- Rules are triggered by one field but validate a completely different field
- A single rule is indexed under multiple unrelated fields
- Validation dependencies create complex relationships between fields

## How It Works Internally

1. When rules are loaded:
   ```rust
   // Split comma-separated key fields
   let key_fields: Vec<String> = rule.key_fields
       .split(',')
       .map(|s| s.trim().to_string())
       .collect();

   // Insert rule into our map, grouped by each key field
   for field in &key_fields {
       self.rules
           .entry(field.clone())
           .or_default()
           .push(rule.clone());
   }
   ```

2. This creates a HashMap where:
   - Keys are the individual key_fields
   - Values are Vec<ValidationRule> containing all rules that apply to that field

3. During validation, the system can quickly look up all rules that apply to a given field