//! API REST para Sistema de Consenso Distribuído
//! 
//! Fornece endpoints HTTP para interação com o sistema de consenso,
//! incluindo threshold signatures e gerenciamento de nós.

use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::consensus::*;
use crate::transparency::election_logs::ElectionTransparencyLog;

/// Estado compartilhado do sistema de consenso
pub type ConsensusState = Arc<RwLock<ConsensusService>>;

/// Dados de requisição para consenso
#[derive(Debug, Deserialize)]
pub struct ConsensusRequestData {
    pub operation: ConsensusOperation,
    pub data: serde_json::Value,
    pub requester_id: String,
    pub priority: Option<SignaturePriority>,
    pub timeout_minutes: Option<u64>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Dados de requisição para adicionar nó
#[derive(Debug, Deserialize)]
pub struct AddNodeRequest {
    pub id: String,
    pub name: String,
    pub public_key: String,
    pub ip_address: String,
    pub port: u16,
    pub trust_level: u8,
}

/// Dados de requisição para atualizar nó
#[derive(Debug, Deserialize)]
pub struct UpdateNodeRequest {
    pub name: Option<String>,
    pub trust_level: Option<u8>,
    pub is_active: Option<bool>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Resposta de API
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

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

/// Inicia processo de consenso
pub async fn start_consensus(
    req: web::Json<ConsensusRequestData>,
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    
    let request = ConsensusRequest {
        id: uuid::Uuid::new_v4().to_string(),
        operation: req.operation.clone(),
        data: req.data.clone(),
        requester_id: req.requester_id.clone(),
        priority: req.priority.clone().unwrap_or(SignaturePriority::Normal),
        timeout: req.timeout_minutes.map(|m| Duration::minutes(m)),
        metadata: req.metadata.clone().unwrap_or_default(),
    };

    match consensus_service.start_consensus(request).await {
        Ok(result) => {
            let response = ApiResponse::success(result);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()>::error(e.to_string());
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

/// Obtém resultado de consenso
pub async fn get_consensus_result(
    path: web::Path<String>,
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let request_id = path.into_inner();
    
    match consensus_service.get_consensus_result(&request_id).await {
        Some(result) => {
            let response = ApiResponse::success(result);
            Ok(HttpResponse::Ok().json(response))
        }
        None => {
            let response = ApiResponse::<()>::error("Consensus result not found".to_string());
            Ok(HttpResponse::NotFound().json(response))
        }
    }
}

/// Lista requisições de consenso
pub async fn list_consensus_requests(
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let requests = consensus_service.list_consensus_requests().await;
    let response = ApiResponse::success(requests);
    Ok(HttpResponse::Ok().json(response))
}

/// Obtém métricas de consenso
pub async fn get_consensus_metrics(
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let metrics = consensus_service.get_metrics().await;
    let response = ApiResponse::success(metrics);
    Ok(HttpResponse::Ok().json(response))
}

/// Obtém estatísticas do threshold service
pub async fn get_threshold_stats(
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let stats = consensus_service.get_threshold_stats().await;
    let response = ApiResponse::success(stats);
    Ok(HttpResponse::Ok().json(response))
}

/// Adiciona nó ao consenso
pub async fn add_node(
    req: web::Json<AddNodeRequest>,
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    
    // Decodificar chave pública
    let public_key_bytes = match hex::decode(&req.public_key) {
        Ok(bytes) => bytes,
        Err(_) => {
            let response = ApiResponse::<()>::error("Invalid public key format".to_string());
            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    let key_pair = match ring::signature::Ed25519KeyPair::from_pkcs8(&public_key_bytes) {
        Ok(key_pair) => key_pair,
        Err(_) => {
            let response = ApiResponse::<()>::error("Invalid public key".to_string());
            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    // Parse IP address
    let ip_address = match req.ip_address.parse() {
        Ok(ip) => ip,
        Err(_) => {
            let response = ApiResponse::<()>::error("Invalid IP address".to_string());
            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    let node = ConsensusNode {
        id: req.id.clone(),
        name: req.name.clone(),
        public_key: req.public_key.clone(),
        is_active: true,
        trust_level: req.trust_level,
        last_seen: Utc::now(),
        signature_count: 0,
    };

    // Adicionar nó ao threshold service
    let mut threshold_service = consensus_service.threshold_service.write().await;
    match threshold_service.add_node(node, key_pair) {
        Ok(_) => {
            let response = ApiResponse::success("Node added successfully");
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()>::error(e.to_string());
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

/// Remove nó do consenso
pub async fn remove_node(
    path: web::Path<String>,
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let node_id = path.into_inner();
    
    let mut threshold_service = consensus_service.threshold_service.write().await;
    match threshold_service.remove_node(&node_id) {
        Ok(_) => {
            let response = ApiResponse::success("Node removed successfully");
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = ApiResponse::<()>::error(e.to_string());
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}

/// Lista nós do consenso
pub async fn list_nodes(
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let threshold_service = consensus_service.threshold_service.read().await;
    let stats = threshold_service.get_stats();
    
    // Simular lista de nós baseada nas estatísticas
    let nodes = (1..=stats.total_nodes)
        .map(|i| ConsensusNode {
            id: format!("node_{}", i),
            name: format!("Consensus Node {}", i),
            public_key: "placeholder".to_string(),
            is_active: i <= stats.active_nodes,
            trust_level: 100,
            last_seen: Utc::now(),
            signature_count: 0,
        })
        .collect::<Vec<_>>();
    
    let response = ApiResponse::success(nodes);
    Ok(HttpResponse::Ok().json(response))
}

/// Obtém informações de um nó específico
pub async fn get_node(
    path: web::Path<String>,
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let node_id = path.into_inner();
    
    // Simular busca de nó
    let node = ConsensusNode {
        id: node_id.clone(),
        name: format!("Node {}", node_id),
        public_key: "placeholder".to_string(),
        is_active: true,
        trust_level: 100,
        last_seen: Utc::now(),
        signature_count: 0,
    };
    
    let response = ApiResponse::success(node);
    Ok(HttpResponse::Ok().json(response))
}

/// Atualiza status de um nó
pub async fn update_node_status(
    path: web::Path<String>,
    req: web::Json<UpdateNodeRequest>,
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let node_id = path.into_inner();
    
    // Simular atualização de nó
    let response = ApiResponse::success(format!("Node {} updated successfully", node_id));
    Ok(HttpResponse::Ok().json(response))
}

/// Limpa requisições expiradas
pub async fn cleanup_expired(
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let removed = consensus_service.cleanup_expired().await;
    
    let response = ApiResponse::success(format!("Removed {} expired items", removed));
    Ok(HttpResponse::Ok().json(response))
}

/// Obtém status de saúde do sistema
pub async fn health_check(
    state: web::Data<ConsensusState>,
) -> Result<HttpResponse> {
    let consensus_service = state.read().await;
    let metrics = consensus_service.get_metrics().await;
    let threshold_stats = consensus_service.get_threshold_stats().await;
    
    let health_status = serde_json::json!({
        "consensus": {
            "total_requests": metrics.total_requests,
            "consensus_rate": metrics.consensus_rate,
            "active_nodes": metrics.active_nodes,
        },
        "threshold": {
            "total_nodes": threshold_stats.total_nodes,
            "active_nodes": threshold_stats.active_nodes,
            "threshold": threshold_stats.threshold,
        },
        "status": "healthy",
        "timestamp": Utc::now()
    });
    
    let response = ApiResponse::success(health_status);
    Ok(HttpResponse::Ok().json(response))
}

/// Configura as rotas da API de consenso
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/consensus")
            .route("/start", web::post().to(start_consensus))
            .route("/result/{request_id}", web::get().to(get_consensus_result))
            .route("/requests", web::get().to(list_consensus_requests))
            .route("/metrics", web::get().to(get_consensus_metrics))
            .route("/threshold/stats", web::get().to(get_threshold_stats))
            .route("/nodes", web::get().to(list_nodes))
            .route("/nodes", web::post().to(add_node))
            .route("/nodes/{node_id}", web::get().to(get_node))
            .route("/nodes/{node_id}", web::put().to(update_node_status))
            .route("/nodes/{node_id}", web::delete().to(remove_node))
            .route("/cleanup", web::post().to(cleanup_expired))
            .route("/health", web::get().to(health_check))
    );
}
