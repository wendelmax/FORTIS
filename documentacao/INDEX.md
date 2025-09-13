# FORTIS
- Índice da Documentação

## Sistema de Votação Eletrônica Brasileiro

### **Navegação Rápida** Este índice organiza toda a documentação técnica do FORTIS para facilitar a navegação e consulta. ---

## **Documentação Principal**

### **Visão Geral do Sistema**
- **[Big Picture](./FORTIS_BIG_PICTURE.md)**
- Visão completa da arquitetura e componentes
- **[Organização Completa](./ORGANIZACAO_COMPLETA.md)**
- Estrutura do projeto e organização
- **[Análise Inicial](./ANALISE_INICIAL_FORTIS.md)**
- Análise técnica inicial do sistema ---

## **APIs e Integração**

### **Especificações de API**
- **[README APIs](./apis/README.md)**
- Especificações gerais das APIs
- **[Análise de Endpoints](./apis/ENDPOINTS_ANALYSIS.md)**
- Análise completa de cobertura de endpoints
- **[Autenticação](./apis/auth.md)**
- Especificações de autenticação e autorização
- **[Votação](./apis/voting.md)**
- APIs de sistema de votação
- **[Auditoria](./apis/audit.md)**
- APIs de auditoria e transparência
- **[Integração TSE](./apis/tse.md)**
- Integração com TSE e Gov.br

