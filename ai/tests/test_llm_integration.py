#!/usr/bin/env python3
"""
FORTIS - Testes de Integração LLM
Testes para integração com modelos de linguagem locais
"""

import unittest
import asyncio
import json
from unittest.mock import Mock, patch, AsyncMock
import sys
import os

# Adiciona o diretório pai ao path para importar módulos
sys.path.append(os.path.join(os.path.dirname(__file__), '..'))

from src.services.llm_service import LLMService
from src.models.nlp_models import ElectionTextAnalyzer
from src.utils.llm_utils import LLMUtils

class TestLLMService(unittest.TestCase):
    """Testes para o serviço de LLM"""
    
    def setUp(self):
        self.llm_service = LLMService({
            'ollama_url': 'http://localhost:11434',
            'default_model': 'llama3.2:3b',
            'timeout': 5
        })
    
    @patch('aiohttp.ClientSession')
    async def test_generate_text(self, mock_session):
        """Testa geração de texto"""
        # Mock da resposta da API
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.json = AsyncMock(return_value={
            'response': 'O sistema eleitoral digital é seguro e transparente.',
            'eval_count': 15,
            'prompt_eval_count': 10,
            'total_duration': 2000000000  # 2 segundos em nanosegundos
        })
        
        mock_session.return_value.__aenter__.return_value.post.return_value = mock_response
        
        result = await self.llm_service.generate_text("Explique o sistema eleitoral")
        
        self.assertTrue(result['success'])
        self.assertIn('text', result)
        self.assertIn('tokens_used', result)
        self.assertEqual(result['tokens_used'], 15)
    
    @patch('aiohttp.ClientSession')
    async def test_analyze_election_sentiment(self, mock_session):
        """Testa análise de sentimento eleitoral"""
        # Mock da resposta da API
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.json = AsyncMock(return_value={
            'response': '{"sentiment": "POSITIVO", "confidence": 0.8, "reasoning": "Texto favorável"}',
            'eval_count': 20,
            'prompt_eval_count': 15,
            'total_duration': 3000000000
        })
        
        mock_session.return_value.__aenter__.return_value.post.return_value = mock_response
        
        result = await self.llm_service.analyze_election_sentiment("O sistema eleitoral está funcionando muito bem!")
        
        self.assertTrue(result['success'])
        self.assertEqual(result['sentiment'], 'POSITIVO')
        self.assertEqual(result['confidence'], 0.8)
        self.assertIn('reasoning', result)
    
    @patch('aiohttp.ClientSession')
    async def test_extract_election_insights(self, mock_session):
        """Testa extração de insights eleitorais"""
        # Mock da resposta da API
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.json = AsyncMock(return_value={
            'response': 'Insights: Transparência, participação cidadã, tecnologia segura',
            'eval_count': 25,
            'prompt_eval_count': 20,
            'total_duration': 4000000000
        })
        
        mock_session.return_value.__aenter__.return_value.post.return_value = mock_response
        
        result = await self.llm_service.extract_election_insights("Análise do sistema eleitoral brasileiro")
        
        self.assertTrue(result['success'])
        self.assertIn('insights', result)
        self.assertIn('original_text', result)
    
    @patch('aiohttp.ClientSession')
    async def test_generate_election_report(self, mock_session):
        """Testa geração de relatório de eleição"""
        # Mock da resposta da API
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.json = AsyncMock(return_value={
            'response': '# Relatório de Eleição\n\n## Resumo Executivo\nSistema funcionando adequadamente.',
            'eval_count': 100,
            'prompt_eval_count': 50,
            'total_duration': 5000000000
        })
        
        mock_session.return_value.__aenter__.return_value.post.return_value = mock_response
        
        data = {
            'total_voters': 1000,
            'participation_rate': 85.5,
            'candidates': ['João', 'Maria', 'Pedro']
        }
        
        result = await self.llm_service.generate_election_report(data)
        
        self.assertTrue(result['success'])
        self.assertIn('report', result)
        self.assertIn('data_source', result)
    
    @patch('aiohttp.ClientSession')
    async def test_classify_election_issues(self, mock_session):
        """Testa classificação de problemas eleitorais"""
        # Mock da resposta da API
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.json = AsyncMock(return_value={
            'response': '{"categories": ["TÉCNICO", "SEGURANÇA"], "severity": "MÉDIA", "description": "Problemas de software"}',
            'eval_count': 30,
            'prompt_eval_count': 25,
            'total_duration': 3500000000
        })
        
        mock_session.return_value.__aenter__.return_value.post.return_value = mock_response
        
        result = await self.llm_service.classify_election_issues("O software da urna apresentou falhas")
        
        self.assertTrue(result['success'])
        self.assertIn('categories', result)
        self.assertIn('severity', result)
        self.assertIn('description', result)
    
    def test_get_usage_statistics(self):
        """Testa estatísticas de uso"""
        # Simula algumas operações
        self.llm_service.usage_stats['total_requests'] = 10
        self.llm_service.usage_stats['successful_requests'] = 8
        self.llm_service.usage_stats['failed_requests'] = 2
        self.llm_service.usage_stats['total_tokens'] = 1500
        self.llm_service.usage_stats['models_used'].add('llama3.2:3b')
        
        stats = self.llm_service.get_usage_statistics()
        
        self.assertEqual(stats['total_requests'], 10)
        self.assertEqual(stats['successful_requests'], 8)
        self.assertEqual(stats['failed_requests'], 2)
        self.assertEqual(stats['success_rate'], 0.8)
        self.assertEqual(stats['total_tokens'], 1500)
        self.assertIn('llama3.2:3b', stats['models_used'])

