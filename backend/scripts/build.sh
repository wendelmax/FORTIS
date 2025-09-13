#!/bin/bash
# FORTIS Backend - Script de Build
# Script para compilar o backend em diferentes ambientes

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

# Verifica se Rust está instalado
check_rust() {
    log "Verificando instalação do Rust..."
    
    if ! command -v rustc &> /dev/null; then
        error "Rust não está instalado. Instale primeiro: https://rustup.rs/"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        error "Cargo não está instalado. Instale primeiro: https://rustup.rs/"
        exit 1
    fi
    
    success "Rust e Cargo encontrados"
}

# Verifica dependências do sistema
check_dependencies() {
    log "Verificando dependências do sistema..."
    
    # Verifica PostgreSQL
    if ! command -v psql &> /dev/null; then
        warning "PostgreSQL não encontrado. Certifique-se de que está instalado."
    fi
    
    # Verifica Redis
    if ! command -v redis-cli &> /dev/null; then
        warning "Redis não encontrado. Certifique-se de que está instalado."
    fi
    
    # Verifica OpenSSL
    if ! command -v openssl &> /dev/null; then
        error "OpenSSL não encontrado. Instale primeiro."
        exit 1
    fi
    
    success "Dependências verificadas"
}

# Limpa build anterior
clean_build() {
    log "Limpando build anterior..."
    
    cargo clean
    
    success "Build anterior limpo"
}

# Executa testes
run_tests() {
    log "Executando testes..."
    
    if cargo test --all-features; then
        success "Todos os testes passaram"
    else
        error "Alguns testes falharam"
        exit 1
    fi
}

# Executa linting
run_lint() {
    log "Executando linting..."
    
    # Instala clippy se não estiver instalado
    if ! command -v cargo-clippy &> /dev/null; then
        log "Instalando clippy..."
        rustup component add clippy
    fi
    
    if cargo clippy --all-features -- -D warnings; then
        success "Linting passou"
    else
        error "Linting falhou"
        exit 1
    fi
}

# Executa formatação
run_format() {
    log "Verificando formatação..."
    
    # Instala rustfmt se não estiver instalado
    if ! command -v cargo-fmt &> /dev/null; then
        log "Instalando rustfmt..."
        rustup component add rustfmt
    fi
    
    if cargo fmt -- --check; then
        success "Formatação está correta"
    else
        warning "Formatação incorreta. Execute 'cargo fmt' para corrigir"
    fi
}

# Compila para desenvolvimento
build_dev() {
    log "Compilando para desenvolvimento..."
    
    if cargo build --all-features; then
        success "Build de desenvolvimento concluído"
    else
        error "Build de desenvolvimento falhou"
        exit 1
    fi
}

# Compila para produção
build_release() {
    log "Compilando para produção..."
    
    if cargo build --release --all-features; then
        success "Build de produção concluído"
    else
        error "Build de produção falhou"
        exit 1
    fi
}

# Gera documentação
generate_docs() {
    log "Gerando documentação..."
    
    if cargo doc --all-features --no-deps --open; then
        success "Documentação gerada"
    else
        error "Falha na geração de documentação"
        exit 1
    fi
}

# Executa análise de segurança
run_security_audit() {
    log "Executando análise de segurança..."
    
    # Instala cargo-audit se não estiver instalado
    if ! command -v cargo-audit &> /dev/null; then
        log "Instalando cargo-audit..."
        cargo install cargo-audit
    fi
    
    if cargo audit; then
        success "Análise de segurança passou"
    else
        error "Vulnerabilidades de segurança encontradas"
        exit 1
    fi
}

# Executa análise de dependências
run_dependency_audit() {
    log "Executando análise de dependências..."
    
    # Instala cargo-outdated se não estiver instalado
    if ! command -v cargo-outdated &> /dev/null; then
        log "Instalando cargo-outdated..."
        cargo install cargo-outdated
    fi
    
    if cargo outdated; then
        success "Análise de dependências concluída"
    else
        warning "Dependências desatualizadas encontradas"
    fi
}

# Gera relatório de cobertura
generate_coverage() {
    log "Gerando relatório de cobertura..."
    
    # Instala cargo-tarpaulin se não estiver instalado
    if ! command -v cargo-tarpaulin &> /dev/null; then
        log "Instalando cargo-tarpaulin..."
        cargo install cargo-tarpaulin
    fi
    
    if cargo tarpaulin --out Html --output-dir coverage; then
        success "Relatório de cobertura gerado em coverage/"
    else
        error "Falha na geração de cobertura"
        exit 1
    fi
}

