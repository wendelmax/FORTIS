# FORTIS - Estratégia de Testes e Qualidade
## Test Writer & Fixer Perspective

### 🎯 **Visão Geral da Estratégia de Testes**

O FORTIS implementa uma estratégia de testes abrangente e multi-camada, garantindo a qualidade, segurança e confiabilidade do sistema de votação eletrônica através de testes automatizados, manuais e de segurança.

---

## 🧪 **Pirâmide de Testes**

### **Estrutura de Testes**
```
┌─────────────────────────────────────────────────────────┐
│                    TEST PYRAMID                        │
├─────────────────────────────────────────────────────────┤
│ E2E Tests (10%) - Testes de ponta a ponta              │
│ Integration Tests (20%) - Testes de integração         │
│ Unit Tests (70%) - Testes unitários                    │
└─────────────────────────────────────────────────────────┘
```

### **Cobertura de Testes por Camada**
- **Unitários**: 90%+ de cobertura
- **Integração**: 80%+ de cobertura
- **E2E**: 100% dos fluxos críticos
- **Segurança**: 100% das vulnerabilidades conhecidas

---

## 🔧 **Testes Unitários**

### **Backend (Rust)**
```rust
// tests/unit/auth_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use mockito::mock;

    #[test]
    async fn test_biometric_authentication_success() {
        // Arrange
        let auth_service = AuthService::new();
        let biometric_data = BiometricData::new(
            "test_fingerprint_data".to_string(),
            BiometricType::Fingerprint
        );
        let cpf = "12345678901";
        
        // Mock do banco de dados
        let mock_db = MockDatabase::new();
        mock_db.expect_find_user_by_cpf()
            .with(eq(cpf))
            .times(1)
            .returning(|_| Ok(Some(create_test_user())));
        
        // Act
        let result = auth_service.authenticate_biometric(&biometric_data, cpf).await;
        
        // Assert
        assert!(result.is_ok());
        let auth_result = result.unwrap();
        assert!(auth_result.authenticated);
        assert!(auth_result.confidence > 0.8);
    }

    #[test]
    async fn test_biometric_authentication_failure() {
        // Arrange
        let auth_service = AuthService::new();
        let invalid_biometric = BiometricData::new(
            "invalid_data".to_string(),
            BiometricType::Fingerprint
        );
        let cpf = "12345678901";
        
        // Act
        let result = auth_service.authenticate_biometric(&invalid_biometric, cpf).await;
        
        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::AuthenticationFailed);
    }

    #[test]
    async fn test_vote_encryption() {
        // Arrange
        let crypto_service = CryptoService::new();
        let vote = Vote {
            id: Uuid::new_v4(),
            election_id: "test_election".to_string(),
            candidate_id: "candidate_1".to_string(),
            timestamp: Utc::now(),
        };
        
        // Act
        let encrypted_vote = crypto_service.encrypt_vote(&vote).await?;
        let decrypted_vote = crypto_service.decrypt_vote(&encrypted_vote).await?;
        
        // Assert
        assert_eq!(vote.id, decrypted_vote.id);
        assert_eq!(vote.candidate_id, decrypted_vote.candidate_id);
        assert_ne!(vote.timestamp, decrypted_vote.timestamp); // Timestamp pode variar
    }

    #[test]
    async fn test_fraud_detection() {
        // Arrange
        let fraud_detector = FraudDetector::new();
        let normal_vote = create_normal_vote();
        let suspicious_vote = create_suspicious_vote();
        
        // Act
        let normal_result = fraud_detector.detect_fraud(&normal_vote).await;
        let suspicious_result = fraud_detector.detect_fraud(&suspicious_vote).await;
        
        // Assert
        assert!(normal_result.is_ok());
        assert!(suspicious_result.is_ok());
        
        let normal_fraud = normal_result.unwrap();
        let suspicious_fraud = suspicious_result.unwrap();
        
        assert!(normal_fraud.fraud_probability < 0.1);
        assert!(suspicious_fraud.fraud_probability > 0.8);
    }
}
```

