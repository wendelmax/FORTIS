//! Serviço principal para urnas eletrônicas - orquestra todos os outros serviços

use crate::models::{
    Urna, UrnaVoteRequest, UrnaVoteResponse, UrnaSyncRequest, UrnaSyncResponse,
    UrnaStatusRequest, UrnaStatusResponse, UrnaHealthCheck, UrnaStatus,
    VoteReceipt, VoteSyncStatus, UrnaAuthentication, AuthResult
};
use crate::services::urna::{
    UrnaAuthService, UrnaSyncService, UrnaSecurityService, UrnaBlockchainService,
    UrnaMonitoringService
};
use crate::config::BlockchainConfig;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use chrono::Utc;

pub struct UrnaService {
    pub auth_service: UrnaAuthService,
    pub sync_service: UrnaSyncService,
    pub security_service: UrnaSecurityService,
    pub blockchain_service: UrnaBlockchainService,
    pub monitoring_service: UrnaMonitoringService,
}

impl UrnaService {
    pub fn new(blockchain_contract_address: String, blockchain_network_id: u64, blockchain_config: BlockchainConfig) -> Self {
        Self {
            auth_service: UrnaAuthService::new(),
            sync_service: UrnaSyncService::new(),
            security_service: UrnaSecurityService::new(),
            blockchain_service: UrnaBlockchainService::new(blockchain_contract_address, blockchain_network_id, blockchain_config),
            monitoring_service: UrnaMonitoringService::new(),
        }
    }

    pub async fn initialize_urna(&mut self, urna: &Urna) -> Result<()> {
        // Inicializar ambiente seguro da urna
        self.security_service.initialize_secure_environment(urna).await?;

        // Iniciar monitoramento da urna
        self.monitoring_service.start_monitoring(urna.id).await?;

        log::info!("Urna {} initialized successfully", urna.serial_number);
        Ok(())
    }

    pub async fn process_vote(&mut self, request: UrnaVoteRequest) -> Result<UrnaVoteResponse> {
        // Verificar se a urna está ativa
        let urna = self.get_urna(request.urna_id).await?;
        if urna.status != UrnaStatus::Active {
            return Err(anyhow!("Urna não está ativa"));
        }

        // Verificar segurança da urna
        let tamper_detected = self.security_service.detect_tampering(urna.id).await?;
        if tamper_detected {
            return Err(anyhow!("Violação de segurança detectada na urna"));
        }

        // Autenticar eleitor
        let auth_result = self.auth_service.authenticate_voter(
            &urna,
            &request.biometric_data,
            request.certificate_data.as_ref(),
        ).await?;

        if auth_result.auth_result != AuthResult::Success {
            return Err(anyhow!("Falha na autenticação do eleitor"));
        }

        // Verificar elegibilidade
        let is_eligible = self.auth_service
            .check_voter_eligibility(auth_result.voter_id, request.election_id)
            .await?;

        if !is_eligible {
            return Err(anyhow!("Eleitor não elegível para esta eleição"));
        }

        // Verificar se já votou
        let already_voted = self.auth_service
            .check_already_voted(auth_result.voter_id, request.election_id)
            .await?;

        if already_voted {
            return Err(anyhow!("Eleitor já votou nesta eleição"));
        }

        // Criar voto
        let vote_id = Uuid::new_v4();
        let vote = self.create_urna_vote(vote_id, &request, &auth_result).await?;

        // Criptografar dados do voto
        let encrypted_vote_data = self.security_service.encrypt_vote_data(
            &serde_json::to_vec(&vote)?
        ).await?;

        // Assinar voto
        let signature = self.security_service.sign_data(&encrypted_vote_data).await?;

        // Registrar voto no blockchain
        let blockchain_hash = self.blockchain_service.register_vote_on_blockchain(&vote).await?;

        // Sincronizar voto
        self.sync_service.queue_vote_for_sync(urna.id, vote.clone()).await?;

        // Criar comprovante
        let receipt = self.create_vote_receipt(&vote, blockchain_hash.as_str()).await?;

        // Log de auditoria
        self.log_vote_event(&urna, &vote, &auth_result).await?;

        Ok(UrnaVoteResponse {
            vote_id,
            success: true,
            message: "Voto registrado com sucesso".to_string(),
            receipt: Some(receipt),
            sync_status: VoteSyncStatus::Pending,
        })
    }

