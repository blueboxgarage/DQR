# DQR API Examples

This directory contains examples for using the DQR API endpoints.

## API Endpoints Overview

DQR exposes the following REST API endpoints:

1. **Health Check**: `GET /health`
   - Returns the health status of the DQR service
   - Includes cache statistics

2. **Get All Rules**: `GET /api/rules`
   - Returns all validation rules in the system
   - Useful for debugging and monitoring

3. **Create Rule**: `POST /api/rules`
   - Creates a new validation rule
   - Supports various validation types: required, regex, length, min, max, enum, etc.

4. **Delete Rule**: `DELETE /api/rules/{id}`
   - Deletes a specific rule by ID

5. **Validate Data**: `POST /api/validate`
   - Validates submitted JSON data against rules
   - Filters rules by journey and system

## Example Script

The `api-examples.sh` script in the parent directory demonstrates how to use all these endpoints with various payloads and validation scenarios:

```bash
# Run the examples
./api-examples.sh
```

### Validation Types Demonstrated:

- **Required Field**: Checks if a field exists in the data
- **Regex Pattern**: Validates field contents against a regular expression
- **Length**: Validates string field length (min/max)
- **Min Value**: Validates a numeric field has a minimum value
- **Max Value**: Validates a numeric field doesn't exceed a maximum value
- **Enum**: Validates a field against a list of allowed values

### Request Payload Examples:

- **User Registration**: Validation of user data with email, password, age
- **Age Verification**: Minimum age validation
- **Payment Processing**: Validates payment amount and currency

## API Specification

### Validation Request

```json
{
  "data": {
    "your_data_here": "..."
  },
  "journey": "journey_name",
  "system": "system_name"
}
```

- `data`: The JSON data to validate
- `journey`: The journey context (defaults to "DEFAULT" if not provided)
- `system`: The system context (defaults to "ALL" if not provided)

### Rule Creation Request

```json
{
  "field_path": "$.path.to.field",
  "validation_type": "validation_type",
  "parameters": "parameters_string",
  "description": "Rule description",
  "journey": "journey_name",
  "system": "system_name"
}
```

- `field_path`: JSONPath to the field to validate
- `validation_type`: Type of validation
- `parameters`: Validation parameters (specific to validation type)
- `description`: Human-readable description
- `journey`: Journey context (defaults to "DEFAULT")
- `system`: System context (defaults to "ALL")