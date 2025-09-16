//! Sistema de Logs Transparentes para Eleições
//! 
//! Implementa logs transparentes inspirados em Certificate Transparency,
//! seguindo rigorosamente a crítica do Prof. Marcos Simplicio de que
//! blockchain não é necessário para transparência eleitoral.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use ring::signature::{Ed25519KeyPair, Signature, UnparsedPublicKey};

/// Entrada de log eleitoral transparente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionLogEntry {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub event_type: ElectionEventType,
    pub event_data: Vec<u8>,
    pub event_hash: String,
    pub merkle_proof: MerkleProof,
    pub verifier_signatures: Vec<VerifierSignature>,
    pub inclusion_proof: InclusionProof,
}

/// Tipos de eventos eleitorais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElectionEventType {
    ElectionCreated,
    ElectionStarted,
    ElectionEnded,
    VoteCast,
    VoteVerified,
    AuditTriggered,
    SecurityAlert,
    SystemEvent,
}

/// Dados do evento eleitoral
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionEvent {
    pub id: String,
    pub event_type: ElectionEventType,
    pub election_id: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub source: String, // Urna, TSE, Sistema, etc.
}

/// Prova Merkle para inclusão no log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_index: u64,
    pub path: Vec<String>,
    pub root_hash: String,
    pub tree_size: u64,
}

/// Assinatura de verificador
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierSignature {
    pub verifier_id: String,
    pub signature: String,
    pub public_key: String,
    pub timestamp: DateTime<Utc>,
}

/// Prova de inclusão no log transparente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub log_entry: ElectionLogEntry,
    pub merkle_proof: MerkleProof,
    pub verifier_signatures: Vec<VerifierSignature>,
    pub verification_status: VerificationStatus,
}

/// Status de verificação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    PartiallyVerified,
}

/// Verificador de log
#[derive(Debug, Clone)]
pub struct LogVerifier {
    pub id: String,
    pub name: String,
    pub public_key: Ed25519KeyPair,
    pub is_active: bool,
    pub trust_level: u8, // 0-100
}

/// Sistema de logs transparentes para eleições
pub struct ElectionTransparencyLog {
    merkle_tree: MerkleTree,
    log_entries: Vec<ElectionLogEntry>,
    verifiers: Vec<LogVerifier>,
    next_index: u64,
    config: LogConfig,
}

/// Configuração do log
#[derive(Debug, Clone)]
pub struct LogConfig {
    pub min_verifiers: usize,
    pub max_verifiers: usize,
    pub signature_threshold: usize,
    pub retention_days: u64,
}

impl ElectionTransparencyLog {
    pub fn new(config: LogConfig) -> Self {
        Self {
            merkle_tree: MerkleTree::new(),
            log_entries: Vec::new(),
            verifiers: Vec::new(),
            next_index: 0,
            config,
        }
    }

    /// Adiciona um verificador ao log
    pub fn add_verifier(&mut self, verifier: LogVerifier) -> Result<()> {
        if self.verifiers.len() >= self.config.max_verifiers {
            return Err(anyhow!("Maximum verifiers reached"));
        }

        self.verifiers.push(verifier);
        Ok(())
    }

