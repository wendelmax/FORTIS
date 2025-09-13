//! APIs para comunicação com urnas eletrônicas

use actix_web::{web, HttpResponse, Result, HttpRequest};
use crate::models::{
    UrnaVoteRequest, UrnaVoteResponse, UrnaSyncRequest, UrnaSyncResponse,
    UrnaStatusRequest, UrnaStatusResponse, Urna, UrnaHealthCheck, UrnaStatus,
    PerformanceMetrics, VoteReceipt, VoteSyncStatus, ApiResponse
};
use crate::services::{urna::{UrnaAuthService, UrnaSyncService}, vote::VoteService};
use anyhow::Result as AnyResult;
use uuid::Uuid;
use chrono::Utc;

/// Configurar rotas de urnas
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/vote", web::post().to(cast_urna_vote))
        .route("/sync", web::post().to(start_urna_sync))
        .route("/sync/{sync_id}", web::get().to(get_sync_status))
        .route("/status/{urna_id}", web::get().to(get_urna_status))
        .route("/health/{urna_id}", web::get().to(get_urna_health))
        .route("/register", web::post().to(register_urna))
        .route("/{urna_id}/votes", web::get().to(get_urna_votes))
        .route("/{urna_id}/audit", web::get().to(get_urna_audit_logs));
}

/// Registrar voto na urna
async fn cast_urna_vote(
    req: web::Json<UrnaVoteRequest>,
    auth_service: web::Data<UrnaAuthService>,
    sync_service: web::Data<UrnaSyncService>,
    vote_service: web::Data<VoteService>,
) -> Result<HttpResponse> {
    let vote_request = req.into_inner();
    
    // Autenticar eleitor
    let auth_result = auth_service
        .authenticate_voter(
            &Urna { id: vote_request.urna_id, serial_number: "".to_string(), model: "".to_string(), 
                   location: crate::models::UrnaLocation { state: "".to_string(), city: "".to_string(), 
                   zone: "".to_string(), section: "".to_string(), address: "".to_string(), coordinates: None },
                   status: UrnaStatus::Active, last_sync: None, created_at: Utc::now(), updated_at: Utc::now() },
            &vote_request.biometric_data,
            vote_request.certificate_data.as_ref(),
        )
        .await;

    let auth = match auth_result {
        Ok(auth) => auth,
        Err(e) => {
            return Ok(HttpResponse::Unauthorized().json(
                ApiResponse::<()>::error(format!("Falha na autenticação: {}", e))
            ));
        }
    };

    // Verificar elegibilidade
    let is_eligible = auth_service
        .check_voter_eligibility(auth.voter_id, vote_request.election_id)
        .await
        .unwrap_or(false);

    if !is_eligible {
        return Ok(HttpResponse::Forbidden().json(
            ApiResponse::<()>::error("Eleitor não elegível para esta eleição".to_string())
        ));
    }

    // Verificar se já votou
    let already_voted = auth_service
        .check_already_voted(auth.voter_id, vote_request.election_id)
        .await
        .unwrap_or(false);

    if already_voted {
        return Ok(HttpResponse::Conflict().json(
            ApiResponse::<()>::error("Eleitor já votou nesta eleição".to_string())
        ));
    }

    // Processar voto
    let vote_id = Uuid::new_v4();
    let vote_result = vote_service.cast_vote(&crate::models::VoteRequest {
        election_id: vote_request.election_id,
        candidate_id: vote_request.candidate_id,
        proof: vote_request.vote_proof,
    }).await;

    match vote_result {
        Ok(_) => {
            // Criar comprovante
            let receipt = VoteReceipt {
                vote_id,
                election_id: vote_request.election_id,
                candidate_number: 123, // Em implementação real, buscaria do banco
                candidate_name: "Candidato Exemplo".to_string(),
                timestamp: Utc::now(),
                qr_code: format!("QR_CODE_{}", vote_id),
                blockchain_hash: None,
            };

            let response = UrnaVoteResponse {
                vote_id,
                success: true,
                message: "Voto registrado com sucesso".to_string(),
                receipt: Some(receipt),
                sync_status: VoteSyncStatus::Pending,
            };

            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error(format!("Erro ao processar voto: {}", e))
            ))
        }
    }
}

