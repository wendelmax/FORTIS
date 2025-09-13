# FORTIS - Especificações de API
## Backend Architect Perspective

### 🎯 **Visão Geral das APIs**

O FORTIS implementa uma arquitetura de APIs RESTful completa com autenticação robusta, rate limiting, versionamento e documentação OpenAPI para garantir integração perfeita entre todos os componentes do sistema.

### 📊 **Análise de Cobertura de Endpoints**

- **[📋 Análise Completa de Endpoints](./ENDPOINTS_ANALYSIS.md)** - Documento detalhado com comparação de endpoints necessários vs implementados
- **✅ Cobertura**: 100% dos endpoints necessários implementados
- **✅ Documentação Swagger**: Interface interativa completa
- **✅ Integração TSE**: 15 endpoints específicos para integração oficial

---

## 🏗️ **Arquitetura de APIs**

### **1. Estrutura de Versionamento**
```
Base URL: https://api.fortis.gov.br
Versioning: /v1/, /v2/, etc.
Content-Type: application/json
Authorization: Bearer <jwt_token>
```

### **2. Padrões de Resposta**
```json
{
  "success": true,
  "data": { ... },
  "message": "Operação realizada com sucesso",
  "timestamp": "2025-12-19T10:30:00Z",
  "request_id": "req_123456789"
}
```

### **3. Códigos de Erro Padronizados**
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Dados de entrada inválidos",
    "details": {
      "field": "cpf",
      "reason": "Formato inválido"
    }
  },
  "timestamp": "2025-12-19T10:30:00Z",
  "request_id": "req_123456789"
}
```

---

## 🔐 **Autenticação e Autorização**

### **Endpoints de Autenticação**

#### **POST /api/v1/auth/login**
```json
// Request
{
  "cpf": "12345678901",
  "biometric_data": {
    "fingerprint": "base64_encoded_data",
    "facial": "base64_encoded_data"
  },
  "certificate": "base64_encoded_certificate"
}

// Response
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600,
    "token_type": "Bearer",
    "user": {
      "id": "user_123",
      "cpf": "12345678901",
      "name": "João Silva",
      "roles": ["voter"],
      "election_eligible": true
    }
  }
}
```

#### **POST /api/v1/auth/refresh**
```json
// Request
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}

// Response
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600
  }
}
```

#### **POST /api/v1/auth/logout**
```json
// Request
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}

// Response
{
  "success": true,
  "message": "Logout realizado com sucesso"
}
```

---

## 🗳️ **APIs de Eleições**

### **GET /api/v1/elections**
Lista todas as eleições disponíveis

```json
// Response
{
  "success": true,
  "data": {
    "elections": [
      {
        "id": "ele_123",
        "name": "Eleição Presidencial 2026",
        "description": "Primeiro turno da eleição presidencial",
        "start_date": "2026-10-06T08:00:00Z",
        "end_date": "2026-10-06T17:00:00Z",
        "status": "active",
        "candidates_count": 8,
        "total_voters": 150000000,
        "votes_cast": 120000000
      }
    ],
    "pagination": {
      "page": 1,
      "per_page": 20,
      "total": 1,
      "total_pages": 1
    }
  }
}
```

### **GET /api/v1/elections/{election_id}**
Detalhes de uma eleição específica

```json
// Response
{
  "success": true,
  "data": {
    "id": "ele_123",
    "name": "Eleição Presidencial 2026",
    "description": "Primeiro turno da eleição presidencial",
    "start_date": "2026-10-06T08:00:00Z",
    "end_date": "2026-10-06T17:00:00Z",
    "status": "active",
    "candidates": [
      {
        "id": "cand_123",
        "name": "João Silva",
        "party": "PT",
        "position": "Presidente",
        "photo_url": "https://cdn.fortis.gov.br/candidates/joao_silva.jpg",
        "number": "13"
      }
    ],
    "voting_stats": {
      "total_voters": 150000000,
      "votes_cast": 120000000,
      "participation_rate": 80.0
    }
  }
}
```

### **POST /api/v1/elections**
Criar nova eleição (apenas administradores)

```json
// Request
{
  "name": "Eleição Municipal 2025",
  "description": "Eleição para prefeito e vereadores",
  "start_date": "2025-10-06T08:00:00Z",
  "end_date": "2025-10-06T17:00:00Z",
  "candidates": [
    {
      "name": "Maria Santos",
      "party": "PSDB",
      "position": "Prefeita",
      "number": "45"
    }
  ]
}

