# FORTIS 3.0 Backend

Sistema de Votação Eletrônica Brasileiro - Backend em Rust

## 🚀 Características

- **Rust**: Performance e segurança máximas
- **Actix-Web**: Framework web assíncrono
- **PostgreSQL**: Banco de dados principal
- **Redis**: Cache e sessões
- **Computação Transparente**: Logs transparentes + Threshold signatures
- **Criptografia**: AES-256, RSA-4096, Argon2
- **Zero-Knowledge Proofs**: Privacidade dos votos

## 📋 Pré-requisitos

- Rust 1.75+
- PostgreSQL 15+
- Redis 7+
- Docker (opcional)

## 🛠️ Instalação

### Desenvolvimento Local

1. **Clone o repositório**
   ```bash
   git clone https://github.com/fortis-gov/fortis.git
   cd fortis/backend
   ```

2. **Instale as dependências**
   ```bash
   cargo build
   ```

3. **Configure o ambiente**
   ```bash
   cp env.example .env
   # Edite o arquivo .env com suas configurações
   ```

4. **Execute as migrações**
   ```bash
   # Certifique-se de que o PostgreSQL está rodando
   cargo run --bin migrate
   ```

5. **Inicie o servidor**
   ```bash
   cargo run
   ```

### Docker

1. **Execute com Docker Compose**
   ```bash
   docker-compose up -d
   ```

## 🔧 Configuração

### Variáveis de Ambiente

| Variável | Descrição | Padrão |
|----------|-----------|---------|
| `SERVER_HOST` | Host do servidor | `0.0.0.0` |
| `SERVER_PORT` | Porta do servidor | `8080` |
| `DATABASE_URL` | URL do PostgreSQL | `postgresql://fortis:password@localhost:5432/fortis` |
| `REDIS_URL` | URL do Redis | `redis://localhost:6379` |
| `JWT_SECRET` | Chave secreta JWT | `fortis-super-secret-key` |
| `ENCRYPTION_KEY` | Chave de criptografia | `fortis-encryption-key-32-chars` |

### Banco de Dados

O FORTIS usa PostgreSQL com as seguintes tabelas principais:

- `elections` - Eleições
- `candidates` - Candidatos
- `voters` - Eleitores (dados criptografados)
- `votes` - Votos (metadados e voto criptografado)
- `auditors` - Auditores públicos

## 📚 API

### Endpoints Principais

#### Autenticação
- `POST /api/v1/auth/login` - Login com biometria
- `POST /api/v1/auth/refresh` - Refresh token
- `POST /api/v1/auth/logout` - Logout
- `POST /api/v1/auth/verify` - Verificar token

#### Eleições
- `GET /api/v1/elections` - Listar eleições
- `POST /api/v1/elections` - Criar eleição
- `GET /api/v1/elections/{id}` - Obter eleição
- `PUT /api/v1/elections/{id}` - Atualizar eleição
- `DELETE /api/v1/elections/{id}` - Deletar eleição

#### Votos
- `POST /api/v1/votes` - Votar
- `GET /api/v1/votes/{election_id}` - Obter votos
- `GET /api/v1/votes/verify/{vote_id}` - Verificar voto
- `GET /api/v1/votes/audit/{election_id}` - Auditoria

#### Nós Distribuídos
- `GET /api/v1/nodes` - Listar nós
- `POST /api/v1/nodes` - Registrar nó
- `GET /api/v1/nodes/{id}` - Obter nó
- `PUT /api/v1/nodes/{id}` - Atualizar nó
- `DELETE /api/v1/nodes/{id}` - Remover nó

#### Auditoria
- `GET /api/v1/audit` - Listar auditorias
- `POST /api/v1/audit` - Criar auditoria
- `GET /api/v1/audit/{id}` - Obter auditoria
- `POST /api/v1/audit/{id}/verify` - Verificar auditoria

### Health Checks

- `GET /health` - Status do serviço
- `GET /health/ready` - Prontidão do serviço

## 🔒 Segurança

### Criptografia

- **AES-256-GCM**: Criptografia de dados sensíveis
- **RSA-4096**: Assinaturas digitais
- **Argon2**: Hash de senhas
- **SHA-256**: Hash de identificadores

### Autenticação

- **JWT**: Tokens de acesso
- **Biometria**: Impressão digital + facial
- **Certificado Digital**: Validação TSE
- **Multi-Factor**: Múltiplos fatores de autenticação

### Auditoria

- **Logs Imutáveis**: Todos os eventos são registrados
- **Logs Transparentes**: Hash das transações
- **IPFS**: Armazenamento distribuído
- **Merkle Trees**: Verificação de integridade

## 🧪 Testes

### Executar Testes

```bash
# Todos os testes
cargo test

# Testes específicos
cargo test auth
cargo test crypto
cargo test transparency

# Testes de performance
cargo test --release --test performance
```

### Cobertura de Testes

```bash
# Instalar cargo-tarpaulin
cargo install cargo-tarpaulin

# Executar cobertura
cargo tarpaulin --out Html
```

## 📊 Monitoramento

### Métricas

- **Performance**: Tempo de resposta, throughput
- **Segurança**: Tentativas de login, falhas de autenticação
- **Sistema**: CPU, memória, disco
- **Banco de Dados**: Conexões, queries lentas

### Logs

- **Estruturados**: JSON format
- **Níveis**: ERROR, WARN, INFO, DEBUG, TRACE
- **Contexto**: Request ID, usuário, eleição

## 🚀 Deploy

### Desenvolvimento

```bash
cargo run
```

### Staging

```bash
cargo run --release
```

### Produção

```bash
docker-compose -f docker-compose.prod.yml up -d
```

## 🤝 Contribuição

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/nova-funcionalidade`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/nova-funcionalidade`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](../../LICENSE) para detalhes.

## 🆘 Suporte

- **Documentação**: [docs.fortis.gov.br](https://docs.fortis.gov.br)
- **Issues**: [GitHub Issues](https://github.com/fortis-gov/fortis/issues)
- **Email**: dev@fortis.gov.br

---

**🇧🇷 Democracia transparente, segura e brasileira.**