# Cria arquivo de configuração
create_config() {
    log "Criando arquivo de configuração..."
    
    if [ ! -f ".env" ]; then
        cp env.example .env
        success "Arquivo .env criado a partir do env.example"
        warning "Edite o arquivo .env com suas configurações"
    else
        log "Arquivo .env já existe"
    fi
}

# Executa migrações do banco
run_migrations() {
    log "Executando migrações do banco de dados..."
    
    # Verifica se o banco está acessível
    if ! psql "$DATABASE_URL" -c "SELECT 1;" &> /dev/null; then
        error "Não foi possível conectar ao banco de dados"
        error "Verifique se o PostgreSQL está rodando e a DATABASE_URL está correta"
        exit 1
    fi
    
    # Executa migrações
    if cargo run --bin migrate; then
        success "Migrações executadas com sucesso"
    else
        error "Falha na execução das migrações"
        exit 1
    fi
}

# Inicia o servidor
start_server() {
    log "Iniciando servidor..."
    
    if [ "$1" = "release" ]; then
        cargo run --release
    else
        cargo run
    fi
}

# Função principal
main() {
    echo -e "${GREEN}🚀 FORTIS Backend - Script de Build${NC}"
    echo "=================================="
    
    # Parse argumentos
    BUILD_TYPE="dev"
    RUN_TESTS=false
    RUN_LINT=false
    RUN_FORMAT=false
    RUN_SECURITY=false
    RUN_COVERAGE=false
    RUN_MIGRATIONS=false
    START_SERVER=false
    GENERATE_DOCS=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --release)
                BUILD_TYPE="release"
                shift
                ;;
            --test)
                RUN_TESTS=true
                shift
                ;;
            --lint)
                RUN_LINT=true
                shift
                ;;
            --format)
                RUN_FORMAT=true
                shift
                ;;
            --security)
                RUN_SECURITY=true
                shift
                ;;
            --coverage)
                RUN_COVERAGE=true
                shift
                ;;
            --migrate)
                RUN_MIGRATIONS=true
                shift
                ;;
            --start)
                START_SERVER=true
                shift
                ;;
            --docs)
                GENERATE_DOCS=true
                shift
                ;;
            --all)
                RUN_TESTS=true
                RUN_LINT=true
                RUN_FORMAT=true
                RUN_SECURITY=true
                RUN_COVERAGE=true
                shift
                ;;
            -h|--help)
                echo "Uso: $0 [opções]"
                echo ""
                echo "Opções:"
                echo "  --release     Compila em modo release"
                echo "  --test        Executa testes"
                echo "  --lint        Executa linting"
                echo "  --format      Verifica formatação"
                echo "  --security    Executa análise de segurança"
                echo "  --coverage    Gera relatório de cobertura"
                echo "  --migrate     Executa migrações do banco"
                echo "  --start       Inicia o servidor"
                echo "  --docs        Gera documentação"
                echo "  --all         Executa todas as verificações"
                echo "  -h, --help    Mostra esta ajuda"
                exit 0
                ;;
            *)
                error "Opção desconhecida: $1"
                exit 1
                ;;
        esac
    done
    
    # Executa verificações
    check_rust
    check_dependencies
    create_config
    
    # Executa migrações se solicitado
    if [ "$RUN_MIGRATIONS" = true ]; then
        run_migrations
    fi
    
    # Executa verificações de código
    if [ "$RUN_FORMAT" = true ]; then
        run_format
    fi
    
    if [ "$RUN_LINT" = true ]; then
        run_lint
    fi
    
    if [ "$RUN_SECURITY" = true ]; then
        run_security_audit
        run_dependency_audit
    fi
    
    if [ "$RUN_TESTS" = true ]; then
        run_tests
    fi
    
    if [ "$RUN_COVERAGE" = true ]; then
        generate_coverage
    fi
    
    # Gera documentação se solicitado
    if [ "$GENERATE_DOCS" = true ]; then
        generate_docs
    fi
    
    # Compila o projeto
    clean_build
    
    if [ "$BUILD_TYPE" = "release" ]; then
        build_release
    else
        build_dev
    fi
    
    # Inicia servidor se solicitado
    if [ "$START_SERVER" = true ]; then
        start_server "$BUILD_TYPE"
    fi
    
    success "Build concluído com sucesso!"
}

# Executa função principal
main "$@"