### **Frontend Administrativo (TypeScript)**
```typescript
// tests/unit/ExecutiveDashboard.test.tsx
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { ExecutiveDashboard } from '../ExecutiveDashboard';
import { useElectionData } from '../hooks/useElectionData';
import { useSystemMetrics } from '../hooks/useSystemMetrics';

// Mock dos hooks
jest.mock('../hooks/useElectionData');
jest.mock('../hooks/useSystemMetrics');

describe('ExecutiveDashboard', () => {
  beforeEach(() => {
    (useElectionData as jest.Mock).mockReturnValue({
      elections: [],
      loading: false,
    });
    (useSystemMetrics as jest.Mock).mockReturnValue({
      metrics: {},
      loading: false,
    });
  });

  it('should render dashboard overview initially', () => {
    render(<ExecutiveDashboard userRole="admin" />);
    
    expect(screen.getByText('FORTIS Admin')).toBeInTheDocument();
    expect(screen.getByText('Sistema de Gestão Eleitoral')).toBeInTheDocument();
  });

  it('should switch to elections tab', async () => {
    render(<ExecutiveDashboard userRole="admin" />);
    
    const electionsTab = screen.getByText('🗳️ Eleições');
    fireEvent.click(electionsTab);
    
    await waitFor(() => {
      expect(screen.getByText('Gestão de Eleições')).toBeInTheDocument();
    });
  });

  it('should switch to nodes tab', async () => {
    render(<ExecutiveDashboard userRole="admin" />);
    
    const nodesTab = screen.getByText('🌐 Nós Distribuídos');
    fireEvent.click(nodesTab);
    
    await waitFor(() => {
      expect(screen.getByText('Nós Distribuídos')).toBeInTheDocument();
    });
  });

  it('should create new election', async () => {
    const mockCreateElection = jest.fn().mockResolvedValue({ id: 'election-123' });
    
    render(<ExecutiveDashboard userRole="admin" />);
    
    const electionsTab = screen.getByText('🗳️ Eleições');
    fireEvent.click(electionsTab);
    
    const createButton = screen.getByText('Nova Eleição');
    fireEvent.click(createButton);
    
    await waitFor(() => {
      expect(screen.getByText('Configuração da Eleição')).toBeInTheDocument();
    });
  });

  it('should manage distributed nodes', async () => {
    render(<ExecutiveDashboard userRole="admin" />);
    
    const nodesTab = screen.getByText('🌐 Nós Distribuídos');
    fireEvent.click(nodesTab);
    
    await waitFor(() => {
      expect(screen.getByText('Lista')).toBeInTheDocument();
      expect(screen.getByText('Topologia')).toBeInTheDocument();
      expect(screen.getByText('Configuração')).toBeInTheDocument();
    });
  });

  it('should handle ministerial approval workflow', async () => {
    render(<ExecutiveDashboard userRole="minister" />);
    
    await waitFor(() => {
      expect(screen.getByText('Aprovações Pendentes')).toBeInTheDocument();
    });
  });
});
```

---

## 🔗 **Testes de Integração**

### **API Integration Tests**
```rust
// tests/integration/api_test.rs
use actix_web::{test, web, App};
use serde_json::json;

#[actix_web::test]
async fn test_vote_endpoint_success() {
    // Arrange
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(create_test_app_state()))
            .service(vote_routes())
    ).await;
    
    let vote_request = json!({
        "election_id": "test_election",
        "candidate_id": "candidate_1",
        "biometric_data": "test_biometric",
        "proof": "test_proof"
    });
    
    // Act
    let req = test::TestRequest::post()
        .uri("/api/v1/vote")
        .set_json(&vote_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), 200);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["success"].as_bool().unwrap());
    assert!(body["transaction_hash"].is_string());
}

#[actix_web::test]
async fn test_vote_endpoint_invalid_biometric() {
    // Arrange
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(create_test_app_state()))
            .service(vote_routes())
    ).await;
    
    let vote_request = json!({
        "election_id": "test_election",
        "candidate_id": "candidate_1",
        "biometric_data": "invalid_biometric",
        "proof": "test_proof"
    });
    
    // Act
    let req = test::TestRequest::post()
        .uri("/api/v1/vote")
        .set_json(&vote_request)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), 401);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "Invalid biometric data");
}

#[actix_web::test]
async fn test_audit_endpoint() {
    // Arrange
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(create_test_app_state()))
            .service(audit_routes())
    ).await;
    
    // Act
    let req = test::TestRequest::get()
        .uri("/api/v1/audit/election/test_election")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), 200);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["election_id"].is_string());
    assert!(body["total_votes"].is_number());
    assert!(body["audit_logs"].is_array());
}
```

