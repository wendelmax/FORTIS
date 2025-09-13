# FORTIS TSE Integration

Sistema de integração com o Tribunal Superior Eleitoral (TSE) e Gov.br para o FORTIS.

## Visão Geral

Este módulo implementa a integração completa com os sistemas do TSE e Gov.br, fornecendo:

- **Autenticação digital** via Gov.br OAuth2
- **Validação de eleitores** em tempo real
- **Sincronização de dados eleitorais** (eleições, candidatos, zonas)
- **Validação de certificados digitais** ICP-Brasil
- **Envio de dados de votação** para o TSE

## Estrutura do Projeto

```
tse-integration/
├── api/                    # Configurações da API
│   └── tse_config.yaml    # Configuração principal
├── certificates/           # Certificados digitais
├── docs/                  # Documentação
│   └── API_REFERENCE.md   # Referência da API
├── scripts/               # Scripts de automação
│   ├── sync_elections.py  # Sincronização de eleições
│   ├── validate_voters.py # Validação de eleitores
│   └── certificate_validator.py # Validação de certificados
├── sync/                  # Serviços de sincronização
├── tests/                 # Testes de integração
│   └── test_tse_integration.py
└── validation/            # Validações de dados
    └── voter_validation.py
```

## Configuração

### 1. Variáveis de Ambiente

```bash
# TSE API
export TSE_API_KEY="sua_chave_api_tse"
export TSE_BASE_URL="https://api.tse.jus.br"

# Gov.br OAuth2
export GOV_BR_CLIENT_ID="seu_client_id"
export GOV_BR_CLIENT_SECRET="seu_client_secret"
export GOV_BR_REDIRECT_URI="http://localhost:3000/auth/callback"

# Certificados
export TSE_CLIENT_CERT="/path/to/client.crt"
export TSE_CLIENT_KEY="/path/to/client.key"
export TSE_CA_CERT="/path/to/ca.crt"
```

### 2. Arquivo de Configuração

Edite `api/tse_config.yaml` com suas configurações:

```yaml
tse:
  base_url: "https://api.tse.jus.br"
  api_key: "${TSE_API_KEY}"
  # ... outras configurações
```

## Uso

### 1. Sincronização de Eleições

```bash
# Sincronizar todas as eleições
python scripts/sync_elections.py --api-key $TSE_API_KEY

# Sincronizar eleição específica
python scripts/sync_elections.py --api-key $TSE_API_KEY --election-id eleicao_2024

# Salvar resultados em arquivo
python scripts/sync_elections.py --api-key $TSE_API_KEY --output results.json
```

### 2. Validação de Eleitores

```bash
# Validar CPF específico
python scripts/validate_voters.py --api-key $TSE_API_KEY --cpf 12345678901

# Validar título específico
python scripts/validate_voters.py --api-key $TSE_API_KEY --voter-id 12345678

# Validar lote de eleitores (CSV)
python scripts/validate_voters.py --api-key $TSE_API_KEY --input-csv voters.csv --output-csv results.csv
```

### 3. Validação de Certificados

```bash
# Validar certificado de arquivo
python scripts/certificate_validator.py --cert-file certificate.pem

# Validar certificado inline
python scripts/certificate_validator.py --cert-data "-----BEGIN CERTIFICATE-----..."

# Salvar resultados
python scripts/certificate_validator.py --cert-file certificate.pem --output results.json
```

## API Endpoints

### Autenticação Gov.br

- `GET /auth/gov-br/url` - Obter URL de autorização
- `POST /auth/gov-br/callback` - Callback de autorização
- `GET /auth/gov-br/user` - Obter dados do usuário

### Validação de Eleitores

- `GET /voter/validate/cpf/{cpf}` - Validar por CPF
- `GET /voter/validate/id/{voter_id}` - Validar por título
- `GET /voter/data/{cpf}` - Obter dados completos
- `GET /voter/can-vote/{cpf}/{election_id}` - Verificar elegibilidade
- `GET /voter/has-voted/{cpf}/{election_id}` - Verificar se já votou
- `GET /voter/history/{cpf}` - Obter histórico de votos

