# FORTIS Backend - API Reference

## Visão Geral

O FORTIS Backend é uma API RESTful desenvolvida em Rust que fornece todas as funcionalidades necessárias para o sistema de votação eletrônica brasileiro.

## Base URL

```
http://localhost:8080/api/v1
```

## Autenticação

O FORTIS usa JWT (JSON Web Tokens) para autenticação. Inclua o token no header `Authorization`:

```
Authorization: Bearer <token>
```

## Endpoints

### Autenticação

#### POST /auth/login
Autentica um usuário no sistema.

**Request Body:**
```json
{
  "cpf": "12345678901",
  "biometric_data": "base64_encoded_fingerprint",
  "certificate": "base64_encoded_certificate"
}
```

**Response:**
```json
{
  "success": true,
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "user": {
    "id": "uuid",
    "cpf": "12345678901",
    "name": "João Silva",
    "zone": "123",
    "section": "456"
  }
}
```

#### POST /auth/refresh
Renova um token de acesso.

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

#### POST /auth/logout
Invalida o token atual.

**Headers:**
```
Authorization: Bearer <token>
```

### Eleições

#### GET /elections
Lista todas as eleições.

**Query Parameters:**
- `status` (optional): Filtrar por status (active, completed, scheduled)
- `limit` (optional): Número máximo de resultados (padrão: 50)
- `offset` (optional): Offset para paginação (padrão: 0)

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "Eleição Municipal 2025",
      "description": "Eleição para prefeito e vereadores",
      "start_date": "2025-10-01T08:00:00Z",
      "end_date": "2025-10-01T17:00:00Z",
      "status": "active",
      "created_at": "2025-09-01T10:00:00Z",
      "updated_at": "2025-09-01T10:00:00Z"
    }
  ],
  "pagination": {
    "total": 1,
    "limit": 50,
    "offset": 0
  }
}
```

#### POST /elections
Cria uma nova eleição.

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "Eleição Municipal 2025",
  "description": "Eleição para prefeito e vereadores",
  "start_date": "2025-10-01T08:00:00Z",
  "end_date": "2025-10-01T17:00:00Z"
}
```

#### GET /elections/{id}
Obtém uma eleição específica.

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Eleição Municipal 2025",
    "description": "Eleição para prefeito e vereadores",
    "start_date": "2025-10-01T08:00:00Z",
    "end_date": "2025-10-01T17:00:00Z",
    "status": "active",
    "candidates": [
      {
        "id": "uuid",
        "name": "João Silva",
        "party": "PT",
        "number": 13,
        "bio": "Candidato a prefeito"
      }
    ],
    "stats": {
      "total_votes": 1500000,
      "unique_voters": 1200000
    }
  }
}
```

### Votos

#### POST /votes
Registra um voto.

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "election_id": "uuid",
  "candidate_id": "uuid",
  "biometric_verification": "base64_encoded_fingerprint"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "vote_id": "uuid",
    "election_id": "uuid",
    "candidate_id": "uuid",
    "timestamp": "2025-10-01T10:30:00Z",
    "verification_hash": "sha256_hash",
    "blockchain_tx": "0x1234567890abcdef..."
  }
}
```

#### GET /votes/{election_id}
Obtém votos de uma eleição (apenas para auditores).

**Headers:**
```
Authorization: Bearer <token>
```

**Response:**
```json
{
  "success": true,
  "data": {
    "election_id": "uuid",
    "total_votes": 1500000,
    "votes_by_candidate": [
      {
        "candidate_id": "uuid",
        "candidate_name": "João Silva",
        "votes": 450000,
        "percentage": 30.0
      }
    ],
    "verification_hashes": [
      "sha256_hash_1",
      "sha256_hash_2"
    ]
  }
}
```

### Candidatos

#### GET /elections/{election_id}/candidates
Lista candidatos de uma eleição.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "João Silva",
      "party": "PT",
      "number": 13,
      "bio": "Candidato a prefeito",
      "votes": 450000,
      "percentage": 30.0
    }
  ]
}
```

### Nós Distribuídos

#### GET /nodes
Lista todos os nós da rede.

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "Nó TSE Brasília",
      "location": "Brasília, DF",
      "ip_address": "200.160.2.3",
      "status": "active",
      "last_heartbeat": "2025-10-01T10:30:00Z",
      "version": "1.0.0"
    }
  ]
}
```

#### POST /nodes
Registra um novo nó na rede.

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "Nó Regional SP",
  "location": "São Paulo, SP",
  "ip_address": "200.160.2.100",
  "public_key": "base64_encoded_public_key"
}
```

### Auditoria

#### GET /audit
Lista auditorias realizadas.

**Query Parameters:**
- `election_id` (optional): Filtrar por eleição
- `auditor_id` (optional): Filtrar por auditor
- `status` (optional): Filtrar por status

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "election_id": "uuid",
      "auditor_id": "uuid",
      "auditor_name": "Maria Santos",
      "status": "completed",
      "created_at": "2025-10-01T10:00:00Z",
      "completed_at": "2025-10-01T11:00:00Z",
      "findings": {
        "total_votes_verified": 1500000,
        "anomalies_found": 0,
        "integrity_verified": true
      }
    }
  ]
}
```

