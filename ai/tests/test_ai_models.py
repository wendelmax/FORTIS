#!/usr/bin/env python3
"""
FORTIS - Testes de Modelos de IA
Testes unitários para os modelos de IA do sistema FORTIS
"""

import unittest
import numpy as np
import pandas as pd
from unittest.mock import Mock, patch
import sys
import os

# Adiciona o diretório pai ao path para importar módulos
sys.path.append(os.path.join(os.path.dirname(__file__), '..'))

from src.models.face_recognition_model import FaceRecognitionModel
from src.models.fraud_detection_model import FraudDetectionModel
from src.prediction.vote_prediction import VotePredictionModel
from src.preprocessing.data_cleaner import DataCleaner
from src.utils.ai_utils import AIUtils

class TestFaceRecognitionModel(unittest.TestCase):
    """Testes para o modelo de reconhecimento facial"""
    
    def setUp(self):
        self.model = FaceRecognitionModel()
        self.sample_image = np.random.randint(0, 255, (100, 100, 3), dtype=np.uint8)
        self.sample_cpf = "12345678901"
        self.sample_name = "João Silva"
    
    def test_add_face(self):
        """Testa adição de face ao modelo"""
        # Mock da função extract_face_encoding
        with patch.object(self.model, 'extract_face_encoding') as mock_extract:
            mock_extract.return_value = np.random.rand(128)  # Encoding simulado
            
            result = self.model.add_face(self.sample_image, self.sample_cpf, self.sample_name)
            
            self.assertTrue(result)
            self.assertIn(self.sample_cpf, self.model.known_faces)
    
    def test_verify_voter(self):
        """Testa verificação de eleitor"""
        # Adiciona face conhecida
        with patch.object(self.model, 'extract_face_encoding') as mock_extract:
            mock_extract.return_value = np.random.rand(128)
            self.model.add_face(self.sample_image, self.sample_cpf, self.sample_name)
        
        # Testa verificação
        with patch.object(self.model, 'extract_face_encoding') as mock_extract:
            mock_extract.return_value = np.random.rand(128)
            
            result = self.model.verify_voter(self.sample_image, self.sample_cpf)
            
            self.assertIn('verified', result)
            self.assertIn('confidence', result)
    
    def test_get_model_stats(self):
        """Testa estatísticas do modelo"""
        stats = self.model.get_model_stats()
        
        self.assertIn('total_faces', stats)
        self.assertIn('model_path', stats)
        self.assertIn('last_updated', stats)

class TestFraudDetectionModel(unittest.TestCase):
    """Testes para o modelo de detecção de fraude"""
    
    def setUp(self):
        self.model = FraudDetectionModel()
        self.sample_data = {
            'hour': 14,
            'votes_per_hour': 10,
            'same_location_votes': 5,
            'is_fraud': False
        }
    
    def test_detect_fraud(self):
        """Testa detecção de fraude"""
        result = self.model.detect_fraud(self.sample_data)
        
        self.assertIn('is_fraud', result)
        self.assertIn('risk_level', result)
        self.assertIn('anomaly_score', result)
        self.assertIn('behavior_score', result)
    
    def test_train_model(self):
        """Testa treinamento do modelo"""
        # Cria dados de treinamento
        data = []
        for i in range(100):
            data.append({
                'hour': np.random.randint(0, 24),
                'votes_per_hour': np.random.poisson(10),
                'same_location_votes': np.random.poisson(5),
                'is_fraud': np.random.random() < 0.1
            })
        
        df = pd.DataFrame(data)
        accuracy = self.model.train_model(df)
        
        self.assertIsInstance(accuracy, float)
        self.assertGreaterEqual(accuracy, 0.0)
        self.assertLessEqual(accuracy, 1.0)

