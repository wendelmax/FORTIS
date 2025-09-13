#!/usr/bin/env python3
"""
OWASP ZAP Security Scanner para FORTIS
Realiza testes de penetração automatizados na API do sistema
"""

import os
import sys
import json
import time
import requests
from datetime import datetime
from zapv2 import ZAPv2

class FortisSecurityScanner:
    def __init__(self, target_url, api_key=None):
        self.target_url = target_url
        self.api_key = api_key
        self.zap = ZAPv2(proxies={'http': 'http://127.0.0.1:8080', 'https': 'http://127.0.0.1:8080'})
        self.results = {
            'scan_id': f"fortis_scan_{int(time.time())}",
            'target': target_url,
            'timestamp': datetime.now().isoformat(),
            'vulnerabilities': [],
            'summary': {}
        }
    
    def start_zap(self):
        """Inicia o ZAP e configura o proxy"""
        print("🔧 Iniciando OWASP ZAP...")
        try:
            # Verifica se o ZAP está rodando
            self.zap.core.version()
            print("✅ ZAP conectado com sucesso")
            return True
        except Exception as e:
            print(f"❌ Erro ao conectar com ZAP: {e}")
            return False
    
    def spider_scan(self):
        """Executa spider scan para descobrir URLs"""
        print("🕷️ Executando spider scan...")
        try:
            scan_id = self.zap.spider.scan(self.target_url)
            
            # Aguarda o scan completar
            while int(self.zap.spider.status(scan_id)) < 100:
                print(f"📊 Spider progress: {self.zap.spider.status(scan_id)}%")
                time.sleep(2)
            
            print("✅ Spider scan concluído")
            return True
        except Exception as e:
            print(f"❌ Erro no spider scan: {e}")
            return False
    
    def active_scan(self):
        """Executa active scan para encontrar vulnerabilidades"""
        print("🔍 Executando active scan...")
        try:
            scan_id = self.zap.ascan.scan(self.target_url)
            
            # Aguarda o scan completar
            while int(self.zap.ascan.status(scan_id)) < 100:
                print(f"📊 Active scan progress: {self.zap.ascan.status(scan_id)}%")
                time.sleep(5)
            
            print("✅ Active scan concluído")
            return True
        except Exception as e:
            print(f"❌ Erro no active scan: {e}")
            return False
    
    def analyze_results(self):
        """Analisa os resultados do scan"""
        print("📊 Analisando resultados...")
        
        try:
            # Obtém alertas por risco
            alerts = self.zap.core.alerts()
            
            risk_levels = {
                'High': [],
                'Medium': [],
                'Low': [],
                'Informational': []
            }
            
            for alert in alerts:
                risk = alert.get('risk', 'Informational')
                if risk in risk_levels:
                    risk_levels[risk].append({
                        'name': alert.get('name', 'Unknown'),
                        'description': alert.get('description', ''),
                        'url': alert.get('url', ''),
                        'solution': alert.get('solution', ''),
                        'reference': alert.get('reference', ''),
                        'cweid': alert.get('cweid', ''),
                        'wascid': alert.get('wascid', '')
                    })
            
            self.results['vulnerabilities'] = risk_levels
            self.results['summary'] = {
                'total_alerts': len(alerts),
                'high_risk': len(risk_levels['High']),
                'medium_risk': len(risk_levels['Medium']),
                'low_risk': len(risk_levels['Low']),
                'informational': len(risk_levels['Informational'])
            }
            
            return True
        except Exception as e:
            print(f"❌ Erro ao analisar resultados: {e}")
            return False
    
    def test_specific_vulnerabilities(self):
        """Testa vulnerabilidades específicas do FORTIS"""
        print("🎯 Testando vulnerabilidades específicas...")
        
        specific_tests = [
            self.test_authentication_bypass,
            self.test_sql_injection,
            self.test_xss,
            self.test_csrf,
            self.test_rate_limiting,
            self.test_crypto_implementation,
            self.test_api_security
        ]
        
        for test in specific_tests:
            try:
                test()
            except Exception as e:
                print(f"⚠️ Erro no teste {test.__name__}: {e}")
    
    def test_authentication_bypass(self):
        """Testa bypass de autenticação"""
        print("🔐 Testando bypass de autenticação...")
        
        # Testa endpoints sem autenticação
        endpoints = [
            '/api/v1/votes',
            '/api/v1/elections',
            '/api/v1/audit/trail',
            '/api/v1/tse/voter'
        ]
        
        for endpoint in endpoints:
            try:
                response = requests.get(f"{self.target_url}{endpoint}")
                if response.status_code == 200:
                    self.add_vulnerability('High', 'Authentication Bypass', 
                                         f'Endpoint {endpoint} acessível sem autenticação')
            except Exception as e:
                print(f"⚠️ Erro ao testar {endpoint}: {e}")
    
    def test_sql_injection(self):
        """Testa injeção SQL"""
        print("💉 Testando injeção SQL...")
        
        payloads = [
            "' OR '1'='1",
            "'; DROP TABLE users; --",
            "' UNION SELECT * FROM users --",
            "1' OR '1'='1' --"
        ]
        
        endpoints = [
            '/api/v1/auth/login',
            '/api/v1/votes',
            '/api/v1/elections'
        ]
        
        for endpoint in endpoints:
            for payload in payloads:
                try:
                    response = requests.post(f"{self.target_url}{endpoint}", 
                                           json={'cpf': payload, 'password': 'test'})
                    if 'error' in response.text.lower() or 'sql' in response.text.lower():
                        self.add_vulnerability('High', 'SQL Injection', 
                                             f'Possível SQL injection em {endpoint}')
                        break
                except Exception as e:
                    print(f"⚠️ Erro ao testar SQL injection em {endpoint}: {e}")
    
    def test_xss(self):
        """Testa Cross-Site Scripting (XSS)"""
        print("🌐 Testando XSS...")
        
        payloads = [
            '<script>alert("XSS")</script>',
            '"><script>alert("XSS")</script>',
            "javascript:alert('XSS')",
            '<img src=x onerror=alert("XSS")>'
        ]
        
        endpoints = [
            '/api/v1/elections',
            '/api/v1/candidates'
        ]
        
        for endpoint in endpoints:
            for payload in payloads:
                try:
                    response = requests.post(f"{self.target_url}{endpoint}", 
                                           json={'name': payload, 'description': payload})
                    if payload in response.text:
                        self.add_vulnerability('Medium', 'XSS', 
                                             f'Possível XSS em {endpoint}')
                        break
                except Exception as e:
                    print(f"⚠️ Erro ao testar XSS em {endpoint}: {e}")
    
    def test_csrf(self):
        """Testa Cross-Site Request Forgery (CSRF)"""
        print("🔄 Testando CSRF...")
        
        # Testa se endpoints críticos têm proteção CSRF
        critical_endpoints = [
            '/api/v1/votes',
            '/api/v1/elections',
            '/api/v1/audit/events'
        ]
        
        for endpoint in critical_endpoints:
            try:
                response = requests.post(f"{self.target_url}{endpoint}", 
                                       json={'test': 'data'})
                if response.status_code == 200:
                    self.add_vulnerability('Medium', 'CSRF', 
                                         f'Endpoint {endpoint} pode ser vulnerável a CSRF')
            except Exception as e:
                print(f"⚠️ Erro ao testar CSRF em {endpoint}: {e}")
    
    def test_rate_limiting(self):
        """Testa rate limiting"""
        print("⏱️ Testando rate limiting...")
        
        endpoint = '/api/v1/auth/login'
        max_requests = 100
        
        for i in range(max_requests):
            try:
                response = requests.post(f"{self.target_url}{endpoint}", 
                                       json={'cpf': 'test', 'password': 'test'})
                if response.status_code == 429:
                    print("✅ Rate limiting funcionando")
                    return
            except Exception as e:
                print(f"⚠️ Erro ao testar rate limiting: {e}")
        
        self.add_vulnerability('Medium', 'Rate Limiting', 
                             'Rate limiting não implementado ou ineficaz')
    
    def test_crypto_implementation(self):
        """Testa implementação criptográfica"""
        print("🔐 Testando implementação criptográfica...")
        
        # Testa endpoints de criptografia
        crypto_endpoints = [
            '/api/v1/crypto/encrypt',
            '/api/v1/crypto/decrypt',
            '/api/v1/zkp/generate-proof'
        ]
        
        for endpoint in crypto_endpoints:
            try:
                response = requests.post(f"{self.target_url}{endpoint}", 
                                       json={'data': 'test'})
                if response.status_code == 200:
                    # Verifica se a resposta está criptografada
                    if 'test' in response.text:
                        self.add_vulnerability('High', 'Crypto Implementation', 
                                             f'Possível vazamento de dados em {endpoint}')
            except Exception as e:
                print(f"⚠️ Erro ao testar criptografia em {endpoint}: {e}")
    
    def test_api_security(self):
        """Testa segurança geral da API"""
        print("🛡️ Testando segurança da API...")
        
        # Testa headers de segurança
        try:
            response = requests.get(self.target_url)
            headers = response.headers
            
            security_headers = {
                'X-Content-Type-Options': 'nosniff',
                'X-Frame-Options': 'DENY',
                'X-XSS-Protection': '1; mode=block',
                'Strict-Transport-Security': 'max-age=31536000',
                'Content-Security-Policy': 'default-src \'self\''
            }
            
            for header, expected in security_headers.items():
                if header not in headers:
                    self.add_vulnerability('Low', 'Missing Security Header', 
                                         f'Header {header} não encontrado')
                elif expected not in headers[header]:
                    self.add_vulnerability('Low', 'Insecure Security Header', 
                                         f'Header {header} com valor inseguro')
        except Exception as e:
            print(f"⚠️ Erro ao testar headers de segurança: {e}")
    
    def add_vulnerability(self, risk, name, description):
        """Adiciona vulnerabilidade aos resultados"""
        vulnerability = {
            'risk': risk,
            'name': name,
            'description': description,
            'timestamp': datetime.now().isoformat()
        }
        
        if risk not in self.results['vulnerabilities']:
            self.results['vulnerabilities'][risk] = []
        
        self.results['vulnerabilities'][risk].append(vulnerability)
    
    def generate_report(self):
        """Gera relatório de segurança"""
        print("📄 Gerando relatório de segurança...")
        
        report_file = f"security/penetration/reports/fortis_security_report_{self.results['scan_id']}.json"
        
        try:
            os.makedirs(os.path.dirname(report_file), exist_ok=True)
            
            with open(report_file, 'w') as f:
                json.dump(self.results, f, indent=2)
            
            print(f"✅ Relatório salvo em: {report_file}")
            return report_file
        except Exception as e:
            print(f"❌ Erro ao gerar relatório: {e}")
            return None
    
    def print_summary(self):
        """Imprime resumo dos resultados"""
        print("\n" + "="*50)
        print("📊 RESUMO DOS TESTES DE SEGURANÇA")
        print("="*50)
        
        summary = self.results['summary']
        print(f"🎯 Target: {self.results['target']}")
        print(f"📅 Data: {self.results['timestamp']}")
        print(f"🔍 Total de Alertas: {summary.get('total_alerts', 0)}")
        print(f"🔴 Alto Risco: {summary.get('high_risk', 0)}")
        print(f"🟡 Médio Risco: {summary.get('medium_risk', 0)}")
        print(f"🟢 Baixo Risco: {summary.get('low_risk', 0)}")
        print(f"ℹ️ Informativo: {summary.get('informational', 0)}")
        
        print("\n" + "="*50)
        print("🚨 VULNERABILIDADES ENCONTRADAS")
        print("="*50)
        
        for risk, vulns in self.results['vulnerabilities'].items():
            if vulns:
                print(f"\n{risk.upper()} RISK ({len(vulns)}):")
                for vuln in vulns:
                    print(f"  • {vuln['name']}: {vuln['description']}")
    
    def run_full_scan(self):
        """Executa scan completo de segurança"""
        print("🚀 Iniciando scan completo de segurança FORTIS...")
        
        if not self.start_zap():
            return False
        
        if not self.spider_scan():
            return False
        
        if not self.active_scan():
            return False
        
        self.test_specific_vulnerabilities()
        
        if not self.analyze_results():
            return False
        
        self.print_summary()
        
        report_file = self.generate_report()
        if report_file:
            print(f"\n✅ Scan concluído! Relatório salvo em: {report_file}")
            return True
        
        return False

def main():
    """Função principal"""
    if len(sys.argv) < 2:
        print("Uso: python owasp_zap_scan.py <target_url> [api_key]")
        sys.exit(1)
    
    target_url = sys.argv[1]
    api_key = sys.argv[2] if len(sys.argv) > 2 else None
    
    scanner = FortisSecurityScanner(target_url, api_key)
    success = scanner.run_full_scan()
    
    if success:
        print("\n🎉 Scan de segurança concluído com sucesso!")
        sys.exit(0)
    else:
        print("\n❌ Scan de segurança falhou!")
        sys.exit(1)

if __name__ == "__main__":
    main()
