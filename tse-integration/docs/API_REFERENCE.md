# FORTIS TSE Integration - API Reference

## Visão Geral

A integração TSE do FORTIS fornece APIs para autenticação, validação de eleitores e sincronização de dados eleitorais com o Tribunal Superior Eleitoral e Gov.br.

## Base URL

```
https://api.fortis.gov.br/api/v1/tse
```

## Autenticação

Todas as requisições requerem autenticação via Bearer Token:

```http
Authorization: Bearer <seu_token_jwt>
```

## Endpoints

### 1. Autenticação Gov.br

#### 1.1 Obter URL de Autorização

```http
GET /auth/gov-br/url?state={state}
```

**Parâmetros:**
- `state` (opcional): String de estado para validação CSRF

**Resposta:**
```json
{
  "success": true,
  "data": {
    "auth_url": "https://sso.acesso.gov.br/authorize?...",
    "state": "default"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 1.2 Callback de Autorização

```http
POST /auth/gov-br/callback
```

**Body:**
```json
{
  "code": "authorization_code",
  "state": "state_value"
}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJSUzI1NiIs...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "refresh_token": "refresh_token_value",
    "scope": "openid profile cpf"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 1.3 Obter Dados do Usuário

```http
GET /auth/gov-br/user?access_token={access_token}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "sub": "12345678901",
    "name": "João Silva",
    "given_name": "João",
    "family_name": "Silva",
    "email": "joao@email.com",
    "phone_number": "+5511999999999",
    "cpf": "12345678901",
    "birthdate": "1990-01-01",
    "pis": "12345678901",
    "voter_id": "12345678",
    "verified": true
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### 2. Validação de Eleitores

#### 2.1 Validar por CPF

```http
GET /voter/validate/cpf/{cpf}
```

**Parâmetros:**
- `cpf`: CPF do eleitor (apenas números)

**Resposta:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "voter_data": {
      "cpf": "12345678901",
      "voter_id": "12345678",
      "name": "João Silva",
      "birth_date": "1990-01-01T00:00:00Z",
      "voting_zone": "123",
      "voting_section": "456",
      "city": "São Paulo",
      "state": "SP",
      "status": "ATIVO",
      "last_vote": "2022-10-02T00:00:00Z",
      "biometric_data": {
        "fingerprint_hash": "abc123...",
        "face_hash": "def456...",
        "voice_hash": "ghi789...",
        "last_update": "2024-01-01T00:00:00Z"
      }
    },
    "validation_timestamp": "2024-01-01T00:00:00Z"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 2.2 Validar por Título

```http
GET /voter/validate/id/{voter_id}
```

**Parâmetros:**
- `voter_id`: Título de eleitor

**Resposta:** Mesmo formato da validação por CPF.

#### 2.3 Obter Dados Completos do Eleitor

```http
GET /voter/data/{cpf}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "cpf": "12345678901",
    "voter_id": "12345678",
    "name": "João Silva",
    "birth_date": "1990-01-01T00:00:00Z",
    "voting_zone": "123",
    "voting_section": "456",
    "city": "São Paulo",
    "state": "SP",
    "status": "ATIVO",
    "last_vote": "2022-10-02T00:00:00Z",
    "biometric_data": {
      "fingerprint_hash": "abc123...",
      "face_hash": "def456...",
      "voice_hash": "ghi789...",
      "last_update": "2024-01-01T00:00:00Z"
    },
    "digital_certificate": "certificate_data"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 2.4 Verificar Elegibilidade para Votar

```http
GET /voter/can-vote/{cpf}/{election_id}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "can_vote": true,
    "cpf": "12345678901",
    "election_id": "eleicao_2024"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 2.5 Verificar se Já Votou

```http
GET /voter/has-voted/{cpf}/{election_id}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "has_voted": false,
    "cpf": "12345678901",
    "election_id": "eleicao_2024"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 2.6 Obter Histórico de Votos

```http
GET /voter/history/{cpf}
```

**Resposta:**
```json
{
  "success": true,
  "data": [
    {
      "election_id": "eleicao_2022",
      "election_name": "Eleições Gerais 2022",
      "vote_date": "2022-10-02T00:00:00Z",
      "voting_zone": "123",
      "voting_section": "456",
      "candidate_id": "cand_123",
      "candidate_name": "Candidato A",
      "position": "Presidente"
    }
  ],
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### 3. Certificados Digitais

#### 3.1 Validar Certificado

```http
POST /certificate/validate
```

**Body:**
```json
{
  "certificate_data": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----"
}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "is_valid": true,
    "validation_timestamp": "2024-01-01T00:00:00Z",
    "errors": [],
    "warnings": [],
    "certificate_info": {
      "serial_number": "1234567890",
      "subject": {
        "common_name": "João Silva",
        "cpf": "12345678901",
        "email": "joao@email.com",
        "organization": "Empresa XYZ",
        "country": "BR"
      },
      "issuer": {
        "common_name": "AC Serasa v5",
        "organization": "Serasa Experian",
        "country": "BR"
      },
      "validity": {
        "not_before": "2023-01-01T00:00:00Z",
        "not_after": "2024-12-31T23:59:59Z",
        "is_valid": true,
        "days_until_expiry": 365
      },
      "key_usage": ["DigitalSignature", "NonRepudiation"],
      "extended_key_usage": ["ClientAuth", "EmailProtection"],
      "certificate_type": "A1"
    }
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 3.2 Assinar Dados

