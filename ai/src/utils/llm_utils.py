#!/usr/bin/env python3
"""
FORTIS - Utilitários para LLM
Funções auxiliares para integração com modelos de linguagem
"""

import json
import re
import hashlib
from typing import Dict, List, Optional, Any, Union
import logging
from datetime import datetime
import asyncio
import aiohttp
from pathlib import Path

class LLMUtils:
    """Utilitários para integração com modelos LLM"""
    
    def __init__(self):
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Templates de prompts
        self.prompt_templates = {
            'sentiment_analysis': {
                'system': "Você é um analista especializado em eleições brasileiras. Analise o sentimento do texto e responda em JSON.",
                'user': "Analise o sentimento do seguinte texto sobre eleições:\n\n{text}\n\nResponda com: {{'sentiment': 'POSITIVO/NEGATIVO/NEUTRO', 'confidence': 0.0-1.0, 'reasoning': 'explicação'}}"
            },
            'entity_extraction': {
                'system': "Você é um especialista em extração de entidades eleitorais. Extraia informações relevantes do texto.",
                'user': "Extraia entidades eleitorais do seguinte texto:\n\n{text}\n\nResponda com: {{'cpfs': [], 'candidatos': [], 'partidos': [], 'secoes': [], 'zonas': [], 'datas': []}}"
            },
            'issue_classification': {
                'system': "Você é um especialista em sistemas eleitorais. Classifique problemas mencionados no texto.",
                'user': "Classifique os problemas eleitorais no texto:\n\n{text}\n\nResponda com: {{'categories': ['TÉCNICO', 'LOGÍSTICO', 'SEGURANÇA', 'JURÍDICO', 'SOCIAL'], 'severity': 'BAIXA/MÉDIA/ALTA', 'description': 'resumo'}}"
            },
            'report_generation': {
                'system': "Você é um analista eleitoral. Gere relatórios profissionais baseados em dados.",
                'user': "Gere um relatório de eleição baseado nos dados:\n\n{data}\n\nInclua: resumo executivo, análise de participação, padrões identificados, recomendações e conclusões."
            },
            'insight_extraction': {
                'system': "Você é um especialista em análise eleitoral. Extraia insights relevantes do texto.",
                'user': "Extraia insights sobre eleições do texto:\n\n{text}\n\nIdentifique: temas principais, preocupações dos eleitores, sugestões de melhoria e padrões de comportamento."
            }
        }
    
    def create_prompt(self, template_name: str, **kwargs) -> Dict[str, str]:
        """Cria prompt usando template"""
        try:
            if template_name not in self.prompt_templates:
                raise ValueError(f"Template '{template_name}' não encontrado")
            
            template = self.prompt_templates[template_name]
            
            # Substitui variáveis no template
            system_prompt = template['system'].format(**kwargs)
            user_prompt = template['user'].format(**kwargs)
            
            return {
                'system': system_prompt,
                'user': user_prompt
            }
            
        except Exception as e:
            self.logger.error(f"Erro ao criar prompt: {e}")
            return {
                'system': "Você é um assistente especializado em eleições.",
                'user': kwargs.get('text', '')
            }
    
    def extract_json_from_response(self, response: str) -> Optional[Dict[str, Any]]:
        """Extrai JSON da resposta do LLM"""
        try:
            # Procura por JSON na resposta
            json_patterns = [
                r'\{[^{}]*\}',  # JSON simples
                r'\{.*?\}',     # JSON com possíveis quebras de linha
                r'```json\s*(\{.*?\})\s*```',  # JSON em bloco de código
                r'```\s*(\{.*?\})\s*```'       # JSON em bloco genérico
            ]
            
            for pattern in json_patterns:
                matches = re.findall(pattern, response, re.DOTALL)
                for match in matches:
                    try:
                        return json.loads(match)
                    except json.JSONDecodeError:
                        continue
            
            # Se não encontrou JSON, tenta extrair campos individuais
            return self._extract_fields_from_text(response)
            
        except Exception as e:
            self.logger.error(f"Erro ao extrair JSON: {e}")
            return None
    
    def _extract_fields_from_text(self, text: str) -> Dict[str, Any]:
        """Extrai campos individuais do texto quando JSON não é encontrado"""
        try:
            fields = {}
            
            # Padrões para extrair campos comuns
            patterns = {
                'sentiment': r'sentiment[:\s]+(POSITIVO|NEGATIVO|NEUTRO)',
                'confidence': r'confidence[:\s]+([0-9.]+)',
                'severity': r'severity[:\s]+(BAIXA|MÉDIA|ALTA)',
                'categories': r'categor(?:y|ies)[:\s]+([A-Za-z\s,]+)',
                'description': r'description[:\s]+([^.\n]+)',
                'reasoning': r'reasoning[:\s]+([^.\n]+)'
            }
            
            for field, pattern in patterns.items():
                match = re.search(pattern, text, re.IGNORECASE)
                if match:
                    value = match.group(1).strip()
                    
                    # Processa valor baseado no tipo
                    if field == 'confidence':
                        try:
                            fields[field] = float(value)
                        except ValueError:
                            fields[field] = 0.5
                    elif field == 'categories':
                        fields[field] = [cat.strip() for cat in value.split(',')]
                    else:
                        fields[field] = value
            
            return fields
            
        except Exception as e:
            self.logger.error(f"Erro ao extrair campos: {e}")
            return {}
    
    def validate_llm_response(self, response: Dict[str, Any], expected_fields: List[str]) -> Dict[str, Any]:
        """Valida resposta do LLM"""
        try:
            validation_result = {
                'is_valid': True,
                'missing_fields': [],
                'invalid_fields': [],
                'warnings': []
            }
            
            # Verifica campos obrigatórios
            for field in expected_fields:
                if field not in response:
                    validation_result['missing_fields'].append(field)
                    validation_result['is_valid'] = False
                elif response[field] is None or response[field] == '':
                    validation_result['invalid_fields'].append(field)
                    validation_result['is_valid'] = False
            
            # Validações específicas
            if 'sentiment' in response:
                valid_sentiments = ['POSITIVO', 'NEGATIVO', 'NEUTRO']
                if response['sentiment'] not in valid_sentiments:
                    validation_result['invalid_fields'].append('sentiment')
                    validation_result['is_valid'] = False
            
            if 'confidence' in response:
                try:
                    conf = float(response['confidence'])
                    if not 0 <= conf <= 1:
                        validation_result['warnings'].append('confidence fora do range 0-1')
                except (ValueError, TypeError):
                    validation_result['invalid_fields'].append('confidence')
                    validation_result['is_valid'] = False
            
            if 'severity' in response:
                valid_severities = ['BAIXA', 'MÉDIA', 'ALTA']
                if response['severity'] not in valid_severities:
                    validation_result['invalid_fields'].append('severity')
                    validation_result['is_valid'] = False
            
            return validation_result
            
        except Exception as e:
            self.logger.error(f"Erro na validação: {e}")
            return {
                'is_valid': False,
                'error': str(e)
            }
    
    def create_conversation_context(self, messages: List[Dict[str, str]]) -> str:
        """Cria contexto de conversação"""
        try:
            context_parts = []
            
            for i, message in enumerate(messages):
                role = message.get('role', 'user')
                content = message.get('content', '')
                
                if role == 'system':
                    context_parts.append(f"Sistema: {content}")
                elif role == 'user':
                    context_parts.append(f"Usuário: {content}")
                elif role == 'assistant':
                    context_parts.append(f"Assistente: {content}")
                
                # Adiciona separador entre mensagens
                if i < len(messages) - 1:
                    context_parts.append("---")
            
            return "\n".join(context_parts)
            
        except Exception as e:
            self.logger.error(f"Erro ao criar contexto: {e}")
            return ""
    
    def calculate_response_quality(self, response: Dict[str, Any]) -> Dict[str, Any]:
        """Calcula qualidade da resposta do LLM"""
        try:
            quality_metrics = {
                'completeness': 0.0,
                'accuracy': 0.0,
                'relevance': 0.0,
                'overall_score': 0.0
            }
            
            # Completude: verifica se campos importantes estão presentes
            important_fields = ['sentiment', 'confidence', 'reasoning']
            present_fields = sum(1 for field in important_fields if field in response and response[field])
            quality_metrics['completeness'] = present_fields / len(important_fields)
            
            # Precisão: verifica se valores estão em ranges válidos
            accuracy_score = 0.0
            accuracy_checks = 0
            
            if 'confidence' in response:
                try:
                    conf = float(response['confidence'])
                    if 0 <= conf <= 1:
                        accuracy_score += 1
                    accuracy_checks += 1
                except (ValueError, TypeError):
                    accuracy_checks += 1
            
            if 'sentiment' in response:
                valid_sentiments = ['POSITIVO', 'NEGATIVO', 'NEUTRO']
                if response['sentiment'] in valid_sentiments:
                    accuracy_score += 1
                accuracy_checks += 1
            
            if accuracy_checks > 0:
                quality_metrics['accuracy'] = accuracy_score / accuracy_checks
            
            # Relevância: verifica se há conteúdo substantivo
            text_content = str(response).lower()
            relevant_keywords = ['eleição', 'voto', 'candidato', 'partido', 'sistema', 'democracia']
            keyword_count = sum(1 for keyword in relevant_keywords if keyword in text_content)
            quality_metrics['relevance'] = min(1.0, keyword_count / 3)
            
            # Score geral
            quality_metrics['overall_score'] = (
                quality_metrics['completeness'] * 0.4 +
                quality_metrics['accuracy'] * 0.4 +
                quality_metrics['relevance'] * 0.2
            )
            
            return quality_metrics
            
        except Exception as e:
            self.logger.error(f"Erro no cálculo de qualidade: {e}")
            return {
                'completeness': 0.0,
                'accuracy': 0.0,
                'relevance': 0.0,
                'overall_score': 0.0,
                'error': str(e)
            }
    
    def create_response_hash(self, response: Dict[str, Any]) -> str:
        """Cria hash da resposta para cache"""
        try:
            # Remove campos que não afetam o conteúdo
            cache_response = response.copy()
            cache_response.pop('timestamp', None)
            cache_response.pop('generation_time', None)
            
            # Converte para string JSON ordenada
            json_str = json.dumps(cache_response, sort_keys=True, default=str)
            
            # Gera hash SHA-256
            return hashlib.sha256(json_str.encode()).hexdigest()
            
        except Exception as e:
            self.logger.error(f"Erro ao criar hash: {e}")
            return ""
    
    def format_llm_response(self, response: Dict[str, Any], format_type: str = 'json') -> str:
        """Formata resposta do LLM"""
        try:
            if format_type == 'json':
                return json.dumps(response, indent=2, ensure_ascii=False, default=str)
            elif format_type == 'markdown':
                return self._format_as_markdown(response)
            elif format_type == 'text':
                return self._format_as_text(response)
            else:
                return str(response)
                
        except Exception as e:
            self.logger.error(f"Erro na formatação: {e}")
            return str(response)
    
    def _format_as_markdown(self, response: Dict[str, Any]) -> str:
        """Formata resposta como Markdown"""
        try:
            lines = ["# Análise de Texto Eleitoral\n"]
            
            if 'sentiment' in response:
                lines.append(f"## Sentimento: {response['sentiment']}")
                if 'confidence' in response:
                    lines.append(f"**Confiança:** {response['confidence']}")
                if 'reasoning' in response:
                    lines.append(f"**Justificativa:** {response['reasoning']}")
            
            if 'categories' in response:
                lines.append(f"## Categorias: {', '.join(response['categories'])}")
            
            if 'severity' in response:
                lines.append(f"## Severidade: {response['severity']}")
            
            if 'description' in response:
                lines.append(f"## Descrição: {response['description']}")
            
            return "\n".join(lines)
            
        except Exception as e:
            self.logger.error(f"Erro na formatação Markdown: {e}")
            return str(response)
    
    def _format_as_text(self, response: Dict[str, Any]) -> str:
        """Formata resposta como texto simples"""
        try:
            lines = []
            
            for key, value in response.items():
                if isinstance(value, list):
                    lines.append(f"{key.title()}: {', '.join(map(str, value))}")
                else:
                    lines.append(f"{key.title()}: {value}")
            
            return "\n".join(lines)
            
        except Exception as e:
            self.logger.error(f"Erro na formatação de texto: {e}")
            return str(response)
    
    def create_llm_config(self, model_name: str, **kwargs) -> Dict[str, Any]:
        """Cria configuração para LLM"""
        default_config = {
            'model': model_name,
            'temperature': 0.7,
            'max_tokens': 2048,
            'top_p': 0.9,
            'frequency_penalty': 0.0,
            'presence_penalty': 0.0,
            'stop': None,
            'stream': False
        }
        
        # Atualiza com parâmetros fornecidos
        default_config.update(kwargs)
        
        return default_config
    
    def test_llm_connection(self, base_url: str, timeout: int = 5) -> Dict[str, Any]:
        """Testa conexão com serviço LLM"""
        async def _test_connection():
            try:
                async with aiohttp.ClientSession() as session:
                    async with session.get(f"{base_url}/api/tags", timeout=timeout) as response:
                        if response.status == 200:
                            data = await response.json()
                            return {
                                'success': True,
                                'status': response.status,
                                'models_available': len(data.get('models', [])),
                                'response_time': response.headers.get('X-Response-Time', 'N/A')
                            }
                        else:
                            return {
                                'success': False,
                                'status': response.status,
                                'error': f"HTTP {response.status}"
                            }
            except Exception as e:
                return {
                    'success': False,
                    'error': str(e)
                }
        
        try:
            return asyncio.run(_test_connection())
        except Exception as e:
            return {
                'success': False,
                'error': str(e)
            }

def main():
    """Função principal para teste"""
    utils = LLMUtils()
    
    print("Utilitários de LLM FORTIS")
    
    # Testa criação de prompt
    prompt = utils.create_prompt('sentiment_analysis', text="O sistema eleitoral está funcionando bem")
    print(f"Prompt criado: {prompt['user'][:100]}...")
    
    # Testa extração de JSON
    response_text = '{"sentiment": "POSITIVO", "confidence": 0.8, "reasoning": "Texto positivo"}'
    json_data = utils.extract_json_from_response(response_text)
    print(f"JSON extraído: {json_data}")
    
    # Testa validação
    validation = utils.validate_llm_response(json_data, ['sentiment', 'confidence'])
    print(f"Validação: {validation}")
    
    # Testa qualidade
    quality = utils.calculate_response_quality(json_data)
    print(f"Qualidade: {quality}")

if __name__ == "__main__":
    main()
