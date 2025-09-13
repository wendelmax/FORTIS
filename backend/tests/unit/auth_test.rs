//! Testes unitários para módulo de autenticação

use actix_web::{test, web, App};
use fortis_backend::auth::jwt::JwtService;
use fortis_backend::auth::biometric::BiometricService;
use fortis_backend::auth::certificate::CertificateService;
use fortis_backend::config::Config;
use serde_json::json;

#[actix_web::test]
async fn test_jwt_token_creation() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": "12345678901",
        "role": "voter"
    });
    
    let token = jwt_service.create_token(&claims).await;
    assert!(token.is_ok());
    
    let token_str = token.unwrap();
    assert!(!token_str.is_empty());
}

#[actix_web::test]
async fn test_jwt_token_validation() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": "12345678901",
        "role": "voter"
    });
    
    let token = jwt_service.create_token(&claims).await.unwrap();
    let validation_result = jwt_service.validate_token(&token).await;
    
    assert!(validation_result.is_ok());
    
    let validated_claims = validation_result.unwrap();
    assert_eq!(validated_claims["user_id"], "test-user-123");
    assert_eq!(validated_claims["cpf"], "12345678901");
}

#[actix_web::test]
async fn test_jwt_token_expiration() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": "12345678901",
        "role": "voter",
        "exp": 1 // Token expirado
    });
    
    let token = jwt_service.create_token(&claims).await.unwrap();
    let validation_result = jwt_service.validate_token(&token).await;
    
    assert!(validation_result.is_err());
}

#[actix_web::test]
async fn test_biometric_verification() {
    let biometric_service = BiometricService::new();
    
    // Dados biométricos de teste
    let fingerprint_data = "test-fingerprint-data";
    let stored_template = "stored-template";
    
    let verification_result = biometric_service.verify_fingerprint(
        fingerprint_data,
        stored_template
    ).await;
    
    assert!(verification_result.is_ok());
}

#[actix_web::test]
async fn test_biometric_template_creation() {
    let biometric_service = BiometricService::new();
    
    let fingerprint_data = "test-fingerprint-data";
    let template_result = biometric_service.create_template(fingerprint_data).await;
    
    assert!(template_result.is_ok());
    
    let template = template_result.unwrap();
    assert!(!template.is_empty());
}

#[actix_web::test]
async fn test_certificate_validation() {
    let certificate_service = CertificateService::new();
    
    // Certificado de teste (base64 encoded)
    let certificate_data = "test-certificate-data";
    let validation_result = certificate_service.validate_certificate(certificate_data).await;
    
    assert!(validation_result.is_ok());
}

#[actix_web::test]
async fn test_certificate_extraction() {
    let certificate_service = CertificateService::new();
    
    let certificate_data = "test-certificate-data";
    let extraction_result = certificate_service.extract_certificate_info(certificate_data).await;
    
    assert!(extraction_result.is_ok());
    
    let info = extraction_result.unwrap();
    assert!(info.contains_key("subject"));
    assert!(info.contains_key("issuer"));
    assert!(info.contains_key("valid_from"));
    assert!(info.contains_key("valid_to"));
}

#[actix_web::test]
async fn test_multi_factor_authentication() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let biometric_service = BiometricService::new();
    let certificate_service = CertificateService::new();
    
    // Simula autenticação multi-fator
    let cpf = "12345678901";
    let fingerprint_data = "test-fingerprint-data";
    let certificate_data = "test-certificate-data";
    
    // Verifica biometria
    let biometric_result = biometric_service.verify_fingerprint(
        fingerprint_data,
        "stored-template"
    ).await;
    assert!(biometric_result.is_ok());
    
    // Verifica certificado
    let certificate_result = certificate_service.validate_certificate(certificate_data).await;
    assert!(certificate_result.is_ok());
    
    // Cria token JWT
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": cpf,
        "role": "voter",
        "auth_methods": ["biometric", "certificate"]
    });
    
    let token_result = jwt_service.create_token(&claims).await;
    assert!(token_result.is_ok());
}

#[actix_web::test]
async fn test_authentication_failure_cases() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    // Testa token inválido
    let invalid_token = "invalid-token";
    let validation_result = jwt_service.validate_token(invalid_token).await;
    assert!(validation_result.is_err());
    
    // Testa token malformado
    let malformed_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid";
    let validation_result = jwt_service.validate_token(malformed_token).await;
    assert!(validation_result.is_err());
}

#[actix_web::test]
async fn test_authentication_rate_limiting() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": "12345678901",
        "role": "voter"
    });
    
    // Simula múltiplas tentativas de autenticação
    for _ in 0..10 {
        let token_result = jwt_service.create_token(&claims).await;
        assert!(token_result.is_ok());
    }
    
    // Em um cenário real, o rate limiting seria testado aqui
    // Por enquanto, apenas verificamos que não há erro
}

#[actix_web::test]
async fn test_authentication_security() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    // Testa com chave secreta diferente
    let different_jwt_service = JwtService::new(
        "different-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": "12345678901",
        "role": "voter"
    });
    
    let token = jwt_service.create_token(&claims).await.unwrap();
    
    // Token criado com uma chave não deve ser válido com outra chave
    let validation_result = different_jwt_service.validate_token(&token).await;
    assert!(validation_result.is_err());
}

#[actix_web::test]
async fn test_authentication_audit_logging() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": "12345678901",
        "role": "voter"
    });
    
    // Simula criação de token (que deve gerar log de auditoria)
    let token_result = jwt_service.create_token(&claims).await;
    assert!(token_result.is_ok());
    
    // Em um cenário real, verificaria se o log foi gerado
    // Por enquanto, apenas verificamos que não há erro
}

#[actix_web::test]
async fn test_authentication_performance() {
    let jwt_service = JwtService::new(
        "test-secret-key",
        "fortis-test",
        "fortis-users"
    );
    
    let claims = json!({
        "user_id": "test-user-123",
        "cpf": "12345678901",
        "role": "voter"
    });
    
    let start_time = std::time::Instant::now();
    
    // Testa performance de criação de token
    for _ in 0..100 {
        let token_result = jwt_service.create_token(&claims).await;
        assert!(token_result.is_ok());
    }
    
    let duration = start_time.elapsed();
    
    // Verifica que a criação de 100 tokens leva menos de 1 segundo
    assert!(duration.as_secs() < 1);
}
