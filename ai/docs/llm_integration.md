# FORTIS - Integração com Modelos LLM Locais

## Visão Geral

O sistema FORTIS integra modelos de linguagem locais (LLM) para análise avançada de texto eleitoral, proporcionando insights inteligentes e relatórios automatizados sem comprometer a privacidade dos dados.

## Modelos Suportados

### Ollama (Recomendado)
- **Llama 3.2**: 3B, 7B, 13B parâmetros
- **Mistral**: 7B, 8x7B parâmetros
- **CodeLlama**: 7B, 13B, 34B parâmetros
- **Phi-3**: 3.8B, 14B parâmetros

### LM Studio
- **Qualquer modelo** compatível com OpenAI API
- **Configuração flexível** via endpoint local
- **Suporte a múltiplos modelos** simultâneos

## Instalação

### Linux/macOS
```bash
# Executa script de instalação
chmod +x ai/scripts/install_ollama.sh
./ai/scripts/install_ollama.sh

# Inicia serviço
./start_ollama.sh
```

### Windows
```powershell
# Executa script PowerShell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\ai\scripts\install_ollama.ps1

# Inicia serviço
.\start_ollama.ps1
```

## Configuração

### Arquivo de Configuração
```yaml
# ai/config/ollama_config.yaml
ollama:
  base_url: "http://localhost:11434"
  timeout: 30
  max_retries: 3

models:
  primary:
    name: "llama3.2:3b"
    temperature: 0.7
    max_tokens: 2048
```

### Variáveis de Ambiente
```bash
export OLLAMA_BASE_URL="http://localhost:11434"
export OLLAMA_DEFAULT_MODEL="llama3.2:3b"
export OLLAMA_TIMEOUT=30
```

## Uso da API

### Análise de Sentimento
```python
from ai.src.services.llm_service import get_llm_service

# Inicializa serviço
llm_service = await get_llm_service()

# Analisa sentimento
result = await llm_service.analyze_election_sentiment(
    "O sistema eleitoral está funcionando muito bem!"
)

print(f"Sentimento: {result['sentiment']}")
print(f"Confiança: {result['confidence']}")
```

### Extração de Entidades
```python
# Extrai entidades eleitorais
result = await llm_service.extract_election_insights(
    "Candidato João Silva do Partido Democrático, CPF: 123.456.789-01"
)

print(f"Insights: {result['insights']}")
```

### Geração de Relatórios
```python
# Gera relatório de eleição
data = {
    'total_voters': 1000,
    'participation_rate': 85.5,
    'candidates': ['João', 'Maria', 'Pedro']
}

result = await llm_service.generate_election_report(data)
print(f"Relatório: {result['report']}")
```

## Templates de Prompts

### Análise de Sentimento
```
Sistema: Você é um analista especializado em eleições brasileiras. 
Analise o sentimento do texto e responda em JSON.

Usuário: Analise o sentimento do seguinte texto sobre eleições:
{text}

Responda com: {"sentiment": "POSITIVO/NEGATIVO/NEUTRO", "confidence": 0.0-1.0, "reasoning": "explicação"}
```

### Classificação de Problemas
```
Sistema: Você é um especialista em sistemas eleitorais. 
Classifique problemas mencionados no texto.

Usuário: Classifique os problemas eleitorais no texto:
{text}

Responda com: {"categories": ["TÉCNICO", "LOGÍSTICO", "SEGURANÇA"], "severity": "BAIXA/MÉDIA/ALTA", "description": "resumo"}
```

## Análise Local vs LLM

### Análise Local (Fallback)
```python
from ai.src.models.nlp_models import ElectionTextAnalyzer

analyzer = ElectionTextAnalyzer()

# Análise de sentimento local
result = analyzer.analyze_sentiment("Texto eleitoral")
print(f"Sentimento: {result['sentiment']}")
print(f"Confiança: {result['confidence']}")
```

### Integração Híbrida
```python
# Tenta LLM primeiro, fallback para local
try:
    result = await llm_service.analyze_election_sentiment(text)
except Exception:
    result = analyzer.analyze_sentiment(text)
```

## Monitoramento e Logs

