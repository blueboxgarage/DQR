use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use crate::models::ValidationRequest;
use crate::validation::ValidationEngine;

pub struct ApiState {
    validation_engine: ValidationEngine,
}

pub async fn validate_json(
    req: web::Json<ValidationRequest>,
    state: web::Data<ApiState>,
) -> impl Responder {
    match state.validation_engine.validate(&req.data) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => {
            log::error!("Validation error: {}", err);
            HttpResponse::InternalServerError().json(format!("Internal server error: {}", err))
        }
    }
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

pub async fn start_server(
    validation_engine: ValidationEngine,
    host: &str,
    port: u16,
) -> std::io::Result<()> {
    log::info!("Starting server at {}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ApiState {
                validation_engine: validation_engine.clone(),
            }))
            .app_data(web::JsonConfig::default().limit(4096000)) // 4MB limit
            .route("/api/validate", web::post().to(validate_json))
            .route("/health", web::get().to(health_check))
    })
    .bind((host, port))?
    .run()
    .await
}