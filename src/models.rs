use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationRequest {
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub rule_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidationRule {
    pub id: String,
    pub selector: String,
    pub condition: String,
    pub key_fields: String,
    pub error_message: String,
}

impl ValidationResponse {
    pub fn success() -> Self {
        ValidationResponse {
            valid: true,
            errors: Vec::new(),
        }
    }

    pub fn failure(errors: Vec<ValidationError>) -> Self {
        ValidationResponse {
            valid: false,
            errors,
        }
    }
}