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

    // Get the relevant rules for a specific journey and system combination
    pub fn get_rules_for_journey_system(&self, journey: &str, system: &str) -> Vec<ValidationRule> {
        // Get all rules from repository
        let all_fields: Vec<String> = vec!["*".to_string()]; // Wildcard to get all rules
        let mut all_rules = self.rule_repository.get_rules_for_key_fields(&all_fields);
        
        // Filter rules based on journey and system
        all_rules.retain(|rule| {
            // Match by journey - if journey is ALL_CHECKS, include all rules
            let journey_match = journey == "ALL_CHECKS" || 
                                rule.journey == journey || 
                                (journey == "DEFAULT" && rule.journey == "DEFAULT");
            
            // Match by system - if rule's system is ALL, it applies to all systems
            let system_match = rule.system == "ALL" || rule.system == system;
            
            journey_match && system_match
        });
        
        all_rules
    }

    pub fn validate(
        &self, 
        json: &Value, 
        journey: &str, 
        system: &str
    ) -> Result<ValidationResponse, DqrError> {
        // Get applicable rules for this journey and system
        let rules = self.get_rules_for_journey_system(journey, system);
        
        // Apply validation rules
        let mut errors = Vec::new();
        
        for rule in rules {
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
            .map_err(|_| vec![ValidationError {
                path: rule.selector.clone(),
                rule_id: rule.id.clone(),
            }])?;
        
        // Handle min_length check for names when individuals.number=1 first
        if rule.condition == "min_length_when_single" {
            // First check if individuals.number=1
            if let Ok(numbers_selection) = jsonpath_lib::select(json, "$.application.individuals.number") {
                if let Some(numbers) = numbers_selection.first() {
                    if numbers.is_number() && numbers.as_i64() == Some(1) {
                        // Check each selected value's length
                        for (idx, value) in selection.iter().enumerate() {
                            if !value.is_string() || value.as_str().unwrap().len() <= 1 {
                                return Err(vec![ValidationError {
                                    path: format!("{} (item {})", rule.selector, idx),
                                    rule_id: rule.id.clone(),
                                }]);
                            }
                        }
                    }
                }
            }
        }
        // Handle required field check
        else if rule.condition == "required" {
            // If no values match and condition is required, that's an error
            if selection.is_empty() {
                return Err(vec![ValidationError {
                    path: rule.selector.clone(),
                    rule_id: rule.id.clone(),
                }]);
            }
            
            // Check each selected value
            for (idx, value) in selection.iter().enumerate() {
                if value.is_null() || (value.is_string() && value.as_str().unwrap().is_empty()) {
                    return Err(vec![ValidationError {
                        path: format!("{} (item {})", rule.selector, idx),
                        rule_id: rule.id.clone(),
                    }]);
                }
            }
        }
        
        Ok(())
    }
}