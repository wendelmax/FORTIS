//! Serviço de sincronização para urnas eletrônicas

use crate::models::{
    UrnaSync, UrnaVote, SyncType, SyncStatus, VoteSyncStatus, Urna,
    UrnaSyncRequest, UrnaSyncResponse, EncryptedVoteData
};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct UrnaSyncService {
    pub sync_queue: RwLock<HashMap<Uuid, Vec<UrnaVote>>>,
    pub active_syncs: RwLock<HashMap<Uuid, UrnaSync>>,
    pub max_retry_attempts: u32,
    pub sync_timeout: Duration,
}

impl UrnaSyncService {
    pub fn new() -> Self {
        Self {
            sync_queue: RwLock::new(HashMap::new()),
            active_syncs: RwLock::new(HashMap::new()),
            max_retry_attempts: 3,
            sync_timeout: Duration::minutes(5),
        }
    }

    pub async fn start_sync(&self, request: UrnaSyncRequest) -> Result<UrnaSyncResponse> {
        let sync_id = Uuid::new_v4();
        let sync = UrnaSync {
            id: sync_id,
            urna_id: request.urna_id,
            sync_type: request.sync_type.clone(),
            status: SyncStatus::Pending,
            started_at: Utc::now(),
            completed_at: None,
            votes_synced: 0,
            errors: Vec::new(),
        };

        // Registrar sincronização ativa
        {
            let mut active_syncs = self.active_syncs.write().await;
            active_syncs.insert(sync_id, sync.clone());
        }

        // Iniciar processo de sincronização em background
        let service = self.clone();
        let urna_id = request.urna_id;
        tokio::spawn(async move {
            if let Err(e) = service.execute_sync(sync_id, request).await {
                log::error!("Sync failed for urna {}: {:?}", urna_id, e);
            }
        });

        Ok(UrnaSyncResponse {
            sync_id,
            status: SyncStatus::Pending,
            votes_synced: 0,
            errors: Vec::new(),
            estimated_completion: Some(Utc::now() + Duration::minutes(2)),
        })
    }

    async fn execute_sync(&self, sync_id: Uuid, request: UrnaSyncRequest) -> Result<()> {
        // Atualizar status para em progresso
        self.update_sync_status(sync_id, SyncStatus::InProgress).await?;

        let mut votes_synced = 0;
        let mut errors = Vec::new();

        // Obter votos pendentes da urna
        let pending_votes = self.get_pending_votes(request.urna_id).await?;

        for vote in pending_votes {
            match self.sync_vote(&vote).await {
                Ok(_) => {
                    votes_synced += 1;
                    self.mark_vote_synced(vote.id).await?;
                }
                Err(e) => {
                    let error_msg = format!("Failed to sync vote {}: {}", vote.id, e);
                    errors.push(error_msg.clone());
                    log::error!("{}", error_msg);
                }
            }
        }

        // Atualizar resultado da sincronização
        self.update_sync_result(sync_id, votes_synced, errors.clone()).await?;

        Ok(())
    }

    async fn sync_vote(&self, vote: &UrnaVote) -> Result<()> {
        // Validar voto localmente
        self.validate_vote_locally(vote).await?;

        // Criptografar dados do voto
        let encrypted_data = self.encrypt_vote_data(&vote.vote_data).await?;

        // Enviar para blockchain
        let blockchain_hash = self.send_to_blockchain(&encrypted_data).await?;

        // Enviar para rede distribuída
        self.send_to_distributed_network(&encrypted_data).await?;

        // Atualizar status do voto
        self.update_vote_sync_status(vote.id, VoteSyncStatus::Synced, Some(blockchain_hash)).await?;

        Ok(())
    }

    async fn validate_vote_locally(&self, vote: &UrnaVote) -> Result<()> {
        // Verificar integridade dos dados
        if vote.vote_data.encrypted_content.is_empty() {
            return Err(anyhow!("Conteúdo criptografado vazio"));
        }

        if vote.vote_data.signature.is_empty() {
            return Err(anyhow!("Assinatura ausente"));
        }

        if vote.vote_data.zk_proof.is_empty() {
            return Err(anyhow!("Prova ZK ausente"));
        }

        // Verificar timestamp do voto
        let now = Utc::now();
        let vote_age = now.signed_duration_since(vote.timestamp);
        if vote_age > Duration::hours(24) {
            return Err(anyhow!("Voto muito antigo"));
        }

        Ok(())
    }

    async fn encrypt_vote_data(&self, vote_data: &EncryptedVoteData) -> Result<EncryptedVoteData> {
        // Em implementação real, faria criptografia adicional
        // Por enquanto, retorna os dados como estão
        Ok(vote_data.clone())
    }

