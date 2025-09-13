use anyhow::Result;
use sha2::{Sha256, Digest};

pub struct Commitment;

impl Commitment {
    pub fn create_commitment(data: &str) -> Result<String> {
        Ok(hex::encode(Sha256::digest(data.as_bytes())))
    }

    pub fn verify_commitment(commitment: &str, data: &str) -> Result<bool> {
        let expected = Self::create_commitment(data)?;
        Ok(commitment == expected)
    }
}
