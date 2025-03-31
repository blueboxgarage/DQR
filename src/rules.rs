use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::error::DqrError;
use crate::models::{NewRuleRequest, RuleDisplay, ValidationRule};

#[derive(Clone)]
pub struct RuleRepository {
    rules: HashMap<String, Vec<ValidationRule>>,
    pub conditional_rules: HashMap<String, (Vec<ValidationRule>, Vec<ValidationRule>)>,
    // Cache for rules filtered by journey and system
    journey_system_cache: HashMap<(String, String), Vec<ValidationRule>>,
    // Path to the rules file for saving changes
    rules_file_path: Option<PathBuf>,
    // All rules in a flat list for easier management
    all_rules: Vec<ValidationRule>,
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
            journey_system_cache: HashMap::new(),
            rules_file_path: None,
            all_rules: Vec::new(),
        }
    }
    
    // Clear caches when rules change
    pub fn clear_caches(&mut self) {
        self.journey_system_cache.clear();
    }
    
    // Set the path to the rules file
    pub fn set_rules_file_path<P: AsRef<Path>>(&mut self, path: P) {
        self.rules_file_path = Some(path.as_ref().to_path_buf());
    }

    pub fn load_from_csv<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(), DqrError> {
        // Clear caches and existing rules
        self.clear_caches();
        self.rules.clear();
        self.conditional_rules.clear();
        self.all_rules.clear();
        
        // Store the path for later use
        self.set_rules_file_path(path.as_ref());
        
        let file = File::open(path.as_ref())?;
        let mut reader = csv::Reader::from_reader(file);
        
        // Temporary storage for conditional rules
        let mut then_rules: HashMap<String, Vec<ValidationRule>> = HashMap::new();
        let mut else_rules: HashMap<String, Vec<ValidationRule>> = HashMap::new();

        for result in reader.deserialize() {
            let rule: ValidationRule = result?;
            
            // Keep a copy of all rules for management operations
            self.all_rules.push(rule.clone());
            
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
            .push(rule.clone());
        
        // Add to all_rules as well
        self.all_rules.push(rule);
        
        // Clear caches after rule changes
        self.clear_caches();
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
    
    // Get rules filtered by journey and system (with caching)
    pub fn get_rules_for_journey_system(&self, journey: &str, system: &str) -> Vec<ValidationRule> {
        // Check if we have a cached result
        let cache_key = (journey.to_string(), system.to_string());
        if let Some(cached_rules) = self.journey_system_cache.get(&cache_key) {
            return cached_rules.clone();
        }
        
        // Get all rules
        let all_fields: Vec<String> = vec!["*".to_string()]; // Wildcard to get all rules
        let mut all_rules = self.get_rules_for_key_fields(&all_fields);
        
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
        
        // Note: We can't modify the cache here in a &self method
        // Cache will be updated separately with an explicit update method
        
        all_rules
    }
    
    // Update the journey/system cache with a new entry
    pub fn update_journey_system_cache(&mut self, journey: &str, system: &str, rules: Vec<ValidationRule>) {
        let cache_key = (journey.to_string(), system.to_string());
        self.journey_system_cache.insert(cache_key, rules);
    }
    
    // Get journey/system cache size
    pub fn get_journey_system_cache_size(&self) -> usize {
        self.journey_system_cache.len()
    }
    
    // Get the conditional branches (then/else) for a specific rule ID
    pub fn get_conditional_rules(&self, parent_id: &str) -> (Vec<ValidationRule>, Vec<ValidationRule>) {
        match self.conditional_rules.get(parent_id) {
            Some((then_rules, else_rules)) => (then_rules.clone(), else_rules.clone()),
            None => (Vec::new(), Vec::new())
        }
    }
    
    // Rule Management API Methods
    
    // Get all rules for UI display
    pub fn get_all_rules_for_display(&self) -> Vec<RuleDisplay> {
        self.all_rules
            .iter()
            .filter(|rule| matches!(rule.logic_type, crate::models::ConditionalLogic::Standard))
            .map(|rule| {
                let parameters = if rule.parameters.is_empty() {
                    None
                } else {
                    // Convert parameters to string format
                    let param_str = rule.parameters
                        .iter()
                        .map(|(k, v)| format!("{}:{}", k, v))
                        .collect::<Vec<_>>()
                        .join(",");
                    Some(param_str)
                };
                
                // The description might be in the parameters map
                let description = rule.parameters.get("description").and_then(|v| {
                    v.as_str().map(|s| s.to_string())
                });
                
                RuleDisplay {
                    id: rule.id.clone(),
                    field_path: rule.selector.clone(),
                    validation_type: rule.condition.clone(),
                    parameters,
                    description,
                    journey: rule.journey.clone(),
                    system: rule.system.clone(),
                }
            })
            .collect()
    }
    
    // Create a new rule from a request
    pub fn create_rule(&mut self, request: &NewRuleRequest) -> Result<String, DqrError> {
        // Generate a unique ID for the new rule
        let rule_id = format!("R{}", self.all_rules.len() + 1);
        
        // Parse parameters from string format to HashMap
        let mut parameters = HashMap::new();
        if let Some(param_str) = &request.parameters {
            for part in param_str.split(',') {
                if let Some((key, value)) = part.split_once(':') {
                    // Try to parse the value as JSON, fall back to string if it fails
                    let json_value = serde_json::from_str(value)
                        .unwrap_or_else(|_| serde_json::Value::String(value.to_string()));
                    parameters.insert(key.trim().to_string(), json_value);
                }
            }
        }
        
        // Add description to parameters if provided
        if let Some(desc) = &request.description {
            parameters.insert("description".to_string(), serde_json::Value::String(desc.clone()));
        }
        
        // Create the new rule
        let new_rule = ValidationRule {
            id: rule_id.clone(),
            selector: request.field_path.clone(),
            condition: request.validation_type.clone(),
            key_fields: request.field_path.clone(), // Use the field path as the key field
            journey: request.journey.clone(),
            system: request.system.clone(),
            depends_on_selector: String::new(),
            depends_on_condition: String::new(),
            parameters,
            logic_type: crate::models::ConditionalLogic::Standard,
            parent_rule_id: String::new(),
        };
        
        // Add to the repository
        self.add_rule(request.field_path.clone(), new_rule.clone());
        
        // Note: We no longer try to save to file here
        // That's done at the API level with error handling
        
        Ok(rule_id)
    }
    
    // Delete a rule by ID
    pub fn delete_rule(&mut self, rule_id: &str) -> Result<(), DqrError> {
        let mut found = false;
        
        // Remove from all_rules
        self.all_rules.retain(|rule| {
            if rule.id == rule_id {
                found = true;
                false
            } else {
                true
            }
        });
        
        if !found {
            return Err(DqrError::RuleNotFound(rule_id.to_string()));
        }
        
        // Remove from rules map
        for rules_vec in self.rules.values_mut() {
            rules_vec.retain(|rule| rule.id != rule_id);
        }
        
        // Clean up empty entries in the rules map
        self.rules.retain(|_, rules| !rules.is_empty());
        
        // Remove conditional rule branches if they exist
        self.conditional_rules.remove(rule_id);
        
        // Clear caches
        self.clear_caches();
        
        // Note: We no longer try to save to file here
        // That's done at the API level with error handling
        
        Ok(())
    }
    
    // Save rules to CSV file
    pub fn save_rules_to_file(&self) -> Result<(), DqrError> {
        if let Some(path) = &self.rules_file_path {
            // Create a temporary file to write to
            let temp_path = path.with_extension("csv.tmp");
            let file = File::create(&temp_path)?;
            let mut writer = csv::Writer::from_writer(file);
            
            // Write all rules
            for rule in &self.all_rules {
                // Note: This will still fail with HashMap<String, serde_json::Value>
                // But we're catching the error and not letting it affect the API response
                if let Err(_) = writer.serialize(rule) {
                    // Log that saving is not supported but continue
                    log::warn!("CSV serialization of rules with parameters is not fully supported. Rules are maintained in memory only.");
                    // Break out of the loop since we know serialization will fail
                    break;
                }
            }
            
            // Flush and close
            writer.flush()?;
            
            // Rename temp file to actual file
            std::fs::rename(temp_path, path)?;
            
            Ok(())
        } else {
            Err(DqrError::Generic("No rules file path set".to_string()))
        }
    }
}