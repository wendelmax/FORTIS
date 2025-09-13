#!/bin/bash
# FORTIS Backend - Script de Deploy
# Script para deploy em diferentes ambientes

set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para log
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Configurações
ENVIRONMENT="development"
DOCKER_REGISTRY="fortis-registry.com"
IMAGE_NAME="fortis-backend"
VERSION="latest"
NAMESPACE="fortis"
REPLICAS=3

# Verifica se Docker está instalado
check_docker() {
    log "Verificando instalação do Docker..."
    
    if ! command -v docker &> /dev/null; then
        error "Docker não está instalado. Instale primeiro: https://docs.docker.com/get-docker/"
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        error "Docker Compose não está instalado. Instale primeiro: https://docs.docker.com/compose/install/"
        exit 1
    fi
    
    success "Docker e Docker Compose encontrados"
}

# Verifica se kubectl está instalado
check_kubectl() {
    log "Verificando instalação do kubectl..."
    
    if ! command -v kubectl &> /dev/null; then
        error "kubectl não está instalado. Instale primeiro: https://kubernetes.io/docs/tasks/tools/"
        exit 1
    fi
    
    success "kubectl encontrado"
}

# Verifica se Helm está instalado
check_helm() {
    log "Verificando instalação do Helm..."
    
    if ! command -v helm &> /dev/null; then
        error "Helm não está instalado. Instale primeiro: https://helm.sh/docs/intro/install/"
        exit 1
    fi
    
    success "Helm encontrado"
}

# Cria imagem Docker
build_docker_image() {
    log "Criando imagem Docker..."
    
    local tag="${DOCKER_REGISTRY}/${IMAGE_NAME}:${VERSION}"
    
    if docker build -t "$tag" .; then
        success "Imagem Docker criada: $tag"
    else
        error "Falha na criação da imagem Docker"
        exit 1
    fi
}

# Faz push da imagem para registry
push_docker_image() {
    log "Fazendo push da imagem para registry..."
    
    local tag="${DOCKER_REGISTRY}/${IMAGE_NAME}:${VERSION}"
    
    if docker push "$tag"; then
        success "Imagem enviada para registry: $tag"
    else
        error "Falha no push da imagem"
        exit 1
    fi
}

# Deploy com Docker Compose
deploy_docker_compose() {
    log "Fazendo deploy com Docker Compose..."
    
    # Para containers existentes
    docker-compose down || true
    
    # Inicia novos containers
    if docker-compose up -d; then
        success "Deploy com Docker Compose concluído"
    else
        error "Falha no deploy com Docker Compose"
        exit 1
    fi
}

# Deploy com Kubernetes
deploy_kubernetes() {
    log "Fazendo deploy com Kubernetes..."
    
    # Aplica namespace
    kubectl apply -f - <<EOF
apiVersion: v1
kind: Namespace
metadata:
  name: $NAMESPACE
EOF
    
    # Aplica ConfigMap
    kubectl apply -f - <<EOF
apiVersion: v1
kind: ConfigMap
metadata:
  name: fortis-backend-config
  namespace: $NAMESPACE
data:
  SERVER_HOST: "0.0.0.0"
  SERVER_PORT: "8080"
  DATABASE_URL: "postgresql://fortis:password@postgres:5432/fortis"
  REDIS_URL: "redis://redis:6379"
  JWT_SECRET: "fortis-super-secret-key"
  ENCRYPTION_KEY: "fortis-encryption-key-32-chars"
EOF
    
    # Aplica Secret
    kubectl apply -f - <<EOF
apiVersion: v1
kind: Secret
metadata:
  name: fortis-backend-secret
  namespace: $NAMESPACE
type: Opaque
data:
  jwt-secret: Zm9ydGlzLXN1cGVyLXNlY3JldC1rZXk=
  encryption-key: Zm9ydGlzLWVuY3J5cHRpb24ta2V5LTMyLWNoYXJz
  database-password: cGFzc3dvcmQ=
  redis-password: ""
EOF
    
    # Aplica Deployment
    kubectl apply -f - <<EOF
apiVersion: apps/v1
kind: Deployment
metadata:
  name: fortis-backend
  namespace: $NAMESPACE
spec:
  replicas: $REPLICAS
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
        image: ${DOCKER_REGISTRY}/${IMAGE_NAME}:${VERSION}
        ports:
        - containerPort: 8080
        env:
        - name: SERVER_HOST
          valueFrom:
            configMapKeyRef:
              name: fortis-backend-config
              key: SERVER_HOST
        - name: SERVER_PORT
          valueFrom:
            configMapKeyRef:
              name: fortis-backend-config
              key: SERVER_PORT
        - name: DATABASE_URL
          valueFrom:
            configMapKeyRef:
              name: fortis-backend-config
              key: DATABASE_URL
        - name: REDIS_URL
          valueFrom:
            configMapKeyRef:
              name: fortis-backend-config
              key: REDIS_URL
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: fortis-backend-secret
              key: jwt-secret
        - name: ENCRYPTION_KEY
          valueFrom:
            secretKeyRef:
              name: fortis-backend-secret
              key: encryption-key
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health/ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
EOF
    
    # Aplica Service
    kubectl apply -f - <<EOF
apiVersion: v1
kind: Service
metadata:
  name: fortis-backend-service
  namespace: $NAMESPACE
spec:
  selector:
    app: fortis-backend
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
EOF
    
    # Aplica Ingress
    kubectl apply -f - <<EOF
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: fortis-backend-ingress
  namespace: $NAMESPACE
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
    nginx.ingress.kubernetes.io/ssl-redirect: "false"
spec:
  rules:
  - host: fortis-backend.local
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: fortis-backend-service
            port:
              number: 80
EOF
    
    success "Deploy com Kubernetes concluído"
}

