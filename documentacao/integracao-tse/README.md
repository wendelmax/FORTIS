# FORTIS - Integração TSE e Gov.br
## Backend Architect Perspective

### 🎯 **Visão Geral da Integração**

O FORTIS implementa integração completa com os sistemas do TSE (Tribunal Superior Eleitoral) e Gov.br para autenticação, validação de eleitores e conformidade legal, garantindo total compatibilidade com a infraestrutura eleitoral brasileira existente.

---

## 🏛️ **Arquitetura de Integração**

### **1. Componentes de Integração**
```
┌─────────────────────────────────────────────────────────┐
│                INTEGRATION LAYER                        │
├─────────────────────────────────────────────────────────┤
│ • TSE API Gateway (validação de eleitores)              │
│ • Gov.br OAuth2 (autenticação digital)                  │
│ • Certificado Digital (assinatura e validação)          │
│ • Biometria TSE (verificação biométrica)                │
│ • Sincronização de dados (eleições e candidatos)        │
└─────────────────────────────────────────────────────────┘
```

### **2. Fluxo de Integração**
```
Eleitor → Gov.br Auth → TSE Validation → FORTIS → Blockchain
    ↓           ↓              ↓           ↓         ↓
  eCPF    OAuth2 Token   Voter Status   Vote    Immutable
```

---

## 🔐 **Integração Gov.br**

### **1. OAuth2 Configuration**
```yaml
# gov-br-config.yaml
gov_br:
  client_id: "fortis-voting-system"
  client_secret: "vault:secret/data/fortis#GOV_BR_CLIENT_SECRET"
  redirect_uri: "https://api.fortis.gov.br/auth/gov-br/callback"
  scope: "openid profile cpf"
  authorization_url: "https://sso.acesso.gov.br/authorize"
  token_url: "https://sso.acesso.gov.br/token"
  userinfo_url: "https://sso.acesso.gov.br/userinfo"
  jwks_url: "https://sso.acesso.gov.br/.well-known/jwks.json"
```

### **2. Implementação Rust**
```rust
// src/integration/gov_br.rs
use serde::{Deserialize, Serialize};
use reqwest::Client;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

#[derive(Debug, Serialize, Deserialize)]
pub struct GovBrConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: String,
    pub authorization_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub jwks_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovBrTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GovBrUserInfo {
    pub sub: String, // CPF
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub cpf: String,
    pub birthdate: Option<String>,
}

pub struct GovBrIntegration {
    config: GovBrConfig,
    client: Client,
    jwks: Jwks,
}

impl GovBrIntegration {
    pub fn new(config: GovBrConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            jwks: Jwks::new(),
        }
    }
    
    /// Gerar URL de autorização
    pub fn get_authorization_url(&self, state: &str) -> String {
        let mut params = std::collections::HashMap::new();
        params.insert("response_type", "code");
        params.insert("client_id", &self.config.client_id);
        params.insert("redirect_uri", &self.config.redirect_uri);
        params.insert("scope", &self.config.scope);
        params.insert("state", state);
        
        let query_string = serde_urlencoded::to_string(&params).unwrap();
        format!("{}?{}", self.config.authorization_url, query_string)
    }
    
    /// Trocar código por token
    pub async fn exchange_code_for_token(
        &self,
        code: &str,
        state: &str,
    ) -> Result<GovBrTokenResponse, IntegrationError> {
        let mut params = std::collections::HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("code", code);
        params.insert("redirect_uri", &self.config.redirect_uri);
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &self.config.client_secret);
        
        let response = self.client
            .post(&self.config.token_url)
            .form(&params)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(IntegrationError::TokenExchangeFailed);
        }
        
        let token_response: GovBrTokenResponse = response.json().await?;
        Ok(token_response)
    }
    
    /// Obter informações do usuário
    pub async fn get_user_info(
        &self,
        access_token: &str,
    ) -> Result<GovBrUserInfo, IntegrationError> {
        let response = self.client
            .get(&self.config.userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(IntegrationError::UserInfoFailed);
        }
        
        let user_info: GovBrUserInfo = response.json().await?;
        Ok(user_info)
    }
    
    /// Validar ID Token
    pub async fn validate_id_token(
        &self,
        id_token: &str,
    ) -> Result<GovBrUserInfo, IntegrationError> {
        // Carregar JWKS se necessário
        if self.jwks.keys.is_empty() {
            self.jwks.load_from_url(&self.config.jwks_url).await?;
        }
        
        // Decodificar e validar JWT
        let header = jsonwebtoken::decode_header(id_token)?;
        let kid = header.kid.ok_or(IntegrationError::InvalidToken)?;
        
        let key = self.jwks.get_key(&kid)?;
        let validation = Validation::new(Algorithm::RS256);
        
        let token_data = decode::<GovBrUserInfo>(
            id_token,
            &DecodingKey::from_rsa_components(&key.n, &key.e)?,
            &validation,
        )?;
        
        Ok(token_data.claims)
    }
}
```

