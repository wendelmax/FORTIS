#!/bin/bash

# FORTIS Kubernetes Deployment Script
# Deploy the complete FORTIS voting system to Kubernetes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to check if kubectl is available
check_kubectl() {
    if ! command -v kubectl &> /dev/null; then
        print_color $RED "❌ kubectl não encontrado. Instale o kubectl primeiro."
        exit 1
    fi
    print_color $GREEN "✅ kubectl encontrado"
}

# Function to check if kustomize is available
check_kustomize() {
    if ! command -v kustomize &> /dev/null; then
        print_color $YELLOW "⚠️ kustomize não encontrado. Instalando..."
        # Install kustomize
        curl -s "https://raw.githubusercontent.com/kubernetes-sigs/kustomize/master/hack/install_kustomize.sh" | bash
        sudo mv kustomize /usr/local/bin/
    fi
    print_color $GREEN "✅ kustomize encontrado"
}

# Function to check cluster connectivity
check_cluster() {
    print_color $BLUE "🔍 Verificando conectividade com o cluster..."
    
    if ! kubectl cluster-info &> /dev/null; then
        print_color $RED "❌ Não foi possível conectar ao cluster Kubernetes"
        exit 1
    fi
    
    print_color $GREEN "✅ Conectado ao cluster Kubernetes"
}

# Function to create namespace
create_namespace() {
    print_color $BLUE "📦 Criando namespace fortis..."
    
    kubectl apply -f namespace.yaml
    
    print_color $GREEN "✅ Namespace fortis criado"
}

# Function to deploy secrets
deploy_secrets() {
    print_color $BLUE "🔐 Deploying secrets..."
    
    kubectl apply -f secrets.yaml
    
    print_color $GREEN "✅ Secrets deployed"
}

# Function to deploy configmaps
deploy_configmaps() {
    print_color $BLUE "⚙️ Deploying configmaps..."
    
    kubectl apply -f configmaps.yaml
    
    print_color $GREEN "✅ ConfigMaps deployed"
}

# Function to deploy database
deploy_database() {
    print_color $BLUE "🗄️ Deploying PostgreSQL database..."
    
    kubectl apply -f postgres.yaml
    
    # Wait for database to be ready
    print_color $YELLOW "⏳ Aguardando PostgreSQL estar pronto..."
    kubectl wait --for=condition=ready pod -l app=fortis-postgres -n fortis --timeout=300s
    
    print_color $GREEN "✅ PostgreSQL deployed and ready"
}

# Function to deploy cache
deploy_cache() {
    print_color $BLUE "💾 Deploying Redis cache..."
    
    kubectl apply -f redis.yaml
    
    # Wait for Redis to be ready
    print_color $YELLOW "⏳ Aguardando Redis estar pronto..."
    kubectl wait --for=condition=ready pod -l app=fortis-redis -n fortis --timeout=300s
    
    print_color $GREEN "✅ Redis deployed and ready"
}

# Function to deploy backend
deploy_backend() {
    print_color $BLUE "🚀 Deploying backend API..."
    
    kubectl apply -f backend.yaml
    
    # Wait for backend to be ready
    print_color $YELLOW "⏳ Aguardando backend estar pronto..."
    kubectl wait --for=condition=ready pod -l app=fortis-backend -n fortis --timeout=300s
    
    print_color $GREEN "✅ Backend deployed and ready"
}

# Function to deploy frontend
deploy_frontend() {
    print_color $BLUE "🌐 Deploying frontend..."
    
    kubectl apply -f frontend.yaml
    
    # Wait for frontend to be ready
    print_color $YELLOW "⏳ Aguardando frontend estar pronto..."
    kubectl wait --for=condition=ready pod -l app=fortis-frontend -n fortis --timeout=300s
    
    print_color $GREEN "✅ Frontend deployed and ready"
}

# Function to deploy nginx
deploy_nginx() {
    print_color $BLUE "🔀 Deploying Nginx proxy..."
    
    kubectl apply -f nginx.yaml
    
    # Wait for nginx to be ready
    print_color $YELLOW "⏳ Aguardando Nginx estar pronto..."
    kubectl wait --for=condition=ready pod -l app=fortis-nginx -n fortis --timeout=300s
    
    print_color $GREEN "✅ Nginx deployed and ready"
}

# Function to deploy monitoring
deploy_monitoring() {
    print_color $BLUE "📊 Deploying monitoring stack..."
    
    kubectl apply -f monitoring.yaml
    
    # Wait for monitoring to be ready
    print_color $YELLOW "⏳ Aguardando monitoring estar pronto..."
    kubectl wait --for=condition=ready pod -l app=fortis-prometheus -n fortis --timeout=300s
    kubectl wait --for=condition=ready pod -l app=fortis-grafana -n fortis --timeout=300s
    
    print_color $GREEN "✅ Monitoring deployed and ready"
}

# Function to deploy ingress
deploy_ingress() {
    print_color $BLUE "🌍 Deploying ingress..."
    
    kubectl apply -f ingress.yaml
    
    print_color $GREEN "✅ Ingress deployed"
}

# Function to show deployment status
show_status() {
    print_color $BLUE "📊 Status do deployment:"
    
    echo ""
    print_color $YELLOW "📦 Pods:"
    kubectl get pods -n fortis
    
    echo ""
    print_color $YELLOW "🔗 Services:"
    kubectl get services -n fortis
    
    echo ""
    print_color $YELLOW "🌍 Ingress:"
    kubectl get ingress -n fortis
    
    echo ""
    print_color $YELLOW "📈 HPA:"
    kubectl get hpa -n fortis
}

