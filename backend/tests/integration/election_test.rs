//! Testes de integração para eleições

use actix_web::{test, web, App};
use serde_json::json;
use uuid::Uuid;

use fortis_backend::{
    api::v1::elections,
    database::{create_election, get_election, get_active_elections, update_election_status},
    models::{CreateElectionRequest, ElectionResponse},
    config::Config,
    database::init as init_db,
};

#[actix_web::test]
async fn test_create_election() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1/elections").configure(elections::configure))
    ).await;

    // Dados da eleição
    let election_data = json!({
        "name": "Eleição Municipal 2025",
        "description": "Eleição para prefeito e vereadores",
        "start_date": "2025-10-01T08:00:00Z",
        "end_date": "2025-10-01T17:00:00Z"
    });

    // Criar eleição
    let req = test::TestRequest::post()
        .uri("/api/v1/elections")
        .set_json(&election_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["name"], "Eleição Municipal 2025");
}

#[actix_web::test]
async fn test_get_elections() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1/elections").configure(elections::configure))
    ).await;

    // Buscar eleições
    let req = test::TestRequest::get()
        .uri("/api/v1/elections")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"].is_array());
}

#[actix_web::test]
async fn test_get_election_by_id() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição diretamente no banco
    let election = fortis_backend::database::Election {
        id: Uuid::new_v4(),
        name: "Eleição Teste".to_string(),
        description: Some("Descrição da eleição teste".to_string()),
        start_date: chrono::Utc::now(),
        end_date: chrono::Utc::now() + chrono::Duration::hours(1),
        status: "active".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let election_id = create_election(&pool, &election).await.expect("Failed to create election");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1/elections").configure(elections::configure))
    ).await;

    // Buscar eleição por ID
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/elections/{}", election_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["name"], "Eleição Teste");
}

#[actix_web::test]
async fn test_update_election_status() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição diretamente no banco
    let election = fortis_backend::database::Election {
        id: Uuid::new_v4(),
        name: "Eleição Teste".to_string(),
        description: Some("Descrição da eleição teste".to_string()),
        start_date: chrono::Utc::now(),
        end_date: chrono::Utc::now() + chrono::Duration::hours(1),
        status: "draft".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let election_id = create_election(&pool, &election).await.expect("Failed to create election");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1/elections").configure(elections::configure))
    ).await;

    // Atualizar status da eleição
    let status_data = json!({
        "status": "active"
    });

    let req = test::TestRequest::put()
        .uri(&format!("/api/v1/elections/{}", election_id))
        .set_json(&status_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
}

#[actix_web::test]
async fn test_get_election_candidates() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição diretamente no banco
    let election = fortis_backend::database::Election {
        id: Uuid::new_v4(),
        name: "Eleição Teste".to_string(),
        description: Some("Descrição da eleição teste".to_string()),
        start_date: chrono::Utc::now(),
        end_date: chrono::Utc::now() + chrono::Duration::hours(1),
        status: "active".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let election_id = create_election(&pool, &election).await.expect("Failed to create election");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1/elections").configure(elections::configure))
    ).await;

    // Buscar candidatos da eleição
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/elections/{}/candidates", election_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"].is_array());
}

#[actix_web::test]
async fn test_get_election_stats() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição diretamente no banco
    let election = fortis_backend::database::Election {
        id: Uuid::new_v4(),
        name: "Eleição Teste".to_string(),
        description: Some("Descrição da eleição teste".to_string()),
        start_date: chrono::Utc::now(),
        end_date: chrono::Utc::now() + chrono::Duration::hours(1),
        status: "active".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let election_id = create_election(&pool, &election).await.expect("Failed to create election");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1/elections").configure(elections::configure))
    ).await;

    // Buscar estatísticas da eleição
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/elections/{}/stats", election_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["total_votes"].is_number());
    assert!(body["data"]["unique_voters"].is_number());
}
