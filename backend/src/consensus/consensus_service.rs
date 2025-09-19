//! Serviço de Consenso Distribuído
//! 
//! Integra threshold signatures com logs transparentes para fornecer
//! consenso distribuído robusto sem blockchain.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::consensus::threshold_signatures::*;
use crate::transparency::election_logs::*;
use sha2::{Sha256, Digest};

/// Configuração do serviço de consenso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusServiceConfig {
    pub threshold_config: ThresholdConfig,
    pub consensus_timeout: Duration,
    pub retry_interval: Duration,
    pub max_consensus_attempts: u32,
    pub enable_audit_logging: bool,
    pub enable_metrics: bool,
}

impl Default for ConsensusServiceConfig {
    fn default() -> Self {
        Self {
            threshold_config: ThresholdConfig::default(),
            consensus_timeout: Duration::seconds(60),
            retry_interval: Duration::seconds(5),
            max_consensus_attempts: 3,
            enable_audit_logging: true,
            enable_metrics: true,
        }
    }
}

/// Tipo de operação que requer consenso
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsensusOperation {
    ElectionStart,
    ElectionEnd,
    VoteValidation,
    AuditTrigger,
    SystemMaintenance,
    SecurityAlert,
    DataIntegrityCheck,
}

/// Requisição de consenso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusRequest {
    pub id: String,
    pub operation: ConsensusOperation,
    pub data: serde_json::Value,
    pub requester_id: String,
    pub priority: SignaturePriority,
    pub timeout: Option<Duration>,
    pub metadata: HashMap<String, String>,
}

/// Resultado do consenso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub request_id: String,
    pub operation: ConsensusOperation,
    pub consensus_reached: bool,
    pub threshold_signature: Option<ThresholdSignature>,
    pub consensus_time: Duration,
    pub participating_nodes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub audit_log_entry: Option<String>,
}

/// Métricas de consenso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMetrics {
    pub total_requests: u64,
    pub successful_consensus: u64,
    pub failed_consensus: u64,
    pub average_consensus_time_ms: f64,
    pub active_nodes: usize,
    pub consensus_rate: f64,
    pub last_updated: DateTime<Utc>,
}

/// Serviço de consenso distribuído
pub struct ConsensusService {
    config: ConsensusServiceConfig,
    threshold_service: Arc<RwLock<ThresholdSignatureService>>,
    transparency_log: Arc<RwLock<ElectionTransparencyLog>>,
    metrics: Arc<RwLock<ConsensusMetrics>>,
    pending_requests: Arc<RwLock<HashMap<String, ConsensusRequest>>>,
}

