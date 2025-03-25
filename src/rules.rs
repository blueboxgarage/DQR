use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use crate::error::DqrError;
use crate::models::ValidationRule;

#[derive(Clone)]
pub struct RuleRepository {
    rules: HashMap<String, Vec<ValidationRule>>,
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
                    .or_insert_with(Vec::new)
                    .push(rule.clone());
            }
        }

        Ok(())
    }

    pub fn get_rules_for_key_field(&self, key_field: &str) -> Vec<ValidationRule> {
        self.rules
            .get(key_field)
            .cloned()
            .unwrap_or_else(Vec::new)
    }
    
    pub fn get_rules_for_key_fields(&self, fields: &[String]) -> Vec<ValidationRule> {
        let mut matched_rules = Vec::new();
        
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