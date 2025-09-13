# FORTIS - Configurações de Infraestrutura
## DevOps Automator Perspective

### 🎯 **Visão Geral da Infraestrutura**

O FORTIS implementa uma infraestrutura cloud-native baseada em Kubernetes com alta disponibilidade, escalabilidade automática e monitoramento completo para suportar 150+ milhões de eleitores brasileiros.

---

## 🏗️ **Arquitetura de Infraestrutura**

### **1. Topologia da Rede**
```
┌─────────────────────────────────────────────────────────┐
│                    CLOUD INFRASTRUCTURE                 │
├─────────────────────────────────────────────────────────┤
│ • AWS/GCP Multi-Region (São Paulo, Rio, Brasília)      │
│ • Kubernetes Clusters (EKS/GKE)                        │
│ • Istio Service Mesh (segurança + observabilidade)     │
│ • CDN CloudFlare (performance global)                  │
│ • Vault (gerenciamento de segredos)                    │
└─────────────────────────────────────────────────────────┘
```

### **2. Componentes Principais**
- **Kubernetes Clusters**: 3 regiões (SP, RJ, DF)
- **Istio Service Mesh**: Roteamento, segurança, observabilidade
- **Prometheus + Grafana**: Monitoramento e alertas
- **ELK Stack**: Logs centralizados
- **Vault**: Gerenciamento de segredos
- **Redis Cluster**: Cache distribuído
- **PostgreSQL**: Banco principal com replicação

---

## ☸️ **Configurações Kubernetes**

### **1. Namespace Principal**
```yaml
# namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: fortis
  labels:
    name: fortis
    environment: production
---
apiVersion: v1
kind: ResourceQuota
metadata:
  name: fortis-quota
  namespace: fortis
spec:
  hard:
    requests.cpu: "100"
    requests.memory: 200Gi
    limits.cpu: "200"
    limits.memory: 400Gi
    persistentvolumeclaims: "10"
```

### **2. ConfigMap de Configuração**
```yaml
# configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: fortis-config
  namespace: fortis
data:
  # Database
  DATABASE_URL: "postgresql://fortis:password@postgres-service:5432/fortis"
  REDIS_URL: "redis://redis-service:6379"
  
  # Blockchain
  POLYGON_RPC_URL: "https://polygon-rpc.com"
  POLYGON_PRIVATE_KEY: "vault:secret/data/fortis#POLYGON_PRIVATE_KEY"
  
  # TSE Integration
  TSE_API_URL: "https://api.tse.jus.br"
  TSE_CERTIFICATE: "vault:secret/data/fortis#TSE_CERTIFICATE"
  
  # Security
  JWT_SECRET: "vault:secret/data/fortis#JWT_SECRET"
  ENCRYPTION_KEY: "vault:secret/data/fortis#ENCRYPTION_KEY"
  
  # Monitoring
  PROMETHEUS_URL: "http://prometheus-service:9090"
  GRAFANA_URL: "http://grafana-service:3000"
```

### **3. Secret Management com Vault**
```yaml
# vault-secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: fortis-vault-secret
  namespace: fortis
type: Opaque
data:
  vault-token: <base64-encoded-vault-token>
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: fortis-vault-sa
  namespace: fortis
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: fortis-vault-binding
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: vault-auth
subjects:
- kind: ServiceAccount
  name: fortis-vault-sa
  namespace: fortis
```

---

## 🚀 **Deployments dos Serviços**

### **1. API Gateway (Kong)**
```yaml
# api-gateway.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
  namespace: fortis
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-gateway
  template:
    metadata:
      labels:
        app: api-gateway
    spec:
      containers:
      - name: kong
        image: kong:3.4
        ports:
        - containerPort: 8000
        - containerPort: 8443
        env:
        - name: KONG_DATABASE
          value: "off"
        - name: KONG_DECLARATIVE_CONFIG
          value: "/kong/kong.yml"
        - name: KONG_PROXY_ACCESS_LOG
          value: "/dev/stdout"
        - name: KONG_ADMIN_ACCESS_LOG
          value: "/dev/stdout"
        - name: KONG_PROXY_ERROR_LOG
          value: "/dev/stderr"
        - name: KONG_ADMIN_ERROR_LOG
          value: "/dev/stderr"
        - name: KONG_ADMIN_LISTEN
          value: "0.0.0.0:8001"
        volumeMounts:
        - name: kong-config
          mountPath: /kong
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /status
            port: 8000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /status
            port: 8000
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: kong-config
        configMap:
          name: kong-config
---
apiVersion: v1
kind: Service
metadata:
  name: api-gateway-service
  namespace: fortis
spec:
  selector:
    app: api-gateway
  ports:
  - name: http
    port: 80
    targetPort: 8000
  - name: https
    port: 443
    targetPort: 8443
  type: LoadBalancer
```

