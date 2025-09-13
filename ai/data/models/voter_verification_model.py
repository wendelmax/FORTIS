# FORTIS AI - Modelo de Verificação de Eleitor
# Modelo de machine learning para verificar autenticidade de eleitores

import joblib
import numpy as np
import pandas as pd
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score, classification_report
from sklearn.preprocessing import StandardScaler
import logging

class VoterVerificationModel:
    """Modelo para verificação de autenticidade de eleitores"""
    
    def __init__(self):
        self.model = RandomForestClassifier(
            n_estimators=100,
            max_depth=10,
            random_state=42
        )
        self.scaler = StandardScaler()
        self.is_trained = False
        
    def prepare_features(self, voter_data):
        """Prepara features para o modelo"""
        features = []
        
        # Features de comportamento
        features.append(voter_data.get('login_frequency', 0))
        features.append(voter_data.get('session_duration', 0))
        features.append(voter_data.get('device_consistency', 1))
        features.append(voter_data.get('location_consistency', 1))
        
        # Features de biometria
        features.append(voter_data.get('biometric_confidence', 0))
        features.append(voter_data.get('face_match_score', 0))
        features.append(voter_data.get('fingerprint_score', 0))
        
        # Features de dispositivo
        features.append(voter_data.get('device_trust_score', 0))
        features.append(voter_data.get('app_version_score', 1))
        features.append(voter_data.get('os_security_score', 1))
        
        return np.array(features).reshape(1, -1)
    
    def train(self, X, y):
        """Treina o modelo"""
        try:
            # Normalizar features
            X_scaled = self.scaler.fit_transform(X)
            
            # Dividir dados
            X_train, X_test, y_train, y_test = train_test_split(
                X_scaled, y, test_size=0.2, random_state=42
            )
            
            # Treinar modelo
            self.model.fit(X_train, y_train)
            
            # Avaliar modelo
            y_pred = self.model.predict(X_test)
            accuracy = accuracy_score(y_test, y_pred)
            
            logging.info(f"Modelo treinado com accuracy: {accuracy:.4f}")
            logging.info(f"Relatório de classificação:\n{classification_report(y_test, y_pred)}")
            
            self.is_trained = True
            return accuracy
            
        except Exception as e:
            logging.error(f"Erro ao treinar modelo: {e}")
            raise
    
    def predict(self, voter_data):
        """Prediz se o eleitor é autêntico"""
        if not self.is_trained:
            raise ValueError("Modelo não foi treinado ainda")
        
        features = self.prepare_features(voter_data)
        features_scaled = self.scaler.transform(features)
        
        prediction = self.model.predict(features_scaled)[0]
        probability = self.model.predict_proba(features_scaled)[0]
        
        return {
            'is_authentic': bool(prediction),
            'confidence': float(max(probability)),
            'risk_score': float(1 - max(probability))
        }
    
    def save_model(self, filepath):
        """Salva o modelo treinado"""
        model_data = {
            'model': self.model,
            'scaler': self.scaler,
            'is_trained': self.is_trained
        }
        joblib.dump(model_data, filepath)
        logging.info(f"Modelo salvo em: {filepath}")
    
    def load_model(self, filepath):
        """Carrega modelo salvo"""
        model_data = joblib.load(filepath)
        self.model = model_data['model']
        self.scaler = model_data['scaler']
        self.is_trained = model_data['is_trained']
        logging.info(f"Modelo carregado de: {filepath}")

# Exemplo de uso
if __name__ == "__main__":
    # Configurar logging
    logging.basicConfig(level=logging.INFO)
    
    # Criar modelo
    model = VoterVerificationModel()
    
    # Dados de exemplo para treinamento
    np.random.seed(42)
    n_samples = 1000
    
    X = np.random.rand(n_samples, 10)  # 10 features
    y = np.random.randint(0, 2, n_samples)  # 0 = fake, 1 = authentic
    
    # Treinar modelo
    accuracy = model.train(X, y)
    print(f"Accuracy: {accuracy:.4f}")
    
    # Salvar modelo
    model.save_model('ai/data/models/voter_verification_model.pkl')
    
    # Testar predição
    test_data = {
        'login_frequency': 0.8,
        'session_duration': 300,
        'device_consistency': 1,
        'location_consistency': 1,
        'biometric_confidence': 0.95,
        'face_match_score': 0.92,
        'fingerprint_score': 0.88,
        'device_trust_score': 0.9,
        'app_version_score': 1,
        'os_security_score': 1
    }
    
    result = model.predict(test_data)
    print(f"Resultado da predição: {result}")
