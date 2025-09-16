//! Módulo de Auditoria com Logs Transparentes
//! 
//! Implementa sistema de auditoria usando logs transparentes (similar a CT logs),
//! seguindo os princípios do Prof. Marcos Simplicio de usar ferramentas
//! apropriadas para cada problema.

pub mod transparent_logs;
pub mod audit_service;
pub mod verification;

pub use transparent_logs::*;
pub use audit_service::*;
