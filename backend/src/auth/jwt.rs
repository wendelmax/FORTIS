//! Módulo JWT do FORTIS

use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use anyhow::Result;
use uuid::Uuid;

/// Claims do JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // CPF do usuário
    pub name: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String, // Issuer
    pub aud: String, // Audience
    pub jti: String, // JWT ID
    pub voter_id: Option<String>,
    pub zone: Option<String>,
    pub section: Option<String>,
}

/// Serviço JWT
#[derive(Clone)]
pub struct JwtService {
    secret: String,
    issuer: String,
    audience: String,
}

impl JwtService {
    pub fn new(secret: &str, issuer: &str, audience: &str) -> Self {
        Self {
            secret: secret.to_string(),
            issuer: issuer.to_string(),
            audience: audience.to_string(),
        }
    }

    /// Gerar token JWT
    pub fn generate_token(&self, cpf: &str, name: &str) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: cpf.to_string(),
            name: name.to_string(),
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            jti: Uuid::new_v4().to_string(),
            voter_id: None,
            zone: None,
            section: None,
        };

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(self.secret.as_ref());
        
        encode(&header, &claims, &key)
            .map_err(|e| anyhow::anyhow!("Erro ao gerar token: {}", e))
    }

    /// Gerar token JWT com dados do eleitor
    pub fn generate_voter_token(&self, cpf: &str, name: &str, voter_id: &str, zone: &str, section: &str) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: cpf.to_string(),
            name: name.to_string(),
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            jti: Uuid::new_v4().to_string(),
            voter_id: Some(voter_id.to_string()),
            zone: Some(zone.to_string()),
            section: Some(section.to_string()),
        };

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(self.secret.as_ref());
        
        encode(&header, &claims, &key)
            .map_err(|e| anyhow::anyhow!("Erro ao gerar token: {}", e))
    }

    /// Validar token JWT
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let key = DecodingKey::from_secret(self.secret.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        
        decode::<Claims>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| anyhow::anyhow!("Token inválido: {}", e))
    }

    /// Verificar se token está expirado
    pub fn is_expired(&self, claims: &Claims) -> bool {
        Utc::now().timestamp() > claims.exp
    }

    /// Renovar token (gerar novo com mesmo CPF)
    pub fn refresh_token(&self, old_claims: &Claims) -> Result<String> {
        let now = Utc::now();
        let new_claims = Claims {
            sub: old_claims.sub.clone(),
            name: old_claims.name.clone(),
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            jti: Uuid::new_v4().to_string(),
            voter_id: old_claims.voter_id.clone(),
            zone: old_claims.zone.clone(),
            section: old_claims.section.clone(),
        };

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(self.secret.as_ref());
        
        encode(&header, &new_claims, &key)
            .map_err(|e| anyhow::anyhow!("Erro ao renovar token: {}", e))
    }

    /// Extrair CPF do token
    pub fn extract_cpf(&self, token: &str) -> Result<String> {
        let claims = self.validate_token(token)?;
        Ok(claims.sub)
    }

    /// Verificar se token é válido e não expirado
    pub fn is_valid(&self, token: &str) -> bool {
        match self.validate_token(token) {
            Ok(claims) => !self.is_expired(&claims),
            Err(_) => false,
        }
    }

    /// Obter tempo restante do token em segundos
    pub fn get_remaining_time(&self, token: &str) -> Result<i64> {
        let claims = self.validate_token(token)?;
        let now = Utc::now().timestamp();
        Ok(claims.exp - now)
    }
}