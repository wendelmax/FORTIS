# FORTIS - Segurança e Criptografia
## Security Specialist Perspective

### 🎯 **Visão Geral de Segurança**

O FORTIS implementa um sistema de segurança de nível militar, combinando criptografia avançada, autenticação multi-fator, blockchain e auditoria contínua para garantir a integridade e confidencialidade do processo eleitoral brasileiro.

---

## 🔐 **Arquitetura de Segurança**

### **Defense in Depth Strategy**
```
┌─────────────────────────────────────────────────────────┐
│                    SECURITY LAYERS                     │
├─────────────────────────────────────────────────────────┤
│ Layer 1: Physical Security (HSM, Tamper Detection)     │
│ Layer 2: Network Security (Firewall, DDoS Protection)  │
│ Layer 3: Application Security (WAF, Input Validation)  │
│ Layer 4: Data Security (Encryption, Access Control)    │
│ Layer 5: Identity Security (MFA, Biometric)            │
│ Layer 6: Audit Security (Logging, Monitoring)          │
└─────────────────────────────────────────────────────────┘
```

### **Princípios de Segurança**
1. **Zero Trust**: Nenhuma confiança implícita
2. **Least Privilege**: Acesso mínimo necessário
3. **Defense in Depth**: Múltiplas camadas de proteção
4. **Fail Secure**: Falha de forma segura
5. **Audit Everything**: Tudo é auditável

---

## 🔑 **Sistema de Criptografia**

### **Criptografia End-to-End**
```rust
// crypto/encryption.rs
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use sha2::{Sha256, Digest};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub struct EncryptionService {
    aes_key: [u8; 32],
    rsa_private_key: RsaPrivateKey,
    rsa_public_key: RsaPublicKey,
    argon2_config: Argon2,
}

impl EncryptionService {
    pub fn new() -> Self {
        let aes_key = Self::generate_aes_key();
        let (private_key, public_key) = Self::generate_rsa_keypair();
        
        Self {
            aes_key,
            rsa_private_key: private_key,
            rsa_public_key: public_key,
            argon2_config: Argon2::default(),
        }
    }
    
    pub fn encrypt_vote(&self, vote: &Vote) -> Result<EncryptedVote, CryptoError> {
        // 1. Serializar o voto
        let vote_bytes = bincode::serialize(vote)?;
        
        // 2. Criptografar com AES-256-GCM
        let cipher = Aes256Gcm::new(Key::from_slice(&self.aes_key));
        let nonce = Self::generate_nonce();
        let ciphertext = cipher.encrypt(&nonce, vote_bytes.as_ref())?;
        
        // 3. Assinar com RSA-4096
        let signature = self.rsa_private_key.sign(
            PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA256)),
            &ciphertext
        )?;
        
        // 4. Gerar hash SHA-256
        let mut hasher = Sha256::new();
        hasher.update(&ciphertext);
        let hash = hasher.finalize();
        
        Ok(EncryptedVote {
            ciphertext,
            nonce: nonce.to_vec(),
            signature,
            hash: hash.to_vec(),
            timestamp: Utc::now(),
        })
    }
    
    pub fn decrypt_vote(&self, encrypted_vote: &EncryptedVote) -> Result<Vote, CryptoError> {
        // 1. Verificar assinatura
        self.verify_signature(&encrypted_vote.ciphertext, &encrypted_vote.signature)?;
        
        // 2. Verificar hash
        let mut hasher = Sha256::new();
        hasher.update(&encrypted_vote.ciphertext);
        let computed_hash = hasher.finalize();
        
        if computed_hash.as_slice() != encrypted_vote.hash {
            return Err(CryptoError::HashMismatch);
        }
        
        // 3. Descriptografar
        let cipher = Aes256Gcm::new(Key::from_slice(&self.aes_key));
        let nonce = Nonce::from_slice(&encrypted_vote.nonce);
        let plaintext = cipher.decrypt(nonce, encrypted_vote.ciphertext.as_ref())?;
        
        // 4. Deserializar
        let vote: Vote = bincode::deserialize(&plaintext)?;
        
        Ok(vote)
    }
}
```

