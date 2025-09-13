#!/bin/bash

# FORTIS Kubernetes Monitoring Script
# Monitor the FORTIS voting system in Kubernetes

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

# Function to check cluster connectivity
check_cluster() {
    if ! kubectl cluster-info &> /dev/null; then
        print_color $RED "❌ Não foi possível conectar ao cluster Kubernetes"
        exit 1
    fi
}

# Function to show pod status
show_pod_status() {
    print_color $BLUE "📦 Status dos Pods:"
    echo ""
    
    kubectl get pods -n fortis -o wide
    echo ""
    
    # Show pod details
    local pods=$(kubectl get pods -n fortis --no-headers | awk '{print $1}')
    
    for pod in $pods; do
        local status=$(kubectl get pod $pod -n fortis -o jsonpath='{.status.phase}')
        local ready=$(kubectl get pod $pod -n fortis -o jsonpath='{.status.conditions[?(@.type=="Ready")].status}')
        local restarts=$(kubectl get pod $pod -n fortis -o jsonpath='{.status.containerStatuses[0].restartCount}')
        
        if [ "$status" = "Running" ] && [ "$ready" = "True" ]; then
            print_color $GREEN "✅ $pod: $status, Ready: $ready, Restarts: $restarts"
        else
            print_color $RED "❌ $pod: $status, Ready: $ready, Restarts: $restarts"
        fi
    done
}

# Function to show service status
show_service_status() {
    print_color $BLUE "🔗 Status dos Services:"
    echo ""
    
    kubectl get services -n fortis -o wide
    echo ""
    
    # Show service endpoints
    print_color $BLUE "📍 Endpoints:"
    kubectl get endpoints -n fortis
}

# Function to show ingress status
show_ingress_status() {
    print_color $BLUE "🌍 Status do Ingress:"
    echo ""
    
    kubectl get ingress -n fortis -o wide
    echo ""
    
    # Show ingress details
    local ingress_ip=$(kubectl get ingress fortis-ingress -n fortis -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "N/A")
    local ingress_hostname=$(kubectl get ingress fortis-ingress -n fortis -o jsonpath='{.status.loadBalancer.ingress[0].hostname}' 2>/dev/null || echo "N/A")
    
    print_color $GREEN "🌐 Ingress IP: $ingress_ip"
    print_color $GREEN "🌐 Ingress Hostname: $ingress_hostname"
}

# Function to show HPA status
show_hpa_status() {
    print_color $BLUE "📈 Status do HPA:"
    echo ""
    
    kubectl get hpa -n fortis -o wide
    echo ""
    
    # Show HPA details
    local hpas=$(kubectl get hpa -n fortis --no-headers | awk '{print $1}')
    
    for hpa in $hpas; do
        local current=$(kubectl get hpa $hpa -n fortis -o jsonpath='{.status.currentReplicas}')
        local desired=$(kubectl get hpa $hpa -n fortis -o jsonpath='{.status.desiredReplicas}')
        local min=$(kubectl get hpa $hpa -n fortis -o jsonpath='{.spec.minReplicas}')
        local max=$(kubectl get hpa $hpa -n fortis -o jsonpath='{.spec.maxReplicas}')
        local cpu=$(kubectl get hpa $hpa -n fortis -o jsonpath='{.status.currentCPUUtilizationPercentage}')
        
        print_color $GREEN "📊 $hpa: $current/$desired (min: $min, max: $max, CPU: ${cpu}%)"
    done
}

# Function to show resource usage
show_resource_usage() {
    print_color $BLUE "💻 Uso de Recursos:"
    echo ""
    
    kubectl top pods -n fortis --containers
    echo ""
    
    kubectl top nodes
}

# Function to show logs
show_logs() {
    local component=${1:-"all"}
    local lines=${2:-"100"}
    
    print_color $BLUE "📋 Logs do $component (últimas $lines linhas):"
    echo ""
    
    case $component in
        "backend")
            kubectl logs -l app=fortis-backend -n fortis --tail=$lines
            ;;
        "frontend")
            kubectl logs -l app=fortis-frontend -n fortis --tail=$lines
            ;;
        "nginx")
            kubectl logs -l app=fortis-nginx -n fortis --tail=$lines
            ;;
        "postgres")
            kubectl logs -l app=fortis-postgres -n fortis --tail=$lines
            ;;
        "redis")
            kubectl logs -l app=fortis-redis -n fortis --tail=$lines
            ;;
        "prometheus")
            kubectl logs -l app=fortis-prometheus -n fortis --tail=$lines
            ;;
        "grafana")
            kubectl logs -l app=fortis-grafana -n fortis --tail=$lines
            ;;
        "all")
            for app in fortis-backend fortis-frontend fortis-nginx fortis-postgres fortis-redis fortis-prometheus fortis-grafana; do
                print_color $YELLOW "=== $app ==="
                kubectl logs -l app=$app -n fortis --tail=50
                echo ""
            done
            ;;
        *)
            print_color $RED "❌ Componente inválido: $component"
            print_color $YELLOW "Componentes disponíveis: backend, frontend, nginx, postgres, redis, prometheus, grafana, all"
            exit 1
            ;;
    esac
}

