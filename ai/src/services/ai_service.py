#!/usr/bin/env python3
"""
FORTIS - Serviço de IA
Serviço principal de Inteligência Artificial para o sistema FORTIS
"""

import asyncio
import logging
from typing import Dict, List, Optional, Any
from datetime import datetime
import json
import os

# Importa módulos de IA
from ..models.face_recognition_model import FaceRecognitionModel
from ..models.fraud_detection_model import FraudDetectionModel
from ..prediction.vote_prediction import VotePredictionModel
from ..preprocessing.data_cleaner import DataCleaner

class AIService:
    """Serviço principal de IA para o sistema FORTIS"""
    
    def __init__(self, config: Optional[Dict] = None):
        self.config = config or {}
        self.is_initialized = False
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Inicializa modelos
        self.face_model = None
        self.fraud_model = None
        self.prediction_model = None
        self.data_cleaner = DataCleaner()
        
        # Estatísticas de uso
        self.usage_stats = {
            'face_verifications': 0,
            'fraud_detections': 0,
            'predictions': 0,
            'data_cleanings': 0,
            'errors': 0
        }
    
    async def initialize(self):
        """Inicializa o serviço de IA"""
        try:
            self.logger.info("Inicializando serviço de IA...")
            
            # Inicializa modelos
            self.face_model = FaceRecognitionModel()
            self.fraud_model = FraudDetectionModel()
            self.prediction_model = VotePredictionModel()
            
            # Carrega modelos treinados se existirem
            self.prediction_model.load_model()
            
            self.is_initialized = True
            self.logger.info("Serviço de IA inicializado com sucesso")
            
        except Exception as e:
            self.logger.error(f"Erro ao inicializar serviço de IA: {e}")
            self.usage_stats['errors'] += 1
            raise
    
    async def verify_voter_identity(self, face_image: bytes, cpf: str) -> Dict:
        """Verifica identidade do eleitor usando reconhecimento facial"""
        try:
            if not self.is_initialized:
                await self.initialize()
            
            self.logger.info(f"Verificando identidade do eleitor CPF: {cpf}")
            
            # Converte bytes para array numpy
            import cv2
            import numpy as np
            
            nparr = np.frombuffer(face_image, np.uint8)
            image = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
            
            # Verifica identidade
            result = self.face_model.verify_voter(image, cpf)
            
            self.usage_stats['face_verifications'] += 1
            
            return {
                'success': True,
                'verified': result['verified'],
                'confidence': result['confidence'],
                'timestamp': datetime.now().isoformat(),
                'cpf': cpf
            }
            
        except Exception as e:
            self.logger.error(f"Erro na verificação de identidade: {e}")
            self.usage_stats['errors'] += 1
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    async def detect_fraud(self, voting_data: Dict) -> Dict:
        """Detecta possíveis fraudes no processo de votação"""
        try:
            if not self.is_initialized:
                await self.initialize()
            
            self.logger.info("Analisando dados para detecção de fraude...")
            
            # Detecta fraudes
            result = self.fraud_model.detect_fraud(voting_data)
            
            self.usage_stats['fraud_detections'] += 1
            
            return {
                'success': True,
                'is_fraud': result['is_fraud'],
                'risk_level': result['risk_level'],
                'anomaly_score': result['anomaly_score'],
                'behavior_score': result['behavior_score'],
                'timestamp': datetime.now().isoformat()
            }
            
        except Exception as e:
            self.logger.error(f"Erro na detecção de fraude: {e}")
            self.usage_stats['errors'] += 1
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    async def predict_voter_behavior(self, voter_data: Dict) -> Dict:
        """Prediz comportamento de voto do eleitor"""
        try:
            if not self.is_initialized:
                await self.initialize()
            
            self.logger.info("Predizendo comportamento de voto...")
            
            # Prediz participação
            turnout_pred = self.prediction_model.predict_turnout(voter_data)
            
            # Prediz candidato
            candidate_pred = self.prediction_model.predict_candidate_preference(voter_data)
            
            self.usage_stats['predictions'] += 1
            
            return {
                'success': True,
                'turnout_prediction': turnout_pred,
                'candidate_prediction': candidate_pred,
                'timestamp': datetime.now().isoformat()
            }
            
        except Exception as e:
            self.logger.error(f"Erro na predição de comportamento: {e}")
            self.usage_stats['errors'] += 1
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    async def clean_election_data(self, data: List[Dict]) -> Dict:
        """Limpa dados de eleição"""
        try:
            if not self.is_initialized:
                await self.initialize()
            
            self.logger.info("Limpando dados de eleição...")
            
            # Converte para DataFrame
            import pandas as pd
            df = pd.DataFrame(data)
            
            # Limpa dados
            cleaned_df = self.data_cleaner.clean_voter_data(df)
            
            # Gera relatório
            report = self.data_cleaner.get_cleaning_report()
            
            self.usage_stats['data_cleanings'] += 1
            
            return {
                'success': True,
                'cleaned_data': cleaned_df.to_dict('records'),
                'cleaning_report': report,
                'timestamp': datetime.now().isoformat()
            }
            
        except Exception as e:
            self.logger.error(f"Erro na limpeza de dados: {e}")
            self.usage_stats['errors'] += 1
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    async def analyze_election_patterns(self, election_data: List[Dict]) -> Dict:
        """Analisa padrões de uma eleição"""
        try:
            if not self.is_initialized:
                await self.initialize()
            
            self.logger.info("Analisando padrões da eleição...")
            
            # Converte para DataFrame
            import pandas as pd
            df = pd.DataFrame(election_data)
            
            # Análise de participação
            participation_analysis = self._analyze_participation(df)
            
            # Análise de distribuição de votos
            vote_distribution = self._analyze_vote_distribution(df)
            
            # Análise temporal
            temporal_analysis = self._analyze_temporal_patterns(df)
            
            # Detecção de anomalias
            anomalies = await self._detect_election_anomalies(df)
            
            return {
                'success': True,
                'participation_analysis': participation_analysis,
                'vote_distribution': vote_distribution,
                'temporal_analysis': temporal_analysis,
                'anomalies': anomalies,
                'timestamp': datetime.now().isoformat()
            }
            
        except Exception as e:
            self.logger.error(f"Erro na análise de padrões: {e}")
            self.usage_stats['errors'] += 1
            return {
                'success': False,
                'error': str(e),
                'timestamp': datetime.now().isoformat()
            }
    
    def _analyze_participation(self, df) -> Dict:
        """Analisa padrões de participação"""
        total_voters = len(df)
        voted = df.get('votou', pd.Series([1] * total_voters)).sum()
        participation_rate = (voted / total_voters) * 100 if total_voters > 0 else 0
        
        return {
            'total_voters': total_voters,
            'voted': voted,
            'participation_rate': participation_rate,
            'participation_level': self._classify_participation(participation_rate)
        }
    
    def _analyze_vote_distribution(self, df) -> Dict:
        """Analisa distribuição de votos"""
        if 'candidate_id' in df.columns:
            vote_counts = df['candidate_id'].value_counts()
            total_votes = vote_counts.sum()
            
            return {
                'candidate_votes': vote_counts.to_dict(),
                'total_votes': total_votes,
                'winner': vote_counts.index[0] if len(vote_counts) > 0 else None,
                'winner_percentage': (vote_counts.iloc[0] / total_votes * 100) if total_votes > 0 else 0
            }
        else:
            return {'error': 'Dados de candidatos não disponíveis'}
    
    def _analyze_temporal_patterns(self, df) -> Dict:
        """Analisa padrões temporais"""
        if 'timestamp' in df.columns:
            df['timestamp'] = pd.to_datetime(df['timestamp'])
            df['hour'] = df['timestamp'].dt.hour
            df['day_of_week'] = df['timestamp'].dt.day_name()
            
            return {
                'votes_by_hour': df['hour'].value_counts().sort_index().to_dict(),
                'votes_by_day': df['day_of_week'].value_counts().to_dict(),
                'peak_hour': df['hour'].mode().iloc[0] if len(df) > 0 else None,
                'peak_day': df['day_of_week'].mode().iloc[0] if len(df) > 0 else None
            }
        else:
            return {'error': 'Dados temporais não disponíveis'}
    
    async def _detect_election_anomalies(self, df) -> List[Dict]:
        """Detecta anomalias na eleição"""
        anomalies = []
        
        # Anomalia: Votos em horário suspeito
        if 'timestamp' in df.columns:
            df['timestamp'] = pd.to_datetime(df['timestamp'])
            night_votes = df[df['timestamp'].dt.hour.between(0, 5)]
            if len(night_votes) > 0:
                anomalies.append({
                    'type': 'night_voting',
                    'count': len(night_votes),
                    'description': 'Votos registrados durante madrugada'
                })
        
        # Anomalia: Muitos votos do mesmo eleitor
        if 'voter_id' in df.columns:
            voter_counts = df['voter_id'].value_counts()
            duplicate_votes = voter_counts[voter_counts > 1]
            if len(duplicate_votes) > 0:
                anomalies.append({
                    'type': 'duplicate_votes',
                    'count': len(duplicate_votes),
                    'description': 'Eleitores com múltiplos votos'
                })
        
        return anomalies
    
    def _classify_participation(self, rate: float) -> str:
        """Classifica nível de participação"""
        if rate >= 80:
            return "Muito Alta"
        elif rate >= 70:
            return "Alta"
        elif rate >= 60:
            return "Média"
        elif rate >= 50:
            return "Baixa"
        else:
            return "Muito Baixa"
    
    def get_service_status(self) -> Dict:
        """Retorna status do serviço"""
        return {
            'initialized': self.is_initialized,
            'usage_stats': self.usage_stats,
            'models_loaded': {
                'face_recognition': self.face_model is not None,
                'fraud_detection': self.fraud_model is not None,
                'vote_prediction': self.prediction_model is not None
            },
            'timestamp': datetime.now().isoformat()
        }
    
    def get_usage_statistics(self) -> Dict:
        """Retorna estatísticas de uso"""
        return {
            'statistics': self.usage_stats,
            'uptime': datetime.now().isoformat(),
            'health_score': self._calculate_health_score()
        }
    
    def _calculate_health_score(self) -> float:
        """Calcula score de saúde do serviço"""
        total_operations = sum(self.usage_stats.values()) - self.usage_stats['errors']
        if total_operations == 0:
            return 100.0
        
        error_rate = self.usage_stats['errors'] / total_operations
        health_score = max(0, (1 - error_rate) * 100)
        return round(health_score, 2)

# Instância global do serviço
ai_service = AIService()

async def get_ai_service() -> AIService:
    """Retorna instância do serviço de IA"""
    if not ai_service.is_initialized:
        await ai_service.initialize()
    return ai_service

def main():
    """Função principal para teste"""
    async def test_service():
        service = await get_ai_service()
        
        print("Serviço de IA FORTIS")
        print(f"Status: {service.get_service_status()}")
        print(f"Estatísticas: {service.get_usage_statistics()}")
    
    asyncio.run(test_service())

if __name__ == "__main__":
    main()