impl ConsensusService {
    /// Cria novo serviço de consenso
    pub fn new(
        config: ConsensusServiceConfig,
        transparency_log: Arc<RwLock<ElectionTransparencyLog>>,
    ) -> Self {
        let threshold_service = Arc::new(RwLock::new(
            ThresholdSignatureService::new(config.threshold_config.clone())
        ));

        let metrics = Arc::new(RwLock::new(ConsensusMetrics {
            total_requests: 0,
            successful_consensus: 0,
            failed_consensus: 0,
            average_consensus_time_ms: 0.0,
            active_nodes: 0,
            consensus_rate: 0.0,
            last_updated: Utc::now(),
        }));

        Self {
            config,
            threshold_service,
            transparency_log,
            metrics,
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Inicializa o serviço de consenso
    pub async fn initialize(&self) -> Result<()> {
        // Adicionar nós iniciais (simulado)
        self.add_initial_nodes().await?;
        
        // Inicializar métricas
        self.update_metrics().await?;
        
        Ok(())
    }

    /// Adiciona nós iniciais ao consenso
    async fn add_initial_nodes(&self) -> Result<()> {
        let mut threshold_service = self.threshold_service.write().await;
        
        for i in 1..=self.config.threshold_config.total_nodes {
            let (key_pair, public_key) = ThresholdUtils::generate_key_pair()?;
            
            let node = ConsensusNode {
                id: format!("node_{}", i),
                name: format!("Consensus Node {}", i),
                public_key,
                is_active: true,
                trust_level: 100,
                last_seen: Utc::now(),
                signature_count: 0,
            };

            threshold_service.add_node(node, key_pair)?;
        }

        Ok(())
    }

    /// Inicia processo de consenso
    pub async fn start_consensus(&self, request: ConsensusRequest) -> Result<ConsensusResult> {
        let start_time = Utc::now();
        let request_id = request.id.clone();

        // Registrar requisição pendente
        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(request_id.clone(), request.clone());
        }

        // Atualizar métricas
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_requests += 1;
        }

        // Criar requisição de assinatura
        let signature_request = SignatureRequest {
            id: request_id.clone(),
            message: serde_json::to_string(&request)?,
            message_hash: self.hash_request(&request)?,
            requester_id: request.requester_id.clone(),
            priority: request.priority.clone(),
            expires_at: start_time + request.timeout.unwrap_or(self.config.consensus_timeout),
            metadata: request.metadata.clone(),
        };

        // Processar consenso
        let result = self.process_consensus(signature_request, request, start_time).await?;

        // Registrar resultado
        self.record_consensus_result(&result).await?;

        Ok(result)
    }

    /// Processa o consenso
    async fn process_consensus(
        &self,
        signature_request: SignatureRequest,
        consensus_request: ConsensusRequest,
        start_time: DateTime<Utc>,
    ) -> Result<ConsensusResult> {
        let mut threshold_service = self.threshold_service.write().await;

        // Criar requisição de assinatura
        threshold_service.create_signature_request(signature_request)?;

        // Coletar assinaturas
        let threshold_signature = threshold_service.collect_signatures(&consensus_request.id)?;

        let consensus_time = Utc::now() - start_time;
        let consensus_reached = threshold_signature.threshold_met;

        // Obter nós participantes
        let participating_nodes: Vec<String> = threshold_signature
            .signatures
            .iter()
            .map(|s| s.node_id.clone())
            .collect();

        // Criar resultado do consenso
        let result = ConsensusResult {
            request_id: consensus_request.id.clone(),
            operation: consensus_request.operation.clone(),
            consensus_reached,
            threshold_signature: Some(threshold_signature),
            consensus_time,
            participating_nodes,
            created_at: Utc::now(),
            audit_log_entry: None,
        };

        // Registrar no log de transparência se habilitado
        if self.config.enable_audit_logging {
            self.log_consensus_event(&result).await?;
        }

        Ok(result)
    }

    /// Registra evento de consenso no log de transparência
    async fn log_consensus_event(&self, result: &ConsensusResult) -> Result<()> {
        let mut log = self.transparency_log.write().await;

        let event = ElectionEvent {
            id: format!("consensus_{}", result.request_id),
            event_type: ElectionEventType::SystemEvent,
            election_id: "consensus".to_string(),
            data: serde_json::json!({
                "operation": result.operation,
                "consensus_reached": result.consensus_reached,
                "participating_nodes": result.participating_nodes,
                "consensus_time_ms": result.consensus_time.num_milliseconds(),
            }),
            timestamp: result.created_at,
            source: "consensus_service".to_string(),
        };

        log.append_election_event(event)?;
        Ok(())
    }

