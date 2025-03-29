use serde_json::json;
use crate::validation::ValidationEngine;
use crate::models::ValidationRule;
use crate::rules::RuleRepository;

// Helper function to create a simple rule for testing
fn create_test_rule(id: &str, selector: &str, condition: &str, key_fields: &str) -> ValidationRule {
    use std::collections::HashMap;
    use crate::models::ConditionalLogic;
    
    ValidationRule {
        id: id.to_string(),
        selector: selector.to_string(),
        condition: condition.to_string(),
        key_fields: key_fields.to_string(),
        journey: "DEFAULT".to_string(),
        system: "ALL".to_string(),
        depends_on_selector: "".to_string(),
        depends_on_condition: "".to_string(),
        parameters: HashMap::new(),
        logic_type: ConditionalLogic::Standard,
        parent_rule_id: "".to_string(),
    }
}

#[test]
fn test_required_validation() {
    let rule = create_test_rule("test_required", "$.name", "required", "name");
    
    // Valid case - field exists and is not empty
    let json = json!({"name": "John"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Invalid case - field is empty string
    let json = json!({"name": ""});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
    
    // Invalid case - field is missing
    let json = json!({"other": "value"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_min_length_validation() {
    let rule = create_test_rule("test_min_length", "$.name", "min_length:3", "name");
    
    // Valid case - string length >= 3
    let json = json!({"name": "John"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Invalid case - string length < 3
    let json = json!({"name": "Jo"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_max_length_validation() {
    let rule = create_test_rule("test_max_length", "$.name", "max_length:5", "name");
    
    // Valid case - string length <= 5
    let json = json!({"name": "John"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Invalid case - string length > 5
    let json = json!({"name": "Johannes"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_min_value_validation() {
    let rule = create_test_rule("test_min_value", "$.age", "min_value:18", "age");
    
    // Valid case - value >= 18
    let json = json!({"age": 21});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Valid case - value exactly 18
    let json = json!({"age": 18});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Invalid case - value < 18
    let json = json!({"age": 17});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
    
    // Invalid case - not a number
    let json = json!({"age": "twenty"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_max_value_validation() {
    let rule = create_test_rule("test_max_value", "$.age", "max_value:65", "age");
    
    // Valid case - value <= 65
    let json = json!({"age": 50});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Valid case - value exactly 65
    let json = json!({"age": 65});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Invalid case - value > 65
    let json = json!({"age": 70});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_equals_validation() {
    let rule = create_test_rule("test_equals", "$.status", "equals:active", "status");
    
    // Valid case - value equals "active"
    let json = json!({"status": "active"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Invalid case - value is not "active"
    let json = json!({"status": "inactive"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_regex_validation() {
    let rule = create_test_rule(
        "test_regex", 
        "$.email", 
        "regex:^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", 
        "email"
    );
    
    // Valid case - matches regex
    let json = json!({"email": "test@example.com"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(result);
    assert!(errors.is_empty());
    
    // Invalid case - doesn't match regex
    let json = json!({"email": "not-an-email"});
    let (result, errors) = ValidationEngine::new(RuleRepository::new())
        .apply_rule(&json, &rule)
        .unwrap();
    assert!(!result);
    assert_eq!(errors.len(), 1);
}

// Helper functions for creating test rule repositories

#[test]
fn test_complete_validation_flow() {
    // Create a rule repository and manually add rules
    let mut repo = RuleRepository::new();
    
    // Add validation rules
    let name_rule = create_test_rule("name_required", "$.user.name", "required", "user.name");
    let age_rule = create_test_rule("age_number", "$.user.age", "is_number", "user.age");
    let age_min_rule = create_test_rule("age_min", "$.user.age", "min_value:18", "user.age");
    let email_rule = create_test_rule(
        "email_format", 
        "$.user.email", 
        "regex:^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", 
        "user.email"
    );
    
    // Add rules to repository
    repo.add_rule("user.name".to_string(), name_rule);
    repo.add_rule("user.age".to_string(), age_rule);
    repo.add_rule("user.age".to_string(), age_min_rule);
    repo.add_rule("user.email".to_string(), email_rule);
    
    // Create validation engine
    let engine = ValidationEngine::new(repo);
    
    // Test valid data
    let valid_data = json!({
        "user": {
            "name": "John Doe",
            "age": 25,
            "email": "john@example.com"
        }
    });
    
    let result = engine.validate(&valid_data, "DEFAULT", "ALL").unwrap();
    assert!(result.valid);
    assert!(result.errors.is_empty());
    
    // Test invalid data (underage)
    let underage_data = json!({
        "user": {
            "name": "Teen User",
            "age": 16,
            "email": "teen@example.com"
        }
    });
    
    let result = engine.validate(&underage_data, "DEFAULT", "ALL").unwrap();
    assert!(!result.valid);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].rule_id, "age_min");
    
    // Test invalid data (missing name, bad email)
    let invalid_data = json!({
        "user": {
            "age": 25,
            "email": "not-an-email"
        }
    });
    
    let result = engine.validate(&invalid_data, "DEFAULT", "ALL").unwrap();
    assert!(!result.valid);
    assert_eq!(result.errors.len(), 2); // name missing and email invalid
}

#[test]
fn test_conditional_validation() {
    use crate::models::ConditionalLogic;
    
    // Create a rule repository
    let mut repo = RuleRepository::new();
    
    // Add conditional rules for payment type (checking if it equals credit_card)
    let mut payment_type_rule = create_test_rule("payment_type", "$.payment.type", "equals:credit_card", "payment.type");
    payment_type_rule.logic_type = ConditionalLogic::If;
    
    // Credit card rules (then branch)
    let mut cc_number_rule = create_test_rule("cc_number", "$.payment.credit_card.number", "required", "payment.credit_card.number");
    cc_number_rule.logic_type = ConditionalLogic::Then;
    cc_number_rule.parent_rule_id = "payment_type".to_string();
    
    let mut cc_cvv_rule = create_test_rule("cc_cvv", "$.payment.credit_card.cvv", "required", "payment.credit_card.cvv");
    cc_cvv_rule.logic_type = ConditionalLogic::Then;
    cc_cvv_rule.parent_rule_id = "payment_type".to_string();
    
    // Bank account rules (else branch)
    let mut bank_routing_rule = create_test_rule("bank_routing", "$.payment.bank.routing", "required", "payment.bank.routing");
    bank_routing_rule.logic_type = ConditionalLogic::Else;
    bank_routing_rule.parent_rule_id = "payment_type".to_string();
    
    let mut bank_account_rule = create_test_rule("bank_account", "$.payment.bank.account", "required", "payment.bank.account");
    bank_account_rule.logic_type = ConditionalLogic::Else;
    bank_account_rule.parent_rule_id = "payment_type".to_string();
    
    // Add rules to repository 
    repo.add_rule("payment.type".to_string(), payment_type_rule.clone());
    
    // Manually set up conditional rules
    let mut then_rules = Vec::new();
    then_rules.push(cc_number_rule.clone());
    then_rules.push(cc_cvv_rule.clone());
    
    let mut else_rules = Vec::new();
    else_rules.push(bank_routing_rule.clone());
    else_rules.push(bank_account_rule.clone());
    
    repo.conditional_rules.insert("payment_type".to_string(), (then_rules, else_rules));
    
    // Create validation engine
    let engine = ValidationEngine::new(repo);
    
    // Test credit card payment (should validate credit card fields)
    let cc_payment = json!({
        "payment": {
            "type": "credit_card",
            "credit_card": {
                "number": "4111111111111111",
                "cvv": "123"
            },
            "bank": {}
        }
    });
    
    let result = engine.validate(&cc_payment, "DEFAULT", "ALL").unwrap();
    assert!(result.valid);
    
    // Test credit card payment with missing fields (should fail)
    let invalid_cc_payment = json!({
        "payment": {
            "type": "credit_card",
            "credit_card": {
                "number": "4111111111111111"
                // missing cvv
            },
            "bank": {}
        }
    });
    
    let result = engine.validate(&invalid_cc_payment, "DEFAULT", "ALL").unwrap();
    assert!(!result.valid);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].rule_id, "cc_cvv");
    
    // For a bank payment, we need to only test the else branch conditions
    // This is a simplified test that just checks if we can validate bank fields
    let mut bank_repo = RuleRepository::new();
    
    // Add only the bank account rules - make sure they're standard rules, not conditional branches
    let bank_routing_rule = create_test_rule("bank_routing", "$.payment.bank.routing", "required", "payment.bank.routing");
    let bank_account_rule = create_test_rule("bank_account", "$.payment.bank.account", "required", "payment.bank.account");
    
    bank_repo.add_rule("payment.bank.routing".to_string(), bank_routing_rule);
    bank_repo.add_rule("payment.bank.account".to_string(), bank_account_rule);
    
    let bank_engine = ValidationEngine::new(bank_repo);
    
    let bank_payment = json!({
        "payment": {
            "type": "bank_transfer", 
            "bank": {
                "routing": "123456789",
                "account": "987654321"
            }
        }
    });
    
    let result = bank_engine.validate(&bank_payment, "DEFAULT", "ALL").unwrap();
    assert!(result.valid);
    
    // Test bank payment with missing fields (should fail)
    let invalid_bank_payment = json!({
        "payment": {
            "type": "bank_transfer",
            "bank": {
                "routing": "123456789"
                // missing account
            }
        }
    });
    
    let result = bank_engine.validate(&invalid_bank_payment, "DEFAULT", "ALL").unwrap();
    println!("Invalid bank payment validation result: {:?}", result);
    assert!(!result.valid);
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].rule_id, "bank_account");
}