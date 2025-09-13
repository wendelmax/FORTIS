#!/usr/bin/env python3
"""
Módulo de validação de eleitores
Implementa validações específicas para dados eleitorais brasileiros
"""

import re
import logging
from datetime import datetime, date
from typing import Dict, List, Optional, Tuple
import hashlib

logger = logging.getLogger(__name__)

class VoterValidator:
    """Validador de dados eleitorais"""
    
    def __init__(self):
        self.cpf_pattern = re.compile(r'^\d{11}$')
        self.voter_id_pattern = re.compile(r'^\d{8,12}$')
        self.email_pattern = re.compile(r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$')
    
    def validate_cpf(self, cpf: str) -> Tuple[bool, str]:
        """
        Valida CPF brasileiro
        
        Args:
            cpf: CPF a ser validado (apenas números)
            
        Returns:
            Tuple[bool, str]: (é_válido, mensagem_erro)
        """
        if not cpf:
            return False, "CPF não pode ser vazio"
        
        # Limpar CPF (remover pontos, traços, espaços)
        clean_cpf = re.sub(r'[^\d]', '', cpf)
        
        if not self.cpf_pattern.match(clean_cpf):
            return False, "CPF deve ter exatamente 11 dígitos"
        
        # Verificar se todos os dígitos são iguais
        if len(set(clean_cpf)) == 1:
            return False, "CPF inválido (todos os dígitos iguais)"
        
        # Calcular primeiro dígito verificador
        sum1 = 0
        for i in range(9):
            sum1 += int(clean_cpf[i]) * (10 - i)
        
        remainder1 = sum1 % 11
        digit1 = 0 if remainder1 < 2 else 11 - remainder1
        
        if int(clean_cpf[9]) != digit1:
            return False, "CPF inválido (primeiro dígito verificador incorreto)"
        
        # Calcular segundo dígito verificador
        sum2 = 0
        for i in range(10):
            sum2 += int(clean_cpf[i]) * (11 - i)
        
        remainder2 = sum2 % 11
        digit2 = 0 if remainder2 < 2 else 11 - remainder2
        
        if int(clean_cpf[10]) != digit2:
            return False, "CPF inválido (segundo dígito verificador incorreto)"
        
        return True, "CPF válido"
    
    def validate_voter_id(self, voter_id: str) -> Tuple[bool, str]:
        """
        Valida título de eleitor brasileiro
        
        Args:
            voter_id: Título de eleitor a ser validado
            
        Returns:
            Tuple[bool, str]: (é_válido, mensagem_erro)
        """
        if not voter_id:
            return False, "Título de eleitor não pode ser vazio"
        
        # Limpar título (remover pontos, traços, espaços)
        clean_id = re.sub(r'[^\d]', '', voter_id)
        
        if not self.voter_id_pattern.match(clean_id):
            return False, "Título de eleitor deve ter entre 8 e 12 dígitos"
        
        # Validação básica do título (algoritmo simplificado)
        if len(clean_id) < 8:
            return False, "Título de eleitor muito curto"
        
        if len(clean_id) > 12:
            return False, "Título de eleitor muito longo"
        
        # Verificar se todos os dígitos são iguais
        if len(set(clean_id)) == 1:
            return False, "Título de eleitor inválido (todos os dígitos iguais)"
        
        return True, "Título de eleitor válido"
    
    def validate_birth_date(self, birth_date: str) -> Tuple[bool, str]:
        """
        Valida data de nascimento
        
        Args:
            birth_date: Data no formato YYYY-MM-DD
            
        Returns:
            Tuple[bool, str]: (é_válida, mensagem_erro)
        """
        if not birth_date:
            return False, "Data de nascimento não pode ser vazia"
        
        try:
            # Tentar parsear a data
            parsed_date = datetime.strptime(birth_date, '%Y-%m-%d').date()
            
            # Verificar se a data não é no futuro
            if parsed_date > date.today():
                return False, "Data de nascimento não pode ser no futuro"
            
            # Verificar se a pessoa tem pelo menos 16 anos (idade mínima para votar)
            today = date.today()
            age = today.year - parsed_date.year - ((today.month, today.day) < (parsed_date.month, parsed_date.day))
            
            if age < 16:
                return False, "Idade mínima para votar é 16 anos"
            
            # Verificar se a pessoa não é muito velha (mais de 120 anos)
            if age > 120:
                return False, "Data de nascimento inválida (idade muito alta)"
            
            return True, "Data de nascimento válida"
            
        except ValueError:
            return False, "Formato de data inválido (use YYYY-MM-DD)"
    
    def validate_mother_name(self, mother_name: str) -> Tuple[bool, str]:
        """
        Valida nome da mãe
        
        Args:
            mother_name: Nome da mãe
            
        Returns:
            Tuple[bool, str]: (é_válido, mensagem_erro)
        """
        if not mother_name:
            return False, "Nome da mãe não pode ser vazio"
        
        # Limpar nome (remover espaços extras)
        clean_name = ' '.join(mother_name.split())
        
        if len(clean_name) < 3:
            return False, "Nome da mãe muito curto"
        
        if len(clean_name) > 100:
            return False, "Nome da mãe muito longo"
        
        # Verificar se contém apenas letras, espaços e caracteres especiais comuns
        if not re.match(r'^[a-zA-ZÀ-ÿ\s\'-]+$', clean_name):
            return False, "Nome da mãe contém caracteres inválidos"
        
        return True, "Nome da mãe válido"
    
    def validate_voting_zone(self, zone: str) -> Tuple[bool, str]:
        """
        Valida zona eleitoral
        
        Args:
            zone: Código da zona eleitoral
            
        Returns:
            Tuple[bool, str]: (é_válida, mensagem_erro)
        """
        if not zone:
            return False, "Zona eleitoral não pode ser vazia"
        
        # Limpar zona (remover espaços)
        clean_zone = zone.strip()
        
        if not clean_zone.isdigit():
            return False, "Zona eleitoral deve conter apenas números"
        
        if len(clean_zone) < 1 or len(clean_zone) > 4:
            return False, "Zona eleitoral deve ter entre 1 e 4 dígitos"
        
        zone_num = int(clean_zone)
        if zone_num < 1 or zone_num > 9999:
            return False, "Zona eleitoral deve estar entre 1 e 9999"
        
        return True, "Zona eleitoral válida"
    
    def validate_voting_section(self, section: str) -> Tuple[bool, str]:
        """
        Valida seção eleitoral
        
        Args:
            section: Código da seção eleitoral
            
        Returns:
            Tuple[bool, str]: (é_válida, mensagem_erro)
        """
        if not section:
            return False, "Seção eleitoral não pode ser vazia"
        
        # Limpar seção (remover espaços)
        clean_section = section.strip()
        
        if not clean_section.isdigit():
            return False, "Seção eleitoral deve conter apenas números"
        
        if len(clean_section) < 1 or len(clean_section) > 4:
            return False, "Seção eleitoral deve ter entre 1 e 4 dígitos"
        
        section_num = int(clean_section)
        if section_num < 1 or section_num > 9999:
            return False, "Seção eleitoral deve estar entre 1 e 9999"
        
        return True, "Seção eleitoral válida"
    
    def validate_email(self, email: str) -> Tuple[bool, str]:
        """
        Valida endereço de email
        
        Args:
            email: Endereço de email
            
        Returns:
            Tuple[bool, str]: (é_válido, mensagem_erro)
        """
        if not email:
            return True, "Email é opcional"  # Email é opcional
        
        if not self.email_pattern.match(email):
            return False, "Formato de email inválido"
        
        if len(email) > 254:
            return False, "Email muito longo"
        
        return True, "Email válido"
    
    def validate_phone(self, phone: str) -> Tuple[bool, str]:
        """
        Valida número de telefone brasileiro
        
        Args:
            phone: Número de telefone
            
        Returns:
            Tuple[bool, str]: (é_válido, mensagem_erro)
        """
        if not phone:
            return True, "Telefone é opcional"  # Telefone é opcional
        
        # Limpar telefone (remover caracteres não numéricos)
        clean_phone = re.sub(r'[^\d]', '', phone)
        
        # Verificar se tem 10 ou 11 dígitos (com DDD)
        if len(clean_phone) not in [10, 11]:
            return False, "Telefone deve ter 10 ou 11 dígitos (com DDD)"
        
        # Verificar se começa com DDD válido (11-99)
        ddd = int(clean_phone[:2])
        if ddd < 11 or ddd > 99:
            return False, "DDD inválido"
        
        return True, "Telefone válido"
    
    def validate_voter_data(self, voter_data: Dict) -> Dict:
        """
        Valida dados completos do eleitor
        
        Args:
            voter_data: Dicionário com dados do eleitor
            
        Returns:
            Dict: Resultado da validação
        """
        result = {
            'valid': True,
            'errors': [],
            'warnings': [],
            'validated_fields': {}
        }
        
        # Validar CPF
        if 'cpf' in voter_data:
            is_valid, message = self.validate_cpf(voter_data['cpf'])
            result['validated_fields']['cpf'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['valid'] = False
                result['errors'].append(f"CPF: {message}")
        
        # Validar título de eleitor
        if 'voter_id' in voter_data:
            is_valid, message = self.validate_voter_id(voter_data['voter_id'])
            result['validated_fields']['voter_id'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['valid'] = False
                result['errors'].append(f"Título: {message}")
        
        # Validar data de nascimento
        if 'birth_date' in voter_data:
            is_valid, message = self.validate_birth_date(voter_data['birth_date'])
            result['validated_fields']['birth_date'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['valid'] = False
                result['errors'].append(f"Data de nascimento: {message}")
        
        # Validar nome da mãe
        if 'mother_name' in voter_data:
            is_valid, message = self.validate_mother_name(voter_data['mother_name'])
            result['validated_fields']['mother_name'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['valid'] = False
                result['errors'].append(f"Nome da mãe: {message}")
        
        # Validar zona eleitoral
        if 'voting_zone' in voter_data:
            is_valid, message = self.validate_voting_zone(voter_data['voting_zone'])
            result['validated_fields']['voting_zone'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['valid'] = False
                result['errors'].append(f"Zona eleitoral: {message}")
        
        # Validar seção eleitoral
        if 'voting_section' in voter_data:
            is_valid, message = self.validate_voting_section(voter_data['voting_section'])
            result['validated_fields']['voting_section'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['valid'] = False
                result['errors'].append(f"Seção eleitoral: {message}")
        
        # Validar email (opcional)
        if 'email' in voter_data:
            is_valid, message = self.validate_email(voter_data['email'])
            result['validated_fields']['email'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['warnings'].append(f"Email: {message}")
        
        # Validar telefone (opcional)
        if 'phone' in voter_data:
            is_valid, message = self.validate_phone(voter_data['phone'])
            result['validated_fields']['phone'] = {
                'valid': is_valid,
                'message': message
            }
            if not is_valid:
                result['warnings'].append(f"Telefone: {message}")
        
        return result
    
    def generate_voter_hash(self, voter_data: Dict) -> str:
        """
        Gera hash único para o eleitor baseado nos dados principais
        
        Args:
            voter_data: Dados do eleitor
            
        Returns:
            str: Hash SHA-256 dos dados
        """
        # Criar string com dados principais para hash
        hash_data = []
        
        if 'cpf' in voter_data:
            hash_data.append(f"cpf:{voter_data['cpf']}")
        
        if 'voter_id' in voter_data:
            hash_data.append(f"voter_id:{voter_data['voter_id']}")
        
        if 'birth_date' in voter_data:
            hash_data.append(f"birth_date:{voter_data['birth_date']}")
        
        if 'mother_name' in voter_data:
            hash_data.append(f"mother_name:{voter_data['mother_name']}")
        
        # Ordenar para garantir consistência
        hash_data.sort()
        
        # Criar hash
        hash_string = '|'.join(hash_data)
        return hashlib.sha256(hash_string.encode('utf-8')).hexdigest()

# Exemplo de uso
if __name__ == '__main__':
    validator = VoterValidator()
    
    # Exemplo de validação
    voter_data = {
        'cpf': '12345678901',
        'voter_id': '12345678',
        'birth_date': '1990-01-01',
        'mother_name': 'Maria Silva',
        'voting_zone': '123',
        'voting_section': '456',
        'email': 'eleitor@email.com',
        'phone': '(11) 99999-9999'
    }
    
    result = validator.validate_voter_data(voter_data)
    print(f"Validação: {'Válido' if result['valid'] else 'Inválido'}")
    print(f"Erros: {result['errors']}")
    print(f"Avisos: {result['warnings']}")
    
    # Gerar hash
    voter_hash = validator.generate_voter_hash(voter_data)
    print(f"Hash do eleitor: {voter_hash}")