// Response
{
  "success": true,
  "data": {
    "id": "ele_124",
    "name": "Eleição Municipal 2025",
    "status": "draft",
    "created_at": "2025-12-19T10:30:00Z"
  }
}
```

---

## 🗳️ **APIs de Votação**

### **POST /api/v1/votes**
Registrar voto

```json
// Request
{
  "election_id": "ele_123",
  "candidate_id": "cand_123",
  "voter_id": "user_123",
  "biometric_verification": {
    "fingerprint_hash": "sha256_hash",
    "facial_hash": "sha256_hash"
  },
  "zk_proof": "base64_encoded_zk_proof",
  "encrypted_vote": "base64_encoded_encrypted_vote"
}

// Response
{
  "success": true,
  "data": {
    "vote_id": "vote_123",
    "transaction_hash": "0x1234567890abcdef...",
    "block_number": 12345678,
    "timestamp": "2025-12-19T10:30:00Z",
    "receipt": {
      "vote_id": "vote_123",
      "election_id": "ele_123",
      "candidate_number": "13",
      "verification_code": "ABC123XYZ"
    }
  }
}
```

### **GET /api/v1/votes/{vote_id}**
Verificar voto (apenas o próprio eleitor)

```json
// Response
{
  "success": true,
  "data": {
    "vote_id": "vote_123",
    "election_id": "ele_123",
    "candidate_number": "13",
    "timestamp": "2025-12-19T10:30:00Z",
    "transaction_hash": "0x1234567890abcdef...",
    "block_number": 12345678,
    "verified": true
  }
}
```

### **GET /api/v1/votes/verify/{verification_code}**
Verificar voto por código de verificação

```json
// Response
{
  "success": true,
  "data": {
    "vote_id": "vote_123",
    "election_id": "ele_123",
    "candidate_number": "13",
    "timestamp": "2025-12-19T10:30:00Z",
    "verified": true
  }
}
```

---

## 🌐 **APIs de Nós Distribuídos**

### **GET /api/v1/nodes**
Lista todos os nós da rede

```json
// Response
{
  "success": true,
  "data": {
    "nodes": [
      {
        "id": "node_sp",
        "name": "TSE São Paulo",
        "state": "SP",
        "status": "active",
        "last_sync": "2025-12-19T10:29:00Z",
        "votes_processed": 15000000,
        "latency_ms": 45
      }
    ],
    "network_stats": {
      "total_nodes": 27,
      "active_nodes": 27,
      "total_votes": 120000000,
      "consensus_rate": 99.9
    }
  }
}
```

### **GET /api/v1/nodes/{node_id}/status**
Status detalhado de um nó

```json
// Response
{
  "success": true,
  "data": {
    "id": "node_sp",
    "name": "TSE São Paulo",
    "status": "active",
    "health": {
      "cpu_usage": 45.2,
      "memory_usage": 67.8,
      "disk_usage": 23.1,
      "network_latency": 45
    },
    "blockchain": {
      "last_block": 12345678,
      "sync_status": "synced",
      "pending_transactions": 0
    },
    "votes": {
      "processed_today": 1500000,
      "total_processed": 15000000,
      "error_rate": 0.01
    }
  }
}
```

---

## 📊 **APIs de Auditoria Pública**

### **GET /api/v1/audit/elections/{election_id}/results**
Resultados públicos de uma eleição

```json
// Response
{
  "success": true,
  "data": {
    "election_id": "ele_123",
    "total_votes": 120000000,
    "results": [
      {
        "candidate_id": "cand_123",
        "candidate_name": "João Silva",
        "candidate_number": "13",
        "party": "PT",
        "votes": 60000000,
        "percentage": 50.0
      }
    ],
    "verification": {
      "blockchain_hash": "0x1234567890abcdef...",
      "merkle_root": "0xabcdef1234567890...",
      "audit_trail": "https://audit.fortis.gov.br/ele_123"
    }
  }
}
```

### **GET /api/v1/audit/votes/verify**
Verificar integridade dos votos

```json
// Request
{
  "election_id": "ele_123",
  "vote_ids": ["vote_123", "vote_124", "vote_125"]
}

