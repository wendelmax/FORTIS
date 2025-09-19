//! Sistema de Logs Transparentes para Auditoria
//! 
//! Implementa logs transparentes (similar a CT logs) para auditoria
//! independente e transparente do processo eleitoral, seguindo os
//! princípios do Prof. Marcos Simplicio de usar ferramentas apropriadas
//! para cada problema.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Entrada de log transparente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub event_type: LogEventType,
    pub event_hash: String,
    pub merkle_proof: MerkleProof,
    pub data: LogData,
}

/// Tipos de eventos que podem ser registrados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogEventType {
    ElectionCreated,
    ElectionStarted,
    ElectionEnded,
    VoteCast,
    VoteVerified,
    AuditTriggered,
    SystemEvent,
    SecurityAlert,
}

/// Dados do evento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogData {
    ElectionData {
        election_id: String,
        title: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    },
    VoteData {
        vote_id: String,
        election_id: String,
        voter_id: String,
        candidate_id: String,
        nullifier: String,
    },
    AuditData {
        audit_id: String,
        audit_type: String,
        findings: Vec<String>,
    },
    SystemData {
        component: String,
        status: String,
        message: String,
    },
}

/// Prova Merkle para inclusão no log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProof {
    pub leaf_index: u64,
    pub path: Vec<String>,
    pub root_hash: String,
}

/// Prova de inclusão no log transparente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProof {
    pub log_entry: LogEntry,
    pub merkle_proof: MerkleProof,
    pub root_hash: String,
    pub tree_size: u64,
}

