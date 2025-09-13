#!/usr/bin/env python3
"""
FORTIS Analytics - Testes de Modelos
Testes unitários para os modelos de analytics
"""

import unittest
import pandas as pd
import numpy as np
from unittest.mock import Mock, patch
import sys
import os

# Adiciona o diretório pai ao path para importar módulos
sys.path.append(os.path.join(os.path.dirname(__file__), '..'))

from models.participation_model import ParticipationModel
from models.security_analysis import SecurityAnalysisModel
from utils.analytics_utils import AnalyticsUtils

class TestParticipationModel(unittest.TestCase):
    """Testes para o modelo de participação"""
    
    def setUp(self):
        self.model = ParticipationModel()
        self.sample_data = pd.DataFrame({
            'state': ['SP', 'RJ', 'MG', 'RS', 'PR'],
            'population': [46000000, 17000000, 21000000, 11000000, 11000000],
            'votes': [500000, 300000, 250000, 200000, 180000],
            'participation_rate': [85.2, 78.5, 82.1, 79.3, 81.7],
            'election_date': ['2025-10-01', '2025-10-01', '2025-10-01', '2025-10-01', '2025-10-01'],
            'candidates_count': [4, 4, 4, 4, 4],
            'previous_participation': [84.0, 77.0, 81.0, 78.0, 80.0]
        })
    
    def test_prepare_features(self):
        """Testa preparação de features"""
        features = self.model.prepare_features(self.sample_data)
        
        self.assertIsInstance(features, pd.DataFrame)
        self.assertGreater(len(features.columns), 0)
        self.assertEqual(len(features), len(self.sample_data))
    
    def test_train_model(self):
        """Testa treinamento do modelo"""
        results = self.model.train_model(self.sample_data)
        
        self.assertIn('r2', results)
        self.assertIn('mse', results)
        self.assertIn('training_samples', results)
        self.assertGreaterEqual(results['r2'], 0.0)
        self.assertLessEqual(results['r2'], 1.0)
    
    def test_predict_participation(self):
        """Testa predição de participação"""
        # Treina modelo primeiro
        self.model.train_model(self.sample_data)
        
        test_data = {
            'state': 'SC',
            'population': 7000000,
            'votes': 150000,
            'election_date': '2025-10-01',
            'candidates_count': 4,
            'previous_participation': 82.0
        }
        
        prediction = self.model.predict_participation(test_data)
        
        self.assertIn('predicted_participation', prediction)
        self.assertIn('confidence_lower', prediction)
        self.assertIn('confidence_upper', prediction)
        self.assertGreaterEqual(prediction['predicted_participation'], 0.0)
        self.assertLessEqual(prediction['predicted_participation'], 100.0)
    
    def test_analyze_participation_factors(self):
        """Testa análise de fatores de participação"""
        # Treina modelo primeiro
        self.model.train_model(self.sample_data)
        
        analysis = self.model.analyze_participation_factors(self.sample_data)
        
        self.assertIn('feature_importance', analysis)
        self.assertIn('correlations', analysis)
        self.assertIn('top_factors', analysis)
        self.assertIsInstance(analysis['feature_importance'], list)
    
    def test_generate_participation_insights(self):
        """Testa geração de insights"""
        insights = self.model.generate_participation_insights(self.sample_data)
        
        self.assertIsInstance(insights, list)
        self.assertGreater(len(insights), 0)
        self.assertIsInstance(insights[0], str)