### **Zero-Knowledge Proofs**
```rust
// crypto/zk_proofs.rs
use ark_ec::PairingEngine;
use ark_ff::PrimeField;
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_std::rand::Rng;

pub struct ZKProofService {
    proving_key: ProvingKey,
    verifying_key: VerifyingKey,
}

impl ZKProofService {
    pub fn generate_vote_proof(
        &self,
        vote: &Vote,
        voter_identity: &VoterIdentity,
        election_id: &str,
    ) -> Result<ZKProof, ZKError> {
        // 1. Criar witness para o circuito
        let witness = VoteWitness {
            vote: vote.clone(),
            voter_identity: voter_identity.clone(),
            election_id: election_id.to_string(),
            nullifier: self.generate_nullifier(voter_identity, election_id),
        };
        
        // 2. Gerar prova ZK-SNARK
        let proof = Groth16::prove(&self.proving_key, witness)?;
        
        // 3. Criar prova verificável
        Ok(ZKProof {
            proof,
            public_inputs: self.extract_public_inputs(&witness),
            nullifier: witness.nullifier,
        })
    }
    
    pub fn verify_vote_proof(
        &self,
        proof: &ZKProof,
        election_id: &str,
    ) -> Result<bool, ZKError> {
        // Verificar a prova ZK-SNARK
        Groth16::verify(&self.verifying_key, &proof.public_inputs, &proof.proof)
    }
    
    fn generate_nullifier(&self, voter_identity: &VoterIdentity, election_id: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(voter_identity.cpf.as_bytes());
        hasher.update(election_id.as_bytes());
        hasher.update(voter_identity.biometric_hash.as_bytes());
        hasher.finalize().into()
    }
}
```

---

## 🔐 **Autenticação Multi-Fator**

### **Sistema de Autenticação Biométrica**
```rust
// auth/biometric.rs
use ring::digest;
use std::collections::HashMap;

pub struct BiometricAuth {
    biometric_database: HashMap<String, BiometricTemplate>,
    liveness_detector: LivenessDetector,
    quality_assessor: QualityAssessor,
}

impl BiometricAuth {
    pub async fn authenticate(
        &self,
        biometric_data: &BiometricData,
        cpf: &str,
    ) -> Result<AuthResult, AuthError> {
        // 1. Verificar qualidade da captura
        let quality_score = self.quality_assessor.assess(biometric_data)?;
        if quality_score < 0.8 {
            return Err(AuthError::LowQuality);
        }
        
        // 2. Detectar liveness (anti-spoofing)
        let is_live = self.liveness_detector.detect(biometric_data).await?;
        if !is_live {
            return Err(AuthError::SpoofingDetected);
        }
        
        // 3. Extrair template biométrico
        let template = self.extract_template(biometric_data)?;
        
        // 4. Comparar com banco de dados
        if let Some(stored_template) = self.biometric_database.get(cpf) {
            let similarity = self.compare_templates(&template, stored_template)?;
            
            if similarity > 0.85 {
                Ok(AuthResult {
                    authenticated: true,
                    confidence: similarity,
                    method: AuthMethod::Biometric,
                    timestamp: Utc::now(),
                })
            } else {
                Err(AuthError::AuthenticationFailed)
            }
        } else {
            Err(AuthError::UserNotFound)
        }
    }
    
    fn extract_template(&self, biometric_data: &BiometricData) -> Result<BiometricTemplate, AuthError> {
        match biometric_data {
            BiometricData::Fingerprint(data) => {
                // Extrair minutiae do fingerprint
                let minutiae = self.extract_minutiae(data)?;
                Ok(BiometricTemplate::Fingerprint(minutiae))
            }
            BiometricData::Facial(data) => {
                // Extrair landmarks faciais
                let landmarks = self.extract_facial_landmarks(data)?;
                Ok(BiometricTemplate::Facial(landmarks))
            }
        }
    }
}
```

