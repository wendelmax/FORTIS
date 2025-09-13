//! Testes de integração para votação

use actix_web::{test, web, App};
use serde_json::json;
use uuid::Uuid;

use fortis_backend::{
    api::v1::votes,
    database::{create_election, create_candidate, create_vote, has_voted, get_vote_count_by_candidate},
    models::{VoteRequest, VoteResponse},
    config::Config,
    database::init as init_db,
    crypto::CryptoService,
    auth::jwt::JwtService,
};

#[actix_web::test]
async fn test_cast_vote() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição
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

    // Criar candidato
    let candidate = fortis_backend::database::Candidate {
        id: Uuid::new_v4(),
        election_id,
        name: "Candidato Teste".to_string(),
        party: Some("Partido Teste".to_string()),
        number: Some(123),
        bio: Some("Biografia do candidato".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let candidate_id = create_candidate(&pool, &candidate).await.expect("Failed to create candidate");

    // Configurar serviços
    let crypto_service = CryptoService::new(&config.security.encryption_key).expect("Failed to create crypto service");
    let jwt_service = JwtService::new(
        &config.security.jwt_secret,
        &config.security.jwt_issuer,
        &config.security.jwt_audience,
    );

    // Gerar token JWT para teste
    let test_cpf = "12345678901";
    let test_name = "Eleitor Teste";
    let token = jwt_service.generate_token(test_cpf, test_name).expect("Failed to generate token");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(crypto_service))
            .app_data(web::Data::new(jwt_service))
            .service(web::scope("/api/v1/votes").configure(votes::configure))
    ).await;

    // Dados do voto
    let vote_data = json!({
        "election_id": election_id,
        "candidate_id": candidate_id
    });

    // Votar
    let req = test::TestRequest::post()
        .uri("/api/v1/votes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&vote_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"]["vote_id"].is_string());
    assert_eq!(body["data"]["election_id"], election_id.to_string());
    assert_eq!(body["data"]["candidate_id"], candidate_id.to_string());
}

