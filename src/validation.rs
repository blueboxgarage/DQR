use serde_json::Value;

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

    pub fn validate(
        &self, 
        json: &Value, 
        journey: &str, 
        system: &str
    ) -> Result<ValidationResponse, DqrError> {
        // Get all rules from repository
        let all_fields: Vec<String> = vec!["*".to_string()]; // Wildcard to get all rules
        let rules = self.rule_repository.get_rules_for_key_fields(&all_fields);
        
        // Apply validation rules
        let mut errors = Vec::new();
        
        for rule in rules {
            // Check if rule applies to this journey/system
            let journey_match = journey == "ALL_CHECKS" || 
                               rule.journey == journey || 
                               (journey == "DEFAULT" && rule.journey == "DEFAULT");
            
            let system_match = rule.system == "ALL" || rule.system == system;
            
            if !journey_match || !system_match {
                continue;
            }
            
            // Apply the rule
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
    
    fn apply_rule(&self, json: &Value, rule: &ValidationRule) -> Result<(), Vec<ValidationError>> {
        // Apply JSON path selector to find the values to validate
        let selection = jsonpath_lib::select(json, &rule.selector)
            .map_err(|e| vec![ValidationError {
                path: rule.selector.clone(),
                message: format!("Invalid JSON path: {}", e),
                rule_id: rule.id.clone(),
            }])?;
        
        // Handle required field check
        if rule.condition == "required" {
            // If no values match and condition is required, that's an error
            if selection.is_empty() {
                return Err(vec![ValidationError {
                    path: rule.selector.clone(),
                    message: rule.error_message.clone(),
                    rule_id: rule.id.clone(),
                }]);
            }
            
            // Check each selected value
            for (idx, value) in selection.iter().enumerate() {
                if value.is_null() || (value.is_string() && value.as_str().unwrap().is_empty()) {
                    return Err(vec![ValidationError {
                        path: format!("{} (item {})", rule.selector, idx),
                        message: rule.error_message.clone(),
                        rule_id: rule.id.clone(),
                    }]);
                }
            }
        }
        
        Ok(())
    }
}