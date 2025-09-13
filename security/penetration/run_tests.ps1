# FORTIS Security Test Suite Runner - PowerShell
# Executa todos os testes de segurança do sistema FORTIS

param(
    [Parameter(Position=0)]
    [string]$Command = "all",
    
    [string]$ApiUrl = "http://localhost:8080",
    [string]$Domain = "fortis.gov.br",
    [string]$ContractPath = "blockchain/contracts/FortisVoting.sol",
    [string]$ConfigFile = "security/penetration/config/security_config.json",
    [switch]$Verbose,
    [switch]$Quiet,
    [switch]$Help
)

# Função para imprimir com cores
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

# Função para verificar dependências
function Test-Dependencies {
    Write-ColorOutput "🔍 Verificando dependências..." "Blue"
    
    # Verifica Python
    if (-not (Get-Command python -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "❌ Python não encontrado" "Red"
        exit 1
    }
    
    # Verifica pip
    if (-not (Get-Command pip -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "❌ pip não encontrado" "Red"
        exit 1
    }
    
    # Verifica Node.js
    if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "❌ Node.js não encontrado" "Red"
        exit 1
    }
    
    # Verifica npm
    if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "❌ npm não encontrado" "Red"
        exit 1
    }
    
    Write-ColorOutput "✅ Todas as dependências encontradas" "Green"
}

# Função para instalar dependências Python
function Install-PythonDependencies {
    Write-ColorOutput "📦 Instalando dependências Python..." "Blue"
    
    pip install -r security/penetration/requirements.txt
    
    Write-ColorOutput "✅ Dependências Python instaladas" "Green"
}

# Função para instalar dependências Node.js
function Install-NodeDependencies {
    Write-ColorOutput "📦 Instalando dependências Node.js..." "Blue"
    
    # Instala dependências do backend
    if (Test-Path "backend") {
        Set-Location backend
        cargo build --release
        Set-Location ..
    }
    
    # Instala dependências do blockchain
    if (Test-Path "blockchain") {
        Set-Location blockchain
        npm install
        Set-Location ..
    }
    
    # Instala dependências do frontend
    if (Test-Path "frontend") {
        Set-Location frontend
        npm install
        Set-Location ..
    }
    
    # Instala dependências do mobile
    if (Test-Path "mobile") {
        Set-Location mobile
        npm install
        Set-Location ..
    }
    
    Write-ColorOutput "✅ Dependências Node.js instaladas" "Green"
}

# Função para configurar ambiente
function Initialize-Environment {
    Write-ColorOutput "⚙️ Configurando ambiente..." "Blue"
    
    # Cria diretórios necessários
    New-Item -ItemType Directory -Force -Path "security/penetration/reports" | Out-Null
    New-Item -ItemType Directory -Force -Path "security/penetration/logs" | Out-Null
    New-Item -ItemType Directory -Force -Path "security/penetration/config" | Out-Null
    
    # Configura variáveis de ambiente
    $env:FORTIS_API_URL = $ApiUrl
    $env:FORTIS_DOMAIN = $Domain
    $env:FORTIS_CONTRACT_PATH = $ContractPath
    
    Write-ColorOutput "✅ Ambiente configurado" "Green"
}

# Função para executar testes de segurança
function Invoke-SecurityTests {
    Write-ColorOutput "🛡️ Executando testes de segurança..." "Blue"
    
    # Executa suite principal de testes
    python security/penetration/scripts/run_security_tests.py --config $ConfigFile
    
    Write-ColorOutput "✅ Testes de segurança concluídos" "Green"
}

# Função para executar testes rápidos
function Invoke-QuickTests {
    Write-ColorOutput "⚡ Executando testes rápidos..." "Blue"
    
    # Executa suite de testes rápidos
    python security/penetration/scripts/run_security_tests.py --config $ConfigFile --quick
    
    Write-ColorOutput "✅ Testes rápidos concluídos" "Green"
}

# Função para executar testes específicos
function Invoke-SpecificTests {
    param([string]$TestType)
    
    switch ($TestType) {
        "owasp" {
            Write-ColorOutput "🔍 Executando testes OWASP ZAP..." "Blue"
            python security/penetration/scripts/owasp_zap_scan.py $env:FORTIS_API_URL
        }
        "smart-contracts" {
            Write-ColorOutput "☸️ Executando testes de smart contracts..." "Blue"
            python security/penetration/scripts/smart_contract_security.py $env:FORTIS_CONTRACT_PATH
        }
        "crypto" {
            Write-ColorOutput "🔐 Executando testes de criptografia..." "Blue"
            python security/penetration/scripts/crypto_security_test.py $env:FORTIS_API_URL
        }
        "infrastructure" {
            Write-ColorOutput "🏗️ Executando testes de infraestrutura..." "Blue"
            python security/penetration/scripts/infrastructure_security.py $env:FORTIS_DOMAIN
        }
        "mobile" {
            Write-ColorOutput "📱 Executando testes de segurança mobile..." "Blue"
            python security/penetration/scripts/mobile_security.py mobile
        }
        default {
            Write-ColorOutput "❌ Tipo de teste inválido: $TestType" "Red"
            Write-ColorOutput "Tipos disponíveis: owasp, smart-contracts, crypto, infrastructure, mobile" "Yellow"
            exit 1
        }
    }
    
    Write-ColorOutput "✅ Testes $TestType concluídos" "Green"
}

