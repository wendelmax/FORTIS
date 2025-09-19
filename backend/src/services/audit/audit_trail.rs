//! Serviço de Trilha de Auditoria
//! 
//! Implementa trilha de auditoria completa para rastreamento
//! e verificação de todas as operações do sistema.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::transparency::election_logs::{AuditEvent, AuditEventType};

/// Serviço de trilha de auditoria
pub struct AuditTrailService {
    events: Vec<AuditEvent>,
    verification_results: HashMap<String, IntegrityVerification>,
}

/// Filtros para busca de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventFilter {
    pub event_types: Option<Vec<AuditEventType>>,
    pub actors: Option<Vec<String>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub election_id: Option<String>,
    pub node_id: Option<String>,
    pub voter_id: Option<String>,
    pub has_errors: Option<bool>,
}

/// Resultado da busca de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventSearchResult {
    pub events: Vec<AuditEvent>,
    pub total_count: usize,
    pub filtered_count: usize,
    pub search_timestamp: DateTime<Utc>,
    pub filters_applied: AuditEventFilter,
}

/// Resumo da trilha de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailSummary {
    pub total_events: usize,
    pub events_by_type: HashMap<String, usize>,
    pub events_by_actor: HashMap<String, usize>,
    pub events_today: usize,
    pub events_this_week: usize,
    pub events_this_month: usize,
    pub error_events: usize,
    pub security_alerts: usize,
    pub verification_rate: f64,
    pub last_updated: DateTime<Utc>,
}

/// Relatório de auditoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub report_id: String,
    pub title: String,
    pub description: String,
    pub generated_at: DateTime<Utc>,
    pub generated_by: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub summary: AuditTrailSummary,
    pub events: Vec<AuditEvent>,
    pub recommendations: Vec<String>,
    pub compliance_status: ComplianceStatus,
}

/// Status de conformidade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    RequiresAttention,
    UnderReview,
}