class TestElectionTextAnalyzer(unittest.TestCase):
    """Testes para o analisador de texto eleitoral"""
    
    def setUp(self):
        self.analyzer = ElectionTextAnalyzer()
    
    def test_preprocess_text(self):
        """Testa pré-processamento de texto"""
        text = "O SISTEMA ELEITORAL está funcionando MUITO BEM!!!"
        processed = self.analyzer.preprocess_text(text)
        
        self.assertIn('sistema eleitoral', processed)
        self.assertIn('funcionando', processed)
        self.assertIn('bem', processed)
        self.assertNotIn('!!!', processed)
    
    def test_extract_entities(self):
        """Testa extração de entidades"""
        text = "Candidato João Silva do Partido Democrático, CPF: 123.456.789-01, Seção 123, Zona 45"
        entities = self.analyzer.extract_entities(text)
        
        self.assertIn('123.456.789-01', entities['cpfs'])
        self.assertIn('João Silva', entities['candidatos'])
        self.assertIn('Democrático', entities['partidos'])
        self.assertIn('123', entities['secoes'])
        self.assertIn('45', entities['zonas'])
    
    def test_analyze_sentiment(self):
        """Testa análise de sentimento"""
        positive_text = "O sistema eleitoral é transparente e seguro, com total confiança"
        negative_text = "O sistema está cheio de fraudes e corrupção"
        neutral_text = "O sistema eleitoral funciona normalmente"
        
        pos_result = self.analyzer.analyze_sentiment(positive_text)
        neg_result = self.analyzer.analyze_sentiment(negative_text)
        neu_result = self.analyzer.analyze_sentiment(neutral_text)
        
        self.assertEqual(pos_result['sentiment'], 'POSITIVO')
        self.assertEqual(neg_result['sentiment'], 'NEGATIVO')
        self.assertEqual(neu_result['sentiment'], 'NEUTRO')
        
        self.assertGreater(pos_result['confidence'], 0.5)
        self.assertGreater(neg_result['confidence'], 0.5)
    
    def test_classify_text_type(self):
        """Testa classificação de tipo de texto"""
        technical_text = "O software da urna eletrônica utiliza criptografia AES-256"
        logistical_text = "A seção eleitoral está localizada na escola municipal"
        
        tech_result = self.analyzer.classify_text_type(technical_text)
        log_result = self.analyzer.classify_text_type(logistical_text)
        
        self.assertEqual(tech_result['type'], 'TECHNICAL')
        self.assertEqual(log_result['type'], 'LOGISTICAL')
    
    def test_extract_keywords(self):
        """Testa extração de palavras-chave"""
        text = "O sistema eleitoral digital é transparente, seguro e confiável. A democracia brasileira é forte."
        keywords = self.analyzer.extract_keywords(text, top_n=5)
        
        self.assertIsInstance(keywords, list)
        self.assertLessEqual(len(keywords), 5)
        self.assertIn(('sistema', 1), keywords)
        self.assertIn(('eleitoral', 1), keywords)
    
    def test_detect_anomalies(self):
        """Testa detecção de anomalias"""
        normal_text = "O sistema eleitoral está funcionando normalmente"
        anomalous_text = "123456789012345678901234567890"  # Número muito longo
        
        normal_result = self.analyzer.detect_anomalies(normal_text)
        anomalous_result = self.analyzer.detect_anomalies(anomalous_text)
        
        self.assertEqual(normal_result['anomalies_found'], 0)
        self.assertGreater(anomalous_result['anomalies_found'], 0)
    
    def test_analyze_text_complexity(self):
        """Testa análise de complexidade do texto"""
        simple_text = "O sistema funciona bem."
        complex_text = "O sistema eleitoral digital brasileiro, implementado com tecnologias de ponta e rigorosos protocolos de segurança, representa um marco na evolução democrática do país."
        
        simple_result = self.analyzer.analyze_text_complexity(simple_text)
        complex_result = self.analyzer.analyze_text_complexity(complex_text)
        
        self.assertEqual(simple_result['complexity'], 'LOW')
        self.assertEqual(complex_result['complexity'], 'HIGH')
    
    def test_comprehensive_analysis(self):
        """Testa análise completa"""
        text = "Candidato João Silva do Partido Democrático está liderando. O sistema eleitoral é transparente e seguro. CPF: 123.456.789-01"
        
        result = self.analyzer.comprehensive_analysis(text)
        
        self.assertIn('entities', result)
        self.assertIn('sentiment', result)
        self.assertIn('classification', result)
        self.assertIn('keywords', result)
        self.assertIn('anomalies', result)
        self.assertIn('complexity', result)
        
        self.assertIn('123.456.789-01', result['entities']['cpfs'])
        self.assertIn('João Silva', result['entities']['candidatos'])

