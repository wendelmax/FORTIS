use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateElectionRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ElectionResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub candidates: Option<Vec<Candidate>>,
    pub stats: Option<ElectionStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Candidate {
    pub id: Uuid,
    pub name: String,
    pub party: String,
    pub number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ElectionStats {
    pub total_votes: i64,
    pub total_voters: i64,
    pub participation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateCandidateRequest {
    pub name: String,
    pub party: String,
    pub number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Voter {
    pub id: Uuid,
    pub cpf: String,
    pub name: String,
    pub name_encrypted: String,
    pub birth_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub is_eligible: bool,
    pub certificate_hash: Option<String>,
    pub biometric_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoteRequest {
    pub election_id: Uuid,
    pub candidate_id: Uuid,
    pub proof: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthRequest {
    pub cpf: String,
    pub password: String,
    pub biometric_data: Option<BiometricData>,
    pub certificate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub cpf: String,
    pub name: String,
    pub roles: Vec<String>,
    pub election_eligible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BiometricData {
    pub fingerprint: String,
    pub fingerprint_hash: String,
    pub face_id: String,
    pub biometric_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            message: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

// ===== MODELOS PARA URNAS ELETRÔNICAS =====

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Urna {
    pub id: Uuid,
    pub serial_number: String,
    pub model: String,
    pub location: UrnaLocation,
    pub status: UrnaStatus,
    pub last_sync: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaLocation {
    pub state: String,
    pub city: String,
    pub zone: String,
    pub section: String,
    pub address: String,
    pub coordinates: Option<Coordinates>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum UrnaStatus {
    Active,
    Inactive,
    Maintenance,
    Offline,
    Error,
    Syncing,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaSync {
    pub id: Uuid,
    pub urna_id: Uuid,
    pub sync_type: SyncType,
    pub status: SyncStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub votes_synced: i32,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum SyncType {
    Full,
    Incremental,
    Emergency,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaVote {
    pub id: Uuid,
    pub urna_id: Uuid,
    pub election_id: Uuid,
    pub voter_id: Uuid,
    pub candidate_id: Uuid,
    pub vote_data: EncryptedVoteData,
    pub biometric_hash: String,
    pub timestamp: DateTime<Utc>,
    pub sync_status: VoteSyncStatus,
    pub blockchain_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EncryptedVoteData {
    pub encrypted_content: String,
    pub encryption_key_id: String,
    pub signature: String,
    pub zk_proof: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum VoteSyncStatus {
    Pending,
    Synced,
    Failed,
    Confirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaAuthentication {
    pub urna_id: Uuid,
    pub voter_id: Uuid,
    pub biometric_data: BiometricData,
    pub certificate_data: Option<CertificateData>,
    pub auth_timestamp: DateTime<Utc>,
    pub auth_method: AuthMethod,
    pub auth_result: AuthResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CertificateData {
    pub certificate_hash: String,
    pub issuer: String,
    pub valid_until: DateTime<Utc>,
    pub serial_number: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum AuthMethod {
    BiometricOnly,
    BiometricAndCertificate,
    CertificateOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum AuthResult {
    Success,
    BiometricFailure,
    CertificateFailure,
    VoterNotEligible,
    AlreadyVoted,
    Timeout,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaAuditLog {
    pub id: Uuid,
    pub urna_id: Uuid,
    pub event_type: AuditEventType,
    pub event_data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub integrity_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum AuditEventType {
    VoterAuthentication,
    VoteCast,
    VoteSync,
    SystemStartup,
    SystemShutdown,
    Error,
    Maintenance,
    SecurityAlert,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaHealthCheck {
    pub urna_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub status: UrnaStatus,
    pub battery_level: Option<f32>,
    pub storage_usage: Option<f32>,
    pub network_connectivity: bool,
    pub last_sync: Option<DateTime<Utc>>,
    pub errors: Vec<String>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PerformanceMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_latency: Option<u64>,
    pub response_time: u64,
}

// ===== REQUESTS E RESPONSES PARA URNAS =====

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaVoteRequest {
    pub urna_id: Uuid,
    pub election_id: Uuid,
    pub candidate_id: Uuid,
    pub biometric_data: BiometricData,
    pub certificate_data: Option<CertificateData>,
    pub vote_proof: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaVoteResponse {
    pub vote_id: Uuid,
    pub success: bool,
    pub message: String,
    pub receipt: Option<VoteReceipt>,
    pub sync_status: VoteSyncStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoteReceipt {
    pub vote_id: Uuid,
    pub election_id: Uuid,
    pub candidate_number: i32,
    pub candidate_name: String,
    pub timestamp: DateTime<Utc>,
    pub qr_code: String,
    pub blockchain_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaSyncRequest {
    pub urna_id: Uuid,
    pub sync_type: SyncType,
    pub force_full_sync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaSyncResponse {
    pub sync_id: Uuid,
    pub status: SyncStatus,
    pub votes_synced: i32,
    pub errors: Vec<String>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaStatusRequest {
    pub urna_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UrnaStatusResponse {
    pub urna: Urna,
    pub health: UrnaHealthCheck,
    pub pending_votes: i32,
    pub last_activity: Option<DateTime<Utc>>,
}