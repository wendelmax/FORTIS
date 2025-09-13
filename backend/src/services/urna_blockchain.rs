//! Serviço de integração entre urnas e blockchain

use crate::models::{UrnaVote, EncryptedVoteData, VoteSyncStatus};
use crate::services::blockchain::BlockchainService;
use crate::config::BlockchainConfig;
use anyhow::{Result, anyhow};
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

pub struct UrnaBlockchainService {
    pub blockchain_service: BlockchainService,
    pub contract_address: String,
    pub network_id: u64,
}

impl UrnaBlockchainService {
    pub fn new(contract_address: String, network_id: u64, config: BlockchainConfig) -> Self {
        Self {
            blockchain_service: BlockchainService::new(config),
            contract_address,
            network_id,
        }
    }

    pub async fn register_vote_on_blockchain(
        &self,
        urna_vote: &UrnaVote,
    ) -> Result<String> {
        // Preparar dados para o contrato inteligente
        let vote_data = self.prepare_vote_data(urna_vote).await?;

        // Enviar transação para o contrato
        let tx_hash = self.send_vote_transaction(&vote_data).await?;

        // Aguardar confirmação da transação
        let confirmed = self.wait_for_transaction_confirmation(&tx_hash).await?;

        if !confirmed {
            return Err(anyhow!("Transação não confirmada no blockchain"));
        }

        // Atualizar status do voto
        self.update_vote_blockchain_status(urna_vote.id, &tx_hash).await?;

        Ok(tx_hash)
    }

    async fn prepare_vote_data(&self, urna_vote: &UrnaVote) -> Result<VoteData> {
        // Validar dados do voto
        self.validate_vote_data(urna_vote).await?;

        // Criar estrutura de dados para o contrato
        let vote_data = VoteData {
            vote_id: urna_vote.id,
            urna_id: urna_vote.urna_id,
            election_id: urna_vote.election_id,
            voter_id: urna_vote.voter_id,
            candidate_id: urna_vote.candidate_id,
            encrypted_content: urna_vote.vote_data.encrypted_content.clone(),
            encryption_key_id: urna_vote.vote_data.encryption_key_id.clone(),
            signature: urna_vote.vote_data.signature.clone(),
            zk_proof: urna_vote.vote_data.zk_proof.clone(),
            biometric_hash: urna_vote.biometric_hash.clone(),
            timestamp: urna_vote.timestamp,
        };

        Ok(vote_data)
    }

    async fn validate_vote_data(&self, urna_vote: &UrnaVote) -> Result<()> {
        // Verificar se o voto tem todos os campos obrigatórios
        if urna_vote.vote_data.encrypted_content.is_empty() {
            return Err(anyhow!("Conteúdo criptografado vazio"));
        }

        if urna_vote.vote_data.signature.is_empty() {
            return Err(anyhow!("Assinatura ausente"));
        }

        if urna_vote.vote_data.zk_proof.is_empty() {
            return Err(anyhow!("Prova ZK ausente"));
        }

        if urna_vote.biometric_hash.is_empty() {
            return Err(anyhow!("Hash biométrico ausente"));
        }

        // Verificar se o voto não é muito antigo
        let now = Utc::now();
        let vote_age = now.signed_duration_since(urna_vote.timestamp);
        if vote_age.num_hours() > 24 {
            return Err(anyhow!("Voto muito antigo para ser registrado"));
        }

        Ok(())
    }

    async fn send_vote_transaction(&self, vote_data: &VoteData) -> Result<String> {
        // Preparar parâmetros para o contrato inteligente
        let contract_params = json!({
            "voteId": vote_data.vote_id,
            "urnaId": vote_data.urna_id,
            "electionId": vote_data.election_id,
            "voterId": vote_data.voter_id,
            "candidateId": vote_data.candidate_id,
            "encryptedContent": vote_data.encrypted_content,
            "encryptionKeyId": vote_data.encryption_key_id,
            "signature": vote_data.signature,
            "zkProof": vote_data.zk_proof,
            "biometricHash": vote_data.biometric_hash,
            "timestamp": vote_data.timestamp.timestamp()
        });

        // Enviar transação para o blockchain
        let tx_hash = self.blockchain_service
            .send_transaction_with_params(
                &self.contract_address,
                "registerVote",
                &contract_params,
            )
            .await?;

        log::info!("Vote transaction sent: {}", tx_hash);
        Ok(tx_hash)
    }

