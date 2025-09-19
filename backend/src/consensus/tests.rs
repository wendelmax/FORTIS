//! Testes para Sistema de Consenso Distribuído
//! 
//! Testa funcionalidades de threshold signatures e consenso distribuído
//! para auditoria eleitoral sem dependência de blockchain.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::*;
    use crate::transparency::election_logs::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use chrono::Duration;

    /// Testa criação do serviço de threshold signatures
    #[test]
    fn test_threshold_service_creation() {
        let config = ThresholdConfig::default();
        let service = ThresholdSignatureService::new(config);
        let stats = service.get_stats();
        
        assert_eq!(stats.total_nodes, 0);
        assert_eq!(stats.threshold, 2);
        assert_eq!(stats.consensus_rate, 0.0);
    }

    /// Testa adição de nós
    #[test]
    fn test_add_nodes() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig::default());
        
        // Adicionar 3 nós
        for i in 1..=3 {
            let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
            let node = ConsensusNode {
                id: format!("node_{}", i),
                name: format!("Node {}", i),
                public_key,
                is_active: true,
                trust_level: 100,
                last_seen: Utc::now(),
                signature_count: 0,
            };
            
            service.add_node(node, key_pair).unwrap();
        }
        
        let stats = service.get_stats();
        assert_eq!(stats.total_nodes, 3);
        assert_eq!(stats.active_nodes, 3);
    }

    /// Testa criação de requisição de assinatura
    #[test]
    fn test_create_signature_request() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig::default());
        
        // Adicionar nó
        let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
        let node = ConsensusNode {
            id: "node1".to_string(),
            name: "Node 1".to_string(),
            public_key,
            is_active: true,
            trust_level: 100,
            last_seen: Utc::now(),
            signature_count: 0,
        };
        service.add_node(node, key_pair).unwrap();
        
        let request = SignatureRequest {
            id: "req1".to_string(),
            message: "Test message".to_string(),
            message_hash: service.hash_message("Test message"),
            requester_id: "user1".to_string(),
            priority: SignaturePriority::Normal,
            expires_at: Utc::now() + Duration::minutes(10),
            metadata: std::collections::HashMap::new(),
        };
        
        let result = service.create_signature_request(request);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "req1");
    }

    /// Testa processo de consenso completo
    #[test]
    fn test_consensus_process() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig {
            total_nodes: 3,
            threshold: 2,
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        });
        
        // Adicionar 3 nós
        for i in 1..=3 {
            let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
            let node = ConsensusNode {
                id: format!("node_{}", i),
                name: format!("Node {}", i),
                public_key,
                is_active: true,
                trust_level: 100,
                last_seen: Utc::now(),
                signature_count: 0,
            };
            
            service.add_node(node, key_pair).unwrap();
        }
        
        // Criar requisição
        let request = SignatureRequest {
            id: "consensus_test".to_string(),
            message: "Consensus test message".to_string(),
            message_hash: service.hash_message("Consensus test message"),
            requester_id: "admin".to_string(),
            priority: SignaturePriority::High,
            expires_at: Utc::now() + Duration::minutes(10),
            metadata: std::collections::HashMap::new(),
        };
        
        service.create_signature_request(request).unwrap();
        
        // Processar consenso
        let threshold_signature = service.collect_signatures("consensus_test").unwrap();
        
        assert_eq!(threshold_signature.id, "consensus_test");
        assert!(threshold_signature.threshold_met);
        assert_eq!(threshold_signature.signatures.len(), 3);
    }

    /// Testa validação de configuração
    #[test]
    fn test_config_validation() {
        // Configuração válida
        let valid_config = ThresholdConfig {
            total_nodes: 5,
            threshold: 3,
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        };
        assert!(ThresholdUtils::validate_config(&valid_config).is_ok());
        
        // Configuração inválida - threshold maior que total de nós
        let invalid_config = ThresholdConfig {
            total_nodes: 3,
            threshold: 5,
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        };
        assert!(ThresholdUtils::validate_config(&invalid_config).is_err());
        
        // Configuração inválida - threshold zero
        let invalid_config2 = ThresholdConfig {
            total_nodes: 3,
            threshold: 0,
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        };
        assert!(ThresholdUtils::validate_config(&invalid_config2).is_err());
    }

    /// Testa cálculo de threshold ótimo
    #[test]
    fn test_optimal_threshold_calculation() {
        assert_eq!(ThresholdUtils::calculate_optimal_threshold(9), 7); // 2/3 de 9 + 1
        assert_eq!(ThresholdUtils::calculate_optimal_threshold(6), 5); // 2/3 de 6 + 1
        assert_eq!(ThresholdUtils::calculate_optimal_threshold(3), 3); // 2/3 de 3 + 1
    }

    /// Testa verificação de segurança do threshold
    #[test]
    fn test_threshold_security() {
        assert!(ThresholdUtils::is_threshold_secure(9, 7)); // Seguro
        assert!(!ThresholdUtils::is_threshold_secure(9, 2)); // Inseguro
        assert!(ThresholdUtils::is_threshold_secure(6, 4)); // Seguro
        assert!(!ThresholdUtils::is_threshold_secure(6, 1)); // Inseguro
    }

    /// Testa geração de par de chaves
    #[test]
    fn test_key_pair_generation() {
        let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
        
        // Verificar que a chave pública é válida
        assert!(!public_key.is_empty());
        assert!(hex::decode(&public_key).is_ok());
        
        // Verificar que o par de chaves é válido
        let test_message = "test message";
        let signature = key_pair.sign(test_message.as_bytes());
        assert!(!signature.as_ref().is_empty());
    }

    /// Testa limpeza de requisições expiradas
    #[test]
    fn test_cleanup_expired() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig::default());
        
        // Adicionar nó
        let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
        let node = ConsensusNode {
            id: "node1".to_string(),
            name: "Node 1".to_string(),
            public_key,
            is_active: true,
            trust_level: 100,
            last_seen: Utc::now(),
            signature_count: 0,
        };
        service.add_node(node, key_pair).unwrap();
        
        // Criar requisição expirada
        let expired_request = SignatureRequest {
            id: "expired_req".to_string(),
            message: "Expired message".to_string(),
            message_hash: service.hash_message("Expired message"),
            requester_id: "user1".to_string(),
            priority: SignaturePriority::Normal,
            expires_at: Utc::now() - Duration::minutes(1), // Expirada
            metadata: std::collections::HashMap::new(),
        };
        
        service.create_signature_request(expired_request).unwrap();
        
        // Verificar que a requisição foi adicionada
        let stats_before = service.get_stats();
        assert_eq!(stats_before.total_requests, 1);
        
        // Limpar expiradas
        let removed = service.cleanup_expired();
        assert_eq!(removed, 1);
        
        // Verificar que a requisição foi removida
        let stats_after = service.get_stats();
        assert_eq!(stats_after.total_requests, 0);
    }

    /// Testa verificação de assinatura
    #[test]
    fn test_signature_verification() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig::default());
        
        // Adicionar nó
        let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
        let node = ConsensusNode {
            id: "node1".to_string(),
            name: "Node 1".to_string(),
            public_key: public_key.clone(),
            is_active: true,
            trust_level: 100,
            last_seen: Utc::now(),
            signature_count: 0,
        };
        service.add_node(node, key_pair).unwrap();
        
        // Criar requisição
        let request = SignatureRequest {
            id: "verify_test".to_string(),
            message: "Verification test".to_string(),
            message_hash: service.hash_message("Verification test"),
            requester_id: "user1".to_string(),
            priority: SignaturePriority::Normal,
            expires_at: Utc::now() + Duration::minutes(10),
            metadata: std::collections::HashMap::new(),
        };
        
        service.create_signature_request(request).unwrap();
        
        // Assinar mensagem
        let node_signature = service.sign_message("node1", "verify_test").unwrap();
        
        // Verificar assinatura
        let is_valid = service.verify_signature(&node_signature).unwrap();
        assert!(is_valid);
    }

    /// Testa tolerância a falhas
    #[test]
    fn test_fault_tolerance() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig {
            total_nodes: 5,
            threshold: 3, // Requer 3 de 5 nós
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        });
        
        // Adicionar 5 nós
        for i in 1..=5 {
            let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
            let node = ConsensusNode {
                id: format!("node_{}", i),
                name: format!("Node {}", i),
                public_key,
                is_active: i <= 3, // Apenas 3 nós ativos
                trust_level: 100,
                last_seen: Utc::now(),
                signature_count: 0,
            };
            
            service.add_node(node, key_pair).unwrap();
        }
        
        // Criar requisição
        let request = SignatureRequest {
            id: "fault_tolerance_test".to_string(),
            message: "Fault tolerance test".to_string(),
            message_hash: service.hash_message("Fault tolerance test"),
            requester_id: "admin".to_string(),
            priority: SignaturePriority::High,
            expires_at: Utc::now() + Duration::minutes(10),
            metadata: std::collections::HashMap::new(),
        };
        
        service.create_signature_request(request).unwrap();
        
        // Processar consenso (deve funcionar com 3 nós ativos)
        let threshold_signature = service.collect_signatures("fault_tolerance_test").unwrap();
        
        assert!(threshold_signature.threshold_met);
        assert_eq!(threshold_signature.signatures.len(), 3);
    }

    /// Testa performance com múltiplas requisições
    #[test]
    fn test_performance_multiple_requests() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig {
            total_nodes: 3,
            threshold: 2,
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        });
        
        // Adicionar nós
        for i in 1..=3 {
            let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
            let node = ConsensusNode {
                id: format!("node_{}", i),
                name: format!("Node {}", i),
                public_key,
                is_active: true,
                trust_level: 100,
                last_seen: Utc::now(),
                signature_count: 0,
            };
            
            service.add_node(node, key_pair).unwrap();
        }
        
        // Processar 100 requisições
        for i in 1..=100 {
            let request = SignatureRequest {
                id: format!("perf_test_{}", i),
                message: format!("Performance test message {}", i),
                message_hash: service.hash_message(&format!("Performance test message {}", i)),
                requester_id: "admin".to_string(),
                priority: SignaturePriority::Normal,
                expires_at: Utc::now() + Duration::minutes(10),
                metadata: std::collections::HashMap::new(),
            };
            
            service.create_signature_request(request).unwrap();
            service.collect_signatures(&format!("perf_test_{}", i)).unwrap();
        }
        
        let stats = service.get_stats();
        assert_eq!(stats.total_requests, 100);
        assert_eq!(stats.completed_requests, 100);
        assert_eq!(stats.consensus_rate, 100.0);
    }
}