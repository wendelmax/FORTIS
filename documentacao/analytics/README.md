# FORTIS - Analytics e Métricas
## Analytics Reporter Perspective

### 🎯 **Visão Geral de Analytics**

O FORTIS implementa um sistema de analytics abrangente que monitora métricas técnicas, de negócio e de experiência do usuário, fornecendo insights valiosos para otimização contínua e transparência total do sistema de votação eletrônica.

---

## 📊 **Framework de Métricas**

### **Categorias de Métricas**
```
┌─────────────────────────────────────────────────────────┐
│                METRICS FRAMEWORK                       │
├─────────────────────────────────────────────────────────┤
│ Technical Metrics (40%) - Performance, Availability    │
│ Business Metrics (30%) - Voter Engagement, Success     │
│ Security Metrics (20%) - Fraud Detection, Incidents    │
│ User Experience (10%) - Usability, Accessibility       │
└─────────────────────────────────────────────────────────┘
```

### **Hierarquia de Métricas**
- **KPIs**: Indicadores-chave de performance
- **OKRs**: Objetivos e resultados-chave
- **SLIs**: Service Level Indicators
- **SLOs**: Service Level Objectives

---

## 🔧 **Métricas Técnicas**

### **Performance Metrics**
```rust
// analytics/performance_metrics.rs
use prometheus::{Counter, Histogram, Gauge, Registry};
use std::collections::HashMap;

lazy_static! {
    // Request Metrics
    static ref HTTP_REQUESTS_TOTAL: Counter = Counter::new(
        "fortis_http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    static ref HTTP_REQUEST_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "fortis_http_request_duration_seconds",
            "HTTP request duration in seconds"
        ).buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0])
    ).unwrap();
    
    // Voting Metrics
    static ref VOTES_TOTAL: Counter = Counter::new(
        "fortis_votes_total",
        "Total number of votes cast"
    ).unwrap();
    
    static ref VOTE_PROCESSING_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new(
            "fortis_vote_processing_duration_seconds",
            "Vote processing duration in seconds"
        ).buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0])
    ).unwrap();
    
    // System Metrics
    static ref ACTIVE_SESSIONS: Gauge = Gauge::new(
        "fortis_active_sessions",
        "Number of active voting sessions"
    ).unwrap();
    
    static ref DATABASE_CONNECTIONS: Gauge = Gauge::new(
        "fortis_database_connections_active",
        "Number of active database connections"
    ).unwrap();
    
    static ref CACHE_HIT_RATIO: Gauge = Gauge::new(
        "fortis_cache_hit_ratio",
        "Cache hit ratio percentage"
    ).unwrap();
}

pub struct PerformanceMetrics {
    registry: Registry,
    custom_metrics: HashMap<String, Box<dyn prometheus::Collector>>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        // Registrar métricas padrão
        registry.register(Box::new(HTTP_REQUESTS_TOTAL.clone())).unwrap();
        registry.register(Box::new(HTTP_REQUEST_DURATION.clone())).unwrap();
        registry.register(Box::new(VOTES_TOTAL.clone())).unwrap();
        registry.register(Box::new(VOTE_PROCESSING_DURATION.clone())).unwrap();
        registry.register(Box::new(ACTIVE_SESSIONS.clone())).unwrap();
        registry.register(Box::new(DATABASE_CONNECTIONS.clone())).unwrap();
        registry.register(Box::new(CACHE_HIT_RATIO.clone())).unwrap();
        
        Self {
            registry,
            custom_metrics: HashMap::new(),
        }
    }
    
    pub fn record_vote(&self, processing_time: f64) {
        VOTES_TOTAL.inc();
        VOTE_PROCESSING_DURATION.observe(processing_time);
    }
    
    pub fn record_http_request(&self, duration: f64, status_code: u16) {
        HTTP_REQUESTS_TOTAL.inc();
        HTTP_REQUEST_DURATION.observe(duration);
        
        // Métricas por status code
        let status_counter = Counter::new(
            &format!("fortis_http_requests_total{{status=\"{}\"}}", status_code),
            "HTTP requests by status code"
        ).unwrap();
        status_counter.inc();
    }
    
    pub fn update_active_sessions(&self, count: f64) {
        ACTIVE_SESSIONS.set(count);
    }
    
    pub fn update_cache_hit_ratio(&self, ratio: f64) {
        CACHE_HIT_RATIO.set(ratio);
    }
}
```

