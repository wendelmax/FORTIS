#!/bin/bash

# FORTIS - Script de Monitoramento de Produção
# Este script monitora a saúde e performance do sistema FORTIS em produção

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
}

# Configurar variáveis de ambiente
setup_environment() {
    if [ -f ".env.production" ]; then
        source .env.production
    else
        export NAMESPACE="fortis"
        export INGRESS_URL="localhost"
    fi
}

# Verificar status dos pods
check_pods() {
    log "Verificando status dos pods..."
    
    local pods=$(kubectl get pods -n "$NAMESPACE" --no-headers | awk '{print $1 " " $3}')
    local unhealthy_pods=()
    
    while IFS= read -r line; do
        local pod_name=$(echo "$line" | awk '{print $1}')
        local status=$(echo "$line" | awk '{print $2}')
        
        if [ "$status" != "Running" ]; then
            unhealthy_pods+=("$pod_name:$status")
        fi
    done <<< "$pods"
    
    if [ ${#unhealthy_pods[@]} -eq 0 ]; then
        success "Todos os pods estão rodando"
    else
        error "Pods com problemas: ${unhealthy_pods[*]}"
        return 1
    fi
}

# Verificar recursos do cluster
check_resources() {
    log "Verificando recursos do cluster..."
    
    # Verificar uso de CPU
    local cpu_usage=$(kubectl top nodes --no-headers | awk '{sum+=$3} END {print sum}')
    if [ "$cpu_usage" -gt 80 ]; then
        warning "Uso de CPU alto: ${cpu_usage}%"
    else
        success "Uso de CPU normal: ${cpu_usage}%"
    fi
    
    # Verificar uso de memória
    local memory_usage=$(kubectl top nodes --no-headers | awk '{sum+=$5} END {print sum}')
    if [ "$memory_usage" -gt 80 ]; then
        warning "Uso de memória alto: ${memory_usage}%"
    else
        success "Uso de memória normal: ${memory_usage}%"
    fi
}

# Verificar conectividade da aplicação
check_connectivity() {
    log "Verificando conectividade da aplicação..."
    
    local endpoints=("/health" "/api/v1/health" "/api/v1/ready")
    local failed_endpoints=()
    
    for endpoint in "${endpoints[@]}"; do
        if ! curl -f "http://$INGRESS_URL$endpoint" &> /dev/null; then
            failed_endpoints+=("$endpoint")
        fi
    done
    
    if [ ${#failed_endpoints[@]} -eq 0 ]; then
        success "Todos os endpoints estão respondendo"
    else
        error "Endpoints com problemas: ${failed_endpoints[*]}"
        return 1
    fi
}

# Verificar logs de erro
check_error_logs() {
    log "Verificando logs de erro..."
    
    local error_count=0
    
    # Verificar logs do backend
    local backend_errors=$(kubectl logs -l app=backend -n "$NAMESPACE" --since=1h | grep -i "error\|exception\|panic" | wc -l)
    error_count=$((error_count + backend_errors))
    
    # Verificar logs do frontend
    local frontend_errors=$(kubectl logs -l app=frontend -n "$NAMESPACE" --since=1h | grep -i "error\|exception" | wc -l)
    error_count=$((error_count + frontend_errors))
    
    if [ "$error_count" -eq 0 ]; then
        success "Nenhum erro encontrado nos logs"
    else
        warning "Encontrados $error_count erros nos logs"
    fi
}

# Verificar métricas de performance
check_performance() {
    log "Verificando métricas de performance..."
    
    # Verificar tempo de resposta da API
    local response_time=$(curl -w "%{time_total}" -o /dev/null -s "http://$INGRESS_URL/api/v1/health")
    local response_time_ms=$(echo "$response_time * 1000" | bc)
    
    if (( $(echo "$response_time_ms > 1000" | bc -l) )); then
        warning "Tempo de resposta alto: ${response_time_ms}ms"
    else
        success "Tempo de resposta normal: ${response_time_ms}ms"
    fi
    
    # Verificar throughput
    local requests_per_second=$(kubectl get --raw /apis/metrics.k8s.io/v1beta1/pods | jq -r '.items[] | select(.metadata.labels.app=="backend") | .containers[0].usage.cpu' | head -1)
    if [ -n "$requests_per_second" ]; then
        success "Throughput: $requests_per_second"
    fi
}

# Verificar segurança
check_security() {
    log "Verificando aspectos de segurança..."
    
    # Verificar se os secrets estão configurados
    local secrets_count=$(kubectl get secrets -n "$NAMESPACE" --no-headers | wc -l)
    if [ "$secrets_count" -gt 0 ]; then
        success "Secrets configurados: $secrets_count"
    else
        warning "Nenhum secret encontrado"
    fi
    
    # Verificar se as imagens são assinadas
    local images=$(kubectl get pods -n "$NAMESPACE" -o jsonpath='{.items[*].spec.containers[*].image}')
    local unsigned_images=()
    
    for image in $images; do
        if ! docker trust inspect "$image" &> /dev/null; then
            unsigned_images+=("$image")
        fi
    done
    
    if [ ${#unsigned_images[@]} -eq 0 ]; then
        success "Todas as imagens estão assinadas"
    else
        warning "Imagens não assinadas: ${unsigned_images[*]}"
    fi
}

# Verificar backup
check_backup() {
    log "Verificando status do backup..."
    
    # Verificar se há backups recentes
    local backup_age=$(find /backups -name "fortis-*.sql" -mtime -1 2>/dev/null | wc -l)
    if [ "$backup_age" -gt 0 ]; then
        success "Backup recente encontrado"
    else
        warning "Nenhum backup recente encontrado"
    fi
}

# Gerar relatório
generate_report() {
    log "Gerando relatório de monitoramento..."
    
    local report_file="monitoring-report-$(date +%Y%m%d-%H%M%S).txt"
    
    {
        echo "FORTIS - Relatório de Monitoramento"
        echo "Data: $(date)"
        echo "Namespace: $NAMESPACE"
        echo "=================================="
        echo ""
        
        echo "Status dos Pods:"
        kubectl get pods -n "$NAMESPACE"
        echo ""
        
        echo "Uso de Recursos:"
        kubectl top nodes
        echo ""
        
        echo "Status dos Serviços:"
        kubectl get services -n "$NAMESPACE"
        echo ""
        
        echo "Status do Ingress:"
        kubectl get ingress -n "$NAMESPACE"
        echo ""
        
        echo "Eventos Recentes:"
        kubectl get events -n "$NAMESPACE" --sort-by='.lastTimestamp' | tail -10
        echo ""
        
    } > "$report_file"
    
    success "Relatório gerado: $report_file"
}

# Monitoramento contínuo
continuous_monitoring() {
    log "Iniciando monitoramento contínuo..."
    
    while true; do
        echo ""
        log "Executando verificação de saúde..."
        
        if check_pods && check_connectivity; then
            success "Sistema saudável"
        else
            error "Problemas detectados no sistema"
            # Enviar alerta (implementar conforme necessário)
        fi
        
        sleep 60
    done
}

# Função principal
main() {
    local mode=${1:-"single"}
    
    setup_environment
    
    case $mode in
        "single")
            log "Executando verificação única..."
            check_pods
            check_resources
            check_connectivity
            check_error_logs
            check_performance
            check_security
            check_backup
            generate_report
            ;;
        "continuous")
            continuous_monitoring
            ;;
        "pods")
            check_pods
            ;;
        "connectivity")
            check_connectivity
            ;;
        "performance")
            check_performance
            ;;
        "security")
            check_security
            ;;
        "report")
            generate_report
            ;;
        *)
            echo "Uso: $0 [single|continuous|pods|connectivity|performance|security|report]"
            exit 1
            ;;
    esac
}

# Executar função principal
main "$@"
