# FORTIS - DevOps e Infraestrutura
## DevOps Automator Perspective

### 🎯 **Visão Geral da Infraestrutura**

O FORTIS implementa uma infraestrutura cloud-native robusta e escalável, garantindo alta disponibilidade, segurança e performance para o sistema de votação eletrônica brasileiro.

---

## ☁️ **Arquitetura de Infraestrutura**

### **Stack Tecnológico**
```yaml
# Infraestrutura Principal
Cloud Provider: AWS (Amazon Web Services)
Container Orchestration: Kubernetes (EKS)
Service Mesh: Istio
CI/CD: GitHub Actions + ArgoCD
Monitoring: Prometheus + Grafana + Jaeger
Logging: ELK Stack (Elasticsearch, Logstash, Kibana)
Security: Vault + Falco + OPA
```

### **Arquitetura de Alta Disponibilidade**
```
┌─────────────────────────────────────────────────────────┐
│                    AWS REGIONS                         │
├─────────────────────────────────────────────────────────┤
│  us-east-1 (Primary)    │  us-west-2 (DR)             │
│  ┌─────────────────┐    │  ┌─────────────────┐        │
│  │   EKS Cluster   │    │  │   EKS Cluster   │        │
│  │   ┌─────────┐   │    │  │   ┌─────────┐   │        │
│  │   │  API    │   │◄───┼──┼──►│  API    │   │        │
│  │   │ Gateway │   │    │  │   │ Gateway │   │        │
│  │   └─────────┘   │    │  │   └─────────┘   │        │
│  │   ┌─────────┐   │    │  │   ┌─────────┐   │        │
│  │   │Microsvcs│   │    │  │   │Microsvcs│   │        │
│  │   └─────────┘   │    │  │   └─────────┘   │        │
│  └─────────────────┘    │  └─────────────────┘        │
└─────────────────────────────────────────────────────────┘
```

---

## 🐳 **Containerização e Kubernetes**

### **Dockerfile Otimizado**
```dockerfile
# Dockerfile.backend
FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/fortis-backend /usr/local/bin/
COPY --from=builder /app/target/release/fortis-backend /usr/local/bin/

EXPOSE 8080
CMD ["fortis-backend"]
```

### **Manifestos Kubernetes**
```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: fortis
  labels:
    name: fortis
    environment: production

---
# k8s/backend-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: fortis-backend
  namespace: fortis
spec:
  replicas: 3
  selector:
    matchLabels:
      app: fortis-backend
  template:
    metadata:
      labels:
        app: fortis-backend
    spec:
      containers:
      - name: backend
        image: fortis/backend:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: fortis-secrets
              key: database-url
        - name: REDIS_URL
          valueFrom:
            secretKeyRef:
              name: fortis-secrets
              key: redis-url
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5

---
# k8s/backend-service.yaml
apiVersion: v1
kind: Service
metadata:
  name: fortis-backend-service
  namespace: fortis
spec:
  selector:
    app: fortis-backend
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: ClusterIP
```

---

## 🚀 **CI/CD Pipeline**

### **GitHub Actions Workflow**
```yaml
# .github/workflows/ci-cd.yml
name: FORTIS CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: fortis

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: |
        cargo test --verbose
        cargo clippy -- -D warnings
        cargo fmt -- --check
    
    - name: Security audit
      run: |
        cargo install cargo-audit
        cargo audit

  build:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Build Docker image
      run: |
        docker build -t ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}/backend:${{ github.sha }} .
        docker build -t ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}/frontend:${{ github.sha }} ./frontend
    
    - name: Push to registry
      run: |
        echo ${{ secrets.GITHUB_TOKEN }} | docker login ${{ env.REGISTRY }} -u ${{ github.actor }} --password-stdin
        docker push ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}/backend:${{ github.sha }}
        docker push ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}/frontend:${{ github.sha }}

  deploy:
    needs: build
    runs-on: ubuntu-latest
    environment: production
    
    steps:
    - name: Deploy to EKS
      run: |
        aws eks update-kubeconfig --region us-east-1 --name fortis-cluster
        kubectl apply -f k8s/
        kubectl rollout status deployment/fortis-backend -n fortis
        kubectl rollout status deployment/fortis-frontend -n fortis
```

### **ArgoCD para GitOps**
```yaml
# argocd/fortis-app.yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: fortis
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/fortis/fortis-k8s
    targetRevision: HEAD
    path: k8s
  destination:
    server: https://kubernetes.default.svc
    namespace: fortis
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
    syncOptions:
    - CreateNamespace=true
```

---

## 📊 **Monitoramento e Observabilidade**

