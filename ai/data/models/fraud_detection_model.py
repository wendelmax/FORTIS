# FORTIS AI - Modelo de Detecção de Fraude
# Modelo de machine learning para detectar tentativas de fraude eleitoral

import joblib
import numpy as np
import pandas as pd
from sklearn.ensemble import IsolationForest
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import classification_report, confusion_matrix
import logging
from typing import Dict, List, Tuple

class FraudDetectionModel:
    """Modelo para detecção de fraudes eleitorais"""
    
    def __init__(self):
        self.model = IsolationForest(
            contamination=0.1,  # 10% de outliers esperados
            random_state=42
        )
        self.scaler = StandardScaler()
        self.is_trained = False
        self.threshold = 0.5
        
    def prepare_features(self, vote_data):
        """Prepara features para detecção de fraude"""
        features = []
        
        # Features temporais
        features.append(vote_data.get('time_since_last_vote', 0))
        features.append(vote_data.get('voting_hour', 12))
        features.append(vote_data.get('day_of_week', 1))
        
        # Features de localização
        features.append(vote_data.get('location_anomaly_score', 0))
        features.append(vote_data.get('ip_reputation_score', 1))
        features.append(vote_data.get('geolocation_consistency', 1))
        
        # Features de dispositivo
        features.append(vote_data.get('device_fingerprint_risk', 0))
        features.append(vote_data.get('browser_anomaly_score', 0))
        features.append(vote_data.get('screen_resolution_risk', 0))
        
        # Features de comportamento
        features.append(vote_data.get('click_pattern_anomaly', 0))
        features.append(vote_data.get('typing_speed_anomaly', 0))
        features.append(vote_data.get('mouse_movement_anomaly', 0))
        
        # Features de rede
        features.append(vote_data.get('network_anomaly_score', 0))
        features.append(vote_data.get('proxy_detection_score', 0))
        features.append(vote_data.get('vpn_detection_score', 0))
        
        return np.array(features).reshape(1, -1)
    
    def train(self, X, y=None):
        """Treina o modelo de detecção de anomalias"""
        try:
            # Normalizar features
            X_scaled = self.scaler.fit_transform(X)
            
            # Treinar modelo
            self.model.fit(X_scaled)
            
            # Calcular threshold baseado nos scores
            scores = self.model.decision_function(X_scaled)
            self.threshold = np.percentile(scores, 10)  # 10% mais suspeitos
            
            logging.info(f"Modelo de detecção de fraude treinado")
            logging.info(f"Threshold definido em: {self.threshold:.4f}")
            
            self.is_trained = True
            return True
            
        except Exception as e:
            logging.error(f"Erro ao treinar modelo de fraude: {e}")
            raise
    
    def predict(self, vote_data):
        """Prediz se o voto é suspeito de fraude"""
        if not self.is_trained:
            raise ValueError("Modelo não foi treinado ainda")
        
        features = self.prepare_features(vote_data)
        features_scaled = self.scaler.transform(features)
        
        # Obter score de anomalia
        anomaly_score = self.model.decision_function(features_scaled)[0]
        
        # Determinar se é fraude
        is_fraud = anomaly_score < self.threshold
        fraud_probability = max(0, min(1, (self.threshold - anomaly_score) / self.threshold))
        
        return {
            'is_fraud': bool(is_fraud),
            'fraud_probability': float(fraud_probability),
            'anomaly_score': float(anomaly_score),
            'risk_level': self._get_risk_level(fraud_probability)
        }
    
    def _get_risk_level(self, probability):
        """Determina o nível de risco baseado na probabilidade"""
        if probability < 0.3:
            return 'LOW'
        elif probability < 0.7:
            return 'MEDIUM'
        else:
            return 'HIGH'
    
    def batch_predict(self, votes_data):
        """Prediz fraude para múltiplos votos"""
        results = []
        
        for vote_data in votes_data:
            try:
                result = self.predict(vote_data)
                results.append(result)
            except Exception as e:
                logging.error(f"Erro ao processar voto: {e}")
                results.append({
                    'is_fraud': False,
                    'fraud_probability': 0.0,
                    'anomaly_score': 0.0,
                    'risk_level': 'LOW',
                    'error': str(e)
                })
        
        return results
    
    def get_fraud_indicators(self, vote_data):
        """Retorna indicadores específicos de fraude"""
        features = self.prepare_features(vote_data)
        indicators = {}
        
        # Verificar indicadores específicos
        indicators['suspicious_timing'] = features[0][0] < 60  # Menos de 1 minuto entre votos
        indicators['unusual_location'] = features[0][3] > 0.7  # Score alto de anomalia de localização
        indicators['device_risk'] = features[0][6] > 0.5  # Dispositivo suspeito
        indicators['network_risk'] = features[0][14] > 0.5  # Rede suspeita
        indicators['behavior_anomaly'] = any(features[0][9:12] > 0.5)  # Comportamento anômalo
        
        return indicators
    
    def save_model(self, filepath):
        """Salva o modelo treinado"""
        model_data = {
            'model': self.model,
            'scaler': self.scaler,
            'threshold': self.threshold,
            'is_trained': self.is_trained
        }
        joblib.dump(model_data, filepath)
        logging.info(f"Modelo de fraude salvo em: {filepath}")
    
    def load_model(self, filepath):
        """Carrega modelo salvo"""
        model_data = joblib.load(filepath)
        self.model = model_data['model']
        self.scaler = model_data['scaler']
        self.threshold = model_data['threshold']
        self.is_trained = model_data['is_trained']
        logging.info(f"Modelo de fraude carregado de: {filepath}")

# Exemplo de uso
if __name__ == "__main__":
    # Configurar logging
    logging.basicConfig(level=logging.INFO)
    
    # Criar modelo
    model = FraudDetectionModel()
    
    # Dados de exemplo para treinamento
    np.random.seed(42)
    n_samples = 1000
    
    # Gerar dados normais (não fraudulentos)
    X_normal = np.random.rand(n_samples, 16)
    
    # Adicionar algumas anomalias
    X_anomalies = np.random.rand(100, 16) * 2 + 1  # Valores mais extremos
    X = np.vstack([X_normal, X_anomalies])
    
    # Treinar modelo
    model.train(X)
    
    # Salvar modelo
    model.save_model('ai/data/models/fraud_detection_model.pkl')
    
    # Testar predição
    test_data = {
        'time_since_last_vote': 30,  # 30 segundos (suspeito)
        'voting_hour': 3,  # 3h da manhã (suspeito)
        'day_of_week': 1,
        'location_anomaly_score': 0.8,  # Alto (suspeito)
        'ip_reputation_score': 0.2,  # Baixo (suspeito)
        'geolocation_consistency': 0.3,  # Baixo (suspeito)
        'device_fingerprint_risk': 0.7,  # Alto (suspeito)
        'browser_anomaly_score': 0.6,  # Alto (suspeito)
        'screen_resolution_risk': 0.4,
        'click_pattern_anomaly': 0.8,  # Alto (suspeito)
        'typing_speed_anomaly': 0.3,
        'mouse_movement_anomaly': 0.7,  # Alto (suspeito)
        'network_anomaly_score': 0.9,  # Muito alto (suspeito)
        'proxy_detection_score': 0.8,  # Alto (suspeito)
        'vpn_detection_score': 0.6,  # Alto (suspeito)
    }
    
    result = model.predict(test_data)
    print(f"Resultado da detecção de fraude: {result}")
    
    indicators = model.get_fraud_indicators(test_data)
    print(f"Indicadores de fraude: {indicators}")