#[actix_web::test]
async fn test_duplicate_vote() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição
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

    // Criar candidato
    let candidate = fortis_backend::database::Candidate {
        id: Uuid::new_v4(),
        election_id,
        name: "Candidato Teste".to_string(),
        party: Some("Partido Teste".to_string()),
        number: Some(123),
        bio: Some("Biografia do candidato".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let candidate_id = create_candidate(&pool, &candidate).await.expect("Failed to create candidate");

    // Configurar serviços
    let crypto_service = CryptoService::new(&config.security.encryption_key).expect("Failed to create crypto service");
    let jwt_service = JwtService::new(
        &config.security.jwt_secret,
        &config.security.jwt_issuer,
        &config.security.jwt_audience,
    );

    // Gerar token JWT para teste
    let test_cpf = "12345678901";
    let test_name = "Eleitor Teste";
    let token = jwt_service.generate_token(test_cpf, test_name).expect("Failed to generate token");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(crypto_service))
            .app_data(web::Data::new(jwt_service))
            .service(web::scope("/api/v1/votes").configure(votes::configure))
    ).await;

    // Dados do voto
    let vote_data = json!({
        "election_id": election_id,
        "candidate_id": candidate_id
    });

    // Primeiro voto
    let req = test::TestRequest::post()
        .uri("/api/v1/votes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&vote_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    // Segundo voto (deve falhar)
    let req = test::TestRequest::post()
        .uri("/api/v1/votes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&vote_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 409); // Conflict

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(!body["success"].as_bool().unwrap());
    assert!(body["error"].as_str().unwrap().contains("já votou"));
}

#[actix_web::test]
async fn test_get_vote_stats() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição
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

    // Criar candidatos
    let candidate1 = fortis_backend::database::Candidate {
        id: Uuid::new_v4(),
        election_id,
        name: "Candidato 1".to_string(),
        party: Some("Partido A".to_string()),
        number: Some(123),
        bio: Some("Biografia do candidato 1".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let candidate2 = fortis_backend::database::Candidate {
        id: Uuid::new_v4(),
        election_id,
        name: "Candidato 2".to_string(),
        party: Some("Partido B".to_string()),
        number: Some(456),
        bio: Some("Biografia do candidato 2".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let candidate1_id = create_candidate(&pool, &candidate1).await.expect("Failed to create candidate 1");
    let candidate2_id = create_candidate(&pool, &candidate2).await.expect("Failed to create candidate 2");

    // Criar votos
    let vote1 = fortis_backend::database::Vote {
        id: Uuid::new_v4(),
        election_id,
        candidate_id: candidate1_id,
        voter_cpf: "11111111111".to_string(),
        encrypted_vote: "encrypted_vote_1".to_string(),
        vote_hash: "hash_1".to_string(),
        zkp_proof: None,
        created_at: chrono::Utc::now(),
    };

    let vote2 = fortis_backend::database::Vote {
        id: Uuid::new_v4(),
        election_id,
        candidate_id: candidate1_id,
        voter_cpf: "22222222222".to_string(),
        encrypted_vote: "encrypted_vote_2".to_string(),
        vote_hash: "hash_2".to_string(),
        zkp_proof: None,
        created_at: chrono::Utc::now(),
    };

    let vote3 = fortis_backend::database::Vote {
        id: Uuid::new_v4(),
        election_id,
        candidate_id: candidate2_id,
        voter_cpf: "33333333333".to_string(),
        encrypted_vote: "encrypted_vote_3".to_string(),
        vote_hash: "hash_3".to_string(),
        zkp_proof: None,
        created_at: chrono::Utc::now(),
    };

    create_vote(&pool, &vote1).await.expect("Failed to create vote 1");
    create_vote(&pool, &vote2).await.expect("Failed to create vote 2");
    create_vote(&pool, &vote3).await.expect("Failed to create vote 3");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/v1/votes").configure(votes::configure))
    ).await;

    // Buscar estatísticas de votos
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/votes/{}", election_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"].is_array());
    
    let stats = body["data"].as_array().unwrap();
    assert_eq!(stats.len(), 2); // Dois candidatos

    // Verificar se os votos estão corretos
    let mut found_candidate1 = false;
    let mut found_candidate2 = false;

    for stat in stats {
        let candidate_id = stat["candidate_id"].as_str().unwrap();
        let vote_count = stat["vote_count"].as_u64().unwrap();

        if candidate_id == candidate1_id.to_string() {
            assert_eq!(vote_count, 2);
            found_candidate1 = true;
        } else if candidate_id == candidate2_id.to_string() {
            assert_eq!(vote_count, 1);
            found_candidate2 = true;
        }
    }

    assert!(found_candidate1);
    assert!(found_candidate2);
}

#[actix_web::test]
async fn test_unauthorized_vote() {
    // Configurar aplicação de teste
    let config = Config::new().expect("Failed to load config");
    let pool = init_db(&config.database).await.expect("Failed to init database");
    
    // Criar eleição
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

    // Criar candidato
    let candidate = fortis_backend::database::Candidate {
        id: Uuid::new_v4(),
        election_id,
        name: "Candidato Teste".to_string(),
        party: Some("Partido Teste".to_string()),
        number: Some(123),
        bio: Some("Biografia do candidato".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let candidate_id = create_candidate(&pool, &candidate).await.expect("Failed to create candidate");

    // Configurar serviços
    let crypto_service = CryptoService::new(&config.security.encryption_key).expect("Failed to create crypto service");
    let jwt_service = JwtService::new(
        &config.security.jwt_secret,
        &config.security.jwt_issuer,
        &config.security.jwt_audience,
    );

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(crypto_service))
            .app_data(web::Data::new(jwt_service))
            .service(web::scope("/api/v1/votes").configure(votes::configure))
    ).await;

    // Dados do voto
    let vote_data = json!({
        "election_id": election_id,
        "candidate_id": candidate_id
    });

    // Tentar votar sem token
    let req = test::TestRequest::post()
        .uri("/api/v1/votes")
        .set_json(&vote_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401); // Unauthorized

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(!body["success"].as_bool().unwrap());
    assert!(body["error"].as_str().unwrap().contains("Token de autorização necessário"));
}
