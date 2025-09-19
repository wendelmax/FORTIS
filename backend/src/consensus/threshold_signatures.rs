//! Sistema de Threshold Signatures para Consenso Distribuído
//! 
//! Implementa threshold signatures usando criptografia de curvas elípticas
//! para consenso distribuído sem blockchain, seguindo os princípios da
//! Computação Transparente do FORTIS 3.0.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, KeyPair};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;

/// Configuração do threshold signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    pub total_nodes: usize,
    pub threshold: usize,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub enable_verification: bool,
}

impl Default for ThresholdConfig {
    fn default() -> Self {
        Self {
            total_nodes: 3,
            threshold: 2,
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        }
    }
}

/// Nó participante do consenso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusNode {
    pub id: String,
    pub name: String,
    pub public_key: String,
    pub is_active: bool,
    pub trust_level: u8, // 0-100
    pub last_seen: DateTime<Utc>,
    pub signature_count: u64,
}

/// Assinatura individual de um nó
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSignature {
    pub node_id: String,
    pub signature: String,
    pub timestamp: DateTime<Utc>,
    pub message_hash: String,
    pub verification_status: SignatureStatus,
}

/// Status da assinatura
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignatureStatus {
    Pending,
    Valid,
    Invalid,
    Expired,
    Revoked,
}

/// Threshold signature completa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdSignature {
    pub id: String,
    pub message: String,
    pub message_hash: String,
    pub signatures: Vec<NodeSignature>,
    pub threshold_met: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub verification_proof: VerificationProof,
}

/// Prova de verificação da threshold signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProof {
    pub valid_signatures: usize,
    pub total_signatures: usize,
    pub threshold_required: usize,
    pub verification_timestamp: DateTime<Utc>,
    pub consensus_reached: bool,
}

impl ThresholdSignature {
    pub fn new(id: String, message: String, threshold_required: usize) -> Self {
        Self {
            id,
            message: message.clone(),
            message_hash: format!("{:x}", sha2::Sha256::digest(message.as_bytes())),
            signatures: Vec::new(),
            threshold_met: false,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
            verification_proof: VerificationProof {
                valid_signatures: 0,
                total_signatures: 0,
                threshold_required,
                verification_timestamp: Utc::now(),
                consensus_reached: false,
            },
        }
    }
}

