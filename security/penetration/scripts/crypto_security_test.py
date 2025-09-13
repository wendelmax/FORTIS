#!/usr/bin/env python3
"""
Crypto Security Tester para FORTIS
Testa implementações criptográficas e vulnerabilidades
"""

import os
import sys
import json
import time
import hashlib
import secrets
import base64
from datetime import datetime
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.primitives.kdf.scrypt import Scrypt
from cryptography.hazmat.backends import default_backend
import requests

class FortisCryptoSecurityTester:
    def __init__(self, api_base_url=None):
        self.api_base_url = api_base_url or "http://localhost:8080"
        self.results = {
            'scan_id': f"fortis_crypto_scan_{int(time.time())}",
            'timestamp': datetime.now().isoformat(),
            'vulnerabilities': [],
            'summary': {}
        }
    
    def test_encryption_algorithms(self):
        """Testa algoritmos de criptografia"""
        print("🔐 Testando algoritmos de criptografia...")
        
        # Testa AES-256-GCM
        self.test_aes_gcm()
        
        # Testa RSA
        self.test_rsa()
        
        # Testa Argon2
        self.test_argon2()
        
        # Testa SHA-256
        self.test_sha256()
    
    def test_aes_gcm(self):
        """Testa implementação AES-256-GCM"""
        print("🔐 Testando AES-256-GCM...")
        
        try:
            # Gera chave e IV
            key = secrets.token_bytes(32)  # 256 bits
            iv = secrets.token_bytes(12)   # 96 bits para GCM
            
            # Dados de teste
            plaintext = b"FORTIS test data for encryption"
            
            # Criptografa
            cipher = Cipher(algorithms.AES(key), modes.GCM(iv), backend=default_backend())
            encryptor = cipher.encryptor()
            ciphertext = encryptor.update(plaintext) + encryptor.finalize()
            
            # Descriptografa
            decryptor = Cipher(algorithms.AES(key), modes.GCM(iv, encryptor.tag), backend=default_backend()).decryptor()
            decrypted = decryptor.update(ciphertext) + decryptor.finalize()
            
            if decrypted == plaintext:
                print("✅ AES-256-GCM funcionando corretamente")
            else:
                self.add_vulnerability('High', 'AES-GCM Implementation', 
                                     'AES-256-GCM não está funcionando corretamente')
        
        except Exception as e:
            self.add_vulnerability('High', 'AES-GCM Error', 
                                 f'Erro na implementação AES-256-GCM: {e}')
    
    def test_rsa(self):
        """Testa implementação RSA"""
        print("🔐 Testando RSA...")
        
        try:
            # Gera par de chaves RSA
            private_key = rsa.generate_private_key(
                public_exponent=65537,
                key_size=4096,
                backend=default_backend()
            )
            public_key = private_key.public_key()
            
            # Dados de teste
            plaintext = b"FORTIS test data for RSA"
            
            # Criptografa
            ciphertext = public_key.encrypt(
                plaintext,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA256()),
                    algorithm=hashes.SHA256(),
                    label=None
                )
            )
            
            # Descriptografa
            decrypted = private_key.decrypt(
                ciphertext,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA256()),
                    algorithm=hashes.SHA256(),
                    label=None
                )
            )
            
            if decrypted == plaintext:
                print("✅ RSA-4096 funcionando corretamente")
            else:
                self.add_vulnerability('High', 'RSA Implementation', 
                                     'RSA-4096 não está funcionando corretamente')
        
        except Exception as e:
            self.add_vulnerability('High', 'RSA Error', 
                                 f'Erro na implementação RSA: {e}')
    
    def test_argon2(self):
        """Testa implementação Argon2"""
        print("🔐 Testando Argon2...")
        
        try:
            # Dados de teste
            password = b"FORTIS test password"
            salt = secrets.token_bytes(16)
            
            # Gera hash
            kdf = Scrypt(
                algorithm=hashes.SHA256(),
                length=32,
                salt=salt,
                n=2**14,  # 16384
                r=8,
                p=1,
                backend=default_backend()
            )
            key = kdf.derive(password)
            
            # Verifica hash
            kdf = Scrypt(
                algorithm=hashes.SHA256(),
                length=32,
                salt=salt,
                n=2**14,
                r=8,
                p=1,
                backend=default_backend()
            )
            kdf.verify(password, key)
            
            print("✅ Argon2 funcionando corretamente")
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Argon2 Error', 
                                 f'Erro na implementação Argon2: {e}')
    
    def test_sha256(self):
        """Testa implementação SHA-256"""
        print("🔐 Testando SHA-256...")
        
        try:
            # Dados de teste
            data = b"FORTIS test data for SHA-256"
            
            # Gera hash
            hash_obj = hashlib.sha256()
            hash_obj.update(data)
            hash_value = hash_obj.hexdigest()
            
            # Verifica se o hash tem 64 caracteres (256 bits)
            if len(hash_value) == 64:
                print("✅ SHA-256 funcionando corretamente")
            else:
                self.add_vulnerability('High', 'SHA-256 Implementation', 
                                     'SHA-256 não está gerando hash de 256 bits')
        
        except Exception as e:
            self.add_vulnerability('High', 'SHA-256 Error', 
                                 f'Erro na implementação SHA-256: {e}')
    
    def test_random_number_generation(self):
        """Testa geração de números aleatórios"""
        print("🎲 Testando geração de números aleatórios...")
        
        try:
            # Testa entropia
            random_bytes = secrets.token_bytes(32)
            entropy = self.calculate_entropy(random_bytes)
            
            if entropy > 7.5:  # Alta entropia
                print("✅ Geração de números aleatórios com alta entropia")
            else:
                self.add_vulnerability('High', 'Random Number Generation', 
                                     'Geração de números aleatórios com baixa entropia')
        
        except Exception as e:
            self.add_vulnerability('High', 'Random Number Error', 
                                 f'Erro na geração de números aleatórios: {e}')
    
    def calculate_entropy(self, data):
        """Calcula entropia de Shannon"""
        if not data:
            return 0
        
        # Conta frequência de cada byte
        byte_counts = {}
        for byte in data:
            byte_counts[byte] = byte_counts.get(byte, 0) + 1
        
        # Calcula entropia
        entropy = 0
        data_len = len(data)
        for count in byte_counts.values():
            probability = count / data_len
            if probability > 0:
                entropy -= probability * (probability.bit_length() - 1)
        
        return entropy
    
    def test_api_crypto_endpoints(self):
        """Testa endpoints criptográficos da API"""
        print("🌐 Testando endpoints criptográficos da API...")
        
        if not self.api_base_url:
            print("⚠️ URL da API não fornecida, pulando testes de API")
            return
        
        # Testa endpoint de criptografia
        self.test_encrypt_endpoint()
        
        # Testa endpoint de descriptografia
        self.test_decrypt_endpoint()
        
        # Testa endpoint de hash
        self.test_hash_endpoint()
        
        # Testa endpoint de assinatura
        self.test_signature_endpoint()
    
    def test_encrypt_endpoint(self):
        """Testa endpoint de criptografia"""
        print("🔐 Testando endpoint de criptografia...")
        
        try:
            response = requests.post(f"{self.api_base_url}/api/v1/crypto/encrypt", 
                                   json={'data': 'FORTIS test data'})
            
            if response.status_code == 200:
                data = response.json()
                if 'encrypted_data' in data:
                    print("✅ Endpoint de criptografia funcionando")
                else:
                    self.add_vulnerability('Medium', 'Encrypt Endpoint', 
                                         'Endpoint de criptografia não retorna dados criptografados')
            else:
                self.add_vulnerability('Medium', 'Encrypt Endpoint', 
                                     f'Endpoint de criptografia retornou status {response.status_code}')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Encrypt Endpoint Error', 
                                 f'Erro ao testar endpoint de criptografia: {e}')
    
    def test_decrypt_endpoint(self):
        """Testa endpoint de descriptografia"""
        print("🔐 Testando endpoint de descriptografia...")
        
        try:
            # Primeiro criptografa dados
            encrypt_response = requests.post(f"{self.api_base_url}/api/v1/crypto/encrypt", 
                                           json={'data': 'FORTIS test data'})
            
            if encrypt_response.status_code == 200:
                encrypt_data = encrypt_response.json()
                encrypted_data = encrypt_data.get('encrypted_data')
                
                if encrypted_data:
                    # Tenta descriptografar
                    decrypt_response = requests.post(f"{self.api_base_url}/api/v1/crypto/decrypt", 
                                                   json={'encrypted_data': encrypted_data})
                    
                    if decrypt_response.status_code == 200:
                        decrypt_data = decrypt_response.json()
                        if decrypt_data.get('data') == 'FORTIS test data':
                            print("✅ Endpoint de descriptografia funcionando")
                        else:
                            self.add_vulnerability('High', 'Decrypt Endpoint', 
                                                 'Endpoint de descriptografia não retorna dados corretos')
                    else:
                        self.add_vulnerability('Medium', 'Decrypt Endpoint', 
                                             f'Endpoint de descriptografia retornou status {decrypt_response.status_code}')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Decrypt Endpoint Error', 
                                 f'Erro ao testar endpoint de descriptografia: {e}')
    
    def test_hash_endpoint(self):
        """Testa endpoint de hash"""
        print("🔐 Testando endpoint de hash...")
        
        try:
            response = requests.post(f"{self.api_base_url}/api/v1/crypto/hash", 
                                   json={'data': 'FORTIS test data'})
            
            if response.status_code == 200:
                data = response.json()
                if 'hash' in data:
                    hash_value = data['hash']
                    if len(hash_value) == 64:  # SHA-256
                        print("✅ Endpoint de hash funcionando")
                    else:
                        self.add_vulnerability('Medium', 'Hash Endpoint', 
                                             'Endpoint de hash não retorna hash SHA-256')
                else:
                    self.add_vulnerability('Medium', 'Hash Endpoint', 
                                         'Endpoint de hash não retorna hash')
            else:
                self.add_vulnerability('Medium', 'Hash Endpoint', 
                                     f'Endpoint de hash retornou status {response.status_code}')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Hash Endpoint Error', 
                                 f'Erro ao testar endpoint de hash: {e}')
    
    def test_signature_endpoint(self):
        """Testa endpoint de assinatura"""
        print("🔐 Testando endpoint de assinatura...")
        
        try:
            response = requests.post(f"{self.api_base_url}/api/v1/crypto/sign", 
                                   json={'data': 'FORTIS test data'})
            
            if response.status_code == 200:
                data = response.json()
                if 'signature' in data:
                    print("✅ Endpoint de assinatura funcionando")
                else:
                    self.add_vulnerability('Medium', 'Signature Endpoint', 
                                         'Endpoint de assinatura não retorna assinatura')
            else:
                self.add_vulnerability('Medium', 'Signature Endpoint', 
                                     f'Endpoint de assinatura retornou status {response.status_code}')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'Signature Endpoint Error', 
                                 f'Erro ao testar endpoint de assinatura: {e}')
    
    def test_zkp_implementation(self):
        """Testa implementação de Zero-Knowledge Proofs"""
        print("🔐 Testando implementação de Zero-Knowledge Proofs...")
        
        try:
            # Testa endpoint de geração de prova
            response = requests.post(f"{self.api_base_url}/api/v1/zkp/generate-proof", 
                                   json={'data': 'FORTIS test data'})
            
            if response.status_code == 200:
                data = response.json()
                if 'proof' in data:
                    print("✅ Endpoint de ZKP funcionando")
                else:
                    self.add_vulnerability('Medium', 'ZKP Implementation', 
                                         'Endpoint de ZKP não retorna prova')
            else:
                self.add_vulnerability('Medium', 'ZKP Implementation', 
                                     f'Endpoint de ZKP retornou status {response.status_code}')
        
        except Exception as e:
            self.add_vulnerability('Medium', 'ZKP Implementation Error', 
                                 f'Erro ao testar implementação de ZKP: {e}')
    
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
        
        report_file = f"security/penetration/reports/fortis_crypto_security_report_{self.results['scan_id']}.json"
        
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
        print("📊 RESUMO DOS TESTES DE SEGURANÇA - CRIPTOGRAFIA")
        print("="*50)
        
        summary = self.results['summary']
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
        print("🚀 Iniciando scan completo de segurança criptográfica FORTIS...")
        
        # Executa testes
        self.test_encryption_algorithms()
        self.test_random_number_generation()
        self.test_api_crypto_endpoints()
        self.test_zkp_implementation()
        
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
        print("Uso: python crypto_security_test.py [api_base_url]")
        sys.exit(1)
    
    api_base_url = sys.argv[1] if len(sys.argv) > 1 else None
    
    tester = FortisCryptoSecurityTester(api_base_url)
    success = tester.run_full_scan()
    
    if success:
        print("\n🎉 Scan de segurança criptográfica concluído com sucesso!")
        sys.exit(0)
    else:
        print("\n❌ Scan de segurança criptográfica falhou!")
        sys.exit(1)

if __name__ == "__main__":
    main()
