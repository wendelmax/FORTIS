//! Middleware de autenticação TSE
//! 
//! Implementa validação de tokens e certificados digitais para APIs TSE

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::basic::BasicAuth;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceResponse, Transform};
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::config::Config;
use crate::services::tse::{GovBrService, DigitalCertificateService};

/// Dados do usuário autenticado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub cpf: String,
    pub name: String,
    pub email: Option<String>,
    pub voter_id: Option<String>,
    pub auth_type: AuthType,
    pub permissions: Vec<String>,
    pub expires_at: DateTime<Utc>,
}

/// Tipo de autenticação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    GovBr,
    DigitalCertificate,
    ApiKey,
    Basic,
}

/// Claims do JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String, // CPF
    pub name: String,
    pub email: Option<String>,
    pub voter_id: Option<String>,
    pub auth_type: String,
    pub permissions: Vec<String>,
    pub exp: i64,
    pub iat: i64,
}

/// Erro de autenticação
#[derive(Debug, thiserror::Error)]
pub enum TseAuthError {
    #[error("Token inválido")]
    InvalidToken,
    #[error("Token expirado")]
    ExpiredToken,
    #[error("Certificado inválido")]
    InvalidCertificate,
    #[error("Usuário não autorizado")]
    Unauthorized,
    #[error("Permissão insuficiente")]
    InsufficientPermissions,
    #[error("Erro de validação: {0}")]
    ValidationError(String),
}

/// Middleware de autenticação TSE
pub struct TseAuthMiddleware {
    config: Config,
    required_permissions: Vec<String>,
}

impl TseAuthMiddleware {
    pub fn new(config: Config, required_permissions: Vec<String>) -> Self {
        Self {
            config,
            required_permissions,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for TseAuthMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TseAuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TseAuthService {
            service,
            config: self.config.clone(),
            required_permissions: self.required_permissions.clone(),
        }))
    }
}

/// Serviço de autenticação TSE
pub struct TseAuthService<S> {
    service: S,
    config: Config,
    required_permissions: Vec<String>,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for TseAuthService<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let config = self.config.clone();
        let required_permissions = self.required_permissions.clone();
        let service = self.service.clone();

        async move {
            // Tentar diferentes métodos de autenticação
            let auth_result = match Self::authenticate_request(&req, &config).await {
                Ok(user) => {
                    // Verificar permissões
                    if Self::check_permissions(&user, &required_permissions) {
                        Ok(user)
                    } else {
                        Err(TseAuthError::InsufficientPermissions)
                    }
                }
                Err(e) => Err(e),
            };

            match auth_result {
                Ok(user) => {
                    // Adicionar usuário autenticado ao request
                    req.extensions_mut().insert(user);
                    service.call(req).await
                }
                Err(e) => {
                    // Retornar erro de autenticação
                    let error_response = actix_web::HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "success": false,
                            "error": e.to_string(),
                            "timestamp": Utc::now()
                        }));
                    
                    Ok(ServiceResponse::new(req.into_parts().0, error_response))
                }
            }
        }
        .boxed_local()
    }
}