### Certificados Digitais

- `POST /certificate/validate` - Validar certificado
- `POST /certificate/sign` - Assinar dados
- `POST /certificate/verify` - Verificar assinatura

### Sincronização

- `POST /elections/sync` - Sincronizar eleições
- `GET /elections/active` - Obter eleições ativas
- `GET /elections/{id}` - Obter eleição específica
- `GET /elections/{id}/candidates` - Obter candidatos
- `GET /elections/{id}/zones` - Obter zonas eleitorais
- `GET /elections/{id}/rules` - Obter regras
- `GET /elections/{id}/stats` - Obter estatísticas

### Votação

- `POST /votes` - Enviar dados de voto

## Exemplos de Uso

### Python

```python
import aiohttp
import asyncio

async def validate_voter():
    async with aiohttp.ClientSession() as session:
        async with session.get(
            'https://api.fortis.gov.br/api/v1/tse/voter/validate/cpf/12345678901',
            headers={'Authorization': 'Bearer seu_token'}
        ) as response:
            data = await response.json()
            print(data)

asyncio.run(validate_voter())
```

### JavaScript

```javascript
const response = await fetch(
    'https://api.fortis.gov.br/api/v1/tse/voter/validate/cpf/12345678901',
    {
        headers: {
            'Authorization': 'Bearer seu_token',
            'Content-Type': 'application/json'
        }
    }
);

const data = await response.json();
console.log(data);
```

### cURL

```bash
curl -X GET \
  'https://api.fortis.gov.br/api/v1/tse/voter/validate/cpf/12345678901' \
  -H 'Authorization: Bearer seu_token' \
  -H 'Content-Type: application/json'
```

## Testes

Execute os testes de integração:

```bash
# Instalar dependências
pip install pytest aiohttp

# Executar testes
pytest tests/test_tse_integration.py -v
```

## Monitoramento

### Métricas Disponíveis

- `tse_requests_total` - Total de requisições para TSE
- `tse_errors_total` - Total de erros na API TSE
- `voter_validations_total` - Total de validações de eleitores
- `certificate_validations_total` - Total de validações de certificados
- `sync_duration_seconds` - Duração das sincronizações

### Health Check

```bash
curl https://api.fortis.gov.br/health
```

### Status da API

```bash
curl https://api.fortis.gov.br/api/v1/tse/health
```

## Segurança

### Autenticação

- **JWT Tokens** para autenticação de usuários
- **Certificados digitais** ICP-Brasil para assinatura
- **API Keys** para serviços de sincronização
- **Basic Auth** para administração

### Criptografia

- **AES-256-GCM** para dados sensíveis
- **RSA-SHA256** para assinatura digital
- **TLS 1.3** para transporte

### Validação

- **Validação de CPF** com algoritmo oficial
- **Validação de título** de eleitor
- **Validação de certificados** com OCSP
- **Rate limiting** para prevenir abuso

## Troubleshooting

### Problemas Comuns

1. **Erro de autenticação**
   - Verificar se o token JWT é válido
   - Verificar se o certificado não expirou
   - Verificar se a API key está correta

2. **Erro de validação de eleitor**
   - Verificar se o CPF é válido
   - Verificar se o eleitor está ativo no TSE
   - Verificar conectividade com a API TSE

3. **Erro de sincronização**
   - Verificar conectividade com TSE
   - Verificar se a API key tem permissões
   - Verificar logs para detalhes do erro

### Logs

```bash
# Logs da aplicação
tail -f /var/log/fortis/tse-integration.log

# Logs de erro
grep ERROR /var/log/fortis/tse-integration.log

# Logs de sincronização
grep "sync" /var/log/fortis/tse-integration.log
```

## Suporte

- **Documentação**: https://docs.fortis.gov.br
- **API Reference**: https://docs.fortis.gov.br/api/tse
- **Status**: https://status.fortis.gov.br
- **Suporte**: suporte@fortis.gov.br

## Licença

MIT License - veja [LICENSE](../../LICENSE) para detalhes.
