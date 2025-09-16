//! Sistema de Armazenamento Distribuído Eficiente
//! 
//! Implementa armazenamento distribuído usando DHT (Distributed Hash Table)
//! e IPFS, seguindo os princípios do Prof. Marcos Simplicio de usar
//! tecnologias apropriadas para cada problema, evitando replicação
//! completa desnecessária.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Cliente IPFS para armazenamento descentralizado
pub struct IpfsClient {
    endpoint: String,
    client: reqwest::Client,
}

impl IpfsClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    /// Adiciona dados ao IPFS
    pub async fn add_data(&self, data: &[u8]) -> Result<String> {
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(data.to_vec()));

        let response = self.client
            .post(&format!("{}/api/v0/add", self.endpoint))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to add data to IPFS: {}", response.status()));
        }

        let result: serde_json::Value = response.json().await?;
        let hash = result["Hash"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid IPFS response"))?;

        Ok(hash.to_string())
    }

    /// Recupera dados do IPFS
    pub async fn get_data(&self, hash: &str) -> Result<Vec<u8>> {
        let response = self.client
            .get(&format!("{}/api/v0/cat?arg={}", self.endpoint, hash))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to get data from IPFS: {}", response.status()));
        }

        let data = response.bytes().await?;
        Ok(data.to_vec())
    }

    /// Verifica se dados existem no IPFS
    pub async fn exists(&self, hash: &str) -> Result<bool> {
        let response = self.client
            .post(&format!("{}/api/v0/pin/ls?arg={}", self.endpoint, hash))
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}

/// Cliente DHT para descoberta de dados
pub struct DhtClient {
    nodes: RwLock<HashMap<String, DhtNode>>,
    local_node_id: String,
}

#[derive(Debug, Clone)]
pub struct DhtNode {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub last_seen: DateTime<Utc>,
}

impl DhtClient {
    pub fn new(local_node_id: String) -> Self {
        Self {
            nodes: RwLock::new(HashMap::new()),
            local_node_id,
        }
    }

    /// Registra um nó na DHT
    pub async fn register_node(&self, node: DhtNode) {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id.clone(), node);
    }

    /// Descobre nós próximos a uma chave
    pub async fn find_nodes(&self, key: &str) -> Result<Vec<DhtNode>> {
        let nodes = self.nodes.read().await;
        let mut closest_nodes = Vec::new();

        // Simulação de descoberta de nós próximos
        // Em implementação real, usaria algoritmo de roteamento DHT
        for (_, node) in nodes.iter() {
            if self.is_close_key(&node.id, key) {
                closest_nodes.push(node.clone());
            }
        }

        // Ordenar por proximidade
        closest_nodes.sort_by(|a, b| {
            self.distance(&a.id, key).cmp(&self.distance(&b.id, key))
        });

        Ok(closest_nodes.into_iter().take(8).collect())
    }

    /// Registra um boletim de urna na DHT
    pub async fn register_ballot(&self, election_id: &str, ballot_hash: &str) -> Result<()> {
        let key = format!("ballot:{}:{}", election_id, ballot_hash);
        let nodes = self.find_nodes(&key).await?;

        // Registrar em múltiplos nós para redundância
        for node in nodes {
            self.register_ballot_at_node(&node, &key, ballot_hash).await?;
        }

        Ok(())
    }

    /// Descobre boletins de urna para uma eleição
    pub async fn discover_ballots(&self, election_id: &str) -> Result<Vec<String>> {
        let key = format!("ballot:{}", election_id);
        let nodes = self.find_nodes(&key).await?;

        let mut ballots = Vec::new();
        for node in nodes {
            let node_ballots = self.get_ballots_from_node(&node, election_id).await?;
            ballots.extend(node_ballots);
        }

        // Remover duplicatas
        ballots.sort();
        ballots.dedup();

        Ok(ballots)
    }

    /// Verifica se uma chave está próxima a um nó
    fn is_close_key(&self, node_id: &str, key: &str) -> bool {
        // Simulação de proximidade baseada em hash
        let node_hash = self.hash_string(node_id);
        let key_hash = self.hash_string(key);
        self.distance(&node_hash, &key_hash) < 1000
    }

    /// Calcula distância entre duas chaves
    fn distance(&self, key1: &str, key2: &str) -> u64 {
        let hash1 = self.hash_string(key1);
        let hash2 = self.hash_string(key2);
        hash1.chars().zip(hash2.chars())
            .map(|(c1, c2)| (c1 as u64).wrapping_sub(c2 as u64))
            .sum()
    }

    /// Calcula hash de uma string
    fn hash_string(&self, input: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Registra boletim em um nó específico
    async fn register_ballot_at_node(&self, node: &DhtNode, key: &str, ballot_hash: &str) -> Result<()> {
        // Em implementação real, faria chamada HTTP para o nó
        // Por enquanto, apenas simula
        log::info!("Registering ballot {} at node {}", ballot_hash, node.id);
        Ok(())
    }

    /// Obtém boletins de um nó específico
    async fn get_ballots_from_node(&self, node: &DhtNode, election_id: &str) -> Result<Vec<String>> {
        // Em implementação real, faria chamada HTTP para o nó
        // Por enquanto, apenas simula
        log::info!("Getting ballots for election {} from node {}", election_id, node.id);
        Ok(vec![])
    }
}