class TestVotePredictionModel(unittest.TestCase):
    """Testes para o modelo de predição de votos"""
    
    def setUp(self):
        self.model = VotePredictionModel()
        self.sample_voter_data = {
            'idade': 35,
            'sexo': 'M',
            'estado': 'SP',
            'votou_ultima_eleicao': 1
        }
    
    def test_predict_turnout(self):
        """Testa predição de participação"""
        # Mock do modelo treinado
        self.model.models['turnout'] = Mock()
        self.model.scalers['turnout'] = Mock()
        self.model.scalers['turnout'].transform.return_value = np.array([[0.5, 0.3, 0.2]])
        self.model.models['turnout'].predict_proba.return_value = np.array([[0.3, 0.7]])
        
        result = self.model.predict_turnout(self.sample_voter_data)
        
        self.assertIn('turnout_probability', result)
        self.assertIn('predicted_turnout', result)
        self.assertIn('confidence', result)
    
    def test_predict_candidate_preference(self):
        """Testa predição de preferência de candidato"""
        # Mock do modelo treinado
        self.model.models['candidate'] = Mock()
        self.model.scalers['candidate'] = Mock()
        self.model.scalers['candidate'].transform.return_value = np.array([[0.5, 0.3, 0.2]])
        self.model.models['candidate'].predict.return_value = np.array([1.5])
        
        result = self.model.predict_candidate_preference(self.sample_voter_data)
        
        self.assertIn('predicted_candidate', result)
        self.assertIn('prediction_score', result)
        self.assertIn('confidence', result)

class TestDataCleaner(unittest.TestCase):
    """Testes para o limpador de dados"""
    
    def setUp(self):
        self.cleaner = DataCleaner()
        self.sample_data = pd.DataFrame({
            'cpf': ['12345678901', '11111111111', '98765432109'],
            'nome': ['João Silva', 'Maria Santos', 'Pedro Oliveira'],
            'data_nascimento': ['1985-03-15', '1990-07-22', '1980-12-12'],
            'estado': ['SP', 'RJ', 'MG']
        })
    
    def test_clean_voter_data(self):
        """Testa limpeza de dados de eleitores"""
        cleaned_data = self.cleaner.clean_voter_data(self.sample_data)
        
        self.assertIsInstance(cleaned_data, pd.DataFrame)
        self.assertLessEqual(len(cleaned_data), len(self.sample_data))
    
    def test_validate_cpf(self):
        """Testa validação de CPF"""
        # CPF válido
        valid_cpf = "12345678901"
        result = self.cleaner._validate_cpf(pd.DataFrame({'cpf': [valid_cpf]}))
        self.assertEqual(len(result), 1)
        
        # CPF inválido
        invalid_cpf = "11111111111"
        result = self.cleaner._validate_cpf(pd.DataFrame({'cpf': [invalid_cpf]}))
        self.assertEqual(len(result), 0)
    
    def test_get_cleaning_report(self):
        """Testa geração de relatório de limpeza"""
        self.cleaner.clean_voter_data(self.sample_data)
        report = self.cleaner.get_cleaning_report()
        
        self.assertIn('statistics', report)
        self.assertIn('data_quality_score', report)
        self.assertIn('recommendations', report)

