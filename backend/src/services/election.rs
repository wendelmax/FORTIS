//! Serviço de eleições do FORTIS

use anyhow::Result;

pub struct ElectionService;

impl ElectionService {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn create_election(&self, _title: &str) -> Result<String> {
        // TODO: Implementar criação de eleição
        Ok("Eleição criada com sucesso".to_string())
    }
}
