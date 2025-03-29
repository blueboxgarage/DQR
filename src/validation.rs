use serde_json::Value;
use regex::Regex;

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
        
        if errors.is_empty() {
            Ok(ValidationResponse::success())
        } else {
            Ok(ValidationResponse::failure(errors))
        }
    }
    
    fn apply_rule(&self, json: &Value, rule: &ValidationRule) -> Result<(bool, Vec<ValidationError>), DqrError> {
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