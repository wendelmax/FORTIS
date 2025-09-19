//! Gerenciador de Nós de Consenso
//! 
//! Gerencia nós participantes do consenso distribuído, incluindo
//! descoberta, verificação de saúde e balanceamento de carga.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::net::IpAddr;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::consensus::threshold_signatures::*;

/// Configuração do gerenciador de nós
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeManagerConfig {
    pub discovery_interval: Duration,
    pub health_check_interval: Duration,
    pub node_timeout: Duration,
    pub max_nodes: usize,
    pub min_nodes: usize,
    pub enable_auto_discovery: bool,
    pub enable_load_balancing: bool,
}

impl Default for NodeManagerConfig {
    fn default() -> Self {
        Self {
            discovery_interval: Duration::seconds(30),
            health_check_interval: Duration::seconds(10),
            node_timeout: Duration::minutes(5),
            max_nodes: 10,
            min_nodes: 3,
            enable_auto_discovery: true,
            enable_load_balancing: true,
        }
    }
}

/// Informações de rede do nó
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeNetworkInfo {
    pub ip_address: IpAddr,
    pub port: u16,
    pub protocol: String,
    pub last_ping: DateTime<Utc>,
    pub latency_ms: Option<u64>,
}

/// Status de saúde do nó
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Informações detalhadas do nó
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub name: String,
    pub public_key: String,
    pub network_info: NodeNetworkInfo,
    pub health_status: NodeHealthStatus,
    pub trust_level: u8,
    pub is_active: bool,
    pub last_seen: DateTime<Utc>,
    pub signature_count: u64,
    pub error_count: u64,
    pub performance_score: f64,
    pub metadata: HashMap<String, String>,
}

/// Métricas de performance do nó
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceMetrics {
    pub average_response_time_ms: f64,
    pub success_rate: f64,
    pub error_rate: f64,
    pub signature_success_rate: f64,
    pub uptime_percentage: f64,
    pub last_updated: DateTime<Utc>,
}

/// Gerenciador de nós de consenso
pub struct NodeManager {
    config: NodeManagerConfig,
    nodes: Arc<RwLock<HashMap<String, NodeInfo>>>,
    performance_metrics: Arc<RwLock<HashMap<String, NodePerformanceMetrics>>>,
    discovery_service: Option<Arc<dyn NodeDiscoveryService>>,
}

/// Trait para serviços de descoberta de nós
pub trait NodeDiscoveryService: Send + Sync {
    fn discover_nodes(&self) -> Result<Vec<NodeInfo>>;
    fn register_node(&self, node: &NodeInfo) -> Result<()>;
    fn unregister_node(&self, node_id: &str) -> Result<()>;
}

/// Serviço de descoberta local (para testes)
pub struct LocalDiscoveryService {
    nodes: Arc<RwLock<Vec<NodeInfo>>>,
}

impl LocalDiscoveryService {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl NodeDiscoveryService for LocalDiscoveryService {
    fn discover_nodes(&self) -> Result<Vec<NodeInfo>> {
        // Em uma implementação real, isso seria síncrono
        Ok(vec![])
    }

    fn register_node(&self, _node: &NodeInfo) -> Result<()> {
        // Em uma implementação real, isso seria síncrono
        Ok(())
    }

    fn unregister_node(&self, _node_id: &str) -> Result<()> {
        // Em uma implementação real, isso seria síncrono
        Ok(())
    }
}

impl NodeManager {
    /// Cria novo gerenciador de nós
    pub fn new(config: NodeManagerConfig) -> Self {
        Self {
            config,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            performance_metrics: Arc::new(RwLock::new(HashMap::new())),
            discovery_service: None,
        }
    }

    /// Configura serviço de descoberta
    pub fn set_discovery_service(&mut self, service: Arc<dyn NodeDiscoveryService>) {
        self.discovery_service = Some(service);
    }