### **Autenticação por Certificado Digital**
```rust
// auth/certificate.rs
use x509_parser::prelude::*;
use ring::signature::{RsaPublicKey, RsaSignature, RsaPadding, SHA256};
use std::time::SystemTime;

pub struct CertificateAuth {
    root_ca_cert: X509Certificate,
    intermediate_ca_certs: Vec<X509Certificate>,
    crl_cache: HashMap<String, CertificateRevocationList>,
}

impl CertificateAuth {
    pub async fn verify_certificate(
        &self,
        certificate: &[u8],
        cpf: &str,
    ) -> Result<CertVerificationResult, CertError> {
        // 1. Parse do certificado
        let (_, cert) = X509Certificate::from_der(certificate)?;
        
        // 2. Verificar validade temporal
        let now = SystemTime::now();
        if !self.is_certificate_valid(&cert, now)? {
            return Err(CertError::Expired);
        }
        
        // 3. Verificar cadeia de certificação
        let chain_valid = self.verify_certificate_chain(&cert).await?;
        if !chain_valid {
            return Err(CertError::InvalidChain);
        }
        
        // 4. Verificar revogação
        let is_revoked = self.check_certificate_revocation(&cert).await?;
        if is_revoked {
            return Err(CertError::Revoked);
        }
        
        // 5. Verificar CPF no certificado
        let cert_cpf = self.extract_cpf_from_certificate(&cert)?;
        if cert_cpf != cpf {
            return Err(CertError::CpfMismatch);
        }
        
        // 6. Verificar assinatura
        let signature_valid = self.verify_certificate_signature(&cert)?;
        if !signature_valid {
            return Err(CertError::InvalidSignature);
        }
        
        Ok(CertVerificationResult {
            valid: true,
            issuer: cert.issuer().to_string(),
            subject: cert.subject().to_string(),
            validity_period: self.extract_validity_period(&cert)?,
        })
    }
}
```

---

## 🛡️ **Segurança de Rede**

### **Firewall e DDoS Protection**
```yaml
# security/network-policies.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: fortis-network-policy
  namespace: fortis
spec:
  podSelector:
    matchLabels:
      app: fortis-backend
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: istio-system
    - podSelector:
        matchLabels:
          app: fortis-frontend
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: postgresql
    ports:
    - protocol: TCP
      port: 5432
  - to:
    - podSelector:
        matchLabels:
          app: redis
    ports:
    - protocol: TCP
      port: 6379
```

### **Rate Limiting e Throttling**
```rust
// security/rate_limiting.rs
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub struct RateLimiter {
    limits: HashMap<String, RateLimit>,
    counters: RwLock<HashMap<String, RateCounter>>,
}

#[derive(Clone)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_size: u32,
}

pub struct RateCounter {
    pub minute_requests: Vec<Instant>,
    pub hour_requests: Vec<Instant>,
    pub burst_tokens: u32,
    pub last_refill: Instant,
}

impl RateLimiter {
    pub async fn check_rate_limit(
        &self,
        client_id: &str,
        limit: &RateLimit,
    ) -> Result<(), RateLimitError> {
        let mut counters = self.counters.write().await;
        let counter = counters.entry(client_id.to_string()).or_insert_with(|| {
            RateCounter {
                minute_requests: Vec::new(),
                hour_requests: Vec::new(),
                burst_tokens: limit.burst_size,
                last_refill: Instant::now(),
            }
        });
        
        let now = Instant::now();
        
        // Limpar requisições antigas
        counter.minute_requests.retain(|&time| now.duration_since(time) < Duration::from_secs(60));
        counter.hour_requests.retain(|&time| now.duration_since(time) < Duration::from_secs(3600));
        
        // Verificar limite por minuto
        if counter.minute_requests.len() >= limit.requests_per_minute as usize {
            return Err(RateLimitError::MinuteLimitExceeded);
        }
        
        // Verificar limite por hora
        if counter.hour_requests.len() >= limit.requests_per_hour as usize {
            return Err(RateLimitError::HourLimitExceeded);
        }
        
        // Verificar burst limit
        if counter.burst_tokens == 0 {
            return Err(RateLimitError::BurstLimitExceeded);
        }
        
        // Adicionar requisição
        counter.minute_requests.push(now);
        counter.hour_requests.push(now);
        counter.burst_tokens -= 1;
        
        // Refill burst tokens
        let time_since_refill = now.duration_since(counter.last_refill);
        if time_since_refill >= Duration::from_secs(1) {
            counter.burst_tokens = limit.burst_size;
            counter.last_refill = now;
        }
        
        Ok(())
    }
}
```

---

## 🔍 **Sistema de Auditoria**