    /// Registra evento eleitoral no log transparente
    pub fn append_election_event(&mut self, event: ElectionEvent) -> Result<InclusionProof> {
        // Serializar evento
        let event_data = serde_json::to_vec(&event)?;
        let event_hash = self.hash_data(&event_data);

        // Verificar se evento já existe
        if self.event_exists(&event_hash) {
            return Err(anyhow!("Event already exists"));
        }

        // Adicionar à árvore Merkle
        let leaf_index = self.merkle_tree.add_leaf(&event_hash);

        // Coletar assinaturas dos verificadores
        let verifier_signatures = self.collect_verifier_signatures(&event, &event_hash)?;

        // Criar entrada de log
        let log_entry = ElectionLogEntry {
            index: self.next_index,
            timestamp: Utc::now(),
            event_type: event.event_type.clone(),
            event_data: event_data.clone(),
            event_hash: event_hash.clone(),
            merkle_proof: MerkleProof {
                leaf_index,
                path: Vec::new(), // Será preenchido abaixo
                root_hash: String::new(), // Será preenchido abaixo
                tree_size: self.merkle_tree.size(),
            },
            verifier_signatures: verifier_signatures.clone(),
            inclusion_proof: InclusionProof {
                log_entry: ElectionLogEntry::default(), // Placeholder
                merkle_proof: MerkleProof::default(),
                verifier_signatures: Vec::new(),
                verification_status: VerificationStatus::Pending,
            },
        };

        // Gerar prova Merkle completa
        let merkle_proof = self.merkle_tree.generate_proof(leaf_index)?;
        let mut complete_entry = log_entry.clone();
        complete_entry.merkle_proof = merkle_proof.clone();

        // Adicionar ao log
        self.log_entries.push(complete_entry.clone());
        self.next_index += 1;

        // Criar prova de inclusão
        let inclusion_proof = InclusionProof {
            log_entry: complete_entry,
            merkle_proof,
            verifier_signatures,
            verification_status: self.verify_event_integrity(&complete_entry)?,
        };

        Ok(inclusion_proof)
    }

    /// Verifica integridade de um evento
    pub fn verify_event_integrity(&self, entry: &ElectionLogEntry) -> Result<VerificationStatus> {
        // Verificar assinaturas dos verificadores
        let signature_count = self.verify_verifier_signatures(entry)?;
        
        // Verificar prova Merkle
        let merkle_valid = self.verify_merkle_proof(&entry.merkle_proof)?;
        
        // Verificar timestamp
        let timestamp_valid = self.verify_timestamp(entry)?;
        
        // Determinar status de verificação
        let verification_status = if signature_count >= self.config.signature_threshold 
            && merkle_valid && timestamp_valid {
            VerificationStatus::Verified
        } else if signature_count > 0 && (merkle_valid || timestamp_valid) {
            VerificationStatus::PartiallyVerified
        } else {
            VerificationStatus::Failed
        };

        Ok(verification_status)
    }

    /// Verifica assinaturas dos verificadores
    fn verify_verifier_signatures(&self, entry: &ElectionLogEntry) -> Result<usize> {
        let mut valid_signatures = 0;
        
        for verifier_sig in &entry.verifier_signatures {
            if let Some(verifier) = self.verifiers.iter()
                .find(|v| v.id == verifier_sig.verifier_id) {
                
                if self.verify_signature(verifier, &entry.event_hash, &verifier_sig.signature)? {
                    valid_signatures += 1;
                }
            }
        }
        
        Ok(valid_signatures)
    }

    /// Verifica assinatura individual
    fn verify_signature(
        &self, 
        verifier: &LogVerifier, 
        message: &str, 
        signature: &str
    ) -> Result<bool> {
        let message_bytes = message.as_bytes();
        let signature_bytes = hex::decode(signature)?;
        
        let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, &verifier.public_key.public_key().as_ref());
        let result = public_key.verify(message_bytes, &signature_bytes);
        
