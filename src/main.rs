use std::env;

use dqr::api;
use dqr::rules::RuleRepository;
use dqr::validation::ValidationEngine;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists
    dotenv::dotenv().ok();
    
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Load configuration from environment variables
    let host = env::var("DQR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("DQR_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Invalid port number");
    let rules_path = env::var("DQR_RULES_PATH")
        .unwrap_or_else(|_| "rules.csv".to_string());
    
    // Initialize rule repository and validation engine
    let mut rule_repository = RuleRepository::new();
    match rule_repository.load_from_csv(&rules_path) {
        Ok(_) => log::info!("Successfully loaded rules from {}", rules_path),
        Err(e) => {
            log::warn!("Failed to load rules from {}: {}", rules_path, e);
            log::warn!("No validation rules are loaded. All validations will pass.");
        }
    }
    
    let validation_engine = ValidationEngine::new(rule_repository);
    
    // Start the HTTP server
    api::start_server(validation_engine, &host, port).await
}
