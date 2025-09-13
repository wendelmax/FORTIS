#!/usr/bin/env python3
"""
FORTIS - Treinamento de Modelos de IA
Script para treinar todos os modelos de IA do sistema FORTIS
"""

import asyncio
import logging
import pandas as pd
import numpy as np
from typing import Dict, List, Optional, Tuple
from datetime import datetime
import os
import json
from pathlib import Path

# Importa modelos
from ..models.face_recognition_model import FaceRecognitionModel
from ..models.fraud_detection_model import FraudDetectionModel
from ..prediction.vote_prediction import VotePredictionModel
from ..preprocessing.data_cleaner import DataCleaner

class ModelTrainer:
    """Treinador de modelos de IA para o sistema FORTIS"""
    
    def __init__(self, data_path: str = "ai/data", model_path: str = "ai/data/models"):
        self.data_path = Path(data_path)
        self.model_path = Path(model_path)
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Estatísticas de treinamento
        self.training_stats = {
            'start_time': None,
            'end_time': None,
            'models_trained': 0,
            'total_errors': 0,
            'training_results': {}
        }
    
    async def train_all_models(self, training_data: Optional[Dict] = None):
        """Treina todos os modelos de IA"""
        try:
            self.training_stats['start_time'] = datetime.now().isoformat()
            self.logger.info("Iniciando treinamento de todos os modelos...")
            
            # Cria diretório de modelos se não existir
            self.model_path.mkdir(parents=True, exist_ok=True)
            
            # Treina modelo de reconhecimento facial
            await self._train_face_recognition_model(training_data)
            
            # Treina modelo de detecção de fraude
            await self._train_fraud_detection_model(training_data)
            
            # Treina modelo de predição de votos
            await self._train_vote_prediction_model(training_data)
            
            self.training_stats['end_time'] = datetime.now().isoformat()
            self.training_stats['models_trained'] = 3
            
            self.logger.info("Treinamento de todos os modelos concluído!")
            self._save_training_report()
            
        except Exception as e:
            self.logger.error(f"Erro no treinamento geral: {e}")
            self.training_stats['total_errors'] += 1
            raise
    
    async def _train_face_recognition_model(self, training_data: Optional[Dict] = None):
        """Treina modelo de reconhecimento facial"""
        try:
            self.logger.info("Treinando modelo de reconhecimento facial...")
            
            # Inicializa modelo
            face_model = FaceRecognitionModel()
            
            # Simula dados de treinamento (em produção, viria de banco de dados)
            if training_data and 'face_samples' in training_data:
                face_samples = training_data['face_samples']
            else:
                face_samples = self._generate_sample_face_data()
            
            # Treina modelo com dados simulados
            trained_faces = 0
            for sample in face_samples:
                success = face_model.add_face(
                    sample['image'], 
                    sample['cpf'], 
                    sample['name']
                )
                if success:
                    trained_faces += 1
            
            # Salva modelo
            face_model.save_model()
            
            self.training_stats['training_results']['face_recognition'] = {
                'status': 'success',
                'trained_faces': trained_faces,
                'model_path': str(face_model.model_path)
            }
            
            self.logger.info(f"Modelo de reconhecimento facial treinado: {trained_faces} faces")
            
        except Exception as e:
            self.logger.error(f"Erro no treinamento de reconhecimento facial: {e}")
            self.training_stats['training_results']['face_recognition'] = {
                'status': 'error',
                'error': str(e)
            }
            self.training_stats['total_errors'] += 1
    
    async def _train_fraud_detection_model(self, training_data: Optional[Dict] = None):
        """Treina modelo de detecção de fraude"""
        try:
            self.logger.info("Treinando modelo de detecção de fraude...")
            
            # Inicializa modelo
            fraud_model = FraudDetectionModel()
            
            # Carrega dados de treinamento
            if training_data and 'fraud_samples' in training_data:
                fraud_samples = training_data['fraud_samples']
            else:
                fraud_samples = self._generate_sample_fraud_data()
            
            # Converte para DataFrame
            df = pd.DataFrame(fraud_samples)
            
            # Treina modelo
            accuracy = fraud_model.train_model(df)
            
            # Salva modelo
            fraud_model.save_model()
            
            self.training_stats['training_results']['fraud_detection'] = {
                'status': 'success',
                'accuracy': accuracy,
                'training_samples': len(fraud_samples),
                'model_path': str(fraud_model.model_path)
            }
            
            self.logger.info(f"Modelo de detecção de fraude treinado: {accuracy:.3f} de acurácia")
            
        except Exception as e:
            self.logger.error(f"Erro no treinamento de detecção de fraude: {e}")
            self.training_stats['training_results']['fraud_detection'] = {
                'status': 'error',
                'error': str(e)
            }
            self.training_stats['total_errors'] += 1
    
    async def _train_vote_prediction_model(self, training_data: Optional[Dict] = None):
        """Treina modelo de predição de votos"""
        try:
            self.logger.info("Treinando modelo de predição de votos...")
            
            # Inicializa modelo
            prediction_model = VotePredictionModel()
            
            # Carrega dados de treinamento
            if training_data and 'voter_data' in training_data:
                voter_data = training_data['voter_data']
            else:
                voter_data = self._generate_sample_voter_data()
            
            # Converte para DataFrame
            df = pd.DataFrame(voter_data)
            
            # Treina modelos
            turnout_accuracy = prediction_model.train_turnout_model(df)
            candidate_r2 = prediction_model.train_candidate_preference_model(df)
            
            # Salva modelo
            prediction_model.save_model()
            
            self.training_stats['training_results']['vote_prediction'] = {
                'status': 'success',
                'turnout_accuracy': turnout_accuracy,
                'candidate_r2': candidate_r2,
                'training_samples': len(voter_data),
                'model_path': str(prediction_model.model_path)
            }
            
            self.logger.info(f"Modelo de predição de votos treinado: {turnout_accuracy:.3f} acurácia, {candidate_r2:.3f} R²")
            
        except Exception as e:
            self.logger.error(f"Erro no treinamento de predição de votos: {e}")
            self.training_stats['training_results']['vote_prediction'] = {
                'status': 'error',
                'error': str(e)
            }
            self.training_stats['total_errors'] += 1
    
    def _generate_sample_face_data(self) -> List[Dict]:
        """Gera dados simulados de faces para treinamento"""
        # Em produção, isso viria de um banco de dados real
        return [
            {
                'cpf': '12345678901',
                'name': 'João Silva',
                'image': np.random.randint(0, 255, (100, 100, 3), dtype=np.uint8)
            },
            {
                'cpf': '98765432109',
                'name': 'Maria Santos',
                'image': np.random.randint(0, 255, (100, 100, 3), dtype=np.uint8)
            }
        ]
    
    def _generate_sample_fraud_data(self) -> List[Dict]:
        """Gera dados simulados de fraude para treinamento"""
        np.random.seed(42)
        n_samples = 1000
        
        data = []
        for i in range(n_samples):
            # Simula dados de votação
            is_fraud = np.random.random() < 0.1  # 10% de fraudes
            
            if is_fraud:
                # Padrões suspeitos para fraudes
                hour = np.random.choice([0, 1, 2, 3, 4, 5])  # Madrugada
                votes_per_hour = np.random.poisson(50)  # Muitos votos
                same_location_votes = np.random.poisson(20)  # Muitos votos do mesmo local
            else:
                # Padrões normais
                hour = np.random.choice(range(6, 22))  # Horário normal
                votes_per_hour = np.random.poisson(10)  # Poucos votos
                same_location_votes = np.random.poisson(5)  # Poucos votos do mesmo local
            
            data.append({
                'hour': hour,
                'votes_per_hour': votes_per_hour,
                'same_location_votes': same_location_votes,
                'is_fraud': is_fraud
            })
        
        return data
    
    def _generate_sample_voter_data(self) -> List[Dict]:
        """Gera dados simulados de eleitores para treinamento"""
        np.random.seed(42)
        n_samples = 2000
        
        data = []
        for i in range(n_samples):
            # Simula dados demográficos
            age = np.random.randint(16, 80)
            sex = np.random.choice(['M', 'F'])
            state = np.random.choice(['SP', 'RJ', 'MG', 'RS', 'PR'])
            
            # Simula participação baseada em idade e sexo
            participation_prob = 0.7
            if age > 60:
                participation_prob += 0.2
            if sex == 'F':
                participation_prob += 0.1
            
            voted = np.random.random() < participation_prob
            
            # Simula candidato baseado em demografia
            if voted:
                if age < 30:
                    candidate = 'CAND001'  # Candidato jovem
                elif age < 50:
                    candidate = 'CAND002'  # Candidato maduro
                else:
                    candidate = 'CAND003'  # Candidato sênior
            else:
                candidate = None
            
            data.append({
                'idade': age,
                'sexo': sex,
                'estado': state,
                'votou_ultima_eleicao': int(voted),
                'candidate_id': candidate,
                'timestamp': datetime.now()
            })
        
        return data
    
    def _save_training_report(self):
        """Salva relatório de treinamento"""
        try:
            report_path = self.model_path / "training_report.json"
            
            with open(report_path, 'w') as f:
                json.dump(self.training_stats, f, indent=2, default=str)
            
            self.logger.info(f"Relatório de treinamento salvo em: {report_path}")
            
        except Exception as e:
            self.logger.error(f"Erro ao salvar relatório: {e}")
    
    def get_training_status(self) -> Dict:
        """Retorna status do treinamento"""
        return {
            'is_training': self.training_stats['start_time'] is not None and self.training_stats['end_time'] is None,
            'models_trained': self.training_stats['models_trained'],
            'total_errors': self.training_stats['total_errors'],
            'training_results': self.training_stats['training_results']
        }

async def main():
    """Função principal para treinamento"""
    trainer = ModelTrainer()
    
    print("Sistema de Treinamento de Modelos FORTIS")
    print("Iniciando treinamento...")
    
    try:
        await trainer.train_all_models()
        
        status = trainer.get_training_status()
        print(f"Treinamento concluído!")
        print(f"Modelos treinados: {status['models_trained']}")
        print(f"Erros: {status['total_errors']}")
        
        for model_name, result in status['training_results'].items():
            print(f"{model_name}: {result['status']}")
            if result['status'] == 'success':
                print(f"  - Detalhes: {result}")
        
    except Exception as e:
        print(f"Erro no treinamento: {e}")

if __name__ == "__main__":
    asyncio.run(main())
