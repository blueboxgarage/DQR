use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use crate::error::DqrError;
use crate::models::ValidationRule;

#[derive(Clone)]
pub struct RuleRepository {
    rules: HashMap<String, Vec<ValidationRule>>,
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
        }
    }

    pub fn load_from_csv<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), DqrError> {
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);

        for result in reader.deserialize() {
            let rule: ValidationRule = result?;
            
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
}