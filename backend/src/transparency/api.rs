//! API REST para Logs Transparentes
//! 
//! Endpoints para interação com o sistema de logs transparentes,
//! seguindo rigorosamente a crítica do Prof. Marcos Simplicio de que
//! blockchain não é necessário para transparência eleitoral.

use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::transparency::election_logs::{
    ElectionTransparencyLog, ElectionEvent, ElectionEventType, 
    LogConfig, LogStats, DetailedLogStats, SearchCriteria,
    InclusionProof, ExportFormat, ConfigValidationResult
};

/// Estado compartilhado do sistema de logs
pub type LogState = Arc<RwLock<ElectionTransparencyLog>>;

/// Dados de criação de evento
#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub event_type: ElectionEventType,
    pub election_id: String,
    pub data: serde_json::Value,
    pub source: String,
}

/// Dados de busca de eventos
#[derive(Debug, Deserialize)]
pub struct SearchEventsRequest {
    pub event_type: Option<ElectionEventType>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub election_id: Option<String>,
    pub verification_status: Option<String>,
}

/// Dados de configuração do log
#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfigRequest {
    pub min_verifiers: usize,
    pub max_verifiers: usize,
    pub signature_threshold: usize,
    pub retention_days: u64,
    pub enable_audit_trail: bool,
    pub enable_performance_metrics: bool,
    pub max_entries_per_batch: usize,
    pub verification_timeout_seconds: u64,
}

/// Resposta de criação de evento
#[derive(Debug, Serialize)]
pub struct CreateEventResponse {
    pub success: bool,
    pub inclusion_proof: Option<InclusionProof>,
    pub message: String,
}

/// Resposta de busca de eventos
#[derive(Debug, Serialize)]
pub struct SearchEventsResponse {
    pub success: bool,
    pub events: Vec<serde_json::Value>,
    pub total_count: usize,
    pub message: String,
}

/// Resposta de estatísticas
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub stats: Option<DetailedLogStats>,
    pub message: String,
}

/// Resposta de configuração
#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub success: bool,
    pub config: Option<LogConfig>,
    pub validation: Option<ConfigValidationResult>,
    pub message: String,
}

/// Cria um novo evento eleitoral no log transparente
pub async fn create_event(
    req: web::Json<CreateEventRequest>,
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let mut log = log_state.write().await;
    
    let event = ElectionEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type: req.event_type.clone(),
        election_id: req.election_id.clone(),
        data: req.data.clone(),
        timestamp: Utc::now(),
        source: req.source.clone(),
    };

    match log.append_election_event(event) {
        Ok(inclusion_proof) => {
            Ok(HttpResponse::Ok().json(CreateEventResponse {
                success: true,
                inclusion_proof: Some(inclusion_proof),
                message: "Event created successfully".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(CreateEventResponse {
                success: false,
                inclusion_proof: None,
                message: format!("Failed to create event: {}", e),
            }))
        }
    }
}

/// Busca eventos no log transparente
pub async fn search_events(
    req: web::Json<SearchEventsRequest>,
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    
    let criteria = SearchCriteria {
        event_type: req.event_type.clone(),
        start_time: req.start_time,
        end_time: req.end_time,
        election_id: req.election_id.clone(),
        verification_status: None, // Seria necessário implementar conversão
    };

    match log.search_events(criteria) {
        Ok(events) => {
            let event_data: Vec<serde_json::Value> = events.iter()
                .map(|entry| serde_json::json!({
                    "index": entry.index,
                    "timestamp": entry.timestamp,
                    "event_type": entry.event_type,
                    "event_hash": entry.event_hash,
                    "verification_status": if entry.verifier_signatures.len() >= 2 { "verified" } else { "pending" },
                    "verifier_count": entry.verifier_signatures.len()
                }))
                .collect();

            Ok(HttpResponse::Ok().json(SearchEventsResponse {
                success: true,
                events: event_data,
                total_count: events.len(),
                message: "Search completed successfully".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(SearchEventsResponse {
                success: false,
                events: vec![],
                total_count: 0,
                message: format!("Search failed: {}", e),
            }))
        }
    }
}

/// Obtém estatísticas do log transparente
pub async fn get_stats(
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    let stats = log.get_detailed_stats();

    Ok(HttpResponse::Ok().json(StatsResponse {
        success: true,
        stats: Some(stats),
        message: "Stats retrieved successfully".to_string(),
    }))
}

/// Obtém configuração do log transparente
pub async fn get_config(
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    let config = log.config.clone();
    let validation = log.validate_config().ok();

    Ok(HttpResponse::Ok().json(ConfigResponse {
        success: true,
        config: Some(config),
        validation,
        message: "Config retrieved successfully".to_string(),
    }))
}

/// Atualiza configuração do log transparente
pub async fn update_config(
    req: web::Json<LogConfigRequest>,
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let mut log = log_state.write().await;
    
    let new_config = LogConfig {
        min_verifiers: req.min_verifiers,
        max_verifiers: req.max_verifiers,
        signature_threshold: req.signature_threshold,
        retention_days: req.retention_days,
        enable_audit_trail: req.enable_audit_trail,
        enable_performance_metrics: req.enable_performance_metrics,
        max_entries_per_batch: req.max_entries_per_batch,
        verification_timeout_seconds: req.verification_timeout_seconds,
    };

    match log.update_config(new_config) {
        Ok(_) => {
            Ok(HttpResponse::Ok().json(ConfigResponse {
                success: true,
                config: Some(log.config.clone()),
                validation: log.validate_config().ok(),
                message: "Config updated successfully".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(ConfigResponse {
                success: false,
                config: None,
                validation: None,
                message: format!("Failed to update config: {}", e),
            }))
        }
    }
}

/// Obtém entrada específica do log
pub async fn get_log_entry(
    path: web::Path<u64>,
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    
    match log.get_log_entry(path.into_inner()) {
        Some(entry) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "entry": {
                    "index": entry.index,
                    "timestamp": entry.timestamp,
                    "event_type": entry.event_type,
                    "event_hash": entry.event_hash,
                    "verifier_signatures": entry.verifier_signatures,
                    "merkle_proof": entry.merkle_proof
                },
                "message": "Entry retrieved successfully"
            })))
        }
        None => {
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "entry": null,
                "message": "Entry not found"
            })))
        }
    }
}