### Métricas de Uso
```python
# Obtém estatísticas
stats = llm_service.get_usage_statistics()
print(f"Total de requisições: {stats['total_requests']}")
print(f"Taxa de sucesso: {stats['success_rate']}")
print(f"Tokens utilizados: {stats['total_tokens']}")
```

### Logs de Debug
```python
import logging

# Configura logging
logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger('llm_service')

# Logs automáticos em todas as operações
```

## Cache e Performance

### Cache Inteligente
- **Hash de requisições**: Evita processamento duplicado
- **TTL configurável**: Cache expira automaticamente
- **Limpeza automática**: Remove entradas antigas

### Otimizações
- **Batch processing**: Múltiplas requisições simultâneas
- **Connection pooling**: Reutilização de conexões
- **Compressão**: Redução de tráfego de rede

## Segurança

### Validação de Entrada
```python
# Sanitiza entrada
def sanitize_input(text: str) -> str:
    # Remove caracteres perigosos
    # Limita tamanho
    # Valida formato
    return cleaned_text
```

### Validação de Resposta
```python
# Valida resposta do LLM
validation = utils.validate_llm_response(response, ['sentiment', 'confidence'])
if not validation['is_valid']:
    # Usa fallback local
    pass
```

## Troubleshooting

### Problemas Comuns

#### Ollama não inicia
```bash
# Verifica se está instalado
which ollama

# Verifica logs
tail -f ~/.ollama/ollama.log

# Reinicia serviço
pkill ollama
ollama serve
```

#### Modelo não encontrado
```bash
# Lista modelos instalados
ollama list

# Instala modelo
ollama pull llama3.2:3b

# Verifica espaço em disco
df -h
```

#### Erro de conexão
```python
# Testa conexão
from ai.src.utils.llm_utils import LLMUtils

utils = LLMUtils()
result = utils.test_llm_connection("http://localhost:11434")
print(f"Conexão: {result['success']}")
```

### Logs de Debug
```bash
# Habilita logs detalhados
export OLLAMA_DEBUG=1
export OLLAMA_VERBOSE=1

# Inicia com logs
ollama serve --verbose
```

## Exemplos Práticos

### Análise de Feedback de Eleitores
```python
async def analyze_voter_feedback(feedback_text: str):
    # Análise de sentimento
    sentiment = await llm_service.analyze_election_sentiment(feedback_text)
    
    # Classificação de problemas
    issues = await llm_service.classify_election_issues(feedback_text)
    
    # Extração de insights
    insights = await llm_service.extract_election_insights(feedback_text)
    
    return {
        'sentiment': sentiment,
        'issues': issues,
        'insights': insights
    }
```

### Geração de Relatório Diário
```python
async def generate_daily_report(election_data: dict):
    # Gera relatório usando LLM
    report = await llm_service.generate_election_report(election_data)
    
    # Salva em arquivo
    with open(f"reports/daily_{datetime.now().date()}.md", "w") as f:
        f.write(report['report'])
    
    return report
```

### Monitoramento em Tempo Real
```python
async def monitor_election_chat(chat_messages: list):
    results = []
    
    for message in chat_messages:
        # Analisa cada mensagem
        analysis = await analyze_voter_feedback(message['text'])
        
        # Adiciona timestamp
        analysis['timestamp'] = message['timestamp']
        analysis['user_id'] = message['user_id']
        
        results.append(analysis)
    
    return results
```

## Roadmap

### Fase 1: Integração Básica ✅
- [x] Suporte a Ollama
- [x] Análise de sentimento
- [x] Extração de entidades
- [x] Geração de relatórios

### Fase 2: Funcionalidades Avançadas
- [ ] Suporte a LM Studio
- [ ] Múltiplos modelos simultâneos
- [ ] Fine-tuning de modelos
- [ ] Análise de imagens

### Fase 3: Otimizações
- [ ] Cache distribuído
- [ ] Load balancing
- [ ] Auto-scaling
- [ ] Métricas avançadas

## Conclusão

A integração com modelos LLM locais proporciona ao FORTIS capacidades avançadas de análise de texto, mantendo a privacidade e segurança dos dados eleitorais. A arquitetura híbrida garante alta disponibilidade e confiabilidade do sistema.
