//! Serviço de criptografia do FORTIS

use crate::crypto::CryptoService;
use anyhow::Result;

pub struct CryptoServiceWrapper {
    crypto: CryptoService,
}

impl CryptoServiceWrapper {
    pub fn new(encryption_key: &str) -> Result<Self> {
        Ok(Self {
            crypto: CryptoService::new(encryption_key)?,
        })
    }
    
    pub async fn encrypt(&self, data: &str) -> Result<String> {
        self.crypto.encrypt(data)
    }
    
    pub async fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        self.crypto.decrypt(encrypted_data)
    }
}
