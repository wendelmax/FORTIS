#!/usr/bin/env python3
"""
FORTIS - Serviço de LLM Local
Integração com modelos LLM locais (Ollama, LM Studio, etc.) para análise de texto
"""

import asyncio
import aiohttp
import json
import logging
from typing import Dict, List, Optional, Any, Union
from datetime import datetime
import os
from pathlib import Path

class LLMService:
    """Serviço de integração com modelos LLM locais para o FORTIS"""
    
    def __init__(self, config: Optional[Dict] = None):
        self.config = config or {
            'ollama_url': 'http://localhost:11434',
            'lm_studio_url': 'http://localhost:1234',
            'default_model': 'llama3.2:3b',
            'timeout': 30,
            'max_tokens': 2048,
            'temperature': 0.7
        }
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Estatísticas de uso
        self.usage_stats = {
            'total_requests': 0,
            'successful_requests': 0,
            'failed_requests': 0,
            'total_tokens': 0,
            'models_used': set()
        }
        
        # Cache de respostas
        self.response_cache = {}
    
    async def initialize(self):
        """Inicializa o serviço de LLM"""
        try:
            self.logger.info("Inicializando serviço de LLM...")
            
            # Testa conectividade com Ollama
            await self._test_ollama_connection()
            
            # Lista modelos disponíveis
            available_models = await self.list_available_models()
            self.logger.info(f"Modelos disponíveis: {available_models}")
            
            self.logger.info("Serviço de LLM inicializado com sucesso")
            
        except Exception as e:
            self.logger.error(f"Erro ao inicializar serviço de LLM: {e}")
            raise
    
    async def _test_ollama_connection(self):
        """Testa conexão com Ollama"""
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(f"{self.config['ollama_url']}/api/tags", timeout=5) as response:
                    if response.status == 200:
                        self.logger.info("Conexão com Ollama estabelecida")
                    else:
                        raise Exception(f"Erro na conexão com Ollama: {response.status}")
        except Exception as e:
            self.logger.warning(f"Não foi possível conectar com Ollama: {e}")
    
    async def list_available_models(self) -> List[str]:
        """Lista modelos disponíveis"""
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(f"{self.config['ollama_url']}/api/tags") as response:
                    if response.status == 200:
                        data = await response.json()
                        models = [model['name'] for model in data.get('models', [])]
                        return models
                    else:
                        return []
        except Exception as e:
            self.logger.error(f"Erro ao listar modelos: {e}")
            return []
    
    async def generate_text(self, 
                          prompt: str, 
                          model: Optional[str] = None,
                          system_prompt: Optional[str] = None,
                          temperature: Optional[float] = None,
                          max_tokens: Optional[int] = None) -> Dict[str, Any]:
        """Gera texto usando modelo LLM local"""
        try:
            model = model or self.config['default_model']
            temperature = temperature or self.config['temperature']
            max_tokens = max_tokens or self.config['max_tokens']
            
            # Verifica cache
            cache_key = f"{model}:{hash(prompt)}:{temperature}:{max_tokens}"
            if cache_key in self.response_cache:
                self.logger.info("Resposta obtida do cache")
                return self.response_cache[cache_key]
            
            # Prepara payload
            payload = {
                "model": model,
                "prompt": prompt,
                "stream": False,
                "options": {
                    "temperature": temperature,
                    "num_predict": max_tokens
                }
            }
            
            if system_prompt:
                payload["system"] = system_prompt
            
            # Faz requisição
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    f"{self.config['ollama_url']}/api/generate",
                    json=payload,
                    timeout=self.config['timeout']
                ) as response:
                    
                    if response.status == 200:
                        data = await response.json()
                        
                        result = {
                            'success': True,
                            'text': data.get('response', ''),
                            'model': model,
                            'tokens_used': data.get('eval_count', 0),
                            'prompt_tokens': data.get('prompt_eval_count', 0),
                            'generation_time': data.get('total_duration', 0) / 1e9,  # Converte de ns para s
                            'timestamp': datetime.now().isoformat()
                        }
                        
                        # Atualiza estatísticas
                        self.usage_stats['total_requests'] += 1
                        self.usage_stats['successful_requests'] += 1
                        self.usage_stats['total_tokens'] += result['tokens_used']
                        self.usage_stats['models_used'].add(model)
                        
                        # Salva no cache
                        self.response_cache[cache_key] = result
                        
                        return result
                    else:
                        error_text = await response.text()
                        raise Exception(f"Erro na API: {response.status} - {error_text}")
                        
        except Exception as e:
            self.logger.error(f"Erro na geração de texto: {e}")
            self.usage_stats['total_requests'] += 1
            self.usage_stats['failed_requests'] += 1
            
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    async def analyze_election_sentiment(self, text: str, model: Optional[str] = None) -> Dict[str, Any]:
        """Analisa sentimento de texto relacionado a eleições"""
        try:
            system_prompt = """Você é um analista especializado em eleições brasileiras. 
            Analise o sentimento do texto fornecido e classifique como:
            - POSITIVO: Texto favorável ao processo eleitoral
            - NEGATIVO: Texto crítico ou desfavorável
            - NEUTRO: Texto neutro ou informativo
            
            Responda apenas com o JSON: {"sentiment": "POSITIVO/NEGATIVO/NEUTRO", "confidence": 0.0-1.0, "reasoning": "explicação"}"""
            
            prompt = f"Analise o sentimento do seguinte texto sobre eleições:\n\n{text}"
            
            result = await self.generate_text(
                prompt=prompt,
                model=model,
                system_prompt=system_prompt,
                temperature=0.3
            )
            
            if result['success']:
                try:
                    # Tenta extrair JSON da resposta
                    response_text = result['text']
                    json_start = response_text.find('{')
                    json_end = response_text.rfind('}') + 1
                    
                    if json_start != -1 and json_end > json_start:
                        json_str = response_text[json_start:json_end]
                        sentiment_data = json.loads(json_str)
                        
                        return {
                            'success': True,
                            'sentiment': sentiment_data.get('sentiment', 'NEUTRO'),
                            'confidence': sentiment_data.get('confidence', 0.5),
                            'reasoning': sentiment_data.get('reasoning', ''),
                            'original_text': text,
                            'timestamp': datetime.now().isoformat()
                        }
                    else:
                        # Fallback: análise simples por palavras-chave
                        return self._simple_sentiment_analysis(text)
                except json.JSONDecodeError:
                    return self._simple_sentiment_analysis(text)
            else:
                return result
                
        except Exception as e:
            self.logger.error(f"Erro na análise de sentimento: {e}")
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    def _simple_sentiment_analysis(self, text: str) -> Dict[str, Any]:
        """Análise de sentimento simples baseada em palavras-chave"""
        positive_words = ['bom', 'ótimo', 'excelente', 'confiança', 'transparência', 'democracia', 'justo', 'seguro']
        negative_words = ['ruim', 'terrível', 'fraude', 'corrupção', 'injusto', 'inseguro', 'manipulação', 'problema']
        
        text_lower = text.lower()
        positive_count = sum(1 for word in positive_words if word in text_lower)
        negative_count = sum(1 for word in negative_words if word in text_lower)
        
        if positive_count > negative_count:
            sentiment = 'POSITIVO'
            confidence = min(0.9, 0.5 + (positive_count - negative_count) * 0.1)
        elif negative_count > positive_count:
            sentiment = 'NEGATIVO'
            confidence = min(0.9, 0.5 + (negative_count - positive_count) * 0.1)
        else:
            sentiment = 'NEUTRO'
            confidence = 0.5
        
        return {
            'success': True,
            'sentiment': sentiment,
            'confidence': confidence,
            'reasoning': f'Análise baseada em palavras-chave: {positive_count} positivas, {negative_count} negativas',
            'original_text': text,
            'timestamp': datetime.now().isoformat()
        }
    
    async def extract_election_insights(self, text: str, model: Optional[str] = None) -> Dict[str, Any]:
        """Extrai insights sobre eleições de texto"""
        try:
            system_prompt = """Você é um especialista em análise eleitoral. 
            Extraia insights relevantes do texto fornecido sobre eleições.
            Identifique:
            - Temas principais mencionados
            - Preocupações dos eleitores
            - Sugestões de melhoria
            - Padrões de comportamento
            
            Responda em formato JSON estruturado."""
            
            prompt = f"Analise o seguinte texto sobre eleições e extraia insights:\n\n{text}"
            
            result = await self.generate_text(
                prompt=prompt,
                model=model,
                system_prompt=system_prompt,
                temperature=0.4
            )
            
            if result['success']:
                return {
                    'success': True,
                    'insights': result['text'],
                    'original_text': text,
                    'timestamp': datetime.now().isoformat()
                }
            else:
                return result
                
        except Exception as e:
            self.logger.error(f"Erro na extração de insights: {e}")
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    async def generate_election_report(self, data: Dict[str, Any], model: Optional[str] = None) -> Dict[str, Any]:
        """Gera relatório de eleição usando LLM"""
        try:
            system_prompt = """Você é um analista eleitoral especializado. 
            Gere um relatório profissional baseado nos dados fornecidos.
            Inclua:
            - Resumo executivo
            - Análise de participação
            - Padrões identificados
            - Recomendações
            - Conclusões
            
            Use linguagem técnica mas acessível."""
            
            prompt = f"Gere um relatório de eleição baseado nos seguintes dados:\n\n{json.dumps(data, indent=2)}"
            
            result = await self.generate_text(
                prompt=prompt,
                model=model,
                system_prompt=system_prompt,
                temperature=0.5
            )
            
            if result['success']:
                return {
                    'success': True,
                    'report': result['text'],
                    'data_source': data,
                    'timestamp': datetime.now().isoformat()
                }
            else:
                return result
                
        except Exception as e:
            self.logger.error(f"Erro na geração de relatório: {e}")
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    async def classify_election_issues(self, text: str, model: Optional[str] = None) -> Dict[str, Any]:
        """Classifica problemas eleitorais mencionados no texto"""
        try:
            system_prompt = """Você é um especialista em sistemas eleitorais. 
            Classifique os problemas mencionados no texto em categorias:
            - TÉCNICO: Problemas com tecnologia, software, hardware
            - LOGÍSTICO: Problemas de organização, transporte, localização
            - SEGURANÇA: Problemas de segurança, fraude, integridade
            - JURÍDICO: Problemas legais, regulamentação, conformidade
            - SOCIAL: Problemas de acesso, inclusão, participação
            - OUTROS: Problemas não categorizados
            
            Responda com JSON: {"categories": ["categoria1", "categoria2"], "severity": "BAIXA/MÉDIA/ALTA", "description": "resumo"}"""
            
            prompt = f"Classifique os problemas eleitorais mencionados no texto:\n\n{text}"
            
            result = await self.generate_text(
                prompt=prompt,
                model=model,
                system_prompt=system_prompt,
                temperature=0.3
            )
            
            if result['success']:
                try:
                    # Tenta extrair JSON da resposta
                    response_text = result['text']
                    json_start = response_text.find('{')
                    json_end = response_text.rfind('}') + 1
                    
                    if json_start != -1 and json_end > json_start:
                        json_str = response_text[json_start:json_end]
                        classification_data = json.loads(json_str)
                        
                        return {
                            'success': True,
                            'categories': classification_data.get('categories', []),
                            'severity': classification_data.get('severity', 'BAIXA'),
                            'description': classification_data.get('description', ''),
                            'original_text': text,
                            'timestamp': datetime.now().isoformat()
                        }
                    else:
                        return self._simple_issue_classification(text)
                except json.JSONDecodeError:
                    return self._simple_issue_classification(text)
            else:
                return result
                
        except Exception as e:
            self.logger.error(f"Erro na classificação de problemas: {e}")
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    def _simple_issue_classification(self, text: str) -> Dict[str, Any]:
        """Classificação simples de problemas baseada em palavras-chave"""
        categories = {
            'TÉCNICO': ['software', 'hardware', 'sistema', 'bug', 'erro', 'falha técnica'],
            'LOGÍSTICO': ['transporte', 'local', 'organização', 'logística', 'distribuição'],
            'SEGURANÇA': ['fraude', 'segurança', 'hack', 'ataque', 'vulnerabilidade'],
            'JURÍDICO': ['legal', 'lei', 'regulamento', 'conformidade', 'jurídico'],
            'SOCIAL': ['acesso', 'inclusão', 'participação', 'barreira', 'dificuldade']
        }
        
        text_lower = text.lower()
        found_categories = []
        
        for category, keywords in categories.items():
            if any(keyword in text_lower for keyword in keywords):
                found_categories.append(category)
        
        if not found_categories:
            found_categories = ['OUTROS']
        
        return {
            'success': True,
            'categories': found_categories,
            'severity': 'MÉDIA',
            'description': 'Classificação baseada em palavras-chave',
            'original_text': text,
            'timestamp': datetime.now().isoformat()
        }
    
    def get_usage_statistics(self) -> Dict[str, Any]:
        """Retorna estatísticas de uso do serviço"""
        return {
            'total_requests': self.usage_stats['total_requests'],
            'successful_requests': self.usage_stats['successful_requests'],
            'failed_requests': self.usage_stats['failed_requests'],
            'success_rate': (
                self.usage_stats['successful_requests'] / self.usage_stats['total_requests'] 
                if self.usage_stats['total_requests'] > 0 else 0
            ),
            'total_tokens': self.usage_stats['total_tokens'],
            'models_used': list(self.usage_stats['models_used']),
            'cache_size': len(self.response_cache),
            'timestamp': datetime.now().isoformat()
        }
    
    def clear_cache(self):
        """Limpa o cache de respostas"""
        self.response_cache.clear()
        self.logger.info("Cache de respostas limpo")

# Instância global do serviço
llm_service = LLMService()

async def get_llm_service() -> LLMService:
    """Retorna instância do serviço de LLM"""
    if not hasattr(llm_service, '_initialized'):
        await llm_service.initialize()
        llm_service._initialized = True
    return llm_service

def main():
    """Função principal para teste"""
    async def test_service():
        service = await get_llm_service()
        
        print("Serviço de LLM FORTIS")
        print(f"Configuração: {service.config}")
        
        # Testa geração de texto
        result = await service.generate_text("Explique o que é um sistema eleitoral digital seguro.")
        print(f"Geração de texto: {result['success']}")
        
        # Testa análise de sentimento
        sentiment = await service.analyze_election_sentiment("O sistema eleitoral está funcionando muito bem!")
        print(f"Análise de sentimento: {sentiment}")
        
        # Estatísticas
        stats = service.get_usage_statistics()
        print(f"Estatísticas: {stats}")
    
    asyncio.run(test_service())

if __name__ == "__main__":
    main()
