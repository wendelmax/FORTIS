//! APIs de Zero-Knowledge Proofs (ZKP)
//! 
//! Endpoints para geração e verificação de provas ZKP

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;

use crate::zkp::{VotingProofSystem, VoterData, CircuitConfig, NullifierManager};

/// Configura rotas ZKP
pub fn config_zkp_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/zkp")
            .route("/voting/prove", web::post().to(generate_voting_proof))
            .route("/voting/verify", web::post().to(verify_voting_proof))
            .route("/eligibility/prove", web::post().to(generate_eligibility_proof))
            .route("/eligibility/verify", web::post().to(verify_eligibility_proof))
            .route("/nullifier/check", web::post().to(check_nullifier))
            .route("/nullifier/add", web::post().to(add_nullifier))
    );
}

/// Resposta padrão da API
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<Utc>,
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

/// Requisição para gerar prova de votação
#[derive(Debug, Deserialize)]
pub struct GenerateVotingProofRequest {
    pub voter_data: VoterData,
    pub candidate_id: String,
    pub election_id: String,
}

/// Requisição para verificar prova de votação
#[derive(Debug, Deserialize)]
pub struct VerifyVotingProofRequest {
    pub proof_data: String,
    pub public_inputs: crate::zkp::VotingPublicInputs,
}

/// Requisição para gerar prova de elegibilidade
#[derive(Debug, Deserialize)]
pub struct GenerateEligibilityProofRequest {
    pub voter_data: VoterData,
    pub election_id: String,
}

/// Requisição para verificar prova de elegibilidade
#[derive(Debug, Deserialize)]
pub struct VerifyEligibilityProofRequest {
    pub proof_data: String,
    pub public_inputs: crate::zkp::EligibilityPublicInputs,
}

/// Requisição para verificar nullifier
#[derive(Debug, Deserialize)]
pub struct CheckNullifierRequest {
    pub nullifier: String,
}

/// Requisição para adicionar nullifier
#[derive(Debug, Deserialize)]
pub struct AddNullifierRequest {
    pub nullifier: String,
}

/// Gera prova de votação
async fn generate_voting_proof(
    req: web::Json<GenerateVotingProofRequest>,
) -> Result<HttpResponse> {
    let config = CircuitConfig {
        trusted_setup: "trusted_setup".to_string(),
        circuit_size: 1000000,
        max_voters: 1000000,
        max_candidates: 1000,
        security_level: 128,
    };
    
    let proof_system = VotingProofSystem::new(config);
    
    match proof_system.generate_voting_proof(
        &req.voter_data.cpf,
        &req.candidate_id,
        &req.election_id,
    ) {
        Ok(proof) => Ok(HttpResponse::Ok().json(ApiResponse::success(proof))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Verifica prova de votação
async fn verify_voting_proof(
    req: web::Json<VerifyVotingProofRequest>,
) -> Result<HttpResponse> {
    let config = CircuitConfig {
        trusted_setup: "trusted_setup".to_string(),
        circuit_size: 1000000,
        max_voters: 1000000,
        max_candidates: 1000,
        security_level: 128,
    };
    
    let proof_system = VotingProofSystem::new(config);
    
    let proof = crate::zkp::VotingProof {
        proof: req.proof_data.clone(),
        proof_data: req.proof_data.clone(),
        timestamp: Utc::now(),
        public_inputs: req.public_inputs.clone(),
    };
    
    match proof_system.verify_voting_proof(&proof) {
        Ok(is_valid) => {
            let response = HashMap::from([
                ("is_valid", is_valid.to_string()),
                ("proof_data", req.proof_data.clone()),
            ]);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        },
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Gera prova de elegibilidade
async fn generate_eligibility_proof(
    req: web::Json<GenerateEligibilityProofRequest>,
) -> Result<HttpResponse> {
    let config = CircuitConfig {
        trusted_setup: "trusted_setup".to_string(),
        circuit_size: 1000000,
        max_voters: 1000000,
        max_candidates: 1000,
        security_level: 128,
    };
    
    let proof_system = VotingProofSystem::new(config);
    
    match proof_system.generate_eligibility_proof(
        &req.voter_data.cpf,
        &req.election_id,
    ) {
        Ok(proof) => Ok(HttpResponse::Ok().json(ApiResponse::success(proof))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Verifica prova de elegibilidade
async fn verify_eligibility_proof(
    req: web::Json<VerifyEligibilityProofRequest>,
) -> Result<HttpResponse> {
    let config = CircuitConfig {
        trusted_setup: "trusted_setup".to_string(),
        circuit_size: 1000000,
        max_voters: 1000000,
        max_candidates: 1000,
        security_level: 128,
    };
    
    let proof_system = VotingProofSystem::new(config);
    
    let proof = crate::zkp::EligibilityProof {
        proof: req.proof_data.clone(),
        proof_data: req.proof_data.clone(),
        timestamp: Utc::now(),
        public_inputs: req.public_inputs.clone(),
    };
    
    match proof_system.verify_eligibility_proof(&proof) {
        Ok(is_valid) => {
            let response = HashMap::from([
                ("is_valid", is_valid.to_string()),
                ("proof_data", req.proof_data.clone()),
            ]);
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        },
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Verifica se nullifier já foi usado
async fn check_nullifier(
    req: web::Json<CheckNullifierRequest>,
) -> Result<HttpResponse> {
    // Em produção, usar um gerenciador global de nullifiers
    let mut nullifier_manager = NullifierManager::new();
    
    let is_used = nullifier_manager.is_nullifier_used(&req.nullifier);
    
    let response = HashMap::from([
        ("nullifier", req.nullifier.clone()),
        ("is_used", is_used.to_string()),
    ]);
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}

/// Adiciona nullifier
async fn add_nullifier(
    req: web::Json<AddNullifierRequest>,
) -> Result<HttpResponse> {
    // Em produção, usar um gerenciador global de nullifiers
    let mut nullifier_manager = NullifierManager::new();
    
    let was_added = nullifier_manager.add_nullifier(req.nullifier.clone());
    
    let response = HashMap::from([
        ("nullifier", req.nullifier.clone()),
        ("was_added", was_added.to_string()),
    ]);
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
}
