#!/bin/bash

# FORTIS Security Test Suite Runner
# Executa todos os testes de segurança do sistema FORTIS

set -e

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Função para imprimir com cores
print_color() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Função para verificar dependências
check_dependencies() {
    print_color $BLUE "🔍 Verificando dependências..."
    
    # Verifica Python
    if ! command -v python3 &> /dev/null; then
        print_color $RED "❌ Python3 não encontrado"
        exit 1
    fi
    
    # Verifica pip
    if ! command -v pip3 &> /dev/null; then
        print_color $RED "❌ pip3 não encontrado"
        exit 1
    fi
    
    # Verifica Node.js
    if ! command -v node &> /dev/null; then
        print_color $RED "❌ Node.js não encontrado"
        exit 1
    fi
    
    # Verifica npm
    if ! command -v npm &> /dev/null; then
        print_color $RED "❌ npm não encontrado"
        exit 1
    fi
    
    print_color $GREEN "✅ Todas as dependências encontradas"
}

# Função para instalar dependências Python
install_python_deps() {
    print_color $BLUE "📦 Instalando dependências Python..."
    
    pip3 install -r security/penetration/requirements.txt
    
    print_color $GREEN "✅ Dependências Python instaladas"
}

# Função para instalar dependências Node.js
install_node_deps() {
    print_color $BLUE "📦 Instalando dependências Node.js..."
    
    # Instala dependências do backend
    if [ -d "backend" ]; then
        cd backend
        cargo build --release
        cd ..
    fi
    
    # Instala dependências do blockchain
    if [ -d "blockchain" ]; then
        cd blockchain
        npm install
        cd ..
    fi
    
    # Instala dependências do frontend
    if [ -d "frontend" ]; then
        cd frontend
        npm install
        cd ..
    fi
    
    # Instala dependências do mobile
    if [ -d "mobile" ]; then
        cd mobile
        npm install
        cd ..
    fi
    
    print_color $GREEN "✅ Dependências Node.js instaladas"
}

# Função para configurar ambiente
setup_environment() {
    print_color $BLUE "⚙️ Configurando ambiente..."
    
    # Cria diretórios necessários
    mkdir -p security/penetration/reports
    mkdir -p security/penetration/logs
    mkdir -p security/penetration/config
    
    # Configura variáveis de ambiente
    export FORTIS_API_URL=${FORTIS_API_URL:-"http://localhost:8080"}
    export FORTIS_DOMAIN=${FORTIS_DOMAIN:-"fortis.gov.br"}
    export FORTIS_CONTRACT_PATH=${FORTIS_CONTRACT_PATH:-"blockchain/contracts/FortisVoting.sol"}
    
    print_color $GREEN "✅ Ambiente configurado"
}

# Função para executar testes de segurança
run_security_tests() {
    print_color $BLUE "🛡️ Executando testes de segurança..."
    
    # Executa suite principal de testes
    python3 security/penetration/scripts/run_security_tests.py \
        --config security/penetration/config/security_config.json
    
    print_color $GREEN "✅ Testes de segurança concluídos"
}

# Função para executar testes rápidos
run_quick_tests() {
    print_color $BLUE "⚡ Executando testes rápidos..."
    
    # Executa suite de testes rápidos
    python3 security/penetration/scripts/run_security_tests.py \
        --config security/penetration/config/security_config.json \
        --quick
    
    print_color $GREEN "✅ Testes rápidos concluídos"
}

# Função para executar testes específicos
run_specific_tests() {
    local test_type=$1
    
    case $test_type in
        "owasp")
            print_color $BLUE "🔍 Executando testes OWASP ZAP..."
            python3 security/penetration/scripts/owasp_zap_scan.py $FORTIS_API_URL
            ;;
        "smart-contracts")
            print_color $BLUE "☸️ Executando testes de smart contracts..."
            python3 security/penetration/scripts/smart_contract_security.py $FORTIS_CONTRACT_PATH
            ;;
        "crypto")
            print_color $BLUE "🔐 Executando testes de criptografia..."
            python3 security/penetration/scripts/crypto_security_test.py $FORTIS_API_URL
            ;;
        "infrastructure")
            print_color $BLUE "🏗️ Executando testes de infraestrutura..."
            python3 security/penetration/scripts/infrastructure_security.py $FORTIS_DOMAIN
            ;;
        "mobile")
            print_color $BLUE "📱 Executando testes de segurança mobile..."
            python3 security/penetration/scripts/mobile_security.py mobile
            ;;
        *)
            print_color $RED "❌ Tipo de teste inválido: $test_type"
            print_color $YELLOW "Tipos disponíveis: owasp, smart-contracts, crypto, infrastructure, mobile"
            exit 1
            ;;
    esac
    
    print_color $GREEN "✅ Testes $test_type concluídos"
}

