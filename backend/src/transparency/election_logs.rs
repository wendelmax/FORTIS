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
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, KeyPair};

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
}

/// Tipos de eventos eleitorais
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub log_index: u64,
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
    pub public_key: Vec<u8>,
    pub is_active: bool,
    pub trust_level: u8, // 0-100
}

/// Sistema de logs transparentes para eleições
#[derive(Clone)]
pub struct ElectionTransparencyLog {
    merkle_tree: MerkleTree,
    log_entries: Vec<ElectionLogEntry>,
    verifiers: Vec<LogVerifier>,
    next_index: u64,
    pub config: LogConfig,
    audit_trail: Vec<AuditEvent>,
    performance_metrics: PerformanceMetrics,
}

/// Configuração do log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub min_verifiers: usize,
    pub max_verifiers: usize,
    pub signature_threshold: usize,
    pub retention_days: u64,
    pub enable_audit_trail: bool,
    pub enable_performance_metrics: bool,
    pub max_entries_per_batch: usize,
    pub verification_timeout_seconds: u64,
}

/// Dados do evento de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventData {
    pub election_id: Option<String>,
    pub voter_id: Option<String>,
    pub node_id: Option<String>,
    pub candidate_id: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub metadata: serde_json::Value,
    pub previous_hash: Option<String>,
}

/// Evento de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub actor: String,
    pub action: String,
    pub target: String,
    pub data: AuditEventData,
    pub hash: String,
    pub signature: String,
    pub block_number: Option<u64>,
    pub transaction_hash: Option<String>,
    // Campos adicionais para compatibilidade
    pub id: String,
    pub user_id: Option<String>,
    pub details: serde_json::Value,
    pub severity: AuditSeverity,
}

/// Tipos de eventos de auditoria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditEventType {
    LogEntryCreated,
    LogEntryVerified,
    LogEntryFailed,
    VerifierAdded,
    VerifierRemoved,
    ConfigChanged,
    SecurityAlert,
    PerformanceAlert,
    SystemError,
    VoteCast,
    VoteVerified,
    AuditCreated,
}

/// Severidade do evento de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Métricas de performance do log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_operations: u64,
    pub average_verification_time_ms: f64,
    pub average_append_time_ms: f64,
    pub success_rate: f64,
    pub error_rate: f64,
    pub last_updated: DateTime<Utc>,
}

