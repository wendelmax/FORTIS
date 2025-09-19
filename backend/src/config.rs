use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub security: SecurityConfig,
    pub tse: TSEConfig,
    pub transparency: TransparencyConfig,
    pub consensus: ConsensusConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyConfig {
    pub log_storage_path: String,
    pub merkle_tree_depth: u32,
    pub verification_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub threshold_nodes: Vec<String>,
    pub threshold_required: usize,
    pub signature_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_key: String,
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TSEConfig {
    pub base_url: String,
    pub gov_br_base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub api_key: String,
    pub sync_interval: u64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: "postgresql://fortis:fortis123@localhost:5432/fortis_voting".to_string(),
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
            },
            transparency: TransparencyConfig {
                log_storage_path: "./logs/transparent".to_string(),
                merkle_tree_depth: 20,
                verification_nodes: vec![
                    "node1.tse.gov.br".to_string(),
                    "node2.tse.gov.br".to_string(),
                    "node3.tse.gov.br".to_string(),
                ],
            },
            consensus: ConsensusConfig {
                threshold_nodes: vec![
                    "node1.tse.gov.br".to_string(),
                    "node2.tse.gov.br".to_string(),
                    "node3.tse.gov.br".to_string(),
                ],
                threshold_required: 2,
                signature_timeout: 30,
            },
            security: SecurityConfig {
                encryption_key: "fortis_encryption_key_32_chars_long".to_string(),
                jwt_secret: "fortis_jwt_secret_key_very_long_and_secure".to_string(),
            },
            tse: TSEConfig {
                base_url: "https://api.tse.jus.br".to_string(),
                gov_br_base_url: "https://sso.acesso.gov.br".to_string(),
                client_id: "fortis_client_id".to_string(),
                client_secret: "fortis_client_secret".to_string(),
                redirect_uri: "http://localhost:3000/auth/callback".to_string(),
                api_key: "fortis_api_key".to_string(),
                sync_interval: 3600,
            },
        }
    }
}
