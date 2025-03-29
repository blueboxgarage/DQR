use std::env;
use std::fs;
use std::io;
use std::process;

use dqr::api;
use dqr::models::ValidationRequest;
use dqr::rules::RuleRepository;
use dqr::validation::ValidationEngine;

async fn run_validation(validation_engine: ValidationEngine, file_path: &str) -> io::Result<()> {
    // Read file content
    let file_content = fs::read_to_string(file_path)?;
    
    // Parse JSON
    let request: ValidationRequest = match serde_json::from_str(&file_content) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            process::exit(1);
        }
    };
    
    // Run validation
    let result = match validation_engine.validate(&request.data, &request.journey, &request.system) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error during validation: {}", e);
            process::exit(1);
        }
    };
    
    // Output results
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
    
    if !result.valid {
        process::exit(1);
    }
    
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists
    dotenv::dotenv().ok();
    
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Load configuration from environment variables
    let host = env::var("DQR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("DQR_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Invalid port number");
    let rules_path = env::var("DQR_RULES_PATH")
        .unwrap_or_else(|_| "rules/default.csv".to_string());
    
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
    
    // Check if we have a validate command and file path
    if args.len() >= 3 && args[1] == "validate" {
        return run_validation(validation_engine, &args[2]).await;
    }
    
    // If no validate command, start the HTTP server
    api::start_server(validation_engine, &host, port).await
}
