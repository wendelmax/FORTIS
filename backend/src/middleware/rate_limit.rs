//! Middleware de rate limiting do FORTIS

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;

pub struct RateLimitMiddleware;

impl RateLimitMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl actix_web::dev::Transform<ServiceRequest, ServiceRequest> for RateLimitMiddleware {
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitService;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Transform, Self::InitError>>>>;

    fn new_transform(&self, _req: ServiceRequest) -> Self::Future {
        Box::pin(async move { Ok(RateLimitService) })
    }
}

pub struct RateLimitService;

impl actix_web::dev::Service<ServiceRequest> for RateLimitService {
    type Response = ServiceResponse;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // TODO: Implementar rate limiting
        Box::pin(async move { Ok(req.into_response(actix_web::HttpResponse::Ok().finish())) })
    }
}