#### POST /audit
Inicia uma nova auditoria.

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "election_id": "uuid",
  "audit_type": "full",
  "description": "Auditoria completa da eleição municipal"
}
```

### Integração TSE

#### GET /tse/sync
Sincroniza dados com o TSE.

**Headers:**
```
Authorization: Bearer <token>
```

**Response:**
```json
{
  "success": true,
  "data": {
    "last_sync": "2025-10-01T10:00:00Z",
    "voters_synced": 50000000,
    "elections_synced": 5,
    "status": "completed"
  }
}
```

#### GET /tse/voters/{cpf}
Valida um eleitor no TSE.

**Response:**
```json
{
  "success": true,
  "data": {
    "cpf": "12345678901",
    "name": "João Silva",
    "birth_date": "1980-01-01",
    "voter_id": "123456789012",
    "zone": "123",
    "section": "456",
    "is_active": true,
    "last_update": "2025-09-01T10:00:00Z"
  }
}
```

### Zero-Knowledge Proofs

#### POST /zkp/generate
Gera prova ZK para um voto.

**Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "vote_data": "encrypted_vote_data",
  "public_inputs": {
    "election_id": "uuid",
    "candidate_id": "uuid"
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "proof": "base64_encoded_proof",
    "public_inputs": {
      "election_id": "uuid",
      "candidate_id": "uuid"
    },
    "verification_key": "base64_encoded_vk"
  }
}
```

#### POST /zkp/verify
Verifica uma prova ZK.

**Request Body:**
```json
{
  "proof": "base64_encoded_proof",
  "public_inputs": {
    "election_id": "uuid",
    "candidate_id": "uuid"
  },
  "verification_key": "base64_encoded_vk"
}
```

## Códigos de Status HTTP

- `200 OK` - Requisição bem-sucedida
- `201 Created` - Recurso criado com sucesso
- `400 Bad Request` - Dados inválidos
- `401 Unauthorized` - Token inválido ou expirado
- `403 Forbidden` - Acesso negado
- `404 Not Found` - Recurso não encontrado
- `409 Conflict` - Conflito (ex: voto duplicado)
- `422 Unprocessable Entity` - Dados válidos mas não processáveis
- `429 Too Many Requests` - Rate limit excedido
- `500 Internal Server Error` - Erro interno do servidor

## Tratamento de Erros

Todos os erros seguem o formato:

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Descrição do erro",
    "details": "Detalhes adicionais (opcional)"
  },
  "timestamp": "2025-10-01T10:30:00Z"
}
```

### Códigos de Erro Comuns

- `INVALID_CREDENTIALS` - Credenciais inválidas
- `TOKEN_EXPIRED` - Token expirado
- `VOTE_ALREADY_CAST` - Voto já registrado
- `ELECTION_NOT_ACTIVE` - Eleição não está ativa
- `VOTER_NOT_FOUND` - Eleitor não encontrado
- `CANDIDATE_NOT_FOUND` - Candidato não encontrado
- `INVALID_BIOMETRIC` - Dados biométricos inválidos
- `RATE_LIMIT_EXCEEDED` - Rate limit excedido
- `BLOCKCHAIN_ERROR` - Erro na blockchain
- `TSE_CONNECTION_ERROR` - Erro de conexão com TSE

## Rate Limiting

O FORTIS implementa rate limiting para prevenir abuso:

- **Autenticação**: 5 tentativas por minuto por IP
- **Votação**: 1 voto por eleição por eleitor
- **API Geral**: 100 requisições por minuto por token
- **Auditoria**: 10 requisições por minuto por auditor

## Webhooks

O FORTIS suporta webhooks para notificações em tempo real:

### Eventos Disponíveis

- `vote.cast` - Voto registrado
- `election.started` - Eleição iniciada
- `election.ended` - Eleição finalizada
- `audit.completed` - Auditoria concluída
- `anomaly.detected` - Anomalia detectada

### Configuração de Webhook

```json
{
  "url": "https://your-app.com/webhooks/fortis",
  "events": ["vote.cast", "election.ended"],
  "secret": "your_webhook_secret"
}
```

## Exemplos de Uso

### Fluxo Completo de Votação

1. **Login do Eleitor**
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "cpf": "12345678901",
    "biometric_data": "base64_fingerprint",
    "certificate": "base64_certificate"
  }'
```

2. **Listar Eleições Ativas**
```bash
curl -X GET http://localhost:8080/api/v1/elections?status=active \
  -H "Authorization: Bearer <token>"
```

3. **Listar Candidatos**
```bash
curl -X GET http://localhost:8080/api/v1/elections/{election_id}/candidates \
  -H "Authorization: Bearer <token>"
```

4. **Registrar Voto**
```bash
curl -X POST http://localhost:8080/api/v1/votes \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "election_id": "uuid",
    "candidate_id": "uuid",
    "biometric_verification": "base64_fingerprint"
  }'
```

5. **Verificar Voto**
```bash
curl -X GET http://localhost:8080/api/v1/votes/verify/{vote_id} \
  -H "Authorization: Bearer <token>"
```

## SDKs e Bibliotecas

### JavaScript/TypeScript
```bash
npm install @fortis/sdk
```

### Python
```bash
pip install fortis-sdk
```

### Rust
```toml
[dependencies]
fortis-client = "0.1.0"
```

## Suporte

Para dúvidas sobre a API:

- **Documentação**: [docs.fortis.gov.br](https://docs.fortis.gov.br)
- **Email**: api-support@fortis.gov.br
- **GitHub**: [github.com/fortis-gov/fortis](https://github.com/fortis-gov/fortis)