/// Cache local para performance
pub struct LocalCache {
    cache: RwLock<HashMap<String, CachedItem>>,
    max_size: usize,
}

#[derive(Debug, Clone)]
pub struct CachedItem {
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub ttl: chrono::Duration,
}

impl LocalCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            max_size,
        }
    }

    /// Armazena item no cache
    pub async fn put(&self, key: &str, data: Vec<u8>, ttl: chrono::Duration) -> Result<()> {
        let mut cache = self.cache.write().await;

        // Verificar se cache está cheio
        if cache.len() >= self.max_size {
            self.evict_oldest(&mut cache).await?;
        }

        let item = CachedItem {
            data,
            timestamp: Utc::now(),
            ttl,
        };

        cache.insert(key.to_string(), item);
        Ok(())
    }

    /// Recupera item do cache
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut cache = self.cache.write().await;

        if let Some(item) = cache.get(key) {
            // Verificar se item expirou
            if Utc::now() - item.timestamp > item.ttl {
                cache.remove(key);
                return Ok(None);
            }

            Ok(Some(item.data.clone()))
        } else {
            Ok(None)
        }
    }

    /// Remove item do cache
    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }

    /// Remove item mais antigo do cache
    async fn evict_oldest(&self, cache: &mut HashMap<String, CachedItem>) -> Result<()> {
        let oldest_key = cache.iter()
            .min_by_key(|(_, item)| item.timestamp)
            .map(|(key, _)| key.clone());

        if let Some(key) = oldest_key {
            cache.remove(&key);
        }

        Ok(())
    }

    /// Limpa cache expirado
    pub async fn cleanup_expired(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        let now = Utc::now();

        cache.retain(|_, item| now - item.timestamp <= item.ttl);
        Ok(())
    }
}

/// Sistema de armazenamento distribuído principal
pub struct DistributedStorage {
    ipfs_client: IpfsClient,
    dht_client: DhtClient,
    local_cache: LocalCache,
}

impl DistributedStorage {
    pub fn new(ipfs_endpoint: String, local_node_id: String, cache_size: usize) -> Self {
        Self {
            ipfs_client: IpfsClient::new(ipfs_endpoint),
            dht_client: DhtClient::new(local_node_id),
            local_cache: LocalCache::new(cache_size),
        }
    }

    /// Armazena boletim de urna
    pub async fn store_ballot(&self, ballot: &Ballot) -> Result<String> {
        // Serializar boletim
        let ballot_data = serde_json::to_vec(ballot)?;

        // Verificar cache primeiro
        let cache_key = format!("ballot:{}", ballot.id);
        if let Some(cached_data) = self.local_cache.get(&cache_key).await? {
            if cached_data == ballot_data {
                return Ok(ballot.id.clone());
            }
        }

        // Armazenar no IPFS
        let ipfs_hash = self.ipfs_client.add_data(&ballot_data).await?;

        // Registrar na DHT para descoberta
        self.dht_client.register_ballot(&ballot.election_id, &ipfs_hash).await?;

        // Armazenar no cache local
        self.local_cache.put(&cache_key, ballot_data, chrono::Duration::hours(24)).await?;

        Ok(ipfs_hash)
    }

