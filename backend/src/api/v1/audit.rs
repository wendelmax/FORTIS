//! APIs de Auditoria
//! 
//! Endpoints para sistema de auditoria imutável e transparente

use actix_web::{web, HttpResponse, Result};
use serde::Serialize;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::services::audit::BlockchainAuditService;
use crate::config::Config;

/// Configura rotas de auditoria
pub fn config_audit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/audit")
            .route("/events", web::get().to(get_audit_events))
            .route("/events", web::post().to(log_audit_event))
            .route("/statistics", web::get().to(get_audit_statistics))
    );
}

/// Resposta padrão da API
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: Utc::now(),
        }
    }
}

/// Obtém eventos de auditoria
async fn get_audit_events(
    config: web::Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let audit_service = BlockchainAuditService::new(config.as_ref().clone());
    
    let start_date = query.get("start_date")
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
    
    let end_date = query.get("end_date")
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|| Utc::now());

    match audit_service.get_audit_events(start_date, end_date, None, None).await {
        Ok(events) => Ok(HttpResponse::Ok().json(ApiResponse::success(events))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Log de evento de auditoria
async fn log_audit_event(
    config: web::Data<Config>,
    req: web::Json<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let audit_service = BlockchainAuditService::new(config.as_ref().clone());
    
    // Criar evento simplificado
    let event = crate::services::audit::blockchain_audit::AuditEvent {
        event_id: uuid::Uuid::new_v4().to_string(),
        event_type: crate::services::audit::blockchain_audit::AuditEventType::AuditCreated,
        timestamp: Utc::now(),
        actor: req.get("actor").unwrap_or(&"system".to_string()).clone(),
        action: req.get("action").unwrap_or(&"LOG_EVENT".to_string()).clone(),
        target: req.get("target").unwrap_or(&"audit_system".to_string()).clone(),
        data: crate::services::audit::blockchain_audit::AuditEventData {
            election_id: req.get("election_id").cloned(),
            voter_id: req.get("voter_id").cloned(),
            node_id: req.get("node_id").cloned(),
            candidate_id: None,
            error_code: None,
            error_message: None,
            metadata: req.clone(),
            previous_hash: None,
        },
        hash: String::new(),
        signature: String::new(),
        block_number: None,
        transaction_hash: None,
    };

    match audit_service.log_audit_event(&event).await {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém estatísticas de auditoria
async fn get_audit_statistics(
    config: web::Data<Config>,
) -> Result<HttpResponse> {
    let audit_service = BlockchainAuditService::new(config.as_ref().clone());
    
    match audit_service.get_audit_statistics().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(ApiResponse::success(stats))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}
