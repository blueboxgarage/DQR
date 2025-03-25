use serde_json::Value;
use std::collections::HashSet;

use crate::error::DqrError;
use crate::models::{ValidationError, ValidationRule, ValidationResponse};
use crate::rules::RuleRepository;

#[derive(Clone)]
pub struct ValidationEngine {
    rule_repository: RuleRepository,
}

impl ValidationEngine {
    pub fn new(rule_repository: RuleRepository) -> Self {
        ValidationEngine { rule_repository }
    }

    pub fn validate(&self, json: &Value) -> Result<ValidationResponse, DqrError> {
        // Extract key fields from the JSON
        let key_fields = self.extract_key_fields(json)?;
        
        // Get applicable rules based on key fields
        let rules = self.rule_repository.get_rules_for_key_fields(&key_fields);
        
        // Apply validation rules
        let mut errors = Vec::new();
        
        for rule in rules {
            if let Err(validation_errors) = self.apply_rule(json, &rule) {
                errors.extend(validation_errors);
            }
        }
        
        if errors.is_empty() {
            Ok(ValidationResponse::success())
        } else {
            Ok(ValidationResponse::failure(errors))
        }
    }
    
    pub fn extract_key_fields(&self, json: &Value) -> Result<Vec<String>, DqrError> {
        let mut fields = HashSet::new();
        
        if let Value::Object(obj) = json {
            for key in obj.keys() {
                fields.insert(key.clone());
                
                // Also add keys from nested objects
                if let Some(Value::Object(nested)) = obj.get(key) {
                    for nested_key in nested.keys() {
                        fields.insert(format!("{}.{}", key, nested_key));
                    }
                }
            }
        }
        
        Ok(fields.into_iter().collect())
    }
    
    fn apply_rule(&self, json: &Value, rule: &ValidationRule) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // Apply JSON path selector to find the values to validate
        let selection = jsonpath_lib::select(json, &rule.selector)
            .map_err(|e| vec![ValidationError {
                path: rule.selector.clone(),
                message: format!("Invalid JSON path: {}", e),
                rule_id: rule.id.clone(),
            }])?;
            
        // If no values match the selector, that's not an error (might be optional field)
        if selection.is_empty() {
            return Ok(());
        }
        
        // For each selected value, apply the condition
        for (idx, selected) in selection.iter().enumerate() {
            let result = self.evaluate_condition(selected, &rule.condition);
            
            if !result {
                errors.push(ValidationError {
                    path: format!("{} (item {})", rule.selector, idx),
                    message: rule.error_message.clone(),
                    rule_id: rule.id.clone(),
                });
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn evaluate_condition(&self, value: &Value, condition: &str) -> bool {
        // For now, we'll implement some basic conditions
        match condition {
            "required" => !value.is_null(),
            "is_number" => value.is_number(),
            "is_string" => value.is_string(),
            "is_boolean" => value.is_boolean(),
            "is_array" => value.is_array(),
            "is_object" => value.is_object(),
            _ if condition.starts_with("min_length:") => {
                if let Some(min_len) = condition.strip_prefix("min_length:") {
                    if let Ok(min_len) = min_len.trim().parse::<usize>() {
                        if let Some(s) = value.as_str() {
                            return s.len() >= min_len;
                        }
                    }
                }
                false
            },
            _ if condition.starts_with("max_length:") => {
                if let Some(max_len) = condition.strip_prefix("max_length:") {
                    if let Ok(max_len) = max_len.trim().parse::<usize>() {
                        if let Some(s) = value.as_str() {
                            return s.len() <= max_len;
                        }
                    }
                }
                false
            },
            _ if condition.starts_with("regex:") => {
                // For a full implementation, you'd use the regex crate here
                // For simplicity, we're just checking if it's a string for now
                value.is_string()
            },
            // Add more conditions as needed
            _ => true, // Unknown condition, assume valid
        }
    }
}