### **Prometheus Configuration**
```yaml
# monitoring/prometheus-config.yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "fortis-rules.yml"

scrape_configs:
  - job_name: 'fortis-backend'
    static_configs:
      - targets: ['fortis-backend-service:80']
    metrics_path: /metrics
    scrape_interval: 5s

  - job_name: 'fortis-frontend'
    static_configs:
      - targets: ['fortis-frontend-service:80']
    metrics_path: /metrics

  - job_name: 'kubernetes-pods'
    kubernetes_sd_configs:
      - role: pod
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_scrape]
        action: keep
        regex: true
```

### **Grafana Dashboards**
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
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(fortis_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      },
      {
        "title": "Error Rate",
        "type": "singlestat",
        "targets": [
          {
            "expr": "rate(fortis_requests_total{status=~\"5..\"}[5m]) / rate(fortis_requests_total[5m]) * 100",
            "legendFormat": "Error Rate %"
          }
        ]
      }
    ]
  }
}
```

### **Jaeger Tracing**
```yaml
# monitoring/jaeger-config.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: jaeger-config
  namespace: fortis
data:
  jaeger.yaml: |
    service:
      name: fortis-backend
    sampling:
      type: const
      param: 1
    reporter:
      logSpans: true
      localAgentHostPort: jaeger-agent:6831
```

---

## 🔒 **Segurança da Infraestrutura**

### **Vault para Gerenciamento de Secrets**
```yaml
# security/vault-config.yaml
apiVersion: v1
kind: Secret
metadata:
  name: fortis-secrets
  namespace: fortis
type: Opaque
data:
  database-url: <base64-encoded>
  redis-url: <base64-encoded>
  jwt-secret: <base64-encoded>
  encryption-key: <base64-encoded>

---
# security/vault-policy.hcl
path "secret/fortis/*" {
  capabilities = ["read"]
}

path "pki/issue/fortis" {
  capabilities = ["create", "update"]
}
```

### **Falco para Detecção de Anomalias**
```yaml
# security/falco-rules.yaml
- rule: Unauthorized Process in Container
  desc: Detect unauthorized processes in containers
  condition: >
    spawned_process and
    container and
    not proc.name in (fortis-backend, fortis-frontend, nginx, redis)
  output: >
    Unauthorized process in container (user=%user.name command=%proc.cmdline container=%container.name)
  priority: WARNING
  tags: [container, process]