### **Database Integration Tests**
```rust
// tests/integration/database_test.rs
use sqlx::PgPool;
use tokio::test;

#[test]
async fn test_vote_persistence() {
    // Arrange
    let pool = create_test_database().await;
    let vote_repository = VoteRepository::new(pool.clone());
    
    let vote = Vote {
        id: Uuid::new_v4(),
        election_id: "test_election".to_string(),
        candidate_id: "candidate_1".to_string(),
        voter_id: "voter_1".to_string(),
        timestamp: Utc::now(),
    };
    
    // Act
    let result = vote_repository.save_vote(&vote).await;
    
    // Assert
    assert!(result.is_ok());
    
    let saved_vote = vote_repository.get_vote_by_id(vote.id).await.unwrap();
    assert_eq!(saved_vote.id, vote.id);
    assert_eq!(saved_vote.candidate_id, vote.candidate_id);
}

#[test]
async fn test_vote_immutability() {
    // Arrange
    let pool = create_test_database().await;
    let vote_repository = VoteRepository::new(pool.clone());
    
    let vote = create_test_vote();
    vote_repository.save_vote(&vote).await.unwrap();
    
    // Act - Tentar atualizar voto existente
    let mut updated_vote = vote.clone();
    updated_vote.candidate_id = "candidate_2".to_string();
    
    let result = vote_repository.update_vote(&updated_vote).await;
    
    // Assert - Deve falhar (votos são imutáveis)
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DatabaseError::VoteImmutable);
}
```

---

## 🌐 **Testes End-to-End**

### **Playwright E2E Tests**
```typescript
// tests/e2e/voting-flow.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Voting Flow', () => {
  test('complete voting process', async ({ page }) => {
    // Navegar para a página de votação
    await page.goto('/vote/election-2025');
    
    // Verificar se a página carregou
    await expect(page.locator('h1')).toContainText('Votação Eletrônica');
    
    // Iniciar autenticação biométrica
    await page.click('button[data-testid="start-biometric-auth"]');
    
    // Simular captura biométrica
    await page.setInputFiles('input[type="file"]', 'test-data/fingerprint.jpg');
    await page.click('button[data-testid="submit-biometric"]');
    
    // Aguardar autenticação
    await expect(page.locator('[data-testid="auth-success"]')).toBeVisible();
    
    // Selecionar candidato
    await page.click('[data-testid="candidate-card-12345"]');
    await expect(page.locator('[data-testid="candidate-selected"]')).toBeVisible();
    
    // Confirmar voto
    await page.click('button[data-testid="confirm-vote"]');
    
    // Verificar confirmação
    await expect(page.locator('[data-testid="vote-confirmed"]')).toBeVisible();
    
    // Verificar comprovante
    await expect(page.locator('[data-testid="vote-receipt"]')).toBeVisible();
    const receiptHash = await page.locator('[data-testid="receipt-hash"]').textContent();
    expect(receiptHash).toMatch(/^[a-f0-9]{64}$/); // Hash SHA-256
  });

  test('voting with invalid biometric', async ({ page }) => {
    await page.goto('/vote/election-2025');
    
    // Tentar autenticação com dados inválidos
    await page.click('button[data-testid="start-biometric-auth"]');
    await page.setInputFiles('input[type="file"]', 'test-data/invalid-fingerprint.jpg');
    await page.click('button[data-testid="submit-biometric"]');
    
    // Verificar erro
    await expect(page.locator('[data-testid="auth-error"]')).toBeVisible();
    await expect(page.locator('[data-testid="auth-error"]')).toContainText('Falha na autenticação biométrica');
  });

  test('voting outside election hours', async ({ page }) => {
    // Mock de data/hora fora do período eleitoral
    await page.addInitScript(() => {
      const mockDate = new Date('2025-01-01T10:00:00Z'); // Fora do horário
      Date.now = () => mockDate.getTime();
    });
    
    await page.goto('/vote/election-2025');
    
    // Verificar mensagem de erro
    await expect(page.locator('[data-testid="election-closed"]')).toBeVisible();
    await expect(page.locator('[data-testid="election-closed"]')).toContainText('Votação não está disponível neste momento');
  });
});
```