class TestAIUtils(unittest.TestCase):
    """Testes para utilitários de IA"""
    
    def setUp(self):
        self.utils = AIUtils()
    
    def test_calculate_confidence_score(self):
        """Testa cálculo de score de confiança"""
        predictions = [0.8, 0.9, 0.7, 0.85]
        confidence = self.utils.calculate_confidence_score(predictions)
        
        self.assertIsInstance(confidence, float)
        self.assertGreaterEqual(confidence, 0.0)
        self.assertLessEqual(confidence, 1.0)
    
    def test_detect_outliers(self):
        """Testa detecção de outliers"""
        data = [1, 2, 3, 4, 5, 100]  # 100 é outlier
        outliers = self.utils.detect_outliers(data)
        
        self.assertEqual(len(outliers), len(data))
        self.assertTrue(outliers[-1])  # Último elemento é outlier
    
    def test_calculate_metrics(self):
        """Testa cálculo de métricas"""
        y_true = [1, 0, 1, 0, 1]
        y_pred = [1, 0, 0, 0, 1]
        metrics = self.utils.calculate_metrics(y_true, y_pred)
        
        self.assertIn('mse', metrics)
        self.assertIn('rmse', metrics)
        self.assertIn('mae', metrics)
        self.assertIn('r2', metrics)
    
    def test_validate_image(self):
        """Testa validação de imagem"""
        # Imagem válida
        valid_image = np.random.randint(0, 255, (100, 100, 3), dtype=np.uint8)
        self.assertTrue(self.utils.validate_image(valid_image))
        
        # Imagem inválida (muito pequena)
        invalid_image = np.random.randint(0, 255, (10, 10, 3), dtype=np.uint8)
        self.assertFalse(self.utils.validate_image(invalid_image))
    
    def test_calculate_similarity(self):
        """Testa cálculo de similaridade"""
        vec1 = np.array([1, 2, 3])
        vec2 = np.array([1, 2, 3])
        
        similarity = self.utils.calculate_similarity(vec1, vec2)
        
        self.assertIsInstance(similarity, float)
        self.assertGreaterEqual(similarity, 0.0)
        self.assertLessEqual(similarity, 1.0)

class TestIntegration(unittest.TestCase):
    """Testes de integração"""
    
    def test_end_to_end_workflow(self):
        """Testa fluxo completo de IA"""
        # 1. Limpa dados
        cleaner = DataCleaner()
        sample_data = pd.DataFrame({
            'cpf': ['12345678901', '98765432109'],
            'nome': ['João Silva', 'Maria Santos'],
            'data_nascimento': ['1985-03-15', '1990-07-22'],
            'estado': ['SP', 'RJ']
        })
        
        cleaned_data = cleaner.clean_voter_data(sample_data)
        self.assertIsInstance(cleaned_data, pd.DataFrame)
        
        # 2. Detecta fraude
        fraud_model = FraudDetectionModel()
        fraud_data = {
            'hour': 14,
            'votes_per_hour': 10,
            'same_location_votes': 5
        }
        
        fraud_result = fraud_model.detect_fraud(fraud_data)
        self.assertIn('is_fraud', fraud_result)
        
        # 3. Prediz comportamento
        prediction_model = VotePredictionModel()
        voter_data = {
            'idade': 35,
            'sexo': 'M',
            'estado': 'SP',
            'votou_ultima_eleicao': 1
        }
        
        # Mock para teste
        prediction_model.models['turnout'] = Mock()
        prediction_model.scalers['turnout'] = Mock()
        prediction_model.scalers['turnout'].transform.return_value = np.array([[0.5, 0.3, 0.2]])
        prediction_model.models['turnout'].predict_proba.return_value = np.array([[0.3, 0.7]])
        
        prediction_result = prediction_model.predict_turnout(voter_data)
        self.assertIn('turnout_probability', prediction_result)

def run_tests():
    """Executa todos os testes"""
    # Cria suite de testes
    test_suite = unittest.TestSuite()
    
    # Adiciona testes
    test_classes = [
        TestFaceRecognitionModel,
        TestFraudDetectionModel,
        TestVotePredictionModel,
        TestDataCleaner,
        TestAIUtils,
        TestIntegration
    ]
    
    for test_class in test_classes:
        tests = unittest.TestLoader().loadTestsFromTestCase(test_class)
        test_suite.addTests(tests)
    
    # Executa testes
    runner = unittest.TextTestRunner(verbosity=2)
    result = runner.run(test_suite)
    
    return result.wasSuccessful()

if __name__ == "__main__":
    print("Executando testes de IA do FORTIS...")
    success = run_tests()
    
    if success:
        print("\n✅ Todos os testes passaram!")
    else:
        print("\n❌ Alguns testes falharam!")
        sys.exit(1)