    async fn send_to_blockchain(&self, encrypted_data: &EncryptedVoteData) -> Result<String> {
        // Simulação de envio para blockchain
        // Em implementação real, faria chamada para o contrato inteligente
        let hash = format!("blockchain_hash_{}", Uuid::new_v4());
        log::info!("Vote sent to blockchain: {}", hash);
        Ok(hash)
    }

    async fn send_to_distributed_network(&self, encrypted_data: &EncryptedVoteData) -> Result<()> {
        // Simulação de envio para rede distribuída
        // Em implementação real, faria broadcast para todos os nós TSE
        log::info!("Vote sent to distributed network");
        Ok(())
    }

    async fn get_pending_votes(&self, urna_id: Uuid) -> Result<Vec<UrnaVote>> {
        // Obter votos pendentes de sincronização
        // Em implementação real, faria query no banco de dados
        Ok(Vec::new())
    }

    async fn mark_vote_synced(&self, vote_id: Uuid) -> Result<()> {
        // Marcar voto como sincronizado
        // Em implementação real, faria update no banco de dados
        log::info!("Vote {} marked as synced", vote_id);
        Ok(())
    }

    async fn update_vote_sync_status(
        &self,
        vote_id: Uuid,
        status: VoteSyncStatus,
        blockchain_hash: Option<String>,
    ) -> Result<()> {
        // Atualizar status de sincronização do voto
        // Em implementação real, faria update no banco de dados
        log::info!("Vote {} sync status updated to {:?}", vote_id, status);
        Ok(())
    }

    async fn update_sync_status(&self, sync_id: Uuid, status: SyncStatus) -> Result<()> {
        let mut active_syncs = self.active_syncs.write().await;
        if let Some(sync) = active_syncs.get_mut(&sync_id) {
            sync.status = status;
        }
        Ok(())
    }

    async fn update_sync_result(
        &self,
        sync_id: Uuid,
        votes_synced: i32,
        errors: Vec<String>,
    ) -> Result<()> {
        let mut active_syncs = self.active_syncs.write().await;
        if let Some(sync) = active_syncs.get_mut(&sync_id) {
            sync.votes_synced = votes_synced;
            sync.errors = errors.clone();
            sync.status = if errors.is_empty() {
                SyncStatus::Completed
            } else {
                SyncStatus::Failed
            };
            sync.completed_at = Some(Utc::now());
        }
        Ok(())
    }

    pub async fn get_sync_status(&self, sync_id: Uuid) -> Result<Option<UrnaSync>> {
        let active_syncs = self.active_syncs.read().await;
        Ok(active_syncs.get(&sync_id).cloned())
    }

    pub async fn queue_vote_for_sync(&self, urna_id: Uuid, vote: UrnaVote) -> Result<()> {
        let mut sync_queue = self.sync_queue.write().await;
        sync_queue.entry(urna_id).or_insert_with(Vec::new).push(vote);
        Ok(())
    }

    pub async fn get_pending_votes_count(&self, urna_id: Uuid) -> Result<i32> {
        let sync_queue = self.sync_queue.read().await;
        Ok(sync_queue.get(&urna_id).map_or(0, |votes| votes.len() as i32))
    }

    pub async fn retry_failed_syncs(&self) -> Result<()> {
        let mut active_syncs = self.active_syncs.write().await;
        let failed_syncs: Vec<Uuid> = active_syncs
            .iter()
            .filter(|(_, sync)| sync.status == SyncStatus::Failed)
            .map(|(id, _)| *id)
            .collect();

        for sync_id in failed_syncs {
            if let Some(sync) = active_syncs.get_mut(&sync_id) {
                sync.status = SyncStatus::Pending;
                sync.started_at = Utc::now();
                sync.completed_at = None;
            }
        }

        Ok(())
    }

    pub async fn cleanup_completed_syncs(&self) -> Result<()> {
        let mut active_syncs = self.active_syncs.write().await;
        let completed_syncs: Vec<Uuid> = active_syncs
            .iter()
            .filter(|(_, sync)| {
                sync.status == SyncStatus::Completed || sync.status == SyncStatus::Failed
            })
            .filter(|(_, sync)| {
                if let Some(completed_at) = sync.completed_at {
                    Utc::now().signed_duration_since(completed_at) > Duration::hours(24)
                } else {
                    false
                }
            })
            .map(|(id, _)| *id)
            .collect();

        for sync_id in completed_syncs {
            active_syncs.remove(&sync_id);
        }

        Ok(())
    }
}

impl Clone for UrnaSyncService {
    fn clone(&self) -> Self {
        Self {
            sync_queue: RwLock::new(HashMap::new()),
            active_syncs: RwLock::new(HashMap::new()),
            max_retry_attempts: self.max_retry_attempts,
            sync_timeout: self.sync_timeout,
        }
    }
}
