#!/usr/bin/env python3
"""
Infrastructure Security Scanner para FORTIS
Testa segurança da infraestrutura Kubernetes e serviços
"""

import os
import sys
import json
import time
import subprocess
import requests
from datetime import datetime
import socket
import ssl
import dns.resolver

class FortisInfrastructureSecurityScanner:
    def __init__(self, target_domain=None, kubernetes_config=None):
        self.target_domain = target_domain or "fortis.gov.br"
        self.kubernetes_config = kubernetes_config or "~/.kube/config"
        self.results = {
            'scan_id': f"fortis_infrastructure_scan_{int(time.time())}",
            'target_domain': self.target_domain,
            'timestamp': datetime.now().isoformat(),
            'vulnerabilities': [],
            'summary': {}
        }
    
    def test_network_security(self):
        """Testa segurança de rede"""
        print("🌐 Testando segurança de rede...")
        
        # Testa portas abertas
        self.test_open_ports()
        
        # Testa SSL/TLS
        self.test_ssl_tls()
        
        # Testa DNS
        self.test_dns_security()
        
        # Testa firewall
        self.test_firewall_rules()
    
    def test_open_ports(self):
        """Testa portas abertas"""
        print("🔍 Testando portas abertas...")
        
        common_ports = [22, 80, 443, 8080, 8443, 3306, 5432, 6379, 9200]
        
        for port in common_ports:
            try:
                sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                sock.settimeout(3)
                result = sock.connect_ex((self.target_domain, port))
                sock.close()
                
                if result == 0:
                    print(f"✅ Porta {port} aberta")
                    # Verifica se a porta é necessária
                    if port in [22, 80, 443]:
                        print(f"✅ Porta {port} necessária")
                    else:
                        self.add_vulnerability('Medium', 'Open Port', 
                                             f'Porta {port} aberta e pode não ser necessária')
                else:
                    print(f"❌ Porta {port} fechada")
            
            except Exception as e:
                print(f"⚠️ Erro ao testar porta {port}: {e}")
    
    def test_ssl_tls(self):
        """Testa configuração SSL/TLS"""
        print("🔐 Testando configuração SSL/TLS...")
        
        try:
            # Testa HTTPS
            context = ssl.create_default_context()
            with socket.create_connection((self.target_domain, 443), timeout=10) as sock:
                with context.wrap_socket(sock, server_hostname=self.target_domain) as ssock:
                    cert = ssock.getpeercert()
                    cipher = ssock.cipher()
                    
                    # Verifica certificado
                    if cert:
                        print("✅ Certificado SSL encontrado")
                        
                        # Verifica validade do certificado
                        from datetime import datetime
                        not_after = datetime.strptime(cert['notAfter'], '%b %d %H:%M:%S %Y %Z')
                        if not_after > datetime.now():
                            print("✅ Certificado SSL válido")
                        else:
                            self.add_vulnerability('High', 'SSL Certificate', 
                                                 'Certificado SSL expirado')
                        
                        # Verifica algoritmo de criptografia
                        if cipher:
                            print(f"✅ Cipher: {cipher[0]}")
                            if 'RC4' in cipher[0] or 'DES' in cipher[0]:
                                self.add_vulnerability('High', 'Weak Cipher', 
                                                     f'Cipher fraco detectado: {cipher[0]}')
                    else:
                        self.add_vulnerability('High', 'SSL Certificate', 
                                             'Certificado SSL não encontrado')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'SSL/TLS Error', 
                                 f'Erro ao testar SSL/TLS: {e}')
    
    def test_dns_security(self):
        """Testa segurança DNS"""
        print("🌐 Testando segurança DNS...")
        
        try:
            # Testa resolução DNS
            resolver = dns.resolver.Resolver()
            resolver.timeout = 5
            resolver.lifetime = 5
            
            # Testa registros A
            try:
                answers = resolver.resolve(self.target_domain, 'A')
                if answers:
                    print("✅ Registros A encontrados")
                    for answer in answers:
                        print(f"  {answer}")
                else:
                    self.add_vulnerability('Medium', 'DNS A Record', 
                                         'Registros A não encontrados')
            except Exception as e:
                self.add_vulnerability('Medium', 'DNS Resolution', 
                                     f'Erro na resolução DNS: {e}')
            
            # Testa registros MX
            try:
                answers = resolver.resolve(self.target_domain, 'MX')
                if answers:
                    print("✅ Registros MX encontrados")
                else:
                    print("ℹ️ Registros MX não encontrados")
            except Exception as e:
                print(f"⚠️ Erro ao testar registros MX: {e}")
            
            # Testa registros TXT
            try:
                answers = resolver.resolve(self.target_domain, 'TXT')
                if answers:
                    print("✅ Registros TXT encontrados")
                    for answer in answers:
                        txt = str(answer)
                        if 'v=spf1' in txt:
                            print("✅ SPF record encontrado")
                        if 'v=DKIM1' in txt:
                            print("✅ DKIM record encontrado")
                        if 'v=DMARC1' in txt:
                            print("✅ DMARC record encontrado")
                else:
                    self.add_vulnerability('Low', 'DNS TXT Record', 
                                         'Registros TXT não encontrados')
            except Exception as e:
                print(f"⚠️ Erro ao testar registros TXT: {e}")
        
        except Exception as e:
            self.add_vulnerability('Medium', 'DNS Security', 
                                 f'Erro ao testar segurança DNS: {e}')
    
    def test_firewall_rules(self):
        """Testa regras de firewall"""
        print("🔥 Testando regras de firewall...")
        
        # Testa se portas sensíveis estão bloqueadas
        sensitive_ports = [21, 23, 135, 139, 445, 1433, 3389]
        
        for port in sensitive_ports:
            try:
                sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                sock.settimeout(3)
                result = sock.connect_ex((self.target_domain, port))
                sock.close()
                
                if result == 0:
                    self.add_vulnerability('High', 'Firewall Rule', 
                                         f'Porta sensível {port} acessível')
                else:
                    print(f"✅ Porta sensível {port} bloqueada")
            
            except Exception as e:
                print(f"⚠️ Erro ao testar porta {port}: {e}")
    
    def test_kubernetes_security(self):
        """Testa segurança do Kubernetes"""
        print("☸️ Testando segurança do Kubernetes...")
        
        if not os.path.exists(os.path.expanduser(self.kubernetes_config)):
            print("⚠️ Configuração do Kubernetes não encontrada")
            return
        
        # Testa configuração do cluster
        self.test_k8s_cluster_config()
        
        # Testa RBAC
        self.test_k8s_rbac()
        
        # Testa network policies
        self.test_k8s_network_policies()
        
        # Testa secrets
        self.test_k8s_secrets()
    
    def test_k8s_cluster_config(self):
        """Testa configuração do cluster"""
        print("☸️ Testando configuração do cluster...")
        
        try:
            # Verifica versão do Kubernetes
            result = subprocess.run(['kubectl', 'version', '--client'], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                print("✅ kubectl configurado")
            else:
                self.add_vulnerability('Medium', 'Kubernetes Config', 
                                     'kubectl não configurado corretamente')
            
            # Verifica nodes
            result = subprocess.run(['kubectl', 'get', 'nodes'], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                print("✅ Nodes acessíveis")
            else:
                self.add_vulnerability('Medium', 'Kubernetes Nodes', 
                                     'Não foi possível acessar nodes')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Kubernetes Error', 
                                 f'Erro ao testar Kubernetes: {e}')
    
    def test_k8s_rbac(self):
        """Testa RBAC do Kubernetes"""
        print("☸️ Testando RBAC...")
        
        try:
            # Verifica roles
            result = subprocess.run(['kubectl', 'get', 'roles'], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                print("✅ Roles configuradas")
            else:
                self.add_vulnerability('Medium', 'Kubernetes RBAC', 
                                     'Roles não configuradas')
            
            # Verifica rolebindings
            result = subprocess.run(['kubectl', 'get', 'rolebindings'], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                print("✅ RoleBindings configuradas")
            else:
                self.add_vulnerability('Medium', 'Kubernetes RBAC', 
                                     'RoleBindings não configuradas')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Kubernetes RBAC Error', 
                                 f'Erro ao testar RBAC: {e}')
    
    def test_k8s_network_policies(self):
        """Testa network policies do Kubernetes"""
        print("☸️ Testando network policies...")
        
        try:
            # Verifica network policies
            result = subprocess.run(['kubectl', 'get', 'networkpolicies'], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                print("✅ Network policies configuradas")
            else:
                self.add_vulnerability('Medium', 'Kubernetes Network Policies', 
                                     'Network policies não configuradas')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Kubernetes Network Policies Error', 
                                 f'Erro ao testar network policies: {e}')
    
    def test_k8s_secrets(self):
        """Testa secrets do Kubernetes"""
        print("☸️ Testando secrets...")
        
        try:
            # Verifica secrets
            result = subprocess.run(['kubectl', 'get', 'secrets'], 
                                  capture_output=True, text=True)
            if result.returncode == 0:
                print("✅ Secrets configuradas")
            else:
                self.add_vulnerability('Medium', 'Kubernetes Secrets', 
                                     'Secrets não configuradas')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Kubernetes Secrets Error', 
                                 f'Erro ao testar secrets: {e}')
    
    def test_application_security(self):
        """Testa segurança da aplicação"""
        print("🔒 Testando segurança da aplicação...")
        
        # Testa headers de segurança
        self.test_security_headers()
        
        # Testa CORS
        self.test_cors()
        
        # Testa rate limiting
        self.test_rate_limiting()
    
    def test_security_headers(self):
        """Testa headers de segurança"""
        print("🔒 Testando headers de segurança...")
        
        try:
            response = requests.get(f"https://{self.target_domain}", timeout=10)
            headers = response.headers
            
            security_headers = {
                'X-Content-Type-Options': 'nosniff',
                'X-Frame-Options': 'DENY',
                'X-XSS-Protection': '1; mode=block',
                'Strict-Transport-Security': 'max-age=31536000',
                'Content-Security-Policy': 'default-src \'self\'',
                'Referrer-Policy': 'strict-origin-when-cross-origin'
            }
            
            for header, expected in security_headers.items():
                if header in headers:
                    print(f"✅ Header {header} encontrado")
                else:
                    self.add_vulnerability('Low', 'Missing Security Header', 
                                         f'Header {header} não encontrado')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Security Headers Error', 
                                 f'Erro ao testar headers de segurança: {e}')
    
    def test_cors(self):
        """Testa configuração CORS"""
        print("🔒 Testando configuração CORS...")
        
        try:
            response = requests.options(f"https://{self.target_domain}", 
                                      headers={'Origin': 'https://malicious.com'})
            
            if 'Access-Control-Allow-Origin' in response.headers:
                origin = response.headers.get('Access-Control-Allow-Origin')
                if origin == '*':
                    self.add_vulnerability('High', 'CORS Configuration', 
                                         'CORS configurado para aceitar qualquer origem')
                else:
                    print("✅ CORS configurado corretamente")
            else:
                print("ℹ️ CORS não configurado")
        
        except Exception as e:
            self.add_vulnerability('Medium', 'CORS Error', 
                                 f'Erro ao testar CORS: {e}')
    
    def test_rate_limiting(self):
        """Testa rate limiting"""
        print("🔒 Testando rate limiting...")
        
        try:
            # Faz múltiplas requisições
            for i in range(100):
                response = requests.get(f"https://{self.target_domain}", timeout=5)
                if response.status_code == 429:
                    print("✅ Rate limiting funcionando")
                    return
            
            self.add_vulnerability('Medium', 'Rate Limiting', 
                                 'Rate limiting não implementado ou ineficaz')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Rate Limiting Error', 
                                 f'Erro ao testar rate limiting: {e}')
    
    def add_vulnerability(self, severity, title, description):
        """Adiciona vulnerabilidade aos resultados"""
        vulnerability = {
            'severity': severity,
            'title': title,
            'description': description,
            'timestamp': datetime.now().isoformat()
        }
        self.results['vulnerabilities'].append(vulnerability)
    
    def generate_summary(self):
        """Gera resumo dos resultados"""
        severity_counts = {'High': 0, 'Medium': 0, 'Low': 0, 'Informational': 0}
        
        for vuln in self.results['vulnerabilities']:
            severity = vuln.get('severity', 'Informational')
            if severity in severity_counts:
                severity_counts[severity] += 1
        
        self.results['summary'] = {
            'total_vulnerabilities': len(self.results['vulnerabilities']),
            'high_severity': severity_counts['High'],
            'medium_severity': severity_counts['Medium'],
            'low_severity': severity_counts['Low'],
            'informational': severity_counts['Informational']
        }
    
    def generate_report(self):
        """Gera relatório de segurança"""
        print("📄 Gerando relatório de segurança...")
        
        report_file = f"security/penetration/reports/fortis_infrastructure_security_report_{self.results['scan_id']}.json"
        
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
        print("📊 RESUMO DOS TESTES DE SEGURANÇA - INFRAESTRUTURA")
        print("="*50)
        
        summary = self.results['summary']
        print(f"🎯 Target: {self.results['target_domain']}")
        print(f"📅 Data: {self.results['timestamp']}")
        print(f"🔍 Total de Vulnerabilidades: {summary.get('total_vulnerabilities', 0)}")
        print(f"🔴 Alto Risco: {summary.get('high_severity', 0)}")
        print(f"🟡 Médio Risco: {summary.get('medium_severity', 0)}")
        print(f"🟢 Baixo Risco: {summary.get('low_severity', 0)}")
        print(f"ℹ️ Informativo: {summary.get('informational', 0)}")
        
        print("\n" + "="*50)
        print("🚨 VULNERABILIDADES ENCONTRADAS")
        print("="*50)
        
        for vuln in self.results['vulnerabilities']:
            severity = vuln.get('severity', 'Unknown')
            title = vuln.get('title', 'Unknown')
            description = vuln.get('description', '')
            
            print(f"\n{severity.upper()} - {title}")
            print(f"  Descrição: {description}")
    
    def run_full_scan(self):
        """Executa scan completo de segurança"""
        print("🚀 Iniciando scan completo de segurança de infraestrutura FORTIS...")
        
        # Executa testes
        self.test_network_security()
        self.test_kubernetes_security()
        self.test_application_security()
        
        # Gera resumo e relatório
        self.generate_summary()
        self.print_summary()
        
        report_file = self.generate_report()
        if report_file:
            print(f"\n✅ Scan concluído! Relatório salvo em: {report_file}")
            return True
        
        return False

def main():
    """Função principal"""
    if len(sys.argv) < 2:
        print("Uso: python infrastructure_security.py <target_domain> [kube_config]")
        sys.exit(1)
    
    target_domain = sys.argv[1]
    kube_config = sys.argv[2] if len(sys.argv) > 2 else None
    
    scanner = FortisInfrastructureSecurityScanner(target_domain, kube_config)
    success = scanner.run_full_scan()
    
    if success:
        print("\n🎉 Scan de segurança de infraestrutura concluído com sucesso!")
        sys.exit(0)
    else:
        print("\n❌ Scan de segurança de infraestrutura falhou!")
        sys.exit(1)

if __name__ == "__main__":
    main()