# Function to show events
show_events() {
    print_color $BLUE "📅 Eventos Recentes:"
    echo ""
    
    kubectl get events -n fortis --sort-by='.lastTimestamp' | tail -20
}

# Function to show network policies
show_network_policies() {
    print_color $BLUE "🔒 Network Policies:"
    echo ""
    
    kubectl get networkpolicies -n fortis -o wide
}

# Function to show persistent volumes
show_persistent_volumes() {
    print_color $BLUE "💾 Persistent Volumes:"
    echo ""
    
    kubectl get pv
    echo ""
    
    kubectl get pvc -n fortis
}

# Function to show secrets
show_secrets() {
    print_color $BLUE "🔐 Secrets:"
    echo ""
    
    kubectl get secrets -n fortis
}

# Function to show configmaps
show_configmaps() {
    print_color $BLUE "⚙️ ConfigMaps:"
    echo ""
    
    kubectl get configmaps -n fortis
}

# Function to run health checks
run_health_checks() {
    print_color $BLUE "🏥 Executando Health Checks:"
    echo ""
    
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
    
    # Check resource usage
    local high_cpu_pods=$(kubectl top pods -n fortis --no-headers | awk '$2 > 80 {print $1}' | wc -l)
    if [ $high_cpu_pods -gt 0 ]; then
        print_color $YELLOW "⚠️ $high_cpu_pods pods com alto uso de CPU"
    fi
    
    local high_memory_pods=$(kubectl top pods -n fortis --no-headers | awk '$3 > 80 {print $1}' | wc -l)
    if [ $high_memory_pods -gt 0 ]; then
        print_color $YELLOW "⚠️ $high_memory_pods pods com alto uso de memória"
    fi
}

# Function to show monitoring URLs
show_monitoring_urls() {
    print_color $BLUE "📊 URLs de Monitoramento:"
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
    echo "  kubectl port-forward service/fortis-grafana 3000:3000 -n fortis"
    echo "  kubectl port-forward service/fortis-prometheus 9090:9090 -n fortis"
    echo "  kubectl port-forward service/fortis-backend 8080:8080 -n fortis"
}

# Function to show help
show_help() {
    echo "FORTIS Kubernetes Monitoring Script"
    echo ""
    echo "Uso: $0 [COMANDO] [OPÇÕES]"
    echo ""
    echo "Comandos:"
    echo "  status              Mostra status geral do sistema"
    echo "  pods                Mostra status dos pods"
    echo "  services            Mostra status dos services"
    echo "  ingress             Mostra status do ingress"
    echo "  hpa                 Mostra status do HPA"
    echo "  resources           Mostra uso de recursos"
    echo "  logs [component]    Mostra logs dos componentes"
    echo "  events              Mostra eventos recentes"
    echo "  network             Mostra network policies"
    echo "  storage             Mostra persistent volumes"
    echo "  secrets             Mostra secrets"
    echo "  configmaps          Mostra configmaps"
    echo "  health              Executa health checks"
    echo "  urls                Mostra URLs de monitoramento"
    echo "  help                Mostra esta ajuda"
    echo ""
    echo "Opções:"
    echo "  --lines N           Número de linhas de log (padrão: 100)"
    echo ""
    echo "Exemplos:"
    echo "  $0 status                    # Status geral"
    echo "  $0 pods                      # Status dos pods"
    echo "  $0 logs backend              # Logs do backend"
    echo "  $0 logs backend --lines 200 # Logs do backend (200 linhas)"
    echo "  $0 health                    # Health checks"
    echo "  $0 urls                      # URLs de monitoramento"
}

# Function to show complete status
show_complete_status() {
    print_color $BLUE "🚀 Status Completo do FORTIS:"
    echo ""
    
    show_pod_status
    echo ""
    show_service_status
    echo ""
    show_ingress_status
    echo ""
    show_hpa_status
    echo ""
    show_resource_usage
    echo ""
    run_health_checks
    echo ""
    show_monitoring_urls
}

# Main script logic
check_cluster

case ${1:-"status"} in
    "status")
        show_complete_status
        ;;
    "pods")
        show_pod_status
        ;;
    "services")
        show_service_status
        ;;
    "ingress")
        show_ingress_status
        ;;
    "hpa")
        show_hpa_status
        ;;
    "resources")
        show_resource_usage
        ;;
    "logs")
        show_logs $2 $3
        ;;
    "events")
        show_events
        ;;
    "network")
        show_network_policies
        ;;
    "storage")
        show_persistent_volumes
        ;;
    "secrets")
        show_secrets
        ;;
    "configmaps")
        show_configmaps
        ;;
    "health")
        run_health_checks
        ;;
    "urls")
        show_monitoring_urls
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
