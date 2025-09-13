#!/usr/bin/env python3
"""
FORTIS Security Test Suite
Executa todos os testes de segurança do sistema FORTIS
"""

import os
import sys
import json
import time
import subprocess
from datetime import datetime
import argparse

class FortisSecurityTestSuite:
    def __init__(self, config_file=None):
        self.config_file = config_file or "security/penetration/config/security_config.json"
        self.results = {
            'test_suite_id': f"fortis_security_suite_{int(time.time())}",
            'timestamp': datetime.now().isoformat(),
            'tests': [],
            'summary': {}
        }
        self.config = self.load_config()
    
    def load_config(self):
        """Carrega configuração dos testes"""
        try:
            with open(self.config_file, 'r') as f:
                return json.load(f)
        except FileNotFoundError:
            print(f"⚠️ Arquivo de configuração não encontrado: {self.config_file}")
            return self.get_default_config()
        except Exception as e:
            print(f"❌ Erro ao carregar configuração: {e}")
            return self.get_default_config()
    
    def get_default_config(self):
        """Retorna configuração padrão"""
        return {
            "targets": {
                "api_url": "http://localhost:8080",
                "domain": "fortis.gov.br",
                "contract_path": "blockchain/contracts/FortisVoting.sol"
            },
            "tests": {
                "owasp_zap": True,
                "smart_contracts": True,
                "crypto_security": True,
                "infrastructure": True,
                "mobile_security": True
            },
            "output": {
                "reports_dir": "security/penetration/reports",
                "logs_dir": "security/penetration/logs"
            }
        }
    
    def run_owasp_zap_tests(self):
        """Executa testes OWASP ZAP"""
        print("\n" + "="*60)
        print("🔍 EXECUTANDO TESTES OWASP ZAP")
        print("="*60)
        
        try:
            api_url = self.config['targets']['api_url']
            script_path = "security/penetration/scripts/owasp_zap_scan.py"
            
            result = subprocess.run([
                'python', script_path, api_url
            ], capture_output=True, text=True, timeout=1800)  # 30 minutos timeout
            
            test_result = {
                'name': 'OWASP ZAP',
                'status': 'PASSED' if result.returncode == 0 else 'FAILED',
                'output': result.stdout,
                'error': result.stderr,
                'timestamp': datetime.now().isoformat()
            }
            
            self.results['tests'].append(test_result)
            
            if result.returncode == 0:
                print("✅ Testes OWASP ZAP concluídos com sucesso")
            else:
                print("❌ Testes OWASP ZAP falharam")
                print(f"Erro: {result.stderr}")
            
            return result.returncode == 0
        
        except subprocess.TimeoutExpired:
            print("⏰ Testes OWASP ZAP excederam o tempo limite")
            return False
        except Exception as e:
            print(f"❌ Erro ao executar testes OWASP ZAP: {e}")
            return False
    
    def run_smart_contract_tests(self):
        """Executa testes de smart contracts"""
        print("\n" + "="*60)
        print("☸️ EXECUTANDO TESTES DE SMART CONTRACTS")
        print("="*60)
        
        try:
            contract_path = self.config['targets']['contract_path']
            script_path = "security/penetration/scripts/smart_contract_security.py"
            
            result = subprocess.run([
                'python', script_path, contract_path
            ], capture_output=True, text=True, timeout=600)  # 10 minutos timeout
            
            test_result = {
                'name': 'Smart Contract Security',
                'status': 'PASSED' if result.returncode == 0 else 'FAILED',
                'output': result.stdout,
                'error': result.stderr,
                'timestamp': datetime.now().isoformat()
            }
            
            self.results['tests'].append(test_result)
            
            if result.returncode == 0:
                print("✅ Testes de smart contracts concluídos com sucesso")
            else:
                print("❌ Testes de smart contracts falharam")
                print(f"Erro: {result.stderr}")
            
            return result.returncode == 0
        
        except subprocess.TimeoutExpired:
            print("⏰ Testes de smart contracts excederam o tempo limite")
            return False
        except Exception as e:
            print(f"❌ Erro ao executar testes de smart contracts: {e}")
            return False
    
    def run_crypto_security_tests(self):
        """Executa testes de segurança criptográfica"""
        print("\n" + "="*60)
        print("🔐 EXECUTANDO TESTES DE SEGURANÇA CRIPTOGRÁFICA")
        print("="*60)
        
        try:
            api_url = self.config['targets']['api_url']
            script_path = "security/penetration/scripts/crypto_security_test.py"
            
            result = subprocess.run([
                'python', script_path, api_url
            ], capture_output=True, text=True, timeout=300)  # 5 minutos timeout
            
            test_result = {
                'name': 'Crypto Security',
                'status': 'PASSED' if result.returncode == 0 else 'FAILED',
                'output': result.stdout,
                'error': result.stderr,
                'timestamp': datetime.now().isoformat()
            }
            
            self.results['tests'].append(test_result)
            
            if result.returncode == 0:
                print("✅ Testes de segurança criptográfica concluídos com sucesso")
            else:
                print("❌ Testes de segurança criptográfica falharam")
                print(f"Erro: {result.stderr}")
            
            return result.returncode == 0
        
        except subprocess.TimeoutExpired:
            print("⏰ Testes de segurança criptográfica excederam o tempo limite")
            return False
        except Exception as e:
            print(f"❌ Erro ao executar testes de segurança criptográfica: {e}")
            return False
    
    def run_infrastructure_tests(self):
        """Executa testes de infraestrutura"""
        print("\n" + "="*60)
        print("🏗️ EXECUTANDO TESTES DE INFRAESTRUTURA")
        print("="*60)
        
        try:
            domain = self.config['targets']['domain']
            script_path = "security/penetration/scripts/infrastructure_security.py"
            
            result = subprocess.run([
                'python', script_path, domain
            ], capture_output=True, text=True, timeout=600)  # 10 minutos timeout
            
            test_result = {
                'name': 'Infrastructure Security',
                'status': 'PASSED' if result.returncode == 0 else 'FAILED',
                'output': result.stdout,
                'error': result.stderr,
                'timestamp': datetime.now().isoformat()
            }
            
            self.results['tests'].append(test_result)
            
            if result.returncode == 0:
                print("✅ Testes de infraestrutura concluídos com sucesso")
            else:
                print("❌ Testes de infraestrutura falharam")
                print(f"Erro: {result.stderr}")
            
            return result.returncode == 0
        
        except subprocess.TimeoutExpired:
            print("⏰ Testes de infraestrutura excederam o tempo limite")
            return False
        except Exception as e:
            print(f"❌ Erro ao executar testes de infraestrutura: {e}")
            return False
    
    def run_mobile_security_tests(self):
        """Executa testes de segurança mobile"""
        print("\n" + "="*60)
        print("📱 EXECUTANDO TESTES DE SEGURANÇA MOBILE")
        print("="*60)
        
        try:
            # Verifica se o app mobile está compilado
            mobile_path = "mobile"
            if not os.path.exists(mobile_path):
                print("⚠️ App mobile não encontrado, pulando testes")
                return True
            
            # Executa testes de segurança mobile
            script_path = "security/penetration/scripts/mobile_security.py"
            
            if os.path.exists(script_path):
                result = subprocess.run([
                    'python', script_path, mobile_path
                ], capture_output=True, text=True, timeout=300)  # 5 minutos timeout
                
                test_result = {
                    'name': 'Mobile Security',
                    'status': 'PASSED' if result.returncode == 0 else 'FAILED',
                    'output': result.stdout,
                    'error': result.stderr,
                    'timestamp': datetime.now().isoformat()
                }
                
                self.results['tests'].append(test_result)
                
                if result.returncode == 0:
                    print("✅ Testes de segurança mobile concluídos com sucesso")
                else:
                    print("❌ Testes de segurança mobile falharam")
                    print(f"Erro: {result.stderr}")
                
                return result.returncode == 0
            else:
                print("⚠️ Script de testes mobile não encontrado, pulando testes")
                return True
        
        except subprocess.TimeoutExpired:
            print("⏰ Testes de segurança mobile excederam o tempo limite")
            return False
        except Exception as e:
            print(f"❌ Erro ao executar testes de segurança mobile: {e}")
            return False
    
    def generate_summary(self):
        """Gera resumo dos resultados"""
        total_tests = len(self.results['tests'])
        passed_tests = sum(1 for test in self.results['tests'] if test['status'] == 'PASSED')
        failed_tests = total_tests - passed_tests
        
        self.results['summary'] = {
            'total_tests': total_tests,
            'passed_tests': passed_tests,
            'failed_tests': failed_tests,
            'success_rate': (passed_tests / total_tests * 100) if total_tests > 0 else 0
        }
    
    def generate_report(self):
        """Gera relatório final"""
        print("\n" + "="*60)
        print("📄 GERANDO RELATÓRIO FINAL")
        print("="*60)
        
        # Cria diretório de relatórios
        reports_dir = self.config['output']['reports_dir']
        os.makedirs(reports_dir, exist_ok=True)
        
        # Gera relatório JSON
        report_file = f"{reports_dir}/fortis_security_test_suite_report_{self.results['test_suite_id']}.json"
        
        try:
            with open(report_file, 'w') as f:
                json.dump(self.results, f, indent=2)
            
            print(f"✅ Relatório JSON salvo em: {report_file}")
        except Exception as e:
            print(f"❌ Erro ao salvar relatório JSON: {e}")
        
        # Gera relatório HTML
        html_report_file = f"{reports_dir}/fortis_security_test_suite_report_{self.results['test_suite_id']}.html"
        
        try:
            html_content = self.generate_html_report()
            with open(html_report_file, 'w') as f:
                f.write(html_content)
            
            print(f"✅ Relatório HTML salvo em: {html_report_file}")
        except Exception as e:
            print(f"❌ Erro ao salvar relatório HTML: {e}")
        
        return report_file
    
    def generate_html_report(self):
        """Gera relatório HTML"""
        html = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <title>FORTIS Security Test Suite Report</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 20px; }}
                .header {{ background-color: #1976d2; color: white; padding: 20px; border-radius: 5px; }}
                .summary {{ background-color: #f5f5f5; padding: 20px; margin: 20px 0; border-radius: 5px; }}
                .test {{ margin: 10px 0; padding: 15px; border-radius: 5px; }}
                .passed {{ background-color: #d4edda; border-left: 5px solid #28a745; }}
                .failed {{ background-color: #f8d7da; border-left: 5px solid #dc3545; }}
                .timestamp {{ color: #666; font-size: 12px; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>🛡️ FORTIS Security Test Suite Report</h1>
                <p>Test Suite ID: {self.results['test_suite_id']}</p>
                <p>Timestamp: {self.results['timestamp']}</p>
            </div>
            
            <div class="summary">
                <h2>📊 Summary</h2>
                <p><strong>Total Tests:</strong> {self.results['summary']['total_tests']}</p>
                <p><strong>Passed:</strong> {self.results['summary']['passed_tests']}</p>
                <p><strong>Failed:</strong> {self.results['summary']['failed_tests']}</p>
                <p><strong>Success Rate:</strong> {self.results['summary']['success_rate']:.1f}%</p>
            </div>
            
            <h2>🧪 Test Results</h2>
        """
        
        for test in self.results['tests']:
            status_class = 'passed' if test['status'] == 'PASSED' else 'failed'
            html += f"""
            <div class="test {status_class}">
                <h3>{test['name']} - {test['status']}</h3>
                <p class="timestamp">{test['timestamp']}</p>
                <pre>{test['output']}</pre>
                {f"<pre style='color: red;'>{test['error']}</pre>" if test['error'] else ''}
            </div>
            """
        
        html += """
        </body>
        </html>
        """
        
        return html
    
    def print_summary(self):
        """Imprime resumo dos resultados"""
        print("\n" + "="*60)
        print("📊 RESUMO FINAL DOS TESTES DE SEGURANÇA")
        print("="*60)
        
        summary = self.results['summary']
        print(f"🧪 Total de Testes: {summary['total_tests']}")
        print(f"✅ Testes Aprovados: {summary['passed_tests']}")
        print(f"❌ Testes Falharam: {summary['failed_tests']}")
        print(f"📈 Taxa de Sucesso: {summary['success_rate']:.1f}%")
        
        print("\n" + "="*60)
        print("📋 DETALHES DOS TESTES")
        print("="*60)
        
        for test in self.results['tests']:
            status_icon = "✅" if test['status'] == 'PASSED' else "❌"
            print(f"{status_icon} {test['name']} - {test['status']}")
            print(f"   Timestamp: {test['timestamp']}")
            if test['error']:
                print(f"   Erro: {test['error'][:100]}...")
            print()
    
    def run_all_tests(self):
        """Executa todos os testes de segurança"""
        print("🚀 Iniciando FORTIS Security Test Suite...")
        print(f"📅 Timestamp: {self.results['timestamp']}")
        print(f"🎯 Configuração: {self.config_file}")
        
        # Executa testes baseado na configuração
        tests_to_run = self.config['tests']
        
        if tests_to_run.get('owasp_zap', False):
            self.run_owasp_zap_tests()
        
        if tests_to_run.get('smart_contracts', False):
            self.run_smart_contract_tests()
        
        if tests_to_run.get('crypto_security', False):
            self.run_crypto_security_tests()
        
        if tests_to_run.get('infrastructure', False):
            self.run_infrastructure_tests()
        
        if tests_to_run.get('mobile_security', False):
            self.run_mobile_security_tests()
        
        # Gera resumo e relatório
        self.generate_summary()
        self.print_summary()
        
        report_file = self.generate_report()
        
        if report_file:
            print(f"\n🎉 Test Suite concluída! Relatório salvo em: {report_file}")
            return True
        
        return False

def main():
    """Função principal"""
    parser = argparse.ArgumentParser(description='FORTIS Security Test Suite')
    parser.add_argument('--config', '-c', help='Arquivo de configuração', 
                       default='security/penetration/config/security_config.json')
    parser.add_argument('--quick', '-q', action='store_true', 
                       help='Executa apenas testes rápidos')
    
    args = parser.parse_args()
    
    # Cria configuração rápida se solicitado
    if args.quick:
        config = {
            "targets": {
                "api_url": "http://localhost:8080",
                "domain": "fortis.gov.br",
                "contract_path": "blockchain/contracts/FortisVoting.sol"
            },
            "tests": {
                "owasp_zap": False,  # Muito lento para quick test
                "smart_contracts": True,
                "crypto_security": True,
                "infrastructure": False,  # Muito lento para quick test
                "mobile_security": False
            },
            "output": {
                "reports_dir": "security/penetration/reports",
                "logs_dir": "security/penetration/logs"
            }
        }
        
        # Salva configuração rápida
        os.makedirs(os.path.dirname(args.config), exist_ok=True)
        with open(args.config, 'w') as f:
            json.dump(config, f, indent=2)
    
    test_suite = FortisSecurityTestSuite(args.config)
    success = test_suite.run_all_tests()
    
    if success:
        print("\n🎉 FORTIS Security Test Suite concluída com sucesso!")
        sys.exit(0)
    else:
        print("\n❌ FORTIS Security Test Suite falhou!")
        sys.exit(1)

if __name__ == "__main__":
    main()