---

## 🏛️ **Integração TSE**

### **1. TSE API Configuration**
```yaml
# tse-config.yaml
tse:
  base_url: "https://api.tse.jus.br"
  api_version: "v1"
  timeout: 30
  retry_attempts: 3
  endpoints:
    voter_validation: "/eleitores/validar"
    election_data: "/eleicoes"
    candidates: "/candidatos"
    polling_stations: "/secoes"
    results: "/resultados"
  authentication:
    certificate_path: "vault:secret/data/fortis#TSE_CERTIFICATE"
    private_key_path: "vault:secret/data/fortis#TSE_PRIVATE_KEY"
    ca_cert_path: "vault:secret/data/fortis#TSE_CA_CERT"
```

### **2. Implementação Rust**
```rust
// src/integration/tse.rs
use serde::{Deserialize, Serialize};
use reqwest::Client;
use rustls::{ClientConfig, RootCertStore};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TseConfig {
    pub base_url: String,
    pub api_version: String,
    pub timeout: u64,
    pub retry_attempts: u32,
    pub endpoints: TseEndpoints,
    pub authentication: TseAuth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TseEndpoints {
    pub voter_validation: String,
    pub election_data: String,
    pub candidates: String,
    pub polling_stations: String,
    pub results: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TseAuth {
    pub certificate_path: String,
    pub private_key_path: String,
    pub ca_cert_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoterValidationRequest {
    pub cpf: String,
    pub birth_date: String,
    pub mother_name: String,
    pub polling_station: String,
    pub zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoterValidationResponse {
    pub valid: bool,
    pub voter_id: Option<String>,
    pub name: Option<String>,
    pub polling_station: Option<PollingStation>,
    pub election_status: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollingStation {
    pub code: String,
    pub name: String,
    pub address: String,
    pub zone: String,
    pub city: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectionData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub party: String,
    pub position: String,
    pub number: String,
    pub photo_url: Option<String>,
    pub biography: Option<String>,
}

pub struct TseIntegration {
    config: TseConfig,
    client: Client,
}

impl TseIntegration {
    pub fn new(config: TseConfig) -> Result<Self, IntegrationError> {
        // Configurar cliente HTTPS com certificados
        let mut root_store = RootCertStore::empty();
        
        // Carregar CA certificate
        let ca_cert = std::fs::read(&config.authentication.ca_cert_path)?;
        root_store.add_parsable_certificates(&[ca_cert])?;
        
        let client_config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_single_cert(
                std::fs::read(&config.authentication.certificate_path)?,
                std::fs::read(&config.authentication.private_key_path)?,
            )?;
        
        let client = Client::builder()
            .use_rustls_tls(Arc::new(client_config))
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()?;
        
        Ok(Self { config, client })
    }
    
    /// Validar eleitor
    pub async fn validate_voter(
        &self,
        request: VoterValidationRequest,
    ) -> Result<VoterValidationResponse, IntegrationError> {
        let url = format!(
            "{}/{}{}",
            self.config.base_url,
            self.config.api_version,
            self.config.endpoints.voter_validation
        );
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(IntegrationError::TseValidationFailed);
        }
        
        let validation_response: VoterValidationResponse = response.json().await?;
        Ok(validation_response)
    }
    
    /// Obter dados da eleição
    pub async fn get_election_data(
        &self,
        election_id: &str,
    ) -> Result<ElectionData, IntegrationError> {
        let url = format!(
            "{}/{}{}/{}",
            self.config.base_url,
            self.config.api_version,
            self.config.endpoints.election_data,
            election_id
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(IntegrationError::TseDataFetchFailed);
        }
        
        let election_data: ElectionData = response.json().await?;
        Ok(election_data)
    }
    
    /// Obter candidatos
    pub async fn get_candidates(
        &self,
        election_id: &str,
    ) -> Result<Vec<Candidate>, IntegrationError> {
        let url = format!(
            "{}/{}{}?eleicao={}",
            self.config.base_url,
            self.config.api_version,
            self.config.endpoints.candidates,
            election_id
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(IntegrationError::TseDataFetchFailed);
        }
        
        let candidates: Vec<Candidate> = response.json().await?;
        Ok(candidates)
    }
    
    /// Obter seções eleitorais
    pub async fn get_polling_stations(
        &self,
        city: &str,
        state: &str,
    ) -> Result<Vec<PollingStation>, IntegrationError> {
        let url = format!(
            "{}/{}{}?cidade={}&estado={}",
            self.config.base_url,
            self.config.api_version,
            self.config.endpoints.polling_stations,
            city,
            state
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(IntegrationError::TseDataFetchFailed);
        }
        
        let polling_stations: Vec<PollingStation> = response.json().await?;
        Ok(polling_stations)
    }
    
    /// Sincronizar dados da eleição
    pub async fn sync_election_data(
        &self,
        election_id: &str,
    ) -> Result<ElectionData, IntegrationError> {
        let election_data = self.get_election_data(election_id).await?;
        let candidates = self.get_candidates(election_id).await?;
        
        // Atualizar dados no banco local
        // TODO: Implementar sincronização com banco de dados
        
        Ok(election_data)
    }
}
```

