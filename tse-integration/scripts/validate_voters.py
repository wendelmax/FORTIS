#!/usr/bin/env python3
"""
Script de validação de eleitores com o TSE
Valida CPFs e títulos de eleitor através da API TSE
"""

import asyncio
import aiohttp
import json
import logging
import os
import sys
import csv
from datetime import datetime, timezone
from typing import Dict, List, Optional
import argparse

# Configuração de logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class VoterValidator:
    """Validador de eleitores"""
    
    def __init__(self, base_url: str, api_key: str):
        self.base_url = base_url
        self.api_key = api_key
        self.session = None
        
    async def __aenter__(self):
        self.session = aiohttp.ClientSession(
            headers={
                'Authorization': f'Bearer {self.api_key}',
                'Content-Type': 'application/json'
            }
        )
        return self
        
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()
    
    async def validate_cpf(self, cpf: str) -> Dict:
        """Valida eleitor por CPF"""
        try:
            # Limpar CPF (remover pontos, traços, espaços)
            clean_cpf = ''.join(filter(str.isdigit, cpf))
            
            if len(clean_cpf) != 11:
                return {
                    'cpf': cpf,
                    'valid': False,
                    'error': 'CPF deve ter 11 dígitos',
                    'timestamp': datetime.now(timezone.utc).isoformat()
                }
            
            async with self.session.get(f'{self.base_url}/api/v1/voter/validate/cpf/{clean_cpf}') as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info(f"CPF {cpf} validado com sucesso")
                    return {
                        'cpf': cpf,
                        'valid': result.get('valid', False),
                        'voter_data': result.get('voter_data'),
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Erro ao validar CPF {cpf}: {error_text}")
                    return {
                        'cpf': cpf,
                        'valid': False,
                        'error': error_text,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
        except Exception as e:
            logger.error(f"Erro na validação do CPF {cpf}: {e}")
            return {
                'cpf': cpf,
                'valid': False,
                'error': str(e),
                'timestamp': datetime.now(timezone.utc).isoformat()
            }
    
    async def validate_voter_id(self, voter_id: str) -> Dict:
        """Valida eleitor por título de eleitor"""
        try:
            # Limpar título (remover pontos, traços, espaços)
            clean_voter_id = ''.join(filter(str.isdigit, voter_id))
            
            if len(clean_voter_id) < 8:
                return {
                    'voter_id': voter_id,
                    'valid': False,
                    'error': 'Título deve ter pelo menos 8 dígitos',
                    'timestamp': datetime.now(timezone.utc).isoformat()
                }
            
            async with self.session.get(f'{self.base_url}/api/v1/voter/validate/id/{clean_voter_id}') as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info(f"Título {voter_id} validado com sucesso")
                    return {
                        'voter_id': voter_id,
                        'valid': result.get('valid', False),
                        'voter_data': result.get('voter_data'),
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Erro ao validar título {voter_id}: {error_text}")
                    return {
                        'voter_id': voter_id,
                        'valid': False,
                        'error': error_text,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
        except Exception as e:
            logger.error(f"Erro na validação do título {voter_id}: {e}")
            return {
                'voter_id': voter_id,
                'valid': False,
                'error': str(e),
                'timestamp': datetime.now(timezone.utc).isoformat()
            }
    
    async def validate_batch(self, voters: List[Dict]) -> List[Dict]:
        """Valida lote de eleitores"""
        results = []
        
        for voter in voters:
            if 'cpf' in voter:
                result = await self.validate_cpf(voter['cpf'])
            elif 'voter_id' in voter:
                result = await self.validate_voter_id(voter['voter_id'])
            else:
                result = {
                    'error': 'Voter deve ter CPF ou voter_id',
                    'timestamp': datetime.now(timezone.utc).isoformat()
                }
            
            results.append(result)
            
            # Pequena pausa para não sobrecarregar a API
            await asyncio.sleep(0.1)
        
        return results

def load_voters_from_csv(filename: str) -> List[Dict]:
    """Carrega eleitores de arquivo CSV"""
    voters = []
    
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            reader = csv.DictReader(f)
            for row in reader:
                voter = {}
                if 'cpf' in row and row['cpf']:
                    voter['cpf'] = row['cpf']
                if 'voter_id' in row and row['voter_id']:
                    voter['voter_id'] = row['voter_id']
                if 'name' in row and row['name']:
                    voter['name'] = row['name']
                if voter:  # Só adicionar se tem pelo menos CPF ou título
                    voters.append(voter)
        
        logger.info(f"Carregados {len(voters)} eleitores do arquivo {filename}")
        return voters
    except Exception as e:
        logger.error(f"Erro ao carregar arquivo CSV: {e}")
        return []

def save_results_to_csv(results: List[Dict], filename: str):
    """Salva resultados em arquivo CSV"""
    try:
        with open(filename, 'w', newline='', encoding='utf-8') as f:
            if not results:
                return
            
            # Determinar campos baseado no primeiro resultado
            fieldnames = set()
            for result in results:
                fieldnames.update(result.keys())
            
            writer = csv.DictWriter(f, fieldnames=sorted(fieldnames))
            writer.writeheader()
            writer.writerows(results)
        
        logger.info(f"Resultados salvos em {filename}")
    except Exception as e:
        logger.error(f"Erro ao salvar resultados: {e}")

async def main():
    """Função principal"""
    parser = argparse.ArgumentParser(description='Validação de eleitores TSE')
    parser.add_argument('--base-url', default='https://api.tse.jus.br', help='URL base da API TSE')
    parser.add_argument('--api-key', required=True, help='Chave da API TSE')
    parser.add_argument('--cpf', help='CPF específico para validar')
    parser.add_argument('--voter-id', help='Título específico para validar')
    parser.add_argument('--input-csv', help='Arquivo CSV com eleitores para validar')
    parser.add_argument('--output-csv', help='Arquivo CSV para salvar resultados')
    parser.add_argument('--output-json', help='Arquivo JSON para salvar resultados')
    parser.add_argument('--verbose', '-v', action='store_true', help='Log verboso')
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    # Validar argumentos
    if not args.cpf and not args.voter_id and not args.input_csv:
        logger.error("Deve especificar --cpf, --voter-id ou --input-csv")
        sys.exit(1)
    
    results = {
        'validation_start': datetime.now(timezone.utc).isoformat(),
        'validations': [],
        'summary': {
            'total': 0,
            'valid': 0,
            'invalid': 0,
            'errors': 0
        }
    }
    
    async with VoterValidator(args.base_url, args.api_key) as validator:
        if args.cpf:
            # Validar CPF específico
            result = await validator.validate_cpf(args.cpf)
            results['validations'].append(result)
            
        elif args.voter_id:
            # Validar título específico
            result = await validator.validate_voter_id(args.voter_id)
            results['validations'].append(result)
            
        elif args.input_csv:
            # Validar lote de eleitores
            voters = load_voters_from_csv(args.input_csv)
            if not voters:
                logger.error("Nenhum eleitor encontrado no arquivo CSV")
                sys.exit(1)
            
            validation_results = await validator.validate_batch(voters)
            results['validations'] = validation_results
    
    # Calcular estatísticas
    for validation in results['validations']:
        results['summary']['total'] += 1
        if validation.get('valid'):
            results['summary']['valid'] += 1
        elif 'error' in validation:
            results['summary']['errors'] += 1
        else:
            results['summary']['invalid'] += 1
    
    results['validation_end'] = datetime.now(timezone.utc).isoformat()
    
    # Salvar resultados
    if args.output_csv:
        save_results_to_csv(results['validations'], args.output_csv)
    
    if args.output_json:
        with open(args.output_json, 'w', encoding='utf-8') as f:
            json.dump(results, f, indent=2, ensure_ascii=False)
        logger.info(f"Resultados salvos em {args.output_json}")
    
    # Exibir resumo
    summary = results['summary']
    logger.info(f"Validação concluída: {summary['total']} total, {summary['valid']} válidos, {summary['invalid']} inválidos, {summary['errors']} erros")
    
    if not args.output_csv and not args.output_json:
        print(json.dumps(results, indent=2, ensure_ascii=False))

if __name__ == '__main__':
    asyncio.run(main())
