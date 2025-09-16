//! FORTIS Backend - Sistema de Votação Eletrônica Brasileiro
//! 
//! Este é o servidor principal do FORTIS, implementado em Rust para máxima
//! performance e segurança.

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use std::env;
use utoipa_swagger_ui::SwaggerUi;

mod auth;
mod blockchain;
mod crypto;
mod database;
mod models;
mod services;
mod utils;
mod api;
mod zkp;
mod validation;
mod audit;
mod storage;
mod monitoring;
// mod middleware;
mod config;
mod api_docs;

use config::Config;
use api_docs::ApiDoc;
use utoipa::OpenApi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializar logging
    env_logger::init();
    
    // Carregar configurações
    let config = Config::new();
    
    log::info!("🚀 Iniciando FORTIS Backend v{}", env!("CARGO_PKG_VERSION"));
    log::info!("🌐 Servidor rodando em: http://{}:{}", config.server.host, config.server.port);
    
    // Inicializar banco de dados
    // database::init(&config.database).await
    //     .expect("Failed to initialize database");
    
    // Inicializar Redis
    let redis_client = redis::Client::open(config.redis.url.as_str())
        .expect("Failed to create Redis client");
    
    // Inicializar blockchain
    let blockchain_service = blockchain::BlockchainService::new(config.blockchain.clone());
    blockchain_service.init().await
        .expect("Failed to initialize blockchain");
    
    // Inicializar serviços
    let crypto_service = crypto::CryptoService::new(&config.security.encryption_key)
        .expect("Failed to initialize crypto service");
    
    let jwt_service = auth::jwt::JwtService::new(
        &config.security.jwt_secret,
        "fortis-voting-system",
        "fortis-voters",
    );
    
    // Salvar configurações para uso posterior
    let server_host = config.server.host.clone();
    let server_port = config.server.port;
    
    // Configurar e iniciar servidor HTTP
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(crypto_service.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .service(
                web::scope("/api/v1")
                    .configure(api::v1::configure)
            )
            .service(
                web::scope("/health")
                    .route("", web::get().to(health_check))
                    .route("/ready", web::get().to(ready_check))
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Serviço saudável", body = serde_json::Value)
    ),
    tag = "Health"
)]
async fn health_check() -> actix_web::Result<actix_web::HttpResponse> {
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "fortis-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    })))
}

/// Ready check endpoint
#[utoipa::path(
    get,
    path = "/health/ready",
    responses(
        (status = 200, description = "Serviço pronto", body = serde_json::Value)
    ),
    tag = "Health"
)]
async fn ready_check() -> actix_web::Result<actix_web::HttpResponse> {
    // TODO: Verificar se todos os serviços estão prontos
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "ready",
        "service": "fortis-backend",
        "checks": {
            "database": "ok",
            "redis": "ok",
            "blockchain": "ok"
        }
    })))
}