### **Documentação Swagger**
- ** Swagger UI**: `http://localhost:8080/swagger-ui/`
- ** OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json` ---

## **Arquitetura e Implementação**

### **Backend (Rust)**
- **[ Backend README](../backend/README.md)**
- Documentação do backend Rust
- **[ Configuração](../backend/src/config.rs)**
- Configurações do sistema
- **[ Banco de Dados](../backend/src/database.rs)**
- Estrutura do banco de dados
- **[ Criptografia](../backend/src/crypto.rs)**
- Módulo de criptografia

### **Frontend (React)**
- **[ Frontend README](../frontend/README.md)**
- Documentação do frontend React
- **[ Componentes](../frontend/src/components/)**
- Componentes da interface

### **Mobile (React Native)**
- **[ Mobile README](../mobile/README.md)**
- Documentação do app mobile
- **[ Screens](../mobile/src/screens/)**
- Telas do aplicativo ---

## **Blockchain e Smart Contracts**

### **Contratos Inteligentes**
- **[ Blockchain README](../blockchain/README.md)**
- Documentação da blockchain
- **[ FortisVoting.sol](../blockchain/contracts/FortisVoting.sol)**
- Contrato principal
- **[ FortisGovernance.sol](../blockchain/contracts/FortisGovernance.sol)**
- Governança

### **Zero-Knowledge Proofs**
- **[ ZKP README](../backend/src/zkp/README.md)**
- Documentação de ZKP
- **[ Circuitos](../backend/src/zkp/circuits.rs)**
- Circuitos de prova
- **[ Verificação](../backend/src/zkp/verifier.rs)**
- Verificação de provas ---

## **Inteligência Artificial**

### **IA e Analytics**
- **[ AI README](../ai/README.md)**
- Documentação de IA
- **[🧠 Modelos](../ai/data/models/)**
- Modelos de machine learning
- **[ Analytics](../analytics/README.md)**
- Sistema de analytics
- **[ Dashboards](../analytics/dashboards/)**
- Dashboards interativos

### **Integração LLM**
- **[ LLM Integration](../ai/docs/llm_integration.md)**
- Integração com modelos locais
- **[ Assistente Eleitoral](../ai/services/chat_service.py)**
- Chatbot eleitoral ---

## **Infraestrutura e DevOps**

### **Kubernetes**
- **[ Infrastructure README](../infrastructure/README.md)**
- Documentação de infraestrutura
- **[ Kubernetes Base](../infrastructure/k8s/base/)**
- Configurações base
- **[ Overlays](../infrastructure/k8s/overlays/)**
- Configurações por ambiente

### **Monitoramento**
- **[ Prometheus](../infrastructure/monitoring/prometheus/)**
- Métricas
- **[ Grafana](../infrastructure/monitoring/grafana/)**
- Dashboards
- **[ Logs](../infrastructure/monitoring/logs/)**
- Sistema de logs ---

## **Segurança**

### **Criptografia e Autenticação**
- **[ Security README](../security/README.md)**
- Documentação de segurança
- **[ Certificados](../security/certificates/)**
- Gestão de certificados
- **[ Firewall](../security/firewall/)**
- Configurações de rede

### **Auditoria**
- **[ Audit System](../backend/src/services/audit/)**
- Sistema de auditoria
- **[ Verificação](../backend/src/services/audit/verification.rs)**
- Verificação de integridade ---

## **Testes e Qualidade**

### **Testes Automatizados**
- **[ Tests README](../tests/README.md)**
- Documentação de testes
- **[ Unit Tests](../tests/unit/)**
- Testes unitários
- **[ Integration Tests](../tests/integration/)**
- Testes de integração
- **[ E2E Tests](../tests/e2e/)**
- Testes end-to-end

### **Qualidade de Código**
- **[ Coverage](../tests/coverage/)**
- Relatórios de cobertura
- **[ Linting](../tests/linting/)**
- Análise de código ---

## **Guias e Tutoriais**

### **Guias de Desenvolvimento**
- **[ Getting Started](./GETTING_STARTED.md)**
- Guia de início rápido
- **[ Development Guide](./DEVELOPMENT_GUIDE.md)**
- Guia de desenvolvimento
- **[ Deployment Guide](./DEPLOYMENT_GUIDE.md)**
- Guia de deploy

### **Tutoriais**
- **[ Mobile App Tutorial](./TUTORIAL_MOBILE.md)**
- Tutorial do app mobile
- **[ Web App Tutorial](./TUTORIAL_WEB.md)**
- Tutorial da aplicação web
- **[ Blockchain Tutorial](./TUTORIAL_BLOCKCHAIN.md)**
- Tutorial de blockchain ---

## **Métricas e Performance**

### **Benchmarks**
- **[ Performance Tests](../tests/performance/)**
- Testes de performance
- **[ Load Tests](../tests/load/)**
- Testes de carga
- **[ Security Tests](../tests/security/)**
- Testes de segurança

### **Métricas de Qualidade (Conceitual)**
- **Arquitetura**: Robusta e escalável
- **Performance**: Otimizada para eleições
- **Disponibilidade**: Alta disponibilidade planejada
- **Segurança**: Múltiplas camadas de proteção > **Nota**: Métricas específicas serão definidas após implementação ---

## **Suporte e Contato**

### **Documentação Técnica**
- **Issues**: [GitHub Issues](https://github.com/fortis-gov/fortis/issues)
- **Discussions**: [GitHub Discussions](https://github.com/fortis-gov/fortis/discussions)
- **Wiki**: [GitHub Wiki](https://github.com/fortis-gov/fortis/wiki)

### **Contato da Equipe**
- **Email**: dev@fortis.gov.br
- **Slack**:

#fortis-development
- **Discord**: FORTIS Development ---

## **Changelog e Versões**

### **Histórico de Versões**
- **[ CHANGELOG.md](./CHANGELOG.md)**
- Histórico de mudanças
- **[ Releases](https://github.com/fortis-gov/fortis/releases)**
- Releases no GitHub
- **[ Roadmap](./ROADMAP.md)**
- Roadmap de desenvolvimento ---

## **Status do Projeto**

### **Fase Embrionária
- Desenvolvimento Conceitual**
- **Backend**: Arquitetura definida
- **Frontend**: Conceito definido
- **Mobile**: Planejamento inicial
- **Blockchain**: Arquitetura definida
- **IA/Analytics**: Conceito com Ollama + Llama3.2
- **Infraestrutura**: Arquitetura definida
- **Documentação**: 100% Conceitual

### **Próximos Passos** 1. **Validação Conceitual**
- Aprovação com TSE 2. **Desenvolvimento de MVP**
- Protótipo funcional 3. **Estudos de Viabilidade**
- Análise técnica e econômica 4. **Definição de Recursos**
- Cronogramas e investimentos --- *Documentação criada em: 2025* *Versão: 1.2
- Índice Conceitual
- Fase Embrionária* *Sistema: FORTIS
- Votação Eletrônica Brasileira* ---

## **Nota
- Fase Embrionária** Este índice apresenta a documentação **conceitual e arquitetural** do FORTIS em sua fase embrionária. **Objetivos desta fase:**
- Definir arquitetura tecnológica robusta
- Validar conceitos com stakeholders
- Estabelecer fundamentos técnicos sólidos
- Preparar para próximas fases de desenvolvimento 