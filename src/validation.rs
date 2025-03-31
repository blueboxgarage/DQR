use serde_json::Value;
use regex::Regex;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::error::DqrError;
use crate::models::{NewRuleRequest, RuleDisplay, ValidationError, ValidationRule, ValidationResponse};
use crate::rules::RuleRepository;

#[derive(Clone)]
pub struct ValidationEngine {
    rule_repository: RuleRepository,
    // Cache for validation results
    validation_cache: HashMap<u64, ValidationResponse>,
}

impl ValidationEngine {
    pub fn new(rule_repository: RuleRepository) -> Self {
        ValidationEngine { 
            rule_repository,
            validation_cache: HashMap::new(),
        }
    }
    
    // Rule management methods
    
    // Get all rules for display
    pub fn get_rules_for_display(&self) -> Vec<RuleDisplay> {
        self.rule_repository.get_all_rules_for_display()
    }
    
    // Create a new rule
    pub fn create_rule(&mut self, req: &NewRuleRequest) -> Result<String, DqrError> {
        self.rule_repository.create_rule(req)
    }
    
    // Delete a rule
    pub fn delete_rule(&mut self, rule_id: &str) -> Result<(), DqrError> {
        self.rule_repository.delete_rule(rule_id)
    }
    
    // Save rules to file
    pub fn save_rules_to_file(&self) -> Result<(), DqrError> {
        self.rule_repository.save_rules_to_file()
    }
    
    // Helper method to calculate a hash for the validation inputs
    fn calculate_hash(&self, json: &Value, journey: &str, system: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        
        // Hash the JSON data
        let json_str = json.to_string();
        json_str.hash(&mut hasher);
        
        // Hash the journey and system
        journey.hash(&mut hasher);
        system.hash(&mut hasher);
        
        hasher.finish()
    }
    
    // Clear the validation cache
    pub fn clear_validation_cache(&mut self) {
        self.validation_cache.clear();
    }
    
    // Get validation cache size
    pub fn get_validation_cache_size(&self) -> usize {
        self.validation_cache.len()
    }
    
    // Get journey system cache size
    pub fn get_journey_system_cache_size(&self) -> usize {
        self.rule_repository.get_journey_system_cache_size()
    }

    // Get the relevant rules for a specific journey and system combination
    pub fn get_rules_for_journey_system(&self, journey: &str, system: &str) -> Vec<ValidationRule> {
        // Use the cached method from repository
        self.rule_repository.get_rules_for_journey_system(journey, system)
    }

    pub fn validate(
        &self, 
        json: &Value, 
        journey: &str, 
        system: &str
    ) -> Result<ValidationResponse, DqrError> {
        // Calculate hash for cache lookup
        let cache_key = self.calculate_hash(json, journey, system);
        
        // Check cache first
        if let Some(cached_result) = self.validation_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }
        
        // Get applicable rules for this journey and system
        let rules = self.get_rules_for_journey_system(journey, system);
        
        // Apply validation rules
        let mut errors = Vec::new();
        
        for rule in rules {
            // Skip conditional branch rules here - they'll be processed by their parent
            if rule.is_conditional_branch() {
                continue;
            }
            
            // Apply the rule
            match self.apply_rule(json, &rule) {
                Ok((condition_result, rule_errors)) => {
                    // Add any errors from this rule
                    errors.extend(rule_errors);
                    
                    // If this is a conditional root rule, process its branches
                    if rule.is_condition_root() {
                        let conditional_errors = self.process_conditional_rules(json, &rule.id, condition_result);
                        errors.extend(conditional_errors);
                    }
                },
                Err(e) => {
                    log::error!("Error applying rule {}: {}", rule.id, e);
                }
            }
        }
        
        // Create the response
        let response = if errors.is_empty() {
            ValidationResponse::success()
        } else {
            ValidationResponse::failure(errors)
        };
        