### **Availability Metrics**
```rust
// analytics/availability_metrics.rs
use chrono::{DateTime, Utc, Duration};

pub struct AvailabilityMetrics {
    uptime_start: DateTime<Utc>,
    downtime_events: Vec<DowntimeEvent>,
    maintenance_windows: Vec<MaintenanceWindow>,
}

#[derive(Debug, Clone)]
pub struct DowntimeEvent {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<Duration>,
    pub cause: String,
    pub severity: Severity,
    pub affected_services: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Critical,    // > 1% dos usuários afetados
    High,        // 0.1% - 1% dos usuários afetados
    Medium,      // 0.01% - 0.1% dos usuários afetados
    Low,         // < 0.01% dos usuários afetados
}

impl AvailabilityMetrics {
    pub fn new() -> Self {
        Self {
            uptime_start: Utc::now(),
            downtime_events: Vec::new(),
            maintenance_windows: Vec::new(),
        }
    }
    
    pub fn record_downtime(&mut self, event: DowntimeEvent) {
        self.downtime_events.push(event);
    }
    
    pub fn calculate_availability(&self, period: Duration) -> f64 {
        let end_time = Utc::now();
        let start_time = end_time - period;
        
        let total_downtime: Duration = self.downtime_events
            .iter()
            .filter(|event| {
                event.start_time >= start_time && 
                event.start_time <= end_time
            })
            .map(|event| {
                event.duration.unwrap_or_else(|| {
                    end_time.signed_duration_since(event.start_time)
                })
            })
            .sum();
        
        let total_period = period.num_seconds() as f64;
        let downtime_seconds = total_downtime.num_seconds() as f64;
        
        ((total_period - downtime_seconds) / total_period) * 100.0
    }
    
    pub fn get_uptime(&self) -> Duration {
        Utc::now().signed_duration_since(self.uptime_start)
    }
    
    pub fn get_mttr(&self) -> Option<Duration> {
        let resolved_events: Vec<&DowntimeEvent> = self.downtime_events
            .iter()
            .filter(|event| event.duration.is_some())
            .collect();
        
        if resolved_events.is_empty() {
            return None;
        }
        
        let total_resolution_time: Duration = resolved_events
            .iter()
            .map(|event| event.duration.unwrap())
            .sum();
        
        Some(total_resolution_time / resolved_events.len() as i32)
    }
}
```

---

## 💼 **Métricas de Negócio**

### **Voter Engagement Metrics**
```rust
// analytics/business_metrics.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VoterEngagementMetrics {
    pub total_eligible_voters: u64,
    pub total_votes_cast: u64,
    pub voter_turnout_percentage: f64,
    pub votes_by_hour: HashMap<u8, u64>,
    pub votes_by_region: HashMap<String, u64>,
    pub votes_by_age_group: HashMap<String, u64>,
    pub votes_by_gender: HashMap<String, u64>,
    pub average_voting_time: f64,
    pub completion_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectionSuccessMetrics {
    pub election_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_duration: Duration,
    pub peak_voting_hour: u8,
    pub peak_voting_rate: f64, // votos por minuto
    pub system_uptime_percentage: f64,
    pub fraud_detection_rate: f64,
    pub user_satisfaction_score: f64,
    pub accessibility_usage: f64,
}

impl VoterEngagementMetrics {
    pub fn calculate_turnout(&mut self) {
        self.voter_turnout_percentage = 
            (self.total_votes_cast as f64 / self.total_eligible_voters as f64) * 100.0;
    }
    
    pub fn calculate_completion_rate(&mut self, started_votes: u64) {
        self.completion_rate = 
            (self.total_votes_cast as f64 / started_votes as f64) * 100.0;
    }
    
    pub fn get_peak_voting_hour(&self) -> u8 {
        self.votes_by_hour
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(&hour, _)| hour)
            .unwrap_or(0)
    }
    
    pub fn get_regional_distribution(&self) -> HashMap<String, f64> {
        let total = self.total_votes_cast as f64;
        self.votes_by_region
            .iter()
            .map(|(region, &count)| {
                (region.clone(), (count as f64 / total) * 100.0)
            })
            .collect()
    }
}

pub struct BusinessMetricsCollector {
    engagement_metrics: VoterEngagementMetrics,
    election_metrics: Vec<ElectionSuccessMetrics>,
}

impl BusinessMetricsCollector {
    pub fn new() -> Self {
        Self {
            engagement_metrics: VoterEngagementMetrics {
                total_eligible_voters: 0,
                total_votes_cast: 0,
                voter_turnout_percentage: 0.0,
                votes_by_hour: HashMap::new(),
                votes_by_region: HashMap::new(),
                votes_by_age_group: HashMap::new(),
                votes_by_gender: HashMap::new(),
                average_voting_time: 0.0,
                completion_rate: 0.0,
            },
            election_metrics: Vec::new(),
        }
    }
    
    pub fn record_vote(&mut self, vote_data: VoteData) {
        self.engagement_metrics.total_votes_cast += 1;
        
        // Atualizar métricas por hora
        let hour = vote_data.timestamp.hour();
        *self.engagement_metrics.votes_by_hour.entry(hour).or_insert(0) += 1;
        
        // Atualizar métricas por região
        *self.engagement_metrics.votes_by_region
            .entry(vote_data.region.clone())
            .or_insert(0) += 1;
        
        // Atualizar métricas demográficas
        if let Some(age_group) = vote_data.age_group {
            *self.engagement_metrics.votes_by_age_group
                .entry(age_group)
                .or_insert(0) += 1;
        }
        
        if let Some(gender) = vote_data.gender {
            *self.engagement_metrics.votes_by_gender
                .entry(gender)
                .or_insert(0) += 1;
        }
        
        // Atualizar tempo médio de votação
        self.update_average_voting_time(vote_data.voting_duration);
    }
    
    fn update_average_voting_time(&mut self, new_duration: f64) {
        let current_avg = self.engagement_metrics.average_voting_time;
        let total_votes = self.engagement_metrics.total_votes_cast as f64;
        
        self.engagement_metrics.average_voting_time = 
            ((current_avg * (total_votes - 1.0)) + new_duration) / total_votes;
    }
}
```