    /// Registra resultado do consenso
    async fn record_consensus_result(&self, result: &ConsensusResult) -> Result<()> {
        // Atualizar métricas
        {
            let mut metrics = self.metrics.write().await;
            
            if result.consensus_reached {
                metrics.successful_consensus += 1;
            } else {
                metrics.failed_consensus += 1;
            }

            // Atualizar tempo médio de consenso
            let total_consensus = metrics.successful_consensus + metrics.failed_consensus;
            if total_consensus > 0 {
                let current_avg = metrics.average_consensus_time_ms;
                let new_time = result.consensus_time.num_milliseconds() as f64;
                metrics.average_consensus_time_ms = 
                    (current_avg * (total_consensus - 1) as f64 + new_time) / total_consensus as f64;
            }

            // Atualizar taxa de consenso
            metrics.consensus_rate = 
                (metrics.successful_consensus as f64 / total_consensus as f64) * 100.0;

            metrics.last_updated = Utc::now();
        }

        // Remover requisição pendente
        {
            let mut pending = self.pending_requests.write().await;
            pending.remove(&result.request_id);
        }

        Ok(())
    }

    /// Obtém métricas do consenso
    pub async fn get_metrics(&self) -> ConsensusMetrics {
        self.metrics.read().await.clone()
    }

    /// Obtém estatísticas do threshold service
    pub async fn get_threshold_stats(&self) -> ConsensusStats {
        let threshold_service = self.threshold_service.read().await;
        threshold_service.get_stats()
    }

    /// Obtém resultado de consenso por ID
    pub async fn get_consensus_result(&self, request_id: &str) -> Option<ConsensusResult> {
        let threshold_service = self.threshold_service.read().await;
        if let Some(threshold_signature) = threshold_service.get_signature(request_id) {
        Some(ConsensusResult {
            request_id: request_id.to_string(),
            operation: ConsensusOperation::SystemMaintenance, // Placeholder
            consensus_reached: threshold_signature.threshold_met,
            threshold_signature: Some(threshold_signature.clone()),
            consensus_time: Duration::zero(),
            participating_nodes: threshold_signature.signatures.iter()
                .map(|s| s.node_id.clone()).collect(),
            created_at: threshold_signature.created_at,
            audit_log_entry: None,
        })
        } else {
            None
        }
    }

    /// Lista todas as requisições de consenso
    pub async fn list_consensus_requests(&self) -> Vec<ConsensusRequest> {
        let pending = self.pending_requests.read().await;
        pending.values().cloned().collect()
    }

    /// Limpa requisições expiradas
    pub async fn cleanup_expired(&self) -> usize {
        let mut removed = 0;
        let now = Utc::now();

        // Limpar requisições pendentes expiradas
        {
            let mut pending = self.pending_requests.write().await;
            pending.retain(|_, request| {
                let timeout = request.timeout.unwrap_or(self.config.consensus_timeout);
                if now - request.metadata.get("created_at")
                    .and_then(|s| s.parse::<DateTime<Utc>>().ok())
                    .unwrap_or(now) > timeout {
                    removed += 1;
                    false
                } else {
                    true
                }
            });
        }

        // Limpar assinaturas expiradas
        {
            let mut threshold_service = self.threshold_service.write().await;
            removed += threshold_service.cleanup_expired();
        }

        removed
    }

    /// Atualiza métricas do sistema
    async fn update_metrics(&self) -> Result<()> {
        let threshold_stats = {
            let threshold_service = self.threshold_service.read().await;
            threshold_service.get_stats()
        };

        let mut metrics = self.metrics.write().await;
        metrics.active_nodes = threshold_stats.active_nodes;
        metrics.last_updated = Utc::now();

        Ok(())
    }

    /// Calcula hash da requisição
    fn hash_request(&self, request: &ConsensusRequest) -> Result<String> {
        let request_json = serde_json::to_string(request)?;
        let mut hasher = Sha256::new();
        hasher.update(request_json.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }
}

/// Utilitários para consenso
pub struct ConsensusUtils;

impl ConsensusUtils {
    /// Valida requisição de consenso
    pub fn validate_consensus_request(request: &ConsensusRequest) -> Result<()> {
        if request.id.is_empty() {
            return Err(anyhow!("Request ID cannot be empty"));
        }

        if request.requester_id.is_empty() {
            return Err(anyhow!("Requester ID cannot be empty"));
        }

        Ok(())
    }