/// Iniciar sincronização da urna
async fn start_urna_sync(
    req: web::Json<UrnaSyncRequest>,
    sync_service: web::Data<UrnaSyncService>,
) -> Result<HttpResponse> {
    let sync_request = req.into_inner();
    
    match sync_service.start_sync(sync_request).await {
        Ok(response) => Ok(HttpResponse::Ok().json(ApiResponse::success(response))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(format!("Erro ao iniciar sincronização: {}", e))
        )),
    }
}

/// Obter status da sincronização
async fn get_sync_status(
    path: web::Path<Uuid>,
    sync_service: web::Data<UrnaSyncService>,
) -> Result<HttpResponse> {
    let sync_id = path.into_inner();
    
    match sync_service.get_sync_status(sync_id).await {
        Ok(Some(sync)) => Ok(HttpResponse::Ok().json(ApiResponse::success(sync))),
        Ok(None) => Ok(HttpResponse::NotFound().json(
            ApiResponse::<()>::error("Sincronização não encontrada".to_string())
        )),
        Err(e) => Ok(HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(format!("Erro ao obter status: {}", e))
        )),
    }
}

/// Obter status da urna
async fn get_urna_status(
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let urna_id = path.into_inner();
    
    // Simulação de dados da urna
    let urna = Urna {
        id: urna_id,
        serial_number: "URNA001".to_string(),
        model: "FORTIS-2025".to_string(),
        location: crate::models::UrnaLocation {
            state: "SP".to_string(),
            city: "São Paulo".to_string(),
            zone: "001".to_string(),
            section: "001".to_string(),
            address: "Rua das Flores, 123".to_string(),
            coordinates: None,
        },
        status: UrnaStatus::Active,
        last_sync: Some(Utc::now()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let health = UrnaHealthCheck {
        urna_id,
        timestamp: Utc::now(),
        status: UrnaStatus::Active,
        battery_level: Some(85.0),
        storage_usage: Some(45.0),
        network_connectivity: true,
        last_sync: Some(Utc::now()),
        errors: Vec::new(),
        performance_metrics: PerformanceMetrics {
            cpu_usage: 25.0,
            memory_usage: 60.0,
            disk_usage: 45.0,
            network_latency: Some(50),
            response_time: 100,
        },
    };

    let response = UrnaStatusResponse {
        urna,
        health,
        pending_votes: 0,
        last_activity: Some(Utc::now()),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Obter saúde da urna
async fn get_urna_health(
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let urna_id = path.into_inner();
    
    let health = UrnaHealthCheck {
        urna_id,
        timestamp: Utc::now(),
        status: UrnaStatus::Active,
        battery_level: Some(85.0),
        storage_usage: Some(45.0),
        network_connectivity: true,
        last_sync: Some(Utc::now()),
        errors: Vec::new(),
        performance_metrics: PerformanceMetrics {
            cpu_usage: 25.0,
            memory_usage: 60.0,
            disk_usage: 45.0,
            network_latency: Some(50),
            response_time: 100,
        },
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(health)))
}

/// Registrar nova urna
async fn register_urna(
    req: web::Json<Urna>,
) -> Result<HttpResponse> {
    let urna = req.into_inner();
    
    // Em implementação real, salvaria no banco de dados
    log::info!("Urna registrada: {}", urna.serial_number);
    
    Ok(HttpResponse::Created().json(ApiResponse::success(urna)))
}

/// Obter votos da urna
async fn get_urna_votes(
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let urna_id = path.into_inner();
    
    // Em implementação real, buscaria votos no banco de dados
    let votes = serde_json::json!({
        "urna_id": urna_id,
        "votes": [],
        "total": 0
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(votes)))
}

/// Obter logs de auditoria da urna
async fn get_urna_audit_logs(
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let urna_id = path.into_inner();
    
    // Em implementação real, buscaria logs no banco de dados
    let logs = serde_json::json!({
        "urna_id": urna_id,
        "logs": [],
        "total": 0
    });

    Ok(HttpResponse::Ok().json(ApiResponse::success(logs)))
}
