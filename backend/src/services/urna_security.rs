//! Serviço de segurança para urnas eletrônicas

use crate::models::{Urna, UrnaAuditLog, AuditEventType, UrnaStatus};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct UrnaSecurityService {
    pub tamper_detection: TamperDetection,
    pub secure_boot: SecureBoot,
    pub encrypted_storage: EncryptedStorage,
    pub hsm_module: HSMModule,
    pub audit_logs: RwLock<Vec<UrnaAuditLog>>,
    pub security_events: RwLock<HashMap<Uuid, Vec<SecurityEvent>>>,
}

#[derive(Debug, Clone)]
pub struct TamperDetection {
    pub sensors_active: bool,
    pub last_check: DateTime<Utc>,
    pub tamper_detected: bool,
}

#[derive(Debug, Clone)]
pub struct SecureBoot {
    pub boot_verified: bool,
    pub last_verification: DateTime<Utc>,
    pub boot_integrity_hash: String,
}

#[derive(Debug, Clone)]
pub struct EncryptedStorage {
    pub encryption_key: String,
    pub storage_encrypted: bool,
    pub last_key_rotation: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct HSMModule {
    pub module_active: bool,
    pub key_generation_count: u64,
    pub last_operation: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum SecurityEventType {
    TamperDetected,
    UnauthorizedAccess,
    InvalidAuthentication,
    DataIntegrityViolation,
    NetworkIntrusion,
    HardwareFailure,
    SoftwareAnomaly,
    KeyCompromise,
}

#[derive(Debug, Clone)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl UrnaSecurityService {
    pub fn new() -> Self {
        Self {
            tamper_detection: TamperDetection {
                sensors_active: true,
                last_check: Utc::now(),
                tamper_detected: false,
            },
            secure_boot: SecureBoot {
                boot_verified: true,
                last_verification: Utc::now(),
                boot_integrity_hash: "initial_hash".to_string(),
            },
            encrypted_storage: EncryptedStorage {
                encryption_key: "initial_key".to_string(),
                storage_encrypted: true,
                last_key_rotation: Utc::now(),
            },
            hsm_module: HSMModule {
                module_active: true,
                key_generation_count: 0,
                last_operation: Utc::now(),
            },
            audit_logs: RwLock::new(Vec::new()),
            security_events: RwLock::new(HashMap::new()),
        }
    }

    pub async fn initialize_secure_environment(&mut self, urna: &Urna) -> Result<()> {
        // Verificar integridade do hardware
        self.check_hardware_integrity().await?;

        // Inicializar boot seguro
        self.verify_secure_boot().await?;

        // Configurar armazenamento criptografado
        self.initialize_encrypted_storage().await?;

        // Inicializar módulo HSM
        self.initialize_hsm_module().await?;

        // Log de inicialização
        self.log_security_event(
            urna.id,
            SecurityEventType::SoftwareAnomaly,
            SecuritySeverity::Low,
            "Sistema de segurança inicializado",
            serde_json::json!({}),
        ).await?;

        Ok(())
    }

    async fn check_hardware_integrity(&mut self) -> Result<()> {
        // Verificar sensores de violação
        if !self.tamper_detection.sensors_active {
            return Err(anyhow!("Sensores de violação inativos"));
        }

        // Verificar se houve tentativa de violação
        if self.tamper_detection.tamper_detected {
            return Err(anyhow!("Violação de hardware detectada"));
        }

        // Atualizar última verificação
        self.tamper_detection.last_check = Utc::now();

        Ok(())
    }

    async fn verify_secure_boot(&mut self) -> Result<()> {
        // Verificar integridade da sequência de boot
        let current_hash = self.calculate_boot_hash().await?;
        
        if current_hash != self.secure_boot.boot_integrity_hash {
            return Err(anyhow!("Integridade do boot comprometida"));
        }

        self.secure_boot.boot_verified = true;
        self.secure_boot.last_verification = Utc::now();

        Ok(())
    }

    async fn calculate_boot_hash(&self) -> Result<String> {
        // Simulação de cálculo de hash do boot
        // Em implementação real, faria hash dos componentes críticos
        let boot_data = format!("boot_data_{}", Utc::now().timestamp());
        let mut hasher = Sha256::new();
        hasher.update(boot_data.as_bytes());
        let hash = hasher.finalize();
        Ok(general_purpose::STANDARD.encode(hash))
    }

    async fn initialize_encrypted_storage(&self) -> Result<()> {
        // Verificar se o armazenamento está criptografado
        if !self.encrypted_storage.storage_encrypted {
            return Err(anyhow!("Armazenamento não criptografado"));
        }

        // Verificar validade da chave de criptografia
        if self.encrypted_storage.encryption_key.is_empty() {
            return Err(anyhow!("Chave de criptografia inválida"));
        }

        Ok(())
    }

    async fn initialize_hsm_module(&mut self) -> Result<()> {
        // Verificar se o módulo HSM está ativo
        if !self.hsm_module.module_active {
            return Err(anyhow!("Módulo HSM inativo"));
        }

        // Testar geração de chave
        let test_key = self.generate_test_key().await?;
        if test_key.is_empty() {
            return Err(anyhow!("Falha na geração de chave de teste"));
        }

        self.hsm_module.last_operation = Utc::now();
        Ok(())
    }

    async fn generate_test_key(&self) -> Result<String> {
        // Simulação de geração de chave de teste
        let key_data = format!("test_key_{}", Utc::now().timestamp());
        let mut hasher = Sha256::new();
        hasher.update(key_data.as_bytes());
        let hash = hasher.finalize();
        Ok(general_purpose::STANDARD.encode(hash))
    }

    pub async fn detect_tampering(&self, urna_id: Uuid) -> Result<bool> {
        // Verificar todos os sensores de violação
        let tamper_detected = self.check_all_tamper_sensors().await?;

        if tamper_detected {
            // Log do evento de violação
            self.log_security_event(
                urna_id,
                SecurityEventType::TamperDetected,
                SecuritySeverity::Critical,
                "Violação de hardware detectada",
                serde_json::json!({
                    "sensors_triggered": true,
                    "timestamp": Utc::now()
                }),
            ).await?;

            // Desativar urna imediatamente
            self.emergency_shutdown(urna_id).await?;
            return Ok(true);
        }

        Ok(false)
    }

    async fn check_all_tamper_sensors(&self) -> Result<bool> {
        // Simulação de verificação de sensores
        // Em implementação real, faria verificação física dos sensores
        Ok(false)
    }

    async fn emergency_shutdown(&mut self, urna_id: Uuid) -> Result<()> {
        // Desativar todos os sistemas críticos
        self.tamper_detection.sensors_active = false;
        self.hsm_module.module_active = false;

        // Log do shutdown de emergência
        self.log_security_event(
            urna_id,
            SecurityEventType::HardwareFailure,
            SecuritySeverity::Critical,
            "Shutdown de emergência ativado",
            serde_json::json!({
                "reason": "tamper_detected",
                "timestamp": Utc::now()
            }),
        ).await?;

        Ok(())
    }

    pub async fn encrypt_vote_data(&mut self, vote_data: &[u8]) -> Result<Vec<u8>> {
        // Criptografar dados do voto usando HSM
        if !self.hsm_module.module_active {
            return Err(anyhow!("Módulo HSM inativo"));
        }

        // Simulação de criptografia
        // Em implementação real, usaria o HSM para criptografia real
        let mut hasher = Sha256::new();
        hasher.update(vote_data);
        hasher.update(self.encrypted_storage.encryption_key.as_bytes());
        let hash = hasher.finalize();
        
        self.hsm_module.last_operation = Utc::now();
        Ok(hash.to_vec())
    }

    pub async fn sign_data(&mut self, data: &[u8]) -> Result<String> {
        // Assinar dados usando HSM
        if !self.hsm_module.module_active {
            return Err(anyhow!("Módulo HSM inativo"));
        }

        // Simulação de assinatura digital
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(self.encrypted_storage.encryption_key.as_bytes());
        let hash = hasher.finalize();
        
        self.hsm_module.last_operation = Utc::now();
        Ok(general_purpose::STANDARD.encode(hash))
    }

    pub async fn verify_signature(&self, data: &[u8], signature: &str) -> Result<bool> {
        // Verificar assinatura digital
        let expected_signature = self.sign_data(data).await?;
        Ok(signature == expected_signature)
    }

    pub async fn log_security_event(
        &self,
        urna_id: Uuid,
        event_type: SecurityEventType,
        severity: SecuritySeverity,
        description: &str,
        details: serde_json::Value,
    ) -> Result<()> {
        let event = SecurityEvent {
            event_type: event_type.clone(),
            severity: severity.clone(),
            timestamp: Utc::now(),
            description: description.to_string(),
            details,
        };

        // Adicionar ao log de eventos de segurança
        {
            let mut security_events = self.security_events.write().await;
            security_events.entry(urna_id).or_insert_with(Vec::new).push(event.clone());
        }

        // Criar log de auditoria
        let audit_log = UrnaAuditLog {
            id: Uuid::new_v4(),
            urna_id,
            event_type: match event_type {
                SecurityEventType::TamperDetected => AuditEventType::SecurityAlert,
                SecurityEventType::UnauthorizedAccess => AuditEventType::SecurityAlert,
                SecurityEventType::InvalidAuthentication => AuditEventType::VoterAuthentication,
                SecurityEventType::DataIntegrityViolation => AuditEventType::Error,
                SecurityEventType::NetworkIntrusion => AuditEventType::SecurityAlert,
                SecurityEventType::HardwareFailure => AuditEventType::Error,
                SecurityEventType::SoftwareAnomaly => AuditEventType::Error,
                SecurityEventType::KeyCompromise => AuditEventType::SecurityAlert,
            },
            event_data: serde_json::json!({
                "severity": format!("{:?}", severity),
                "description": description,
                "details": event.details
            }),
            timestamp: Utc::now(),
            user_id: None,
            integrity_hash: self.calculate_integrity_hash(&event).await?,
        };

        // Adicionar ao log de auditoria
        {
            let mut audit_logs = self.audit_logs.write().await;
            audit_logs.push(audit_log);
        }

        // Log para console
        log::warn!(
            "Security event - Urna: {}, Type: {:?}, Severity: {:?}, Description: {}",
            urna_id,
            event_type,
            severity,
            description
        );

        Ok(())
    }

    async fn calculate_integrity_hash(&self, event: &SecurityEvent) -> Result<String> {
        let event_data = serde_json::to_string(event)?;
        let mut hasher = Sha256::new();
        hasher.update(event_data.as_bytes());
        hasher.update(self.encrypted_storage.encryption_key.as_bytes());
        let hash = hasher.finalize();
        Ok(general_purpose::STANDARD.encode(hash))
    }

    pub async fn get_security_events(&self, urna_id: Uuid) -> Result<Vec<SecurityEvent>> {
        let security_events = self.security_events.read().await;
        Ok(security_events.get(&urna_id).cloned().unwrap_or_default())
    }

    pub async fn get_audit_logs(&self, urna_id: Uuid) -> Result<Vec<UrnaAuditLog>> {
        let audit_logs = self.audit_logs.read().await;
        Ok(audit_logs
            .iter()
            .filter(|log| log.urna_id == urna_id)
            .cloned()
            .collect())
    }

    pub async fn verify_integrity(&self, urna_id: Uuid) -> Result<bool> {
        // Verificar integridade de todos os logs de auditoria
        let audit_logs = self.get_audit_logs(urna_id).await?;
        
        for log in audit_logs {
            let expected_hash = self.calculate_integrity_hash(&SecurityEvent {
                event_type: SecurityEventType::SoftwareAnomaly,
                severity: SecuritySeverity::Low,
                timestamp: log.timestamp,
                description: "".to_string(),
                details: log.event_data,
            }).await?;
            
            if log.integrity_hash != expected_hash {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub async fn rotate_encryption_key(&mut self) -> Result<()> {
        // Gerar nova chave de criptografia
        let new_key = self.generate_new_encryption_key().await?;
        
        // Atualizar chave
        self.encrypted_storage.encryption_key = new_key;
        self.encrypted_storage.last_key_rotation = Utc::now();

        log::info!("Encryption key rotated successfully");
        Ok(())
    }

    async fn generate_new_encryption_key(&mut self) -> Result<String> {
        let key_data = format!("new_key_{}", Utc::now().timestamp());
        let mut hasher = Sha256::new();
        hasher.update(key_data.as_bytes());
        hasher.update(self.hsm_module.key_generation_count.to_string().as_bytes());
        let hash = hasher.finalize();
        
        self.hsm_module.key_generation_count += 1;
        self.hsm_module.last_operation = Utc::now();
        
        Ok(general_purpose::STANDARD.encode(hash))
    }
}