### **API E2E Tests**
```typescript
// tests/e2e/api-e2e.spec.ts
import { test, expect } from '@playwright/test';

test.describe('API E2E Tests', () => {
  test('complete voting API flow', async ({ request }) => {
    // 1. Autenticar usuário
    const authResponse = await request.post('/api/v1/auth/biometric', {
      data: {
        cpf: '12345678901',
        biometric_data: 'test_biometric_data'
      }
    });
    
    expect(authResponse.ok()).toBeTruthy();
    const authData = await authResponse.json();
    expect(authData.token).toBeDefined();
    
    // 2. Obter lista de candidatos
    const candidatesResponse = await request.get('/api/v1/elections/test_election/candidates', {
      headers: {
        'Authorization': `Bearer ${authData.token}`
      }
    });
    
    expect(candidatesResponse.ok()).toBeTruthy();
    const candidates = await candidatesResponse.json();
    expect(candidates.length).toBeGreaterThan(0);
    
    // 3. Votar
    const voteResponse = await request.post('/api/v1/vote', {
      headers: {
        'Authorization': `Bearer ${authData.token}`
      },
      data: {
        election_id: 'test_election',
        candidate_id: candidates[0].id,
        proof: 'test_proof'
      }
    });
    
    expect(voteResponse.ok()).toBeTruthy();
    const voteData = await voteResponse.json();
    expect(voteData.transaction_hash).toBeDefined();
    
    // 4. Verificar voto
    const verifyResponse = await request.get(`/api/v1/vote/${voteData.transaction_hash}/verify`);
    expect(verifyResponse.ok()).toBeTruthy();
    const verifyData = await verifyResponse.json();
    expect(verifyData.verified).toBe(true);
  });
});
```

---

## 🔒 **Testes de Segurança**

### **Penetration Testing**
```python
# tests/security/penetration_test.py
import requests
import pytest
from security_test_utils import SecurityTester

class TestSecurity:
    def setup_method(self):
        self.tester = SecurityTester()
        self.base_url = "https://fortis.gov.br/api/v1"
    
    def test_sql_injection_protection(self):
        """Teste de proteção contra SQL injection"""
        malicious_payloads = [
            "'; DROP TABLE votes; --",
            "1' OR '1'='1",
            "admin'--",
            "1' UNION SELECT * FROM users--"
        ]
        
        for payload in malicious_payloads:
            response = self.tester.test_endpoint(
                f"{self.base_url}/vote",
                method="POST",
                data={"candidate_id": payload}
            )
            
            # Deve retornar erro 400, não 500
            assert response.status_code == 400
            assert "SQL" not in response.text
    
    def test_xss_protection(self):
        """Teste de proteção contra XSS"""
        xss_payloads = [
            "<script>alert('XSS')</script>",
            "javascript:alert('XSS')",
            "<img src=x onerror=alert('XSS')>",
            "';alert('XSS');//"
        ]
        
        for payload in xss_payloads:
            response = self.tester.test_endpoint(
                f"{self.base_url}/vote",
                method="POST",
                data={"candidate_id": payload}
            )
            
            # Deve escapar ou rejeitar o payload
            assert payload not in response.text
    
    def test_authentication_bypass(self):
        """Teste de bypass de autenticação"""
        # Tentar acessar endpoint protegido sem token
        response = requests.post(f"{self.base_url}/vote", json={
            "election_id": "test",
            "candidate_id": "candidate_1"
        })
        
        assert response.status_code == 401
        assert "token" in response.json()["error"].lower()
    
    def test_rate_limiting(self):
        """Teste de rate limiting"""
        # Fazer muitas requisições rapidamente
        for i in range(100):
            response = requests.post(f"{self.base_url}/vote", json={
                "election_id": "test",
                "candidate_id": "candidate_1"
            })
            
            if i > 10:  # Após 10 requisições
                assert response.status_code == 429  # Too Many Requests
    
    def test_encryption_validation(self):
        """Teste de validação de criptografia"""
        # Verificar se dados sensíveis estão criptografados
        response = requests.get(f"{self.base_url}/audit/election/test")
        
        assert response.status_code == 200
        data = response.json()
        
        # Verificar se hashes estão presentes (dados criptografados)
        for vote in data["votes"]:
            assert "hash" in vote
            assert "encrypted_data" in vote
            assert len(vote["hash"]) == 64  # SHA-256
```