### **ROI e Impact Metrics**
```rust
// analytics/roi_metrics.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ROIMetrics {
    pub total_investment: f64,
    pub operational_savings: f64,
    pub efficiency_gains: f64,
    pub fraud_prevention_savings: f64,
    pub transparency_benefits: f64,
    pub net_roi: f64,
    pub payback_period_months: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImpactMetrics {
    pub democracy_impact: DemocracyImpact,
    pub technology_impact: TechnologyImpact,
    pub social_impact: SocialImpact,
    pub economic_impact: EconomicImpact,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DemocracyImpact {
    pub voter_confidence_score: f64,
    pub transparency_index: f64,
    pub auditability_score: f64,
    pub accessibility_score: f64,
    pub participation_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TechnologyImpact {
    pub innovation_index: f64,
    pub export_potential: f64,
    pub technology_transfer: f64,
    pub open_source_contributions: f64,
}

impl ROIMetrics {
    pub fn calculate_net_roi(&mut self) {
        let total_benefits = 
            self.operational_savings + 
            self.efficiency_gains + 
            self.fraud_prevention_savings + 
            self.transparency_benefits;
        
        self.net_roi = ((total_benefits - self.total_investment) / self.total_investment) * 100.0;
    }
    
    pub fn calculate_payback_period(&mut self) {
        let monthly_benefits = 
            (self.operational_savings + self.efficiency_gains) / 12.0;
        
        self.payback_period_months = self.total_investment / monthly_benefits;
    }
}
```

---

## 🔒 **Métricas de Segurança**

### **Security Metrics**
```rust
// analytics/security_metrics.rs
use std::collections::HashMap;

pub struct SecurityMetrics {
    pub authentication_attempts: u64,
    pub authentication_failures: u64,
    pub fraud_detections: u64,
    pub security_incidents: u64,
    pub false_positives: u64,
    pub false_negatives: u64,
    pub threat_level: ThreatLevel,
    pub response_times: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl SecurityMetrics {
    pub fn calculate_authentication_success_rate(&self) -> f64 {
        if self.authentication_attempts == 0 {
            return 0.0;
        }
        
        let success_rate = (self.authentication_attempts - self.authentication_failures) as f64 
            / self.authentication_attempts as f64;
        
        success_rate * 100.0
    }
    
    pub fn calculate_fraud_detection_accuracy(&self) -> f64 {
        let total_detections = self.fraud_detections + self.false_positives;
        
        if total_detections == 0 {
            return 0.0;
        }
        
        let accuracy = (self.fraud_detections as f64 / total_detections as f64) * 100.0;
        accuracy
    }
    
    pub fn calculate_false_positive_rate(&self) -> f64 {
        let total_detections = self.fraud_detections + self.false_positives;
        
        if total_detections == 0 {
            return 0.0;
        }
        
        (self.false_positives as f64 / total_detections as f64) * 100.0
    }
    
    pub fn update_threat_level(&mut self) {
        let incident_rate = self.security_incidents as f64 / 24.0; // por hora
        
        self.threat_level = if incident_rate > 10.0 {
            ThreatLevel::Critical
        } else if incident_rate > 5.0 {
            ThreatLevel::High
        } else if incident_rate > 2.0 {
            ThreatLevel::Medium
        } else {
            ThreatLevel::Low
        };
    }
}
```

