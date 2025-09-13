//! Módulo de Auditoria Imutável
//! 
//! Este módulo implementa o sistema de auditoria completo do FORTIS,
//! garantindo transparência e imutabilidade através de blockchain.

pub mod blockchain_audit;
pub mod event_logger;
pub mod audit_trail;
pub mod verification;
pub mod reporting;

pub use blockchain_audit::BlockchainAuditService;
pub use event_logger::EventLogger;
pub use audit_trail::AuditTrailService;
pub use verification::AuditVerificationService;
pub use reporting::AuditReportingService;