class TestSecurityAnalysisModel(unittest.TestCase):
    """Testes para o modelo de análise de segurança"""
    
    def setUp(self):
        self.model = SecurityAnalysisModel()
        self.sample_data = pd.DataFrame({
            'vote_id': [f'vote_{i}' for i in range(1000)],
            'timestamp': pd.date_range('2025-10-01 08:00:00', periods=1000, freq='1min'),
            'zone_id': np.random.choice(['Z001', 'Z002', 'Z003', 'Z004'], 1000),
            'section_id': np.random.choice(['S001', 'S002', 'S003'], 1000),
            'voter_cpf': [f'{np.random.randint(10000000000, 99999999999)}' for _ in range(1000)],
            'candidate_id': np.random.choice(['C001', 'C002', 'C003', 'C004'], 1000),
            'is_verified': np.random.choice([True, False], 1000, p=[0.95, 0.05]),
            'is_audited': np.random.choice([True, False], 1000, p=[0.8, 0.2])
        })
    
    def test_prepare_security_features(self):
        """Testa preparação de features de segurança"""
        features = self.model.prepare_security_features(self.sample_data)
        
        self.assertIsInstance(features, pd.DataFrame)
        self.assertGreater(len(features.columns), 0)
        self.assertEqual(len(features), len(self.sample_data))
    
    def test_train_anomaly_detector(self):
        """Testa treinamento do detector de anomalias"""
        results = self.model.train_anomaly_detector(self.sample_data)
        
        self.assertIn('training_samples', results)
        self.assertIn('anomalies_detected', results)
        self.assertIn('anomaly_rate', results)
        self.assertGreaterEqual(results['training_samples'], 0)
        self.assertGreaterEqual(results['anomalies_detected'], 0)
    
    def test_train_clusterer(self):
        """Testa treinamento do clusterer"""
        results = self.model.train_clusterer(self.sample_data)
        
        self.assertIn('n_clusters', results)
        self.assertIn('n_noise', results)
        self.assertIn('silhouette_score', results)
        self.assertGreaterEqual(results['n_clusters'], 0)
        self.assertGreaterEqual(results['n_noise'], 0)
    
    def test_detect_anomalies(self):
        """Testa detecção de anomalias"""
        # Treina modelo primeiro
        self.model.train_anomaly_detector(self.sample_data)
        
        anomaly_analysis = self.model.detect_anomalies(self.sample_data)
        
        self.assertIn('total_votes', anomaly_analysis)
        self.assertIn('anomalies_detected', anomaly_analysis)
        self.assertIn('anomaly_rate', anomaly_analysis)
        self.assertIn('security_score', anomaly_analysis)
        self.assertGreaterEqual(anomaly_analysis['total_votes'], 0)
        self.assertGreaterEqual(anomaly_analysis['anomalies_detected'], 0)
    
    def test_analyze_voting_patterns(self):
        """Testa análise de padrões de votação"""
        patterns = self.model.analyze_voting_patterns(self.sample_data)
        
        self.assertIsInstance(patterns, dict)
        # Verifica se pelo menos algumas chaves estão presentes
        expected_keys = ['peak_hour', 'lowest_hour', 'hourly_distribution']
        for key in expected_keys:
            if key in patterns:
                self.assertIsNotNone(patterns[key])
    
    def test_generate_security_report(self):
        """Testa geração de relatório de segurança"""
        # Treina modelo primeiro
        self.model.train_anomaly_detector(self.sample_data)
        
        report = self.model.generate_security_report(self.sample_data)
        
        self.assertIn('summary', report)
        self.assertIn('anomaly_analysis', report)
        self.assertIn('pattern_analysis', report)
        self.assertIn('recommendations', report)
        self.assertIsInstance(report['recommendations'], list)

class TestAnalyticsUtils(unittest.TestCase):
    """Testes para utilitários de analytics"""
    
    def setUp(self):
        self.utils = AnalyticsUtils()
        self.sample_data = pd.DataFrame({
            'state': ['SP', 'RJ', 'MG', 'RS', 'PR'],
            'participation_rate': [85.2, 78.5, 82.1, 79.3, 81.7],
            'total_votes': [500000, 300000, 250000, 200000, 180000],
            'unique_voters': [400000, 240000, 200000, 160000, 144000],
            'is_verified': [True, True, True, True, True],
            'is_audited': [True, True, False, True, False],
            'timestamp': pd.date_range('2025-10-01 08:00:00', periods=5, freq='1H')
        })
    
    def test_calculate_participation_metrics(self):
        """Testa cálculo de métricas de participação"""
        metrics = self.utils.calculate_participation_metrics(self.sample_data)
        
        self.assertIn('mean_participation', metrics)
        self.assertIn('median_participation', metrics)
        self.assertIn('std_participation', metrics)
        self.assertGreaterEqual(metrics['mean_participation'], 0.0)
        self.assertLessEqual(metrics['mean_participation'], 100.0)
    
    def test_analyze_voting_patterns(self):
        """Testa análise de padrões de votação"""
        patterns = self.utils.analyze_voting_patterns(self.sample_data)
        
        self.assertIsInstance(patterns, dict)
        # Verifica se pelo menos algumas chaves estão presentes
        if 'peak_hour' in patterns:
            self.assertIsInstance(patterns['peak_hour'], int)
    
    def test_detect_outliers(self):
        """Testa detecção de outliers"""
        outliers = self.utils.detect_outliers(self.sample_data, 'participation_rate')
        
        self.assertIsInstance(outliers, list)
        self.assertGreaterEqual(len(outliers), 0)
    
    def test_calculate_correlation_matrix(self):
        """Testa cálculo de matriz de correlação"""
        correlation_matrix = self.utils.calculate_correlation_matrix(self.sample_data)
        
        self.assertIsInstance(correlation_matrix, pd.DataFrame)
        self.assertGreater(len(correlation_matrix.columns), 0)
    
    def test_generate_time_series_features(self):
        """Testa geração de features de série temporal"""
        features = self.utils.generate_time_series_features(
            self.sample_data, 'timestamp', 'participation_rate'
        )
        
        self.assertIsInstance(features, pd.DataFrame)
        self.assertGreater(len(features.columns), len(self.sample_data.columns))
    
    def test_create_geographic_analysis(self):
        """Testa criação de análise geográfica"""
        analysis = self.utils.create_geographic_analysis(
            self.sample_data, 'state', 'participation_rate'
        )
        
        self.assertIn('total_states', analysis)
        self.assertIn('total_value', analysis)
        self.assertIn('top_state', analysis)
        self.assertIn('bottom_state', analysis)
        self.assertGreater(analysis['total_states'], 0)
    
    def test_calculate_security_metrics(self):
        """Testa cálculo de métricas de segurança"""
        metrics = self.utils.calculate_security_metrics(self.sample_data)
        
        self.assertIn('verification_rate', metrics)
        self.assertIn('audit_rate', metrics)
        self.assertIn('overall_security_score', metrics)
        self.assertGreaterEqual(metrics['verification_rate'], 0.0)
        self.assertLessEqual(metrics['verification_rate'], 1.0)
    
    def test_generate_insights(self):
        """Testa geração de insights"""
        insights = self.utils.generate_insights(self.sample_data)
        
        self.assertIsInstance(insights, list)
        self.assertGreater(len(insights), 0)
        self.assertIsInstance(insights[0], str)
    
    def test_validate_data_integrity(self):
        """Testa validação de integridade dos dados"""
        validation = self.utils.validate_data_integrity(self.sample_data)
        
        self.assertIn('is_valid', validation)
        self.assertIn('issues', validation)
        self.assertIn('warnings', validation)
        self.assertIsInstance(validation['is_valid'], bool)
        self.assertIsInstance(validation['issues'], list)
        self.assertIsInstance(validation['warnings'], list)
    
    def test_generate_data_hash(self):
        """Testa geração de hash dos dados"""
        hash_value = self.utils.generate_data_hash(self.sample_data)
        
        self.assertIsInstance(hash_value, str)
        self.assertEqual(len(hash_value), 64)  # SHA-256 hash length
    
    def test_export_to_json(self):
        """Testa exportação para JSON"""
        success = self.utils.export_to_json(self.sample_data, 'test_output.json')
        
        self.assertTrue(success)
        self.assertTrue(os.path.exists('test_output.json'))
        
        # Limpa arquivo de teste
        if os.path.exists('test_output.json'):
            os.remove('test_output.json')
    
    def test_export_to_csv(self):
        """Testa exportação para CSV"""
        success = self.utils.export_to_csv(self.sample_data, 'test_output.csv')
        
        self.assertTrue(success)
        self.assertTrue(os.path.exists('test_output.csv'))
        
        # Limpa arquivo de teste
        if os.path.exists('test_output.csv'):
            os.remove('test_output.csv')