# Função para gerar relatório consolidado
function New-ConsolidatedReport {
    Write-ColorOutput "📄 Gerando relatório consolidado..." "Blue"
    
    # Agrega todos os relatórios em um único arquivo
    python security/penetration/scripts/consolidate_reports.py
    
    Write-ColorOutput "✅ Relatório consolidado gerado" "Green"
}

# Função para limpar arquivos temporários
function Clear-TemporaryFiles {
    Write-ColorOutput "🧹 Limpando arquivos temporários..." "Blue"
    
    # Remove arquivos temporários
    if (Test-Path "security/penetration/temp/") {
        Remove-Item -Recurse -Force "security/penetration/temp/"
    }
    
    if (Test-Path "security/penetration/logs/") {
        Get-ChildItem "security/penetration/logs/*.tmp" | Remove-Item -Force
    }
    
    Write-ColorOutput "✅ Limpeza concluída" "Green"
}

# Função para mostrar ajuda
function Show-Help {
    Write-Host "FORTIS Security Test Suite Runner - PowerShell"
    Write-Host ""
    Write-Host "Uso: .\run_tests.ps1 [COMANDO] [OPÇÕES]"
    Write-Host ""
    Write-Host "Comandos:"
    Write-Host "  all                 Executa todos os testes de segurança"
    Write-Host "  quick              Executa apenas testes rápidos"
    Write-Host "  owasp              Executa apenas testes OWASP ZAP"
    Write-Host "  smart-contracts    Executa apenas testes de smart contracts"
    Write-Host "  crypto             Executa apenas testes de criptografia"
    Write-Host "  infrastructure     Executa apenas testes de infraestrutura"
    Write-Host "  mobile             Executa apenas testes de segurança mobile"
    Write-Host "  setup              Configura ambiente e instala dependências"
    Write-Host "  clean              Limpa arquivos temporários"
    Write-Host "  help               Mostra esta ajuda"
    Write-Host ""
    Write-Host "Opções:"
    Write-Host "  -ApiUrl URL        URL da API (padrão: http://localhost:8080)"
    Write-Host "  -Domain DOMAIN     Domínio alvo (padrão: fortis.gov.br)"
    Write-Host "  -ContractPath PATH Caminho do contrato (padrão: blockchain/contracts/FortisVoting.sol)"
    Write-Host "  -ConfigFile FILE   Arquivo de configuração personalizado"
    Write-Host "  -Verbose           Modo verboso"
    Write-Host "  -Quiet             Modo silencioso"
    Write-Host "  -Help              Mostra esta ajuda"
    Write-Host ""
    Write-Host "Exemplos:"
    Write-Host "  .\run_tests.ps1 setup                    # Configura ambiente"
    Write-Host "  .\run_tests.ps1 all                      # Executa todos os testes"
    Write-Host "  .\run_tests.ps1 quick                    # Executa testes rápidos"
    Write-Host "  .\run_tests.ps1 owasp                    # Executa apenas testes OWASP"
    Write-Host "  .\run_tests.ps1 -ApiUrl http://api.fortis.gov.br all  # Executa com API personalizada"
}

# Função principal
function Main {
    if ($Help) {
        Show-Help
        return
    }
    
    # Configura modo verboso/silencioso
    if ($Verbose) {
        $VerbosePreference = "Continue"
    }
    
    if ($Quiet) {
        $VerbosePreference = "SilentlyContinue"
        $WarningPreference = "SilentlyContinue"
    }
    
    # Executa comando
    switch ($Command) {
        "setup" {
            Test-Dependencies
            Install-PythonDependencies
            Install-NodeDependencies
            Initialize-Environment
        }
        "all" {
            Test-Dependencies
            Initialize-Environment
            Invoke-SecurityTests
            New-ConsolidatedReport
            Clear-TemporaryFiles
        }
        "quick" {
            Test-Dependencies
            Initialize-Environment
            Invoke-QuickTests
            Clear-TemporaryFiles
        }
        "owasp" {
            Test-Dependencies
            Initialize-Environment
            Invoke-SpecificTests "owasp"
            Clear-TemporaryFiles
        }
        "smart-contracts" {
            Test-Dependencies
            Initialize-Environment
            Invoke-SpecificTests "smart-contracts"
            Clear-TemporaryFiles
        }
        "crypto" {
            Test-Dependencies
            Initialize-Environment
            Invoke-SpecificTests "crypto"
            Clear-TemporaryFiles
        }
        "infrastructure" {
            Test-Dependencies
            Initialize-Environment
            Invoke-SpecificTests "infrastructure"
            Clear-TemporaryFiles
        }
        "mobile" {
            Test-Dependencies
            Initialize-Environment
            Invoke-SpecificTests "mobile"
            Clear-TemporaryFiles
        }
        "clean" {
            Clear-TemporaryFiles
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
}

# Executa função principal
Main