    /// Inicializa o gerenciador de nós
    pub async fn initialize(&self) -> Result<()> {
        // Adicionar nós iniciais
        self.add_initial_nodes().await?;
        
        // Iniciar descoberta automática se habilitada
        if self.config.enable_auto_discovery {
            self.start_discovery().await?;
        }
        
        // Iniciar verificação de saúde
        self.start_health_checks().await?;
        
        Ok(())
    }

    /// Adiciona nós iniciais
    async fn add_initial_nodes(&self) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        
        for i in 1..=self.config.min_nodes {
            let (_, public_key) = ThresholdUtils::generate_key_pair()?;
            
            let node_info = NodeInfo {
                id: format!("node_{}", i),
                name: format!("Consensus Node {}", i),
                public_key,
                network_info: NodeNetworkInfo {
                    ip_address: "127.0.0.1".parse()?,
                    port: 8080 + i as u16,
                    protocol: "http".to_string(),
                    last_ping: Utc::now(),
                    latency_ms: Some(1),
                },
                health_status: NodeHealthStatus::Healthy,
                trust_level: 100,
                is_active: true,
                last_seen: Utc::now(),
                signature_count: 0,
                error_count: 0,
                performance_score: 1.0,
                metadata: HashMap::new(),
            };
            
            nodes.insert(node_info.id.clone(), node_info);
        }
        
        Ok(())
    }

    /// Inicia processo de descoberta
    async fn start_discovery(&self) -> Result<()> {
        if let Some(discovery_service) = &self.discovery_service {
            let discovered_nodes = discovery_service.discover_nodes()?;
            
            let mut nodes = self.nodes.write().await;
            for node in discovered_nodes {
                if nodes.len() < self.config.max_nodes {
                    nodes.insert(node.id.clone(), node);
                }
            }
        }
        
        Ok(())
    }

    /// Inicia verificação de saúde dos nós
    async fn start_health_checks(&self) -> Result<()> {
        // Em uma implementação real, isso seria executado em background
        // Por enquanto, apenas simulamos a verificação
        Ok(())
    }

    /// Adiciona um nó ao gerenciador
    pub async fn add_node(&self, node: NodeInfo) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        
        if nodes.len() >= self.config.max_nodes {
            return Err(anyhow!("Maximum nodes reached"));
        }
        
        nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Remove um nó do gerenciador
    pub async fn remove_node(&self, node_id: &str) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        nodes.remove(node_id);
        Ok(())
    }

    /// Obtém informações de um nó
    pub async fn get_node(&self, node_id: &str) -> Option<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes.get(node_id).cloned()
    }

    /// Lista todos os nós
    pub async fn list_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }

    /// Lista nós ativos
    pub async fn list_active_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes.values()
            .filter(|n| n.is_active)
            .cloned()
            .collect()
    }

    /// Lista nós saudáveis
    pub async fn list_healthy_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.read().await;
        nodes.values()
            .filter(|n| n.is_active && n.health_status == NodeHealthStatus::Healthy)
            .cloned()
            .collect()
    }

    /// Atualiza status de saúde de um nó
    pub async fn update_node_health(&self, node_id: &str, status: NodeHealthStatus) -> Result<()> {
        let mut nodes = self.nodes.write().await;
        
        if let Some(node) = nodes.get_mut(node_id) {
            node.health_status = status;
            node.last_seen = Utc::now();
        }
        
        Ok(())
    }

    /// Atualiza métricas de performance de um nó
    pub async fn update_node_performance(
        &self,
        node_id: &str,
        response_time_ms: u64,
        success: bool,
    ) -> Result<()> {
        // Atualizar contadores do nó
        {
            let mut nodes = self.nodes.write().await;
            if let Some(node) = nodes.get_mut(node_id) {
                if success {
                    node.signature_count += 1;
                } else {
                    node.error_count += 1;
                }
                node.last_seen = Utc::now();
            }
        }

        // Atualizar métricas de performance
        {
            let mut metrics = self.performance_metrics.write().await;
            let node_metrics = metrics.entry(node_id.to_string()).or_insert(
                NodePerformanceMetrics {
                    average_response_time_ms: 0.0,
                    success_rate: 100.0,
                    error_rate: 0.0,
                    signature_success_rate: 100.0,
                    uptime_percentage: 100.0,
                    last_updated: Utc::now(),
                }
            );

            // Atualizar tempo médio de resposta
            let total_operations = node_metrics.success_rate + node_metrics.error_rate;
            node_metrics.average_response_time_ms = 
                (node_metrics.average_response_time_ms * total_operations + response_time_ms as f64) 
                / (total_operations + 1.0);

            // Atualizar taxa de sucesso
            if success {
                node_metrics.success_rate += 1.0;
            } else {
                node_metrics.error_rate += 1.0;
            }

            let total = node_metrics.success_rate + node_metrics.error_rate;
            node_metrics.signature_success_rate = (node_metrics.success_rate / total) * 100.0;
            node_metrics.last_updated = Utc::now();
        }

        Ok(())
    }

    /// Seleciona nós para consenso baseado em critérios
    pub async fn select_nodes_for_consensus(
        &self,
        required_count: usize,
        _operation: &str, // Simplificado para evitar dependência circular
    ) -> Result<Vec<NodeInfo>> {
        let healthy_nodes = self.list_healthy_nodes().await;
        
        if healthy_nodes.len() < required_count {
            return Err(anyhow!("Not enough healthy nodes available"));
        }

        // Ordenar nós por performance score (maior é melhor)
        let mut sorted_nodes = healthy_nodes;
        sorted_nodes.sort_by(|a, b| b.performance_score.partial_cmp(&a.performance_score).unwrap());

        // Selecionar os melhores nós
        let selected = sorted_nodes.into_iter().take(required_count).collect();
        Ok(selected)
    }

    /// Obtém estatísticas do gerenciador
    pub async fn get_stats(&self) -> NodeManagerStats {
        let nodes = self.nodes.read().await;
        let active_nodes = nodes.values().filter(|n| n.is_active).count();
        let healthy_nodes = nodes.values()
            .filter(|n| n.is_active && n.health_status == NodeHealthStatus::Healthy)
            .count();
        let total_signatures: u64 = nodes.values().map(|n| n.signature_count).sum();
        let total_errors: u64 = nodes.values().map(|n| n.error_count).sum();

        NodeManagerStats {
            total_nodes: nodes.len(),
            active_nodes,
            healthy_nodes,
            total_signatures,
            total_errors,
            average_performance: nodes.values()
                .map(|n| n.performance_score)
                .sum::<f64>() / nodes.len() as f64,
            last_updated: Utc::now(),
        }
    }

    /// Limpa nós inativos
    pub async fn cleanup_inactive_nodes(&self) -> usize {
        let mut nodes = self.nodes.write().await;
        let now = Utc::now();
        let mut removed = 0;

        nodes.retain(|_, node| {
            if now - node.last_seen > self.config.node_timeout {
                removed += 1;
                false
            } else {
                true
            }
        });

        removed
    }

    /// Obtém métricas de performance de um nó
    pub async fn get_node_metrics(&self, node_id: &str) -> Option<NodePerformanceMetrics> {
        let metrics = self.performance_metrics.read().await;
        metrics.get(node_id).cloned()
    }
}

/// Estatísticas do gerenciador de nós
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeManagerStats {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub healthy_nodes: usize,
    pub total_signatures: u64,
    pub total_errors: u64,
    pub average_performance: f64,
    pub last_updated: DateTime<Utc>,
}

/// Utilitários para gerenciamento de nós
pub struct NodeUtils;

impl NodeUtils {
    /// Calcula score de performance de um nó
    pub fn calculate_performance_score(
        signature_count: u64,
        error_count: u64,
        response_time_ms: u64,
        uptime_percentage: f64,
    ) -> f64 {
        let success_rate = if signature_count + error_count > 0 {
            signature_count as f64 / (signature_count + error_count) as f64
        } else {
            1.0
        };

        let response_score = if response_time_ms < 100 {
            1.0
        } else if response_time_ms < 500 {
            0.8
        } else if response_time_ms < 1000 {
            0.6
        } else {
            0.4
        };

        (success_rate * 0.4 + response_score * 0.3 + uptime_percentage * 0.3) * 100.0
    }

