//! Integração com Gov.br para autenticação digital
//! 
//! Implementa OAuth2 e validação de identidade através do Gov.br

use crate::config::Config;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Serviço de integração com Gov.br
pub struct GovBrService {
    client: Client,
    base_url: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

/// Token de acesso do Gov.br
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovBrToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: String,
}

/// Dados do usuário autenticado no Gov.br
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovBrUser {
    pub sub: String, // CPF
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub birthdate: Option<String>,
    pub cpf: String,
    pub pis: Option<String>,
    pub voter_id: Option<String>, // Título de eleitor
    pub verified: bool,
}

/// Resposta de validação de eleitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterValidationResponse {
    pub cpf: String,
    pub voter_id: String,
    pub name: String,
    pub birth_date: String,
    pub voting_zone: String,
    pub voting_section: String,
    pub city: String,
    pub state: String,
    pub status: VoterStatus,
    pub last_vote: Option<DateTime<Utc>>,
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
}

impl GovBrService {
    /// Cria nova instância do serviço Gov.br
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            base_url: config.tse.gov_br_base_url.clone(),
            client_id: config.tse.client_id.clone(),
            client_secret: config.tse.client_secret.clone(),
            redirect_uri: config.tse.redirect_uri.clone(),
        }
    }

    /// Gera URL de autorização OAuth2
    pub fn get_authorization_url(&self, state: &str) -> String {
        let mut params = HashMap::new();
        params.insert("response_type", "code");
        params.insert("client_id", &self.client_id);
        params.insert("redirect_uri", &self.redirect_uri);
        params.insert("scope", "openid profile cpf");
        params.insert("state", state);
        let nonce = uuid::Uuid::new_v4().to_string();
        params.insert("nonce", &nonce);

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        format!("{}/authorize?{}", self.base_url, query_string)
    }

    /// Troca código de autorização por token de acesso
    pub async fn exchange_code_for_token(&self, code: &str) -> Result<GovBrToken> {
        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("redirect_uri", &self.redirect_uri);
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);

        let response = self.client
            .post(&format!("{}/token", self.base_url))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Erro ao trocar código por token: {}", error_text));
        }

        let token: GovBrToken = response.json().await?;
        Ok(token)
    }

    /// Obtém dados do usuário usando token de acesso
    pub async fn get_user_info(&self, access_token: &str) -> Result<GovBrUser> {
        let response = self.client
            .get(&format!("{}/userinfo", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Erro ao obter dados do usuário: {}", error_text));
        }

        let user: GovBrUser = response.json().await?;
        Ok(user)
    }

    /// Valida se o usuário é um eleitor ativo no TSE
    pub async fn validate_voter(&self, cpf: &str, access_token: &str) -> Result<VoterValidationResponse> {
        let response = self.client
            .get(&format!("{}/api/v1/voter/validate/{}", self.base_url, cpf))
            .bearer_auth(access_token)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Erro ao validar eleitor: {}", error_text));
        }

        let validation: VoterValidationResponse = response.json().await?;
        Ok(validation)
    }

    /// Verifica se o token ainda é válido
    pub async fn validate_token(&self, access_token: &str) -> Result<bool> {
        let response = self.client
            .get(&format!("{}/tokeninfo", self.base_url))
            .bearer_auth(access_token)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    /// Renova token de acesso usando refresh token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<GovBrToken> {
        let mut params = HashMap::new();
        params.insert("grant_type", "refresh_token");
        params.insert("refresh_token", refresh_token);
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);

        let response = self.client
            .post(&format!("{}/token", self.base_url))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Erro ao renovar token: {}", error_text));
        }

        let token: GovBrToken = response.json().await?;
        Ok(token)
    }

    /// Revoga token de acesso
    pub async fn revoke_token(&self, access_token: &str) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("token", access_token);
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);

        let response = self.client
            .post(&format!("{}/revoke", self.base_url))
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Erro ao revogar token: {}", error_text));
        }

        Ok(())
    }
}
