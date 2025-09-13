#!/bin/bash

# FORTIS - Script de Deploy para Produção
# Este script automatiza o deploy completo do sistema FORTIS em produção

set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para logging
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

success() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] ✅${NC} $1"
}

warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] ⚠️${NC} $1"
}

error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ❌${NC} $1"
    exit 1
}

# Verificar se está rodando como root
if [[ $EUID -eq 0 ]]; then
   error "Este script não deve ser executado como root por questões de segurança"
fi

# Verificar dependências
check_dependencies() {
    log "Verificando dependências..."
    
    local deps=("kubectl" "helm" "docker" "git" "curl" "jq")
    local missing_deps=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing_deps+=("$dep")
        fi
    done
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        error "Dependências faltando: ${missing_deps[*]}"
    fi
    
    success "Todas as dependências estão instaladas"
}

# Configurar variáveis de ambiente
setup_environment() {
    log "Configurando variáveis de ambiente..."
    
    # Carregar configurações do arquivo .env
    if [ -f ".env.production" ]; then
        source .env.production
        success "Variáveis de ambiente carregadas do .env.production"
    else
        warning "Arquivo .env.production não encontrado, usando valores padrão"
        export NAMESPACE="fortis"
        export REGISTRY="your-registry.com"
        export TAG="latest"
        export ENVIRONMENT="production"
    fi
    
    # Validar variáveis obrigatórias
    local required_vars=("NAMESPACE" "REGISTRY" "TAG" "ENVIRONMENT")
    for var in "${required_vars[@]}"; do
        if [ -z "${!var}" ]; then
            error "Variável obrigatória não definida: $var"
        fi
    done
    
    success "Variáveis de ambiente configuradas"
}

# Construir e fazer push das imagens Docker
build_and_push_images() {
    log "Construindo e fazendo push das imagens Docker..."
    
    local services=("backend" "frontend" "mobile")
    
    for service in "${services[@]}"; do
        log "Construindo imagem para $service..."
        
        # Construir imagem
        docker build -t "$REGISTRY/fortis-$service:$TAG" "./$service" || error "Falha ao construir imagem $service"
        
        # Fazer push da imagem
        docker push "$REGISTRY/fortis-$service:$TAG" || error "Falha ao fazer push da imagem $service"
        
        success "Imagem $service construída e enviada com sucesso"
    done
}

# Configurar namespace Kubernetes
setup_namespace() {
    log "Configurando namespace Kubernetes..."
    
    # Criar namespace se não existir
    kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f - || warning "Namespace $NAMESPACE já existe"
    
    # Aplicar labels
    kubectl label namespace "$NAMESPACE" environment="$ENVIRONMENT" --overwrite
    
    success "Namespace $NAMESPACE configurado"
}

# Deploy do banco de dados
deploy_database() {
    log "Fazendo deploy do banco de dados..."
    
    # Aplicar ConfigMaps e Secrets
    kubectl apply -f kubernetes/configmaps.yaml -n "$NAMESPACE"
    kubectl apply -f kubernetes/secrets.yaml -n "$NAMESPACE"
    
    # Deploy PostgreSQL
    kubectl apply -f kubernetes/postgres.yaml -n "$NAMESPACE"
    
    # Deploy Redis
    kubectl apply -f kubernetes/redis.yaml -n "$NAMESPACE"
    
    # Aguardar pods ficarem prontos
    kubectl wait --for=condition=ready pod -l app=postgres -n "$NAMESPACE" --timeout=300s
    kubectl wait --for=condition=ready pod -l app=redis -n "$NAMESPACE" --timeout=300s
    
    success "Banco de dados deployado com sucesso"
}

# Deploy da aplicação
deploy_application() {
    log "Fazendo deploy da aplicação..."
    
    # Atualizar tags das imagens nos manifests
    sed -i "s|image: .*|image: $REGISTRY/fortis-backend:$TAG|g" kubernetes/backend.yaml
    sed -i "s|image: .*|image: $REGISTRY/fortis-frontend:$TAG|g" kubernetes/frontend.yaml
    
    # Deploy backend
    kubectl apply -f kubernetes/backend.yaml -n "$NAMESPACE"
    
    # Deploy frontend
    kubectl apply -f kubernetes/frontend.yaml -n "$NAMESPACE"
    
    # Deploy Nginx
    kubectl apply -f kubernetes/nginx.yaml -n "$NAMESPACE"
    
    # Aguardar pods ficarem prontos
    kubectl wait --for=condition=ready pod -l app=backend -n "$NAMESPACE" --timeout=300s
    kubectl wait --for=condition=ready pod -l app=frontend -n "$NAMESPACE" --timeout=300s
    kubectl wait --for=condition=ready pod -l app=nginx -n "$NAMESPACE" --timeout=300s
    
    success "Aplicação deployada com sucesso"
}

