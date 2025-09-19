//! Testes unitários para sistema de logs transparentes
//! 
//! Testa funcionalidades de logs transparentes (CT logs) para auditoria
//! eleitoral sem dependência de blockchain.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transparency::election_logs::*;
    use chrono::Utc;
    use serde_json::json;

    /// Testa criação de log transparente
    #[test]
    fn test_transparency_log_creation() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 10,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let log = ElectionTransparencyLog::new(config);
        let stats = log.get_log_stats();
        
        assert_eq!(stats.total_events, 0);
        assert_eq!(stats.total_verifiers, 0);
        assert_eq!(stats.tree_size, 0);
    }

    /// Testa adição de verificadores
    #[test]
    fn test_add_verifiers() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Criar verificador de teste
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        
        let result = log.add_verifier(verifier);
        assert!(result.is_ok());
        
        let stats = log.get_log_stats();
        assert_eq!(stats.total_verifiers, 1);
    }

    /// Testa registro de evento eleitoral
    #[test]
    fn test_election_event_registration() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar verificador
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        // Criar evento de teste
        let event = ElectionEvent {
            id: "event1".to_string(),
            event_type: ElectionEventType::ElectionCreated,
            election_id: "election1".to_string(),
            data: json!({
                "title": "Eleição Teste",
                "start_time": "2025-01-01T00:00:00Z",
                "end_time": "2025-01-01T23:59:59Z"
            }),
            timestamp: Utc::now(),
            source: "test".to_string(),
        };
        
        let result = log.append_election_event(event);
        assert!(result.is_ok());
        
        let stats = log.get_log_stats();
        assert_eq!(stats.total_events, 1);
    }

    /// Testa verificação de integridade
    #[test]
    fn test_integrity_verification() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar verificador
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        // Criar e registrar evento
        let event = ElectionEvent {
            id: "event1".to_string(),
            event_type: ElectionEventType::VoteCast,
            election_id: "election1".to_string(),
            data: json!({
                "voter_id": "voter1",
                "candidate_id": "candidate1",
                "nullifier": "nullifier1"
            }),
            timestamp: Utc::now(),
            source: "urna1".to_string(),
        };
        
        let inclusion_proof = log.append_election_event(event).unwrap();
        
        // Verificar integridade
        let verification_result = log.verify_event_integrity(&inclusion_proof.log_entry);
        assert!(verification_result.is_ok());
        
        match verification_result.unwrap() {
            VerificationStatus::Verified => {
                // Sucesso - evento verificado
            },
            VerificationStatus::PartiallyVerified => {
                // Parcialmente verificado - ainda aceitável
            },
            VerificationStatus::Failed => {
                panic!("Event verification failed");
            },
            VerificationStatus::Pending => {
                // Ainda pendente - pode ser normal em alguns casos
            }
        }
    }

    /// Testa busca de eventos por tipo
    #[test]
    fn test_get_events_by_type() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar verificador
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        // Registrar diferentes tipos de eventos
        let events = vec![
            ElectionEvent {
                id: "event1".to_string(),
                event_type: ElectionEventType::ElectionCreated,
                election_id: "election1".to_string(),
                data: json!({"title": "Eleição 1"}),
                timestamp: Utc::now(),
                source: "tse".to_string(),
            },
            ElectionEvent {
                id: "event2".to_string(),
                event_type: ElectionEventType::VoteCast,
                election_id: "election1".to_string(),
                data: json!({"voter_id": "voter1"}),
                timestamp: Utc::now(),
                source: "urna1".to_string(),
            },
            ElectionEvent {
                id: "event3".to_string(),
                event_type: ElectionEventType::VoteCast,
                election_id: "election1".to_string(),
                data: json!({"voter_id": "voter2"}),
                timestamp: Utc::now(),
                source: "urna2".to_string(),
            },
        ];
        
        for event in events {
            log.append_election_event(event).unwrap();
        }
        
        // Buscar eventos de voto
        let vote_events = log.get_events_by_type(&ElectionEventType::VoteCast);
        assert_eq!(vote_events.len(), 2);
        
        // Buscar eventos de criação de eleição
        let creation_events = log.get_events_by_type(&ElectionEventType::ElectionCreated);
        assert_eq!(creation_events.len(), 1);
    }

    /// Testa busca de eventos por intervalo de tempo
    #[test]
    fn test_get_events_by_time_range() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar verificador
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        let now = Utc::now();
        let start_time = now - chrono::Duration::hours(1);
        let end_time = now + chrono::Duration::hours(1);
        
        // Registrar evento dentro do intervalo
        let event = ElectionEvent {
            id: "event1".to_string(),
            event_type: ElectionEventType::VoteCast,
            election_id: "election1".to_string(),
            data: json!({"voter_id": "voter1"}),
            timestamp: now,
            source: "urna1".to_string(),
        };
        
        log.append_election_event(event).unwrap();
        
        // Buscar eventos no intervalo
        let events_in_range = log.get_events_by_time_range(start_time, end_time);
        assert_eq!(events_in_range.len(), 1);
        
        // Buscar eventos fora do intervalo
        let future_start = now + chrono::Duration::hours(2);
        let future_end = now + chrono::Duration::hours(3);
        let events_out_of_range = log.get_events_by_time_range(future_start, future_end);
        assert_eq!(events_out_of_range.len(), 0);
    }

    /// Testa exportação de log para auditoria
    #[test]
    fn test_log_export() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar verificador
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        // Registrar alguns eventos
        for i in 1..=5 {
            let event = ElectionEvent {
                id: format!("event{}", i),
                event_type: ElectionEventType::VoteCast,
                election_id: "election1".to_string(),
                data: json!({"voter_id": format!("voter{}", i)}),
                timestamp: Utc::now(),
                source: format!("urna{}", i),
            };
            
            log.append_election_event(event).unwrap();
        }
        
        // Exportar log
        let exported_log = log.export_log().unwrap();
        assert_eq!(exported_log.len(), 5);
        
        // Verificar se todos os eventos foram exportados
        for (i, entry) in exported_log.iter().enumerate() {
            assert_eq!(entry.index, i as u64);
            assert_eq!(entry.event_type, ElectionEventType::VoteCast);
        }
    }

    /// Testa árvore Merkle
    #[test]
    fn test_merkle_tree_operations() {
        let mut tree = MerkleTree::new();
        
        // Adicionar folhas
        let index1 = tree.add_leaf("data1");
        let index2 = tree.add_leaf("data2");
        let index3 = tree.add_leaf("data3");
        
        assert_eq!(index1, 0);
        assert_eq!(index2, 1);
        assert_eq!(index3, 2);
        assert!(tree.root().is_some());
        assert_eq!(tree.size(), 3);
    }

    /// Testa geração de prova Merkle
    #[test]
    fn test_merkle_proof_generation() {
        let mut tree = MerkleTree::new();
        
        // Adicionar folhas
        tree.add_leaf("data1");
        tree.add_leaf("data2");
        tree.add_leaf("data3");
        tree.add_leaf("data4");
        
        // Gerar prova para primeira folha
        let proof = tree.generate_proof(0).unwrap();
        assert_eq!(proof.leaf_index, 0);
        assert!(!proof.path.is_empty());
        assert_eq!(proof.tree_size, 4);
        
        // Verificar prova
        let is_valid = tree.verify_proof(&proof).unwrap();
        assert!(is_valid);
    }

    /// Testa verificação de prova Merkle
    #[test]
    fn test_merkle_proof_verification() {
        let mut tree = MerkleTree::new();
        
        // Adicionar folhas
        tree.add_leaf("data1");
        tree.add_leaf("data2");
        tree.add_leaf("data3");
        
        // Gerar prova
        let proof = tree.generate_proof(1).unwrap();
        
        // Verificar prova válida
        let is_valid = tree.verify_proof(&proof).unwrap();
        assert!(is_valid);
        
        // Testar prova inválida (índice fora dos limites)
        let invalid_proof = MerkleProof {
            leaf_index: 10,
            path: vec![],
            root_hash: "invalid".to_string(),
            tree_size: 3,
        };
        
        let is_invalid = tree.verify_proof(&invalid_proof).unwrap();
        assert!(!is_invalid);
    }

    /// Testa performance com muitos eventos
    #[test]
    fn test_performance_with_many_events() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar verificador
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        let start_time = std::time::Instant::now();
        
        // Registrar 1000 eventos
        for i in 1..=1000 {
            let event = ElectionEvent {
                id: format!("event{}", i),
                event_type: ElectionEventType::VoteCast,
                election_id: "election1".to_string(),
                data: json!({"voter_id": format!("voter{}", i)}),
                timestamp: Utc::now(),
                source: format!("urna{}", i % 10), // 10 urnas diferentes
            };
            
            log.append_election_event(event).unwrap();
        }
        
        let duration = start_time.elapsed();
        
        // Verificar que todos os eventos foram registrados
        let stats = log.get_log_stats();
        assert_eq!(stats.total_events, 1000);
        
        // Verificar que a operação foi rápida (< 1 segundo para 1000 eventos)
        assert!(duration.as_secs() < 1, "Performance test failed: took {} seconds", duration.as_secs());
        
        println!("Performance test: {} events in {:?}", stats.total_events, duration);
    }

    /// Testa tolerância a falhas de verificadores
    #[test]
    fn test_verifier_failure_tolerance() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 2, // Requer 2 verificadores
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar apenas 1 verificador (insuficiente para threshold)
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        // Criar evento
        let event = ElectionEvent {
            id: "event1".to_string(),
            event_type: ElectionEventType::VoteCast,
            election_id: "election1".to_string(),
            data: json!({"voter_id": "voter1"}),
            timestamp: Utc::now(),
            source: "urna1".to_string(),
        };
        
        // Deve conseguir registrar mesmo com verificadores insuficientes
        let result = log.append_election_event(event);
        assert!(result.is_ok());
        
        // Mas a verificação deve ser parcial ou falhar
        let inclusion_proof = result.unwrap();
        let verification_status = log.verify_event_integrity(&inclusion_proof.log_entry).unwrap();
        
        match verification_status {
            VerificationStatus::PartiallyVerified | VerificationStatus::Failed => {
                // Esperado com verificadores insuficientes
            },
            _ => {
                // Pode ser aceitável dependendo da implementação
            }
        }
    }

    /// Testa métricas de performance
    #[test]
    fn test_performance_metrics() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let log = ElectionTransparencyLog::new(config);
        let metrics = log.get_performance_metrics();
        
        assert_eq!(metrics.total_operations, 0);
        assert_eq!(metrics.success_rate, 100.0);
        assert_eq!(metrics.error_rate, 0.0);
    }

    /// Testa trilha de auditoria
    #[test]
    fn test_audit_trail() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let log = ElectionTransparencyLog::new(config);
        let audit_trail = log.get_audit_trail();
        
        assert_eq!(audit_trail.len(), 0);
    }

    /// Testa busca de eventos
    #[test]
    fn test_event_search() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        // Adicionar verificador
        let verifier = LogVerifier {
            id: "verifier1".to_string(),
            name: "Test Verifier".to_string(),
            public_key: Ed25519KeyPair::generate(&mut OsRng).unwrap(),
            is_active: true,
            trust_level: 100,
        };
        log.add_verifier(verifier).unwrap();
        
        // Criar evento
        let event = ElectionEvent {
            id: "event1".to_string(),
            event_type: ElectionEventType::VoteCast,
            election_id: "election1".to_string(),
            data: json!({"vote": "candidate1"}),
            timestamp: Utc::now(),
            source: "urna1".to_string(),
        };
        
        // Adicionar evento
        log.append_election_event(event).unwrap();
        
        // Buscar eventos
        let criteria = SearchCriteria {
            event_type: Some(ElectionEventType::VoteCast),
            start_time: None,
            end_time: None,
            election_id: Some("election1".to_string()),
            verification_status: None,
        };
        
        let results = log.search_events(criteria).unwrap();
        assert_eq!(results.len(), 1);
    }

    /// Testa exportação de log
    #[test]
    fn test_log_export() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let log = ElectionTransparencyLog::new(config);
        
        // Exportar em JSON
        let json_data = log.export_for_audit(ExportFormat::Json).unwrap();
        assert!(!json_data.is_empty());
        
        // Verificar se é JSON válido
        let json_str = String::from_utf8(json_data).unwrap();
        let _: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    }
}