# Function to show access information
show_access_info() {
    print_color $BLUE "🌐 Informações de acesso:"
    
    echo ""
    print_color $GREEN "🔗 URLs de acesso:"
    echo "  Frontend: https://fortis.gov.br"
    echo "  API: https://api.fortis.gov.br"
    echo "  Admin: https://admin.fortis.gov.br"
    echo "  Monitor: https://monitor.fortis.gov.br"
    
    echo ""
    print_color $GREEN "🔑 Credenciais:"
    echo "  Grafana Admin: admin / fortis-grafana-password"
    echo "  Prometheus: http://monitor.fortis.gov.br:9090"
    
    echo ""
    print_color $GREEN "📊 Comandos úteis:"
    echo "  kubectl get pods -n fortis"
    echo "  kubectl logs -f deployment/fortis-backend -n fortis"
    echo "  kubectl port-forward service/fortis-grafana 3000:3000 -n fortis"
}

# Function to run health checks
run_health_checks() {
    print_color $BLUE "🏥 Executando health checks..."
    
    # Check if all pods are running
    local failed_pods=$(kubectl get pods -n fortis --field-selector=status.phase!=Running --no-headers | wc -l)
    
    if [ $failed_pods -gt 0 ]; then
        print_color $RED "❌ $failed_pods pods não estão rodando"
        kubectl get pods -n fortis --field-selector=status.phase!=Running
        return 1
    fi
    
    print_color $GREEN "✅ Todos os pods estão rodando"
    
    # Check if services are available
    local services=$(kubectl get services -n fortis --no-headers | wc -l)
    print_color $GREEN "✅ $services services disponíveis"
    
    # Check if ingress is ready
    local ingress_ready=$(kubectl get ingress fortis-ingress -n fortis -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "")
    
    if [ -n "$ingress_ready" ]; then
        print_color $GREEN "✅ Ingress configurado: $ingress_ready"
    else
        print_color $YELLOW "⚠️ Ingress ainda não está pronto"
    fi
}

# Function to cleanup on failure
cleanup_on_failure() {
    print_color $RED "❌ Deploy falhou. Executando cleanup..."
    
    kubectl delete namespace fortis --ignore-not-found=true
    
    print_color $YELLOW "🧹 Cleanup concluído"
}

# Main deployment function
deploy() {
    print_color $BLUE "🚀 Iniciando deploy do FORTIS no Kubernetes..."
    
    # Check prerequisites
    check_kubectl
    check_kustomize
    check_cluster
    
    # Deploy components in order
    create_namespace
    deploy_secrets
    deploy_configmaps
    deploy_database
    deploy_cache
    deploy_backend
    deploy_frontend
    deploy_nginx
    deploy_monitoring
    deploy_ingress
    
    # Show status and access info
    show_status
    show_access_info
    
    # Run health checks
    if run_health_checks; then
        print_color $GREEN "🎉 Deploy do FORTIS concluído com sucesso!"
    else
        print_color $RED "❌ Deploy falhou. Verifique os logs acima."
        cleanup_on_failure
        exit 1
    fi
}

# Function to show help
show_help() {
    echo "FORTIS Kubernetes Deployment Script"
    echo ""
    echo "Uso: $0 [COMANDO]"
    echo ""
    echo "Comandos:"
    echo "  deploy     Deploy completo do sistema FORTIS"
    echo "  status     Mostra status do deployment"
    echo "  logs       Mostra logs dos pods"
    echo "  cleanup    Remove todos os recursos"
    echo "  help       Mostra esta ajuda"
    echo ""
    echo "Exemplos:"
    echo "  $0 deploy                    # Deploy completo"
    echo "  $0 status                    # Status do deployment"
    echo "  $0 logs backend              # Logs do backend"
    echo "  $0 cleanup                   # Remove tudo"
}

# Function to show logs
show_logs() {
    local component=${1:-"all"}
    
    case $component in
        "backend")
            kubectl logs -f deployment/fortis-backend -n fortis
            ;;
        "frontend")
            kubectl logs -f deployment/fortis-frontend -n fortis
            ;;
        "nginx")
            kubectl logs -f deployment/fortis-nginx -n fortis
            ;;
        "postgres")
            kubectl logs -f statefulset/fortis-postgres -n fortis
            ;;
        "redis")
            kubectl logs -f statefulset/fortis-redis -n fortis
            ;;
        "monitoring")
            kubectl logs -f deployment/fortis-prometheus -n fortis
            kubectl logs -f deployment/fortis-grafana -n fortis
            ;;
        "all")
            kubectl logs -f -l app=fortis-backend -n fortis &
            kubectl logs -f -l app=fortis-frontend -n fortis &
            kubectl logs -f -l app=fortis-nginx -n fortis &
            wait
            ;;
        *)
            print_color $RED "❌ Componente inválido: $component"
            print_color $YELLOW "Componentes disponíveis: backend, frontend, nginx, postgres, redis, monitoring, all"
            exit 1
            ;;
    esac
}

# Function to cleanup
cleanup() {
    print_color $YELLOW "🧹 Removendo todos os recursos do FORTIS..."
    
    kubectl delete namespace fortis --ignore-not-found=true
    
    print_color $GREEN "✅ Cleanup concluído"
}

# Main script logic
case ${1:-"deploy"} in
    "deploy")
        deploy
        ;;
    "status")
        show_status
        ;;
    "logs")
        show_logs $2
        ;;
    "cleanup")
        cleanup
        ;;
    "help")
        show_help
        ;;
    *)
        print_color $RED "❌ Comando inválido: $1"
        show_help
        exit 1
        ;;
esac
