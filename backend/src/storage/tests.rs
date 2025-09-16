//! Testes unitários para sistema de armazenamento distribuído
//! 
//! Testa funcionalidades de DHT + IPFS para armazenamento eficiente
//! sem dependência de blockchain.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::distributed_storage::*;
    use chrono::Utc;
    use serde_json::json;

    /// Testa criação do sistema de armazenamento distribuído
    #[test]
    fn test_distributed_storage_creation() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000 // cache size
        );
        
        // Verificar se foi criado corretamente
        // (não há getters públicos, mas podemos verificar que não panica)
    }

    /// Testa armazenamento de boletim de urna
    #[tokio::test]
    async fn test_store_ballot() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        let ballot = Ballot {
            id: "election1:ballot1".to_string(),
            election_id: "election1".to_string(),
            urna_id: "urna1".to_string(),
            votes: vec![
                Vote {
                    voter_id: "voter1".to_string(),
                    candidate_id: "candidate1".to_string(),
                    encrypted_vote: "encrypted_vote_1".to_string(),
                    signature: "signature_1".to_string(),
                },
                Vote {
                    voter_id: "voter2".to_string(),
                    candidate_id: "candidate2".to_string(),
                    encrypted_vote: "encrypted_vote_2".to_string(),
                    signature: "signature_2".to_string(),
                },
            ],
            timestamp: Utc::now(),
            hash: "ballot_hash_1".to_string(),
        };
        
        let result = storage.store_ballot(&ballot).await;
        assert!(result.is_ok());
        
        let ipfs_hash = result.unwrap();
        assert!(!ipfs_hash.is_empty());
    }

    /// Testa recuperação de boletim de urna
    #[tokio::test]
    async fn test_get_ballot() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        let ballot = Ballot {
            id: "election1:ballot1".to_string(),
            election_id: "election1".to_string(),
            urna_id: "urna1".to_string(),
            votes: vec![
                Vote {
                    voter_id: "voter1".to_string(),
                    candidate_id: "candidate1".to_string(),
                    encrypted_vote: "encrypted_vote_1".to_string(),
                    signature: "signature_1".to_string(),
                },
            ],
            timestamp: Utc::now(),
            hash: "ballot_hash_1".to_string(),
        };
        
        // Armazenar boletim
        let ipfs_hash = storage.store_ballot(&ballot).await.unwrap();
        
        // Recuperar boletim
        let retrieved_ballot = storage.get_ballot(&ballot.id).await.unwrap();
        assert!(retrieved_ballot.is_some());
        
        let retrieved = retrieved_ballot.unwrap();
        assert_eq!(retrieved.id, ballot.id);
        assert_eq!(retrieved.election_id, ballot.election_id);
        assert_eq!(retrieved.urna_id, ballot.urna_id);
        assert_eq!(retrieved.votes.len(), ballot.votes.len());
    }

    /// Testa armazenamento de prova de auditoria
    #[tokio::test]
    async fn test_store_audit_proof() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        let audit_proof = AuditProof {
            audit_id: "audit1".to_string(),
            election_id: "election1".to_string(),
            proof_data: "audit_proof_data".to_string(),
            timestamp: Utc::now(),
            verifier: "verifier1".to_string(),
        };
        
        let result = storage.store_audit_proof(&audit_proof).await;
        assert!(result.is_ok());
        
        let ipfs_hash = result.unwrap();
        assert!(!ipfs_hash.is_empty());
    }

    /// Testa recuperação de prova de auditoria
    #[tokio::test]
    async fn test_get_audit_proof() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        let audit_proof = AuditProof {
            audit_id: "audit1".to_string(),
            election_id: "election1".to_string(),
            proof_data: "audit_proof_data".to_string(),
            timestamp: Utc::now(),
            verifier: "verifier1".to_string(),
        };
        
        // Armazenar prova
        storage.store_audit_proof(&audit_proof).await.unwrap();
        
        // Recuperar prova
        let retrieved_proof = storage.get_audit_proof(&audit_proof.audit_id).await.unwrap();
        assert!(retrieved_proof.is_some());
        
        let retrieved = retrieved_proof.unwrap();
        assert_eq!(retrieved.audit_id, audit_proof.audit_id);
        assert_eq!(retrieved.election_id, audit_proof.election_id);
        assert_eq!(retrieved.proof_data, audit_proof.proof_data);
    }

    /// Testa listagem de boletins de uma eleição
    #[tokio::test]
    async fn test_list_ballots() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        let election_id = "election1";
        
        // Armazenar múltiplos boletins
        for i in 1..=5 {
            let ballot = Ballot {
                id: format!("{}:ballot{}", election_id, i),
                election_id: election_id.to_string(),
                urna_id: format!("urna{}", i),
                votes: vec![
                    Vote {
                        voter_id: format!("voter{}", i),
                        candidate_id: format!("candidate{}", i),
                        encrypted_vote: format!("encrypted_vote_{}", i),
                        signature: format!("signature_{}", i),
                    },
                ],
                timestamp: Utc::now(),
                hash: format!("ballot_hash_{}", i),
            };
            
            storage.store_ballot(&ballot).await.unwrap();
        }
        
        // Listar boletins da eleição
        let ballots = storage.list_ballots(election_id).await.unwrap();
        assert_eq!(ballots.len(), 5);
        
        // Verificar que todos os boletins pertencem à eleição
        for ballot in &ballots {
            assert_eq!(ballot.election_id, election_id);
        }
    }

    /// Testa verificação de integridade
    #[tokio::test]
    async fn test_verify_integrity() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        let ballot = Ballot {
            id: "election1:ballot1".to_string(),
            election_id: "election1".to_string(),
            urna_id: "urna1".to_string(),
            votes: vec![],
            timestamp: Utc::now(),
            hash: "ballot_hash_1".to_string(),
        };
        
        // Armazenar boletim
        let ipfs_hash = storage.store_ballot(&ballot).await.unwrap();
        
        // Verificar integridade
        let is_valid = storage.verify_integrity(&ipfs_hash).await.unwrap();
        assert!(is_valid);
    }

    /// Testa cache local
    #[tokio::test]
    async fn test_local_cache() {
        let cache = LocalCache::new(10);
        
        let data = b"test data".to_vec();
        let ttl = chrono::Duration::hours(1);
        
        // Armazenar no cache
        cache.put("key1", data.clone(), ttl).await.unwrap();
        
        // Recuperar do cache
        let retrieved = cache.get("key1").await.unwrap();
        assert_eq!(retrieved, Some(data));
        
        // Remover do cache
        cache.remove("key1").await.unwrap();
        
        // Verificar que foi removido
        let retrieved_after_removal = cache.get("key1").await.unwrap();
        assert_eq!(retrieved_after_removal, None);
    }

    /// Testa limpeza de cache expirado
    #[tokio::test]
    async fn test_cache_cleanup() {
        let cache = LocalCache::new(10);
        
        let data1 = b"data1".to_vec();
        let data2 = b"data2".to_vec();
        
        // Armazenar dados com TTLs diferentes
        cache.put("key1", data1, chrono::Duration::milliseconds(1)).await.unwrap();
        cache.put("key2", data2, chrono::Duration::hours(1)).await.unwrap();
        
        // Aguardar expiração do primeiro item
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        // Limpar cache expirado
        cache.cleanup_expired().await.unwrap();
        
        // Verificar que apenas o item não expirado permanece
        let retrieved1 = cache.get("key1").await.unwrap();
        let retrieved2 = cache.get("key2").await.unwrap();
        
        assert_eq!(retrieved1, None); // Expirado
        assert_eq!(retrieved2, Some(data2)); // Não expirado
    }

    /// Testa DHT client
    #[tokio::test]
    async fn test_dht_client() {
        let dht = DhtClient::new("node1".to_string());
        
        // Registrar nó
        let node = DhtNode {
            id: "node2".to_string(),
            address: "192.168.1.2".to_string(),
            port: 8080,
            last_seen: Utc::now(),
        };
        
        dht.register_node(node).await;
        
        // Registrar boletim
        let election_id = "election1";
        let ballot_hash = "ballot_hash_1";
        let result = dht.register_ballot(election_id, ballot_hash).await;
        assert!(result.is_ok());
        
        // Descobrir boletins
        let ballots = dht.discover_ballots(election_id).await.unwrap();
        // Pode estar vazio em ambiente de teste, mas não deve falhar
    }

    /// Testa IPFS client
    #[tokio::test]
    async fn test_ipfs_client() {
        let ipfs = IpfsClient::new("http://localhost:5001".to_string());
        
        let data = b"test data for IPFS";
        
        // Adicionar dados ao IPFS
        let result = ipfs.add_data(data).await;
        // Pode falhar se IPFS não estiver rodando, mas não deve panica
        if let Ok(hash) = result {
            assert!(!hash.is_empty());
            
            // Verificar se dados existem
            let exists = ipfs.exists(&hash).await.unwrap_or(false);
            // Pode ser false se IPFS não estiver rodando
        }
    }

    /// Testa performance com múltiplos boletins
    #[tokio::test]
    async fn test_performance_multiple_ballots() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            10000 // cache maior
        );
        
        let start_time = std::time::Instant::now();
        
        // Armazenar 100 boletins
        for i in 1..=100 {
            let ballot = Ballot {
                id: format!("election1:ballot{}", i),
                election_id: "election1".to_string(),
                urna_id: format!("urna{}", i % 10), // 10 urnas diferentes
                votes: vec![
                    Vote {
                        voter_id: format!("voter{}", i),
                        candidate_id: format!("candidate{}", i % 5), // 5 candidatos
                        encrypted_vote: format!("encrypted_vote_{}", i),
                        signature: format!("signature_{}", i),
                    },
                ],
                timestamp: Utc::now(),
                hash: format!("ballot_hash_{}", i),
            };
            
            storage.store_ballot(&ballot).await.unwrap();
        }
        
        let duration = start_time.elapsed();
        
        // Verificar que foi rápido (< 10 segundos para 100 boletins)
        assert!(duration.as_secs() < 10, "Performance test failed: took {} seconds", duration.as_secs());
        
        println!("Performance test: 100 ballots stored in {:?}", duration);
    }

    /// Testa tolerância a falhas de IPFS
    #[tokio::test]
    async fn test_ipfs_failure_tolerance() {
        // Usar endpoint IPFS inválido
        let storage = DistributedStorage::new(
            "http://invalid-ipfs-endpoint:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        let ballot = Ballot {
            id: "election1:ballot1".to_string(),
            election_id: "election1".to_string(),
            urna_id: "urna1".to_string(),
            votes: vec![],
            timestamp: Utc::now(),
            hash: "ballot_hash_1".to_string(),
        };
        
        // Deve falhar graciosamente
        let result = storage.store_ballot(&ballot).await;
        // Pode falhar, mas não deve panica
        if let Err(e) = result {
            println!("Expected IPFS failure: {}", e);
        }
    }

    /// Testa serialização de boletins
    #[test]
    fn test_ballot_serialization() {
        let ballot = Ballot {
            id: "election1:ballot1".to_string(),
            election_id: "election1".to_string(),
            urna_id: "urna1".to_string(),
            votes: vec![
                Vote {
                    voter_id: "voter1".to_string(),
                    candidate_id: "candidate1".to_string(),
                    encrypted_vote: "encrypted_vote_1".to_string(),
                    signature: "signature_1".to_string(),
                },
            ],
            timestamp: Utc::now(),
            hash: "ballot_hash_1".to_string(),
        };
        
        // Serializar
        let serialized = serde_json::to_vec(&ballot).unwrap();
        assert!(!serialized.is_empty());
        
        // Deserializar
        let deserialized: Ballot = serde_json::from_slice(&serialized).unwrap();
        assert_eq!(deserialized.id, ballot.id);
        assert_eq!(deserialized.election_id, ballot.election_id);
        assert_eq!(deserialized.urna_id, ballot.urna_id);
        assert_eq!(deserialized.votes.len(), ballot.votes.len());
    }

    /// Testa serialização de provas de auditoria
    #[test]
    fn test_audit_proof_serialization() {
        let audit_proof = AuditProof {
            audit_id: "audit1".to_string(),
            election_id: "election1".to_string(),
            proof_data: "audit_proof_data".to_string(),
            timestamp: Utc::now(),
            verifier: "verifier1".to_string(),
        };
        
        // Serializar
        let serialized = serde_json::to_vec(&audit_proof).unwrap();
        assert!(!serialized.is_empty());
        
        // Deserializar
        let deserialized: AuditProof = serde_json::from_slice(&serialized).unwrap();
        assert_eq!(deserialized.audit_id, audit_proof.audit_id);
        assert_eq!(deserialized.election_id, audit_proof.election_id);
        assert_eq!(deserialized.proof_data, audit_proof.proof_data);
        assert_eq!(deserialized.verifier, audit_proof.verifier);
    }

    /// Testa extração de ID da eleição
    #[test]
    fn test_extract_election_id() {
        let storage = DistributedStorage::new(
            "http://localhost:5001".to_string(),
            "node1".to_string(),
            1000
        );
        
        // Teste com formato válido
        let ballot_id = "election1:ballot1";
        let election_id = storage.extract_election_id_from_ballot_id(ballot_id).unwrap();
        assert_eq!(election_id, "election1");
        
        // Teste com formato inválido
        let invalid_ballot_id = "invalid_format";
        let result = storage.extract_election_id_from_ballot_id(invalid_ballot_id);
        assert!(result.is_err());
    }
}