### **Logging Estruturado**
```rust
// audit/logging.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: HashMap<String, serde_json::Value>,
    pub hash: String,
    pub previous_hash: Option<String>,
}

pub enum EventType {
    Authentication,
    VoteCast,
    VoteVerification,
    SystemAccess,
    ConfigurationChange,
    SecurityIncident,
}

impl AuditLog {
    pub fn new(event_type: EventType, details: HashMap<String, serde_json::Value>) -> Self {
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        Self {
            id,
            timestamp,
            event_type,
            user_id: None,
            session_id: None,
            ip_address: None,
            user_agent: None,
            details,
            hash: String::new(),
            previous_hash: None,
        }
    }
    
    pub fn calculate_hash(&mut self, previous_hash: Option<&str>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.id.as_bytes());
        hasher.update(self.timestamp.to_rfc3339().as_bytes());
        hasher.update(format!("{:?}", self.event_type).as_bytes());
        
        if let Some(user_id) = &self.user_id {
            hasher.update(user_id.as_bytes());
        }
        
        if let Some(prev_hash) = previous_hash {
            hasher.update(prev_hash.as_bytes());
        }
        
        let hash = hasher.finalize();
        let hash_string = format!("{:x}", hash);
        self.hash = hash_string.clone();
        self.previous_hash = previous_hash.map(|s| s.to_string());
        
        hash_string
    }
}
```

### **Integridade dos Logs**
```rust
// audit/integrity.rs
use std::collections::VecDeque;

pub struct AuditChain {
    logs: VecDeque<AuditLog>,
    merkle_tree: MerkleTree,
}

impl AuditChain {
    pub fn add_log(&mut self, mut log: AuditLog) -> Result<(), AuditError> {
        // Calcular hash do log anterior
        let previous_hash = self.logs.back().map(|l| l.hash.as_str());
        log.calculate_hash(previous_hash);
        
        // Verificar integridade da cadeia
        if let Some(last_log) = self.logs.back() {
            if log.previous_hash.as_ref() != Some(&last_log.hash) {
                return Err(AuditError::ChainIntegrityViolation);
            }
        }
        
        // Adicionar à cadeia
        self.logs.push_back(log);
        
        // Atualizar árvore de Merkle
        self.update_merkle_tree()?;
        
        Ok(())
    }
    
    pub fn verify_integrity(&self) -> Result<bool, AuditError> {
        let mut previous_hash: Option<&str> = None;
        
        for log in &self.logs {
            // Verificar hash do log
            let expected_hash = self.calculate_expected_hash(log, previous_hash);
            if log.hash != expected_hash {
                return Ok(false);
            }
            
            previous_hash = Some(&log.hash);
        }
        
        // Verificar árvore de Merkle
        self.merkle_tree.verify()
    }
}
```

---

## 🚨 **Detecção de Intrusão**

### **Sistema de Anomalias**
```rust
// security/anomaly_detection.rs
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

pub struct AnomalyDetector {
    patterns: HashMap<String, BehaviorPattern>,
    thresholds: AnomalyThresholds,
}

pub struct BehaviorPattern {
    pub user_id: String,
    pub normal_voting_hours: Vec<(u8, u8)>, // (start_hour, end_hour)
    pub typical_location: Option<String>,
    pub device_fingerprint: Option<String>,
    pub voting_frequency: Duration,
}

pub struct AnomalyThresholds {
    pub unusual_time_weight: f64,
    pub location_deviation_weight: f64,
    pub device_change_weight: f64,
    pub frequency_deviation_weight: f64,
    pub total_threshold: f64,
}

impl AnomalyDetector {
    pub fn detect_anomalies(
        &self,
        event: &SecurityEvent,
    ) -> Result<Vec<Anomaly>, AnomalyError> {
        let mut anomalies = Vec::new();
        
        if let Some(pattern) = self.patterns.get(&event.user_id) {
            // Verificar horário incomum
            if self.is_unusual_time(event, pattern) {
                anomalies.push(Anomaly {
                    type_: AnomalyType::UnusualTime,
                    severity: AnomalySeverity::Medium,
                    description: "Voto em horário incomum".to_string(),
                    confidence: self.calculate_time_anomaly_score(event, pattern),
                });
            }
            
            // Verificar localização incomum
            if self.is_unusual_location(event, pattern) {
                anomalies.push(Anomaly {
                    type_: AnomalyType::UnusualLocation,
                    severity: AnomalySeverity::High,
                    description: "Voto de localização incomum".to_string(),
                    confidence: self.calculate_location_anomaly_score(event, pattern),
                });
            }
            
            // Verificar mudança de dispositivo
            if self.is_device_change(event, pattern) {
                anomalies.push(Anomaly {
                    type_: AnomalyType::DeviceChange,
                    severity: AnomalySeverity::Medium,
                    description: "Mudança de dispositivo".to_string(),
                    confidence: self.calculate_device_anomaly_score(event, pattern),
                });
            }
            
            // Verificar frequência incomum
            if self.is_unusual_frequency(event, pattern) {
                anomalies.push(Anomaly {
                    type_: AnomalyType::UnusualFrequency,
                    severity: AnomalySeverity::High,
                    description: "Frequência de votação incomum".to_string(),
                    confidence: self.calculate_frequency_anomaly_score(event, pattern),
                });
            }
        }
        
        Ok(anomalies)
    }
}
```

