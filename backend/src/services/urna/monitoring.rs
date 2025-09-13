//! Serviço de monitoramento para urnas eletrônicas

use crate::models::{
    Urna, UrnaHealthCheck, UrnaStatus, PerformanceMetrics, UrnaAuditLog, AuditEventType
};
use anyhow::{Result, anyhow};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde_json::json;

pub struct UrnaMonitoringService {
    pub health_checks: RwLock<HashMap<Uuid, UrnaHealthCheck>>,
    pub performance_metrics: RwLock<HashMap<Uuid, Vec<PerformanceMetrics>>>,
    pub alert_thresholds: AlertThresholds,
    pub monitoring_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub cpu_usage_max: f32,
    pub memory_usage_max: f32,
    pub disk_usage_max: f32,
    pub battery_level_min: f32,
    pub response_time_max: u64,
    pub network_latency_max: u64,
    pub sync_delay_max: Duration,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_usage_max: 80.0,
            memory_usage_max: 85.0,
            disk_usage_max: 90.0,
            battery_level_min: 20.0,
            response_time_max: 5000, // 5 segundos
            network_latency_max: 1000, // 1 segundo
            sync_delay_max: Duration::minutes(30),
        }
    }
}

impl UrnaMonitoringService {
    pub fn new() -> Self {
        Self {
            health_checks: RwLock::new(HashMap::new()),
            performance_metrics: RwLock::new(HashMap::new()),
            alert_thresholds: AlertThresholds::default(),
            monitoring_interval: Duration::minutes(5),
        }
    }