        Ok(result.is_ok())
    }

    /// Verifica prova Merkle
    fn verify_merkle_proof(&self, proof: &MerkleProof) -> Result<bool> {
        self.merkle_tree.verify_proof(proof)
    }

    /// Verifica timestamp
    fn verify_timestamp(&self, entry: &ElectionLogEntry) -> Result<bool> {
        let now = Utc::now();
        let time_diff = (now - entry.timestamp).num_seconds().abs();
        
        // Timestamp não deve ser muito antigo ou futuro
        Ok(time_diff <= 3600) // 1 hora de tolerância
    }

    /// Coleta assinaturas dos verificadores
    fn collect_verifier_signatures(
        &self, 
        event: &ElectionEvent, 
        event_hash: &str
    ) -> Result<Vec<VerifierSignature>> {
        let mut signatures = Vec::new();
        
        for verifier in &self.verifiers {
            if !verifier.is_active {
                continue;
            }
            
            // Simular assinatura (em implementação real, seria assíncrono)
            let signature = self.sign_with_verifier(verifier, event_hash)?;
            
            signatures.push(VerifierSignature {
                verifier_id: verifier.id.clone(),
                signature,
                public_key: hex::encode(verifier.public_key.public_key().as_ref()),
                timestamp: Utc::now(),
            });
        }
        
        Ok(signatures)
    }

    /// Assina com verificador
    fn sign_with_verifier(&self, verifier: &LogVerifier, message: &str) -> Result<String> {
        let message_bytes = message.as_bytes();
        let signature = verifier.public_key.sign(message_bytes);
        Ok(hex::encode(signature.as_ref()))
    }

    /// Verifica se evento já existe
    fn event_exists(&self, event_hash: &str) -> bool {
        self.log_entries.iter()
            .any(|entry| entry.event_hash == event_hash)
    }

    /// Obtém eventos por tipo
    pub fn get_events_by_type(&self, event_type: &ElectionEventType) -> Vec<&ElectionLogEntry> {
        self.log_entries.iter()
            .filter(|entry| std::mem::discriminant(&entry.event_type) == std::mem::discriminant(event_type))
            .collect()
    }

    /// Obtém eventos em intervalo de tempo
    pub fn get_events_by_time_range(
        &self, 
        start: DateTime<Utc>, 
        end: DateTime<Utc>
    ) -> Vec<&ElectionLogEntry> {
        self.log_entries.iter()
            .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
            .collect()
    }

    /// Obtém estatísticas do log
    pub fn get_log_stats(&self) -> LogStats {
        let verified_events = self.log_entries.iter()
            .filter(|entry| matches!(entry.inclusion_proof.verification_status, VerificationStatus::Verified))
            .count();

        LogStats {
            total_events: self.log_entries.len(),
            verified_events,
            total_verifiers: self.verifiers.len(),
            active_verifiers: self.verifiers.iter().filter(|v| v.is_active).count(),
            tree_size: self.merkle_tree.size(),
            root_hash: self.merkle_tree.root().unwrap_or_default(),
        }
    }

    /// Exporta log para auditoria externa
    pub fn export_log(&self) -> Result<Vec<ElectionLogEntry>> {
        Ok(self.log_entries.clone())
    }

    /// Calcula hash de dados
    fn hash_data(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

/// Árvore Merkle otimizada para logs transparentes
#[derive(Debug)]
pub struct MerkleTree {
    leaves: Vec<String>,
    nodes: HashMap<String, String>,
    root: Option<String>,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self {
            leaves: Vec::new(),
            nodes: HashMap::new(),
            root: None,
        }
    }

    pub fn add_leaf(&mut self, data: &str) -> u64 {
        let leaf_hash = self.hash_data(data);
        let index = self.leaves.len() as u64;
        self.leaves.push(leaf_hash.clone());
        self.rebuild_tree();
        index
    }

    pub fn generate_proof(&self, leaf_index: u64) -> Result<MerkleProof> {
        if leaf_index >= self.leaves.len() as u64 {
            return Err(anyhow!("Leaf index out of bounds"));
        }

        let mut path = Vec::new();
        let mut current_index = leaf_index as usize;
        let mut level_size = self.leaves.len();

        while level_size > 1 {
            if level_size % 2 == 1 {
                level_size += 1;
            }

            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < level_size {
                path.push(self.leaves[sibling_index].clone());
            }

            current_index /= 2;
            level_size /= 2;
        }

        Ok(MerkleProof {
            leaf_index,
            path,
            root_hash: self.root.clone().unwrap_or_default(),
            tree_size: self.leaves.len() as u64,
        })
    }

    pub fn verify_proof(&self, proof: &MerkleProof) -> Result<bool> {
        if proof.leaf_index >= self.leaves.len() as u64 {
            return Ok(false);
        }

        let mut current_hash = self.leaves[proof.leaf_index as usize].clone();
        let mut current_index = proof.leaf_index as usize;
        let mut level_size = self.leaves.len();

        for sibling_hash in &proof.path {
            if level_size % 2 == 1 {
                level_size += 1;
            }

            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < level_size {
                let combined = if current_index % 2 == 0 {
                    format!("{}{}", current_hash, sibling_hash)
                } else {
                    format!("{}{}", sibling_hash, current_hash)
                };
                current_hash = self.hash_data(&combined);
            }

            current_index /= 2;
            level_size /= 2;
        }

        Ok(current_hash == proof.root_hash)
    }

    pub fn root(&self) -> Option<String> {
        self.root.clone()
    }

    pub fn size(&self) -> u64 {
        self.leaves.len() as u64
    }

    fn rebuild_tree(&mut self) {
        if self.leaves.is_empty() {
            self.root = None;
            return;
        }

        let mut current_level = self.leaves.clone();
        let mut level_size = current_level.len();

        while level_size > 1 {
            let mut next_level = Vec::new();

            for i in (0..level_size).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < level_size {
                    &current_level[i + 1]
                } else {
                    left
                };

                let combined = format!("{}{}", left, right);
                let hash = self.hash_data(&combined);
                next_level.push(hash);
            }

            current_level = next_level;
            level_size = current_level.len();
        }

        self.root = current_level.first().cloned();
    }

    fn hash_data(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Estatísticas do log transparente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStats {
    pub total_events: usize,
    pub verified_events: usize,
    pub total_verifiers: usize,
    pub active_verifiers: usize,
    pub tree_size: u64,
    pub root_hash: String,
}

// Implementações Default necessárias
impl Default for ElectionLogEntry {
    fn default() -> Self {
        Self {
            index: 0,
            timestamp: Utc::now(),
            event_type: ElectionEventType::SystemEvent,
            event_data: Vec::new(),
            event_hash: String::new(),
            merkle_proof: MerkleProof {
                leaf_index: 0,
                path: Vec::new(),
                root_hash: String::new(),
                tree_size: 0,
            },
            verifier_signatures: Vec::new(),
            inclusion_proof: InclusionProof {
                log_entry: ElectionLogEntry::default(),
                merkle_proof: MerkleProof {
                    leaf_index: 0,
                    path: Vec::new(),
                    root_hash: String::new(),
                    tree_size: 0,
                },
                verifier_signatures: Vec::new(),
                verification_status: VerificationStatus::Pending,
            },
        }
    }
}

impl Default for MerkleProof {
    fn default() -> Self {
        Self {
            leaf_index: 0,
            path: Vec::new(),
            root_hash: String::new(),
            tree_size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_operations() {
        let mut tree = MerkleTree::new();
        let index1 = tree.add_leaf("data1");
        let index2 = tree.add_leaf("data2");
        
        assert_eq!(index1, 0);
        assert_eq!(index2, 1);
        assert!(tree.root().is_some());
    }

    #[test]
    fn test_merkle_proof_generation() {
        let mut tree = MerkleTree::new();
        tree.add_leaf("data1");
        tree.add_leaf("data2");
        tree.add_leaf("data3");

        let proof = tree.generate_proof(0).unwrap();
        assert_eq!(proof.leaf_index, 0);
        assert!(!proof.path.is_empty());
    }

    #[test]
    fn test_election_log_creation() {
        let config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 10,
            signature_threshold: 1,
            retention_days: 365,
        };
        
        let mut log = ElectionTransparencyLog::new(config);
        
        let event = ElectionEvent {
            id: "test_event".to_string(),
            event_type: ElectionEventType::SystemEvent,
            election_id: "test_election".to_string(),
            data: serde_json::json!({"test": "data"}),
            timestamp: Utc::now(),
            source: "test".to_string(),
        };

        let result = log.append_election_event(event);
        assert!(result.is_ok());
    }
}