impl<S> TseAuthService<S> {
    /// Autentica a requisição usando diferentes métodos
    async fn authenticate_request(
        req: &ServiceRequest,
        config: &Config,
    ) -> Result<AuthenticatedUser, TseAuthError> {
        // 1. Tentar autenticação Bearer (JWT)
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    return Self::authenticate_jwt(token, config).await;
                }
            }
        }

        // 2. Tentar autenticação Basic
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Basic ") {
                    let token = &auth_str[6..];
                    return Self::authenticate_basic(token, config).await;
                }
            }
        }

        // 3. Tentar autenticação por certificado digital
        if let Some(cert_header) = req.headers().get("X-Client-Certificate") {
            if let Ok(cert_data) = cert_header.to_str() {
                return Self::authenticate_certificate(cert_data, config).await;
            }
        }

        // 4. Tentar autenticação por API Key
        if let Some(api_key) = req.headers().get("X-API-Key") {
            if let Ok(key) = api_key.to_str() {
                return Self::authenticate_api_key(key, config).await;
            }
        }

        Err(TseAuthError::Unauthorized)
    }

    /// Autentica usando JWT
    async fn authenticate_jwt(
        token: &str,
        config: &Config,
    ) -> Result<AuthenticatedUser, TseAuthError> {
        let validation = Validation::new(Algorithm::HS256);
        let key = DecodingKey::from_secret(config.security.jwt_secret.as_bytes());

        match decode::<JwtClaims>(token, &key, &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;
                
                // Verificar se o token não expirou
                let now = Utc::now().timestamp();
                if claims.exp < now {
                    return Err(TseAuthError::ExpiredToken);
                }

                Ok(AuthenticatedUser {
                    user_id: claims.sub.clone(),
                    cpf: claims.sub,
                    name: claims.name,
                    email: claims.email,
                    voter_id: claims.voter_id,
                    auth_type: AuthType::GovBr,
                    permissions: claims.permissions,
                    expires_at: DateTime::from_timestamp(claims.exp, 0)
                        .unwrap_or_else(|| Utc::now()),
                })
            }
            Err(_) => Err(TseAuthError::InvalidToken),
        }
    }

    /// Autentica usando Basic Auth
    async fn authenticate_basic(
        token: &str,
        config: &Config,
    ) -> Result<AuthenticatedUser, TseAuthError> {
        // Decodificar Basic Auth
        let decoded = match base64::decode(token) {
            Ok(d) => d,
            Err(_) => return Err(TseAuthError::InvalidToken),
        };

        let credentials = match String::from_utf8(decoded) {
            Ok(c) => c,
            Err(_) => return Err(TseAuthError::InvalidToken),
        };

        let parts: Vec<&str> = credentials.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(TseAuthError::InvalidToken);
        }

        let username = parts[0];
        let password = parts[1];

        // Validar credenciais (em implementação real, verificar no banco de dados)
        if username == "fortis" && password == "fortis123" {
            Ok(AuthenticatedUser {
                user_id: "system".to_string(),
                cpf: "00000000000".to_string(),
                name: "Sistema FORTIS".to_string(),
                email: Some("sistema@fortis.gov.br".to_string()),
                voter_id: None,
                auth_type: AuthType::Basic,
                permissions: vec!["admin".to_string(), "sync".to_string()],
                expires_at: Utc::now() + chrono::Duration::hours(1),
            })
        } else {
            Err(TseAuthError::Unauthorized)
        }
    }

    /// Autentica usando certificado digital
    async fn authenticate_certificate(
        cert_data: &str,
        config: &Config,
    ) -> Result<AuthenticatedUser, TseAuthError> {
        let cert_service = DigitalCertificateService::new();
        
        match cert_service.validate_certificate(cert_data).await {
            Ok(validation) => {
                if !validation.is_valid {
                    return Err(TseAuthError::InvalidCertificate);
                }

                if let Some(cert_info) = validation.certificate_info {
                    // Extrair CPF do subject
                    let cpf = cert_info.subject.cpf;
                    
                    // Validar CPF
                    if cpf.len() != 11 {
                        return Err(TseAuthError::ValidationError("CPF inválido".to_string()));
                    }

                    Ok(AuthenticatedUser {
                        user_id: cpf.clone(),
                        cpf,
                        name: cert_info.subject.common_name,
                        email: cert_info.subject.email,
                        voter_id: None, // Será preenchido após validação TSE
                        auth_type: AuthType::DigitalCertificate,
                        permissions: vec!["vote".to_string(), "certificate".to_string()],
                        expires_at: cert_info.validity.not_after,
                    })
                } else {
                    Err(TseAuthError::InvalidCertificate)
                }
            }
            Err(e) => Err(TseAuthError::ValidationError(e.to_string())),
        }
    }

    /// Autentica usando API Key
    async fn authenticate_api_key(
        api_key: &str,
        config: &Config,
    ) -> Result<AuthenticatedUser, TseAuthError> {
        // Verificar se a API key é válida
        if api_key == config.tse.api_key {
            Ok(AuthenticatedUser {
                user_id: "api_client".to_string(),
                cpf: "00000000000".to_string(),
                name: "Cliente API".to_string(),
                email: Some("api@fortis.gov.br".to_string()),
                voter_id: None,
                auth_type: AuthType::ApiKey,
                permissions: vec!["api".to_string(), "sync".to_string()],
                expires_at: Utc::now() + chrono::Duration::hours(24),
            })
        } else {
            Err(TseAuthError::Unauthorized)
        }
    }

    /// Verifica se o usuário tem as permissões necessárias
    fn check_permissions(
        user: &AuthenticatedUser,
        required_permissions: &[String],
    ) -> bool {
        if required_permissions.is_empty() {
            return true;
        }

        required_permissions.iter().all(|permission| {
            user.permissions.contains(permission) || user.permissions.contains(&"admin".to_string())
        })
    }
}

/// Extrator para obter usuário autenticado
pub fn get_authenticated_user(req: &ServiceRequest) -> Option<AuthenticatedUser> {
    req.extensions().get::<AuthenticatedUser>().cloned()
}

/// Macro para verificar permissões
#[macro_export]
macro_rules! require_permissions {
    ($req:expr, $permissions:expr) => {
        if let Some(user) = get_authenticated_user($req) {
            if !$permissions.iter().all(|p| user.permissions.contains(p)) {
                return Ok(actix_web::HttpResponse::Forbidden().json(serde_json::json!({
                    "success": false,
                    "error": "Permissão insuficiente",
                    "timestamp": chrono::Utc::now()
                })));
            }
        } else {
            return Ok(actix_web::HttpResponse::Unauthorized().json(serde_json::json!({
                "success": false,
                "error": "Usuário não autenticado",
                "timestamp": chrono::Utc::now()
            })));
        }
    };
}

/// Middleware para APIs públicas (sem autenticação)
pub struct PublicApiMiddleware;

impl<S, B> Transform<S, ServiceRequest> for PublicApiMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = S;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(service))
    }
}

/// Middleware para APIs de administração (requer permissões admin)
pub struct AdminApiMiddleware {
    config: Config,
}

impl AdminApiMiddleware {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AdminApiMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TseAuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TseAuthService {
            service,
            config: self.config.clone(),
            required_permissions: vec!["admin".to_string()],
        }))
    }
}

/// Middleware para APIs de sincronização (requer permissões sync)
pub struct SyncApiMiddleware {
    config: Config,
}

impl SyncApiMiddleware {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for SyncApiMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TseAuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TseAuthService {
            service,
            config: self.config.clone(),
            required_permissions: vec!["sync".to_string()],
        }))
    }
}
