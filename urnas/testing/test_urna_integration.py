#!/usr/bin/env python3
"""
FORTIS Urna Integration Tests
Testes de integração para urnas eletrônicas FORTIS

Copyright (C) 2024 FORTIS Team
License: MIT
"""

import asyncio
import json
import uuid
import time
import logging
from datetime import datetime, timezone
from typing import Dict, List, Optional, Any
import aiohttp
import pytest

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class FortisUrnaTester:
    """Testador de integração para urnas FORTIS"""
    
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url
        self.session: Optional[aiohttp.ClientSession] = None
        self.test_results: List[Dict[str, Any]] = []
        
    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self
        
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()
    
    async def test_urna_registration(self) -> bool:
        """Testa registro de nova urna"""
        logger.info("Testing urna registration...")
        
        urna_data = {
            "serial_number": f"URNA-TEST-{uuid.uuid4().hex[:8]}",
            "model": "FORTIS-2025",
            "location": {
                "state": "SP",
                "city": "São Paulo",
                "zone": "001",
                "section": "001",
                "address": "Rua de Teste, 123",
                "coordinates": {
                    "latitude": -23.5505,
                    "longitude": -46.6333
                }
            },
            "status": "Inactive"
        }
        
        try:
            async with self.session.post(
                f"{self.base_url}/api/v1/urnas/register",
                json=urna_data
            ) as response:
                if response.status == 201:
                    result = await response.json()
                    logger.info(f"Urna registered successfully: {result['data']['id']}")
                    return True
                else:
                    logger.error(f"Failed to register urna: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error registering urna: {e}")
            return False
    
    async def test_urna_status(self, urna_id: str) -> bool:
        """Testa status da urna"""
        logger.info(f"Testing urna status for {urna_id}...")
        
        try:
            async with self.session.get(
                f"{self.base_url}/api/v1/urnas/status/{urna_id}"
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info(f"Urna status: {result['data']['urna']['status']}")
                    return True
                else:
                    logger.error(f"Failed to get urna status: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error getting urna status: {e}")
            return False
    
    async def test_urna_health(self, urna_id: str) -> bool:
        """Testa saúde da urna"""
        logger.info(f"Testing urna health for {urna_id}...")
        
        try:
            async with self.session.get(
                f"{self.base_url}/api/v1/urnas/health/{urna_id}"
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    health = result['data']
                    logger.info(f"Urna health: CPU={health['performance_metrics']['cpu_usage']}%, "
                              f"Memory={health['performance_metrics']['memory_usage']}%")
                    return True
                else:
                    logger.error(f"Failed to get urna health: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error getting urna health: {e}")
            return False
    
    async def test_vote_authentication(self, urna_id: str) -> bool:
        """Testa autenticação de voto"""
        logger.info(f"Testing vote authentication for urna {urna_id}...")
        
        vote_data = {
            "urna_id": urna_id,
            "election_id": str(uuid.uuid4()),
            "candidate_id": str(uuid.uuid4()),
            "biometric_data": {
                "fingerprint": "dummy_fingerprint_data",
                "fingerprint_hash": "dummy_fingerprint_hash",
                "face_id": "dummy_face_data"
            },
            "certificate_data": {
                "certificate_hash": "dummy_cert_hash",
                "issuer": "ICP-Brasil",
                "valid_until": "2024-12-31T23:59:59Z",
                "serial_number": "123456789"
            },
            "vote_proof": "dummy_zk_proof"
        }
        
        try:
            async with self.session.post(
                f"{self.base_url}/api/v1/urnas/vote",
                json=vote_data
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info(f"Vote authenticated successfully: {result['data']['vote_id']}")
                    return True
                else:
                    logger.error(f"Failed to authenticate vote: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error authenticating vote: {e}")
            return False
    
    async def test_vote_sync(self, urna_id: str) -> bool:
        """Testa sincronização de votos"""
        logger.info(f"Testing vote sync for urna {urna_id}...")
        
        sync_data = {
            "urna_id": urna_id,
            "sync_type": "Incremental",
            "force_full_sync": False
        }
        
        try:
            async with self.session.post(
                f"{self.base_url}/api/v1/urnas/sync",
                json=sync_data
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info(f"Vote sync started: {result['data']['sync_id']}")
                    return True
                else:
                    logger.error(f"Failed to start vote sync: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error starting vote sync: {e}")
            return False
    
    async def test_audit_logs(self, urna_id: str) -> bool:
        """Testa logs de auditoria"""
        logger.info(f"Testing audit logs for urna {urna_id}...")
        
        try:
            async with self.session.get(
                f"{self.base_url}/api/v1/urnas/{urna_id}/audit"
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info(f"Audit logs retrieved: {result['data']['total']} logs")
                    return True
                else:
                    logger.error(f"Failed to get audit logs: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error getting audit logs: {e}")
            return False
    
    async def test_performance_metrics(self, urna_id: str) -> bool:
        """Testa métricas de performance"""
        logger.info(f"Testing performance metrics for urna {urna_id}...")
        
        try:
            async with self.session.get(
                f"{self.base_url}/api/v1/urnas/health/{urna_id}"
            ) as response:
                if response.status == 200:
                    result = await response.json()
                    metrics = result['data']['performance_metrics']
                    
                    # Verificar métricas críticas
                    cpu_usage = metrics['cpu_usage']
                    memory_usage = metrics['memory_usage']
                    disk_usage = metrics['disk_usage']
                    
                    if cpu_usage > 80.0:
                        logger.warning(f"High CPU usage: {cpu_usage}%")
                    
                    if memory_usage > 85.0:
                        logger.warning(f"High memory usage: {memory_usage}%")
                    
                    if disk_usage > 90.0:
                        logger.warning(f"High disk usage: {disk_usage}%")
                    
                    logger.info(f"Performance metrics: CPU={cpu_usage}%, "
                              f"Memory={memory_usage}%, Disk={disk_usage}%")
                    return True
                else:
                    logger.error(f"Failed to get performance metrics: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error getting performance metrics: {e}")
            return False
    
    async def test_security_features(self, urna_id: str) -> bool:
        """Testa recursos de segurança"""
        logger.info(f"Testing security features for urna {urna_id}...")
        
        # Teste 1: Verificar se urna está ativa
        status_result = await self.test_urna_status(urna_id)
        if not status_result:
            return False
        
        # Teste 2: Verificar logs de auditoria
        audit_result = await self.test_audit_logs(urna_id)
        if not audit_result:
            return False
        
        # Teste 3: Verificar métricas de performance
        performance_result = await self.test_performance_metrics(urna_id)
        if not performance_result:
            return False
        
        logger.info("Security features test passed")
        return True
    
    async def test_error_handling(self, urna_id: str) -> bool:
        """Testa tratamento de erros"""
        logger.info(f"Testing error handling for urna {urna_id}...")
        
        # Teste 1: Voto com dados inválidos
        invalid_vote_data = {
            "urna_id": "invalid_urna_id",
            "election_id": "invalid_election_id",
            "candidate_id": "invalid_candidate_id",
            "biometric_data": {
                "fingerprint": "",
                "fingerprint_hash": "",
                "face_id": ""
            },
            "vote_proof": ""
        }
        
        try:
            async with self.session.post(
                f"{self.base_url}/api/v1/urnas/vote",
                json=invalid_vote_data
            ) as response:
                if response.status == 400 or response.status == 422:
                    logger.info("Error handling test passed: Invalid data rejected")
                    return True
                else:
                    logger.error(f"Expected error response, got: {response.status}")
                    return False
        except Exception as e:
            logger.error(f"Error in error handling test: {e}")
            return False
    
    async def run_all_tests(self) -> Dict[str, Any]:
        """Executa todos os testes"""
        logger.info("Starting FORTIS urna integration tests...")
        
        test_results = {
            "start_time": datetime.now(timezone.utc).isoformat(),
            "tests": [],
            "summary": {
                "total": 0,
                "passed": 0,
                "failed": 0
            }
        }
        
        # Registrar urna de teste
        urna_id = str(uuid.uuid4())
        
        # Lista de testes
        tests = [
            ("urna_registration", self.test_urna_registration),
            ("urna_status", lambda: self.test_urna_status(urna_id)),
            ("urna_health", lambda: self.test_urna_health(urna_id)),
            ("vote_authentication", lambda: self.test_vote_authentication(urna_id)),
            ("vote_sync", lambda: self.test_vote_sync(urna_id)),
            ("audit_logs", lambda: self.test_audit_logs(urna_id)),
            ("performance_metrics", lambda: self.test_performance_metrics(urna_id)),
            ("security_features", lambda: self.test_security_features(urna_id)),
            ("error_handling", lambda: self.test_error_handling(urna_id))
        ]
        
        # Executar testes
        for test_name, test_func in tests:
            logger.info(f"Running test: {test_name}")
            start_time = time.time()
            
            try:
                result = await test_func()
                duration = time.time() - start_time
                
                test_result = {
                    "name": test_name,
                    "status": "PASSED" if result else "FAILED",
                    "duration": duration,
                    "timestamp": datetime.now(timezone.utc).isoformat()
                }
                
                test_results["tests"].append(test_result)
                test_results["summary"]["total"] += 1
                
                if result:
                    test_results["summary"]["passed"] += 1
                    logger.info(f"Test {test_name}: PASSED ({duration:.2f}s)")
                else:
                    test_results["summary"]["failed"] += 1
                    logger.error(f"Test {test_name}: FAILED ({duration:.2f}s)")
                    
            except Exception as e:
                duration = time.time() - start_time
                test_result = {
                    "name": test_name,
                    "status": "ERROR",
                    "duration": duration,
                    "error": str(e),
                    "timestamp": datetime.now(timezone.utc).isoformat()
                }
                
                test_results["tests"].append(test_result)
                test_results["summary"]["total"] += 1
                test_results["summary"]["failed"] += 1
                
                logger.error(f"Test {test_name}: ERROR ({duration:.2f}s) - {e}")
        
        test_results["end_time"] = datetime.now(timezone.utc).isoformat()
        test_results["total_duration"] = (
            datetime.fromisoformat(test_results["end_time"]) - 
            datetime.fromisoformat(test_results["start_time"])
        ).total_seconds()
        
        # Log resumo
        summary = test_results["summary"]
        logger.info(f"Test summary: {summary['passed']}/{summary['total']} passed, "
                   f"{summary['failed']} failed")
        
        return test_results

async def main():
    """Função principal"""
    async with FortisUrnaTester() as tester:
        results = await tester.run_all_tests()
        
        # Salvar resultados
        with open("urna_test_results.json", "w") as f:
            json.dump(results, f, indent=2)
        
        # Retornar código de saída baseado nos resultados
        if results["summary"]["failed"] == 0:
            logger.info("All tests passed!")
            return 0
        else:
            logger.error(f"{results['summary']['failed']} tests failed!")
            return 1

if __name__ == "__main__":
    exit_code = asyncio.run(main())
    exit(exit_code)
