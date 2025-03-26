#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::models::ValidationRule;
    use crate::rules::RuleRepository;
    use crate::validation::ValidationEngine;

    #[test]
    fn test_validation_with_valid_data() {
        // Create a test rules file
        let rule_repository = RuleRepository::new();
        
        // Create a ValidationEngine with the rules
        let validation_engine = ValidationEngine::new(rule_repository);
        
        // Create a test JSON
        let json = json!({
            "name": "John Doe",
            "age": 30,
            "email": "john@example.com",
            "address": {
                "city": "New York"
            },
            "items": [
                {"id": "item1", "quantity": 5},
                {"id": "item2", "quantity": 10}
            ]
        });
        
        // Validate the JSON with DEFAULT journey and CUSTOMER system
        let result = validation_engine.validate(&json, "DEFAULT", "CUSTOMER").unwrap();
        
        // The JSON should be valid (no rules to break)
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }
    
    #[test]
    fn test_key_field_extraction() {
        // Create a ValidationEngine
        let rule_repository = RuleRepository::new();
        let validation_engine = ValidationEngine::new(rule_repository);
        
        // Create a test JSON
        let json = json!({
            "name": "John Doe",
            "nested": {
                "field": "value"
            }
        });
        
        // Extract key fields
        let key_fields = validation_engine.extract_key_fields(&json).unwrap();
        
        // Check that the key fields were correctly extracted
        assert!(key_fields.contains(&"name".to_string()));
        assert!(key_fields.contains(&"nested".to_string()));
        assert!(key_fields.contains(&"nested.field".to_string()));
    }
    
    #[test]
    fn test_journey_and_system_filtering() {
        // Create a test repository with rules
        let mut rule_repository = RuleRepository::new();
        
        // Manually add rules with different journeys and systems
        let mut rules = Vec::new();
        
        // Rule 1: DEFAULT journey, CUSTOMER system
        rules.push(ValidationRule {
            id: "rule1".to_string(),
            selector: "$.name".to_string(),
            condition: "is_string".to_string(),  // Change to is_string since empty string passes required check
            key_fields: "name".to_string(),
            error_message: "Name must be a string".to_string(),
            journey: "DEFAULT".to_string(),
            system: "CUSTOMER".to_string(),
        });
        
        // Rule 2: ALL_CHECKS journey, CUSTOMER system
        rules.push(ValidationRule {
            id: "rule2".to_string(),
            selector: "$.email".to_string(),
            condition: "min_length:5".to_string(),
            key_fields: "email".to_string(),
            error_message: "Email must be at least 5 characters".to_string(),
            journey: "ALL_CHECKS".to_string(),
            system: "CUSTOMER".to_string(),
        });
        
        // Rule 3: FAST_CHECK journey, INVENTORY system
        rules.push(ValidationRule {
            id: "rule3".to_string(),
            selector: "$.quantity".to_string(),
            condition: "is_number".to_string(),
            key_fields: "quantity".to_string(),
            error_message: "Quantity must be a number".to_string(),
            journey: "FAST_CHECK".to_string(),
            system: "INVENTORY".to_string(),
        });
        
        // Add rules to repository (simulating rule repository behavior)
        for rule in &rules {
            let key_fields = rule.key_fields.split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
                
            for field in &key_fields {
                rule_repository.add_rule(field.clone(), rule.clone());
            }
        }
        
        let validation_engine = ValidationEngine::new(rule_repository);
        
        // Test data
        let json = json!({
            "name": 123,  // Should fail is_string check
            "email": "abc",
            "quantity": "not-a-number"
        });
        
        // Test 1: DEFAULT journey, CUSTOMER system
        // Should only apply rule1
        let result1 = validation_engine.validate(&json, "DEFAULT", "CUSTOMER").unwrap();
        assert!(!result1.valid);
        assert_eq!(result1.errors.len(), 1);
        assert_eq!(result1.errors[0].rule_id, "rule1");
        
        // Test 2: ALL_CHECKS journey, CUSTOMER system
        // Should apply rule1 and rule2
        let result2 = validation_engine.validate(&json, "ALL_CHECKS", "CUSTOMER").unwrap();
        assert!(!result2.valid);
        assert_eq!(result2.errors.len(), 2);
        
        // Test 3: FAST_CHECK journey, INVENTORY system
        // Should only apply rule3
        let result3 = validation_engine.validate(&json, "FAST_CHECK", "INVENTORY").unwrap();
        assert!(!result3.valid);
        assert_eq!(result3.errors.len(), 1);
        assert_eq!(result3.errors[0].rule_id, "rule3");
    }
}