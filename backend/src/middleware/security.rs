//! Middleware de segurança para o FORTIS Backend

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpRequest, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
    time::{Duration, Instant},
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

/// Rate limiter para prevenir abuso
#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            requests: HashMap::new(),
            max_requests,
            window_duration,
        }
    }

    pub fn is_allowed(&mut self, key: &str) -> bool {
        let now = Instant::now();
        let window_start = now - self.window_duration;

        // Remove requisições antigas
        if let Some(requests) = self.requests.get_mut(key) {
            requests.retain(|&time| time > window_start);
            
            if requests.len() < self.max_requests as usize {
                requests.push(now);
                true
            } else {
                false
            }
        } else {
            self.requests.insert(key.to_string(), vec![now]);
            true
        }
    }
}

/// Middleware de rate limiting
pub struct RateLimitMiddleware {
    rate_limiter: Rc<Mutex<RateLimiter>>,
}

impl RateLimitMiddleware {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            rate_limiter: Rc::new(Mutex::new(RateLimiter::new(max_requests, window_duration))),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitService {
            service: Rc::new(service),
            rate_limiter: self.rate_limiter.clone(),
        }))
    }
}

pub struct RateLimitService<S> {
    service: Rc<S>,
    rate_limiter: Rc<Mutex<RateLimiter>>,
}

impl<S, B> Service<ServiceRequest> for RateLimitService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let rate_limiter = self.rate_limiter.clone();

        Box::pin(async move {
            // Obtém IP do cliente
            let client_ip = req
                .connection_info()
                .remote_addr()
                .unwrap_or("unknown")
                .to_string();

            // Verifica rate limit
            let is_allowed = {
                let mut limiter = rate_limiter.lock().unwrap();
                limiter.is_allowed(&client_ip)
            };

            if !is_allowed {
                let response = HttpResponse::TooManyRequests()
                    .json(json!({
                        "success": false,
                        "error": {
                            "code": "RATE_LIMIT_EXCEEDED",
                            "message": "Muitas requisições. Tente novamente mais tarde.",
                            "retry_after": 60
                        },
                        "timestamp": SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                    }));
                
                return Ok(req.into_response(response));
            }

            // Continua com a requisição
            service.call(req).await
        })
    }
}

/// Middleware de validação de headers de segurança
pub struct SecurityHeadersMiddleware;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeadersMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHeadersService {
            service: Rc::new(service),
        }))
    }
}

pub struct SecurityHeadersService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let mut response = service.call(req).await?;

            // Adiciona headers de segurança
            response.headers_mut().insert(
                "X-Content-Type-Options",
                "nosniff".parse().unwrap(),
            );
            response.headers_mut().insert(
                "X-Frame-Options",
                "DENY".parse().unwrap(),
            );
            response.headers_mut().insert(
                "X-XSS-Protection",
                "1; mode=block".parse().unwrap(),
            );
            response.headers_mut().insert(
                "Strict-Transport-Security",
                "max-age=31536000; includeSubDomains".parse().unwrap(),
            );
            response.headers_mut().insert(
                "Content-Security-Policy",
                "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'".parse().unwrap(),
            );
            response.headers_mut().insert(
                "Referrer-Policy",
                "strict-origin-when-cross-origin".parse().unwrap(),
            );
            response.headers_mut().insert(
                "Permissions-Policy",
                "geolocation=(), microphone=(), camera=()".parse().unwrap(),
            );

            Ok(response)
        })
    }
}

/// Middleware de validação de entrada
pub struct InputValidationMiddleware;

impl<S, B> Transform<S, ServiceRequest> for InputValidationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = InputValidationService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InputValidationService {
            service: Rc::new(service),
        }))
    }
}