# Deploy com Helm
deploy_helm() {
    log "Fazendo deploy com Helm..."
    
    # Cria valores para Helm
    cat > values.yaml <<EOF
image:
  repository: ${DOCKER_REGISTRY}/${IMAGE_NAME}
  tag: ${VERSION}
  pullPolicy: IfNotPresent

replicaCount: ${REPLICAS}

service:
  type: ClusterIP
  port: 80
  targetPort: 8080

ingress:
  enabled: true
  className: nginx
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /
  hosts:
    - host: fortis-backend.local
      paths:
        - path: /
          pathType: Prefix
  tls: []

resources:
  limits:
    cpu: 500m
    memory: 512Mi
  requests:
    cpu: 250m
    memory: 256Mi

autoscaling:
  enabled: false
  minReplicas: 1
  maxReplicas: 10
  targetCPUUtilizationPercentage: 80

nodeSelector: {}

tolerations: []

affinity: {}

config:
  serverHost: "0.0.0.0"
  serverPort: 8080
  databaseUrl: "postgresql://fortis:password@postgres:5432/fortis"
  redisUrl: "redis://redis:6379"

secrets:
  jwtSecret: "fortis-super-secret-key"
  encryptionKey: "fortis-encryption-key-32-chars"
  databasePassword: "password"
  redisPassword: ""
EOF
    
    # Instala/atualiza com Helm
    if helm upgrade --install fortis-backend ./helm/fortis-backend \
        --namespace $NAMESPACE \
        --create-namespace \
        --values values.yaml; then
        success "Deploy com Helm concluído"
    else
        error "Falha no deploy com Helm"
        exit 1
    fi
    
    # Limpa arquivo temporário
    rm -f values.yaml
}

# Verifica status do deploy
check_deploy_status() {
    log "Verificando status do deploy..."
    
    if [ "$ENVIRONMENT" = "kubernetes" ] || [ "$ENVIRONMENT" = "helm" ]; then
        # Verifica pods
        kubectl get pods -n $NAMESPACE
        
        # Verifica serviços
        kubectl get services -n $NAMESPACE
        
        # Verifica ingress
        kubectl get ingress -n $NAMESPACE
        
        # Verifica logs
        kubectl logs -n $NAMESPACE -l app=fortis-backend --tail=50
    elif [ "$ENVIRONMENT" = "docker" ]; then
        # Verifica containers
        docker-compose ps
        
        # Verifica logs
        docker-compose logs --tail=50
    fi
    
    success "Status do deploy verificado"
}

# Executa testes de smoke
run_smoke_tests() {
    log "Executando testes de smoke..."
    
    local url="http://localhost:8080"
    
    # Testa health check
    if curl -f -s "$url/health" > /dev/null; then
        success "Health check passou"
    else
        error "Health check falhou"
        exit 1
    fi
    
    # Testa ready check
    if curl -f -s "$url/health/ready" > /dev/null; then
        success "Ready check passou"
    else
        error "Ready check falhou"
        exit 1
    fi
    
    # Testa API
    if curl -f -s "$url/api/v1/elections" > /dev/null; then
        success "API test passou"
    else
        error "API test falhou"
        exit 1
    fi
    
    success "Testes de smoke passaram"
}

