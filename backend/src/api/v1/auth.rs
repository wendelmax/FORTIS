//! Módulo de autenticação da API v1

use actix_web::{web, HttpResponse, Result};
use crate::models::{AuthRequest, AuthResponse, ApiResponse, ApiError};
use utoipa::OpenApi;
use crate::services::auth::AuthService;

/// Configurar rotas de autenticação
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/login", web::post().to(login))
        .route("/refresh", web::post().to(refresh))
        .route("/logout", web::post().to(logout))
        .route("/verify", web::post().to(verify));
}

/// Endpoint de login
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = AuthRequest,
    responses(
        (status = 200, description = "Login realizado com sucesso", body = ApiResponse<AuthResponse>),
        (status = 401, description = "Credenciais inválidas", body = ApiResponse<String>),
        (status = 400, description = "Dados inválidos", body = ApiResponse<String>)
    ),
    tag = "Autenticação"
)]
async fn login(
    auth_service: web::Data<AuthService>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse> {
    match auth_service.authenticate(&req).await {
        Ok(response) => Ok(HttpResponse::Ok().json(ApiResponse::success(response))),
        Err(e) => Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Endpoint de refresh token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token renovado com sucesso", body = ApiResponse<AuthResponse>),
        (status = 401, description = "Token inválido", body = ApiResponse<String>)
    ),
    tag = "Autenticação"
)]
async fn refresh(
    auth_service: web::Data<AuthService>,
    req: web::Json<RefreshTokenRequest>,
) -> Result<HttpResponse> {
    match auth_service.refresh_token(&req.refresh_token).await {
        Ok(response) => Ok(HttpResponse::Ok().json(ApiResponse::success(response))),
        Err(e) => Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Endpoint de logout
async fn logout(
    auth_service: web::Data<AuthService>,
    req: web::Json<LogoutRequest>,
) -> Result<HttpResponse> {
    match auth_service.logout(&req.refresh_token).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::success("Logout realizado com sucesso"))),
        Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

/// Endpoint de verificação de token
async fn verify(
    auth_service: web::Data<AuthService>,
    req: web::Json<VerifyTokenRequest>,
) -> Result<HttpResponse> {
    match auth_service.verify_token(&req.token).await {
        Ok(user_info) => Ok(HttpResponse::Ok().json(ApiResponse::success(user_info))),
        Err(e) => Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e.to_string()))),
    }
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
struct RefreshTokenRequest {
    refresh_token: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
struct LogoutRequest {
    refresh_token: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
struct VerifyTokenRequest {
    token: String,
}