# Deploy do monitoramento
deploy_monitoring() {
    log "Fazendo deploy do monitoramento..."
    
    # Deploy Prometheus
    kubectl apply -f kubernetes/monitoring.yaml -n "$NAMESPACE"
    
    # Aguardar pods ficarem prontos
    kubectl wait --for=condition=ready pod -l app=prometheus -n "$NAMESPACE" --timeout=300s
    kubectl wait --for=condition=ready pod -l app=grafana -n "$NAMESPACE" --timeout=300s
    
    success "Monitoramento deployado com sucesso"
}

# Configurar Ingress
setup_ingress() {
    log "Configurando Ingress..."
    
    # Deploy Ingress
    kubectl apply -f kubernetes/ingress.yaml -n "$NAMESPACE"
    
    # Aguardar Ingress ficar disponível
    kubectl wait --for=condition=ready ingress fortis-ingress -n "$NAMESPACE" --timeout=300s
    
    success "Ingress configurado com sucesso"
}

# Executar migrações do banco
run_migrations() {
    log "Executando migrações do banco de dados..."
    
    # Obter pod do backend
    local backend_pod=$(kubectl get pods -l app=backend -n "$NAMESPACE" -o jsonpath='{.items[0].metadata.name}')
    
    if [ -z "$backend_pod" ]; then
        error "Pod do backend não encontrado"
    fi
    
    # Executar migrações
    kubectl exec "$backend_pod" -n "$NAMESPACE" -- cargo run --bin migrate || error "Falha ao executar migrações"
    
    success "Migrações executadas com sucesso"
}

# Verificar saúde da aplicação
health_check() {
    log "Verificando saúde da aplicação..."
    
    # Obter URL do Ingress
    local ingress_url=$(kubectl get ingress fortis-ingress -n "$NAMESPACE" -o jsonpath='{.status.loadBalancer.ingress[0].hostname}')
    
    if [ -z "$ingress_url" ]; then
        warning "Ingress não possui hostname, usando IP do LoadBalancer"
        ingress_url=$(kubectl get ingress fortis-ingress -n "$NAMESPACE" -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
    fi
    
    if [ -z "$ingress_url" ]; then
        error "Não foi possível obter URL do Ingress"
    fi
    
    # Aguardar aplicação ficar disponível
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if curl -f "http://$ingress_url/health" &> /dev/null; then
            success "Aplicação está saudável e respondendo"
            break
        fi
        
        if [ $attempt -eq $max_attempts ]; then
            error "Aplicação não está respondendo após $max_attempts tentativas"
        fi
        
        log "Tentativa $attempt/$max_attempts - Aguardando aplicação ficar disponível..."
        sleep 10
        ((attempt++))
    done
    
    # Mostrar URLs de acesso
    echo ""
    success "Deploy concluído com sucesso!"
    echo ""
    echo "URLs de acesso:"
    echo "  - API: http://$ingress_url/api/v1"
    echo "  - Frontend: http://$ingress_url"
    echo "  - Grafana: http://$ingress_url/grafana"
    echo "  - Prometheus: http://$ingress_url/prometheus"
    echo ""
}

# Rollback em caso de erro
rollback() {
    error "Erro durante o deploy, executando rollback..."
    
    # Deletar recursos criados
    kubectl delete -f kubernetes/ingress.yaml -n "$NAMESPACE" --ignore-not-found=true
    kubectl delete -f kubernetes/monitoring.yaml -n "$NAMESPACE" --ignore-not-found=true
    kubectl delete -f kubernetes/backend.yaml -n "$NAMESPACE" --ignore-not-found=true
    kubectl delete -f kubernetes/frontend.yaml -n "$NAMESPACE" --ignore-not-found=true
    kubectl delete -f kubernetes/nginx.yaml -n "$NAMESPACE" --ignore-not-found=true
    kubectl delete -f kubernetes/postgres.yaml -n "$NAMESPACE" --ignore-not-found=true
    kubectl delete -f kubernetes/redis.yaml -n "$NAMESPACE" --ignore-not-found=true
    
    error "Rollback concluído"
}

# Configurar trap para rollback em caso de erro
trap rollback ERR

# Função principal
main() {
    log "Iniciando deploy do FORTIS em produção..."
    
    check_dependencies
    setup_environment
    build_and_push_images
    setup_namespace
    deploy_database
    deploy_application
    deploy_monitoring
    setup_ingress
    run_migrations
    health_check
    
    success "Deploy do FORTIS concluído com sucesso!"
}

# Executar função principal
main "$@"
