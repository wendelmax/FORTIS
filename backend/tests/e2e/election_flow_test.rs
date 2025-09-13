//! Testes end-to-end para fluxo completo de eleição

use actix_web::{test, web, App};
use fortis_backend::config::Config;
use fortis_backend::database::Pool;
use fortis_backend::auth::jwt::JwtService;
use fortis_backend::crypto::CryptoService;
use serde_json::json;
use uuid::Uuid;

// Helper para criar app de teste
async fn create_test_app() -> impl actix_web::dev::Service<actix_web::HttpRequest, Response = actix_web::HttpResponse, Error = actix_web::Error> {
    let config = Config::new().expect("Failed to load config");
    let pool = Pool::connect_lazy(&config.database.url).expect("Failed to create pool");
    let jwt_service = JwtService::new(
        &config.security.jwt_secret,
        &config.security.jwt_issuer,
        &config.security.jwt_audience,
    );
    let crypto_service = CryptoService::new(&config.security.encryption_key).expect("Failed to create crypto service");

    App::new()
        .app_data(web::Data::new(config))
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(jwt_service))
        .app_data(web::Data::new(crypto_service))
        .service(
            web::scope("/api/v1")
                .configure(fortis_backend::api::v1::configure)
        )
}

#[actix_web::test]
async fn test_complete_election_flow() {
    let app = create_test_app().await;
    let app = test::init_service(app).await;

    // 1. Criar eleição
    let election_data = json!({
        "name": "Eleição Teste 2025",
        "description": "Eleição para testes",
        "start_date": "2025-10-01T08:00:00Z",
        "end_date": "2025-10-01T17:00:00Z"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/elections")
        .set_json(&election_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let election_response: serde_json::Value = test::read_body_json(resp).await;
    let election_id = election_response["data"]["id"].as_str().unwrap();

    // 2. Adicionar candidatos
    let candidate1_data = json!({
        "name": "João Silva",
        "party": "PT",
        "number": 13,
        "bio": "Candidato a prefeito"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/elections/{}/candidates", election_id))
        .set_json(&candidate1_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let candidate2_data = json!({
        "name": "Maria Santos",
        "party": "PSDB",
        "number": 45,
        "bio": "Candidata a prefeita"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/elections/{}/candidates", election_id))
        .set_json(&candidate2_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // 3. Listar candidatos
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/elections/{}/candidates", election_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let candidates_response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(candidates_response["data"].as_array().unwrap().len(), 2);

    // 4. Simular login de eleitor
    let login_data = json!({
        "cpf": "12345678901",
        "biometric_data": "test-fingerprint",
        "certificate": "test-certificate"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&login_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let login_response: serde_json::Value = test::read_body_json(resp).await;
    let token = login_response["token"].as_str().unwrap();

    // 5. Registrar voto
    let vote_data = json!({
        "election_id": election_id,
        "candidate_id": candidates_response["data"][0]["id"],
        "biometric_verification": "test-fingerprint"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/votes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&vote_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let vote_response: serde_json::Value = test::read_body_json(resp).await;
    let vote_id = vote_response["data"]["vote_id"].as_str().unwrap();

    // 6. Verificar voto
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/votes/verify/{}", vote_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // 7. Tentar votar novamente (deve falhar)
    let req = test::TestRequest::post()
        .uri("/api/v1/votes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&vote_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 409); // Conflict - voto duplicado

    // 8. Obter resultados da eleição
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/votes/{}", election_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let results_response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(results_response["data"]["total_votes"], 1);
}

#[actix_web::test]
async fn test_election_with_multiple_voters() {
    let app = create_test_app().await;
    let app = test::init_service(app).await;

    // 1. Criar eleição
    let election_data = json!({
        "name": "Eleição Multi-Votante 2025",
        "description": "Eleição com múltiplos votantes",
        "start_date": "2025-10-01T08:00:00Z",
        "end_date": "2025-10-01T17:00:00Z"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/elections")
        .set_json(&election_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let election_response: serde_json::Value = test::read_body_json(resp).await;
    let election_id = election_response["data"]["id"].as_str().unwrap();

    // 2. Adicionar candidatos
    let candidate_data = json!({
        "name": "Candidato Único",
        "party": "TEST",
        "number": 99,
        "bio": "Candidato de teste"
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/elections/{}/candidates", election_id))
        .set_json(&candidate_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let candidates_response: serde_json::Value = test::read_body_json(resp).await;
    let candidate_id = candidates_response["data"]["id"].as_str().unwrap();

    // 3. Simular múltiplos votantes
    let voters = vec![
        "12345678901",
        "12345678902",
        "12345678903",
        "12345678904",
        "12345678905"
    ];

    for (i, cpf) in voters.iter().enumerate() {
        // Login do eleitor
        let login_data = json!({
            "cpf": cpf,
            "biometric_data": format!("test-fingerprint-{}", i),
            "certificate": format!("test-certificate-{}", i)
        });

        let req = test::TestRequest::post()
            .uri("/api/v1/auth/login")
            .set_json(&login_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let login_response: serde_json::Value = test::read_body_json(resp).await;
        let token = login_response["token"].as_str().unwrap();

        // Registrar voto
        let vote_data = json!({
            "election_id": election_id,
            "candidate_id": candidate_id,
            "biometric_verification": format!("test-fingerprint-{}", i)
        });

        let req = test::TestRequest::post()
            .uri("/api/v1/votes")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&vote_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    // 4. Verificar resultados
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/votes/{}", election_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let results_response: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(results_response["data"]["total_votes"], 5);
    assert_eq!(results_response["data"]["unique_voters"], 5);
}

#[actix_web::test]
async fn test_election_audit_flow() {
    let app = create_test_app().await;
    let app = test::init_service(app).await;

    // 1. Criar eleição
    let election_data = json!({
        "name": "Eleição Auditável 2025",
        "description": "Eleição para auditoria",
        "start_date": "2025-10-01T08:00:00Z",
        "end_date": "2025-10-01T17:00:00Z"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/elections")
        .set_json(&election_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let election_response: serde_json::Value = test::read_body_json(resp).await;
    let election_id = election_response["data"]["id"].as_str().unwrap();

    // 2. Simular alguns votos
    let voters = vec!["12345678901", "12345678902", "12345678903"];

    for (i, cpf) in voters.iter().enumerate() {
        let login_data = json!({
            "cpf": cpf,
            "biometric_data": format!("test-fingerprint-{}", i),
            "certificate": format!("test-certificate-{}", i)
        });

        let req = test::TestRequest::post()
            .uri("/api/v1/auth/login")
            .set_json(&login_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let login_response: serde_json::Value = test::read_body_json(resp).await;
        let token = login_response["token"].as_str().unwrap();

        let vote_data = json!({
            "election_id": election_id,
            "candidate_id": "test-candidate-id",
            "biometric_verification": format!("test-fingerprint-{}", i)
        });

        let req = test::TestRequest::post()
            .uri("/api/v1/votes")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&vote_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    // 3. Iniciar auditoria
    let audit_data = json!({
        "election_id": election_id,
        "audit_type": "full",
        "description": "Auditoria completa da eleição"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/audit")
        .set_json(&audit_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let audit_response: serde_json::Value = test::read_body_json(resp).await;
    let audit_id = audit_response["data"]["id"].as_str().unwrap();

    // 4. Verificar auditoria
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/audit/{}", audit_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let audit_details: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(audit_details["data"]["election_id"], election_id);
    assert_eq!(audit_details["data"]["status"], "completed");
}

#[actix_web::test]
async fn test_election_error_handling() {
    let app = create_test_app().await;
    let app = test::init_service(app).await;

    // 1. Tentar criar eleição com dados inválidos
    let invalid_election_data = json!({
        "name": "", // Nome vazio
        "start_date": "invalid-date",
        "end_date": "2025-10-01T17:00:00Z"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/elections")
        .set_json(&invalid_election_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);

    // 2. Tentar votar sem autenticação
    let vote_data = json!({
        "election_id": "test-election-id",
        "candidate_id": "test-candidate-id",
        "biometric_verification": "test-fingerprint"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/votes")
        .set_json(&vote_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);

    // 3. Tentar acessar eleição inexistente
    let req = test::TestRequest::get()
        .uri("/api/v1/elections/non-existent-id")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_election_performance() {
    let app = create_test_app().await;
    let app = test::init_service(app).await;

    let start_time = std::time::Instant::now();

    // 1. Criar eleição
    let election_data = json!({
        "name": "Eleição Performance 2025",
        "description": "Eleição para testes de performance",
        "start_date": "2025-10-01T08:00:00Z",
        "end_date": "2025-10-01T17:00:00Z"
    });

    let req = test::TestRequest::post()
        .uri("/api/v1/elections")
        .set_json(&election_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let election_response: serde_json::Value = test::read_body_json(resp).await;
    let election_id = election_response["data"]["id"].as_str().unwrap();

    // 2. Simular múltiplas requisições simultâneas
    let handles: Vec<_> = (0..100)
        .map(|i| {
            let app = app.clone();
            let election_id = election_id.to_string();
            tokio::spawn(async move {
                let req = test::TestRequest::get()
                    .uri(&format!("/api/v1/elections/{}", election_id))
                    .to_request();

                let resp = test::call_service(&app, req).await;
                assert_eq!(resp.status(), 200);
            })
        })
        .collect();

    // Aguarda todas as requisições concluírem
    for handle in handles {
        handle.await.unwrap();
    }

    let duration = start_time.elapsed();

    // Verifica que 100 requisições levam menos de 5 segundos
    assert!(duration.as_secs() < 5);
}