### **2. Backend Services (Rust)**
```yaml
# backend-services.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: fortis-backend
  namespace: fortis
spec:
  replicas: 5
  selector:
    matchLabels:
      app: fortis-backend
  template:
    metadata:
      labels:
        app: fortis-backend
    spec:
      containers:
      - name: fortis-backend
        image: fortis/backend:latest
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
        - name: DATABASE_URL
          valueFrom:
            configMapKeyRef:
              name: fortis-config
              key: DATABASE_URL
        - name: REDIS_URL
          valueFrom:
            configMapKeyRef:
              name: fortis-config
              key: REDIS_URL
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: fortis-secrets
              key: jwt-secret
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
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
        volumeMounts:
        - name: vault-secrets
          mountPath: /vault/secrets
          readOnly: true
      volumes:
      - name: vault-secrets
        secret:
          secretName: fortis-vault-secret
---
apiVersion: v1
kind: Service
metadata:
  name: fortis-backend-service
  namespace: fortis
spec:
  selector:
    app: fortis-backend
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP
```

### **3. Frontend (React)**
```yaml
# frontend.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: fortis-frontend
  namespace: fortis
spec:
  replicas: 3
  selector:
    matchLabels:
      app: fortis-frontend
  template:
    metadata:
      labels:
        app: fortis-frontend
    spec:
      containers:
      - name: fortis-frontend
        image: fortis/frontend:latest
        ports:
        - containerPort: 3000
        env:
        - name: REACT_APP_API_URL
          value: "https://api.fortis.gov.br"
        - name: REACT_APP_BLOCKCHAIN_URL
          value: "https://polygonscan.com"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: fortis-frontend-service
  namespace: fortis
spec:
  selector:
    app: fortis-frontend
  ports:
  - port: 3000
    targetPort: 3000
  type: ClusterIP
```

---

## 🗄️ **Banco de Dados**

### **1. PostgreSQL com Replicação**
```yaml
# postgresql.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: postgres
  namespace: fortis
spec:
  serviceName: postgres-service
  replicas: 3
  selector:
    matchLabels:
      app: postgres
  template:
    metadata:
      labels:
        app: postgres
    spec:
      containers:
      - name: postgres
        image: postgres:15
        ports:
        - containerPort: 5432
        env:
        - name: POSTGRES_DB
          value: "fortis"
        - name: POSTGRES_USER
          value: "fortis"
        - name: POSTGRES_PASSWORD
          valueFrom:
            secretKeyRef:
              name: postgres-secret
              key: password
        - name: PGDATA
          value: "/var/lib/postgresql/data/pgdata"
        volumeMounts:
        - name: postgres-storage
          mountPath: /var/lib/postgresql/data
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
        livenessProbe:
          exec:
            command:
            - pg_isready
            - -U
            - fortis
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          exec:
            command:
            - pg_isready
            - -U
            - fortis
          initialDelaySeconds: 5
          periodSeconds: 5
  volumeClaimTemplates:
  - metadata:
      name: postgres-storage
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 100Gi
---
apiVersion: v1
kind: Service
metadata:
  name: postgres-service
  namespace: fortis
spec:
  selector:
    app: postgres
  ports:
  - port: 5432
    targetPort: 5432
  type: ClusterIP
```

### **2. Redis Cluster**
```yaml
# redis.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
  namespace: fortis
spec:
  replicas: 3
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:7-alpine
        ports:
        - containerPort: 6379
        command:
        - redis-server
        - --appendonly
        - "yes"
        - --cluster-enabled
        - "yes"
        - --cluster-config-file
        - /data/nodes.conf
        - --cluster-node-timeout
        - "5000"
        - --appendfsync
        - "everysec"
        - --save
        - "900 1"
        - --save
        - "300 10"
        - --save
        - "60 10000"
        volumeMounts:
        - name: redis-storage
          mountPath: /data
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          exec:
            command:
            - redis-cli
            - ping
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          exec:
            command:
            - redis-cli
            - ping
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: redis-storage
        persistentVolumeClaim:
          claimName: redis-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: redis-service
  namespace: fortis
spec:
  selector:
    app: redis
  ports:
  - port: 6379
    targetPort: 6379
  type: ClusterIP
```

