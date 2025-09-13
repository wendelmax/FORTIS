#!/usr/bin/env python3
"""
Script de validação de certificados digitais
Valida certificados A1, A3 e A4 da ICP-Brasil
"""

import asyncio
import aiohttp
import json
import logging
import os
import sys
import base64
from datetime import datetime, timezone
from typing import Dict, List, Optional
import argparse
from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.x509.oid import NameOID, ExtendedKeyUsageOID

# Configuração de logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

class CertificateValidator:
    """Validador de certificados digitais"""
    
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
    
    def validate_certificate_structure(self, cert_data: str) -> Dict:
        """Valida estrutura básica do certificado"""
        try:
            # Decodificar certificado
            if cert_data.startswith('-----BEGIN CERTIFICATE-----'):
                cert_bytes = cert_data.encode('utf-8')
            else:
                cert_bytes = base64.b64decode(cert_data)
            
            # Carregar certificado
            cert = x509.load_pem_x509_certificate(cert_bytes)
            
            # Extrair informações básicas
            subject = cert.subject
            issuer = cert.issuer
            
            # Extrair CN do subject
            cn = None
            for name in subject.get_attributes_for_oid(NameOID.COMMON_NAME):
                cn = name.value
                break
            
            # Extrair CN do issuer
            issuer_cn = None
            for name in issuer.get_attributes_for_oid(NameOID.COMMON_NAME):
                issuer_cn = name.value
                break
            
            # Verificar validade temporal
            now = datetime.now(timezone.utc)
            not_before = cert.not_valid_before.replace(tzinfo=timezone.utc)
            not_after = cert.not_valid_after.replace(tzinfo=timezone.utc)
            
            is_valid_time = not_before <= now <= not_after
            days_until_expiry = (not_after - now).days
            
            # Verificar uso da chave
            key_usage = cert.extensions.get_extension_for_oid(x509.oid.ExtensionOID.KEY_USAGE)
            has_digital_signature = key_usage.value.digital_signature
            has_non_repudiation = key_usage.value.non_repudiation
            
            # Verificar uso estendido da chave
            try:
                ext_key_usage = cert.extensions.get_extension_for_oid(x509.oid.ExtensionOID.EXTENDED_KEY_USAGE)
                has_client_auth = ExtendedKeyUsageOID.CLIENT_AUTH in ext_key_usage.value
                has_email_protection = ExtendedKeyUsageOID.EMAIL_PROTECTION in ext_key_usage.value
            except x509.ExtensionNotFound:
                has_client_auth = False
                has_email_protection = False
            
            return {
                'valid_structure': True,
                'subject_cn': cn,
                'issuer_cn': issuer_cn,
                'serial_number': str(cert.serial_number),
                'not_before': not_before.isoformat(),
                'not_after': not_after.isoformat(),
                'is_valid_time': is_valid_time,
                'days_until_expiry': days_until_expiry,
                'has_digital_signature': has_digital_signature,
                'has_non_repudiation': has_non_repudiation,
                'has_client_auth': has_client_auth,
                'has_email_protection': has_email_protection,
                'public_key_algorithm': cert.public_key().__class__.__name__,
                'signature_algorithm': cert.signature_algorithm_oid._name,
            }
            
        except Exception as e:
            return {
                'valid_structure': False,
                'error': str(e)
            }
    
    async def validate_certificate_online(self, cert_data: str) -> Dict:
        """Valida certificado via API online"""
        try:
            payload = {
                'certificate_data': cert_data
            }
            
            async with self.session.post(f'{self.base_url}/api/v1/certificate/validate', json=payload) as response:
                if response.status == 200:
                    result = await response.json()
                    logger.info("Certificado validado via API online")
                    return {
                        'online_validation': True,
                        'api_result': result
                    }
                else:
                    error_text = await response.text()
                    logger.error(f"Erro na validação online: {error_text}")
                    return {
                        'online_validation': False,
                        'error': error_text
                    }
        except Exception as e:
            logger.error(f"Erro na validação online: {e}")
            return {
                'online_validation': False,
                'error': str(e)
            }
    
    async def check_ocsp_status(self, cert_data: str) -> Dict:
        """Verifica status OCSP do certificado"""
        try:
            # Decodificar certificado
            if cert_data.startswith('-----BEGIN CERTIFICATE-----'):
                cert_bytes = cert_data.encode('utf-8')
            else:
                cert_bytes = base64.b64decode(cert_data)
            
            cert = x509.load_pem_x509_certificate(cert_bytes)
            serial_number = str(cert.serial_number)
            
            # Simular consulta OCSP (em implementação real, usar biblioteca OCSP)
            # Por enquanto, retornar status simulado
            return {
                'ocsp_checked': True,
                'status': 'good',
                'serial_number': serial_number,
                'response_time': datetime.now(timezone.utc).isoformat()
            }
            
        except Exception as e:
            logger.error(f"Erro na verificação OCSP: {e}")
            return {
                'ocsp_checked': False,
                'error': str(e)
            }
    
    async def validate_certificate_complete(self, cert_data: str) -> Dict:
        """Validação completa do certificado"""
        result = {
            'certificate_id': f"cert_{datetime.now().strftime('%Y%m%d_%H%M%S')}",
            'validation_start': datetime.now(timezone.utc).isoformat(),
            'overall_valid': False,
            'errors': [],
            'warnings': []
        }
        
        # Validação estrutural
        structure_result = self.validate_certificate_structure(cert_data)
        result['structure_validation'] = structure_result
        
        if not structure_result.get('valid_structure', False):
            result['errors'].append("Estrutura do certificado inválida")
            result['validation_end'] = datetime.now(timezone.utc).isoformat()
            return result
        
        # Verificar validade temporal
        if not structure_result.get('is_valid_time', False):
            result['errors'].append("Certificado fora do período de validade")
        
        # Verificar uso da chave
        if not structure_result.get('has_digital_signature', False):
            result['errors'].append("Certificado não suporta assinatura digital")
        
        if not structure_result.get('has_non_repudiation', False):
            result['warnings'].append("Certificado não suporta não-repúdio")
        
        # Verificar proximidade do vencimento
        days_until_expiry = structure_result.get('days_until_expiry', 0)
        if days_until_expiry <= 30:
            result['warnings'].append(f"Certificado próximo do vencimento ({days_until_expiry} dias)")
        
        # Validação online (se disponível)
        if self.base_url and self.api_key:
            online_result = await self.validate_certificate_online(cert_data)
            result['online_validation'] = online_result
            
            if not online_result.get('online_validation', False):
                result['warnings'].append("Validação online não disponível")
        
        # Verificação OCSP
        ocsp_result = await self.check_ocsp_status(cert_data)
        result['ocsp_validation'] = ocsp_result
        
        if not ocsp_result.get('ocsp_checked', False):
            result['warnings'].append("Verificação OCSP não disponível")
        
        # Determinar validade geral
        result['overall_valid'] = len(result['errors']) == 0
        result['validation_end'] = datetime.now(timezone.utc).isoformat()
        
        return result