```http
POST /certificate/sign
```

**Body:**
```json
{
  "data": "dados_para_assinatura",
  "certificate_data": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----"
}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "signature": "assinatura_base64",
    "data": "dados_para_assinatura"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 3.3 Verificar Assinatura

```http
POST /certificate/verify
```

**Body:**
```json
{
  "data": "dados_originais",
  "signature": "assinatura_base64",
  "certificate_data": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----"
}
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "is_valid": true,
    "data": "dados_originais"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### 4. Sincronização de Eleições

#### 4.1 Sincronizar Todas as Eleições

```http
POST /elections/sync
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "success": true,
    "elections_synced": 3,
    "errors": [],
    "last_sync": "2024-01-01T00:00:00Z",
    "next_sync": "2024-01-01T01:00:00Z"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 4.2 Obter Eleições Ativas

```http
GET /elections/active
```

**Resposta:**
```json
{
  "success": true,
  "data": [
    {
      "election_id": "eleicao_2024",
      "name": "Eleições Municipais 2024",
      "description": "Eleições para prefeito e vereadores",
      "start_date": "2024-10-06T08:00:00Z",
      "end_date": "2024-10-06T17:00:00Z",
      "status": "AGENDADA",
      "election_type": "MAYOR",
      "voting_zones": [
        {
          "zone_id": "zone_123",
          "name": "Zona 123",
          "state": "SP",
          "city": "São Paulo",
          "sections": [
            {
              "section_id": "sec_456",
              "number": "456",
              "location": "Escola Municipal",
              "address": "Rua das Flores, 123",
              "capacity": 300,
              "voters_count": 250
            }
          ]
        }
      ],
      "candidates": [
        {
          "candidate_id": "cand_123",
          "name": "João Silva",
          "party": "PT",
          "number": "13",
          "position": "Prefeito",
          "photo_url": "https://example.com/photo.jpg",
          "biography": "Biografia do candidato",
          "proposals": ["Proposta 1", "Proposta 2"],
          "status": "ATIVO"
        }
      ],
      "rules": {
        "voting_hours": {
          "start_time": "08:00",
          "end_time": "17:00",
          "timezone": "America/Sao_Paulo"
        },
        "allowed_voters": ["12345678901"],
        "voting_methods": ["ELECTRONIC_MACHINE", "MOBILE_APP"],
        "security_requirements": {
          "biometric_verification": true,
          "digital_certificate": true,
          "two_factor_auth": false,
          "encryption_required": true,
          "audit_logging": true
        },
        "audit_requirements": {
          "real_time_monitoring": true,
          "vote_receipt": true,
          "public_verification": true,
          "immutable_logs": true,
          "external_auditors": true
        }
      },
      "last_sync": "2024-01-01T00:00:00Z"
    }
  ],
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 4.3 Obter Eleição Específica

```http
GET /elections/{election_id}
```

**Resposta:** Mesmo formato do item da lista de eleições ativas.

#### 4.4 Obter Candidatos da Eleição

```http
GET /elections/{election_id}/candidates
```

**Resposta:**
```json
{
  "success": true,
  "data": [
    {
      "candidate_id": "cand_123",
      "name": "João Silva",
      "party": "PT",
      "number": "13",
      "position": "Prefeito",
      "photo_url": "https://example.com/photo.jpg",
      "biography": "Biografia do candidato",
      "proposals": ["Proposta 1", "Proposta 2"],
      "status": "ATIVO"
    }
  ],
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 4.5 Obter Zonas Eleitorais

```http
GET /elections/{election_id}/zones
```

**Resposta:**
```json
{
  "success": true,
  "data": [
    {
      "zone_id": "zone_123",
      "name": "Zona 123",
      "state": "SP",
      "city": "São Paulo",
      "sections": [
        {
          "section_id": "sec_456",
          "number": "456",
          "location": "Escola Municipal",
          "address": "Rua das Flores, 123",
          "capacity": 300,
          "voters_count": 250
        }
      ]
    }
  ],
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 4.6 Obter Regras da Eleição

```http
GET /elections/{election_id}/rules
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "voting_hours": {
      "start_time": "08:00",
      "end_time": "17:00",
      "timezone": "America/Sao_Paulo"
    },
    "allowed_voters": ["12345678901"],
    "voting_methods": ["ELECTRONIC_MACHINE", "MOBILE_APP"],
    "security_requirements": {
      "biometric_verification": true,
      "digital_certificate": true,
      "two_factor_auth": false,
      "encryption_required": true,
      "audit_logging": true
    },
    "audit_requirements": {
      "real_time_monitoring": true,
      "vote_receipt": true,
      "public_verification": true,
      "immutable_logs": true,
      "external_auditors": true
    }
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### 4.7 Obter Estatísticas da Eleição

```http
GET /elections/{election_id}/stats
```

**Resposta:**
```json
{
  "success": true,
  "data": {
    "total_voters": 1000000,
    "votes_cast": 750000,
    "participation_rate": 75.0,
    "votes_by_candidate": {
      "cand_123": 400000,
      "cand_456": 350000
    },
    "votes_by_zone": {
      "zone_123": 50000,
      "zone_456": 45000
    },
    "real_time_updates": true,
    "last_update": "2024-01-01T00:00:00Z"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### 5. Envio de Dados de Votação

#### 5.1 Enviar Dados de Voto

```http
POST /votes
```

**Body:**
```json
{
  "election_id": "eleicao_2024",
  "voter_cpf": "12345678901",
  "candidate_id": "cand_123",
  "voting_zone": "123",
  "voting_section": "456",
  "vote_hash": "hash_do_voto",
  "signature": "assinatura_do_voto",
  "verification_data": {
    "biometric_hash": "hash_biometrico",
    "device_id": "device_123",
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
```

**Resposta:**
```json
{
  "success": true,
  "data": "Voto enviado com sucesso",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Códigos de Erro

### Erros de Validação

- `VALIDATION_ERROR`: Dados de entrada inválidos
- `CPF_INVALID`: CPF inválido
- `VOTER_ID_INVALID`: Título de eleitor inválido
- `BIRTH_DATE_INVALID`: Data de nascimento inválida
- `MOTHER_NAME_INVALID`: Nome da mãe inválido

### Erros de Autenticação

- `AUTH_TOKEN_INVALID`: Token de autenticação inválido
- `AUTH_TOKEN_EXPIRED`: Token de autenticação expirado
- `GOV_BR_ERROR`: Erro na integração com Gov.br
- `TSE_AUTH_ERROR`: Erro na autenticação com TSE

### Erros de Certificado

- `CERTIFICATE_INVALID`: Certificado digital inválido
- `CERTIFICATE_EXPIRED`: Certificado expirado
- `CERTIFICATE_REVOKED`: Certificado revogado
- `CERTIFICATE_CHAIN_ERROR`: Erro na cadeia de certificação

### Erros de Sincronização

- `SYNC_ERROR`: Erro na sincronização
- `TSE_API_ERROR`: Erro na API do TSE
- `DATA_NOT_FOUND`: Dados não encontrados
- `SYNC_TIMEOUT`: Timeout na sincronização

## Rate Limiting

- **Limite padrão**: 100 requisições por minuto
- **Burst**: 20 requisições em 1 segundo
- **Headers de resposta**:
  - `X-RateLimit-Limit`: Limite total
  - `X-RateLimit-Remaining`: Requisições restantes
  - `X-RateLimit-Reset`: Timestamp de reset

## Exemplos de Uso

### Python

```python
import requests

# Configurar autenticação
headers = {
    'Authorization': 'Bearer seu_token_jwt',
    'Content-Type': 'application/json'
}

# Validar eleitor
response = requests.get(
    'https://api.fortis.gov.br/api/v1/tse/voter/validate/cpf/12345678901',
    headers=headers
)

if response.status_code == 200:
    data = response.json()
    if data['success'] and data['data']['valid']:
        print("Eleitor válido")
    else:
        print("Eleitor inválido")
```

### JavaScript

```javascript
// Validar eleitor
const response = await fetch(
    'https://api.fortis.gov.br/api/v1/tse/voter/validate/cpf/12345678901',
    {
        headers: {
            'Authorization': 'Bearer seu_token_jwt',
            'Content-Type': 'application/json'
        }
    }
);

const data = await response.json();
if (data.success && data.data.valid) {
    console.log('Eleitor válido');
} else {
    console.log('Eleitor inválido');
}
```

### cURL

```bash
# Validar eleitor
curl -X GET \
  'https://api.fortis.gov.br/api/v1/tse/voter/validate/cpf/12345678901' \
  -H 'Authorization: Bearer seu_token_jwt' \
  -H 'Content-Type: application/json'
```

## Suporte

Para suporte técnico, entre em contato:

- **Email**: suporte@fortis.gov.br
- **Documentação**: https://docs.fortis.gov.br
- **Status da API**: https://status.fortis.gov.br