---

## 🔐 **Certificado Digital**

### **1. Implementação de Certificado Digital**
```rust
// src/integration/certificate.rs
use openssl::x509::X509;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use openssl::sign::Signer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DigitalCertificate {
    pub certificate: X509,
    pub private_key: PKey<openssl::pkey::Private>,
    pub serial_number: String,
    pub subject: String,
    pub issuer: String,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: chrono::DateTime<chrono::Utc>,
}

impl DigitalCertificate {
    pub fn from_files(
        cert_path: &str,
        key_path: &str,
    ) -> Result<Self, CertificateError> {
        let cert_pem = std::fs::read(cert_path)?;
        let key_pem = std::fs::read(key_path)?;
        
        let certificate = X509::from_pem(&cert_pem)?;
        let private_key = PKey::private_key_from_pem(&key_pem)?;
        
        let serial_number = certificate.serial_number().to_string();
        let subject = certificate.subject_name().to_string();
        let issuer = certificate.issuer_name().to_string();
        
        let valid_from = certificate.not_before().to_string();
        let valid_to = certificate.not_after().to_string();
        
        Ok(Self {
            certificate,
            private_key,
            serial_number,
            subject,
            issuer,
            valid_from: chrono::DateTime::parse_from_rfc3339(&valid_from)?.with_timezone(&chrono::Utc),
            valid_to: chrono::DateTime::parse_from_rfc3339(&valid_to)?.with_timezone(&chrono::Utc),
        })
    }
    
    /// Verificar se o certificado é válido
    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now();
        now >= self.valid_from && now <= self.valid_to
    }
    
    /// Assinar dados com o certificado
    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, CertificateError> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.private_key)?;
        signer.update(data)?;
        Ok(signer.sign_to_vec()?)
    }
    
    /// Verificar assinatura
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, CertificateError> {
        let mut verifier = openssl::sign::Verifier::new(MessageDigest::sha256(), &self.private_key)?;
        verifier.update(data)?;
        Ok(verifier.verify(signature)?)
    }
    
    /// Obter informações do certificado
    pub fn get_info(&self) -> CertificateInfo {
        CertificateInfo {
            serial_number: self.serial_number.clone(),
            subject: self.subject.clone(),
            issuer: self.issuer.clone(),
            valid_from: self.valid_from,
            valid_to: self.valid_to,
            is_valid: self.is_valid(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub serial_number: String,
    pub subject: String,
    pub issuer: String,
    pub valid_from: chrono::DateTime<chrono::Utc>,
    pub valid_to: chrono::DateTime<chrono::Utc>,
    pub is_valid: bool,
}
```

---

## 🔄 **Sincronização de Dados**

### **1. Sincronização Automática**
```rust
// src/integration/sync.rs
use tokio::time::{interval, Duration};
use std::collections::HashMap;

pub struct DataSync {
    tse_integration: TseIntegration,
    gov_br_integration: GovBrIntegration,
    sync_interval: Duration,
}

impl DataSync {
    pub fn new(
        tse_integration: TseIntegration,
        gov_br_integration: GovBrIntegration,
        sync_interval: Duration,
    ) -> Self {
        Self {
            tse_integration,
            gov_br_integration,
            sync_interval,
        }
    }
    
    /// Iniciar sincronização automática
    pub async fn start_sync(&self) {
        let mut interval = interval(self.sync_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.sync_all_data().await {
                log::error!("Erro na sincronização: {}", e);
            }
        }
    }
    
    /// Sincronizar todos os dados
    pub async fn sync_all_data(&self) -> Result<(), SyncError> {
        // Sincronizar eleições ativas
        self.sync_active_elections().await?;
        
        // Sincronizar candidatos
        self.sync_candidates().await?;
        
        // Sincronizar seções eleitorais
        self.sync_polling_stations().await?;
        
        // Sincronizar dados de eleitores (se necessário)
        self.sync_voter_data().await?;
        
        Ok(())
    }
    
    /// Sincronizar eleições ativas
    async fn sync_active_elections(&self) -> Result<(), SyncError> {
        // TODO: Implementar lógica de sincronização de eleições
        Ok(())
    }
    
    /// Sincronizar candidatos
    async fn sync_candidates(&self) -> Result<(), SyncError> {
        // TODO: Implementar lógica de sincronização de candidatos
        Ok(())
    }
    
    /// Sincronizar seções eleitorais
    async fn sync_polling_stations(&self) -> Result<(), SyncError> {
        // TODO: Implementar lógica de sincronização de seções
        Ok(())
    }
    
    /// Sincronizar dados de eleitores
    async fn sync_voter_data(&self) -> Result<(), SyncError> {
        // TODO: Implementar lógica de sincronização de eleitores
        Ok(())
    }
}
```

