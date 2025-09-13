# FORTIS Kubernetes Deployment Script - PowerShell
# Deploy the complete FORTIS voting system to Kubernetes

param(
    [Parameter(Position=0)]
    [string]$Command = "deploy",
    
    [string]$Component = "all",
    [int]$Lines = 100,
    [switch]$Help
)

# Function to print colored output
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    
    $colorMap = @{
        "Red" = "Red"
        "Green" = "Green"
        "Yellow" = "Yellow"
        "Blue" = "Blue"
        "Cyan" = "Cyan"
        "Magenta" = "Magenta"
        "White" = "White"
    }
    
    Write-Host $Message -ForegroundColor $colorMap[$Color]
}

# Function to check if kubectl is available
function Test-Kubectl {
    if (-not (Get-Command kubectl -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "❌ kubectl não encontrado. Instale o kubectl primeiro." "Red"
        exit 1
    }
    Write-ColorOutput "✅ kubectl encontrado" "Green"
}

# Function to check cluster connectivity
function Test-Cluster {
    Write-ColorOutput "🔍 Verificando conectividade com o cluster..." "Blue"
    
    if (-not (kubectl cluster-info 2>$null)) {
        Write-ColorOutput "❌ Não foi possível conectar ao cluster Kubernetes" "Red"
        exit 1
    }
    
    Write-ColorOutput "✅ Conectado ao cluster Kubernetes" "Green"
}

# Function to create namespace
function New-Namespace {
    Write-ColorOutput "📦 Criando namespace fortis..." "Blue"
    
    kubectl apply -f namespace.yaml
    
    Write-ColorOutput "✅ Namespace fortis criado" "Green"
}

# Function to deploy secrets
function Deploy-Secrets {
    Write-ColorOutput "🔐 Deploying secrets..." "Blue"
    
    kubectl apply -f secrets.yaml
    
    Write-ColorOutput "✅ Secrets deployed" "Green"
}

# Function to deploy configmaps
function Deploy-ConfigMaps {
    Write-ColorOutput "⚙️ Deploying configmaps..." "Blue"
    
    kubectl apply -f configmaps.yaml
    
    Write-ColorOutput "✅ ConfigMaps deployed" "Green"
}

# Function to deploy database
function Deploy-Database {
    Write-ColorOutput "🗄️ Deploying PostgreSQL database..." "Blue"
    
    kubectl apply -f postgres.yaml
    
    # Wait for database to be ready
    Write-ColorOutput "⏳ Aguardando PostgreSQL estar pronto..." "Yellow"
    kubectl wait --for=condition=ready pod -l app=fortis-postgres -n fortis --timeout=300s
    
    Write-ColorOutput "✅ PostgreSQL deployed and ready" "Green"
}

# Function to deploy cache
function Deploy-Cache {
    Write-ColorOutput "💾 Deploying Redis cache..." "Blue"
    
    kubectl apply -f redis.yaml
    
    # Wait for Redis to be ready
    Write-ColorOutput "⏳ Aguardando Redis estar pronto..." "Yellow"
    kubectl wait --for=condition=ready pod -l app=fortis-redis -n fortis --timeout=300s
    
    Write-ColorOutput "✅ Redis deployed and ready" "Green"
}

# Function to deploy backend
function Deploy-Backend {
    Write-ColorOutput "🚀 Deploying backend API..." "Blue"
    
    kubectl apply -f backend.yaml
    
    # Wait for backend to be ready
    Write-ColorOutput "⏳ Aguardando backend estar pronto..." "Yellow"
    kubectl wait --for=condition=ready pod -l app=fortis-backend -n fortis --timeout=300s
    
    Write-ColorOutput "✅ Backend deployed and ready" "Green"
}

# Function to deploy frontend
function Deploy-Frontend {
    Write-ColorOutput "🌐 Deploying frontend..." "Blue"
    
    kubectl apply -f frontend.yaml
    
    # Wait for frontend to be ready
    Write-ColorOutput "⏳ Aguardando frontend estar pronto..." "Yellow"
    kubectl wait --for=condition=ready pod -l app=fortis-frontend -n fortis --timeout=300s
    
    Write-ColorOutput "✅ Frontend deployed and ready" "Green"
}

# Function to deploy nginx
function Deploy-Nginx {
    Write-ColorOutput "🔀 Deploying Nginx proxy..." "Blue"
    
    kubectl apply -f nginx.yaml
    
    # Wait for nginx to be ready
    Write-ColorOutput "⏳ Aguardando Nginx estar pronto..." "Yellow"
    kubectl wait --for=condition=ready pod -l app=fortis-nginx -n fortis --timeout=300s
    
    Write-ColorOutput "✅ Nginx deployed and ready" "Green"
}

# Function to deploy monitoring
function Deploy-Monitoring {
    Write-ColorOutput "📊 Deploying monitoring stack..." "Blue"
    
    kubectl apply -f monitoring.yaml
    
    # Wait for monitoring to be ready
    Write-ColorOutput "⏳ Aguardando monitoring estar pronto..." "Yellow"
    kubectl wait --for=condition=ready pod -l app=fortis-prometheus -n fortis --timeout=300s
    kubectl wait --for=condition=ready pod -l app=fortis-grafana -n fortis --timeout=300s
    
    Write-ColorOutput "✅ Monitoring deployed and ready" "Green"
}

# Function to deploy ingress
function Deploy-Ingress {
    Write-ColorOutput "🌍 Deploying ingress..." "Blue"
    
    kubectl apply -f ingress.yaml
    
    Write-ColorOutput "✅ Ingress deployed" "Green"
}

# Function to show deployment status
function Show-Status {
    Write-ColorOutput "📊 Status do deployment:" "Blue"
    
    Write-Host ""
    Write-ColorOutput "📦 Pods:" "Yellow"
    kubectl get pods -n fortis
    
    Write-Host ""
    Write-ColorOutput "🔗 Services:" "Yellow"
    kubectl get services -n fortis
    
    Write-Host ""
    Write-ColorOutput "🌍 Ingress:" "Yellow"
    kubectl get ingress -n fortis
    
    Write-Host ""
    Write-ColorOutput "📈 HPA:" "Yellow"
    kubectl get hpa -n fortis
}

# Function to show access information
function Show-AccessInfo {
    Write-ColorOutput "🌐 Informações de acesso:" "Blue"
    
    Write-Host ""
    Write-ColorOutput "🔗 URLs de acesso:" "Green"
    Write-Host "  Frontend: https://fortis.gov.br"
    Write-Host "  API: https://api.fortis.gov.br"
    Write-Host "  Admin: https://admin.fortis.gov.br"
    Write-Host "  Monitor: https://monitor.fortis.gov.br"
    
    Write-Host ""
    Write-ColorOutput "🔑 Credenciais:" "Green"
    Write-Host "  Grafana Admin: admin / fortis-grafana-password"
    Write-Host "  Prometheus: http://monitor.fortis.gov.br:9090"
    
    Write-Host ""
    Write-ColorOutput "📊 Comandos úteis:" "Green"
    Write-Host "  kubectl get pods -n fortis"
    Write-Host "  kubectl logs -f deployment/fortis-backend -n fortis"
    Write-Host "  kubectl port-forward service/fortis-grafana 3000:3000 -n fortis"
}

# Function to run health checks
function Test-HealthChecks {
    Write-ColorOutput "🏥 Executando health checks..." "Blue"
    
    # Check if all pods are running
    $failedPods = kubectl get pods -n fortis --field-selector=status.phase!=Running --no-headers | Measure-Object | Select-Object -ExpandProperty Count
    
    if ($failedPods -gt 0) {
        Write-ColorOutput "❌ $failedPods pods não estão rodando" "Red"
        kubectl get pods -n fortis --field-selector=status.phase!=Running
        return $false
    }
    
    Write-ColorOutput "✅ Todos os pods estão rodando" "Green"
    
    # Check if services are available
    $services = kubectl get services -n fortis --no-headers | Measure-Object | Select-Object -ExpandProperty Count
    Write-ColorOutput "✅ $services services disponíveis" "Green"
    
    # Check if ingress is ready
    $ingressReady = kubectl get ingress fortis-ingress -n fortis -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>$null
    
    if ($ingressReady) {
        Write-ColorOutput "✅ Ingress configurado: $ingressReady" "Green"
    } else {
        Write-ColorOutput "⚠️ Ingress ainda não está pronto" "Yellow"
    }
    
    return $true
}

# Function to cleanup on failure
function Remove-OnFailure {
    Write-ColorOutput "❌ Deploy falhou. Executando cleanup..." "Red"
    
    kubectl delete namespace fortis --ignore-not-found=true
    
    Write-ColorOutput "🧹 Cleanup concluído" "Yellow"
}

# Function to show logs
function Show-Logs {
    param([string]$Component = "all", [int]$Lines = 100)
    
    Write-ColorOutput "📋 Logs do $Component (últimas $Lines linhas):" "Blue"
    Write-Host ""
    
    switch ($Component) {
        "backend" {
            kubectl logs -l app=fortis-backend -n fortis --tail=$Lines
        }
        "frontend" {
            kubectl logs -l app=fortis-frontend -n fortis --tail=$Lines
        }
        "nginx" {
            kubectl logs -l app=fortis-nginx -n fortis --tail=$Lines
        }
        "postgres" {
            kubectl logs -l app=fortis-postgres -n fortis --tail=$Lines
        }
        "redis" {
            kubectl logs -l app=fortis-redis -n fortis --tail=$Lines
        }
        "prometheus" {
            kubectl logs -l app=fortis-prometheus -n fortis --tail=$Lines
        }
        "grafana" {
            kubectl logs -l app=fortis-grafana -n fortis --tail=$Lines
        }
        "all" {
            $apps = @("fortis-backend", "fortis-frontend", "fortis-nginx", "fortis-postgres", "fortis-redis", "fortis-prometheus", "fortis-grafana")
            foreach ($app in $apps) {
                Write-ColorOutput "=== $app ===" "Yellow"
                kubectl logs -l app=$app -n fortis --tail=50
                Write-Host ""
            }
        }
        default {
            Write-ColorOutput "❌ Componente inválido: $Component" "Red"
            Write-ColorOutput "Componentes disponíveis: backend, frontend, nginx, postgres, redis, prometheus, grafana, all" "Yellow"
            exit 1
        }
    }
}

# Function to cleanup
function Remove-All {
    Write-ColorOutput "🧹 Removendo todos os recursos do FORTIS..." "Yellow"
    
    kubectl delete namespace fortis --ignore-not-found=true
    
    Write-ColorOutput "✅ Cleanup concluído" "Green"
}

# Function to show help
function Show-Help {
    Write-Host "FORTIS Kubernetes Deployment Script - PowerShell"
    Write-Host ""
    Write-Host "Uso: .\deploy.ps1 [COMANDO] [OPÇÕES]"
    Write-Host ""
    Write-Host "Comandos:"
    Write-Host "  deploy     Deploy completo do sistema FORTIS"
    Write-Host "  status     Mostra status do deployment"
    Write-Host "  logs       Mostra logs dos pods"
    Write-Host "  cleanup    Remove todos os recursos"
    Write-Host "  help       Mostra esta ajuda"
    Write-Host ""
    Write-Host "Opções:"
    Write-Host "  -Component COMPONENTE  Componente específico para logs"
    Write-Host "  -Lines N               Número de linhas de log (padrão: 100)"
    Write-Host ""
    Write-Host "Exemplos:"
    Write-Host "  .\deploy.ps1 deploy                    # Deploy completo"
    Write-Host "  .\deploy.ps1 status                    # Status do deployment"
    Write-Host "  .\deploy.ps1 logs -Component backend   # Logs do backend"
    Write-Host "  .\deploy.ps1 cleanup                   # Remove tudo"
}

# Main deployment function
function Deploy-All {
    Write-ColorOutput "🚀 Iniciando deploy do FORTIS no Kubernetes..." "Blue"
    
    # Check prerequisites
    Test-Kubectl
    Test-Cluster
    
    # Deploy components in order
    New-Namespace
    Deploy-Secrets
    Deploy-ConfigMaps
    Deploy-Database
    Deploy-Cache
    Deploy-Backend
    Deploy-Frontend
    Deploy-Nginx
    Deploy-Monitoring
    Deploy-Ingress
    
    # Show status and access info
    Show-Status
    Show-AccessInfo
    
    # Run health checks
    if (Test-HealthChecks) {
        Write-ColorOutput "🎉 Deploy do FORTIS concluído com sucesso!" "Green"
    } else {
        Write-ColorOutput "❌ Deploy falhou. Verifique os logs acima." "Red"
        Remove-OnFailure
        exit 1
    }
}

# Main script logic
if ($Help) {
    Show-Help
    return
}

switch ($Command) {
    "deploy" {
        Deploy-All
    }
    "status" {
        Show-Status
    }
    "logs" {
        Show-Logs -Component $Component -Lines $Lines
    }
    "cleanup" {
        Remove-All
    }
    "help" {
        Show-Help
    }
    default {
        Write-ColorOutput "❌ Comando inválido: $Command" "Red"
        Show-Help
        exit 1
    }
}