---

## 🔒 **Hardware Security Module (HSM)**

### **Integração com HSM**
```rust
// security/hsm.rs
use pkcs11::*;
use std::sync::Arc;

pub struct HSMService {
    context: Arc<Ctx>,
    session: Session,
}

impl HSMService {
    pub fn new() -> Result<Self, HSMError> {
        let context = Arc::new(Ctx::new_and_initialize(CkInitializeArgs::new())?);
        let session = context.open_session(
            SlotId::new(0),
            CKF_SERIAL_SESSION | CKF_RW_SESSION,
            None,
            None,
        )?;
        
        Ok(Self { context, session })
    }
    
    pub fn generate_key_pair(&self) -> Result<(PublicKey, PrivateKey), HSMError> {
        let mechanism = Mechanism::RsaPkcsKeyPairGen;
        let public_key_template = vec![
            Attribute::Token(true),
            Attribute::Private(false),
            Attribute::ModulusBits(4096),
            Attribute::PublicExponent(vec![1, 0, 1]),
        ];
        
        let private_key_template = vec![
            Attribute::Token(true),
            Attribute::Private(true),
            Attribute::Sensitive(true),
            Attribute::Extractable(false),
        ];
        
        let (public_key, private_key) = self.session.generate_key_pair(
            &mechanism,
            &public_key_template,
            &private_key_template,
        )?;
        
        Ok((public_key, private_key))
    }
    
    pub fn sign_with_hsm(&self, data: &[u8], private_key: PrivateKey) -> Result<Vec<u8>, HSMError> {
        let mechanism = Mechanism::RsaPkcs;
        self.session.sign_init(&mechanism, private_key)?;
        self.session.sign(data)
    }
}
```

---

## 📊 **Métricas de Segurança**

### **KPIs de Segurança**
```rust
// security/metrics.rs
use prometheus::{Counter, Histogram, Gauge, Registry};

lazy_static! {
    static ref AUTHENTICATION_ATTEMPTS: Counter = Counter::new(
        "fortis_auth_attempts_total",
        "Total number of authentication attempts"
    ).unwrap();
    
    static ref AUTHENTICATION_FAILURES: Counter = Counter::new(
        "fortis_auth_failures_total",
        "Total number of authentication failures"
    ).unwrap();
    
    static ref FRAUD_DETECTIONS: Counter = Counter::new(
        "fortis_fraud_detections_total",
        "Total number of fraud detections"
    ).unwrap();
    
    static ref SECURITY_INCIDENTS: Counter = Counter::new(
        "fortis_security_incidents_total",
        "Total number of security incidents"
    ).unwrap();
    
    static ref AUTHENTICATION_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "fortis_auth_duration_seconds",
            "Time spent on authentication"
        ).buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0])
    ).unwrap();
    
    static ref ACTIVE_SESSIONS: Gauge = Gauge::new(
        "fortis_active_sessions",
        "Number of active sessions"
    ).unwrap();
}
```

---

## 🎯 **Próximos Passos**

### **Fase 1: Segurança Base (2 meses)**
- [ ] Implementar criptografia end-to-end
- [ ] Sistema de autenticação multi-fator
- [ ] Logging e auditoria básica
- [ ] Testes de penetração

### **Fase 2: Segurança Avançada (2 meses)**
- [ ] Zero-Knowledge Proofs
- [ ] Detecção de anomalias
- [ ] Integração com HSM
- [ ] Monitoramento de segurança

### **Fase 3: Produção (2 meses)**
- [ ] Deploy em produção
- [ ] Monitoramento contínuo
- [ ] Resposta a incidentes
- [ ] Auditoria de segurança

---

*Documentação de Segurança FORTIS - Desenvolvida pelo Security Specialist Agent*