impl ElectionTransparencyLog {
    pub fn new(config: LogConfig) -> Self {
        Self {
            merkle_tree: MerkleTree::new(),
            log_entries: Vec::new(),
            verifiers: Vec::new(),
            next_index: 0,
            config,
            audit_trail: Vec::new(),
            performance_metrics: PerformanceMetrics {
                total_operations: 0,
                average_verification_time_ms: 0.0,
                average_append_time_ms: 0.0,
                success_rate: 100.0,
                error_rate: 0.0,
                last_updated: Utc::now(),
            },
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
            log_index: complete_entry.index,
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
        // Simular verificação de assinatura para demonstração
        let message_bytes = message.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(message_bytes);
        hasher.update(&verifier.public_key);
        let expected_hash = hasher.finalize();
        let expected_signature = hex::encode(expected_hash);
        
        Ok(signature == expected_signature)
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
                public_key: hex::encode(&verifier.public_key),
                timestamp: Utc::now(),
            });
        }
        
        Ok(signatures)
    }

    /// Assina com verificador
    fn sign_with_verifier(&self, verifier: &LogVerifier, message: &str) -> Result<String> {
        // Simular assinatura para demonstração
        let message_bytes = message.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(message_bytes);
        hasher.update(&verifier.public_key);
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
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
            .filter(|entry| {
                // Verificar status de verificação baseado nas assinaturas
                entry.verifier_signatures.len() >= self.config.signature_threshold
            })
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

    /// Adiciona evento de auditoria
    fn add_audit_event(&mut self, event_type: AuditEventType, details: serde_json::Value, severity: AuditSeverity) {
        if !self.config.enable_audit_trail {
            return;
        }

        let audit_event = AuditEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            actor: "system".to_string(),
            action: "audit_event".to_string(),
            target: "election_log".to_string(),
            data: AuditEventData {
                election_id: None,
                voter_id: None,
                node_id: None,
                candidate_id: None,
                error_code: None,
                error_message: None,
                metadata: details.clone(),
                previous_hash: None,
            },
            hash: String::new(),
            signature: String::new(),
            block_number: None,
            transaction_hash: None,
            // Campos adicionais para compatibilidade
            id: uuid::Uuid::new_v4().to_string(),
            user_id: None,
            details,
            severity,
        };

        self.audit_trail.push(audit_event);
    }

    /// Atualiza métricas de performance
    fn update_performance_metrics(&mut self, operation_time_ms: f64, success: bool) {
        if !self.config.enable_performance_metrics {
            return;
        }

        self.performance_metrics.total_operations += 1;
        
        // Atualizar tempo médio de operação
        let total_ops = self.performance_metrics.total_operations as f64;
        self.performance_metrics.average_append_time_ms = 
            (self.performance_metrics.average_append_time_ms * (total_ops - 1.0) + operation_time_ms) / total_ops;

        // Atualizar taxa de sucesso
        if success {
            self.performance_metrics.success_rate = 
                (self.performance_metrics.success_rate * (total_ops - 1.0) + 100.0) / total_ops;
        } else {
            self.performance_metrics.success_rate = 
                (self.performance_metrics.success_rate * (total_ops - 1.0)) / total_ops;
        }

        self.performance_metrics.error_rate = 100.0 - self.performance_metrics.success_rate;
        self.performance_metrics.last_updated = Utc::now();
    }

    /// Obtém métricas de performance
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    /// Obtém trilha de auditoria
    pub fn get_audit_trail(&self) -> &Vec<AuditEvent> {
        &self.audit_trail
    }

    /// Verifica integridade de todo o log
    pub fn verify_log_integrity(&self) -> Result<LogIntegrityReport> {
        let mut report = LogIntegrityReport {
            total_entries: self.log_entries.len(),
            verified_entries: 0,
            failed_entries: 0,
            partially_verified_entries: 0,
            integrity_score: 0.0,
            issues: Vec::new(),
        };

        for entry in &self.log_entries {
            match self.verify_event_integrity(entry)? {
                VerificationStatus::Verified => report.verified_entries += 1,
                VerificationStatus::Failed => report.failed_entries += 1,
                VerificationStatus::PartiallyVerified => report.partially_verified_entries += 1,
                VerificationStatus::Pending => {
                    report.issues.push("Entry pending verification".to_string());
                }
            }
        }

        if report.total_entries > 0 {
            report.integrity_score = (report.verified_entries as f64 / report.total_entries as f64) * 100.0;
        }

        Ok(report)
    }

    /// Busca eventos por critérios
    pub fn search_events(&self, criteria: SearchCriteria) -> Result<Vec<&ElectionLogEntry>> {
        let mut results = Vec::new();

        for entry in &self.log_entries {
            let mut matches = true;

            if let Some(event_type) = &criteria.event_type {
                if entry.event_type != *event_type {
                    matches = false;
                }
            }

            if let Some(start_time) = criteria.start_time {
                if entry.timestamp < start_time {
                    matches = false;
                }
            }

            if let Some(end_time) = criteria.end_time {
                if entry.timestamp > end_time {
                    matches = false;
                }
            }

            if let Some(election_id) = &criteria.election_id {
                if let Ok(event) = serde_json::from_slice::<ElectionEvent>(&entry.event_data) {
                    if event.election_id != *election_id {
                        matches = false;
                    }
                }
            }

            if matches {
                results.push(entry);
            }
        }

        Ok(results)
    }

    /// Exporta log para auditoria externa
    pub fn export_for_audit(&self, format: ExportFormat) -> Result<Vec<u8>> {
        match format {
            ExportFormat::Json => {
                let audit_data = serde_json::json!({
                    "log_stats": self.get_log_stats(),
                    "entries": self.log_entries,
                    "verifiers": self.verifiers.iter().map(|v| serde_json::json!({
                        "id": v.id,
                        "name": v.name,
                        "is_active": v.is_active,
                        "trust_level": v.trust_level
                    })).collect::<Vec<_>>(),
                    "audit_trail": self.audit_trail,
                    "performance_metrics": self.performance_metrics,
                    "export_timestamp": Utc::now()
                });
                Ok(serde_json::to_vec_pretty(&audit_data)?)
            }
            ExportFormat::Csv => {
                let mut csv_data = String::new();
                csv_data.push_str("index,timestamp,event_type,verification_status,verifier_count\n");
                
                for entry in &self.log_entries {
                    csv_data.push_str(&format!(
                        "{},{},{},{},{}\n",
                        entry.index,
                        entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                        format!("{:?}", entry.event_type),
                        "Unknown", // Seria necessário calcular o status
                        entry.verifier_signatures.len()
                    ));
                }
                
                Ok(csv_data.into_bytes())
            }
        }
    }

    /// Calcula hash de dados
    fn hash_data(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Obtém entrada de log por índice
    pub fn get_log_entry(&self, index: u64) -> Option<&ElectionLogEntry> {
        self.log_entries.iter().find(|entry| entry.index == index)
    }

    /// Obtém todas as entradas de log
    pub fn get_all_entries(&self) -> &Vec<ElectionLogEntry> {
        &self.log_entries
    }

    /// Obtém verificadores ativos
    pub fn get_active_verifiers(&self) -> Vec<&LogVerifier> {
        self.verifiers.iter().filter(|v| v.is_active).collect()
    }

    /// Adiciona verificador com validação
    pub fn add_verifier_with_validation(&mut self, verifier: LogVerifier) -> Result<()> {
        if self.verifiers.len() >= self.config.max_verifiers {
            return Err(anyhow!("Maximum verifiers reached: {}", self.config.max_verifiers));
        }

        if self.verifiers.iter().any(|v| v.id == verifier.id) {
            return Err(anyhow!("Verifier with ID {} already exists", verifier.id));
        }

        self.verifiers.push(verifier);
        self.add_audit_event(
            AuditEventType::VerifierAdded,
            serde_json::json!({"verifier_count": self.verifiers.len()}),
            AuditSeverity::Info
        );

        Ok(())
    }

    /// Remove verificador
    pub fn remove_verifier(&mut self, verifier_id: &str) -> Result<()> {
        if let Some(pos) = self.verifiers.iter().position(|v| v.id == verifier_id) {
            self.verifiers.remove(pos);
            self.add_audit_event(
                AuditEventType::VerifierRemoved,
                serde_json::json!({"verifier_id": verifier_id, "verifier_count": self.verifiers.len()}),
                AuditSeverity::Warning
            );
            Ok(())
        } else {
            Err(anyhow!("Verifier not found: {}", verifier_id))
        }
    }

    /// Atualiza configuração do log
    pub fn update_config(&mut self, new_config: LogConfig) -> Result<()> {
        let old_config = self.config.clone();
        self.config = new_config;
        
        self.add_audit_event(
            AuditEventType::ConfigChanged,
            serde_json::json!({
                "old_config": {
                    "min_verifiers": old_config.min_verifiers,
                    "max_verifiers": old_config.max_verifiers,
                    "signature_threshold": old_config.signature_threshold
                },
                "new_config": {
                    "min_verifiers": self.config.min_verifiers,
                    "max_verifiers": self.config.max_verifiers,
                    "signature_threshold": self.config.signature_threshold
                }
            }),
            AuditSeverity::Info
        );

        Ok(())
    }

    /// Limpa logs antigos baseado na retenção
    pub fn cleanup_old_logs(&mut self) -> Result<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
        let initial_count = self.log_entries.len();
        
        self.log_entries.retain(|entry| entry.timestamp >= cutoff_date);
        
        let removed_count = initial_count - self.log_entries.len();
        
        if removed_count > 0 {
            self.add_audit_event(
                AuditEventType::LogEntryCreated,
                serde_json::json!({
                    "action": "cleanup_old_logs",
                    "removed_count": removed_count,
                    "remaining_count": self.log_entries.len()
                }),
                AuditSeverity::Info
            );
        }

        Ok(removed_count)
    }

    /// Obtém estatísticas detalhadas
    pub fn get_detailed_stats(&self) -> DetailedLogStats {
        let event_type_counts = self.log_entries.iter()
            .fold(std::collections::HashMap::new(), |mut acc, entry| {
                let event_type = format!("{:?}", entry.event_type);
                *acc.entry(event_type).or_insert(0) += 1;
                acc
            });

        let verification_status_counts = self.log_entries.iter()
            .fold(std::collections::HashMap::new(), |mut acc, entry| {
                let status = if entry.verifier_signatures.len() >= self.config.signature_threshold {
                    "verified".to_string()
                } else if entry.verifier_signatures.len() > 0 {
                    "partially_verified".to_string()
                } else {
                    "pending".to_string()
                };
                *acc.entry(status).or_insert(0) += 1;
                acc
            });

        DetailedLogStats {
            basic_stats: self.get_log_stats(),
            event_type_counts,
            verification_status_counts,
            average_verification_time: self.performance_metrics.average_verification_time_ms,
            total_operations: self.performance_metrics.total_operations,
            success_rate: self.performance_metrics.success_rate,
            error_rate: self.performance_metrics.error_rate,
        }
    }

    /// Valida configuração do log
    pub fn validate_config(&self) -> Result<ConfigValidationResult> {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        if self.config.min_verifiers > self.config.max_verifiers {
            issues.push("min_verifiers cannot be greater than max_verifiers".to_string());
        }

        if self.config.signature_threshold > self.verifiers.len() {
            issues.push("signature_threshold cannot be greater than number of verifiers".to_string());
        }

        if self.config.signature_threshold < self.config.min_verifiers {
            warnings.push("signature_threshold is less than min_verifiers".to_string());
        }

        if self.verifiers.len() < self.config.min_verifiers {
            issues.push("Number of verifiers is below minimum required".to_string());
        }

        let is_valid = issues.is_empty();
        let severity = if issues.is_empty() && warnings.is_empty() {
            "valid"
        } else if issues.is_empty() {
            "warning"
        } else {
            "error"
        };

        Ok(ConfigValidationResult {
            is_valid,
            severity: severity.to_string(),
            issues,
            warnings,
        })
    }
}

/// Árvore Merkle otimizada para logs transparentes
#[derive(Debug, Clone)]
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

/// Critérios de busca para eventos
#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub event_type: Option<ElectionEventType>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub election_id: Option<String>,
    pub verification_status: Option<VerificationStatus>,
}

/// Relatório de integridade do log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogIntegrityReport {
    pub total_entries: usize,
    pub verified_entries: usize,
    pub failed_entries: usize,
    pub partially_verified_entries: usize,
    pub integrity_score: f64,
    pub issues: Vec<String>,
}

/// Formatos de exportação
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Csv,
}

/// Estatísticas detalhadas do log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedLogStats {
    pub basic_stats: LogStats,
    pub event_type_counts: std::collections::HashMap<String, usize>,
    pub verification_status_counts: std::collections::HashMap<String, usize>,
    pub average_verification_time: f64,
    pub total_operations: u64,
    pub success_rate: f64,
    pub error_rate: f64,
}

/// Resultado de validação de configuração
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidationResult {
    pub is_valid: bool,
    pub severity: String,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
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
            enable_audit_trail: true,
            enable_performance_metrics: true,
            max_entries_per_batch: 100,
            verification_timeout_seconds: 30,
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