        // Note: We can't modify the cache here in a &self method
        // Cache will be updated separately with an explicit update method in the mutable version
        
        Ok(response)
    }
    
    // Mutable version of validate that updates caches
    pub fn validate_mut(
        &mut self, 
        json: &Value, 
        journey: &str, 
        system: &str
    ) -> Result<ValidationResponse, DqrError> {
        // Calculate hash for cache lookup
        let cache_key = self.calculate_hash(json, journey, system);
        
        // Check cache first
        if let Some(cached_result) = self.validation_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }
        
        // Get applicable rules and update rule cache
        let rules = self.rule_repository.get_rules_for_journey_system(journey, system);
        
        // Apply validation rules
        let mut errors = Vec::new();
        
        for rule in &rules {
            // Skip conditional branch rules here - they'll be processed by their parent
            if rule.is_conditional_branch() {
                continue;
            }
            
            // Apply the rule
            match self.apply_rule(json, rule) {
                Ok((condition_result, rule_errors)) => {
                    // Add any errors from this rule
                    errors.extend(rule_errors);
                    
                    // If this is a conditional root rule, process its branches
                    if rule.is_condition_root() {
                        let conditional_errors = self.process_conditional_rules(json, &rule.id, condition_result);
                        errors.extend(conditional_errors);
                    }
                },
                Err(e) => {
                    log::error!("Error applying rule {}: {}", rule.id, e);
                }
            }
        }
        
        // Create the response
        let response = if errors.is_empty() {
            ValidationResponse::success()
        } else {
            ValidationResponse::failure(errors)
        };
        
        // Update the cache
        self.validation_cache.insert(cache_key, response.clone());
        
        Ok(response)
    }
    
    pub fn apply_rule(&self, json: &Value, rule: &ValidationRule) -> Result<(bool, Vec<ValidationError>), DqrError> {
        // Check dependency condition first if it exists
        if !rule.depends_on_selector.is_empty() && !rule.depends_on_condition.is_empty() {
            // Apply the depends_on selector
            let depends_selection = match jsonpath_lib::select(json, &rule.depends_on_selector) {
                Ok(selection) => selection,
                Err(_) => {
                    // If we can't evaluate the dependency, skip this rule
                    return Ok((true, Vec::new()));
                }
            };
            
            // Check if any selected value matches the dependency condition
            let mut dependency_met = false;
            for depends_value in depends_selection.iter() {
                // Check the dependency condition
                if rule.depends_on_condition.starts_with("equals:") {
                    if let Some(expected_value) = rule.depends_on_condition.strip_prefix("equals:") {
                        match depends_value {
                            Value::String(s) => {
                                if s == expected_value {
                                    dependency_met = true;
                                    break;
                                }
                            },
                            Value::Number(n) => {
                                if let Ok(num) = expected_value.parse::<f64>() {
                                    if let Some(val_num) = n.as_f64() {
                                        if (val_num - num).abs() < f64::EPSILON {
                                            dependency_met = true;
                                            break;
                                        }
                                    }
                                }
                            },
                            Value::Bool(b) => {
                                if let Ok(expected_bool) = expected_value.parse::<bool>() {
                                    if *b == expected_bool {
                                        dependency_met = true;
                                        break;
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                } else if rule.depends_on_condition == "not_empty" {
                    // Check if the value is not null and not an empty string
                    match depends_value {
                        Value::String(s) => {
                            if !s.is_empty() {
                                dependency_met = true;
                                break;
                            }
                        },
                        Value::Array(arr) => {
                            if !arr.is_empty() {
                                dependency_met = true;
                                break;
                            }
                        },
                        Value::Object(obj) => {
                            if !obj.is_empty() {
                                dependency_met = true;
                                break;
                            }
                        },
                        Value::Number(_) | Value::Bool(_) => {
                            // Numbers and booleans are never empty
                            dependency_met = true;
                            break;
                        },
                        Value::Null => {
                            // Null is considered empty
                        }
                    }
                }
                // Add more dependency condition types here if needed
            }
            
            // If the dependency condition is not met, skip this rule
            if !dependency_met {
                return Ok((true, Vec::new()));
            }
        }
        
        // Apply JSON path selector to find the values to validate
        let selection = jsonpath_lib::select(json, &rule.selector)
            .map_err(|_| DqrError::JsonPathError(format!("Invalid JSONPath: {}", rule.selector)))?;
        
        // If no values match and condition is required, that's an error
        if rule.condition == "required" && selection.is_empty() {
            return Ok((false, vec![ValidationError {
                path: rule.selector.clone(),
                rule_id: rule.id.clone(),
            }]));
        }
        
        // If selection is empty for non-required conditions, consider validation passed
        if selection.is_empty() && rule.condition != "required" {
            return Ok((true, Vec::new()));
        }
        
        // Process each condition
        let mut errors = Vec::new();
        let mut condition_passed = true;
        
        for (idx, value) in selection.iter().enumerate() {
            let path = format!("{} (item {})", rule.selector, idx);
            
            match rule.condition.as_str() {
                "required" => {
                    if value.is_null() || (value.is_string() && value.as_str().unwrap().is_empty()) {
                        errors.push(ValidationError {
                            path: path.clone(),
                            rule_id: rule.id.clone(),
                        });
                    }
                },
                "is_number" => {
                    if !value.is_number() {
                        errors.push(ValidationError {
                            path: path.clone(),
                            rule_id: rule.id.clone(),
                        });
                    }
                },
                "is_string" => {
                    if !value.is_string() {
                        errors.push(ValidationError {
                            path: path.clone(),
                            rule_id: rule.id.clone(),
                        });
                    }
                },
                "is_boolean" => {
                    if !value.is_boolean() {
                        errors.push(ValidationError {
                            path: path.clone(),
                            rule_id: rule.id.clone(),
                        });
                    }
                },
                "is_array" => {
                    if !value.is_array() {
                        errors.push(ValidationError {
                            path: path.clone(),
                            rule_id: rule.id.clone(),
                        });
                    }
                },
                "is_object" => {
                    if !value.is_object() {
                        errors.push(ValidationError {
                            path: path.clone(),
                            rule_id: rule.id.clone(),
                        });
                    }
                },
                condition if condition.starts_with("min_length:") => {
                    if let Some(min_length_str) = condition.strip_prefix("min_length:") {
                        if let Ok(min_length) = min_length_str.parse::<usize>() {
                            if !value.is_string() || value.as_str().unwrap().len() < min_length {
                                errors.push(ValidationError {
                                    path: path.clone(),
                                    rule_id: rule.id.clone(),
                                });
                            }
                        }
                    }
                },
                "min_length_when_single" => {
                    // Special case for single applicant
                    if let Ok(numbers_selection) = jsonpath_lib::select(json, "$.application.individuals.number") {
                        if let Some(numbers) = numbers_selection.first() {
                            if numbers.is_number() && numbers.as_i64() == Some(1) {
                                if !value.is_string() || value.as_str().unwrap().len() <= 1 {
                                    errors.push(ValidationError {
                                        path: path.clone(),
                                        rule_id: rule.id.clone(),
                                    });
                                }
                            }
                        }
                    }
                },
                condition if condition.starts_with("max_length:") => {
                    if let Some(max_length_str) = condition.strip_prefix("max_length:") {
                        if let Ok(max_length) = max_length_str.parse::<usize>() {
                            if !value.is_string() || value.as_str().unwrap().len() > max_length {
                                errors.push(ValidationError {
                                    path: path.clone(),
                                    rule_id: rule.id.clone(),
                                });
                            }
                        }
                    }
                },
                condition if condition.starts_with("min_value:") => {
                    if let Some(min_value_str) = condition.strip_prefix("min_value:") {
                        if let Ok(min_value) = min_value_str.parse::<f64>() {
                            if !value.is_number() || value.as_f64().map_or(true, |v| v < min_value) {
                                errors.push(ValidationError {
                                    path: path.clone(),
                                    rule_id: rule.id.clone(),
                                });
                            }
                        }
                    }
                },
                condition if condition.starts_with("max_value:") => {
                    if let Some(max_value_str) = condition.strip_prefix("max_value:") {
                        if let Ok(max_value) = max_value_str.parse::<f64>() {
                            if !value.is_number() || value.as_f64().map_or(true, |v| v > max_value) {
                                errors.push(ValidationError {
                                    path: path.clone(),
                                    rule_id: rule.id.clone(),
                                });
                            }
                        }
                    }
                },
                condition if condition.starts_with("equals:") => {
                    if let Some(expected_value) = condition.strip_prefix("equals:") {
                        let matches = match value {
                            Value::String(s) => s == expected_value,
                            Value::Number(n) => {
                                if let Ok(num) = expected_value.parse::<f64>() {
                                    if let Some(val_num) = n.as_f64() {
                                        (val_num - num).abs() < f64::EPSILON
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            },
                            Value::Bool(b) => {
                                if let Ok(expected_bool) = expected_value.parse::<bool>() {
                                    *b == expected_bool
                                } else {
                                    false
                                }
                            },
                            _ => false,
                        };
                        
                        if !matches {
                            errors.push(ValidationError {
                                path: path.clone(),
                                rule_id: rule.id.clone(),
                            });
                        }
                    }
                },
                condition if condition.starts_with("regex:") => {
                    if let Some(pattern) = condition.strip_prefix("regex:") {
                        if value.is_string() {
                            match Regex::new(pattern) {
                                Ok(regex) => {
                                    if !regex.is_match(value.as_str().unwrap()) {
                                        errors.push(ValidationError {
                                            path: path.clone(),
                                            rule_id: rule.id.clone(),
                                        });
                                    }
                                },
                                Err(_) => {
                                    // Invalid regex pattern - consider this a configuration error
                                    // For now, we'll just skip this validation
                                }
                            }
                        } else {
                            errors.push(ValidationError {
                                path: path.clone(),
                                rule_id: rule.id.clone(),
                            });
                        }
                    }
                },
                _ => {
                    // Unknown condition - consider this a configuration error
                    // For now, we'll just skip this validation
                }
            }
        }
        
        // If there are errors, we consider the condition to have failed
        if !errors.is_empty() {
            condition_passed = false;
        }
        
        Ok((condition_passed, errors))
    }
    
    // Process conditional branches based on the result of the "if" rule
    fn process_conditional_rules(&self, json: &Value, parent_id: &str, condition_passed: bool) -> Vec<ValidationError> {
        // Get the appropriate branch based on the condition result
        let (then_rules, else_rules) = self.rule_repository.get_conditional_rules(parent_id);
        let branch_to_process = if condition_passed { then_rules } else { else_rules };
        
        let mut all_errors = Vec::new();
        
        // Process all rules in the selected branch
        for rule in branch_to_process {
            // If this is another conditional root, process it recursively
            if rule.is_condition_root() {
                match self.apply_rule(json, &rule) {
                    Ok((result, errors)) => {
                        all_errors.extend(errors);
                        // Process nested conditionals
                        let nested_errors = self.process_conditional_rules(json, &rule.id, result);
                        all_errors.extend(nested_errors);
                    },
                    Err(e) => {
                        log::error!("Error processing conditional rule {}: {}", rule.id, e);
                    }
                }
            } else {
                // Process standard rule
                match self.apply_rule(json, &rule) {
                    Ok((_, errors)) => {
                        all_errors.extend(errors);
                    },
                    Err(e) => {
                        log::error!("Error processing rule {}: {}", rule.id, e);
                    }
                }
            }
        }
        
        all_errors
    }
}