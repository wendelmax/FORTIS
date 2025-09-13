# FORTIS - Script de Instalação do Ollama para Windows
# Instala e configura Ollama para integração com LLM

param(
    [switch]$SkipModels = $false,
    [switch]$SkipTest = $false
)

# Configuração de cores
$ErrorActionPreference = "Stop"

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Write-Log {
    param([string]$Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Write-ColorOutput "[$timestamp] $Message" "Cyan"
}

function Write-Success {
    param([string]$Message)
    Write-ColorOutput "✅ $Message" "Green"
}

function Write-Error {
    param([string]$Message)
    Write-ColorOutput "❌ $Message" "Red"
}

function Write-Warning {
    param([string]$Message)
    Write-ColorOutput "⚠️ $Message" "Yellow"
}

# Verifica se está rodando como administrador
function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Verifica pré-requisitos
function Test-Prerequisites {
    Write-Log "Verificando pré-requisitos..."
    
    # Verifica PowerShell versão
    if ($PSVersionTable.PSVersion.Major -lt 5) {
        Write-Error "PowerShell 5.0 ou superior é necessário"
        exit 1
    }
    
    # Verifica se está rodando como administrador
    if (-not (Test-Administrator)) {
        Write-Warning "Recomendado executar como administrador para instalação completa"
    }
    
    # Verifica conectividade
    try {
        $response = Invoke-WebRequest -Uri "https://ollama.ai" -UseBasicParsing -TimeoutSec 10
        Write-Success "Conectividade com ollama.ai verificada"
    }
    catch {
        Write-Error "Falha na conectividade com ollama.ai"
        exit 1
    }
}

# Baixa e instala Ollama
function Install-Ollama {
    Write-Log "Baixando e instalando Ollama..."
    
    $ollamaUrl = "https://ollama.ai/download/windows"
    $ollamaZip = "$env:TEMP\ollama-windows.zip"
    $ollamaDir = "C:\ollama"
    
    try {
        # Baixa Ollama
        Write-Log "Baixando Ollama..."
        Invoke-WebRequest -Uri $ollamaUrl -OutFile $ollamaZip -UseBasicParsing
        
        # Cria diretório de instalação
        if (Test-Path $ollamaDir) {
            Remove-Item $ollamaDir -Recurse -Force
        }
        New-Item -ItemType Directory -Path $ollamaDir -Force | Out-Null
        
        # Extrai arquivo
        Write-Log "Extraindo Ollama..."
        Expand-Archive -Path $ollamaZip -DestinationPath $ollamaDir -Force
        
        # Adiciona ao PATH
        $currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
        if ($currentPath -notlike "*$ollamaDir*") {
            Write-Log "Adicionando Ollama ao PATH..."
            [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$ollamaDir", "Machine")
            $env:PATH += ";$ollamaDir"
        }
        
        # Limpa arquivo temporário
        Remove-Item $ollamaZip -Force
        
        Write-Success "Ollama instalado com sucesso"
    }
    catch {
        Write-Error "Falha na instalação do Ollama: $($_.Exception.Message)"
        exit 1
    }
}

# Configura Ollama
function Set-OllamaConfig {
    Write-Log "Configurando Ollama..."
    
    $configDir = "$env:USERPROFILE\.ollama"
    $configFile = "$configDir\config.yaml"
    
    # Cria diretório de configuração
    if (-not (Test-Path $configDir)) {
        New-Item -ItemType Directory -Path $configDir -Force | Out-Null
    }
    
    # Cria arquivo de configuração
    $configContent = @"
# Configuração do Ollama para FORTIS
server:
  host: "0.0.0.0"
  port: 11434
  timeout: 30

models:
  cache_dir: "$env:USERPROFILE\.ollama\models"
  max_size: "10GB"

logging:
  level: "info"
  file: "$env:USERPROFILE\.ollama\ollama.log"
"@
    
    Set-Content -Path $configFile -Value $configContent -Encoding UTF8
    Write-Success "Configuração do Ollama criada"
}

# Inicia serviço Ollama
function Start-OllamaService {
    Write-Log "Iniciando serviço Ollama..."
    
    # Para instâncias existentes
    Get-Process -Name "ollama" -ErrorAction SilentlyContinue | Stop-Process -Force
    
    # Inicia Ollama em background
    $ollamaExe = "C:\ollama\ollama.exe"
    if (Test-Path $ollamaExe) {
        Start-Process -FilePath $ollamaExe -ArgumentList "serve" -WindowStyle Hidden
        
        # Aguarda inicialização
        Write-Log "Aguardando inicialização do serviço..."
        Start-Sleep -Seconds 10
        
        # Verifica se está rodando
        $maxAttempts = 30
        $attempt = 0
        do {
            try {
                $response = Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -UseBasicParsing -TimeoutSec 5
                if ($response.StatusCode -eq 200) {
                    Write-Success "Serviço Ollama iniciado com sucesso"
                    return
                }
            }
            catch {
                # Continua tentando
            }
            
            $attempt++
            Start-Sleep -Seconds 2
        } while ($attempt -lt $maxAttempts)
        
        Write-Error "Falha ao iniciar serviço Ollama"
        exit 1
    }
    else {
        Write-Error "Executável do Ollama não encontrado"
        exit 1
    }
}

# Instala modelos recomendados
function Install-Models {
    if ($SkipModels) {
        Write-Log "Pulando instalação de modelos (--SkipModels especificado)"
        return
    }
    
    Write-Log "Instalando modelos recomendados..."
    
    $models = @(
        "llama3.2:3b",
        "llama3.2:7b",
        "llama3.2:13b"
    )
    
    foreach ($model in $models) {
        Write-Log "Instalando modelo: $model"
        try {
            & "C:\ollama\ollama.exe" pull $model
            Write-Success "Modelo $model instalado com sucesso"
        }
        catch {
            Write-Warning "Falha na instalação do modelo $model"
        }
    }
}

# Testa instalação
function Test-Installation {
    if ($SkipTest) {
        Write-Log "Pulando teste de instalação (--SkipTest especificado)"
        return
    }
    
    Write-Log "Testando instalação..."
    
    # Testa conexão
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-Success "Conexão com Ollama estabelecida"
        }
        else {
            Write-Error "Falha na conexão com Ollama"
            exit 1
        }
    }
    catch {
        Write-Error "Falha na conexão com Ollama: $($_.Exception.Message)"
        exit 1
    }
    
    # Lista modelos instalados
    Write-Log "Modelos instalados:"
    & "C:\ollama\ollama.exe" list
    
    # Testa geração de texto
    Write-Log "Testando geração de texto..."
    $testPrompt = "O que é um sistema eleitoral digital?"
    $testPrompt | & "C:\ollama\ollama.exe" run llama3.2:3b
    
    Write-Success "Teste de instalação concluído"
}