    async fn get_urna(&self, urna_id: Uuid) -> Result<Urna> {
        // Em implementação real, buscaria no banco de dados
        Ok(Urna {
            id: urna_id,
            serial_number: "URNA001".to_string(),
            model: "FORTIS-2025".to_string(),
            location: crate::models::UrnaLocation {
                state: "SP".to_string(),
                city: "São Paulo".to_string(),
                zone: "001".to_string(),
                section: "001".to_string(),
                address: "Rua das Flores, 123".to_string(),
                coordinates: None,
            },
            status: UrnaStatus::Active,
            last_sync: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn create_urna_vote(
        &self,
        vote_id: Uuid,
        request: &UrnaVoteRequest,
        auth: &UrnaAuthentication,
    ) -> Result<crate::models::UrnaVote> {
        Ok(crate::models::UrnaVote {
            id: vote_id,
            urna_id: request.urna_id,
            election_id: request.election_id,
            voter_id: auth.voter_id,
            candidate_id: request.candidate_id,
            vote_data: crate::models::EncryptedVoteData {
                encrypted_content: "encrypted_content".to_string(),
                encryption_key_id: "key_123".to_string(),
                signature: "signature".to_string(),
                zk_proof: request.vote_proof.clone(),
            },
            biometric_hash: request.biometric_data.biometric_hash.clone(),
            timestamp: Utc::now(),
            sync_status: VoteSyncStatus::Pending,
            blockchain_hash: None,
        })
    }

    async fn create_vote_receipt(
        &self,
        vote: &crate::models::UrnaVote,
        blockchain_hash: &str,
    ) -> Result<VoteReceipt> {
        Ok(VoteReceipt {
            vote_id: vote.id,
            election_id: vote.election_id,
            candidate_number: 123, // Em implementação real, buscaria do banco
            candidate_name: "Candidato Exemplo".to_string(),
            timestamp: vote.timestamp,
            qr_code: format!("QR_CODE_{}", vote.id),
            blockchain_hash: Some(blockchain_hash.to_string()),
        })
    }

    async fn log_vote_event(
        &self,
        urna: &Urna,
        vote: &crate::models::UrnaVote,
        auth: &UrnaAuthentication,
    ) -> Result<()> {
        // Log de auditoria do voto
        self.security_service.log_security_event(
            urna.id,
            crate::services::urna::security::SecurityEventType::SoftwareAnomaly,
            crate::services::urna::security::SecuritySeverity::Low,
            "Voto registrado com sucesso",
            serde_json::json!({
                "vote_id": vote.id,
                "voter_id": vote.voter_id,
                "candidate_id": vote.candidate_id,
                "auth_method": format!("{:?}", auth.auth_method)
            }),
        ).await?;

        Ok(())
    }

    pub async fn start_sync(&self, request: UrnaSyncRequest) -> Result<UrnaSyncResponse> {
        self.sync_service.start_sync(request).await
    }

    pub async fn get_sync_status(&self, sync_id: Uuid) -> Result<Option<crate::models::UrnaSync>> {
        self.sync_service.get_sync_status(sync_id).await
    }

    pub async fn get_urna_status(&self, urna_id: Uuid) -> Result<UrnaStatusResponse> {
        let urna = self.get_urna(urna_id).await?;
        let health = self.monitoring_service.get_health_status(urna_id).await?
            .unwrap_or_else(|| UrnaHealthCheck {
                urna_id,
                timestamp: Utc::now(),
                status: UrnaStatus::Unknown,
                battery_level: None,
                storage_usage: None,
                network_connectivity: false,
                last_sync: None,
                errors: Vec::new(),
                performance_metrics: crate::models::PerformanceMetrics {
                    cpu_usage: 0.0,
                    memory_usage: 0.0,
                    disk_usage: 0.0,
                    network_latency: None,
                    response_time: 0,
                },
            });

        let pending_votes = self.sync_service.get_pending_votes_count(urna_id).await?;

        Ok(UrnaStatusResponse {
            urna,
            health,
            pending_votes,
            last_activity: Some(Utc::now()),
        })
    }

    pub async fn get_urna_health(&self, urna_id: Uuid) -> Result<UrnaHealthCheck> {
        self.monitoring_service.perform_health_check(urna_id).await
    }

    pub async fn register_urna(&mut self, urna: Urna) -> Result<Urna> {
        // Registrar nova urna no sistema
        // Em implementação real, salvaria no banco de dados
        
        // Inicializar urna
        self.initialize_urna(&urna).await?;

        log::info!("Urna {} registered successfully", urna.serial_number);
        Ok(urna)
    }

    pub async fn get_urna_votes(
        &self,
        urna_id: Uuid,
        election_id: Option<Uuid>,
    ) -> Result<Vec<crate::models::UrnaVote>> {
        // Obter votos da urna
        // Em implementação real, buscaria no banco de dados
        Ok(Vec::new())
    }

    pub async fn get_urna_audit_logs(
        &self,
        urna_id: Uuid,
        event_type: Option<crate::models::AuditEventType>,
    ) -> Result<Vec<crate::models::UrnaAuditLog>> {
        self.security_service.get_audit_logs(urna_id).await
    }

    pub async fn get_urna_performance_history(
        &self,
        urna_id: Uuid,
        hours: i64,
    ) -> Result<Vec<crate::models::PerformanceMetrics>> {
        self.monitoring_service.get_performance_history(urna_id, hours).await
    }

    pub async fn get_urnas_with_issues(&self) -> Result<Vec<UrnaHealthCheck>> {
        self.monitoring_service.get_urnas_with_issues().await
    }

    pub async fn generate_health_report(&self) -> Result<crate::services::urna::monitoring::HealthReport> {
        self.monitoring_service.generate_health_report().await
    }

    pub async fn emergency_shutdown(&mut self, urna_id: Uuid) -> Result<()> {
        // Desligamento de emergência da urna
        self.security_service.emergency_shutdown(urna_id).await?;
        log::warn!("Emergency shutdown activated for urna {}", urna_id);
        Ok(())
    }

    pub async fn maintenance_mode(&mut self, urna_id: Uuid, enable: bool) -> Result<()> {
        // Ativar/desativar modo de manutenção
        // Em implementação real, atualizaria status no banco de dados
        let status = if enable { UrnaStatus::Maintenance } else { UrnaStatus::Active };
        log::info!("Urna {} maintenance mode: {}", urna_id, enable);
        Ok(())
    }
}
