//! Módulo de serviços para urnas eletrônicas
//! 
//! Este módulo contém todos os serviços relacionados ao funcionamento
//! das urnas eletrônicas do sistema FORTIS, organizados por funcionalidade.

pub mod auth;
pub mod blockchain;
pub mod monitoring;
pub mod security;
pub mod sync;
pub mod service;

// Re-exportar os serviços principais para facilitar o uso
pub use auth::UrnaAuthService;
pub use blockchain::UrnaBlockchainService;
pub use monitoring::UrnaMonitoringService;
pub use security::UrnaSecurityService;
pub use sync::UrnaSyncService;
pub use service::UrnaService;
