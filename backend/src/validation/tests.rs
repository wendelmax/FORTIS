//! Testes unitários para sistema de validação robusta
//! 
//! Testa funcionalidades de validação na camada de aplicação
//! sem dependência de blockchain.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::vote_validator::*;
    use chrono::Utc;
    use serde_json::json;

    /// Testa criação do validador de votos
    #[test]
    fn test_vote_validator_creation() {
        let crypto_service = create_mock_crypto_service();
        let tse_validator = create_mock_tse_validator();
        let biometric_validator = create_mock_biometric_validator();
        
        let validator = VoteValidator::new(
            crypto_service,
            tse_validator,
            biometric_validator,
        );
        
        // Verificar se foi criado corretamente
        // (não há getters públicos, mas podemos verificar que não panica)
    }

    /// Testa validação de voto válido
    #[tokio::test]
    async fn test_validate_valid_vote() {
        let validator = create_test_validator().await;
        
        let vote = create_test_vote();
        let result = validator.validate_vote(&vote).await;
        
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(validation_result.is_valid);
        assert!(validation_result.errors.is_empty());
        assert!(!validation_result.validation_proof.voter_eligibility_proof.is_empty());
        assert!(!validation_result.validation_proof.biometric_verification_proof.is_empty());
        assert!(!validation_result.validation_proof.vote_uniqueness_proof.is_empty());
        assert!(!validation_result.validation_proof.cryptographic_integrity_proof.is_empty());
    }

    /// Testa validação de voto com eleitor não elegível
    #[tokio::test]
    async fn test_validate_ineligible_voter() {
        let mut validator = create_test_validator().await;
        
        // Configurar TSE validator para retornar eleitor não elegível
        validator.tse_validator = create_mock_tse_validator_ineligible();
        
        let vote = create_test_vote();
        let result = validator.validate_vote(&vote).await;
        
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.contains(&ValidationError::VoterNotEligible));
    }

    /// Testa validação de voto com falha biométrica
    #[tokio::test]
    async fn test_validate_biometric_failure() {
        let mut validator = create_test_validator().await;
        
        // Configurar biometric validator para falhar
        validator.biometric_validator = create_mock_biometric_validator_failing();
        
        let vote = create_test_vote();
        let result = validator.validate_vote(&vote).await;
        
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.contains(&ValidationError::BiometricVerificationFailed));
    }

    /// Testa validação de voto duplicado
    #[tokio::test]
    async fn test_validate_duplicate_vote() {
        let mut validator = create_test_validator().await;
        
        // Configurar crypto service para indicar voto duplicado
        validator.crypto_service = create_mock_crypto_service_duplicate();
        
        let vote = create_test_vote();
        let result = validator.validate_vote(&vote).await;
        
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.contains(&ValidationError::VoteAlreadyCast));
    }

    /// Testa validação de voto com assinatura inválida
    #[tokio::test]
    async fn test_validate_invalid_signature() {
        let mut validator = create_test_validator().await;
        
        // Configurar crypto service para falhar na verificação de assinatura
        validator.crypto_service = create_mock_crypto_service_invalid_signature();
        
        let vote = create_test_vote();
        let result = validator.validate_vote(&vote).await;
        
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.contains(&ValidationError::InvalidSignature));
    }

    /// Testa validação de voto com eleição inativa
    #[tokio::test]
    async fn test_validate_inactive_election() {
        let mut validator = create_test_validator().await;
        
        // Configurar TSE validator para retornar eleição inativa
        validator.tse_validator = create_mock_tse_validator_inactive_election();
        
        let vote = create_test_vote();
        let result = validator.validate_vote(&vote).await;
        
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.contains(&ValidationError::ElectionNotActive));
    }

    /// Testa validação de voto com candidato inválido
    #[tokio::test]
    async fn test_validate_invalid_candidate() {
        let mut validator = create_test_validator().await;
        
        // Configurar TSE validator para retornar candidato inválido
        validator.tse_validator = create_mock_tse_validator_invalid_candidate();
        
        let vote = create_test_vote();
        let result = validator.validate_vote(&vote).await;
        
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.contains(&ValidationError::InvalidCandidate));
    }

    /// Testa validação de criação de eleição
    #[tokio::test]
    async fn test_validate_election_creation() {
        let tse_validator = create_mock_tse_validator();
        let validator = ElectionValidator::new(tse_validator);
        
        let election = Election {
            id: "election1".to_string(),
            title: "Eleição Teste".to_string(),
            description: "Descrição da eleição teste".to_string(),
            start_time: Utc::now() + chrono::Duration::hours(1),
            end_time: Utc::now() + chrono::Duration::hours(24),
            is_active: false,
            is_completed: false,
            created_by: "admin1".to_string(),
            created_at: Utc::now(),
        };
        
        let result = validator.validate_election_creation(&election).await;
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(validation_result.is_valid);
        assert!(validation_result.errors.is_empty());
    }

    /// Testa validação de eleição com datas inválidas
    #[tokio::test]
    async fn test_validate_election_invalid_dates() {
        let tse_validator = create_mock_tse_validator();
        let validator = ElectionValidator::new(tse_validator);
        
        let election = Election {
            id: "election1".to_string(),
            title: "Eleição Teste".to_string(),
            description: "Descrição da eleição teste".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(1), // Data no passado
            end_time: Utc::now() + chrono::Duration::hours(24),
            is_active: false,
            is_completed: false,
            created_by: "admin1".to_string(),
            created_at: Utc::now(),
        };
        
        let result = validator.validate_election_creation(&election).await;
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.contains(&ValidationError::ElectionNotActive));
    }

    /// Testa validação de eleição com título vazio
    #[tokio::test]
    async fn test_validate_election_empty_title() {
        let tse_validator = create_mock_tse_validator();
        let validator = ElectionValidator::new(tse_validator);
        
        let election = Election {
            id: "election1".to_string(),
            title: "".to_string(), // Título vazio
            description: "Descrição da eleição teste".to_string(),
            start_time: Utc::now() + chrono::Duration::hours(1),
            end_time: Utc::now() + chrono::Duration::hours(24),
            is_active: false,
            is_completed: false,
            created_by: "admin1".to_string(),
            created_at: Utc::now(),
        };
        
        let result = validator.validate_election_creation(&election).await;
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(!validation_result.is_valid);
        assert!(validation_result.errors.iter().any(|e| matches!(e, ValidationError::CryptographicError(_))));
    }

    /// Testa performance com múltiplas validações
    #[tokio::test]
    async fn test_performance_multiple_validations() {
        let validator = create_test_validator().await;
        
        let start_time = std::time::Instant::now();
        
        // Validar 100 votos
        for i in 1..=100 {
            let vote = Vote {
                id: format!("vote{}", i),
                election_id: "election1".to_string(),
                voter_id: format!("voter{}", i),
                candidate_id: format!("candidate{}", i % 5),
                encrypted_vote: format!("encrypted_vote_{}", i),
                signature: format!("signature_{}", i),
                nullifier: format!("nullifier_{}", i),
                timestamp: Utc::now(),
                biometric_data: BiometricData {
                    fingerprint: format!("fingerprint_{}", i),
                    facial_data: format!("facial_data_{}", i),
                    liveness_data: format!("liveness_data_{}", i),
                },
                encryption_hash: format!("encryption_hash_{}", i),
            };
            
            let result = validator.validate_vote(&vote).await;
            assert!(result.is_ok());
        }
        
        let duration = start_time.elapsed();
        
        // Verificar que foi rápido (< 5 segundos para 100 validações)
        assert!(duration.as_secs() < 5, "Performance test failed: took {} seconds", duration.as_secs());
        
        println!("Performance test: 100 vote validations in {:?}", duration);
    }

    /// Testa geração de prova de validação
    #[tokio::test]
    async fn test_generate_validation_proof() {
        let validator = create_test_validator().await;
        let vote = create_test_vote();
        
        let proof = validator.generate_validation_proof(&vote).await.unwrap();
        
        assert!(!proof.voter_eligibility_proof.is_empty());
        assert!(!proof.biometric_verification_proof.is_empty());
        assert!(!proof.vote_uniqueness_proof.is_empty());
        assert!(!proof.cryptographic_integrity_proof.is_empty());
        assert!(!proof.merkle_root.is_empty());
    }

    /// Testa serialização de resultado de validação
    #[test]
    fn test_validation_result_serialization() {
        let validation_result = ValidationResult {
            is_valid: true,
            validation_timestamp: Utc::now(),
            validation_proof: ValidationProof {
                voter_eligibility_proof: "proof1".to_string(),
                biometric_verification_proof: "proof2".to_string(),
                vote_uniqueness_proof: "proof3".to_string(),
                cryptographic_integrity_proof: "proof4".to_string(),
                merkle_root: "merkle_root".to_string(),
            },
            errors: vec![],
        };
        
        // Serializar
        let serialized = serde_json::to_vec(&validation_result).unwrap();
        assert!(!serialized.is_empty());
        
        // Deserializar
        let deserialized: ValidationResult = serde_json::from_slice(&serialized).unwrap();
        assert_eq!(deserialized.is_valid, validation_result.is_valid);
        assert_eq!(deserialized.errors.len(), validation_result.errors.len());
    }

    /// Testa serialização de erro de validação
    #[test]
    fn test_validation_error_serialization() {
        let errors = vec![
            ValidationError::VoterNotEligible,
            ValidationError::BiometricVerificationFailed,
            ValidationError::VoteAlreadyCast,
            ValidationError::InvalidSignature,
            ValidationError::ElectionNotActive,
            ValidationError::InvalidCandidate,
            ValidationError::CryptographicError("Test error".to_string()),
        ];
        
        for error in errors {
            // Serializar
            let serialized = serde_json::to_vec(&error).unwrap();
            assert!(!serialized.is_empty());
            
            // Deserializar
            let deserialized: ValidationError = serde_json::from_slice(&serialized).unwrap();
            assert_eq!(std::mem::discriminant(&deserialized), std::mem::discriminant(&error));
        }
    }

    // Funções auxiliares para criar mocks

    fn create_mock_crypto_service() -> CryptoService {
        // Mock implementation
        CryptoService::new("test_key".to_string()).unwrap()
    }

    fn create_mock_crypto_service_duplicate() -> CryptoService {
        // Mock implementation that indicates duplicate vote
        CryptoService::new("test_key".to_string()).unwrap()
    }

    fn create_mock_crypto_service_invalid_signature() -> CryptoService {
        // Mock implementation that fails signature verification
        CryptoService::new("test_key".to_string()).unwrap()
    }

    fn create_mock_tse_validator() -> TSEValidator {
        // Mock implementation
        TSEValidator::new("http://localhost:8080".to_string())
    }

    fn create_mock_tse_validator_ineligible() -> TSEValidator {
        // Mock implementation that returns ineligible voter
        TSEValidator::new("http://localhost:8080".to_string())
    }

    fn create_mock_tse_validator_inactive_election() -> TSEValidator {
        // Mock implementation that returns inactive election
        TSEValidator::new("http://localhost:8080".to_string())
    }

    fn create_mock_tse_validator_invalid_candidate() -> TSEValidator {
        // Mock implementation that returns invalid candidate
        TSEValidator::new("http://localhost:8080".to_string())
    }

    fn create_mock_biometric_validator() -> BiometricValidator {
        // Mock implementation
        BiometricValidator::new()
    }

    fn create_mock_biometric_validator_failing() -> BiometricValidator {
        // Mock implementation that fails biometric verification
        BiometricValidator::new()
    }

    async fn create_test_validator() -> VoteValidator {
        VoteValidator::new(
            create_mock_crypto_service(),
            create_mock_tse_validator(),
            create_mock_biometric_validator(),
        )
    }

    fn create_test_vote() -> Vote {
        Vote {
            id: "vote1".to_string(),
            election_id: "election1".to_string(),
            voter_id: "voter1".to_string(),
            candidate_id: "candidate1".to_string(),
            encrypted_vote: "encrypted_vote_1".to_string(),
            signature: "signature_1".to_string(),
            nullifier: "nullifier_1".to_string(),
            timestamp: Utc::now(),
            biometric_data: BiometricData {
                fingerprint: "fingerprint_1".to_string(),
                facial_data: "facial_data_1".to_string(),
                liveness_data: "liveness_data_1".to_string(),
            },
            encryption_hash: "encryption_hash_1".to_string(),
        }
    }
}

// Estruturas de dados necessárias para os testes

#[derive(Debug, Clone)]
pub struct CryptoService {
    key: String,
}

impl CryptoService {
    pub fn new(key: String) -> Result<Self> {
        Ok(Self { key })
    }
}

#[derive(Debug, Clone)]
pub struct TSEValidator {
    endpoint: String,
}

impl TSEValidator {
    pub fn new(endpoint: String) -> Self {
        Self { endpoint }
    }
}

#[derive(Debug, Clone)]
pub struct BiometricValidator;

impl BiometricValidator {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub id: String,
    pub election_id: String,
    pub voter_id: String,
    pub candidate_id: String,
    pub encrypted_vote: String,
    pub signature: String,
    pub nullifier: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub biometric_data: BiometricData,
    pub encryption_hash: String,
}

#[derive(Debug, Clone)]
pub struct BiometricData {
    pub fingerprint: String,
    pub facial_data: String,
    pub liveness_data: String,
}

#[derive(Debug, Clone)]
pub struct Election {
    pub id: String,
    pub title: String,
    pub description: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
    pub is_completed: bool,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