# Rollback do deploy
rollback_deploy() {
    log "Fazendo rollback do deploy..."
    
    if [ "$ENVIRONMENT" = "kubernetes" ]; then
        kubectl rollout undo deployment/fortis-backend -n $NAMESPACE
    elif [ "$ENVIRONMENT" = "helm" ]; then
        helm rollback fortis-backend -n $NAMESPACE
    elif [ "$ENVIRONMENT" = "docker" ]; then
        docker-compose down
        docker-compose up -d --scale fortis-backend=0
    fi
    
    success "Rollback concluído"
}

# Limpa recursos
cleanup() {
    log "Limpando recursos..."
    
    if [ "$ENVIRONMENT" = "kubernetes" ]; then
        kubectl delete namespace $NAMESPACE --ignore-not-found=true
    elif [ "$ENVIRONMENT" = "helm" ]; then
        helm uninstall fortis-backend -n $NAMESPACE
        kubectl delete namespace $NAMESPACE --ignore-not-found=true
    elif [ "$ENVIRONMENT" = "docker" ]; then
        docker-compose down -v
    fi
    
    success "Recursos limpos"
}

# Função principal
main() {
    echo -e "${GREEN}🚀 FORTIS Backend - Script de Deploy${NC}"
    echo "====================================="
    
    # Parse argumentos
    while [[ $# -gt 0 ]]; do
        case $1 in
            --env)
                ENVIRONMENT="$2"
                shift 2
                ;;
            --registry)
                DOCKER_REGISTRY="$2"
                shift 2
                ;;
            --version)
                VERSION="$2"
                shift 2
                ;;
            --namespace)
                NAMESPACE="$2"
                shift 2
                ;;
            --replicas)
                REPLICAS="$2"
                shift 2
                ;;
            --smoke-tests)
                RUN_SMOKE_TESTS=true
                shift
                ;;
            --rollback)
                ROLLBACK=true
                shift
                ;;
            --cleanup)
                CLEANUP=true
                shift
                ;;
            -h|--help)
                echo "Uso: $0 [opções]"
                echo ""
                echo "Opções:"
                echo "  --env ENV        Ambiente (docker, kubernetes, helm) [default: development]"
                echo "  --registry URL   Registry Docker [default: fortis-registry.com]"
                echo "  --version VER    Versão da imagem [default: latest]"
                echo "  --namespace NS   Namespace Kubernetes [default: fortis]"
                echo "  --replicas N     Número de réplicas [default: 3]"
                echo "  --smoke-tests    Executa testes de smoke"
                echo "  --rollback       Faz rollback do deploy"
                echo "  --cleanup        Limpa recursos"
                echo "  -h, --help       Mostra esta ajuda"
                exit 0
                ;;
            *)
                error "Opção desconhecida: $1"
                exit 1
                ;;
        esac
    done
    
    # Executa rollback se solicitado
    if [ "$ROLLBACK" = true ]; then
        rollback_deploy
        exit 0
    fi
    
    # Executa cleanup se solicitado
    if [ "$CLEANUP" = true ]; then
        cleanup
        exit 0
    fi
    
    # Verifica dependências
    check_docker
    
    if [ "$ENVIRONMENT" = "kubernetes" ] || [ "$ENVIRONMENT" = "helm" ]; then
        check_kubectl
    fi
    
    if [ "$ENVIRONMENT" = "helm" ]; then
        check_helm
    fi
    
    # Cria imagem Docker
    build_docker_image
    
    # Faz push da imagem se não for desenvolvimento local
    if [ "$ENVIRONMENT" != "development" ]; then
        push_docker_image
    fi
    
    # Executa deploy baseado no ambiente
    case $ENVIRONMENT in
        "docker")
            deploy_docker_compose
            ;;
        "kubernetes")
            deploy_kubernetes
            ;;
        "helm")
            deploy_helm
            ;;
        *)
            error "Ambiente não suportado: $ENVIRONMENT"
            exit 1
            ;;
    esac
    
    # Verifica status do deploy
    check_deploy_status
    
    # Executa testes de smoke se solicitado
    if [ "$RUN_SMOKE_TESTS" = true ]; then
        run_smoke_tests
    fi
    
    success "Deploy concluído com sucesso!"
}

# Executa função principal
main "$@"