    async fn wait_for_transaction_confirmation(&self, tx_hash: &str) -> Result<bool> {
        // Aguardar confirmação da transação
        let max_attempts = 30; // 5 minutos com tentativas a cada 10 segundos
        let mut attempts = 0;

        while attempts < max_attempts {
            match self.blockchain_service.get_transaction_status(tx_hash).await {
                Ok(status) => {
                    match status.as_str() {
                        "confirmed" => return Ok(true),
                        "failed" => return Ok(false),
                        "pending" => {
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            attempts += 1;
                        }
                        _ => {
                            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            attempts += 1;
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Error checking transaction status: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    attempts += 1;
                }
            }
        }

        Ok(false)
    }

    async fn update_vote_blockchain_status(
        &self,
        vote_id: Uuid,
        blockchain_hash: &str,
    ) -> Result<()> {
        // Atualizar status do voto no banco de dados
        // Em implementação real, faria update na tabela urna_votes
        log::info!("Vote {} blockchain status updated: {}", vote_id, blockchain_hash);
        Ok(())
    }

    pub async fn verify_vote_on_blockchain(
        &self,
        vote_id: Uuid,
    ) -> Result<bool> {
        // Verificar se o voto existe no blockchain
        let contract_params = json!({
            "voteId": vote_id
        });

        let result = self.blockchain_service
            .call_contract_function(
                &self.contract_address,
                "getVote",
                &contract_params,
            )
            .await?;

        // Verificar se o resultado indica que o voto existe
        Ok(result.get("exists").and_then(|v| v.as_bool()).unwrap_or(false))
    }

    pub async fn get_vote_count_for_election(
        &self,
        election_id: Uuid,
    ) -> Result<u64> {
        // Obter contagem de votos para uma eleição específica
        let contract_params = json!({
            "electionId": election_id
        });

        let result = self.blockchain_service
            .call_contract_function(
                &self.contract_address,
                "getVoteCount",
                &contract_params,
            )
            .await?;

        result.get("count")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Invalid vote count response"))
    }

    pub async fn get_vote_count_for_candidate(
        &self,
        election_id: Uuid,
        candidate_id: Uuid,
    ) -> Result<u64> {
        // Obter contagem de votos para um candidato específico
        let contract_params = json!({
            "electionId": election_id,
            "candidateId": candidate_id
        });

        let result = self.blockchain_service
            .call_contract_function(
                &self.contract_address,
                "getCandidateVoteCount",
                &contract_params,
            )
            .await?;

        result.get("count")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Invalid candidate vote count response"))
    }

    pub async fn sync_urna_with_blockchain(
        &self,
        urna_id: Uuid,
        from_block: Option<u64>,
    ) -> Result<SyncResult> {
        // Sincronizar urna com o estado atual do blockchain
        let start_block = from_block.unwrap_or(0);
        
        // Obter eventos de votação do blockchain
        let events = self.blockchain_service
            .get_events(
                &self.contract_address,
                "VoteRegistered",
                start_block,
                None,
            )
            .await?;

        let mut votes_synced = 0;
        let mut errors = Vec::new();

        for event in events {
            match self.process_vote_event(&event).await {
                Ok(_) => votes_synced += 1,
                Err(e) => {
                    let error_msg = format!("Failed to process vote event: {}", e);
                    errors.push(error_msg);
                    log::error!("{}", error_msg);
                }
            }
        }

        Ok(SyncResult {
            urna_id,
            votes_synced,
            errors,
            last_block_processed: start_block + events.len() as u64,
            timestamp: Utc::now(),
        })
    }

    async fn process_vote_event(&self, event: &serde_json::Value) -> Result<()> {
        // Processar evento de voto do blockchain
        let vote_id = event.get("voteId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing voteId in event"))?;

        let urna_id = event.get("urnaId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing urnaId in event"))?;

        // Atualizar status do voto como confirmado
        self.update_vote_sync_status(
            Uuid::parse_str(vote_id)?,
            VoteSyncStatus::Confirmed,
        ).await?;

        log::info!("Vote {} from urna {} processed successfully", vote_id, urna_id);
        Ok(())
    }

    async fn update_vote_sync_status(
        &self,
        vote_id: Uuid,
        status: VoteSyncStatus,
    ) -> Result<()> {
        // Atualizar status de sincronização do voto
        // Em implementação real, faria update no banco de dados
        log::info!("Vote {} sync status updated to {:?}", vote_id, status);
        Ok(())
    }

    pub async fn get_urna_vote_stats(
        &self,
        urna_id: Uuid,
        election_id: Uuid,
    ) -> Result<UrnaVoteStats> {
        // Obter estatísticas de votos de uma urna específica
        let contract_params = json!({
            "urnaId": urna_id,
            "electionId": election_id
        });

        let result = self.blockchain_service
            .call_contract_function(
                &self.contract_address,
                "getUrnaStats",
                &contract_params,
            )
            .await?;

        Ok(UrnaVoteStats {
            urna_id,
            election_id,
            total_votes: result.get("totalVotes").and_then(|v| v.as_u64()).unwrap_or(0),
            synced_votes: result.get("syncedVotes").and_then(|v| v.as_u64()).unwrap_or(0),
            pending_votes: result.get("pendingVotes").and_then(|v| v.as_u64()).unwrap_or(0),
            last_sync: result.get("lastSync")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct VoteData {
    pub vote_id: Uuid,
    pub urna_id: Uuid,
    pub election_id: Uuid,
    pub voter_id: Uuid,
    pub candidate_id: Uuid,
    pub encrypted_content: String,
    pub encryption_key_id: String,
    pub signature: String,
    pub zk_proof: String,
    pub biometric_hash: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub urna_id: Uuid,
    pub votes_synced: u64,
    pub errors: Vec<String>,
    pub last_block_processed: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct UrnaVoteStats {
    pub urna_id: Uuid,
    pub election_id: Uuid,
    pub total_votes: u64,
    pub synced_votes: u64,
    pub pending_votes: u64,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
}