---

## 🧪 **Testes de Integração**

### **1. Testes Gov.br**
```rust
// tests/integration/gov_br_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    
    #[tokio::test]
    async fn test_gov_br_authorization_url() {
        let config = GovBrConfig {
            client_id: "test-client".to_string(),
            client_secret: "test-secret".to_string(),
            redirect_uri: "http://localhost:3000/callback".to_string(),
            scope: "openid profile cpf".to_string(),
            authorization_url: "https://sso.acesso.gov.br/authorize".to_string(),
            token_url: "https://sso.acesso.gov.br/token".to_string(),
            userinfo_url: "https://sso.acesso.gov.br/userinfo".to_string(),
            jwks_url: "https://sso.acesso.gov.br/.well-known/jwks.json".to_string(),
        };
        
        let integration = GovBrIntegration::new(config);
        let state = "test-state";
        let url = integration.get_authorization_url(state);
        
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=test-client"));
        assert!(url.contains("state=test-state"));
    }
    
    #[tokio::test]
    async fn test_gov_br_token_exchange() {
        let _m = mock("POST", "/token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"access_token":"test-token","token_type":"Bearer","expires_in":3600}"#)
            .create();
            
        let config = create_test_config();
        let integration = GovBrIntegration::new(config);
        
        let result = integration.exchange_code_for_token("test-code", "test-state").await;
        assert!(result.is_ok());
        
        let token_response = result.unwrap();
        assert_eq!(token_response.access_token, "test-token");
        assert_eq!(token_response.token_type, "Bearer");
    }
}
```

### **2. Testes TSE**
```rust
// tests/integration/tse_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tse_voter_validation() {
        let config = create_test_tse_config();
        let integration = TseIntegration::new(config).unwrap();
        
        let request = VoterValidationRequest {
            cpf: "12345678901".to_string(),
            birth_date: "1990-01-01".to_string(),
            mother_name: "Maria Silva".to_string(),
            polling_station: "1234".to_string(),
            zone: "5678".to_string(),
        };
        
        let result = integration.validate_voter(request).await;
        assert!(result.is_ok());
    }
}
```

---

## 📊 **Monitoramento de Integração**

### **1. Métricas de Integração**
```rust
// src/integration/metrics.rs
use prometheus::{Counter, Histogram, Registry};

pub struct IntegrationMetrics {
    pub gov_br_requests: Counter,
    pub gov_br_errors: Counter,
    pub tse_requests: Counter,
    pub tse_errors: Counter,
    pub response_time: Histogram,
}

impl IntegrationMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let gov_br_requests = Counter::new(
            "gov_br_requests_total",
            "Total number of Gov.br requests"
        )?;
        
        let gov_br_errors = Counter::new(
            "gov_br_errors_total",
            "Total number of Gov.br errors"
        )?;
        
        let tse_requests = Counter::new(
            "tse_requests_total",
            "Total number of TSE requests"
        )?;
        
        let tse_errors = Counter::new(
            "tse_errors_total",
            "Total number of TSE errors"
        )?;
        
        let response_time = Histogram::new(
            "integration_response_time_seconds",
            "Response time for integration requests"
        )?;
        
        registry.register(Box::new(gov_br_requests.clone()))?;
        registry.register(Box::new(gov_br_errors.clone()))?;
        registry.register(Box::new(tse_requests.clone()))?;
        registry.register(Box::new(tse_errors.clone()))?;
        registry.register(Box::new(response_time.clone()))?;
        
        Ok(Self {
            gov_br_requests,
            gov_br_errors,
            tse_requests,
            tse_errors,
            response_time,
        })
    }
}
```

---

## 🔧 **Configuração de Deploy**

### **1. Docker Compose para Desenvolvimento**
```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  fortis-backend:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://fortis:password@postgres:5432/fortis
      - REDIS_URL=redis://redis:6379
      - GOV_BR_CLIENT_ID=dev-client-id
      - TSE_BASE_URL=https://api.tse.jus.br
    volumes:
      - ./certs:/app/certs
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:15
    environment:
      - POSTGRES_DB=fortis
      - POSTGRES_USER=fortis
      - POSTGRES_PASSWORD=password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

---

*Documentação de Integração TSE FORTIS - Desenvolvida pelo Backend Architect Agent*
