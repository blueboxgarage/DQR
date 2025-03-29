# DQR Validation Rules

This directory contains CSV files with validation rules for the DQR tool.

## Available Rule Files

- `default.csv`: Standard validation rules for data quality (used by default)

### Example Rules (in examples/ subdirectory)

- `examples/multiple-key-fields.csv`: Example rules demonstrating multiple key fields functionality
- `examples/dependencies.csv`: Example rules demonstrating conditional validation with depends_on
- `examples/conditionals.csv`: Example rules demonstrating if/then/else conditional validation
- `examples/advanced-key-fields.csv`: Example rules showing advanced key_fields patterns

## Rule Format

Each rule follows this CSV format:

```
id,selector,condition,key_fields,journey,system,depends_on_selector,depends_on_condition,logic_type,parent_rule_id
```

### Core Fields
- `id`: Unique identifier for the rule
- `selector`: JSONPath selector for the field(s) to validate
- `condition`: The validation condition (e.g., required, min_length:2, is_number)
- `key_fields`: Comma-separated field paths for indexing (can specify multiple fields)

### Context Fields
- `journey`: The journey this rule applies to (or "DEFAULT" for all)
- `system`: The system this rule applies to (or "ALL" for all)

### Dependency Fields
- `depends_on_selector`: JSONPath selector for dependency condition (optional)
- `depends_on_condition`: Condition that must be met for rule to apply (optional)

### Conditional Logic Fields
- `logic_type`: Type of conditional logic (if, then, else, or standard)
- `parent_rule_id`: For then/else rules, the ID of the parent "if" rule

## Using Rules Files

Specify the rules file to use when running validation:

```bash
# Method 1: Environment variable
export DQR_RULES_PATH=rules/dependencies.csv
cargo run -- validate examples/sample.json

# Method 2: Direct variable assignment
DQR_RULES_PATH=rules/multiple-key-fields.csv cargo run -- validate examples/sample.json
```

By default, the system will use `rules/default.csv` if no rules file is specified.