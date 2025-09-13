#!/usr/bin/env python3
"""
Mobile Security Scanner para FORTIS
Testa segurança do aplicativo mobile React Native
"""

import os
import sys
import json
import time
import subprocess
from datetime import datetime
import re

class FortisMobileSecurityScanner:
    def __init__(self, mobile_path):
        self.mobile_path = mobile_path
        self.results = {
            'scan_id': f"fortis_mobile_scan_{int(time.time())}",
            'mobile_path': mobile_path,
            'timestamp': datetime.now().isoformat(),
            'vulnerabilities': [],
            'summary': {}
        }
    
    def test_package_security(self):
        """Testa segurança do package.json"""
        print("📦 Testando segurança do package.json...")
        
        package_json_path = os.path.join(self.mobile_path, 'package.json')
        
        if not os.path.exists(package_json_path):
            self.add_vulnerability('High', 'Package.json Missing', 
                                 'Arquivo package.json não encontrado')
            return
        
        try:
            with open(package_json_path, 'r') as f:
                package_data = json.load(f)
            
            # Verifica dependências vulneráveis
            self.check_vulnerable_dependencies(package_data)
            
            # Verifica scripts inseguros
            self.check_insecure_scripts(package_data)
            
            # Verifica configurações de segurança
            self.check_security_configs(package_data)
            
        except Exception as e:
            self.add_vulnerability('Medium', 'Package.json Error', 
                                 f'Erro ao analisar package.json: {e}')
    
    def check_vulnerable_dependencies(self, package_data):
        """Verifica dependências vulneráveis"""
        print("🔍 Verificando dependências vulneráveis...")
        
        # Lista de dependências conhecidas por vulnerabilidades
        vulnerable_deps = {
            'react-native': ['0.60.0', '0.61.0', '0.62.0'],
            'react-native-vector-icons': ['< 8.0.0'],
            'react-native-camera': ['< 4.0.0'],
            'react-native-keychain': ['< 6.0.0']
        }
        
        dependencies = package_data.get('dependencies', {})
        dev_dependencies = package_data.get('devDependencies', {})
        all_deps = {**dependencies, **dev_dependencies}
        
        for dep, vulnerable_versions in vulnerable_deps.items():
            if dep in all_deps:
                version = all_deps[dep]
                for vuln_version in vulnerable_versions:
                    if self.version_matches(version, vuln_version):
                        self.add_vulnerability('High', 'Vulnerable Dependency', 
                                             f'Dependência {dep} versão {version} tem vulnerabilidades conhecidas')
    
    def check_insecure_scripts(self, package_data):
        """Verifica scripts inseguros"""
        print("🔍 Verificando scripts inseguros...")
        
        scripts = package_data.get('scripts', {})
        
        # Verifica scripts que podem expor informações sensíveis
        insecure_patterns = [
            r'--verbose',
            r'--debug',
            r'--inspect',
            r'console\.log',
            r'console\.warn',
            r'console\.error'
        ]
        
        for script_name, script_content in scripts.items():
            for pattern in insecure_patterns:
                if re.search(pattern, script_content):
                    self.add_vulnerability('Medium', 'Insecure Script', 
                                         f'Script {script_name} pode expor informações sensíveis')
    
    def check_security_configs(self, package_data):
        """Verifica configurações de segurança"""
        print("🔍 Verificando configurações de segurança...")
        
        # Verifica se há configurações de segurança
        if 'security' not in package_data:
            self.add_vulnerability('Low', 'Security Config', 
                                 'Configurações de segurança não encontradas no package.json')
        
        # Verifica se há configurações de CSP
        if 'csp' not in package_data:
            self.add_vulnerability('Low', 'CSP Config', 
                                 'Content Security Policy não configurada')
    
    def test_source_code_security(self):
        """Testa segurança do código fonte"""
        print("📱 Testando segurança do código fonte...")
        
        # Verifica arquivos TypeScript/JavaScript
        self.scan_js_files()
        
        # Verifica configurações de segurança
        self.scan_security_configs()
        
        # Verifica implementações criptográficas
        self.scan_crypto_implementations()
    
    def scan_js_files(self):
        """Escaneia arquivos JavaScript/TypeScript"""
        print("🔍 Escaneando arquivos JavaScript/TypeScript...")
        
        js_files = []
        for root, dirs, files in os.walk(self.mobile_path):
            for file in files:
                if file.endswith(('.js', '.jsx', '.ts', '.tsx')):
                    js_files.append(os.path.join(root, file))
        
        for file_path in js_files:
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Verifica padrões inseguros
                self.check_insecure_patterns(file_path, content)
                
            except Exception as e:
                print(f"⚠️ Erro ao analisar {file_path}: {e}")
    
    def check_insecure_patterns(self, file_path, content):
        """Verifica padrões inseguros no código"""
        # Padrões inseguros comuns
        insecure_patterns = [
            (r'eval\s*\(', 'High', 'Use of eval()'),
            (r'innerHTML\s*=', 'High', 'Direct innerHTML assignment'),
            (r'document\.write\s*\(', 'High', 'Use of document.write()'),
            (r'setTimeout\s*\(\s*["\']', 'Medium', 'String-based setTimeout'),
            (r'setInterval\s*\(\s*["\']', 'Medium', 'String-based setInterval'),
            (r'console\.log\s*\(', 'Low', 'Console.log in production'),
            (r'debugger\s*;', 'Low', 'Debugger statement'),
            (r'alert\s*\(', 'Low', 'Alert in production'),
            (r'confirm\s*\(', 'Low', 'Confirm in production'),
            (r'prompt\s*\(', 'Low', 'Prompt in production')
        ]
        
        for pattern, severity, description in insecure_patterns:
            if re.search(pattern, content):
                self.add_vulnerability(severity, description, 
                                     f'Encontrado em {file_path}')
    
    def scan_security_configs(self):
        """Escaneia configurações de segurança"""
        print("🔍 Escaneando configurações de segurança...")
        
        # Verifica arquivos de configuração
        config_files = [
            'metro.config.js',
            'babel.config.js',
            'tsconfig.json',
            'android/app/build.gradle',
            'ios/FortisMobile.xcodeproj/project.pbxproj'
        ]
        
        for config_file in config_files:
            file_path = os.path.join(self.mobile_path, config_file)
            if os.path.exists(file_path):
                self.check_config_file_security(file_path)
    
    def check_config_file_security(self, file_path):
        """Verifica segurança de arquivo de configuração"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Verifica configurações inseguras
            insecure_configs = [
                (r'debug\s*:\s*true', 'Medium', 'Debug mode enabled'),
                (r'verbose\s*:\s*true', 'Low', 'Verbose mode enabled'),
                (r'minify\s*:\s*false', 'Low', 'Minification disabled'),
                (r'sourceMap\s*:\s*true', 'Low', 'Source maps enabled in production')
            ]
            
            for pattern, severity, description in insecure_configs:
                if re.search(pattern, content):
                    self.add_vulnerability(severity, description, 
                                         f'Encontrado em {file_path}')
        
        except Exception as e:
            print(f"⚠️ Erro ao analisar {file_path}: {e}")
    
    def scan_crypto_implementations(self):
        """Escaneia implementações criptográficas"""
        print("🔍 Escaneando implementações criptográficas...")
        
        # Verifica se há implementações criptográficas seguras
        crypto_files = []
        for root, dirs, files in os.walk(self.mobile_path):
            for file in files:
                if file.endswith(('.js', '.jsx', '.ts', '.tsx')):
                    file_path = os.path.join(root, file)
                    try:
                        with open(file_path, 'r', encoding='utf-8') as f:
                            content = f.read()
                        
                        if any(keyword in content.lower() for keyword in ['crypto', 'encrypt', 'decrypt', 'hash', 'sign']):
                            crypto_files.append(file_path)
                    except Exception as e:
                        print(f"⚠️ Erro ao analisar {file_path}: {e}")
        
        for file_path in crypto_files:
            self.check_crypto_implementation(file_path)
    
    def check_crypto_implementation(self, file_path):
        """Verifica implementação criptográfica"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Verifica padrões criptográficos inseguros
            insecure_crypto_patterns = [
                (r'MD5\s*\(', 'High', 'Use of MD5 hash'),
                (r'SHA1\s*\(', 'High', 'Use of SHA1 hash'),
                (r'DES\s*\(', 'High', 'Use of DES encryption'),
                (r'RC4\s*\(', 'High', 'Use of RC4 encryption'),
                (r'Math\.random\s*\(', 'High', 'Use of Math.random() for crypto'),
            ]
            
            for pattern, severity, description in insecure_crypto_patterns:
                if re.search(pattern, content):
                    self.add_vulnerability(severity, description, 
                                         f'Encontrado em {file_path}')
            
            # Verifica se há implementações criptográficas seguras
            secure_crypto_patterns = [
                r'AES\s*\(',
                r'SHA256\s*\(',
                r'crypto\.randomBytes',
                r'crypto\.createHash\s*\(',
                r'crypto\.createCipher\s*\(',
                r'crypto\.createDecipher\s*\('
            ]
            
            has_secure_crypto = any(re.search(pattern, content) for pattern in secure_crypto_patterns)
            if not has_secure_crypto and any(keyword in content.lower() for keyword in ['crypto', 'encrypt', 'decrypt']):
                self.add_vulnerability('Medium', 'Crypto Implementation', 
                                     f'Implementação criptográfica insegura em {file_path}')
        
        except Exception as e:
            print(f"⚠️ Erro ao analisar {file_path}: {e}")
    
    def test_android_security(self):
        """Testa segurança do Android"""
        print("🤖 Testando segurança do Android...")
        
        android_path = os.path.join(self.mobile_path, 'android')
        if not os.path.exists(android_path):
            print("⚠️ Diretório Android não encontrado")
            return
        
        # Verifica AndroidManifest.xml
        self.check_android_manifest(android_path)
        
        # Verifica build.gradle
        self.check_android_build_gradle(android_path)
        
        # Verifica configurações de segurança
        self.check_android_security_configs(android_path)
    
    def check_android_manifest(self, android_path):
        """Verifica AndroidManifest.xml"""
        manifest_path = os.path.join(android_path, 'app', 'src', 'main', 'AndroidManifest.xml')
        
        if not os.path.exists(manifest_path):
            self.add_vulnerability('High', 'AndroidManifest Missing', 
                                 'AndroidManifest.xml não encontrado')
            return
        
        try:
            with open(manifest_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Verifica permissões inseguras
            insecure_permissions = [
                'android.permission.INTERNET',
                'android.permission.ACCESS_NETWORK_STATE',
                'android.permission.ACCESS_WIFI_STATE',
                'android.permission.CAMERA',
                'android.permission.RECORD_AUDIO',
                'android.permission.ACCESS_FINE_LOCATION',
                'android.permission.ACCESS_COARSE_LOCATION'
            ]
            
            for permission in insecure_permissions:
                if permission in content:
                    self.add_vulnerability('Medium', 'Android Permission', 
                                         f'Permissão {permission} pode ser sensível')
            
            # Verifica se há proteções de segurança
            security_features = [
                'android:allowBackup="false"',
                'android:debuggable="false"',
                'android:extractNativeLibs="false"'
            ]
            
            for feature in security_features:
                if feature not in content:
                    self.add_vulnerability('Low', 'Android Security Feature', 
                                         f'Recurso de segurança {feature} não encontrado')
        
        except Exception as e:
            print(f"⚠️ Erro ao analisar AndroidManifest.xml: {e}")
    
    def check_android_build_gradle(self, android_path):
        """Verifica build.gradle"""
        build_gradle_path = os.path.join(android_path, 'app', 'build.gradle')
        
        if not os.path.exists(build_gradle_path):
            self.add_vulnerability('High', 'Build.gradle Missing', 
                                 'build.gradle não encontrado')
            return
        
        try:
            with open(build_gradle_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Verifica configurações inseguras
            insecure_configs = [
                (r'debuggable\s*true', 'High', 'Debug mode enabled'),
                (r'minifyEnabled\s*false', 'Low', 'Minification disabled'),
                (r'shrinkResources\s*false', 'Low', 'Resource shrinking disabled')
            ]
            
            for pattern, severity, description in insecure_configs:
                if re.search(pattern, content):
                    self.add_vulnerability(severity, description, 
                                         f'Encontrado em build.gradle')
        
        except Exception as e:
            print(f"⚠️ Erro ao analisar build.gradle: {e}")
    
    def check_android_security_configs(self, android_path):
        """Verifica configurações de segurança do Android"""
        # Verifica se há network_security_config.xml
        network_config_path = os.path.join(android_path, 'app', 'src', 'main', 'res', 'xml', 'network_security_config.xml')
        
        if not os.path.exists(network_config_path):
            self.add_vulnerability('Medium', 'Network Security Config', 
                                 'network_security_config.xml não encontrado')
        else:
            try:
                with open(network_config_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Verifica se há certificate pinning
                if 'pin-set' not in content:
                    self.add_vulnerability('Medium', 'Certificate Pinning', 
                                         'Certificate pinning não configurado')
                else:
                    print("✅ Certificate pinning configurado")
            
            except Exception as e:
                print(f"⚠️ Erro ao analisar network_security_config.xml: {e}")
    
    def test_ios_security(self):
        """Testa segurança do iOS"""
        print("🍎 Testando segurança do iOS...")
        
        ios_path = os.path.join(self.mobile_path, 'ios')
        if not os.path.exists(ios_path):
            print("⚠️ Diretório iOS não encontrado")
            return
        
        # Verifica Info.plist
        self.check_ios_info_plist(ios_path)
        
        # Verifica configurações de segurança
        self.check_ios_security_configs(ios_path)
    
    def check_ios_info_plist(self, ios_path):
        """Verifica Info.plist"""
        info_plist_path = os.path.join(ios_path, 'FortisMobile', 'Info.plist')
        
        if not os.path.exists(info_plist_path):
            self.add_vulnerability('High', 'Info.plist Missing', 
                                 'Info.plist não encontrado')
            return
        
        try:
            with open(info_plist_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Verifica configurações inseguras
            insecure_configs = [
                (r'<key>UIFileSharingEnabled</key>\s*<true/>', 'Medium', 'File sharing enabled'),
                (r'<key>NSAppTransportSecurity</key>', 'Low', 'App Transport Security not configured')
            ]
            
            for pattern, severity, description in insecure_configs:
                if re.search(pattern, content):
                    self.add_vulnerability(severity, description, 
                                         f'Encontrado em Info.plist')
        
        except Exception as e:
            print(f"⚠️ Erro ao analisar Info.plist: {e}")
    
    def check_ios_security_configs(self, ios_path):
        """Verifica configurações de segurança do iOS"""
        # Verifica se há configurações de segurança
        security_configs = [
            'NSAppTransportSecurity',
            'NSAllowsArbitraryLoads',
            'NSExceptionDomains'
        ]
        
        info_plist_path = os.path.join(ios_path, 'FortisMobile', 'Info.plist')
        if os.path.exists(info_plist_path):
            try:
                with open(info_plist_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                for config in security_configs:
                    if config not in content:
                        self.add_vulnerability('Low', 'iOS Security Config', 
                                             f'Configuração de segurança {config} não encontrada')
            
            except Exception as e:
                print(f"⚠️ Erro ao analisar configurações de segurança iOS: {e}")
    
    def version_matches(self, version, constraint):
        """Verifica se uma versão corresponde a uma restrição"""
        # Implementação simples de verificação de versão
        if constraint.startswith('<'):
            return version < constraint[1:]
        elif constraint.startswith('>'):
            return version > constraint[1:]
        elif constraint.startswith('>='):
            return version >= constraint[2:]
        elif constraint.startswith('<='):
            return version <= constraint[2:]
        else:
            return version == constraint
    
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
        
        report_file = f"security/penetration/reports/fortis_mobile_security_report_{self.results['scan_id']}.json"
        
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
        print("📊 RESUMO DOS TESTES DE SEGURANÇA - MOBILE")
        print("="*50)
        
        summary = self.results['summary']
        print(f"📱 App Mobile: {self.results['mobile_path']}")
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
        print("🚀 Iniciando scan completo de segurança mobile FORTIS...")
        
        # Executa testes
        self.test_package_security()
        self.test_source_code_security()
        self.test_android_security()
        self.test_ios_security()
        
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
        print("Uso: python mobile_security.py <mobile_path>")
        sys.exit(1)
    
    mobile_path = sys.argv[1]
    
    scanner = FortisMobileSecurityScanner(mobile_path)
    success = scanner.run_full_scan()
    
    if success:
        print("\n🎉 Scan de segurança mobile concluído com sucesso!")
        sys.exit(0)
    else:
        print("\n❌ Scan de segurança mobile falhou!")
        sys.exit(1)

if __name__ == "__main__":
    main()