// Response
{
  "success": true,
  "data": {
    "verified_votes": 3,
    "total_votes": 3,
    "integrity_score": 100.0,
    "verification_details": [
      {
        "vote_id": "vote_123",
        "verified": true,
        "blockchain_confirmed": true,
        "zk_proof_valid": true
      }
    ]
  }
}
```

---

## 🤖 **APIs de IA**

### **POST /api/v1/ai/assistant/chat**
Chat com assistente eleitoral

```json
// Request
{
  "message": "Como posso votar?",
  "context": {
    "election_id": "ele_123",
    "user_id": "user_123"
  }
}

// Response
{
  "success": true,
  "data": {
    "response": "Para votar, você deve se dirigir à sua seção eleitoral no dia da eleição com seu documento de identidade. O processo é simples: 1) Identifique-se na urna, 2) Digite o número do candidato, 3) Confirme seu voto.",
    "suggestions": [
      "Onde fica minha seção eleitoral?",
      "Quais documentos preciso levar?",
      "Posso votar em qualquer horário?"
    ],
    "confidence": 0.95
  }
}
```

### **POST /api/v1/ai/fraud/detect**
Detecção de fraude

```json
// Request
{
  "vote_data": {
    "voter_id": "user_123",
    "election_id": "ele_123",
    "candidate_id": "cand_123",
    "timestamp": "2025-12-19T10:30:00Z",
    "location": {
      "latitude": -23.5505,
      "longitude": -46.6333
    }
  }
}

// Response
{
  "success": true,
  "data": {
    "fraud_score": 0.15,
    "risk_level": "low",
    "anomalies": [],
    "recommendation": "Voto aprovado"
  }
}
```

---

## 🔧 **APIs de Administração**

### **GET /api/v1/admin/dashboard**
Dashboard administrativo

```json
// Response
{
  "success": true,
  "data": {
    "elections": {
      "total": 5,
      "active": 1,
      "completed": 4
    },
    "votes": {
      "total_today": 1500000,
      "total_all_time": 120000000,
      "participation_rate": 80.0
    },
    "nodes": {
      "total": 27,
      "active": 27,
      "issues": 0
    },
    "system_health": {
      "status": "healthy",
      "uptime": 99.9,
      "response_time": 45
    }
  }
}
```

### **POST /api/v1/admin/elections/{election_id}/approve**
Aprovar eleição (apenas ministros)

```json
// Request
{
  "approval_data": {
    "minister_id": "minister_123",
    "digital_signature": "base64_encoded_signature",
    "certificate": "base64_encoded_certificate"
  }
}

// Response
{
  "success": true,
  "data": {
    "election_id": "ele_123",
    "status": "approved",
    "approved_by": "minister_123",
    "approved_at": "2025-12-19T10:30:00Z"
  }
}
```

---

## 📈 **Rate Limiting e Segurança**

### **Rate Limits**
```
- Autenticação: 5 requests/minuto por IP
- Votação: 1 request/minuto por usuário
- Consultas: 100 requests/minuto por usuário
- Administração: 1000 requests/minuto por usuário
```

### **Headers de Segurança**
```
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
Content-Security-Policy: default-src 'self'
```

---

## 📚 **Documentação OpenAPI**

### **Swagger UI**
```
URL: https://api.fortis.gov.br/docs
Versão: OpenAPI 3.0
Autenticação: Bearer Token
```

### **Especificação Completa**
```yaml
openapi: 3.0.0
info:
  title: FORTIS API
  description: Sistema de Votação Eletrônica Brasileiro
  version: 1.0.0
  contact:
    name: Equipe FORTIS
    email: api@fortis.gov.br
servers:
  - url: https://api.fortis.gov.br/v1
    description: Produção
  - url: https://staging-api.fortis.gov.br/v1
    description: Staging
```

---

## 🧪 **Testes de API**

### **Coleção Postman**
```
URL: https://api.fortis.gov.br/postman/collection
Ambiente: Produção, Staging, Desenvolvimento
Autenticação: Configurada automaticamente
```

### **Testes Automatizados**
```bash
# Executar todos os testes
npm run test:api

# Testes específicos
npm run test:api -- --grep "authentication"
npm run test:api -- --grep "voting"
```

---

*Documentação de APIs FORTIS - Desenvolvida pelo Backend Architect Agent*