impl AuditTrailService {
    /// Cria nova instância do serviço
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            verification_results: HashMap::new(),
        }
    }

    /// Adiciona evento à trilha de auditoria
    pub fn add_event(&mut self, event: AuditEvent) {
        self.events.push(event);
    }

    /// Busca eventos com filtros
    pub fn search_events(&self, filter: &AuditEventFilter) -> Result<AuditEventSearchResult> {
        let mut filtered_events = self.events.clone();

        // Filtrar por tipo de evento
        if let Some(event_types) = &filter.event_types {
            filtered_events.retain(|event| event_types.contains(&event.event_type));
        }

        // Filtrar por atores
        if let Some(actors) = &filter.actors {
            filtered_events.retain(|event| actors.contains(&event.actor));
        }

        // Filtrar por data
        if let Some(start_date) = filter.start_date {
            filtered_events.retain(|event| event.timestamp >= start_date);
        }
        if let Some(end_date) = filter.end_date {
            filtered_events.retain(|event| event.timestamp <= end_date);
        }

        // Filtrar por eleição
        if let Some(election_id) = &filter.election_id {
            filtered_events.retain(|event| 
                event.data.election_id.as_ref() == Some(election_id)
            );
        }

        // Filtrar por nó
        if let Some(node_id) = &filter.node_id {
            filtered_events.retain(|event| 
                event.data.node_id.as_ref() == Some(node_id)
            );
        }

        // Filtrar por eleitor
        if let Some(voter_id) = &filter.voter_id {
            filtered_events.retain(|event| 
                event.data.voter_id.as_ref() == Some(voter_id)
            );
        }

        // Filtrar por erros
        if let Some(has_errors) = filter.has_errors {
            filtered_events.retain(|event| {
                let has_error = event.data.error_code.is_some() || event.data.error_message.is_some();
                if has_errors { has_error } else { !has_error }
            });
        }

        // Ordenar por timestamp (mais recente primeiro)
        filtered_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        let filtered_count = filtered_events.len();
        Ok(AuditEventSearchResult {
            events: filtered_events,
            total_count: self.events.len(),
            filtered_count,
            search_timestamp: Utc::now(),
            filters_applied: filter.clone(),
        })
    }

    /// Obtém resumo da trilha de auditoria
    pub fn get_audit_trail_summary(&self) -> AuditTrailSummary {
        let now = Utc::now();
        let today = now.date_naive();
        let week_ago = now - chrono::Duration::days(7);
        let month_ago = now - chrono::Duration::days(30);

        let mut events_by_type = HashMap::new();
        let mut events_by_actor = HashMap::new();
        let mut events_today = 0;
        let mut events_this_week = 0;
        let mut events_this_month = 0;
        let mut error_events = 0;
        let mut security_alerts = 0;

        for event in &self.events {
            // Contar por tipo
            let type_key = format!("{:?}", event.event_type);
            *events_by_type.entry(type_key).or_insert(0) += 1;

            // Contar por ator
            *events_by_actor.entry(event.actor.clone()).or_insert(0) += 1;

            // Contar por período
            if event.timestamp.date_naive() == today {
                events_today += 1;
            }
            if event.timestamp >= week_ago {
                events_this_week += 1;
            }
            if event.timestamp >= month_ago {
                events_this_month += 1;
            }

            // Contar erros
            if event.data.error_code.is_some() || event.data.error_message.is_some() {
                error_events += 1;
            }

            // Contar alertas de segurança
            if matches!(event.event_type, AuditEventType::SecurityAlert) {
                security_alerts += 1;
            }
        }

        // Calcular taxa de verificação
        let verified_events = self.verification_results.values()
            .filter(|v| v.is_valid)
            .count();
        let total_verified = self.verification_results.len();
        let verification_rate = if total_verified > 0 {
            verified_events as f64 / total_verified as f64
        } else {
            0.0
        };

        AuditTrailSummary {
            total_events: self.events.len(),
            events_by_type,
            events_by_actor,
            events_today,
            events_this_week,
            events_this_month,
            error_events,
            security_alerts,
            verification_rate,
            last_updated: now,
        }
    }

    /// Gera relatório de auditoria
    pub fn generate_audit_report(
        &self,
        title: String,
        description: String,
        generated_by: String,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<AuditReport> {
        // Filtrar eventos do período
        let period_events: Vec<AuditEvent> = self.events
            .iter()
            .filter(|event| event.timestamp >= period_start && event.timestamp <= period_end)
            .cloned()
            .collect();

        // Gerar resumo para o período
        let mut summary = self.get_audit_trail_summary();
        summary.total_events = period_events.len();

        // Gerar recomendações
        let recommendations = self.generate_recommendations(&period_events);

        // Determinar status de conformidade
        let compliance_status = self.determine_compliance_status(&period_events);

        Ok(AuditReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            generated_at: Utc::now(),
            generated_by,
            period_start,
            period_end,
            summary,
            events: period_events,
            recommendations,
            compliance_status,
        })
    }

    /// Gera recomendações baseadas nos eventos
    fn generate_recommendations(&self, events: &[AuditEvent]) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Analisar taxa de erros
        let error_count = events.iter()
            .filter(|e| e.data.error_code.is_some() || e.data.error_message.is_some())
            .count();
        let error_rate = if !events.is_empty() {
            error_count as f64 / events.len() as f64
        } else {
            0.0
        };

        if error_rate > 0.05 {
            recommendations.push("Taxa de erro elevada detectada. Revisar logs de sistema e implementar melhorias.".to_string());
        }

        // Analisar alertas de segurança
        let security_alerts = events.iter()
            .filter(|e| matches!(e.event_type, AuditEventType::SecurityAlert))
            .count();

        if security_alerts > 0 {
            recommendations.push("Alertas de segurança detectados. Revisar configurações de segurança.".to_string());
        }

        // Analisar verificação de votos
        let vote_events = events.iter()
            .filter(|e| matches!(e.event_type, AuditEventType::VoteCast))
            .count();
        let verified_votes = events.iter()
            .filter(|e| matches!(e.event_type, AuditEventType::VoteVerified))
            .count();

        if vote_events > 0 && (verified_votes as f64 / vote_events as f64) < 0.95 {
            recommendations.push("Taxa de verificação de votos baixa. Revisar processo de verificação.".to_string());
        }

        // Analisar atividade de nós
        let node_events = events.iter()
            .filter(|e| e.data.node_id.is_some())
            .count();
        let unique_nodes = events.iter()
            .filter_map(|e| e.data.node_id.as_ref())
            .collect::<std::collections::HashSet<_>>()
            .len();

        if node_events > 0 && unique_nodes < 5 {
            recommendations.push("Poucos nós ativos detectados. Verificar conectividade da rede.".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Sistema operando dentro dos parâmetros normais.".to_string());
        }

        recommendations
    }

    /// Determina status de conformidade
    fn determine_compliance_status(&self, events: &[AuditEvent]) -> ComplianceStatus {
        let error_count = events.iter()
            .filter(|e| e.data.error_code.is_some() || e.data.error_message.is_some())
            .count();
        let security_alerts = events.iter()
            .filter(|e| matches!(e.event_type, AuditEventType::SecurityAlert))
            .count();

        if security_alerts > 0 {
            ComplianceStatus::NonCompliant
        } else if error_count > events.len() / 10 {
            ComplianceStatus::RequiresAttention
        } else {
            ComplianceStatus::Compliant
        }
    }

    /// Obtém eventos por eleição
    pub fn get_election_events(&self, election_id: &str) -> Vec<AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.data.election_id.as_ref() == Some(&election_id.to_string()))
            .cloned()
            .collect()
    }

    /// Obtém eventos por nó
    pub fn get_node_events(&self, node_id: &str) -> Vec<AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.data.node_id.as_ref() == Some(&node_id.to_string()))
            .cloned()
            .collect()
    }

    /// Obtém eventos por eleitor
    pub fn get_voter_events(&self, voter_id: &str) -> Vec<AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.data.voter_id.as_ref() == Some(&voter_id.to_string()))
            .cloned()
            .collect()
    }

    /// Obtém eventos de erro
    pub fn get_error_events(&self) -> Vec<AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.data.error_code.is_some() || event.data.error_message.is_some())
            .cloned()
            .collect()
    }

    /// Obtém alertas de segurança
    pub fn get_security_alerts(&self) -> Vec<AuditEvent> {
        self.events
            .iter()
            .filter(|event| matches!(event.event_type, AuditEventType::SecurityAlert))
            .cloned()
            .collect()
    }

    /// Adiciona resultado de verificação
    pub fn add_verification_result(&mut self, event_id: String, verification: IntegrityVerification) {
        self.verification_results.insert(event_id, verification);
    }

    /// Obtém resultado de verificação
    pub fn get_verification_result(&self, event_id: &str) -> Option<&IntegrityVerification> {
        self.verification_results.get(event_id)
    }

    /// Obtém todos os resultados de verificação
    pub fn get_all_verification_results(&self) -> &HashMap<String, IntegrityVerification> {
        &self.verification_results
    }

    /// Limpa eventos antigos (manter apenas últimos N dias)
    pub fn cleanup_old_events(&mut self, days_to_keep: i64) {
        let cutoff_date = Utc::now() - chrono::Duration::days(days_to_keep);
        self.events.retain(|event| event.timestamp >= cutoff_date);
    }

    /// Exporta trilha de auditoria
    pub fn export_audit_trail(&self, format: ExportFormat) -> Result<Vec<u8>> {
        match format {
            ExportFormat::Json => {
                let json = serde_json::to_string_pretty(&self.events)?;
                Ok(json.into_bytes())
            },
            ExportFormat::Csv => {
                // TODO: Implementar exportação CSV
                Ok(b"CSV export not implemented yet".to_vec())
            },
            ExportFormat::Xml => {
                // TODO: Implementar exportação XML
                Ok(b"XML export not implemented yet".to_vec())
            },
        }
    }
}

/// Formato de exportação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Xml,
}
