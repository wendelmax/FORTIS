#!/usr/bin/env python3
"""
Script de sincronização de eleições com o TSE
Sincroniza dados de eleições, candidatos e zonas eleitorais
"""

import asyncio
import aiohttp
import json
import logging
import os
import sys
from datetime import datetime, timezone
from typing import Dict, List, Optional
import argparse

# Configuração de logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class TSESyncClient:
    """Cliente para sincronização com TSE"""
    
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
    
    async def sync_elections(self) -> Dict:
        """Sincroniza todas as eleições ativas"""
        try:
            async with self.session.get(f'{self.base_url}/api/v1/elections/active') as response:
                if response.status == 200:
                    elections = await response.json()
                    logger.info(f"Sincronizadas {len(elections)} eleições")
                    return {
                        'success': True,
                        'elections': elections,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Erro ao sincronizar eleições: {error_text}")
                    return {
                        'success': False,
                        'error': error_text,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
        except Exception as e:
            logger.error(f"Erro na sincronização: {e}")
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now(timezone.utc).isoformat()
            }
    
    async def sync_candidates(self, election_id: str) -> Dict:
        """Sincroniza candidatos de uma eleição"""
        try:
            async with self.session.get(f'{self.base_url}/api/v1/elections/{election_id}/candidates') as response:
                if response.status == 200:
                    candidates = await response.json()
                    logger.info(f"Sincronizados {len(candidates)} candidatos para eleição {election_id}")
                    return {
                        'success': True,
                        'candidates': candidates,
                        'election_id': election_id,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Erro ao sincronizar candidatos: {error_text}")
                    return {
                        'success': False,
                        'error': error_text,
                        'election_id': election_id,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
        except Exception as e:
            logger.error(f"Erro na sincronização de candidatos: {e}")
            return {
                'success': False,
                'error': str(e),
                'election_id': election_id,
                'timestamp': datetime.now(timezone.utc).isoformat()
            }
    
    async def sync_voting_zones(self, election_id: str) -> Dict:
        """Sincroniza zonas eleitorais de uma eleição"""
        try:
            async with self.session.get(f'{self.base_url}/api/v1/elections/{election_id}/zones') as response:
                if response.status == 200:
                    zones = await response.json()
                    logger.info(f"Sincronizadas {len(zones)} zonas eleitorais para eleição {election_id}")
                    return {
                        'success': True,
                        'zones': zones,
                        'election_id': election_id,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Erro ao sincronizar zonas eleitorais: {error_text}")
                    return {
                        'success': False,
                        'error': error_text,
                        'election_id': election_id,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
        except Exception as e:
            logger.error(f"Erro na sincronização de zonas: {e}")
            return {
                'success': False,
                'error': str(e),
                'election_id': election_id,
                'timestamp': datetime.now(timezone.utc).isoformat()
            }
    
    async def sync_election_rules(self, election_id: str) -> Dict:
        """Sincroniza regras de uma eleição"""
        try:
            async with self.session.get(f'{self.base_url}/api/v1/elections/{election_id}/rules') as response:
                if response.status == 200:
                    rules = await response.json()
                    logger.info(f"Sincronizadas regras para eleição {election_id}")
                    return {
                        'success': True,
                        'rules': rules,
                        'election_id': election_id,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Erro ao sincronizar regras: {error_text}")
                    return {
                        'success': False,
                        'error': error_text,
                        'election_id': election_id,
                        'timestamp': datetime.now(timezone.utc).isoformat()
                    }
        except Exception as e:
            logger.error(f"Erro na sincronização de regras: {e}")
            return {
                'success': False,
                'error': str(e),
                'election_id': election_id,
                'timestamp': datetime.now(timezone.utc).isoformat()
            }

async def main():
    """Função principal"""
    parser = argparse.ArgumentParser(description='Sincronização de dados TSE')
    parser.add_argument('--base-url', default='https://api.tse.jus.br', help='URL base da API TSE')
    parser.add_argument('--api-key', required=True, help='Chave da API TSE')
    parser.add_argument('--election-id', help='ID específico da eleição para sincronizar')
    parser.add_argument('--output', help='Arquivo de saída para salvar resultados')
    parser.add_argument('--verbose', '-v', action='store_true', help='Log verboso')
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    results = {
        'sync_start': datetime.now(timezone.utc).isoformat(),
        'elections': [],
        'errors': []
    }
    
    async with TSESyncClient(args.base_url, args.api_key) as client:
        # Sincronizar eleições
        elections_result = await client.sync_elections()
        results['elections'] = elections_result
        
        if elections_result['success'] and elections_result.get('elections'):
            # Se especificou uma eleição, sincronizar apenas ela
            if args.election_id:
                target_elections = [e for e in elections_result['elections'] if e['election_id'] == args.election_id]
            else:
                target_elections = elections_result['elections']
            
            # Sincronizar dados detalhados de cada eleição
            for election in target_elections:
                election_id = election['election_id']
                logger.info(f"Sincronizando dados detalhados da eleição {election_id}")
                
                # Sincronizar candidatos
                candidates_result = await client.sync_candidates(election_id)
                election['candidates_sync'] = candidates_result
                
                # Sincronizar zonas eleitorais
                zones_result = await client.sync_voting_zones(election_id)
                election['zones_sync'] = zones_result
                
                # Sincronizar regras
                rules_result = await client.sync_election_rules(election_id)
                election['rules_sync'] = rules_result
                
                # Verificar se houve erros
                if not candidates_result['success']:
                    results['errors'].append(f"Erro ao sincronizar candidatos da eleição {election_id}")
                if not zones_result['success']:
                    results['errors'].append(f"Erro ao sincronizar zonas da eleição {election_id}")
                if not rules_result['success']:
                    results['errors'].append(f"Erro ao sincronizar regras da eleição {election_id}")
    
    results['sync_end'] = datetime.now(timezone.utc).isoformat()
    results['success'] = len(results['errors']) == 0
    
    # Salvar resultados
    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            json.dump(results, f, indent=2, ensure_ascii=False)
        logger.info(f"Resultados salvos em {args.output}")
    else:
        print(json.dumps(results, indent=2, ensure_ascii=False))
    
    # Log final
    if results['success']:
        logger.info("Sincronização concluída com sucesso")
    else:
        logger.error(f"Sincronização concluída com {len(results['errors'])} erros")
        sys.exit(1)

if __name__ == '__main__':
    asyncio.run(main())
