//! APIs de integração TSE
//! 
//! Endpoints para autenticação Gov.br, validação de eleitores e sincronização

use actix_web::{web, HttpResponse, Result as ActixResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::services::tse::{GovBrService, VoterValidationService, DigitalCertificateService, ElectionSyncService};
use crate::config::Config;

/// Configura rotas TSE
pub fn config_tse_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tse")
            .route("/auth/gov-br/url", web::get().to(get_gov_br_auth_url))
            .route("/auth/gov-br/callback", web::post().to(gov_br_callback))
            .route("/auth/gov-br/user", web::get().to(get_gov_br_user))
            .route("/voter/validate/cpf/{cpf}", web::get().to(validate_voter_cpf))
            .route("/voter/validate/id/{voter_id}", web::get().to(validate_voter_id))
            .route("/voter/data/{cpf}", web::get().to(get_voter_data))
            .route("/voter/can-vote/{cpf}/{election_id}", web::get().to(can_vote_in_election))
            .route("/voter/has-voted/{cpf}/{election_id}", web::get().to(has_voted))
            .route("/voter/history/{cpf}", web::get().to(get_vote_history))
            .route("/certificate/validate", web::post().to(validate_certificate))
            .route("/certificate/sign", web::post().to(sign_data))
            .route("/certificate/verify", web::post().to(verify_signature))
            .route("/elections/sync", web::post().to(sync_elections))
            .route("/elections/active", web::get().to(get_active_elections))
            .route("/elections/{election_id}", web::get().to(get_election))
            .route("/elections/{election_id}/candidates", web::get().to(get_election_candidates))
            .route("/elections/{election_id}/zones", web::get().to(get_election_zones))
            .route("/elections/{election_id}/rules", web::get().to(get_election_rules))
            .route("/elections/{election_id}/stats", web::get().to(get_election_stats))
            .route("/votes", web::post().to(send_vote_data))
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

/// Gera URL de autorização Gov.br
async fn get_gov_br_auth_url(
    config: web::Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let default_state = "default".to_string();
    let state = query.get("state").unwrap_or(&default_state);
    
    let gov_br_service = GovBrService::new(&config);
    let auth_url = gov_br_service.get_authorization_url(state);
    
    let response = HashMap::from([
        ("auth_url", auth_url),
        ("state", state.clone()),
    ]);
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Callback de autorização Gov.br
#[derive(Debug, Deserialize)]
pub struct GovBrCallbackRequest {
    pub code: String,
    pub state: String,
}

async fn gov_br_callback(
    config: web::Data<Config>,
    req: web::Json<GovBrCallbackRequest>,
) -> ActixResult<HttpResponse> {
    let gov_br_service = GovBrService::new(&config);
    
    match gov_br_service.exchange_code_for_token(&req.code).await {
        Ok(token) => Ok(HttpResponse::Ok().json(ApiResponse::success(token))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém dados do usuário Gov.br
async fn get_gov_br_user(
    config: web::Data<Config>,
    query: web::Query<HashMap<String, String>>,
) -> ActixResult<HttpResponse> {
    let access_token = match query.get("access_token") {
        Some(token) => token,
        None => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Token de acesso necessário".to_string()))),
    };
    
    let gov_br_service = GovBrService::new(&config);
    
    match gov_br_service.get_user_info(access_token).await {
        Ok(user) => Ok(HttpResponse::Ok().json(ApiResponse::success(user))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Valida eleitor por CPF
async fn validate_voter_cpf(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let cpf = path.into_inner();
    let voter_service = VoterValidationService::new(&config);
    
    match voter_service.validate_voter_by_cpf(&cpf).await {
        Ok(validation) => Ok(HttpResponse::Ok().json(ApiResponse::success(validation))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Valida eleitor por título
async fn validate_voter_id(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let voter_id = path.into_inner();
    let voter_service = VoterValidationService::new(&config);
    
    match voter_service.validate_voter_by_id(&voter_id).await {
        Ok(validation) => Ok(HttpResponse::Ok().json(ApiResponse::success(validation))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém dados completos do eleitor
async fn get_voter_data(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let cpf = path.into_inner();
    let voter_service = VoterValidationService::new(&config);
    
    match voter_service.get_voter_data(&cpf).await {
        Ok(Some(data)) => Ok(HttpResponse::Ok().json(ApiResponse::success(data))),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error("Eleitor não encontrado".to_string()))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Verifica se eleitor pode votar
async fn can_vote_in_election(
    config: web::Data<Config>,
    path: web::Path<(String, String)>,
) -> ActixResult<HttpResponse> {
    let (cpf, election_id) = path.into_inner();
    let voter_service = VoterValidationService::new(&config);
    
    match voter_service.can_vote_in_election(&cpf, &election_id).await {
        Ok(can_vote) => {
            let response = HashMap::from([
                ("can_vote", can_vote.to_string()),
                ("cpf", cpf),
                ("election_id", election_id),
            ]);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        },
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Verifica se eleitor já votou
async fn has_voted(
    config: web::Data<Config>,
    path: web::Path<(String, String)>,
) -> ActixResult<HttpResponse> {
    let (cpf, election_id) = path.into_inner();
    let voter_service = VoterValidationService::new(&config);
    
    match voter_service.has_voted(&cpf, &election_id).await {
        Ok(has_voted) => {
            let response = HashMap::from([
                ("has_voted", has_voted.to_string()),
                ("cpf", cpf),
                ("election_id", election_id),
            ]);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        },
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém histórico de votos
async fn get_vote_history(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let cpf = path.into_inner();
    let voter_service = VoterValidationService::new(&config);
    
    match voter_service.get_vote_history(&cpf).await {
        Ok(history) => Ok(HttpResponse::Ok().json(ApiResponse::success(history))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Valida certificado digital
#[derive(Debug, Deserialize)]
pub struct ValidateCertificateRequest {
    pub certificate_data: String,
}

async fn validate_certificate(
    req: web::Json<ValidateCertificateRequest>,
) -> ActixResult<HttpResponse> {
    let cert_service = DigitalCertificateService::new();
    
    match cert_service.validate_certificate(&req.certificate_data).await {
        Ok(validation) => Ok(HttpResponse::Ok().json(ApiResponse::success(validation))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Assina dados com certificado
#[derive(Debug, Deserialize)]
pub struct SignDataRequest {
    pub data: String,
    pub certificate_data: String,
}

async fn sign_data(
    req: web::Json<SignDataRequest>,
) -> ActixResult<HttpResponse> {
    let cert_service = DigitalCertificateService::new();
    
    // Primeiro validar o certificado
    let validation = match cert_service.validate_certificate(&req.certificate_data).await {
        Ok(validation) => validation,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    };
    
    if !validation.is_valid {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Certificado inválido".to_string())));
    }
    
    let certificate = validation.certificate_info.unwrap();
    
    match cert_service.sign_data(&req.data, &certificate).await {
        Ok(signature) => {
            let response = HashMap::from([
                ("signature", signature),
                ("data", req.data.clone()),
            ]);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        },
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Verifica assinatura
#[derive(Debug, Deserialize)]
pub struct VerifySignatureRequest {
    pub data: String,
    pub signature: String,
    pub certificate_data: String,
}

async fn verify_signature(
    req: web::Json<VerifySignatureRequest>,
) -> ActixResult<HttpResponse> {
    let cert_service = DigitalCertificateService::new();
    
    // Primeiro validar o certificado
    let validation = match cert_service.validate_certificate(&req.certificate_data).await {
        Ok(validation) => validation,
        Err(e) => return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    };
    
    if !validation.is_valid {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error("Certificado inválido".to_string())));
    }
    
    let certificate = validation.certificate_info.unwrap();
    
    match cert_service.verify_signature(&req.data, &req.signature, &certificate).await {
        Ok(is_valid) => {
            let response = HashMap::from([
                ("is_valid", is_valid.to_string()),
                ("data", req.data.clone()),
            ]);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        },
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Sincroniza eleições
async fn sync_elections(
    config: web::Data<Config>,
) -> ActixResult<HttpResponse> {
    let sync_service = ElectionSyncService::new(&config);
    
    match sync_service.sync_all_elections().await {
        Ok(result) => Ok(HttpResponse::Ok().json(ApiResponse::success(result))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém eleições ativas
async fn get_active_elections(
    config: web::Data<Config>,
) -> ActixResult<HttpResponse> {
    let sync_service = ElectionSyncService::new(&config);
    
    match sync_service.get_active_elections().await {
        Ok(elections) => Ok(HttpResponse::Ok().json(ApiResponse::success(elections))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém eleição específica
async fn get_election(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let election_id = path.into_inner();
    let sync_service = ElectionSyncService::new(&config);
    
    match sync_service.sync_election(&election_id).await {
        Ok(election) => Ok(HttpResponse::Ok().json(ApiResponse::success(election))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém candidatos da eleição
async fn get_election_candidates(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let election_id = path.into_inner();
    let sync_service = ElectionSyncService::new(&config);
    
    match sync_service.sync_candidates(&election_id).await {
        Ok(candidates) => Ok(HttpResponse::Ok().json(ApiResponse::success(candidates))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém zonas eleitorais
async fn get_election_zones(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let election_id = path.into_inner();
    let sync_service = ElectionSyncService::new(&config);
    
    match sync_service.sync_voting_zones(&election_id).await {
        Ok(zones) => Ok(HttpResponse::Ok().json(ApiResponse::success(zones))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém regras da eleição
async fn get_election_rules(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let election_id = path.into_inner();
    let sync_service = ElectionSyncService::new(&config);
    
    match sync_service.sync_election_rules(&election_id).await {
        Ok(rules) => Ok(HttpResponse::Ok().json(ApiResponse::success(rules))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Obtém estatísticas da eleição
async fn get_election_stats(
    config: web::Data<Config>,
    path: web::Path<String>,
) -> ActixResult<HttpResponse> {
    let election_id = path.into_inner();
    let sync_service = ElectionSyncService::new(&config);
    
    match sync_service.get_election_stats(&election_id).await {
        Ok(stats) => Ok(HttpResponse::Ok().json(ApiResponse::success(stats))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Envia dados de votação
#[derive(Debug, Deserialize)]
pub struct SendVoteDataRequest {
    pub election_id: String,
    pub voter_cpf: String,
    pub candidate_id: Option<String>,
    pub voting_zone: String,
    pub voting_section: String,
    pub vote_hash: String,
    pub signature: String,
    pub verification_data: HashMap<String, String>,
}

async fn send_vote_data(
    config: web::Data<Config>,
    req: web::Json<SendVoteDataRequest>,
) -> ActixResult<HttpResponse> {
    let sync_service = ElectionSyncService::new(&config);
    
    let vote_data = crate::services::tse::election_sync::VoteData {
        election_id: req.election_id.clone(),
        voter_cpf: req.voter_cpf.clone(),
        candidate_id: req.candidate_id.clone(),
        vote_timestamp: Utc::now(),
        voting_zone: req.voting_zone.clone(),
        voting_section: req.voting_section.clone(),
        vote_hash: req.vote_hash.clone(),
        signature: req.signature.clone(),
        verification_data: req.verification_data.clone(),
    };
    
    match sync_service.send_vote_data(&vote_data).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("Voto enviado com sucesso"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}