    /// Verifica se um nó está disponível para consenso
    pub fn is_node_available(node: &NodeInfo, min_trust_level: u8) -> bool {
        node.is_active
            && node.health_status == NodeHealthStatus::Healthy
            && node.trust_level >= min_trust_level
            && node.performance_score > 50.0
    }

    /// Calcula prioridade de um nó para seleção
    pub fn calculate_node_priority(node: &NodeInfo) -> f64 {
        let trust_weight = node.trust_level as f64 / 100.0;
        let performance_weight = node.performance_score / 100.0;
        let health_weight = match node.health_status {
            NodeHealthStatus::Healthy => 1.0,
            NodeHealthStatus::Degraded => 0.7,
            NodeHealthStatus::Unhealthy => 0.3,
            NodeHealthStatus::Unknown => 0.1,
        };

        (trust_weight * 0.4 + performance_weight * 0.4 + health_weight * 0.2) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_node_manager_creation() {
        let config = NodeManagerConfig::default();
        let manager = NodeManager::new(config);
        manager.initialize().await.unwrap();
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_nodes, 3); // min_nodes
        assert_eq!(stats.active_nodes, 3);
    }

    #[tokio::test]
    async fn test_add_node() {
        let config = NodeManagerConfig::default();
        let manager = NodeManager::new(config);
        manager.initialize().await.unwrap();

        let (_, public_key) = ThresholdUtils::generate_key_pair().unwrap();
        let node = NodeInfo {
            id: "test_node".to_string(),
            name: "Test Node".to_string(),
            public_key,
            network_info: NodeNetworkInfo {
                ip_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port: 8080,
                protocol: "http".to_string(),
                last_ping: Utc::now(),
                latency_ms: Some(1),
            },
            health_status: NodeHealthStatus::Healthy,
            trust_level: 100,
            is_active: true,
            last_seen: Utc::now(),
            signature_count: 0,
            error_count: 0,
            performance_score: 1.0,
            metadata: HashMap::new(),
        };

        let result = manager.add_node(node).await;
        assert!(result.is_ok());
        
        let stats = manager.get_stats().await;
        assert_eq!(stats.total_nodes, 4);
    }

    #[tokio::test]
    async fn test_select_nodes_for_consensus() {
        let config = NodeManagerConfig::default();
        let manager = NodeManager::new(config);
        manager.initialize().await.unwrap();

        let selected = manager.select_nodes_for_consensus(2, "ElectionStart").await;
        assert!(selected.is_ok());
        
        let nodes = selected.unwrap();
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_node_utils() {
        // Teste de cálculo de score de performance
        let score = NodeUtils::calculate_performance_score(100, 0, 50, 99.9);
        assert!(score > 90.0);

        // Teste de disponibilidade do nó
        let (_, public_key) = ThresholdUtils::generate_key_pair().unwrap();
        let node = NodeInfo {
            id: "test".to_string(),
            name: "Test".to_string(),
            public_key,
            network_info: NodeNetworkInfo {
                ip_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port: 8080,
                protocol: "http".to_string(),
                last_ping: Utc::now(),
                latency_ms: Some(1),
            },
            health_status: NodeHealthStatus::Healthy,
            trust_level: 100,
            is_active: true,
            last_seen: Utc::now(),
            signature_count: 100,
            error_count: 0,
            performance_score: 95.0,
            metadata: HashMap::new(),
        };

        assert!(NodeUtils::is_node_available(&node, 80));
        assert!(!NodeUtils::is_node_available(&node, 101));

        // Teste de cálculo de prioridade
        let priority = NodeUtils::calculate_node_priority(&node);
        assert!(priority > 80.0);
    }
}