- rule: Network Connection to External IP
  desc: Detect network connections to external IPs
  condition: >
    outbound and
    container and
    not fd.sip in (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
  output: >
    Network connection to external IP (user=%user.name command=%proc.cmdline connection=%fd.name container=%container.name)
  priority: WARNING
  tags: [container, network]
```

---

## 🗄️ **Banco de Dados e Storage**

### **PostgreSQL com TimescaleDB**
```yaml
# database/postgresql-deployment.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgresql
  namespace: fortis
spec:
  serviceName: postgresql
  replicas: 1
  selector:
    matchLabels:
      app: postgresql
  template:
    metadata:
      labels:
        app: postgresql
    spec:
      containers:
      - name: postgresql
        image: timescale/timescaledb:latest-pg15
        ports:
        - containerPort: 5432
        env:
        - name: POSTGRES_DB
          value: fortis
        - name: POSTGRES_USER
          value: fortis_user
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: fortis-secrets
              key: postgres-password
        volumeMounts:
        - name: postgresql-storage
          mountPath: /var/lib/postgresql/data
        resources:
          requests:
            memory: "2Gi"
            cpu: "500m"
          limits:
            memory: "4Gi"
            cpu: "1000m"
  volumeClaimTemplates:
  - metadata:
      name: postgresql-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 100Gi
```

### **Redis Cluster**
```yaml
# cache/redis-cluster.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: redis-cluster
  namespace: fortis
spec:
  serviceName: redis-cluster
  replicas: 6
  selector:
    matchLabels:
      app: redis-cluster
  template:
    metadata:
      labels:
        app: redis-cluster
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
        - containerPort: 16379
        command:
        - redis-server
        - /etc/redis/redis.conf
        - --cluster-enabled
        - --cluster-config-file
        - /etc/redis/nodes.conf
        - --cluster-node-timeout
        - "5000"
        - --appendonly
        - "yes"
        volumeMounts:
        - name: redis-config
          mountPath: /etc/redis
        - name: redis-data
          mountPath: /data
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "200m"
```

---

## 🌐 **Load Balancing e CDN**

### **Istio Service Mesh**
```yaml
# networking/istio-gateway.yaml
apiVersion: networking.istio.io/v1alpha3
kind: Gateway
metadata:
  name: fortis-gateway
  namespace: fortis
spec:
  selector:
    istio: ingressgateway
  servers:
  - port:
      number: 80
      name: http
      protocol: HTTP
    hosts:
    - fortis.gov.br
    - api.fortis.gov.br
    tls:
      httpsRedirect: true
  - port:
      number: 443
      name: https
      protocol: HTTPS
    hosts:
    - fortis.gov.br
    - api.fortis.gov.br
    tls:
      mode: SIMPLE
      credentialName: fortis-tls-cert

---
# networking/istio-virtualservice.yaml
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: fortis-vs
  namespace: fortis
spec:
  hosts:
  - fortis.gov.br
  - api.fortis.gov.br
  gateways:
  - fortis-gateway
  http:
  - match:
    - uri:
        prefix: /api
    route:
    - destination:
        host: fortis-backend-service
        port:
          number: 80
  - match:
    - uri:
        prefix: /
    route:
    - destination:
        host: fortis-frontend-service
        port:
          number: 80
```

---

## 📈 **Auto-scaling e Performance**

### **Horizontal Pod Autoscaler**
```yaml
# scaling/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: fortis-backend-hpa
  namespace: fortis
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: fortis-backend
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleUp:
      stabilizationWindowSeconds: 60
      policies:
      - type: Percent
        value: 100
        periodSeconds: 15
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 10
        periodSeconds: 60
```

### **Vertical Pod Autoscaler**
```yaml
# scaling/vpa.yaml
apiVersion: autoscaling.k8s.io/v1
kind: VerticalPodAutoscaler
metadata:
  name: fortis-backend-vpa
  namespace: fortis
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: fortis-backend
  updatePolicy:
    updateMode: "Auto"
  resourcePolicy:
    containerPolicies:
    - containerName: backend
      minAllowed:
        cpu: 100m
        memory: 128Mi
      maxAllowed:
        cpu: 2000m
        memory: 4Gi
```

---

## 🔧 **Backup e Disaster Recovery**

### **Backup Strategy**
```bash
#!/bin/bash
# backup/backup-script.sh

# Database backup
kubectl exec -n fortis postgresql-0 -- pg_dump -U fortis_user fortis > backup_$(date +%Y%m%d_%H%M%S).sql

# Upload to S3
aws s3 cp backup_$(date +%Y%m%d_%H%M%S).sql s3://fortis-backups/database/

# Cleanup old backups (keep 30 days)
aws s3 ls s3://fortis-backups/database/ | awk '$1 < "'$(date -d '30 days ago' '+%Y-%m-%d')'" {print $4}' | xargs -I {} aws s3 rm s3://fortis-backups/database/{}
```

### **Disaster Recovery Plan**
```yaml
# dr/disaster-recovery.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: dr-plan
  namespace: fortis
data:
  dr-procedures: |
    1. Assess the situation and determine RTO/RPO
    2. Activate DR site in us-west-2
    3. Restore database from latest backup
    4. Update DNS to point to DR site
    5. Monitor system health
    6. Plan failback to primary site
```

---

## 📊 **Métricas e Alertas**

### **AlertManager Configuration**
```yaml
# monitoring/alertmanager-config.yaml
global:
  smtp_smarthost: 'localhost:587'
  smtp_from: 'alerts@fortis.gov.br'

route:
  group_by: ['alertname']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 1h
  receiver: 'web.hook'

receivers:
- name: 'web.hook'
  webhook_configs:
  - url: 'http://alertmanager:9093/api/v1/alerts'
    send_resolved: true

- name: 'email'
  email_configs:
  - to: 'admin@fortis.gov.br'
    subject: 'FORTIS Alert: {{ .GroupLabels.alertname }}'
    body: |
      {{ range .Alerts }}
      Alert: {{ .Annotations.summary }}
      Description: {{ .Annotations.description }}
      {{ end }}
```

---

## 🎯 **Próximos Passos**

### **Fase 1: Infraestrutura Base (2 meses)**
- [ ] Setup do cluster EKS
- [ ] Configuração do Istio
- [ ] Implementação do CI/CD
- [ ] Setup de monitoramento básico

### **Fase 2: Segurança e Otimização (2 meses)**
- [ ] Implementação de Vault
- [ ] Configuração de Falco
- [ ] Otimização de performance
- [ ] Setup de backup

### **Fase 3: Produção (2 meses)**
- [ ] Deploy em produção
- [ ] Testes de carga
- [ ] Monitoramento avançado
- [ ] Documentação operacional

---

*Documentação DevOps FORTIS - Desenvolvida pelo DevOps Automator Agent*