### **Security Headers Testing**
```python
# tests/security/headers_test.py
def test_security_headers():
    """Teste de headers de segurança"""
    response = requests.get("https://fortis.gov.br")
    
    # Verificar headers de segurança
    assert response.headers.get("X-Content-Type-Options") == "nosniff"
    assert response.headers.get("X-Frame-Options") == "DENY"
    assert response.headers.get("X-XSS-Protection") == "1; mode=block"
    assert response.headers.get("Strict-Transport-Security") is not None
    assert response.headers.get("Content-Security-Policy") is not None
    assert response.headers.get("Referrer-Policy") is not None
```

---

## 📊 **Testes de Performance**

### **Load Testing**
```python
# tests/performance/load_test.py
import asyncio
import aiohttp
import time
from statistics import mean, median

class LoadTester:
    def __init__(self, base_url: str):
        self.base_url = base_url
        self.results = []
    
    async def test_concurrent_votes(self, num_users: int, votes_per_user: int):
        """Teste de votação concorrente"""
        async with aiohttp.ClientSession() as session:
            tasks = []
            
            for user_id in range(num_users):
                for vote_id in range(votes_per_user):
                    task = self.simulate_vote(session, user_id, vote_id)
                    tasks.append(task)
            
            start_time = time.time()
            results = await asyncio.gather(*tasks, return_exceptions=True)
            end_time = time.time()
            
            # Analisar resultados
            successful_votes = [r for r in results if isinstance(r, dict) and r.get("success")]
            failed_votes = [r for r in results if isinstance(r, Exception)]
            
            print(f"Votos bem-sucedidos: {len(successful_votes)}")
            print(f"Votos falharam: {len(failed_votes)}")
            print(f"Tempo total: {end_time - start_time:.2f}s")
            print(f"Votos por segundo: {len(successful_votes) / (end_time - start_time):.2f}")
    
    async def simulate_vote(self, session, user_id: int, vote_id: int):
        """Simular um voto individual"""
        start_time = time.time()
        
        try:
            # 1. Autenticar
            auth_response = await session.post(f"{self.base_url}/auth/biometric", json={
                "cpf": f"1234567890{user_id}",
                "biometric_data": f"test_biometric_{user_id}"
            })
            
            if auth_response.status != 200:
                return Exception(f"Auth failed: {auth_response.status}")
            
            auth_data = await auth_response.json()
            token = auth_data["token"]
            
            # 2. Votar
            vote_response = await session.post(f"{self.base_url}/vote", 
                headers={"Authorization": f"Bearer {token}"},
                json={
                    "election_id": "test_election",
                    "candidate_id": f"candidate_{vote_id % 5}",
                    "proof": f"proof_{vote_id}"
                }
            )
            
            end_time = time.time()
            
            if vote_response.status == 200:
                return {
                    "success": True,
                    "response_time": end_time - start_time,
                    "user_id": user_id,
                    "vote_id": vote_id
                }
            else:
                return Exception(f"Vote failed: {vote_response.status}")
                
        except Exception as e:
            return e

# Executar teste de carga
async def main():
    tester = LoadTester("https://fortis.gov.br/api/v1")
    await tester.test_concurrent_votes(num_users=100, votes_per_user=10)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 📈 **Métricas de Qualidade**

### **Code Coverage**
```yaml
# .github/workflows/coverage.yml
name: Code Coverage

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  coverage:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Run tests with coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Html --output-dir coverage
    
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./coverage/cobertura.xml
        flags: unittests
        name: codecov-umbrella
```

### **Quality Gates**
```yaml
# quality-gates.yml
quality_gates:
  coverage:
    minimum: 90
    target: 95
  
  performance:
    response_time_p95: 200ms
    response_time_p99: 500ms
    throughput: 1000_rps
  
  security:
    vulnerabilities: 0
    security_score: A
  
  reliability:
    availability: 99.9%
    error_rate: 0.1%
```

---

## 🎯 **Próximos Passos**

### **Fase 1: Testes Base (2 meses)**
- [ ] Implementar testes unitários
- [ ] Configurar CI/CD com testes
- [ ] Testes de integração básicos
- [ ] Cobertura de código 90%+

### **Fase 2: Testes Avançados (2 meses)**
- [ ] Testes E2E completos
- [ ] Testes de segurança
- [ ] Testes de performance
- [ ] Testes de acessibilidade

### **Fase 3: Produção (2 meses)**
- [ ] Testes em ambiente de produção
- [ ] Monitoramento de qualidade
- [ ] Testes de regressão
- [ ] Otimização contínua

---

*Documentação de Testes FORTIS - Desenvolvida pelo Test Writer & Fixer Agent*
