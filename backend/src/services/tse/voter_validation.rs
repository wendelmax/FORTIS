//! Serviço de validação de eleitores
//! 
//! Implementa validação completa de eleitores através do TSE

use crate::config::Config;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Serviço de validação de eleitores
pub struct VoterValidationService {
    client: Client,
    tse_base_url: String,
    api_key: String,
}

/// Dados completos do eleitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterData {
    pub cpf: String,
    pub voter_id: String,
    pub name: String,
    pub birth_date: DateTime<Utc>,
    pub voting_zone: String,
    pub voting_section: String,
    pub city: String,
    pub state: String,
    pub status: VoterStatus,
    pub last_vote: Option<DateTime<Utc>>,
    pub biometric_data: Option<BiometricData>,
    pub digital_certificate: Option<String>,
}

/// Dados biométricos do eleitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    pub fingerprint_hash: String,
    pub face_hash: String,
    pub voice_hash: Option<String>,
    pub last_update: DateTime<Utc>,
}

/// Status do eleitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoterStatus {
    #[serde(rename = "ATIVO")]
    Ativo,
    #[serde(rename = "SUSPENSO")]
    Suspenso,
    #[serde(rename = "CANCELADO")]
    Cancelado,
    #[serde(rename = "PENDENTE")]
    Pendente,
    #[serde(rename = "FALECIDO")]
    Falecido,
}

/// Resposta de validação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub voter_data: Option<VoterData>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub validation_timestamp: DateTime<Utc>,
}

/// Dados de eleição ativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveElection {
    pub election_id: String,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: ElectionStatus,
    pub candidates: Vec<Candidate>,
    pub voting_zones: Vec<String>,
}

/// Status da eleição
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElectionStatus {
    #[serde(rename = "AGENDADA")]
    Agendada,
    #[serde(rename = "EM_ANDAMENTO")]
    EmAndamento,
    #[serde(rename = "FINALIZADA")]
    Finalizada,
    #[serde(rename = "CANCELADA")]
    Cancelada,
}

/// Candidato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub party: String,
    pub number: String,
    pub position: String,
    pub photo_url: Option<String>,
}

impl VoterValidationService {
    /// Cria nova instância do serviço
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            tse_base_url: config.tse.base_url.clone(),
            api_key: config.tse.api_key.clone(),
        }
    }

    /// Valida se um CPF é um eleitor ativo
    pub async fn validate_voter_by_cpf(&self, cpf: &str) -> Result<ValidationResponse> {
        let response = self.client
            .get(&format!("{}/api/v1/voter/validate/cpf/{}", self.tse_base_url, cpf))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Ok(ValidationResponse {
                valid: false,
                voter_data: None,
                error_code: Some("TSE_API_ERROR".to_string()),
                error_message: Some(error_text),
                validation_timestamp: Utc::now(),
            });
        }

        let validation: ValidationResponse = response.json().await?;
        Ok(validation)
    }

    /// Valida eleitor por título de eleitor
    pub async fn validate_voter_by_id(&self, voter_id: &str) -> Result<ValidationResponse> {
        let response = self.client
            .get(&format!("{}/api/v1/voter/validate/id/{}", self.tse_base_url, voter_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Ok(ValidationResponse {
                valid: false,
                voter_data: None,
                error_code: Some("TSE_API_ERROR".to_string()),
                error_message: Some(error_text),
                validation_timestamp: Utc::now(),
            });
        }

        let validation: ValidationResponse = response.json().await?;
        Ok(validation)
    }

    /// Obtém dados completos do eleitor
    pub async fn get_voter_data(&self, cpf: &str) -> Result<Option<VoterData>> {
        let response = self.client
            .get(&format!("{}/api/v1/voter/data/{}", self.tse_base_url, cpf))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let voter_data: VoterData = response.json().await?;
        Ok(Some(voter_data))
    }

    /// Verifica se o eleitor pode votar em uma eleição específica
    pub async fn can_vote_in_election(&self, cpf: &str, election_id: &str) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/api/v1/voter/{}/can-vote/{}", self.tse_base_url, cpf, election_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(false);
        }

        let result: HashMap<String, bool> = response.json().await?;
        Ok(*result.get("can_vote").unwrap_or(&false))
    }

    /// Obtém eleições ativas
    pub async fn get_active_elections(&self) -> Result<Vec<ActiveElection>> {
        let response = self.client
            .get(&format!("{}/api/v1/elections/active", self.tse_base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Erro ao obter eleições ativas: {}", error_text));
        }

        let elections: Vec<ActiveElection> = response.json().await?;
        Ok(elections)
    }

    /// Registra tentativa de voto
    pub async fn register_vote_attempt(&self, cpf: &str, election_id: &str, success: bool) -> Result<()> {
        let mut params = HashMap::new();
        let success_str = success.to_string();
        let timestamp_str = Utc::now().to_rfc3339();
        params.insert("cpf", cpf);
        params.insert("election_id", election_id);
        params.insert("success", &success_str);
        params.insert("timestamp", &timestamp_str);

        let response = self.client
            .post(&format!("{}/api/v1/vote-attempt", self.tse_base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Erro ao registrar tentativa de voto: {}", error_text));
        }

        Ok(())
    }

    /// Verifica se o eleitor já votou em uma eleição
    pub async fn has_voted(&self, cpf: &str, election_id: &str) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/api/v1/voter/{}/has-voted/{}", self.tse_base_url, cpf, election_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(false);
        }

        let result: HashMap<String, bool> = response.json().await?;
        Ok(*result.get("has_voted").unwrap_or(&false))
    }

    /// Obtém histórico de votos do eleitor
    pub async fn get_vote_history(&self, cpf: &str) -> Result<Vec<VoteRecord>> {
        let response = self.client
            .get(&format!("{}/api/v1/voter/{}/vote-history", self.tse_base_url, cpf))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let history: Vec<VoteRecord> = response.json().await?;
        Ok(history)
    }
}

/// Registro de voto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord {
    pub election_id: String,
    pub election_name: String,
    pub vote_date: DateTime<Utc>,
    pub voting_zone: String,
    pub voting_section: String,
    pub candidate_id: Option<String>,
    pub candidate_name: Option<String>,
    pub position: String,
}
