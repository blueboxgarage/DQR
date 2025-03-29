use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use crate::error::DqrError;
use crate::models::ValidationRule;

#[derive(Clone)]
pub struct RuleRepository {
    rules: HashMap<String, Vec<ValidationRule>>,
    conditional_rules: HashMap<String, (Vec<ValidationRule>, Vec<ValidationRule>)>,
}

impl Default for RuleRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleRepository {
    pub fn new() -> Self {
        RuleRepository {
            rules: HashMap::new(),
            conditional_rules: HashMap::new(),
        }
    }

    pub fn load_from_csv<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), DqrError> {
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);
        
        // Temporary storage for conditional rules
        let mut then_rules: HashMap<String, Vec<ValidationRule>> = HashMap::new();
        let mut else_rules: HashMap<String, Vec<ValidationRule>> = HashMap::new();

        for result in reader.deserialize() {
            let rule: ValidationRule = result?;
            
            // Handle conditional rules specially
            match rule.logic_type {
                crate::models::ConditionalLogic::Then => {
                    // Store "then" rules indexed by their parent rule ID
                    then_rules
                        .entry(rule.parent_rule_id.clone())
                        .or_default()
                        .push(rule);
                    continue;
                },
                crate::models::ConditionalLogic::Else => {
                    // Store "else" rules indexed by their parent rule ID
                    else_rules
                        .entry(rule.parent_rule_id.clone())
                        .or_default()
                        .push(rule);
                    continue;
                },
                _ => {
                    // Standard rules and "if" rules are processed normally
                }
            }
            
            // Split comma-separated key fields
            let key_fields: Vec<String> = if rule.key_fields.is_empty() {
                Vec::new()
            } else {
                rule.key_fields
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            };
            
            // Insert rule into our map, grouped by each key field
            for field in &key_fields {
                self.rules
                    .entry(field.clone())
                    .or_default()
                    .push(rule.clone());
            }
        }
        
        // Now that we've processed all rules, organize the conditional rules
        for (parent_id, then_branch) in then_rules {
            let else_branch = else_rules.remove(&parent_id).unwrap_or_default();
            self.conditional_rules.insert(parent_id, (then_branch, else_branch));
        }
        
        // If there are any remaining else rules without matching then rules, add them too
        for (parent_id, else_branch) in else_rules {
            self.conditional_rules.insert(parent_id, (Vec::new(), else_branch));
        }

        Ok(())
    }
    
    // Method to add a rule directly (useful for testing)
    pub fn add_rule(&mut self, key_field: String, rule: ValidationRule) {
        self.rules
            .entry(key_field)
            .or_default()
            .push(rule);
    }

    pub fn get_rules_for_key_field(&self, key_field: &str) -> Vec<ValidationRule> {
        self.rules
            .get(key_field)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
    
    pub fn get_rules_for_key_fields(&self, fields: &[String]) -> Vec<ValidationRule> {
        let mut matched_rules = Vec::new();
        
        // Special case: if fields contains "*", return all rules
        if fields.contains(&"*".to_string()) {
            // Collect all unique rules
            let mut all_rules = Vec::new();
            for rules in self.rules.values() {
                all_rules.extend(rules.clone());
            }
            
            // Remove duplicates
            all_rules.sort_by(|a, b| a.id.cmp(&b.id));
            all_rules.dedup_by(|a, b| a.id == b.id);
            
            return all_rules;
        }
        
        // Normal case: match by field
        for field in fields {
            if let Some(rules) = self.rules.get(field) {
                matched_rules.extend(rules.clone());
            }
        }
        
        // Remove duplicates (if a rule matches multiple key fields)
        matched_rules.sort_by(|a, b| a.id.cmp(&b.id));
        matched_rules.dedup_by(|a, b| a.id == b.id);
        
        matched_rules
    }
    
    // Get the conditional branches (then/else) for a specific rule ID
    pub fn get_conditional_rules(&self, parent_id: &str) -> (Vec<ValidationRule>, Vec<ValidationRule>) {
        match self.conditional_rules.get(parent_id) {
            Some((then_rules, else_rules)) => (then_rules.clone(), else_rules.clone()),
            None => (Vec::new(), Vec::new())
        }
    }
}