class TestAnalyticsIntegration(unittest.TestCase):
    """Testes de integração para analytics"""
    
    def setUp(self):
        self.participation_model = ParticipationModel()
        self.security_model = SecurityAnalysisModel()
        self.utils = AnalyticsUtils()
        
        self.sample_data = pd.DataFrame({
            'state': ['SP', 'RJ', 'MG', 'RS', 'PR'],
            'population': [46000000, 17000000, 21000000, 11000000, 11000000],
            'votes': [500000, 300000, 250000, 200000, 180000],
            'participation_rate': [85.2, 78.5, 82.1, 79.3, 81.7],
            'election_date': ['2025-10-01', '2025-10-01', '2025-10-01', '2025-10-01', '2025-10-01'],
            'candidates_count': [4, 4, 4, 4, 4],
            'previous_participation': [84.0, 77.0, 81.0, 78.0, 80.0],
            'timestamp': pd.date_range('2025-10-01 08:00:00', periods=5, freq='1H'),
            'is_verified': [True, True, True, True, True],
            'is_audited': [True, True, False, True, False]
        })
    
    def test_end_to_end_analysis(self):
        """Testa análise completa end-to-end"""
        # Treina modelos
        participation_results = self.participation_model.train_model(self.sample_data)
        security_results = self.security_model.train_anomaly_detector(self.sample_data)
        
        # Valida treinamento
        self.assertGreater(participation_results['r2'], 0.0)
        self.assertGreater(security_results['training_samples'], 0)
        
        # Gera insights
        insights = self.utils.generate_insights(self.sample_data)
        self.assertGreater(len(insights), 0)
        
        # Calcula métricas
        participation_metrics = self.utils.calculate_participation_metrics(self.sample_data)
        security_metrics = self.utils.calculate_security_metrics(self.sample_data)
        
        self.assertIn('mean_participation', participation_metrics)
        self.assertIn('overall_security_score', security_metrics)
        
        # Valida dados
        validation = self.utils.validate_data_integrity(self.sample_data)
        self.assertTrue(validation['is_valid'])

def run_analytics_tests():
    """Executa todos os testes de analytics"""
    # Cria suite de testes
    test_suite = unittest.TestSuite()
    
    # Adiciona testes
    test_classes = [
        TestParticipationModel,
        TestSecurityAnalysisModel,
        TestAnalyticsUtils,
        TestAnalyticsIntegration
    ]
    
    for test_class in test_classes:
        tests = unittest.TestLoader().loadTestsFromTestCase(test_class)
        test_suite.addTests(tests)
    
    # Executa testes
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(test_suite)
    
    return result.wasSuccessful()

if __name__ == "__main__":
    print("Executando testes de analytics do FORTIS...")
    success = run_analytics_tests()
    
    if success:
        print("\n✅ Todos os testes de analytics passaram!")
    else:
        print("\n❌ Alguns testes de analytics falharam!")
        sys.exit(1)