/// Exporta log para auditoria
pub async fn export_log(
    query: web::Query<std::collections::HashMap<String, String>>,
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    
    let format = match query.get("format").map(|s| s.as_str()) {
        Some("csv") => ExportFormat::Csv,
        _ => ExportFormat::Json,
    };

    match log.export_for_audit(format) {
        Ok(data) => {
            let content_type = match format {
                ExportFormat::Json => "application/json",
                ExportFormat::Csv => "text/csv",
            };

            Ok(HttpResponse::Ok()
                .content_type(content_type)
                .body(data))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "message": format!("Export failed: {}", e)
            })))
        }
    }
}

/// Verifica integridade do log
pub async fn verify_integrity(
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    
    match log.verify_log_integrity() {
        Ok(report) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "integrity_report": report,
                "message": "Integrity verification completed"
            })))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "message": format!("Integrity verification failed: {}", e)
            })))
        }
    }
}

/// Limpa logs antigos
pub async fn cleanup_logs(
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let mut log = log_state.write().await;
    
    match log.cleanup_old_logs() {
        Ok(removed_count) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "removed_count": removed_count,
                "message": format!("Cleaned up {} old log entries", removed_count)
            })))
        }
        Err(e) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "message": format!("Cleanup failed: {}", e)
            })))
        }
    }
}

/// Obtém trilha de auditoria
pub async fn get_audit_trail(
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    let audit_trail = log.get_audit_trail();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "audit_trail": audit_trail,
        "message": "Audit trail retrieved successfully"
    })))
}

/// Obtém métricas de performance
pub async fn get_performance_metrics(
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    let metrics = log.get_performance_metrics();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "metrics": metrics,
        "message": "Performance metrics retrieved successfully"
    })))
}

/// Health check do sistema de logs
pub async fn health_check(
    log_state: web::Data<LogState>,
) -> Result<HttpResponse> {
    let log = log_state.read().await;
    let stats = log.get_log_stats();
    let validation = log.validate_config().unwrap_or_else(|_| ConfigValidationResult {
        is_valid: false,
        severity: "error".to_string(),
        issues: vec!["Config validation failed".to_string()],
        warnings: vec![],
    });

    let is_healthy = validation.is_valid && stats.total_verifiers > 0;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "healthy": is_healthy,
        "stats": stats,
        "validation": validation,
        "message": if is_healthy { "System is healthy" } else { "System has issues" }
    })))
}

/// Configura as rotas da API de logs transparentes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/v1/transparency")
                .route("/events", web::post().to(create_event))
                .route("/events/search", web::post().to(search_events))
                .route("/events/{index}", web::get().to(get_log_entry))
                .route("/stats", web::get().to(get_stats))
                .route("/config", web::get().to(get_config))
                .route("/config", web::put().to(update_config))
                .route("/export", web::get().to(export_log))
                .route("/verify", web::post().to(verify_integrity))
                .route("/cleanup", web::post().to(cleanup_logs))
                .route("/audit", web::get().to(get_audit_trail))
                .route("/metrics", web::get().to(get_performance_metrics))
                .route("/health", web::get().to(health_check))
        );
}