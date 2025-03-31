# DQR API Reference

This document provides example request/response pairs for all DQR API endpoints.

## Health Check API

**Endpoint:** `GET /health`

**Response:**
```json
{
  "status": "healthy",
  "cache_stats": {
    "validation_cache_size": 24,
    "journey_system_cache_size": 6
  }
}
```

## Get All Rules API

**Endpoint:** `GET /api/rules`

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "rule123",
      "field_path": "$.user.email",
      "validation_type": "required",
      "parameters": null,
      "description": "Email is required",
      "journey": "registration",
      "system": "user-portal"
    },
    {
      "id": "rule456",
      "field_path": "$.user.email",
      "validation_type": "regex",
      "parameters": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
      "description": "Email must be in valid format",
      "journey": "registration",
      "system": "user-portal"
    }
  ],
  "error": null
}
```

## Create Rule API

**Endpoint:** `POST /api/rules`

### Required Field Validation

**Request:**
```json
{
  "field_path": "$.user.email",
  "validation_type": "required",
  "description": "User email is required",
  "journey": "registration",
  "system": "user-portal"
}
```

**Response:**
```json
{
  "success": true,
  "data": "rule123",
  "error": null
}
```

### Regex Pattern Validation

**Request:**
```json
{
  "field_path": "$.user.email",
  "validation_type": "regex",
  "parameters": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
  "description": "User email must be in valid format",
  "journey": "registration",
  "system": "user-portal"
}
```

**Response:**
```json
{
  "success": true,
  "data": "rule456",
  "error": null
}
```

### Length Validation

**Request:**
```json
{
  "field_path": "$.user.password",
  "validation_type": "length",
  "parameters": "min=8,max=64",
  "description": "Password must be between 8 and 64 characters",
  "journey": "registration",
  "system": "user-portal"
}
```

**Response:**
```json
{
  "success": true,
  "data": "rule789",
  "error": null
}
```

### Min Value Validation

**Request:**
```json
{
  "field_path": "$.user.age",
  "validation_type": "min",
  "parameters": "18",
  "description": "User must be at least 18 years old",
  "journey": "age_verification",
  "system": "ALL"
}
```

**Response:**
```json
{
  "success": true,
  "data": "rule101112",
  "error": null
}
```

### Max Value Validation

**Request:**
```json
{
  "field_path": "$.payment.amount",
  "validation_type": "max",
  "parameters": "10000",
  "description": "Payment amount cannot exceed 10000",
  "journey": "payment_processing",
  "system": "checkout"
}
```

**Response:**
```json
{
  "success": true,
  "data": "rule131415",
  "error": null
}
```

### Enum Validation

**Request:**
```json
{
  "field_path": "$.payment.currency",
  "validation_type": "enum",
  "parameters": "USD,EUR,GBP,JPY,CAD",
  "description": "Currency must be one of the supported currencies",
  "journey": "payment_processing",
  "system": "checkout"
}
```

**Response:**
```json
{
  "success": true,
  "data": "rule161718",
  "error": null
}
```

## Validation API

**Endpoint:** `POST /api/validate`

### Valid Request

**Request:**
```json
{
  "data": {
    "user": {
      "email": "john.doe@example.com",
      "password": "securePassword123",
      "age": 30
    }
  },
  "journey": "registration",
  "system": "user-portal"
}
```

**Response:**
```json
{
  "valid": true,
  "errors": []
}
```

### Invalid Request

**Request:**
```json
{
  "data": {
    "user": {
      "email": "not-an-email",
      "password": "short",
      "age": 16
    }
  },
  "journey": "registration",
  "system": "user-portal"
}
```

**Response:**
```json
{
  "valid": false,
  "errors": [
    {
      "path": "$.user.email",
      "rule_id": "rule456"
    },
    {
      "path": "$.user.password",
      "rule_id": "rule789"
    }
  ]
}
```

## Delete Rule API

**Endpoint:** `DELETE /api/rules/{id}`

**Response:**
```json
{
  "success": true,
  "data": null,
  "error": null
}
```

## Error Responses

### Invalid Rule Creation

**Response:**
```json
{
  "success": false,
  "data": null,
  "error": "Failed to create rule: Invalid validation type"
}
```

### Rule Not Found

**Response:**
```json
{
  "success": false,
  "data": null,
  "error": "Failed to delete rule: Rule not found"
}
```

### Internal Server Error

**Response:**
```json
{
  "success": false,
  "data": null,
  "error": "Internal server error"
}
```