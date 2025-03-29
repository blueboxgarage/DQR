# Example Rule Files

This directory contains example rule files demonstrating different DQR validation features. These files are used by the example scripts in the `/examples` directory.

## Available Example Rule Files

- `multiple-key-fields.csv`: Demonstrates how a single rule can be applied to multiple related fields
- `dependencies.csv`: Shows validation rules that only apply when certain conditions are met
- `conditionals.csv`: Illustrates if/then/else branching logic for complex validation scenarios
- `advanced-key-fields.csv`: Demonstrates advanced key_fields usage where selectors and key_fields are different
- `journey-specific.csv`: Shows how to create different validation rules for different journeys and conditionally validate fields

## How to Use Example Rules

Each example rule file is paired with one or more example scripts in the `/examples` directory. To use an example rule file:

```bash
# Set the environment variable to point to the example rule file
export DQR_RULES_PATH=rules/examples/conditionals.csv

# Run the validation with sample data
cargo run -- validate examples/conditionals/user-payment-example.json
```

Or run the example scripts directly:

```bash
./examples/conditionals/test-conditionals.sh
```

## Creating Your Own Rule Files

You can use these examples as templates to create your own rule sets:

1. Copy an example rule file that's closest to your needs
2. Modify the rules to match your data structure and validation requirements
3. Set the `DQR_RULES_PATH` environment variable to point to your new rule file

For example:
```bash
cp rules/examples/conditionals.csv rules/my-custom-rules.csv
# Edit my-custom-rules.csv
DQR_RULES_PATH=rules/my-custom-rules.csv cargo run -- validate my-data.json
```