    /// Recupera boletim de urna
    pub async fn get_ballot(&self, ballot_id: &str) -> Result<Option<Ballot>> {
        // Verificar cache primeiro
        let cache_key = format!("ballot:{}", ballot_id);
        if let Some(cached_data) = self.local_cache.get(&cache_key).await? {
            let ballot: Ballot = serde_json::from_slice(&cached_data)?;
            return Ok(Some(ballot));
        }

        // Buscar na DHT
        let election_id = self.extract_election_id_from_ballot_id(ballot_id)?;
        let ballot_hashes = self.dht_client.discover_ballots(&election_id).await?;

        for hash in ballot_hashes {
            if let Ok(ballot_data) = self.ipfs_client.get_data(&hash).await {
                if let Ok(ballot) = serde_json::from_slice::<Ballot>(&ballot_data) {
                    if ballot.id == ballot_id {
                        // Armazenar no cache
                        self.local_cache.put(&cache_key, ballot_data, chrono::Duration::hours(24)).await?;
                        return Ok(Some(ballot));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Armazena prova de auditoria
    pub async fn store_audit_proof(&self, proof: &AuditProof) -> Result<String> {
        let proof_data = serde_json::to_vec(proof)?;
        let ipfs_hash = self.ipfs_client.add_data(&proof_data).await?;

        // Registrar na DHT
        let key = format!("audit:{}", proof.audit_id);
        self.dht_client.register_ballot(&key, &ipfs_hash).await?;

        Ok(ipfs_hash)
    }

    /// Recupera prova de auditoria
    pub async fn get_audit_proof(&self, audit_id: &str) -> Result<Option<AuditProof>> {
        let key = format!("audit:{}", audit_id);
        let hashes = self.dht_client.discover_ballots(&key).await?;

        for hash in hashes {
            if let Ok(proof_data) = self.ipfs_client.get_data(&hash).await {
                if let Ok(proof) = serde_json::from_slice::<AuditProof>(&proof_data) {
                    if proof.audit_id == audit_id {
                        return Ok(Some(proof));
                    }
                }
            }
        }

        Ok(None)
    }

    /// Lista todos os boletins de uma eleição
    pub async fn list_ballots(&self, election_id: &str) -> Result<Vec<Ballot>> {
        let ballot_hashes = self.dht_client.discover_ballots(election_id).await?;
        let mut ballots = Vec::new();

        for hash in ballot_hashes {
            if let Ok(ballot_data) = self.ipfs_client.get_data(&hash).await {
                if let Ok(ballot) = serde_json::from_slice::<Ballot>(&ballot_data) {
                    ballots.push(ballot);
                }
            }
        }

        Ok(ballots)
    }

    /// Verifica integridade de dados armazenados
    pub async fn verify_integrity(&self, hash: &str) -> Result<bool> {
        self.ipfs_client.exists(hash).await
    }

    /// Limpa cache expirado
    pub async fn cleanup_cache(&self) -> Result<()> {
        self.local_cache.cleanup_expired().await
    }

    /// Extrai ID da eleição do ID do boletim
    fn extract_election_id_from_ballot_id(&self, ballot_id: &str) -> Result<String> {
        // Assumindo formato: election_id:ballot_id
        ballot_id.split(':').next()
            .ok_or_else(|| anyhow!("Invalid ballot ID format"))
            .map(|s| s.to_string())
    }
}

/// Boletim de urna
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub id: String,
    pub election_id: String,
    pub urna_id: String,
    pub votes: Vec<Vote>,
    pub timestamp: DateTime<Utc>,
    pub hash: String,
}

/// Voto individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter_id: String,
    pub candidate_id: String,
    pub encrypted_vote: String,
    pub signature: String,
}

/// Prova de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditProof {
    pub audit_id: String,
    pub election_id: String,
    pub proof_data: String,
    pub timestamp: DateTime<Utc>,
    pub verifier: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_distributed_storage() {
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
            hash: "test_hash".to_string(),
        };

        // Teste de armazenamento
        let result = storage.store_ballot(&ballot).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_local_cache() {
        let cache = LocalCache::new(10);
        
        let data = b"test data".to_vec();
        cache.put("key1", data.clone(), chrono::Duration::hours(1)).await.unwrap();
        
        let retrieved = cache.get("key1").await.unwrap();
        assert_eq!(retrieved, Some(data));
    }
}