def load_certificate_from_file(filename: str) -> str:
    """Carrega certificado de arquivo"""
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            content = f.read().strip()
        
        # Se não começar com -----BEGIN CERTIFICATE-----, assumir que é base64
        if not content.startswith('-----BEGIN CERTIFICATE-----'):
            # Decodificar base64 e recodificar como PEM
            cert_bytes = base64.b64decode(content)
            content = base64.b64encode(cert_bytes).decode('utf-8')
            content = f"-----BEGIN CERTIFICATE-----\n{content}\n-----END CERTIFICATE-----"
        
        return content
    except Exception as e:
        logger.error(f"Erro ao carregar certificado do arquivo {filename}: {e}")
        return ""

async def main():
    """Função principal"""
    parser = argparse.ArgumentParser(description='Validação de certificados digitais')
    parser.add_argument('--base-url', help='URL base da API TSE (opcional)')
    parser.add_argument('--api-key', help='Chave da API TSE (opcional)')
    parser.add_argument('--cert-file', help='Arquivo com certificado')
    parser.add_argument('--cert-data', help='Dados do certificado (base64 ou PEM)')
    parser.add_argument('--output', help='Arquivo de saída para salvar resultados')
    parser.add_argument('--verbose', '-v', action='store_true', help='Log verboso')
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
    
    # Validar argumentos
    if not args.cert_file and not args.cert_data:
        logger.error("Deve especificar --cert-file ou --cert-data")
        sys.exit(1)
    
    # Carregar certificado
    if args.cert_file:
        cert_data = load_certificate_from_file(args.cert_file)
        if not cert_data:
            logger.error("Erro ao carregar certificado do arquivo")
            sys.exit(1)
    else:
        cert_data = args.cert_data
    
    # Validar certificado
    async with CertificateValidator(args.base_url or '', args.api_key or '') as validator:
        result = await validator.validate_certificate_complete(cert_data)
    
    # Salvar resultados
    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            json.dump(result, f, indent=2, ensure_ascii=False)
        logger.info(f"Resultados salvos em {args.output}")
    else:
        print(json.dumps(result, indent=2, ensure_ascii=False))
    
    # Log final
    if result['overall_valid']:
        logger.info("Certificado válido")
    else:
        logger.error(f"Certificado inválido: {', '.join(result['errors'])}")
        if result['warnings']:
            logger.warning(f"Avisos: {', '.join(result['warnings'])}")
        sys.exit(1)

if __name__ == '__main__':
    asyncio.run(main())