# Função para gerar relatório consolidado
generate_consolidated_report() {
    print_color $BLUE "📄 Gerando relatório consolidado..."
    
    # Agrega todos os relatórios em um único arquivo
    python3 security/penetration/scripts/consolidate_reports.py
    
    print_color $GREEN "✅ Relatório consolidado gerado"
}

# Função para limpar arquivos temporários
cleanup() {
    print_color $BLUE "🧹 Limpando arquivos temporários..."
    
    # Remove arquivos temporários
    rm -rf security/penetration/temp/
    rm -rf security/penetration/logs/*.tmp
    
    print_color $GREEN "✅ Limpeza concluída"
}

# Função para mostrar ajuda
show_help() {
    echo "FORTIS Security Test Suite Runner"
    echo ""
    echo "Uso: $0 [OPÇÕES] [COMANDO]"
    echo ""
    echo "Comandos:"
    echo "  all                 Executa todos os testes de segurança"
    echo "  quick              Executa apenas testes rápidos"
    echo "  owasp              Executa apenas testes OWASP ZAP"
    echo "  smart-contracts    Executa apenas testes de smart contracts"
    echo "  crypto             Executa apenas testes de criptografia"
    echo "  infrastructure     Executa apenas testes de infraestrutura"
    echo "  mobile             Executa apenas testes de segurança mobile"
    echo "  setup              Configura ambiente e instala dependências"
    echo "  clean              Limpa arquivos temporários"
    echo "  help               Mostra esta ajuda"
    echo ""
    echo "Opções:"
    echo "  --api-url URL      URL da API (padrão: http://localhost:8080)"
    echo "  --domain DOMAIN    Domínio alvo (padrão: fortis.gov.br)"
    echo "  --contract PATH    Caminho do contrato (padrão: blockchain/contracts/FortisVoting.sol)"
    echo "  --config FILE      Arquivo de configuração personalizado"
    echo "  --verbose          Modo verboso"
    echo "  --quiet            Modo silencioso"
    echo ""
    echo "Exemplos:"
    echo "  $0 setup                    # Configura ambiente"
    echo "  $0 all                      # Executa todos os testes"
    echo "  $0 quick                    # Executa testes rápidos"
    echo "  $0 owasp                    # Executa apenas testes OWASP"
    echo "  $0 --api-url http://api.fortis.gov.br all  # Executa com API personalizada"
}

# Função principal
main() {
    local command=${1:-"all"}
    local verbose=false
    local quiet=false
    local config_file=""
    
    # Processa argumentos
    while [[ $# -gt 0 ]]; do
        case $1 in
            --api-url)
                export FORTIS_API_URL="$2"
                shift 2
                ;;
            --domain)
                export FORTIS_DOMAIN="$2"
                shift 2
                ;;
            --contract)
                export FORTIS_CONTRACT_PATH="$2"
                shift 2
                ;;
            --config)
                config_file="$2"
                shift 2
                ;;
            --verbose)
                verbose=true
                shift
                ;;
            --quiet)
                quiet=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                command=$1
                shift
                ;;
        esac
    done
    
    # Configura modo verboso/silencioso
    if [ "$verbose" = true ]; then
        set -x
    fi
    
    if [ "$quiet" = true ]; then
        exec > /dev/null 2>&1
    fi
    
    # Executa comando
    case $command in
        "setup")
            check_dependencies
            install_python_deps
            install_node_deps
            setup_environment
            ;;
        "all")
            check_dependencies
            setup_environment
            run_security_tests
            generate_consolidated_report
            cleanup
            ;;
        "quick")
            check_dependencies
            setup_environment
            run_quick_tests
            cleanup
            ;;
        "owasp"|"smart-contracts"|"crypto"|"infrastructure"|"mobile")
            check_dependencies
            setup_environment
            run_specific_tests $command
            cleanup
            ;;
        "clean")
            cleanup
            ;;
        "help")
            show_help
            ;;
        *)
            print_color $RED "❌ Comando inválido: $command"
            show_help
            exit 1
            ;;
    esac
}

# Executa função principal
main "$@"
