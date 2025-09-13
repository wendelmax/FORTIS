//! Documentação OpenAPI/Swagger para o FORTIS Backend

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::models::{
    CreateElectionRequest, ElectionResponse, ApiResponse, AuthRequest, AuthResponse,
    VoteRequest, UserInfo, BiometricData, Candidate, ElectionStats, CreateCandidateRequest,
};

/// Estrutura principal da documentação OpenAPI
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::v1::auth::login,
        crate::health_check,
        crate::ready_check,
    ),
    components(
        schemas(
            CreateElectionRequest,
            ElectionResponse,
            ApiResponse<String>,
            ApiResponse<Vec<String>>,
            ApiResponse<ElectionResponse>,
            ApiResponse<serde_json::Value>,
            AuthRequest,
            AuthResponse,
            UserInfo,
            BiometricData,
            Candidate,
            ElectionStats,
            CreateCandidateRequest,
            VoteRequest,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Autenticação", description = "Endpoints de autenticação e autorização"),
        (name = "Health", description = "Health checks e monitoramento"),
    )
)]
pub struct ApiDoc;

/// Adiciona configurações de segurança à documentação
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}