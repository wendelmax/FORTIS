#!/bin/bash
# FORTIS - Script de Instalação do Ollama
# Instala e configura Ollama para integração com LLM

set -e

echo "🚀 FORTIS - Instalação do Ollama"
echo "================================="

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

# Verifica se o sistema é suportado
check_system() {
    log "Verificando sistema operacional..."
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        log "Sistema Linux detectado"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        log "Sistema macOS detectado"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        OS="windows"
        log "Sistema Windows detectado"
    else
        error "Sistema operacional não suportado: $OSTYPE"
        exit 1
    fi
}

# Instala Ollama no Linux
install_ollama_linux() {
    log "Instalando Ollama no Linux..."
    
    # Verifica se curl está instalado
    if ! command -v curl &> /dev/null; then
        error "curl não está instalado. Instale primeiro: sudo apt-get install curl"
        exit 1
    fi
    
    # Baixa e instala Ollama
    curl -fsSL https://ollama.ai/install.sh | sh
    
    if [ $? -eq 0 ]; then
        success "Ollama instalado com sucesso no Linux"
    else
        error "Falha na instalação do Ollama"
        exit 1
    fi
}

# Instala Ollama no macOS
install_ollama_macos() {
    log "Instalando Ollama no macOS..."
    
    # Verifica se Homebrew está instalado
    if ! command -v brew &> /dev/null; then
        warning "Homebrew não encontrado. Instalando..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    
    # Instala Ollama via Homebrew
    brew install ollama
    
    if [ $? -eq 0 ]; then
        success "Ollama instalado com sucesso no macOS"
    else
        error "Falha na instalação do Ollama"
        exit 1
    fi
}

# Instala Ollama no Windows
install_ollama_windows() {
    log "Instalando Ollama no Windows..."
    
    # Verifica se PowerShell está disponível
    if ! command -v powershell &> /dev/null; then
        error "PowerShell não encontrado"
        exit 1
    fi
    
    # Baixa e instala Ollama
    powershell -Command "Invoke-WebRequest -Uri https://ollama.ai/download/windows -OutFile ollama-windows.zip"
    powershell -Command "Expand-Archive -Path ollama-windows.zip -DestinationPath C:\ollama"
    
    # Adiciona ao PATH
    echo 'C:\ollama' >> $env:PATH
    
    if [ $? -eq 0 ]; then
        success "Ollama instalado com sucesso no Windows"
    else
        error "Falha na instalação do Ollama"
        exit 1
    fi
}

# Instala modelos recomendados
install_models() {
    log "Instalando modelos recomendados..."
    
    # Lista de modelos para instalar
    MODELS=(
        "llama3.2:3b"
        "llama3.2:7b"
        "llama3.2:13b"
    )
    
    for model in "${MODELS[@]}"; do
        log "Instalando modelo: $model"
        ollama pull "$model"
        
        if [ $? -eq 0 ]; then
            success "Modelo $model instalado com sucesso"
        else
            warning "Falha na instalação do modelo $model"
        fi
    done
}

# Configura Ollama
configure_ollama() {
    log "Configurando Ollama..."
    
    # Cria diretório de configuração
    mkdir -p ~/.ollama
    
    # Cria arquivo de configuração
    cat > ~/.ollama/config.yaml << EOF
# Configuração do Ollama para FORTIS
server:
  host: "0.0.0.0"
  port: 11434
  timeout: 30

models:
  cache_dir: "~/.ollama/models"
  max_size: "10GB"

logging:
  level: "info"
  file: "~/.ollama/ollama.log"
EOF
    
    success "Configuração do Ollama criada"
}

# Inicia serviço Ollama
start_ollama() {
    log "Iniciando serviço Ollama..."
    
    # Para o serviço se estiver rodando
    pkill -f ollama || true
    
    # Inicia em background
    nohup ollama serve > ~/.ollama/ollama.log 2>&1 &
    
    # Aguarda o serviço iniciar
    sleep 5
    
    # Verifica se está rodando
    if curl -s http://localhost:11434/api/tags > /dev/null; then
        success "Serviço Ollama iniciado com sucesso"
    else
        error "Falha ao iniciar serviço Ollama"
        exit 1
    fi
}

# Testa instalação
test_installation() {
    log "Testando instalação..."
    
    # Testa conexão
    if curl -s http://localhost:11434/api/tags > /dev/null; then
        success "Conexão com Ollama estabelecida"
    else
        error "Falha na conexão com Ollama"
        exit 1
    fi
    
    # Lista modelos instalados
    log "Modelos instalados:"
    ollama list
    
    # Testa geração de texto
    log "Testando geração de texto..."
    echo "O que é um sistema eleitoral digital?" | ollama run llama3.2:3b
    
    success "Teste de instalação concluído"
}

# Cria script de inicialização
create_startup_script() {
    log "Criando script de inicialização..."
    
    cat > start_ollama.sh << 'EOF'
#!/bin/bash
# Script para iniciar Ollama para FORTIS

echo "🚀 Iniciando Ollama para FORTIS..."

# Verifica se Ollama está instalado
if ! command -v ollama &> /dev/null; then
    echo "❌ Ollama não está instalado"
    exit 1
fi

# Para instâncias existentes
pkill -f ollama || true

# Inicia Ollama
echo "📡 Iniciando servidor Ollama..."
nohup ollama serve > ~/.ollama/ollama.log 2>&1 &

# Aguarda inicialização
echo "⏳ Aguardando inicialização..."
sleep 5

# Verifica status
if curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "✅ Ollama iniciado com sucesso!"
    echo "🌐 Servidor rodando em: http://localhost:11434"
    echo "📊 Modelos disponíveis:"
    ollama list
else
    echo "❌ Falha ao iniciar Ollama"
    exit 1
fi
EOF
    
    chmod +x start_ollama.sh
    success "Script de inicialização criado: start_ollama.sh"
}

# Cria script de parada
create_stop_script() {
    log "Criando script de parada..."
    
    cat > stop_ollama.sh << 'EOF'
#!/bin/bash
# Script para parar Ollama

echo "🛑 Parando Ollama..."

# Para todas as instâncias
pkill -f ollama || true

echo "✅ Ollama parado com sucesso!"
EOF
    
    chmod +x stop_ollama.sh
    success "Script de parada criado: stop_ollama.sh"
}

# Função principal
main() {
    log "Iniciando instalação do Ollama para FORTIS..."
    
    # Verifica sistema
    check_system
    
    # Instala Ollama baseado no sistema
    case $OS in
        "linux")
            install_ollama_linux
            ;;
        "macos")
            install_ollama_macos
            ;;
        "windows")
            install_ollama_windows
            ;;
    esac
    
    # Configura Ollama
    configure_ollama
    
    # Inicia serviço
    start_ollama
    
    # Instala modelos
    install_models
    
    # Testa instalação
    test_installation
    
    # Cria scripts auxiliares
    create_startup_script
    create_stop_script
    
    success "Instalação do Ollama concluída com sucesso!"
    echo ""
    echo "📋 Próximos passos:"
    echo "1. Execute './start_ollama.sh' para iniciar o serviço"
    echo "2. Execute './stop_ollama.sh' para parar o serviço"
    echo "3. Acesse http://localhost:11434 para interface web"
    echo "4. Use 'ollama list' para ver modelos instalados"
    echo "5. Use 'ollama run llama3.2:3b' para testar"
    echo ""
    echo "🔗 Integração com FORTIS:"
    echo "O sistema FORTIS irá automaticamente detectar e usar o Ollama"
    echo "quando estiver rodando em http://localhost:11434"
}

# Executa função principal
main "$@"