class TestLLMUtils(unittest.TestCase):
    """Testes para utilitários de LLM"""
    
    def setUp(self):
        self.utils = LLMUtils()
    
    def test_create_prompt(self):
        """Testa criação de prompt"""
        prompt = self.utils.create_prompt('sentiment_analysis', text="Teste de sentimento")
        
        self.assertIn('system', prompt)
        self.assertIn('user', prompt)
        self.assertIn('Teste de sentimento', prompt['user'])
    
    def test_extract_json_from_response(self):
        """Testa extração de JSON da resposta"""
        response_text = '{"sentiment": "POSITIVO", "confidence": 0.8}'
        json_data = self.utils.extract_json_from_response(response_text)
        
        self.assertIsNotNone(json_data)
        self.assertEqual(json_data['sentiment'], 'POSITIVO')
        self.assertEqual(json_data['confidence'], 0.8)
    
    def test_validate_llm_response(self):
        """Testa validação de resposta do LLM"""
        valid_response = {
            'sentiment': 'POSITIVO',
            'confidence': 0.8,
            'reasoning': 'Texto positivo'
        }
        
        invalid_response = {
            'sentiment': 'INVALIDO',
            'confidence': 1.5
        }
        
        valid_result = self.utils.validate_llm_response(valid_response, ['sentiment', 'confidence'])
        invalid_result = self.utils.validate_llm_response(invalid_response, ['sentiment', 'confidence'])
        
        self.assertTrue(valid_result['is_valid'])
        self.assertFalse(invalid_result['is_valid'])
        self.assertIn('sentiment', invalid_result['invalid_fields'])
    
    def test_calculate_response_quality(self):
        """Testa cálculo de qualidade da resposta"""
        good_response = {
            'sentiment': 'POSITIVO',
            'confidence': 0.8,
            'reasoning': 'O sistema eleitoral está funcionando bem'
        }
        
        quality = self.utils.calculate_response_quality(good_response)
        
        self.assertIn('completeness', quality)
        self.assertIn('accuracy', quality)
        self.assertIn('relevance', quality)
        self.assertIn('overall_score', quality)
        
        self.assertGreater(quality['overall_score'], 0.5)
    
    def test_create_response_hash(self):
        """Testa criação de hash da resposta"""
        response = {
            'sentiment': 'POSITIVO',
            'confidence': 0.8,
            'timestamp': '2025-01-01T00:00:00'
        }
        
        hash1 = self.utils.create_response_hash(response)
        hash2 = self.utils.create_response_hash(response)
        
        self.assertEqual(hash1, hash2)
        self.assertIsInstance(hash1, str)
        self.assertEqual(len(hash1), 64)  # SHA-256 hash length
    
    def test_format_llm_response(self):
        """Testa formatação da resposta"""
        response = {
            'sentiment': 'POSITIVO',
            'confidence': 0.8,
            'reasoning': 'Texto positivo'
        }
        
        json_format = self.utils.format_llm_response(response, 'json')
        markdown_format = self.utils.format_llm_response(response, 'markdown')
        text_format = self.utils.format_llm_response(response, 'text')
        
        self.assertIn('"sentiment": "POSITIVO"', json_format)
        self.assertIn('# Análise de Texto Eleitoral', markdown_format)
        self.assertIn('Sentimento: POSITIVO', text_format)
    
    def test_create_llm_config(self):
        """Testa criação de configuração LLM"""
        config = self.utils.create_llm_config('llama3.2:3b', temperature=0.5, max_tokens=1000)
        
        self.assertEqual(config['model'], 'llama3.2:3b')
        self.assertEqual(config['temperature'], 0.5)
        self.assertEqual(config['max_tokens'], 1000)
        self.assertIn('top_p', config)