    pub async fn start_monitoring(&self, urna_id: Uuid) -> Result<()> {
        // Iniciar monitoramento contínuo da urna
        let service = self.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = service.perform_health_check(urna_id).await {
                    log::error!("Health check failed for urna {}: {}", urna_id, e);
                }
                
                if let Err(e) = service.collect_performance_metrics(urna_id).await {
                    log::error!("Performance metrics collection failed for urna {}: {}", urna_id, e);
                }
                
                if let Err(e) = service.check_alerts(urna_id).await {
                    log::error!("Alert check failed for urna {}: {}", urna_id, e);
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    service.monitoring_interval.num_seconds() as u64
                )).await;
            }
        });

        Ok(())
    }

    pub async fn perform_health_check(&self, urna_id: Uuid) -> Result<UrnaHealthCheck> {
        // Realizar verificação de saúde da urna
        let health_check = UrnaHealthCheck {
            urna_id,
            timestamp: Utc::now(),
            status: self.check_urna_status(urna_id).await?,
            battery_level: self.check_battery_level(urna_id).await?,
            storage_usage: self.check_storage_usage(urna_id).await?,
            network_connectivity: self.check_network_connectivity(urna_id).await?,
            last_sync: self.get_last_sync_time(urna_id).await?,
            errors: self.collect_errors(urna_id).await?,
            performance_metrics: self.get_current_performance_metrics(urna_id).await?,
        };

        // Armazenar resultado da verificação
        {
            let mut health_checks = self.health_checks.write().await;
            health_checks.insert(urna_id, health_check.clone());
        }

        Ok(health_check)
    }

    async fn check_urna_status(&self, urna_id: Uuid) -> Result<UrnaStatus> {
        // Verificar status da urna
        // Em implementação real, faria verificação real do hardware/software
        Ok(UrnaStatus::Active)
    }

    async fn check_battery_level(&self, urna_id: Uuid) -> Result<Option<f32>> {
        // Verificar nível da bateria
        // Em implementação real, faria leitura real do hardware
        Ok(Some(85.0))
    }

    async fn check_storage_usage(&self, urna_id: Uuid) -> Result<Option<f32>> {
        // Verificar uso do armazenamento
        // Em implementação real, faria verificação real do sistema de arquivos
        Ok(Some(45.0))
    }

    async fn check_network_connectivity(&self, urna_id: Uuid) -> Result<bool> {
        // Verificar conectividade de rede
        // Em implementação real, faria ping para servidores TSE
        Ok(true)
    }

    async fn get_last_sync_time(&self, urna_id: Uuid) -> Result<Option<DateTime<Utc>>> {
        // Obter timestamp da última sincronização
        // Em implementação real, faria consulta no banco de dados
        Ok(Some(Utc::now() - Duration::minutes(5)))
    }

    async fn collect_errors(&self, urna_id: Uuid) -> Result<Vec<String>> {
        // Coletar erros recentes da urna
        // Em implementação real, faria consulta nos logs
        Ok(Vec::new())
    }

    async fn get_current_performance_metrics(&self, urna_id: Uuid) -> Result<PerformanceMetrics> {
        // Obter métricas de performance atuais
        let metrics = PerformanceMetrics {
            cpu_usage: self.measure_cpu_usage(urna_id).await?,
            memory_usage: self.measure_memory_usage(urna_id).await?,
            disk_usage: self.measure_disk_usage(urna_id).await?,
            network_latency: self.measure_network_latency(urna_id).await?,
            response_time: self.measure_response_time(urna_id).await?,
        };

        // Armazenar métricas históricas
        {
            let mut performance_metrics = self.performance_metrics.write().await;
            performance_metrics.entry(urna_id).or_insert_with(Vec::new).push(metrics.clone());
            
            // Manter apenas as últimas 100 métricas
            if let Some(metrics_vec) = performance_metrics.get_mut(&urna_id) {
                if metrics_vec.len() > 100 {
                    metrics_vec.remove(0);
                }
            }
        }

        Ok(metrics)
    }

    async fn measure_cpu_usage(&self, urna_id: Uuid) -> Result<f32> {
        // Medir uso de CPU
        // Em implementação real, faria medição real do sistema
        Ok(25.0)
    }

    async fn measure_memory_usage(&self, urna_id: Uuid) -> Result<f32> {
        // Medir uso de memória
        // Em implementação real, faria medição real do sistema
        Ok(60.0)
    }

    async fn measure_disk_usage(&self, urna_id: Uuid) -> Result<f32> {
        // Medir uso de disco
        // Em implementação real, faria medição real do sistema
        Ok(45.0)
    }

    async fn measure_network_latency(&self, urna_id: Uuid) -> Result<Option<u64>> {
        // Medir latência de rede
        // Em implementação real, faria ping para servidores
        Ok(Some(50))
    }

    async fn measure_response_time(&self, urna_id: Uuid) -> Result<u64> {
        // Medir tempo de resposta
        // Em implementação real, faria medição real de APIs
        Ok(100)
    }

    pub async fn collect_performance_metrics(&self, urna_id: Uuid) -> Result<()> {
        // Coletar métricas de performance
        let metrics = self.get_current_performance_metrics(urna_id).await?;
        
        // Log das métricas
        log::debug!(
            "Performance metrics for urna {}: CPU: {}%, Memory: {}%, Disk: {}%, Latency: {:?}ms, Response: {}ms",
            urna_id,
            metrics.cpu_usage,
            metrics.memory_usage,
            metrics.disk_usage,
            metrics.network_latency,
            metrics.response_time
        );

        Ok(())
    }

    pub async fn check_alerts(&self, urna_id: Uuid) -> Result<()> {
        // Verificar se há alertas para a urna
        let health_check = {
            let health_checks = self.health_checks.read().await;
            health_checks.get(&urna_id).cloned()
        };

        if let Some(health) = health_check {
            // Verificar alertas de CPU
            if health.performance_metrics.cpu_usage > self.alert_thresholds.cpu_usage_max {
                self.trigger_alert(urna_id, "High CPU Usage", &json!({
                    "cpu_usage": health.performance_metrics.cpu_usage,
                    "threshold": self.alert_thresholds.cpu_usage_max
                })).await?;
            }

            // Verificar alertas de memória
            if health.performance_metrics.memory_usage > self.alert_thresholds.memory_usage_max {
                self.trigger_alert(urna_id, "High Memory Usage", &json!({
                    "memory_usage": health.performance_metrics.memory_usage,
                    "threshold": self.alert_thresholds.memory_usage_max
                })).await?;
            }

            // Verificar alertas de disco
            if health.performance_metrics.disk_usage > self.alert_thresholds.disk_usage_max {
                self.trigger_alert(urna_id, "High Disk Usage", &json!({
                    "disk_usage": health.performance_metrics.disk_usage,
                    "threshold": self.alert_thresholds.disk_usage_max
                })).await?;
            }

            // Verificar alertas de bateria
            if let Some(battery_level) = health.battery_level {
                if battery_level < self.alert_thresholds.battery_level_min {
                    self.trigger_alert(urna_id, "Low Battery", &json!({
                        "battery_level": battery_level,
                        "threshold": self.alert_thresholds.battery_level_min
                    })).await?;
                }
            }

            // Verificar alertas de conectividade
            if !health.network_connectivity {
                self.trigger_alert(urna_id, "Network Connectivity Lost", &json!({})).await?;
            }

            // Verificar alertas de sincronização
            if let Some(last_sync) = health.last_sync {
                let sync_delay = Utc::now().signed_duration_since(last_sync);
                if sync_delay > self.alert_thresholds.sync_delay_max {
                    self.trigger_alert(urna_id, "Sync Delay", &json!({
                        "sync_delay_minutes": sync_delay.num_minutes(),
                        "threshold_minutes": self.alert_thresholds.sync_delay_max.num_minutes()
                    })).await?;
                }
            }
        }

        Ok(())
    }

    async fn trigger_alert(&self, urna_id: Uuid, alert_type: &str, details: &serde_json::Value) -> Result<()> {
        // Disparar alerta
        log::warn!(
            "ALERT - Urna: {}, Type: {}, Details: {}",
            urna_id,
            alert_type,
            details
        );

        // Em implementação real, enviaria notificação para administradores
        // e registraria no sistema de alertas

        Ok(())
    }

    pub async fn get_health_status(&self, urna_id: Uuid) -> Result<Option<UrnaHealthCheck>> {
        let health_checks = self.health_checks.read().await;
        Ok(health_checks.get(&urna_id).cloned())
    }

    pub async fn get_performance_history(
        &self,
        urna_id: Uuid,
        hours: i64,
    ) -> Result<Vec<PerformanceMetrics>> {
        let performance_metrics = self.performance_metrics.read().await;
        let cutoff_time = Utc::now() - Duration::hours(hours);
        
        Ok(performance_metrics
            .get(&urna_id)
            .map(|metrics| {
                metrics
                    .iter()
                    .filter(|m| {
                        // Em implementação real, teria timestamp nas métricas
                        true
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn get_urnas_with_issues(&self) -> Result<Vec<UrnaHealthCheck>> {
        let health_checks = self.health_checks.read().await;
        let mut issues = Vec::new();

        for health in health_checks.values() {
            if health.status != UrnaStatus::Active || !health.errors.is_empty() {
                issues.push(health.clone());
            }
        }

        Ok(issues)
    }

    pub async fn generate_health_report(&self) -> Result<HealthReport> {
        let health_checks = self.health_checks.read().await;
        let total_urnas = health_checks.len();
        let active_urnas = health_checks.values().filter(|h| h.status == UrnaStatus::Active).count();
        let issues = health_checks.values().filter(|h| !h.errors.is_empty()).count();

        Ok(HealthReport {
            total_urnas,
            active_urnas,
            inactive_urnas: total_urnas - active_urnas,
            urna_with_issues: issues,
            generated_at: Utc::now(),
        })
    }
}

impl Clone for UrnaMonitoringService {
    fn clone(&self) -> Self {
        Self {
            health_checks: RwLock::new(HashMap::new()),
            performance_metrics: RwLock::new(HashMap::new()),
            alert_thresholds: self.alert_thresholds.clone(),
            monitoring_interval: self.monitoring_interval,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthReport {
    pub total_urnas: usize,
    pub active_urnas: usize,
    pub inactive_urnas: usize,
    pub urna_with_issues: usize,
    pub generated_at: DateTime<Utc>,
}
