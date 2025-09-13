//! Middleware de autenticação do FORTIS

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::http::header;
use actix_web::HttpResponse;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl actix_web::dev::Transform<ServiceRequest, ServiceRequest> for AuthMiddleware {
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthService;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Transform, Self::InitError>>>>;

    fn new_transform(&self, _req: ServiceRequest) -> Self::Future {
        Box::pin(async move { Ok(AuthService) })
    }
}

pub struct AuthService;

impl actix_web::dev::Service<ServiceRequest> for AuthService {
    type Response = ServiceResponse;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Verificar se é uma rota que precisa de autenticação
        let path = req.path();
        let needs_auth = !path.starts_with("/health") && !path.starts_with("/api/v1/auth");
        
        if !needs_auth {
            return Box::pin(async move { Ok(req.into_response(actix_web::HttpResponse::Ok().finish())) });
        }
        
        // Verificar header Authorization
        if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    // TODO: Verificar token JWT
                    return Box::pin(async move { Ok(req.into_response(actix_web::HttpResponse::Ok().finish())) });
                }
            }
        }
        
        // Retornar erro 401 se não autenticado
        Box::pin(async move {
            Ok(req.into_response(
                HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "success": false,
                        "message": "Token de autenticação necessário",
                        "timestamp": chrono::Utc::now()
                    }))
            ))
        })
    }
}
