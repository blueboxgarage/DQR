use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationRequest {
    pub data: serde_json::Value,
    #[serde(default = "default_journey")]
    pub journey: String,
    #[serde(default = "default_system")]
    pub system: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub path: String,
    pub rule_id: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum ConditionalLogic {
    #[serde(rename = "if")]
    If,
    #[serde(rename = "then")]
    Then,
    #[serde(rename = "else")]
    Else,
    #[serde(rename = "standard")]
    Standard,
}

impl Default for ConditionalLogic {
    fn default() -> Self {
        ConditionalLogic::Standard
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidationRule {
    pub id: String,
    pub selector: String,
    pub condition: String,
    pub key_fields: String,
    #[serde(default = "default_journey")]
    pub journey: String,
    #[serde(default = "default_system")]
    pub system: String,
    #[serde(default = "String::new")]
    pub depends_on_selector: String,
    #[serde(default = "String::new")]
    pub depends_on_condition: String,
    #[serde(default = "default_empty_map")]
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub logic_type: ConditionalLogic,
    #[serde(default = "String::new")]
    pub parent_rule_id: String,
}

impl ValidationRule {
    pub fn is_conditional_branch(&self) -> bool {
        matches!(self.logic_type, ConditionalLogic::Then | ConditionalLogic::Else)
    }
    
    pub fn is_condition_root(&self) -> bool {
        matches!(self.logic_type, ConditionalLogic::If)
    }
}

fn default_journey() -> String {
    "DEFAULT".to_string()
}

fn default_system() -> String {
    "ALL".to_string()
}

fn default_empty_map() -> std::collections::HashMap<String, serde_json::Value> {
    std::collections::HashMap::new()
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