    /// Calcula prioridade baseada na operação
    pub fn calculate_priority(operation: &ConsensusOperation) -> SignaturePriority {
        match operation {
            ConsensusOperation::SecurityAlert => SignaturePriority::Critical,
            ConsensusOperation::ElectionStart | ConsensusOperation::ElectionEnd => SignaturePriority::High,
            ConsensusOperation::VoteValidation | ConsensusOperation::AuditTrigger => SignaturePriority::Normal,
            _ => SignaturePriority::Low,
        }
    }

    /// Verifica se operação requer consenso
    pub fn requires_consensus(operation: &ConsensusOperation) -> bool {
        match operation {
            ConsensusOperation::ElectionStart
            | ConsensusOperation::ElectionEnd
            | ConsensusOperation::VoteValidation
            | ConsensusOperation::AuditTrigger
            | ConsensusOperation::SecurityAlert => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transparency::election_logs::LogConfig;

    #[tokio::test]
    async fn test_consensus_service_creation() {
        let config = ConsensusServiceConfig::default();
        let log_config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let transparency_log = Arc::new(RwLock::new(
            ElectionTransparencyLog::new(log_config)
        ));
        
        let service = ConsensusService::new(config, transparency_log);
        service.initialize().await.unwrap();
        
        let metrics = service.get_metrics().await;
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.successful_consensus, 0);
    }

    #[tokio::test]
    async fn test_consensus_request() {
        let config = ConsensusServiceConfig::default();
        let log_config = LogConfig {
            min_verifiers: 1,
            max_verifiers: 5,
            signature_threshold: 1,
            retention_days: 365,
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
        };
        
        let transparency_log = Arc::new(RwLock::new(
            ElectionTransparencyLog::new(log_config)
        ));
        
        let service = ConsensusService::new(config, transparency_log);
        service.initialize().await.unwrap();

        let request = ConsensusRequest {
            id: "test_consensus".to_string(),
            operation: ConsensusOperation::ElectionStart,
            data: serde_json::json!({"election_id": "test_election"}),
            requester_id: "admin".to_string(),
            priority: SignaturePriority::High,
            timeout: Some(Duration::minutes(5)),
            metadata: HashMap::new(),
        };

        let result = service.start_consensus(request).await;
        assert!(result.is_ok());
        
        let consensus_result = result.unwrap();
        assert_eq!(consensus_result.request_id, "test_consensus");
        assert_eq!(consensus_result.operation, ConsensusOperation::ElectionStart);
    }

    #[test]
    fn test_consensus_utils() {
        // Teste de validação de requisição
        let valid_request = ConsensusRequest {
            id: "test".to_string(),
            operation: ConsensusOperation::ElectionStart,
            data: serde_json::json!({}),
            requester_id: "admin".to_string(),
            priority: SignaturePriority::High,
            timeout: None,
            metadata: HashMap::new(),
        };
        assert!(ConsensusUtils::validate_consensus_request(&valid_request).is_ok());

        // Teste de requisição inválida
        let invalid_request = ConsensusRequest {
            id: "".to_string(), // ID vazio
            operation: ConsensusOperation::ElectionStart,
            data: serde_json::json!({}),
            requester_id: "admin".to_string(),
            priority: SignaturePriority::High,
            timeout: None,
            metadata: HashMap::new(),
        };
        assert!(ConsensusUtils::validate_consensus_request(&invalid_request).is_err());

        // Teste de cálculo de prioridade
        assert_eq!(
            ConsensusUtils::calculate_priority(&ConsensusOperation::SecurityAlert),
            SignaturePriority::Critical
        );

        // Teste de verificação de consenso necessário
        assert!(ConsensusUtils::requires_consensus(&ConsensusOperation::ElectionStart));
        assert!(!ConsensusUtils::requires_consensus(&ConsensusOperation::SystemMaintenance));
    }
}
