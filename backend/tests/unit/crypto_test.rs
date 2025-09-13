//! Testes unitários para módulo de criptografia

use fortis_backend::crypto::CryptoService;
use serde_json::json;

#[actix_web::test]
async fn test_encryption_decryption() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "Voto para candidato 123";
    let encryption_result = crypto_service.encrypt_vote(plaintext).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    assert!(!encrypted_data.is_empty());
    assert_ne!(encrypted_data, plaintext);
    
    let decryption_result = crypto_service.decrypt_vote(&encrypted_data).await;
    assert!(decryption_result.is_ok());
    
    let decrypted_text = decryption_result.unwrap();
    assert_eq!(decrypted_text, plaintext);
}

#[actix_web::test]
async fn test_encryption_with_different_keys() {
    let crypto_service1 = CryptoService::new("test-encryption-key-32-chars-1").unwrap();
    let crypto_service2 = CryptoService::new("test-encryption-key-32-chars-2").unwrap();
    
    let plaintext = "Voto para candidato 123";
    let encrypted_data = crypto_service1.encrypt_vote(plaintext).await.unwrap();
    
    // Dados criptografados com uma chave não devem ser descriptografados com outra
    let decryption_result = crypto_service2.decrypt_vote(&encrypted_data).await;
    assert!(decryption_result.is_err());
}

#[actix_web::test]
async fn test_encryption_with_empty_data() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "";
    let encryption_result = crypto_service.encrypt_vote(plaintext).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    let decryption_result = crypto_service.decrypt_vote(&encrypted_data).await;
    
    assert!(decryption_result.is_ok());
    assert_eq!(decryption_result.unwrap(), plaintext);
}

#[actix_web::test]
async fn test_encryption_with_large_data() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    // Dados grandes (1MB)
    let plaintext = "x".repeat(1024 * 1024);
    let encryption_result = crypto_service.encrypt_vote(&plaintext).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    let decryption_result = crypto_service.decrypt_vote(&encrypted_data).await;
    
    assert!(decryption_result.is_ok());
    assert_eq!(decryption_result.unwrap(), plaintext);
}

#[actix_web::test]
async fn test_encryption_with_special_characters() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "Voto com caracteres especiais: áéíóú, ç, ñ, @#$%^&*()";
    let encryption_result = crypto_service.encrypt_vote(plaintext).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    let decryption_result = crypto_service.decrypt_vote(&encrypted_data).await;
    
    assert!(decryption_result.is_ok());
    assert_eq!(decryption_result.unwrap(), plaintext);
}

#[actix_web::test]
async fn test_encryption_with_json_data() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let vote_data = json!({
        "election_id": "uuid-123",
        "candidate_id": "uuid-456",
        "voter_cpf": "12345678901",
        "timestamp": "2025-10-01T10:30:00Z"
    });
    
    let plaintext = serde_json::to_string(&vote_data).unwrap();
    let encryption_result = crypto_service.encrypt_vote(&plaintext).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    let decryption_result = crypto_service.decrypt_vote(&encrypted_data).await;
    
    assert!(decryption_result.is_ok());
    
    let decrypted_text = decryption_result.unwrap();
    let decrypted_data: serde_json::Value = serde_json::from_str(&decrypted_text).unwrap();
    assert_eq!(decrypted_data, vote_data);
}

#[actix_web::test]
async fn test_encryption_with_invalid_key() {
    let crypto_service_result = CryptoService::new("short-key");
    assert!(crypto_service_result.is_err());
}

#[actix_web::test]
async fn test_encryption_with_corrupted_data() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let corrupted_data = "corrupted-encrypted-data";
    let decryption_result = crypto_service.decrypt_vote(corrupted_data).await;
    
    assert!(decryption_result.is_err());
}

#[actix_web::test]
async fn test_encryption_with_base64_encoded_data() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "Voto para candidato 123";
    let encryption_result = crypto_service.encrypt_vote(plaintext).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    
    // Verifica se os dados criptografados são válidos base64
    let base64_decode_result = base64::decode(&encrypted_data);
    assert!(base64_decode_result.is_ok());
}

#[actix_web::test]
async fn test_encryption_performance() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "Voto para candidato 123";
    let start_time = std::time::Instant::now();
    
    // Testa performance de criptografia/descriptografia
    for _ in 0..1000 {
        let encrypted_data = crypto_service.encrypt_vote(plaintext).await.unwrap();
        let decrypted_text = crypto_service.decrypt_vote(&encrypted_data).await.unwrap();
        assert_eq!(decrypted_text, plaintext);
    }
    
    let duration = start_time.elapsed();
    
    // Verifica que 1000 operações de criptografia/descriptografia levam menos de 5 segundos
    assert!(duration.as_secs() < 5);
}

#[actix_web::test]
async fn test_encryption_with_different_nonces() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "Voto para candidato 123";
    
    // Criptografa o mesmo texto duas vezes
    let encrypted_data1 = crypto_service.encrypt_vote(plaintext).await.unwrap();
    let encrypted_data2 = crypto_service.encrypt_vote(plaintext).await.unwrap();
    
    // Os dados criptografados devem ser diferentes devido ao nonce único
    assert_ne!(encrypted_data1, encrypted_data2);
    
    // Mas ambos devem descriptografar para o mesmo texto
    let decrypted_text1 = crypto_service.decrypt_vote(&encrypted_data1).await.unwrap();
    let decrypted_text2 = crypto_service.decrypt_vote(&encrypted_data2).await.unwrap();
    
    assert_eq!(decrypted_text1, plaintext);
    assert_eq!(decrypted_text2, plaintext);
}

#[actix_web::test]
async fn test_encryption_with_unicode_data() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "Voto com emoji 🗳️ e caracteres unicode: αβγδε";
    let encryption_result = crypto_service.encrypt_vote(plaintext).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    let decryption_result = crypto_service.decrypt_vote(&encrypted_data).await;
    
    assert!(decryption_result.is_ok());
    assert_eq!(decryption_result.unwrap(), plaintext);
}

#[actix_web::test]
async fn test_encryption_with_binary_data() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    // Dados binários simulados
    let plaintext = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD, 0xFC];
    let plaintext_str = String::from_utf8_lossy(&plaintext);
    
    let encryption_result = crypto_service.encrypt_vote(&plaintext_str).await;
    
    assert!(encryption_result.is_ok());
    
    let encrypted_data = encryption_result.unwrap();
    let decryption_result = crypto_service.decrypt_vote(&encrypted_data).await;
    
    assert!(decryption_result.is_ok());
    assert_eq!(decryption_result.unwrap(), plaintext_str);
}

#[actix_web::test]
async fn test_encryption_with_concurrent_access() {
    let crypto_service = CryptoService::new("test-encryption-key-32-chars").unwrap();
    
    let plaintext = "Voto para candidato 123";
    
    // Testa acesso concorrente
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let crypto_service = crypto_service.clone();
            let plaintext = plaintext.to_string();
            tokio::spawn(async move {
                let encrypted_data = crypto_service.encrypt_vote(&plaintext).await.unwrap();
                let decrypted_text = crypto_service.decrypt_vote(&encrypted_data).await.unwrap();
                assert_eq!(decrypted_text, plaintext);
            })
        })
        .collect();
    
    // Aguarda todas as tarefas concluírem
    for handle in handles {
        handle.await.unwrap();
    }
}
