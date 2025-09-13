//! Middleware CORS do FORTIS

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::http::header;
use actix_web::middleware::DefaultHeaders;
use actix_web::App;

pub struct Cors;

impl Cors {
    pub fn new() -> Self {
        Self
    }
}

impl actix_web::dev::Transform<App, ServiceRequest> for Cors {
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = CorsMiddleware;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Transform, Self::InitError>>>>;

    fn new_transform(&self, _app: App) -> Self::Future {
        Box::pin(async move { Ok(CorsMiddleware) })
    }
}

pub struct CorsMiddleware;

impl actix_web::dev::Transform<ServiceRequest, ServiceRequest> for CorsMiddleware {
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = CorsService;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Transform, Self::InitError>>>>;

    fn new_transform(&self, _req: ServiceRequest) -> Self::Future {
        Box::pin(async move { Ok(CorsService) })
    }
}

pub struct CorsService;

impl actix_web::dev::Service<ServiceRequest> for CorsService {
    type Response = ServiceResponse;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        Box::pin(async move {
            // Adicionar headers CORS
            let mut res = req.into_response(actix_web::HttpResponse::Ok().finish());
            
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::HeaderValue::from_static("*")
            );
            
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                header::HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS")
            );
            
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                header::HeaderValue::from_static("Content-Type, Authorization")
            );
            
            Ok(res)
        })
    }
}