class TestLLMIntegration(unittest.TestCase):
    """Testes de integração LLM"""
    
    @patch('aiohttp.ClientSession')
    async def test_end_to_end_workflow(self, mock_session):
        """Testa fluxo completo de integração LLM"""
        # Mock da resposta da API
        mock_response = AsyncMock()
        mock_response.status = 200
        mock_response.json = AsyncMock(return_value={
            'response': '{"sentiment": "POSITIVO", "confidence": 0.9, "reasoning": "Sistema funcionando bem"}',
            'eval_count': 20,
            'prompt_eval_count': 15,
            'total_duration': 2000000000
        })
        
        mock_session.return_value.__aenter__.return_value.post.return_value = mock_response
        
        # Inicializa serviços
        llm_service = LLMService()
        text_analyzer = ElectionTextAnalyzer()
        utils = LLMUtils()
        
        # Texto de teste
        text = "O sistema eleitoral digital está funcionando muito bem com total transparência"
        
        # Análise com LLM
        llm_result = await llm_service.analyze_election_sentiment(text)
        
        # Análise local
        local_result = text_analyzer.analyze_sentiment(text)
        
        # Validação
        validation = utils.validate_llm_response(llm_result, ['sentiment', 'confidence'])
        
        # Verificações
        self.assertTrue(llm_result['success'])
        self.assertTrue(validation['is_valid'])
        self.assertEqual(local_result['sentiment'], 'POSITIVO')
        self.assertGreater(local_result['confidence'], 0.5)

def run_llm_tests():
    """Executa todos os testes de LLM"""
    # Cria suite de testes
    test_suite = unittest.TestSuite()
    
    # Adiciona testes
    test_classes = [
        TestLLMService,
        TestElectionTextAnalyzer,
        TestLLMUtils,
        TestLLMIntegration
    ]
    
    for test_class in test_classes:
        tests = unittest.TestLoader().loadTestsFromTestCase(test_class)
        test_suite.addTests(tests)
    
    # Executa testes
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(test_suite)
    
    return result.wasSuccessful()

if __name__ == "__main__":
    print("Executando testes de integração LLM do FORTIS...")
    success = run_llm_tests()
    
    if success:
        print("\n✅ Todos os testes de LLM passaram!")
    else:
        print("\n❌ Alguns testes de LLM falharam!")
        sys.exit(1)
