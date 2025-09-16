//! Testes unitários para sistema de threshold signatures
//! 
//! Testa funcionalidades de assinaturas distribuídas para consenso
//! sem dependência de blockchain.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consensus::threshold_signatures::*;
    use chrono::Utc;
    use rand::rngs::OsRng;

    /// Testa criação do sistema de threshold signatures
    #[test]
    fn test_threshold_system_creation() {
        let system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            3, // threshold
            5  // total nodes
        );
        
        let stats = system.get_system_stats();
        assert_eq!(stats.total_nodes, 0);
        assert_eq!(stats.active_nodes, 0);
        assert_eq!(stats.threshold, 3);
        assert_eq!(stats.total_signatures, 0);
    }

    /// Testa geração de chaves threshold
    #[test]
    fn test_threshold_key_generation() {
        let system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            3,
            5
        );
        
        let (private_key, public_key) = system.generate_threshold_keys().unwrap();
        
        assert_eq!(private_key.node_id, "node1");
        assert_eq!(private_key.threshold, 3);
        assert_eq!(private_key.total_nodes, 5);
        assert_eq!(public_key.node_id, "node1");
        assert_eq!(public_key.threshold, 3);
        assert_eq!(public_key.total_nodes, 5);
        assert!(!public_key.public_key.is_empty());
    }

    /// Testa criação de share de assinatura
    #[test]
    fn test_signature_share_creation() {
        let system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            3,
            5
        );
        
        let (private_key, _) = system.generate_threshold_keys().unwrap();
        let message = b"test message";
        
        let share = system.create_signature_share(&private_key, message).unwrap();
        
        assert_eq!(share.node_id, "node1");
        assert!(!share.share.is_empty());
        assert!(!share.proof.is_empty());
        assert!(share.timestamp <= Utc::now());
    }

    /// Testa adição de nós ao sistema
    #[test]
    fn test_add_nodes() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            3,
            5
        );
        
        // Adicionar nós
        for i in 1..=4 {
            let node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 3,
                    total_nodes: 5,
                },
                is_active: true,
                trust_score: 0.8 + (i as f64 * 0.05),
                last_seen: Utc::now(),
            };
            
            let result = system.add_node(node);
            assert!(result.is_ok());
        }
        
        let stats = system.get_system_stats();
        assert_eq!(stats.total_nodes, 4);
        assert_eq!(stats.active_nodes, 4);
    }

    /// Testa coleta de threshold signature
    #[tokio::test]
    async fn test_collect_threshold_signature() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            2, // threshold baixo para teste
            3
        );
        
        // Adicionar nós ativos
        for i in 1..=3 {
            let node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 2,
                    total_nodes: 3,
                },
                is_active: true,
                trust_score: 0.9,
                last_seen: Utc::now(),
            };
            
            system.add_node(node).unwrap();
        }
        
        let message = b"test message for threshold signature";
        let result = system.collect_threshold_signature(message, Some(2)).await;
        
        assert!(result.is_ok());
        
        let threshold_sig = result.unwrap();
        assert_eq!(threshold_sig.message, message);
        assert!(!threshold_sig.signature.is_empty());
        assert!(threshold_sig.participating_nodes.len() >= 2);
        assert_eq!(threshold_sig.threshold, 2);
        assert_eq!(threshold_sig.total_nodes, 3);
    }

    /// Testa verificação de threshold signature
    #[tokio::test]
    async fn test_verify_threshold_signature() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            2,
            3
        );
        
        // Adicionar nós
        for i in 1..=3 {
            let node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 2,
                    total_nodes: 3,
                },
                is_active: true,
                trust_score: 0.9,
                last_seen: Utc::now(),
            };
            
            system.add_node(node).unwrap();
        }
        
        let message = b"test message for verification";
        let threshold_sig = system.collect_threshold_signature(message, Some(2)).await.unwrap();
        
        // Verificar assinatura
        let is_valid = system.verify_threshold_signature(&threshold_sig).unwrap();
        assert!(is_valid);
    }

    /// Testa threshold signature com nós insuficientes
    #[tokio::test]
    async fn test_insufficient_nodes() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            3, // threshold alto
            5
        );
        
        // Adicionar apenas 2 nós (insuficiente para threshold 3)
        for i in 1..=2 {
            let node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 3,
                    total_nodes: 5,
                },
                is_active: true,
                trust_score: 0.9,
                last_seen: Utc::now(),
            };
            
            system.add_node(node).unwrap();
        }
        
        let message = b"test message";
        let result = system.collect_threshold_signature(message, Some(3)).await;
        
        // Deve falhar por nós insuficientes
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Insufficient signature shares"));
    }

    /// Testa atualização de status de nós
    #[test]
    fn test_update_node_status() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            2,
            3
        );
        
        // Adicionar nó
        let node = ThresholdNode {
            id: "node2".to_string(),
            name: "Node 2".to_string(),
            public_key: ThresholdPublicKey {
                node_id: "node2".to_string(),
                public_key: vec![2; 32],
                threshold: 2,
                total_nodes: 3,
            },
            is_active: true,
            trust_score: 0.9,
            last_seen: Utc::now(),
        };
        
        system.add_node(node).unwrap();
        
        // Verificar que nó está ativo
        let active_nodes = system.get_active_nodes();
        assert_eq!(active_nodes.len(), 1);
        
        // Desativar nó
        let result = system.update_node_status("node2", false);
        assert!(result.is_ok());
        
        // Verificar que nó não está mais ativo
        let active_nodes = system.get_active_nodes();
        assert_eq!(active_nodes.len(), 0);
    }

    /// Testa serviço de consenso
    #[tokio::test]
    async fn test_consensus_service() {
        let mut service = ConsensusService::new(
            "node1".to_string(),
            2,
            3
        );
        
        // Inicializar serviço
        service.initialize().await.unwrap();
        
        // Adicionar nó remoto
        let remote_node = ThresholdNode {
            id: "node2".to_string(),
            name: "Remote Node".to_string(),
            public_key: ThresholdPublicKey {
                node_id: "node2".to_string(),
                public_key: vec![2; 32],
                threshold: 2,
                total_nodes: 3,
            },
            is_active: true,
            trust_score: 0.9,
            last_seen: Utc::now(),
        };
        
        service.add_remote_node(remote_node).unwrap();
        
        // Criar evento eleitoral
        let event = ElectionEvent {
            id: "event1".to_string(),
            event_type: "vote_cast".to_string(),
            election_id: "election1".to_string(),
            data: serde_json::json!({
                "voter_id": "voter1",
                "candidate_id": "candidate1"
            }),
            timestamp: Utc::now(),
            source: "urna1".to_string(),
        };
        
        // Criar consenso
        let result = service.create_election_consensus(&event).await;
        assert!(result.is_ok());
        
        let threshold_sig = result.unwrap();
        assert_eq!(threshold_sig.message.len(), serde_json::to_vec(&event).unwrap().len());
    }

    /// Testa verificação de consenso
    #[tokio::test]
    async fn test_consensus_verification() {
        let mut service = ConsensusService::new(
            "node1".to_string(),
            2,
            3
        );
        
        service.initialize().await.unwrap();
        
        // Adicionar nó remoto
        let remote_node = ThresholdNode {
            id: "node2".to_string(),
            name: "Remote Node".to_string(),
            public_key: ThresholdPublicKey {
                node_id: "node2".to_string(),
                public_key: vec![2; 32],
                threshold: 2,
                total_nodes: 3,
            },
            is_active: true,
            trust_score: 0.9,
            last_seen: Utc::now(),
        };
        
        service.add_remote_node(remote_node).unwrap();
        
        let event = ElectionEvent {
            id: "event1".to_string(),
            event_type: "election_created".to_string(),
            election_id: "election1".to_string(),
            data: serde_json::json!({"title": "Test Election"}),
            timestamp: Utc::now(),
            source: "tse".to_string(),
        };
        
        // Criar consenso
        let threshold_sig = service.create_election_consensus(&event).await.unwrap();
        
        // Verificar consenso
        let is_valid = service.verify_election_consensus(&threshold_sig).unwrap();
        assert!(is_valid);
    }

    /// Testa estatísticas do consenso
    #[tokio::test]
    async fn test_consensus_stats() {
        let mut service = ConsensusService::new(
            "node1".to_string(),
            2,
            3
        );
        
        service.initialize().await.unwrap();
        
        // Adicionar nós remotos
        for i in 2..=3 {
            let remote_node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Remote Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 2,
                    total_nodes: 3,
                },
                is_active: true,
                trust_score: 0.9,
                last_seen: Utc::now(),
            };
            
            service.add_remote_node(remote_node).unwrap();
        }
        
        let stats = service.get_consensus_stats();
        assert_eq!(stats.total_nodes, 3);
        assert_eq!(stats.active_nodes, 3);
        assert_eq!(stats.threshold, 2);
    }

    /// Testa performance com múltiplas assinaturas
    #[tokio::test]
    async fn test_performance_multiple_signatures() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            2,
            5
        );
        
        // Adicionar nós
        for i in 1..=5 {
            let node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 2,
                    total_nodes: 5,
                },
                is_active: true,
                trust_score: 0.9,
                last_seen: Utc::now(),
            };
            
            system.add_node(node).unwrap();
        }
        
        let start_time = std::time::Instant::now();
        
        // Criar 100 assinaturas threshold
        for i in 1..=100 {
            let message = format!("test message {}", i).into_bytes();
            let result = system.collect_threshold_signature(&message, Some(2)).await;
            assert!(result.is_ok());
        }
        
        let duration = start_time.elapsed();
        
        // Verificar que foi rápido (< 5 segundos para 100 assinaturas)
        assert!(duration.as_secs() < 5, "Performance test failed: took {} seconds", duration.as_secs());
        
        println!("Performance test: 100 threshold signatures in {:?}", duration);
    }

    /// Testa tolerância a falhas de nós
    #[tokio::test]
    async fn test_node_failure_tolerance() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            2, // threshold 2
            5  // total 5 nós
        );
        
        // Adicionar 5 nós
        for i in 1..=5 {
            let node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 2,
                    total_nodes: 5,
                },
                is_active: i <= 3, // Apenas 3 nós ativos
                trust_score: 0.9,
                last_seen: Utc::now(),
            };
            
            system.add_node(node).unwrap();
        }
        
        let message = b"test message with node failures";
        
        // Deve conseguir criar assinatura mesmo com alguns nós falhando
        let result = system.collect_threshold_signature(message, Some(2)).await;
        assert!(result.is_ok());
        
        let threshold_sig = result.unwrap();
        assert!(threshold_sig.participating_nodes.len() >= 2);
    }

    /// Testa assinatura com threshold dinâmico
    #[tokio::test]
    async fn test_dynamic_threshold() {
        let mut system = ThresholdSignatureSystem::new(
            "node1".to_string(),
            2,
            5
        );
        
        // Adicionar nós
        for i in 1..=5 {
            let node = ThresholdNode {
                id: format!("node{}", i),
                name: format!("Node {}", i),
                public_key: ThresholdPublicKey {
                    node_id: format!("node{}", i),
                    public_key: vec![i as u8; 32],
                    threshold: 2,
                    total_nodes: 5,
                },
                is_active: true,
                trust_score: 0.9,
                last_seen: Utc::now(),
            };
            
            system.add_node(node).unwrap();
        }
        
        let message = b"test message with dynamic threshold";
        
        // Testar com diferentes thresholds
        for threshold in 2..=4 {
            let result = system.collect_threshold_signature(message, Some(threshold)).await;
            assert!(result.is_ok());
            
            let threshold_sig = result.unwrap();
            assert!(threshold_sig.participating_nodes.len() >= threshold);
        }
    }
}