pub struct InputValidationService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for InputValidationService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            // Valida tamanho da requisição
            if let Some(content_length) = req.headers().get("content-length") {
                if let Ok(length) = content_length.to_str() {
                    if let Ok(size) = length.parse::<usize>() {
                        if size > 10 * 1024 * 1024 { // 10MB
                            let response = HttpResponse::PayloadTooLarge()
                                .json(json!({
                                    "success": false,
                                    "error": {
                                        "code": "PAYLOAD_TOO_LARGE",
                                        "message": "Requisição muito grande. Máximo 10MB."
                                    },
                                    "timestamp": SystemTime::now()
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs()
                                }));
                            
                            return Ok(req.into_response(response));
                        }
                    }
                }
            }

            // Valida User-Agent
            if let Some(user_agent) = req.headers().get("user-agent") {
                if let Ok(ua) = user_agent.to_str() {
                    if ua.is_empty() || ua.len() > 500 {
                        let response = HttpResponse::BadRequest()
                            .json(json!({
                                "success": false,
                                "error": {
                                    "code": "INVALID_USER_AGENT",
                                    "message": "User-Agent inválido."
                                },
                                "timestamp": SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs()
                            }));
                        
                        return Ok(req.into_response(response));
                    }
                }
            }

            // Valida Content-Type para requisições POST/PUT
            if matches!(req.method(), &actix_web::http::Method::POST | &actix_web::http::Method::PUT) {
                if let Some(content_type) = req.headers().get("content-type") {
                    if let Ok(ct) = content_type.to_str() {
                        if !ct.starts_with("application/json") && !ct.starts_with("application/x-www-form-urlencoded") {
                            let response = HttpResponse::UnsupportedMediaType()
                                .json(json!({
                                    "success": false,
                                    "error": {
                                        "code": "UNSUPPORTED_MEDIA_TYPE",
                                        "message": "Content-Type não suportado. Use application/json."
                                    },
                                    "timestamp": SystemTime::now()
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs()
                                }));
                            
                            return Ok(req.into_response(response));
                        }
                    }
                }
            }

            // Continua com a requisição
            service.call(req).await
        })
    }
}

/// Middleware de logging de segurança
pub struct SecurityLoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for SecurityLoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityLoggingService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityLoggingService {
            service: Rc::new(service),
        }))
    }
}

pub struct SecurityLoggingService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SecurityLoggingService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let start_time = Instant::now();
            let client_ip = req
                .connection_info()
                .remote_addr()
                .unwrap_or("unknown")
                .to_string();
            let user_agent = req
                .headers()
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown")
                .to_string();
            let method = req.method().to_string();
            let path = req.path().to_string();

            let response = service.call(req).await?;

            let duration = start_time.elapsed();
            let status = response.status().as_u16();

            // Log de segurança
            log::info!(
                "Security Log: {} {} {} {} {} {}ms",
                client_ip,
                method,
                path,
                status,
                user_agent,
                duration.as_millis()
            );

            // Log de eventos suspeitos
            if status >= 400 {
                log::warn!(
                    "Suspicious Activity: {} {} {} {} {}",
                    client_ip,
                    method,
                    path,
                    status,
                    user_agent
                );
            }

            // Log de rate limiting
            if status == 429 {
                log::warn!(
                    "Rate Limit Exceeded: {} {} {}",
                    client_ip,
                    method,
                    path
                );
            }

            Ok(response)
        })
    }
}

/// Middleware de validação de origem
pub struct OriginValidationMiddleware {
    allowed_origins: Vec<String>,
}

impl OriginValidationMiddleware {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self { allowed_origins }
    }
}

impl<S, B> Transform<S, ServiceRequest> for OriginValidationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = OriginValidationService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OriginValidationService {
            service: Rc::new(service),
            allowed_origins: self.allowed_origins.clone(),
        }))
    }
}

pub struct OriginValidationService<S> {
    service: Rc<S>,
    allowed_origins: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for OriginValidationService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let allowed_origins = self.allowed_origins.clone();

        Box::pin(async move {
            // Verifica origem para requisições CORS
            if let Some(origin) = req.headers().get("origin") {
                if let Ok(origin_str) = origin.to_str() {
                    if !allowed_origins.contains(&origin_str.to_string()) {
                        let response = HttpResponse::Forbidden()
                            .json(json!({
                                "success": false,
                                "error": {
                                    "code": "ORIGIN_NOT_ALLOWED",
                                    "message": "Origem não permitida."
                                },
                                "timestamp": SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs()
                            }));
                        
                        return Ok(req.into_response(response));
                    }
                }
            }

            // Continua com a requisição
            service.call(req).await
        })
    }
}

/// Configuração de segurança
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub rate_limit_requests: u32,
    pub rate_limit_window: Duration,
    pub allowed_origins: Vec<String>,
    pub max_payload_size: usize,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            rate_limit_requests: 100,
            rate_limit_window: Duration::from_secs(60),
            allowed_origins: vec!["http://localhost:3000".to_string()],
            max_payload_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Aplica todos os middlewares de segurança
pub fn apply_security_middleware<S, B>(
    app: actix_web::App<S>,
    config: &SecurityConfig,
) -> actix_web::App<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    app
        .wrap(RateLimitMiddleware::new(
            config.rate_limit_requests,
            config.rate_limit_window,
        ))
        .wrap(SecurityHeadersMiddleware)
        .wrap(InputValidationMiddleware)
        .wrap(SecurityLoggingMiddleware)
        .wrap(OriginValidationMiddleware::new(config.allowed_origins.clone()))
}
