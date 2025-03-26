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
        // Extract key fields from the JSON
        let key_fields = self.extract_key_fields(json)?;
        
        // Get applicable rules based on key fields
        let mut rules = self.rule_repository.get_rules_for_key_fields(&key_fields);
        
        // Filter rules based on journey and system
        rules.retain(|rule| {
            // Match by journey - if journey is ALL_CHECKS, include all rules
            let journey_match = journey == "ALL_CHECKS" || 
                                rule.journey == journey || 
                                (journey == "DEFAULT" && rule.journey == "DEFAULT");
            
            // Match by system - if rule's system is ALL, it applies to all systems
            let system_match = rule.system == "ALL" || rule.system == system;
            
            journey_match && system_match
        });
        
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
        
        log::debug!("Applying rule {}: {} on {}", rule.id, rule.condition, rule.selector);
        
        // Check dependency condition if it exists
        if !rule.depends_on_selector.is_empty() && !rule.depends_on_condition.is_empty() {
            log::debug!("Rule has dependency: {} {}", rule.depends_on_selector, rule.depends_on_condition);
            
            // Get the dependency value
            let dep_selection = jsonpath_lib::select(json, &rule.depends_on_selector)
                .map_err(|e| vec![ValidationError {
                    path: rule.depends_on_selector.clone(),
                    message: format!("Invalid dependency JSON path: {}", e),
                    rule_id: rule.id.clone(),
                }])?;
                
            // If dependency selector doesn't match anything, skip this rule
            if dep_selection.is_empty() {
                log::debug!("Dependency not found, skipping rule");
                return Ok(());
            }
            
            // Check if the dependency condition is met
            let dep_result = self.evaluate_condition(&dep_selection[0], &rule.depends_on_condition);
            log::debug!("Dependency condition result: {}", dep_result);
            
            // If dependency condition is not met, skip this rule
            if !dep_result {
                log::debug!("Dependency condition not met, skipping rule");
                return Ok(());
            }
            
            log::debug!("Dependency condition met, continuing with rule");
        }
        
        // For required fields, we need special handling for nested paths
        if rule.condition == "required" {
            // If selector is like $.payment.type, we need to handle missing paths
            
            // Check if the parent path exists
            if rule.selector.contains(".") {
                let parent_path = rule.selector.rsplitn(2, '.').collect::<Vec<&str>>()[1].to_string();
                log::debug!("Checking parent path: {}", parent_path);
                
                let parent_selection = jsonpath_lib::select(json, &parent_path)
                    .map_err(|e| vec![ValidationError {
                        path: parent_path.clone(),
                        message: format!("Invalid JSON path: {}", e),
                        rule_id: rule.id.clone(),
                    }])?;
                
                // If parent exists but doesn't have the child property, that's an error
                if !parent_selection.is_empty() {
                    let child_prop = rule.selector.rsplitn(2, '.').collect::<Vec<&str>>()[0]
                        .trim_end_matches(']')  // Handle array notation
                        .trim_start_matches('[');
                    
                    log::debug!("Parent exists, checking child property: {}", child_prop);
                    
                    // Try direct property lookup for objects
                    let child_exists = parent_selection.iter().any(|parent| {
                        if let Value::Object(obj) = parent {
                            obj.contains_key(child_prop)
                        } else {
                            false
                        }
                    });
                    
                    if !child_exists {
                        log::debug!("Child property {} doesn't exist", child_prop);
                        errors.push(ValidationError {
                            path: rule.selector.clone(),
                            message: rule.error_message.clone(),
                            rule_id: rule.id.clone(),
                        });
                        return Err(errors);
                    }
                }
            }
        }
        
        // Apply JSON path selector to find the values to validate
        let selection = jsonpath_lib::select(json, &rule.selector)
            .map_err(|e| vec![ValidationError {
                path: rule.selector.clone(),
                message: format!("Invalid JSON path: {}", e),
                rule_id: rule.id.clone(),
            }])?;
        
        log::debug!("JSONPath selection result count: {}", selection.len());
        if !selection.is_empty() {
            log::debug!("Selection values: {:?}", selection);
        }
            
        // If no values match the selector
        if selection.is_empty() {
            // For required fields with no matches, this is an error
            if rule.condition == "required" {
                log::debug!("Required field not found: {}", rule.selector);
                errors.push(ValidationError {
                    path: rule.selector.clone(),
                    message: rule.error_message.clone(),
                    rule_id: rule.id.clone(),
                });
                return Err(errors);
            }
            return Ok(());
        }
        
        // For each selected value, apply the condition
        for (idx, selected) in selection.iter().enumerate() {
            let result = self.evaluate_condition(selected, &rule.condition);
            log::debug!("Condition {} on value {:?} result: {}", rule.condition, selected, result);
            
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
            _ if condition.starts_with("equals:") => {
                if let Some(expected_val) = condition.strip_prefix("equals:") {
                    let expected_val = expected_val.trim();
                    
                    // Handle different value types
                    if let Ok(expected_num) = expected_val.parse::<i64>() {
                        if let Some(actual_num) = value.as_i64() {
                            return actual_num == expected_num;
                        }
                    } else if let Ok(expected_float) = expected_val.parse::<f64>() {
                        if let Some(actual_float) = value.as_f64() {
                            return (actual_float - expected_float).abs() < f64::EPSILON;
                        }
                    } else if expected_val == "true" && value.is_boolean() {
                        return value.as_bool().unwrap_or(false);
                    } else if expected_val == "false" && value.is_boolean() {
                        return !value.as_bool().unwrap_or(true);
                    } else if let Some(actual_str) = value.as_str() {
                        return actual_str == expected_val;
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