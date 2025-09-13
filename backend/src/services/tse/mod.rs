//! Módulo de Integração TSE (Tribunal Superior Eleitoral)
//! 
//! Este módulo implementa a integração completa com o TSE e Gov.br
//! para autenticação digital e validação de eleitores.

pub mod gov_br;
pub mod voter_validation;
pub mod digital_certificate;
pub mod election_sync;

pub use gov_br::GovBrService;
pub use voter_validation::VoterValidationService;
pub use digital_certificate::DigitalCertificateService;
pub use election_sync::ElectionSyncService;