---

## 📊 **Monitoramento e Observabilidade**

### **1. Prometheus**
```yaml
# prometheus.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: prometheus
  namespace: fortis
spec:
  replicas: 1
  selector:
    matchLabels:
      app: prometheus
  template:
    metadata:
      labels:
        app: prometheus
    spec:
      containers:
      - name: prometheus
        image: prom/prometheus:latest
        ports:
        - containerPort: 9090
        args:
        - --config.file=/etc/prometheus/prometheus.yml
        - --storage.tsdb.path=/prometheus/
        - --web.console.libraries=/etc/prometheus/console_libraries
        - --web.console.templates=/etc/prometheus/consoles
        - --storage.tsdb.retention.time=200h
        - --web.enable-lifecycle
        volumeMounts:
        - name: prometheus-config
          mountPath: /etc/prometheus
        - name: prometheus-storage
          mountPath: /prometheus
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
      volumes:
      - name: prometheus-config
        configMap:
          name: prometheus-config
      - name: prometheus-storage
        persistentVolumeClaim:
          claimName: prometheus-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: prometheus-service
  namespace: fortis
spec:
  selector:
    app: prometheus
  ports:
  - port: 9090
    targetPort: 9090
  type: ClusterIP
```

### **2. Grafana**
```yaml
# grafana.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: grafana
  namespace: fortis
spec:
  replicas: 1
  selector:
    matchLabels:
      app: grafana
  template:
    metadata:
      labels:
        app: grafana
    spec:
      containers:
      - name: grafana
        image: grafana/grafana:latest
        ports:
        - containerPort: 3000
        env:
        - name: GF_SECURITY_ADMIN_PASSWORD
          valueFrom:
            secretKeyRef:
              name: grafana-secret
              key: admin-password
        - name: GF_INSTALL_PLUGINS
          value: "grafana-piechart-panel,grafana-worldmap-panel"
        volumeMounts:
        - name: grafana-storage
          mountPath: /var/lib/grafana
        - name: grafana-datasources
          mountPath: /etc/grafana/provisioning/datasources
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /api/health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: grafana-storage
        persistentVolumeClaim:
          claimName: grafana-pvc
      - name: grafana-datasources
        configMap:
          name: grafana-datasources
---
apiVersion: v1
kind: Service
metadata:
  name: grafana-service
  namespace: fortis
spec:
  selector:
    app: grafana
  ports:
  - port: 3000
    targetPort: 3000
  type: LoadBalancer
```

---

## 🔒 **Segurança e Istio**

### **1. Istio Service Mesh**
```yaml
# istio-gateway.yaml
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
    - api.fortis.gov.br
    - admin.fortis.gov.br
    tls:
      httpsRedirect: true
  - port:
      number: 443
      name: https
      protocol: HTTPS
    hosts:
    - api.fortis.gov.br
    - admin.fortis.gov.br
    tls:
      mode: SIMPLE
      credentialName: fortis-tls-cert
---
apiVersion: networking.istio.io/v1alpha3
kind: VirtualService
metadata:
  name: fortis-vs
  namespace: fortis
spec:
  hosts:
  - api.fortis.gov.br
  - admin.fortis.gov.br
  gateways:
  - fortis-gateway
  http:
  - match:
    - uri:
        prefix: /api
    route:
    - destination:
        host: api-gateway-service
        port:
          number: 80
  - match:
    - uri:
        prefix: /
    route:
    - destination:
        host: fortis-frontend-service
        port:
          number: 3000
```

### **2. Security Policies**
```yaml
# security-policy.yaml
apiVersion: networking.istio.io/v1alpha3
kind: AuthorizationPolicy
metadata:
  name: fortis-authz
  namespace: fortis
spec:
  selector:
    matchLabels:
      app: fortis-backend
  rules:
  - from:
    - source:
        principals: ["cluster.local/ns/fortis/sa/api-gateway-sa"]
    to:
    - operation:
        methods: ["GET", "POST", "PUT", "DELETE"]
    when:
    - key: request.headers[authorization]
      values: ["Bearer *"]
---
apiVersion: security.istio.io/v1beta1
kind: PeerAuthentication
metadata:
  name: fortis-mtls
  namespace: fortis
spec:
  mtls:
    mode: STRICT
```

---

## 🚀 **CI/CD Pipeline**

