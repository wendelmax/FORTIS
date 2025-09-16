//! Sistema de Monitoramento para FORTIS 3.0
//! 
//! Módulo de monitoramento completo para o sistema sem blockchain,
//! incluindo métricas, alertas e verificação de saúde.

pub mod metrics;
pub mod health_checks;
pub mod alerts;
pub mod dashboards;

pub use metrics::*;
pub use health_checks::*;
pub use alerts::*;
pub use dashboards::*;
