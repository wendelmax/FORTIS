//! Serviço de votação do FORTIS

use crate::models::VoteRequest;
use anyhow::Result;

pub struct VoteService;

impl VoteService {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn cast_vote(&self, vote: &VoteRequest) -> Result<String> {
        // TODO: Implementar votação
        Ok("Voto registrado com sucesso".to_string())
    }
}
