//! Serviço de autenticação do FORTIS

use crate::models::{AuthRequest, AuthResponse, UserInfo, BiometricData};
use sqlx::{Pool, Postgres};
use crate::crypto::CryptoService;
use anyhow::Result;
use uuid::Uuid;
use chrono::{Utc, Duration};

pub struct AuthService {
    db: Pool<Postgres>,
    crypto: CryptoService,
}

impl AuthService {
    pub fn new(db: Pool<Postgres>, crypto: CryptoService) -> Self {
        Self { db, crypto }
    }
    
    /// Autenticar usuário com biometria e certificado digital
    pub async fn authenticate(&self, req: &AuthRequest) -> Result<AuthResponse> {
        // 1. Validar CPF
        self.validate_cpf(&req.cpf)?;
        
        // 2. Verificar biometria
        let voter = if let Some(biometric) = &req.biometric_data {
            self.verify_biometric(&req.cpf, biometric).await?
        } else {
            return Err(anyhow::anyhow!("Dados biométricos são obrigatórios"));
        };
        
        // 3. Verificar certificado digital (se fornecido)
        if let Some(cert) = &req.certificate {
            self.verify_certificate(cert, &voter.certificate_hash.as_ref().unwrap())?;
        }
        
        // 4. Verificar elegibilidade
        if !voter.is_eligible {
            return Err(anyhow::anyhow!("Eleitor não está elegível para votar"));
        }
        
        // 5. Gerar tokens JWT
        let access_token = self.generate_access_token(&voter)?;
        let refresh_token = self.generate_refresh_token(&voter)?;
        
        // 6. Criar resposta
        Ok(AuthResponse {
            access_token,
            refresh_token,
            expires_in: 3600, // 1 hora
            token_type: "Bearer".to_string(),
            user: UserInfo {
                id: voter.id,
                cpf: req.cpf.clone(),
                name: self.crypto.decrypt(&voter.name_encrypted)?,
                roles: vec!["voter".to_string()],
                election_eligible: voter.is_eligible,
            },
        })
    }
    
    /// Refresh token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResponse> {
        // TODO: Implementar refresh token
        Err(anyhow::anyhow!("Refresh token não implementado"))
    }
    
    /// Logout
    pub async fn logout(&self, refresh_token: &str) -> Result<()> {
        // TODO: Implementar logout
        Ok(())
    }
    
    /// Verificar token
    pub async fn verify_token(&self, token: &str) -> Result<UserInfo> {
        // TODO: Implementar verificação de token
        Err(anyhow::anyhow!("Verificação de token não implementada"))
    }
    
    /// Validar CPF
    fn validate_cpf(&self, cpf: &str) -> Result<()> {
        // Remove caracteres não numéricos
        let cpf_clean = cpf.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        
        if cpf_clean.len() != 11 {
            return Err(anyhow::anyhow!("CPF deve ter 11 dígitos"));
        }
        
        // Verificar se todos os dígitos são iguais
        if cpf_clean.chars().all(|c| c == cpf_clean.chars().next().unwrap()) {
            return Err(anyhow::anyhow!("CPF inválido"));
        }
        
        // Validar dígitos verificadores
        let digits: Vec<u32> = cpf_clean.chars().map(|c| c.to_digit(10).unwrap()).collect();
        
        // Primeiro dígito verificador
        let mut sum = 0;
        for i in 0..9 {
            sum += digits[i] * (10 - i as u32);
        }
        let first_digit = (sum * 10) % 11;
        let first_digit = if first_digit == 10 { 0 } else { first_digit };
        
        if first_digit != digits[9] {
            return Err(anyhow::anyhow!("CPF inválido"));
        }
        
        // Segundo dígito verificador
        let mut sum = 0;
        for i in 0..10 {
            sum += digits[i] * (11 - i as u32);
        }
        let second_digit = (sum * 10) % 11;
        let second_digit = if second_digit == 10 { 0 } else { second_digit };
        
        if second_digit != digits[10] {
            return Err(anyhow::anyhow!("CPF inválido"));
        }
        
        Ok(())
    }
    
    /// Verificar biometria
    async fn verify_biometric(&self, cpf: &str, biometric: &BiometricData) -> Result<crate::models::Voter> {
        let cpf_hash = self.crypto.hash_sha256(cpf);
        
        // Buscar eleitor no banco
        let voter = sqlx::query_as::<_, crate::models::Voter>(
            "SELECT * FROM voters WHERE cpf_hash = $1"
        )
        .bind(&cpf_hash?)
        .fetch_one(&self.db)
        .await?;
        
        // Verificar hash da biometria
        if voter.biometric_hash != biometric.fingerprint_hash {
            return Err(anyhow::anyhow!("Biometria não confere"));
        }
        
        Ok(voter)
    }
    
    /// Verificar certificado digital
    fn verify_certificate(&self, certificate: &str, stored_hash: &str) -> Result<()> {
        let cert_hash = self.crypto.hash_sha256(certificate);
        if cert_hash? != *stored_hash {
            return Err(anyhow::anyhow!("Certificado digital não confere"));
        }
        Ok(())
    }
    
    /// Gerar access token
    fn generate_access_token(&self, voter: &crate::models::Voter) -> Result<String> {
        // TODO: Implementar geração de JWT
        Ok("mock_access_token".to_string())
    }
    
    /// Gerar refresh token
    fn generate_refresh_token(&self, voter: &crate::models::Voter) -> Result<String> {
        // TODO: Implementar geração de refresh token
        Ok("mock_refresh_token".to_string())
    }
}
