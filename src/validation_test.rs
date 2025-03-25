#[cfg(test)]
mod tests {
    use serde_json::json;

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
        
        // Validate the JSON
        let result = validation_engine.validate(&json).unwrap();
        
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
}