/// Requisição de assinatura
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureRequest {
    pub id: String,
    pub message: String,
    pub message_hash: String,
    pub requester_id: String,
    pub priority: SignaturePriority,
    pub expires_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Prioridade da assinatura
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignaturePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Sistema de threshold signatures
pub struct ThresholdSignatureService {
    config: ThresholdConfig,
    nodes: HashMap<String, ConsensusNode>,
    key_pairs: HashMap<String, Ed25519KeyPair>,
    pending_requests: HashMap<String, SignatureRequest>,
    completed_signatures: HashMap<String, ThresholdSignature>,
}

impl ThresholdSignatureService {
    /// Cria novo serviço de threshold signatures
    pub fn new(config: ThresholdConfig) -> Self {
        Self {
            config,
            nodes: HashMap::new(),
            key_pairs: HashMap::new(),
            pending_requests: HashMap::new(),
            completed_signatures: HashMap::new(),
        }
    }

    /// Adiciona um nó ao consenso
    pub fn add_node(&mut self, node: ConsensusNode, key_pair: Ed25519KeyPair) -> Result<()> {
        if self.nodes.len() >= self.config.total_nodes {
            return Err(anyhow!("Maximum nodes reached"));
        }

        let node_id = node.id.clone();
        self.nodes.insert(node_id.clone(), node);
        self.key_pairs.insert(node_id, key_pair);
        Ok(())
    }

    /// Remove um nó do consenso
    pub fn remove_node(&mut self, node_id: &str) -> Result<()> {
        self.nodes.remove(node_id);
        self.key_pairs.remove(node_id);
        Ok(())
    }

    /// Cria uma nova requisição de assinatura
    pub fn create_signature_request(&mut self, request: SignatureRequest) -> Result<String> {
        let request_id = request.id.clone();
        
        // Verificar se a requisição já existe
        if self.pending_requests.contains_key(&request_id) {
            return Err(anyhow!("Request already exists"));
        }

        // Validar hash da mensagem
        let calculated_hash = self.hash_message(&request.message);
        if calculated_hash != request.message_hash {
            return Err(anyhow!("Invalid message hash"));
        }

        self.pending_requests.insert(request_id.clone(), request);
        Ok(request_id)
    }

    /// Assina uma mensagem com a chave do nó
    pub fn sign_message(&mut self, node_id: &str, request_id: &str) -> Result<NodeSignature> {
        // Verificar se o nó existe e está ativo
        let node = self.nodes.get(node_id)
            .ok_or_else(|| anyhow!("Node not found"))?;
        
        if !node.is_active {
            return Err(anyhow!("Node is not active"));
        }

        // Verificar se a requisição existe
        let request = self.pending_requests.get(request_id)
            .ok_or_else(|| anyhow!("Request not found"))?;

        // Verificar se a requisição não expirou
        if Utc::now() > request.expires_at {
            return Err(anyhow!("Request expired"));
        }

        // Obter chave privada do nó
        let key_pair = self.key_pairs.get(node_id)
            .ok_or_else(|| anyhow!("Key pair not found"))?;

        // Assinar a mensagem
        let signature = self.sign_with_key(key_pair, &request.message_hash)?;
        let signature_hex = hex::encode(signature);

        // Criar assinatura do nó
        let node_signature = NodeSignature {
            node_id: node_id.to_string(),
            signature: signature_hex,
            timestamp: Utc::now(),
            message_hash: request.message_hash.clone(),
            verification_status: SignatureStatus::Valid,
        };

        // Atualizar contador de assinaturas do nó
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.signature_count += 1;
            node.last_seen = Utc::now();
        }

        Ok(node_signature)
    }

    /// Verifica uma assinatura individual
    pub fn verify_signature(&self, signature: &NodeSignature) -> Result<bool> {
        // Verificar se o nó existe
        let node = self.nodes.get(&signature.node_id)
            .ok_or_else(|| anyhow!("Node not found"))?;

        // Verificar se o nó está ativo
        if !node.is_active {
            return Ok(false);
        }

        // Decodificar chave pública
        let public_key_bytes = hex::decode(&node.public_key)?;
        let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, &public_key_bytes);

        // Decodificar assinatura
        let signature_bytes = hex::decode(&signature.signature)?;

        // Verificar assinatura
        let result = public_key.verify(signature.message_hash.as_bytes(), &signature_bytes);
        Ok(result.is_ok())
    }

    /// Coleta assinaturas para uma requisição
    pub fn collect_signatures(&mut self, request_id: &str) -> Result<ThresholdSignature> {
        let request = self.pending_requests.get(request_id)
            .ok_or_else(|| anyhow!("Request not found"))?
            .clone();

        // Simular coleta de assinaturas dos nós ativos
        let mut signatures = Vec::new();
        let mut valid_count = 0;

        // Coletar IDs dos nós ativos primeiro
        let active_node_ids: Vec<String> = self.nodes
            .iter()
            .filter(|(_, node)| node.is_active)
            .map(|(id, _)| id.clone())
            .collect();

        for node_id in active_node_ids {
            // Simular assinatura do nó
            if let Ok(node_signature) = self.sign_message(&node_id, request_id) {
                if self.verify_signature(&node_signature).unwrap_or(false) {
                    valid_count += 1;
                    signatures.push(node_signature);
                }
            }
        }

        // Verificar se o threshold foi atingido
        let threshold_met = valid_count >= self.config.threshold;

        // Criar threshold signature
        let threshold_signature = ThresholdSignature {
            id: request_id.to_string(),
            message: request.message.clone(),
            message_hash: request.message_hash.clone(),
            signatures,
            threshold_met,
            created_at: Utc::now(),
            expires_at: request.expires_at,
            verification_proof: VerificationProof {
                valid_signatures: valid_count,
                total_signatures: self.nodes.len(),
                threshold_required: self.config.threshold,
                verification_timestamp: Utc::now(),
                consensus_reached: threshold_met,
            },
        };

        // Mover para assinaturas completadas
        self.completed_signatures.insert(request_id.to_string(), threshold_signature.clone());
        self.pending_requests.remove(request_id);

        Ok(threshold_signature)
    }

    /// Verifica se o consenso foi atingido
    pub fn is_consensus_reached(&self, request_id: &str) -> Result<bool> {
        if let Some(signature) = self.completed_signatures.get(request_id) {
            Ok(signature.threshold_met)
        } else {
            Ok(false)
        }
    }

    /// Obtém estatísticas do sistema
    pub fn get_stats(&self) -> ConsensusStats {
        let active_nodes = self.nodes.values().filter(|n| n.is_active).count();
        let total_requests = self.pending_requests.len() + self.completed_signatures.len();
        let completed_requests = self.completed_signatures.len();
        let consensus_rate = if total_requests > 0 {
            (completed_requests as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        ConsensusStats {
            total_nodes: self.nodes.len(),
            active_nodes,
            threshold: self.config.threshold,
            total_requests,
            completed_requests,
            consensus_rate,
            last_updated: Utc::now(),
        }
    }

    /// Obtém assinatura por ID
    pub fn get_signature(&self, signature_id: &str) -> Option<&ThresholdSignature> {
        self.completed_signatures.get(signature_id)
    }

    /// Lista todas as assinaturas
    pub fn list_signatures(&self) -> Vec<&ThresholdSignature> {
        self.completed_signatures.values().collect()
    }

    /// Limpa assinaturas expiradas
    pub fn cleanup_expired(&mut self) -> usize {
        let now = Utc::now();
        let mut removed = 0;

        // Remover requisições expiradas
        self.pending_requests.retain(|_, request| {
            if now > request.expires_at {
                removed += 1;
                false
            } else {
                true
            }
        });

        // Remover assinaturas expiradas
        self.completed_signatures.retain(|_, signature| {
            if now > signature.expires_at {
                removed += 1;
                false
            } else {
                true
            }
        });

        removed
    }

    /// Assina mensagem com chave privada
    fn sign_with_key(&self, key_pair: &Ed25519KeyPair, message: &str) -> Result<Vec<u8>> {
        let message_bytes = message.as_bytes();
        let signature = key_pair.sign(message_bytes);
        Ok(signature.as_ref().to_vec())
    }

    /// Calcula hash da mensagem
    fn hash_message(&self, message: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(message.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// Estatísticas do consenso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStats {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub threshold: usize,
    pub total_requests: usize,
    pub completed_requests: usize,
    pub consensus_rate: f64,
    pub last_updated: DateTime<Utc>,
}

/// Utilitários para threshold signatures
pub struct ThresholdUtils;

impl ThresholdUtils {
    /// Gera par de chaves para um nó
    pub fn generate_key_pair() -> Result<(Ed25519KeyPair, String)> {
        let rng = OsRng;
        let key_pair = Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new())?;
        let key_pair = Ed25519KeyPair::from_pkcs8(key_pair.as_ref())?;
        let public_key = hex::encode(key_pair.public_key().as_ref());
        Ok((key_pair, public_key))
    }

    /// Valida configuração de threshold
    pub fn validate_config(config: &ThresholdConfig) -> Result<()> {
        if config.threshold > config.total_nodes {
            return Err(anyhow!("Threshold cannot be greater than total nodes"));
        }

        if config.threshold == 0 {
            return Err(anyhow!("Threshold must be greater than 0"));
        }

        if config.total_nodes == 0 {
            return Err(anyhow!("Total nodes must be greater than 0"));
        }

        Ok(())
    }

    /// Calcula threshold ótimo baseado no número de nós
    pub fn calculate_optimal_threshold(total_nodes: usize) -> usize {
        // Threshold de 2/3 para tolerância a falhas bizantinas
        (total_nodes * 2) / 3 + 1
    }

    /// Verifica se um threshold é seguro
    pub fn is_threshold_secure(total_nodes: usize, threshold: usize) -> bool {
        // Deve ser maior que 1/3 para tolerância a falhas bizantinas
        threshold > total_nodes / 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_threshold_service_creation() {
        let config = ThresholdConfig::default();
        let service = ThresholdSignatureService::new(config);
        let stats = service.get_stats();
        
        assert_eq!(stats.total_nodes, 0);
        assert_eq!(stats.active_nodes, 0);
        assert_eq!(stats.threshold, 2);
    }

    #[test]
    fn test_add_node() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig::default());
        
        let (key_pair, public_key) = ThresholdUtils::generate_key_pair().unwrap();
        let node = ConsensusNode {
            id: "node1".to_string(),
            name: "Test Node".to_string(),
            public_key,
            is_active: true,
            trust_level: 100,
            last_seen: Utc::now(),
            signature_count: 0,
        };

        let result = service.add_node(node, key_pair);
        assert!(result.is_ok());
        
        let stats = service.get_stats();
        assert_eq!(stats.total_nodes, 1);
        assert_eq!(stats.active_nodes, 1);
    }

    #[test]
    fn test_signature_request() {
        let mut service = ThresholdSignatureService::new(ThresholdConfig::default());
        
        let request = SignatureRequest {
            id: "req1".to_string(),
            message: "Test message".to_string(),
            message_hash: service.hash_message("Test message"),
            requester_id: "user1".to_string(),
            priority: SignaturePriority::Normal,
            expires_at: Utc::now() + Duration::minutes(10),
            metadata: HashMap::new(),
        };

        let result = service.create_signature_request(request);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "req1");
    }

    #[test]
    fn test_threshold_utils() {
        // Teste de validação de configuração
        let valid_config = ThresholdConfig {
            total_nodes: 5,
            threshold: 3,
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        };
        assert!(ThresholdUtils::validate_config(&valid_config).is_ok());

        // Teste de configuração inválida
        let invalid_config = ThresholdConfig {
            total_nodes: 3,
            threshold: 5, // Threshold maior que total de nós
            timeout_seconds: 30,
            max_retries: 3,
            enable_verification: true,
        };
        assert!(ThresholdUtils::validate_config(&invalid_config).is_err());

        // Teste de threshold ótimo
        let optimal = ThresholdUtils::calculate_optimal_threshold(9);
        assert_eq!(optimal, 7); // 2/3 de 9 + 1 = 7

        // Teste de segurança do threshold
        assert!(ThresholdUtils::is_threshold_secure(9, 7));
        assert!(!ThresholdUtils::is_threshold_secure(9, 2));
    }
}