/// Árvore Merkle para logs transparentes
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

    /// Adiciona uma folha à árvore Merkle
    pub fn add_leaf(&mut self, data: &str) -> u64 {
        let leaf_hash = self.hash_data(data);
        let index = self.leaves.len() as u64;
        self.leaves.push(leaf_hash.clone());
        self.rebuild_tree();
        index
    }

    /// Gera prova de inclusão para uma folha
    pub fn generate_proof(&self, leaf_index: u64) -> Result<MerkleProof> {
        if leaf_index >= self.leaves.len() as u64 {
            return Err(anyhow!("Leaf index out of bounds"));
        }

        let mut path = Vec::new();
        let mut current_index = leaf_index as usize;
        let mut level_size = self.leaves.len();

        while level_size > 1 {
            if level_size % 2 == 1 {
                // Se o nível tem número ímpar de nós, duplica o último
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
        })
    }

    /// Retorna o hash da raiz
    pub fn root(&self) -> Option<String> {
        self.root.clone()
    }

    /// Reconstrói a árvore Merkle
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
                    left // Duplica o último nó se ímpar
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

    /// Calcula hash SHA-256 dos dados
    fn hash_data(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Sistema de logs transparentes
pub struct TransparentLog {
    merkle_tree: MerkleTree,
    log_entries: Vec<LogEntry>,
    next_index: u64,
}

impl TransparentLog {
    pub fn new() -> Self {
        Self {
            merkle_tree: MerkleTree::new(),
            log_entries: Vec::new(),
            next_index: 0,
        }
    }

    /// Adiciona um evento de auditoria ao log
    pub fn append_audit_event(&mut self, event_type: LogEventType, data: LogData) -> Result<InclusionProof> {
        let timestamp = Utc::now();
        let event_data = serde_json::to_string(&data)?;
        let event_hash = self.hash_data(&event_data);

        // Adicionar à árvore Merkle
        let leaf_index = self.merkle_tree.add_leaf(&event_hash);

        // Criar entrada de log
        let log_entry = LogEntry {
            index: self.next_index,
            timestamp,
            event_type,
            event_hash,
            merkle_proof: MerkleProof {
                leaf_index,
                path: Vec::new(), // Será preenchido abaixo
                root_hash: String::new(), // Será preenchido abaixo
            },
            data,
        };

        // Gerar prova de inclusão
        let merkle_proof = self.merkle_tree.generate_proof(leaf_index)?;

        // Atualizar entrada com prova completa
        let mut complete_entry = log_entry.clone();
        complete_entry.merkle_proof = merkle_proof.clone();

        // Adicionar ao log
        self.log_entries.push(complete_entry.clone());
        self.next_index += 1;

        Ok(InclusionProof {
            log_entry: complete_entry,
            merkle_proof,
            root_hash: self.merkle_tree.root().unwrap_or_default(),
            tree_size: self.log_entries.len() as u64,
        })
    }

    /// Verifica se um evento está incluído no log
    pub fn verify_inclusion(&self, proof: &InclusionProof) -> Result<bool> {
        // Verificar se a entrada existe
        if proof.log_entry.index >= self.log_entries.len() as u64 {
            return Ok(false);
        }

        let stored_entry = &self.log_entries[proof.log_entry.index as usize];
        
        // Verificar se os dados coincidem
        if stored_entry.event_hash != proof.log_entry.event_hash {
            return Ok(false);
        }

        // Verificar prova Merkle
        self.verify_merkle_proof(&proof.merkle_proof)
    }

    /// Verifica prova Merkle
    fn verify_merkle_proof(&self, proof: &MerkleProof) -> Result<bool> {
        let mut current_hash = self.merkle_tree.leaves[proof.leaf_index as usize].clone();
        let mut current_index = proof.leaf_index as usize;
        let mut level_size = self.merkle_tree.leaves.len();

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

    /// Obtém todas as entradas de um tipo específico
    pub fn get_entries_by_type(&self, event_type: &LogEventType) -> Vec<&LogEntry> {
        self.log_entries
            .iter()
            .filter(|entry| std::mem::discriminant(&entry.event_type) == std::mem::discriminant(event_type))
            .collect()
    }

    /// Obtém entradas em um intervalo de tempo
    pub fn get_entries_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&LogEntry> {
        self.log_entries
            .iter()
            .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
            .collect()
    }

    /// Calcula hash SHA-256 dos dados
    fn hash_data(&self, data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Retorna estatísticas do log
    pub fn get_stats(&self) -> LogStats {
        LogStats {
            total_entries: self.log_entries.len(),
            tree_size: self.merkle_tree.leaves.len(),
            root_hash: self.merkle_tree.root().unwrap_or_default(),
            first_entry_time: self.log_entries.first().map(|e| e.timestamp),
            last_entry_time: self.log_entries.last().map(|e| e.timestamp),
        }
    }
}

/// Estatísticas do log transparente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStats {
    pub total_entries: usize,
    pub tree_size: usize,
    pub root_hash: String,
    pub first_entry_time: Option<DateTime<Utc>>,
    pub last_entry_time: Option<DateTime<Utc>>,
}

/// Serviço de auditoria transparente
pub struct TransparentAuditService {
    log: TransparentLog,
}

impl TransparentAuditService {
    pub fn new() -> Self {
        Self {
            log: TransparentLog::new(),
        }
    }

    /// Registra criação de eleição
    pub async fn log_election_created(&mut self, election_id: String, title: String, 
                                    start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result<InclusionProof> {
        let data = LogData::ElectionData {
            election_id,
            title,
            start_time,
            end_time,
        };

        self.log.append_audit_event(LogEventType::ElectionCreated, data)
    }

    /// Registra voto
    pub async fn log_vote_cast(&mut self, vote_id: String, election_id: String, 
                              voter_id: String, candidate_id: String, nullifier: String) -> Result<InclusionProof> {
        let data = LogData::VoteData {
            vote_id,
            election_id,
            voter_id,
            candidate_id,
            nullifier,
        };

        self.log.append_audit_event(LogEventType::VoteCast, data)
    }

    /// Registra auditoria
    pub async fn log_audit(&mut self, audit_id: String, audit_type: String, 
                          findings: Vec<String>) -> Result<InclusionProof> {
        let data = LogData::AuditData {
            audit_id,
            audit_type,
            findings,
        };

        self.log.append_audit_event(LogEventType::AuditTriggered, data)
    }

    /// Registra evento do sistema
    pub async fn log_system_event(&mut self, component: String, status: String, 
                                 message: String) -> Result<InclusionProof> {
        let data = LogData::SystemData {
            component,
            status,
            message,
        };

        self.log.append_audit_event(LogEventType::SystemEvent, data)
    }

    /// Verifica inclusão de evento
    pub fn verify_event_inclusion(&self, proof: &InclusionProof) -> Result<bool> {
        self.log.verify_inclusion(proof)
    }

    /// Obtém estatísticas do log
    pub fn get_log_stats(&self) -> LogStats {
        self.log.get_stats()
    }

    /// Exporta log para auditoria externa
    pub fn export_log(&self) -> Result<Vec<LogEntry>> {
        Ok(self.log.log_entries.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_creation() {
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

    #[tokio::test]
    async fn test_transparent_log_operations() {
        let mut log = TransparentLog::new();
        
        let data = LogData::SystemData {
            component: "test".to_string(),
            status: "ok".to_string(),
            message: "test message".to_string(),
        };

        let proof = log.append_audit_event(LogEventType::SystemEvent, data).unwrap();
        assert!(log.verify_inclusion(&proof).unwrap());
    }

    #[tokio::test]
    async fn test_audit_service() {
        let mut service = TransparentAuditService::new();
        
        let proof = service.log_system_event(
            "test".to_string(),
            "ok".to_string(),
            "test message".to_string()
        ).await.unwrap();

        assert!(service.verify_event_inclusion(&proof).unwrap());
    }
}
