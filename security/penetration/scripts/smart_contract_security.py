#!/usr/bin/env python3
"""
Smart Contract Security Scanner para FORTIS
Testa vulnerabilidades em smart contracts Solidity
"""

import os
import sys
import json
import time
import subprocess
from datetime import datetime
from web3 import Web3
from mythril import Mythril
from slither import Slither

class FortisSmartContractScanner:
    def __init__(self, contract_path, rpc_url=None):
        self.contract_path = contract_path
        self.rpc_url = rpc_url or "http://localhost:8545"
        self.results = {
            'scan_id': f"fortis_smart_contract_scan_{int(time.time())}",
            'contract_path': contract_path,
            'timestamp': datetime.now().isoformat(),
            'vulnerabilities': [],
            'summary': {}
        }
    
    def run_mythril_analysis(self):
        """Executa análise com Mythril"""
        print("🔍 Executando análise Mythril...")
        
        try:
            # Configura Mythril
            mythril = Mythril()
            
            # Analisa o contrato
            result = mythril.analyze_contract(self.contract_path)
            
            if result:
                for issue in result:
                    vulnerability = {
                        'tool': 'Mythril',
                        'severity': issue.get('severity', 'Unknown'),
                        'title': issue.get('title', 'Unknown'),
                        'description': issue.get('description', ''),
                        'swc_id': issue.get('swc_id', ''),
                        'function': issue.get('function', ''),
                        'line': issue.get('line', ''),
                        'code': issue.get('code', '')
                    }
                    self.results['vulnerabilities'].append(vulnerability)
            
            print("✅ Análise Mythril concluída")
            return True
        except Exception as e:
            print(f"❌ Erro na análise Mythril: {e}")
            return False
    
    def run_slither_analysis(self):
        """Executa análise com Slither"""
        print("🔍 Executando análise Slither...")
        
        try:
            # Analisa o contrato com Slither
            slither = Slither(self.contract_path)
            
            for contract in slither.contracts:
                for function in contract.functions:
                    # Verifica vulnerabilidades comuns
                    self.check_reentrancy(function)
                    self.check_integer_overflow(function)
                    self.check_access_control(function)
                    self.check_external_calls(function)
                    self.check_timestamp_dependence(function)
                    self.check_randomness(function)
            
            print("✅ Análise Slither concluída")
            return True
        except Exception as e:
            print(f"❌ Erro na análise Slither: {e}")
            return False
    
    def check_reentrancy(self, function):
        """Verifica vulnerabilidades de reentrancy"""
        if not function.is_implemented:
            return
        
        # Verifica se há chamadas externas antes de atualizar estado
        external_calls = []
        state_updates = []
        
        for node in function.nodes:
            if node.type == 'CALL':
                external_calls.append(node)
            elif node.type == 'SSTORE':
                state_updates.append(node)
        
        if external_calls and state_updates:
            # Verifica se chamadas externas vêm antes de atualizações de estado
            for call in external_calls:
                for update in state_updates:
                    if call.node_id < update.node_id:
                        self.add_vulnerability('High', 'Reentrancy', 
                                             f'Possível vulnerabilidade de reentrancy em {function.name}',
                                             function.name, call.line)
    
    def check_integer_overflow(self, function):
        """Verifica vulnerabilidades de integer overflow"""
        if not function.is_implemented:
            return
        
        # Verifica operações aritméticas sem verificação de overflow
        for node in function.nodes:
            if node.type in ['ADD', 'SUB', 'MUL', 'DIV']:
                # Verifica se há verificação de overflow
                has_overflow_check = False
                for child in node.children:
                    if child.type == 'LT' or child.type == 'GT':
                        has_overflow_check = True
                        break
                
                if not has_overflow_check:
                    self.add_vulnerability('Medium', 'Integer Overflow', 
                                         f'Possível integer overflow em {function.name}',
                                         function.name, node.line)
    
    def check_access_control(self, function):
        """Verifica controles de acesso"""
        if not function.is_implemented:
            return
        
        # Verifica se funções críticas têm controles de acesso
        critical_functions = ['vote', 'addCandidate', 'removeCandidate', 'startElection', 'endElection']
        
        if function.name in critical_functions:
            has_access_control = False
            
            for node in function.nodes:
                if node.type == 'REQUIRE':
                    # Verifica se há verificação de permissões
                    if 'onlyOwner' in str(node) or 'onlyAdmin' in str(node) or 'msg.sender' in str(node):
                        has_access_control = True
                        break
            
            if not has_access_control:
                self.add_vulnerability('High', 'Access Control', 
                                     f'Função {function.name} sem controle de acesso adequado',
                                     function.name, function.line)
    
    def check_external_calls(self, function):
        """Verifica chamadas externas perigosas"""
        if not function.is_implemented:
            return
        
        for node in function.nodes:
            if node.type == 'CALL':
                # Verifica se há verificação de retorno
                has_return_check = False
                for child in node.children:
                    if child.type == 'REQUIRE':
                        has_return_check = True
                        break
                
                if not has_return_check:
                    self.add_vulnerability('Medium', 'External Call', 
                                         f'Chamada externa sem verificação de retorno em {function.name}',
                                         function.name, node.line)
    
    def check_timestamp_dependence(self, function):
        """Verifica dependência de timestamp"""
        if not function.is_implemented:
            return
        
        for node in function.nodes:
            if node.type == 'TIMESTAMP':
                self.add_vulnerability('Low', 'Timestamp Dependence', 
                                     f'Uso de timestamp em {function.name} pode ser manipulado',
                                     function.name, node.line)
    
    def check_randomness(self, function):
        """Verifica uso de aleatoriedade insegura"""
        if not function.is_implemented:
            return
        
        for node in function.nodes:
            if node.type == 'BLOCKHASH' or node.type == 'TIMESTAMP':
                self.add_vulnerability('Medium', 'Insecure Randomness', 
                                     f'Uso de aleatoriedade insegura em {function.name}',
                                     function.name, node.line)
    
    def test_specific_vulnerabilities(self):
        """Testa vulnerabilidades específicas do FORTIS"""
        print("🎯 Testando vulnerabilidades específicas do FORTIS...")
        
        # Testa vulnerabilidades específicas de votação
        self.test_vote_manipulation()
        self.test_election_manipulation()
        self.test_audit_manipulation()
        self.test_identity_manipulation()
    
    def test_vote_manipulation(self):
        """Testa manipulação de votos"""
        print("🗳️ Testando manipulação de votos...")
        
        # Verifica se há proteção contra votos duplicados
        with open(self.contract_path, 'r') as f:
            content = f.read()
        
        if 'mapping(address => bool) public hasVoted' in content:
            print("✅ Proteção contra votos duplicados encontrada")
        else:
            self.add_vulnerability('High', 'Vote Duplication', 
                                 'Proteção contra votos duplicados não encontrada')
        
        # Verifica se há verificação de elegibilidade
        if 'require(isEligible[msg.sender]' in content:
            print("✅ Verificação de elegibilidade encontrada")
        else:
            self.add_vulnerability('Medium', 'Eligibility Check', 
                                 'Verificação de elegibilidade não encontrada')
    
    def test_election_manipulation(self):
        """Testa manipulação de eleições"""
        print("🏛️ Testando manipulação de eleições...")
        
        with open(self.contract_path, 'r') as f:
            content = f.read()
        
        # Verifica se há controle de tempo de eleição
        if 'require(block.timestamp >= startTime' in content:
            print("✅ Controle de tempo de eleição encontrado")
        else:
            self.add_vulnerability('Medium', 'Election Timing', 
                                 'Controle de tempo de eleição não encontrado')
        
        # Verifica se há proteção contra alteração de candidatos
        if 'onlyOwner' in content and 'addCandidate' in content:
            print("✅ Proteção de candidatos encontrada")
        else:
            self.add_vulnerability('High', 'Candidate Protection', 
                                 'Proteção contra alteração de candidatos não encontrada')
    
    def test_audit_manipulation(self):
        """Testa manipulação de auditoria"""
        print("📊 Testando manipulação de auditoria...")
        
        with open(self.contract_path, 'r') as f:
            content = f.read()
        
        # Verifica se há logs de auditoria
        if 'event' in content and 'VoteCast' in content:
            print("✅ Logs de auditoria encontrados")
        else:
            self.add_vulnerability('Medium', 'Audit Logging', 
                                 'Logs de auditoria não encontrados')
        
        # Verifica se há hash de integridade
        if 'keccak256' in content and 'vote' in content:
            print("✅ Hash de integridade encontrado")
        else:
            self.add_vulnerability('Medium', 'Integrity Hash', 
                                 'Hash de integridade não encontrado')
    
    def test_identity_manipulation(self):
        """Testa manipulação de identidade"""
        print("🆔 Testando manipulação de identidade...")
        
        with open(self.contract_path, 'r') as f:
            content = f.read()
        
        # Verifica se há verificação de identidade
        if 'require(identityVerified[msg.sender]' in content:
            print("✅ Verificação de identidade encontrada")
        else:
            self.add_vulnerability('High', 'Identity Verification', 
                                 'Verificação de identidade não encontrada')
        
        # Verifica se há proteção contra sybil attacks
        if 'mapping(address => bool) public registered' in content:
            print("✅ Proteção contra sybil attacks encontrada")
        else:
            self.add_vulnerability('Medium', 'Sybil Protection', 
                                 'Proteção contra sybil attacks não encontrada')
    
    def add_vulnerability(self, severity, title, description, function='', line=''):
        """Adiciona vulnerabilidade aos resultados"""
        vulnerability = {
            'tool': 'Custom',
            'severity': severity,
            'title': title,
            'description': description,
            'function': function,
            'line': line,
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
        
        report_file = f"security/penetration/reports/fortis_smart_contract_report_{self.results['scan_id']}.json"
        
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
        print("📊 RESUMO DOS TESTES DE SEGURANÇA - SMART CONTRACTS")
        print("="*50)
        
        summary = self.results['summary']
        print(f"📁 Contrato: {self.results['contract_path']}")
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
            function = vuln.get('function', '')
            line = vuln.get('line', '')
            
            print(f"\n{severity.upper()} - {title}")
            print(f"  Descrição: {description}")
            if function:
                print(f"  Função: {function}")
            if line:
                print(f"  Linha: {line}")
    
    def run_full_scan(self):
        """Executa scan completo de segurança"""
        print("🚀 Iniciando scan completo de segurança de smart contracts FORTIS...")
        
        # Executa análises
        self.run_mythril_analysis()
        self.run_slither_analysis()
        self.test_specific_vulnerabilities()
        
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
        print("Uso: python smart_contract_security.py <contract_path> [rpc_url]")
        sys.exit(1)
    
    contract_path = sys.argv[1]
    rpc_url = sys.argv[2] if len(sys.argv) > 2 else None
    
    scanner = FortisSmartContractScanner(contract_path, rpc_url)
    success = scanner.run_full_scan()
    
    if success:
        print("\n🎉 Scan de segurança de smart contracts concluído com sucesso!")
        sys.exit(0)
    else:
        print("\n❌ Scan de segurança de smart contracts falhou!")
        sys.exit(1)

if __name__ == "__main__":
    main()