### **1. GitHub Actions**
```yaml
# .github/workflows/deploy.yml
name: Deploy to Kubernetes

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v3
      
    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v2
      
    - name: Log in to Container Registry
      uses: docker/login-action@v2
      with:
        registry: ${{ env.REGISTRY }}
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}
        
    - name: Build and push Docker image
      uses: docker/build-push-action@v4
      with:
        context: .
        push: true
        tags: |
          ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest
          ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${{ github.sha }}
          
    - name: Deploy to Kubernetes
      uses: azure/k8s-deploy@v1
      with:
        manifests: |
          k8s/
        images: |
          ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${{ github.sha }}
        kubeconfig: ${{ secrets.KUBE_CONFIG }}
```

### **2. Helm Charts**
```yaml
# Chart.yaml
apiVersion: v2
name: fortis
description: Sistema de Votação Eletrônica Brasileiro
type: application
version: 1.0.0
appVersion: "1.0.0"

dependencies:
- name: postgresql
  version: 12.1.2
  repository: https://charts.bitnami.com/bitnami
- name: redis
  version: 17.3.7
  repository: https://charts.bitnami.com/bitnami
- name: prometheus
  version: 19.6.1
  repository: https://prometheus-community.github.io/helm-charts
- name: grafana
  version: 6.43.5
  repository: https://grafana.github.io/helm-charts
```

---

## 📈 **Scaling e Performance**

### **1. Horizontal Pod Autoscaler**
```yaml
# hpa.yaml
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
  maxReplicas: 50
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

### **2. Vertical Pod Autoscaler**
```yaml
# vpa.yaml
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
    - containerName: fortis-backend
      minAllowed:
        cpu: 100m
        memory: 128Mi
      maxAllowed:
        cpu: 2000m
        memory: 4Gi
```

---

## 🔧 **Scripts de Deploy**

### **1. Deploy Script**
```bash
#!/bin/bash
# deploy.sh

set -e

echo "🚀 Deploying FORTIS to Kubernetes..."

# Check if kubectl is configured
if ! kubectl cluster-info &> /dev/null; then
    echo "❌ kubectl not configured or cluster not accessible"
    exit 1
fi

# Create namespace
echo "📦 Creating namespace..."
kubectl apply -f k8s/namespace.yaml

# Apply secrets
echo "🔐 Applying secrets..."
kubectl apply -f k8s/secrets/

# Apply configmaps
echo "⚙️ Applying configmaps..."
kubectl apply -f k8s/configmaps/

# Deploy database
echo "🗄️ Deploying database..."
kubectl apply -f k8s/postgresql.yaml
kubectl apply -f k8s/redis.yaml

# Wait for database to be ready
echo "⏳ Waiting for database to be ready..."
kubectl wait --for=condition=ready pod -l app=postgres -n fortis --timeout=300s

# Deploy backend services
echo "🔧 Deploying backend services..."
kubectl apply -f k8s/backend-services.yaml

# Deploy frontend
echo "🎨 Deploying frontend..."
kubectl apply -f k8s/frontend.yaml

# Deploy monitoring
echo "📊 Deploying monitoring..."
kubectl apply -f k8s/prometheus.yaml
kubectl apply -f k8s/grafana.yaml

# Deploy Istio
echo "🔒 Deploying Istio..."
kubectl apply -f k8s/istio/

# Wait for all deployments
echo "⏳ Waiting for deployments to be ready..."
kubectl wait --for=condition=available deployment --all -n fortis --timeout=600s

echo "✅ Deploy completed successfully!"
echo "🌐 Access URLs:"
echo "   API: https://api.fortis.gov.br"
echo "   Admin: https://admin.fortis.gov.br"
echo "   Grafana: https://grafana.fortis.gov.br"
```

### **2. Health Check Script**
```bash
#!/bin/bash
# health-check.sh

echo "🔍 Checking FORTIS health..."

# Check pods
echo "📦 Pod status:"
kubectl get pods -n fortis

# Check services
echo "🌐 Service status:"
kubectl get services -n fortis

# Check ingress
echo "🚪 Ingress status:"
kubectl get ingress -n fortis

# Check logs for errors
echo "📋 Recent errors:"
kubectl logs -l app=fortis-backend -n fortis --tail=50 | grep -i error || echo "No errors found"

echo "✅ Health check completed!"
```

---

*Documentação de Infraestrutura FORTIS - Desenvolvida pelo DevOps Automator Agent*