# Cria scripts auxiliares
function New-AuxiliaryScripts {
    Write-Log "Criando scripts auxiliares..."
    
    # Script de inicialização
    $startScript = @"
# Script para iniciar Ollama para FORTIS
Write-Host "🚀 Iniciando Ollama para FORTIS..." -ForegroundColor Green

# Verifica se Ollama está instalado
if (-not (Test-Path "C:\ollama\ollama.exe")) {
    Write-Host "❌ Ollama não está instalado" -ForegroundColor Red
    exit 1
}

# Para instâncias existentes
Get-Process -Name "ollama" -ErrorAction SilentlyContinue | Stop-Process -Force

# Inicia Ollama
Write-Host "📡 Iniciando servidor Ollama..." -ForegroundColor Cyan
Start-Process -FilePath "C:\ollama\ollama.exe" -ArgumentList "serve" -WindowStyle Hidden

# Aguarda inicialização
Write-Host "⏳ Aguardando inicialização..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Verifica status
try {
    `$response = Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -UseBasicParsing -TimeoutSec 5
    if (`$response.StatusCode -eq 200) {
        Write-Host "✅ Ollama iniciado com sucesso!" -ForegroundColor Green
        Write-Host "🌐 Servidor rodando em: http://localhost:11434" -ForegroundColor Cyan
        Write-Host "📊 Modelos disponíveis:" -ForegroundColor Cyan
        & "C:\ollama\ollama.exe" list
    }
    else {
        Write-Host "❌ Falha ao iniciar Ollama" -ForegroundColor Red
        exit 1
    }
}
catch {
    Write-Host "❌ Falha ao iniciar Ollama" -ForegroundColor Red
    exit 1
}
"@
    
    Set-Content -Path "start_ollama.ps1" -Value $startScript -Encoding UTF8
    
    # Script de parada
    $stopScript = @"
# Script para parar Ollama
Write-Host "🛑 Parando Ollama..." -ForegroundColor Yellow

# Para todas as instâncias
Get-Process -Name "ollama" -ErrorAction SilentlyContinue | Stop-Process -Force

Write-Host "✅ Ollama parado com sucesso!" -ForegroundColor Green
"@
    
    Set-Content -Path "stop_ollama.ps1" -Value $stopScript -Encoding UTF8
    
    Write-Success "Scripts auxiliares criados"
}

# Função principal
function Main {
    Write-ColorOutput "🚀 FORTIS - Instalação do Ollama" "Green"
    Write-ColorOutput "=================================" "Green"
    
    try {
        # Verifica pré-requisitos
        Test-Prerequisites
        
        # Instala Ollama
        Install-Ollama
        
        # Configura Ollama
        Set-OllamaConfig
        
        # Inicia serviço
        Start-OllamaService
        
        # Instala modelos
        Install-Models
        
        # Testa instalação
        Test-Installation
        
        # Cria scripts auxiliares
        New-AuxiliaryScripts
        
        Write-Success "Instalação do Ollama concluída com sucesso!"
        Write-ColorOutput "" "White"
        Write-ColorOutput "📋 Próximos passos:" "Cyan"
        Write-ColorOutput "1. Execute '.\start_ollama.ps1' para iniciar o serviço" "White"
        Write-ColorOutput "2. Execute '.\stop_ollama.ps1' para parar o serviço" "White"
        Write-ColorOutput "3. Acesse http://localhost:11434 para interface web" "White"
        Write-ColorOutput "4. Use 'C:\ollama\ollama.exe list' para ver modelos instalados" "White"
        Write-ColorOutput "5. Use 'C:\ollama\ollama.exe run llama3.2:3b' para testar" "White"
        Write-ColorOutput "" "White"
        Write-ColorOutput "🔗 Integração com FORTIS:" "Cyan"
        Write-ColorOutput "O sistema FORTIS irá automaticamente detectar e usar o Ollama" "White"
        Write-ColorOutput "quando estiver rodando em http://localhost:11434" "White"
    }
    catch {
        Write-Error "Falha na instalação: $($_.Exception.Message)"
        exit 1
    }
}

# Executa função principal
Main