---

## 📱 **Métricas de Experiência do Usuário**

### **UX Metrics**
```rust
// analytics/ux_metrics.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UXMetrics {
    pub task_completion_rate: f64,
    pub average_task_time: f64,
    pub error_rate: f64,
    pub user_satisfaction_score: f64,
    pub accessibility_usage: f64,
    pub mobile_usage_percentage: f64,
    pub accessibility_features_used: HashMap<String, u64>,
    pub user_feedback: Vec<UserFeedback>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFeedback {
    pub user_id: String,
    pub rating: u8, // 1-5
    pub comments: String,
    pub timestamp: DateTime<Utc>,
    pub category: FeedbackCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeedbackCategory {
    Usability,
    Performance,
    Accessibility,
    Security,
    General,
}

impl UXMetrics {
    pub fn calculate_task_completion_rate(&mut self, completed_tasks: u64, total_tasks: u64) {
        if total_tasks > 0 {
            self.task_completion_rate = (completed_tasks as f64 / total_tasks as f64) * 100.0;
        }
    }
    
    pub fn calculate_average_satisfaction(&mut self) {
        if self.user_feedback.is_empty() {
            return;
        }
        
        let total_rating: u64 = self.user_feedback
            .iter()
            .map(|feedback| feedback.rating as u64)
            .sum();
        
        self.user_satisfaction_score = total_rating as f64 / self.user_feedback.len() as f64;
    }
    
    pub fn get_accessibility_usage_percentage(&self) -> f64 {
        self.accessibility_usage
    }
    
    pub fn get_most_used_accessibility_feature(&self) -> Option<String> {
        self.accessibility_features_used
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(feature, _)| feature.clone())
    }
}
```

---

## 📊 **Dashboards e Visualizações**

### **Grafana Dashboard Configuration**
```json
{
  "dashboard": {
    "title": "FORTIS System Overview",
    "panels": [
      {
        "title": "Votes Per Second",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(fortis_votes_total[5m])",
            "legendFormat": "Votes/sec"
          }
        ],
        "yAxes": [
          {
            "label": "Votes per Second",
            "min": 0
          }
        ]
      },
      {
        "title": "Voter Turnout by Region",
        "type": "piechart",
        "targets": [
          {
            "expr": "sum by (region) (fortis_votes_by_region)",
            "legendFormat": "{{region}}"
          }
        ]
      },
      {
        "title": "Authentication Success Rate",
        "type": "singlestat",
        "targets": [
          {
            "expr": "rate(fortis_auth_success_total[5m]) / rate(fortis_auth_attempts_total[5m]) * 100",
            "legendFormat": "Success Rate %"
          }
        ],
        "thresholds": "70,85,95"
      },
      {
        "title": "System Availability",
        "type": "singlestat",
        "targets": [
          {
            "expr": "avg_over_time(fortis_system_availability[24h])",
            "legendFormat": "Availability %"
          }
        ],
        "thresholds": "99,99.5,99.9"
      },
      {
        "title": "Response Time Distribution",
        "type": "histogram",
        "targets": [
          {
            "expr": "histogram_quantile(0.50, rate(fortis_http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "50th percentile"
          },
          {
            "expr": "histogram_quantile(0.95, rate(fortis_http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          },
          {
            "expr": "histogram_quantile(0.99, rate(fortis_http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "99th percentile"
          }
        ]
      }
    ]
  }
}
```

