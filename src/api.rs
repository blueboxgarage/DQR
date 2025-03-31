use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use crate::models::{ApiResponse, NewRuleRequest, RuleDisplay, ValidationRequest};
use crate::validation::ValidationEngine;

use std::sync::{Arc, Mutex};

pub struct ApiState {
    validation_engine: Arc<Mutex<ValidationEngine>>,
}

pub async fn validate_json(
    req: web::Json<ValidationRequest>,
    state: web::Data<ApiState>,
) -> impl Responder {
    // Extract journey and system from the request
    let journey = &req.journey;
    let system = &req.system;
    
    log::info!("Validating request for journey: {}, system: {}", journey, system);
    
    // Get the validation engine from the state
    let engine = match state.validation_engine.lock() {
        Ok(engine) => engine,
        Err(_) => {
            log::error!("Failed to acquire lock on validation engine");
            return HttpResponse::InternalServerError().json("Internal server error");
        }
    };
    
    // Log the rules being applied
    let rules = engine.get_rules_for_journey_system(journey, system);
    for rule in &rules {
        log::info!("Rule {} will be applied for {}:{}: [{}] {}", 
                  rule.id, journey, system, rule.selector, rule.condition);
    }
    
    match engine.validate(&req.data, journey, system) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => {
            log::error!("Validation error: {}", err);
            HttpResponse::InternalServerError().json(format!("Internal server error: {}", err))
        }
    }
}

pub async fn health_check(state: web::Data<ApiState>) -> impl Responder {
    log::info!("Health check endpoint called");
    
    // Get the validation engine from the state
    let engine = match state.validation_engine.lock() {
        Ok(engine) => engine,
        Err(_) => {
            log::error!("Failed to acquire lock on validation engine");
            return HttpResponse::InternalServerError().json("Internal server error");
        }
    };
    
    // Get cache statistics
    let validation_cache_size = engine.get_validation_cache_size();
    let journey_system_cache_size = engine.get_journey_system_cache_size();
    
    HttpResponse::Ok().json(serde_json::json!({ 
        "status": "healthy",
        "cache_stats": {
            "validation_cache_size": validation_cache_size,
            "journey_system_cache_size": journey_system_cache_size
        }
    }))
}

// API endpoint to get all rules
pub async fn get_rules(state: web::Data<ApiState>) -> impl Responder {
    log::info!("Get rules endpoint called");
    
    // Get the validation engine from the state
    let engine = match state.validation_engine.lock() {
        Ok(engine) => engine,
        Err(_) => {
            log::error!("Failed to acquire lock on validation engine");
            return HttpResponse::InternalServerError().json(ApiResponse::<Vec<RuleDisplay>> {
                success: false,
                data: None,
                error: Some("Internal server error".to_string()),
            });
        }
    };
    
    let rules = engine.get_rules_for_display();
    
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(rules),
        error: None,
    })
}

// API endpoint to create a new rule
pub async fn create_rule(
    req: web::Json<NewRuleRequest>,
    state: web::Data<ApiState>,
) -> impl Responder {
    log::info!("Create rule endpoint called for {}", req.field_path);
    
    // Get the validation engine from the state
    let mut engine = match state.validation_engine.lock() {
        Ok(engine) => engine,
        Err(_) => {
            log::error!("Failed to acquire lock on validation engine");
            return HttpResponse::InternalServerError().json(ApiResponse::<String> {
                success: false,
                data: None,
                error: Some("Internal server error".to_string()),
            });
        }
    };
    
    match engine.create_rule(&req) {
        Ok(rule_id) => {
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(rule_id),
                error: None,
            })
        }
        Err(err) => {
            log::error!("Error creating rule: {}", err);
            HttpResponse::BadRequest().json(ApiResponse::<String> {
                success: false,
                data: None,
                error: Some(format!("Failed to create rule: {}", err)),
            })
        }
    }
}

// API endpoint to delete a rule
pub async fn delete_rule(
    path: web::Path<String>,
    state: web::Data<ApiState>,
) -> impl Responder {
    let rule_id = path.into_inner();
    log::info!("Delete rule endpoint called for rule ID: {}", rule_id);
    
    // Get the validation engine from the state
    let mut engine = match state.validation_engine.lock() {
        Ok(engine) => engine,
        Err(_) => {
            log::error!("Failed to acquire lock on validation engine");
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Internal server error".to_string()),
            });
        }
    };
    
    match engine.delete_rule(&rule_id) {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                success: true,
                data: None,
                error: None,
            })
        }
        Err(err) => {
            log::error!("Error deleting rule: {}", err);
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some(format!("Failed to delete rule: {}", err)),
            })
        }
    }
}

pub async fn start_server(
    validation_engine: ValidationEngine,
    host: &str,
    port: u16,
) -> std::io::Result<()> {
    log::info!("Starting server at {}:{}", host, port);
    
    // Wrap the validation engine in Arc<Mutex>
    let engine = Arc::new(Mutex::new(validation_engine));
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ApiState {
                validation_engine: engine.clone(),
            }))
            .app_data(web::JsonConfig::default().limit(4096000)) // 4MB limit
            .route("/api/validate", web::post().to(validate_json))
            .route("/api/rules", web::get().to(get_rules))
            .route("/api/rules", web::post().to(create_rule))
            .route("/api/rules/{id}", web::delete().to(delete_rule))
            .route("/health", web::get().to(health_check))
            // Add CORS middleware for frontend
            .wrap(actix_web::middleware::DefaultHeaders::new()
                .add(("Access-Control-Allow-Origin", "*"))
                .add(("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS"))
                .add(("Access-Control-Allow-Headers", "Content-Type"))
            )
    })
    .bind((host, port))?
    .run()
    .await
}