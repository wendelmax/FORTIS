# FORTIS Mobile App

Sistema de Votação Eletrônica Seguro para dispositivos móveis.

## 🚀 Características

- **Segurança Máxima**: Criptografia de ponta a ponta e autenticação biométrica
- **Transparência Total**: Blockchain para auditoria e verificação
- **Privacidade Garantida**: Zero-Knowledge Proofs para proteção dos dados
- **Interface Intuitiva**: Design moderno e acessível
- **Offline First**: Funciona mesmo com conexão instável

## 🛠️ Tecnologias

- **React Native 0.72.6**
- **TypeScript**
- **React Navigation 6**
- **React Native Paper** (Material Design)
- **React Query** (Gerenciamento de estado)
- **Zustand** (Estado global)
- **React Native Biometrics** (Autenticação biométrica)
- **React Native Keychain** (Armazenamento seguro)

## 📱 Funcionalidades

### Autenticação
- Login com CPF e senha
- Autenticação biométrica (impressão digital, face, voz)
- Verificação de segurança do dispositivo
- Integração com Gov.br

### Votação
- Lista de eleições ativas
- Interface de votação intuitiva
- Confirmação de voto
- Comprovante de votação
- Verificação de integridade

### Segurança
- Verificação de dispositivo seguro
- Detecção de root/jailbreak
- Criptografia de dados
- Logs de auditoria

### Interface
- Design responsivo
- Acessibilidade completa
- Modo escuro/claro
- Feedback háptico
- Efeitos sonoros

## 🚀 Instalação

### Pré-requisitos

- Node.js 16+
- React Native CLI
- Android Studio (para Android)
- Xcode (para iOS)
- Java 11+

### Instalação

```bash
# Instalar dependências
npm install

# iOS (apenas no macOS)
cd ios && pod install && cd ..

# Executar no Android
npm run android

# Executar no iOS
npm run ios
```

## 🔧 Configuração

### Variáveis de Ambiente

Crie um arquivo `.env` na raiz do projeto:

```env
API_BASE_URL=http://localhost:8080
BLOCKCHAIN_RPC_URL=https://polygon-mumbai.g.alchemy.com/v2/demo
CONTRACT_ADDRESS=0x1234567890123456789012345678901234567890
```

### Configuração de Desenvolvimento

```bash
# Modo desenvolvimento
npm start

# Limpar cache
npm run clean

# Executar testes
npm test

# Lint
npm run lint
```

## 📱 Build de Produção

### Android

```bash
# Build de debug
npm run build:android

# Build de release
cd android
./gradlew assembleRelease
```

### iOS

```bash
# Build de release
npm run build:ios
```

## 🔒 Segurança

### Verificações Implementadas

- **Dispositivo Seguro**: Verifica se o dispositivo não está rootado/jailbroken
- **Versão do Sistema**: Requer versões mínimas do Android/iOS
- **Patch de Segurança**: Verifica se o dispositivo está atualizado
- **Emulador**: Impede execução em emuladores
- **Debug**: Desabilita modo debug em produção

### Criptografia

- **AES-256-GCM**: Para criptografia de dados
- **RSA-4096**: Para assinaturas digitais
- **Argon2**: Para hash de senhas
- **Zero-Knowledge Proofs**: Para privacidade dos votos

## 🧪 Testes

```bash
# Executar todos os testes
npm test

# Testes unitários
npm run test:unit

# Testes de integração
npm run test:integration

# Testes E2E
npm run test:e2e
```

## 📊 Monitoramento

### Métricas Coletadas

- Tempo de votação
- Taxa de sucesso
- Erros de autenticação
- Performance do app
- Uso de recursos

### Logs

- Logs de auditoria
- Logs de segurança
- Logs de erro
- Logs de performance

## 🚀 Deploy

### Android (Google Play)

1. Gerar keystore de release
2. Configurar assinatura
3. Build de release
4. Upload para Play Console

### iOS (App Store)

1. Configurar certificados
2. Build de release
3. Upload para App Store Connect

## 📚 Documentação

- [Guia de Desenvolvimento](./docs/development.md)
- [Guia de Segurança](./docs/security.md)
- [Guia de Testes](./docs/testing.md)
- [Guia de Deploy](./docs/deployment.md)

## 🤝 Contribuição

1. Fork o projeto
2. Crie uma branch para sua feature
3. Commit suas mudanças
4. Push para a branch
5. Abra um Pull Request

## 📄 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](../LICENSE) para detalhes.

## 🆘 Suporte

- **Email**: suporte@fortis.gov.br
- **Telefone**: 0800 123 4567
- **Chat**: Disponível no app
- **Site**: www.fortis.gov.br

## 🔄 Changelog

### v1.0.0 (2025-12-15)

- ✅ Autenticação biométrica
- ✅ Interface de votação
- ✅ Sistema de segurança
- ✅ Integração com blockchain
- ✅ Comprovantes de votação
- ✅ Configurações do usuário
- ✅ Central de ajuda

---

**FORTIS** - Sistema de Votação Eletrônica Seguro 🇧🇷