### **Real-time Monitoring**
```rust
// analytics/real_time_monitoring.rs
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RealTimeMonitor {
    metrics: Arc<RwLock<SystemMetrics>>,
    alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub current_votes_per_second: f64,
    pub active_sessions: u64,
    pub system_load: f64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub max_votes_per_second: f64,
    pub max_active_sessions: u64,
    pub max_system_load: f64,
    pub max_memory_usage: f64,
    pub max_cpu_usage: f64,
    pub max_error_rate: f64,
}

impl RealTimeMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(SystemMetrics {
                current_votes_per_second: 0.0,
                active_sessions: 0,
                system_load: 0.0,
                memory_usage: 0.0,
                cpu_usage: 0.0,
                error_rate: 0.0,
            })),
            alert_thresholds: AlertThresholds {
                max_votes_per_second: 1000.0,
                max_active_sessions: 10000,
                max_system_load: 0.8,
                max_memory_usage: 0.9,
                max_cpu_usage: 0.9,
                max_error_rate: 0.05,
            },
        }
    }
    
    pub async fn start_monitoring(&self) {
        let mut interval = interval(Duration::from_secs(1));
        let metrics = Arc::clone(&self.metrics);
        let thresholds = self.alert_thresholds.clone();
        
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                
                // Coletar métricas em tempo real
                let current_metrics = Self::collect_metrics().await;
                
                // Atualizar métricas
                {
                    let mut metrics_guard = metrics.write().await;
                    *metrics_guard = current_metrics.clone();
                }
                
                // Verificar alertas
                Self::check_alerts(&current_metrics, &thresholds).await;
            }
        });
    }
    
    async fn collect_metrics() -> SystemMetrics {
        // Implementar coleta de métricas em tempo real
        SystemMetrics {
            current_votes_per_second: 0.0,
            active_sessions: 0,
            system_load: 0.0,
            memory_usage: 0.0,
            cpu_usage: 0.0,
            error_rate: 0.0,
        }
    }
    
    async fn check_alerts(metrics: &SystemMetrics, thresholds: &AlertThresholds) {
        let mut alerts = Vec::new();
        
        if metrics.current_votes_per_second > thresholds.max_votes_per_second {
            alerts.push("High vote rate detected".to_string());
        }
        
        if metrics.active_sessions > thresholds.max_active_sessions {
            alerts.push("High number of active sessions".to_string());
        }
        
        if metrics.system_load > thresholds.max_system_load {
            alerts.push("High system load".to_string());
        }
        
        if metrics.memory_usage > thresholds.max_memory_usage {
            alerts.push("High memory usage".to_string());
        }
        
        if metrics.cpu_usage > thresholds.max_cpu_usage {
            alerts.push("High CPU usage".to_string());
        }
        
        if metrics.error_rate > thresholds.max_error_rate {
            alerts.push("High error rate".to_string());
        }
        
        // Enviar alertas
        for alert in alerts {
            Self::send_alert(alert).await;
        }
    }
    
    async fn send_alert(message: String) {
        // Implementar envio de alertas
        println!("ALERT: {}", message);
    }
}
```

---

## 📈 **Relatórios e Insights**

### **Relatório de Performance**
```rust
// analytics/performance_report.rs
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub period: DateRange,
    pub summary: PerformanceSummary,
    pub detailed_metrics: DetailedMetrics,
    pub recommendations: Vec<Recommendation>,
    pub trends: Vec<Trend>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub total_votes: u64,
    pub average_response_time: f64,
    pub availability_percentage: f64,
    pub error_rate: f64,
    pub peak_load: f64,
    pub user_satisfaction: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedMetrics {
    pub response_time_p50: f64,
    pub response_time_p95: f64,
    pub response_time_p99: f64,
    pub throughput: f64,
    pub concurrent_users: u64,
    pub database_performance: DatabaseMetrics,
    pub cache_performance: CacheMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub query_time_avg: f64,
    pub connection_pool_usage: f64,
    pub slow_queries: u64,
    pub deadlocks: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub hit_ratio: f64,
    pub miss_ratio: f64,
    pub eviction_rate: f64,
    pub memory_usage: f64,
}

impl PerformanceReport {
    pub fn generate_monthly_report() -> Self {
        // Implementar geração de relatório mensal
        todo!()
    }
    
    pub fn generate_daily_report() -> Self {
        // Implementar geração de relatório diário
        todo!()
    }
    
    pub fn generate_election_report(election_id: String) -> Self {
        // Implementar geração de relatório por eleição
        todo!()
    }
}
```

---

## 🎯 **Próximos Passos**

### **Fase 1: Métricas Base (2 meses)**
- [ ] Implementar métricas técnicas
- [ ] Configurar dashboards básicos
- [ ] Alertas de sistema
- [ ] Relatórios automáticos

### **Fase 2: Métricas Avançadas (2 meses)**
- [ ] Métricas de negócio
- [ ] Métricas de UX
- [ ] Análise preditiva
- [ ] Insights automáticos

### **Fase 3: Otimização (2 meses)**
- [ ] A/B testing
- [ ] Otimização contínua
- [ ] Relatórios executivos
- [ ] Tomada de decisão baseada em dados

---

*Documentação de Analytics FORTIS - Desenvolvida pelo Analytics Reporter Agent*
