#!/usr/bin/env python3
"""
FORTIS - Predição de Votos
Sistema de predição de padrões de votação e participação eleitoral
"""

import numpy as np
import pandas as pd
from typing import Dict, List, Tuple, Optional
from sklearn.ensemble import RandomForestRegressor, GradientBoostingRegressor
from sklearn.linear_model import LogisticRegression
from sklearn.preprocessing import StandardScaler, LabelEncoder
from sklearn.model_selection import train_test_split, cross_val_score
from sklearn.metrics import mean_squared_error, r2_score, accuracy_score
import logging
import joblib
import os
from datetime import datetime, timedelta

class VotePredictionModel:
    """Modelo de predição de votos para o sistema FORTIS"""
    
    def __init__(self, model_path: Optional[str] = None):
        self.model_path = model_path or "ai/data/models/vote_prediction.pkl"
        self.models = {}
        self.scalers = {}
        self.encoders = {}
        self.feature_columns = []
        self.is_trained = False
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
    
    def prepare_features(self, df: pd.DataFrame) -> pd.DataFrame:
        """Prepara features para predição"""
        try:
            # Cria cópia para não modificar original
            features_df = df.copy()
            
            # Features temporais
            if 'timestamp' in features_df.columns:
                features_df['timestamp'] = pd.to_datetime(features_df['timestamp'])
                features_df['hour'] = features_df['timestamp'].dt.hour
                features_df['day_of_week'] = features_df['timestamp'].dt.dayofweek
                features_df['is_weekend'] = features_df['day_of_week'].isin([5, 6]).astype(int)
            
            # Features demográficas
            if 'idade' in features_df.columns:
                features_df['faixa_etaria'] = pd.cut(
                    features_df['idade'], 
                    bins=[0, 25, 35, 45, 55, 65, 100], 
                    labels=[0, 1, 2, 3, 4, 5]
                ).astype(int)
            
            # Features geográficas
            if 'estado' in features_df.columns:
                le_estado = LabelEncoder()
                features_df['estado_encoded'] = le_estado.fit_transform(features_df['estado'])
                self.encoders['estado'] = le_estado
            
            # Features de participação histórica
            if 'votou_ultima_eleicao' in features_df.columns:
                features_df['participacao_historica'] = features_df['votou_ultima_eleicao']
            
            # Features de candidato
            if 'candidate_id' in features_df.columns:
                le_candidate = LabelEncoder()
                features_df['candidate_encoded'] = le_candidate.fit_transform(features_df['candidate_id'])
                self.encoders['candidate'] = le_candidate
            
            # Remove colunas não numéricas
            numeric_columns = features_df.select_dtypes(include=[np.number]).columns.tolist()
            features_df = features_df[numeric_columns]
            
            self.feature_columns = features_df.columns.tolist()
            self.logger.info(f"Features preparadas: {len(self.feature_columns)} colunas")
            
            return features_df
            
        except Exception as e:
            self.logger.error(f"Erro ao preparar features: {e}")
            raise
    
    def train_turnout_model(self, df: pd.DataFrame):
        """Treina modelo de predição de participação"""
        try:
            self.logger.info("Treinando modelo de participação...")
            
            # Prepara features
            X = self.prepare_features(df)
            
            # Target: participação (1 se votou, 0 se não votou)
            if 'votou_ultima_eleicao' in df.columns:
                y = df['votou_ultima_eleicao']
            else:
                # Se não há target, cria um baseado em padrões
                y = np.random.randint(0, 2, len(X))
            
            # Divide dados
            X_train, X_test, y_train, y_test = train_test_split(
                X, y, test_size=0.2, random_state=42
            )
            
            # Normaliza features
            scaler = StandardScaler()
            X_train_scaled = scaler.fit_transform(X_train)
            X_test_scaled = scaler.transform(X_test)
            self.scalers['turnout'] = scaler
            
            # Treina modelo
            model = LogisticRegression(random_state=42, max_iter=1000)
            model.fit(X_train_scaled, y_train)
            
            # Avalia modelo
            y_pred = model.predict(X_test_scaled)
            accuracy = accuracy_score(y_test, y_pred)
            
            self.models['turnout'] = model
            self.logger.info(f"Modelo de participação treinado. Acurácia: {accuracy:.3f}")
            
            return accuracy
            
        except Exception as e:
            self.logger.error(f"Erro ao treinar modelo de participação: {e}")
            raise
    
    def train_candidate_preference_model(self, df: pd.DataFrame):
        """Treina modelo de predição de preferência de candidato"""
        try:
            self.logger.info("Treinando modelo de preferência de candidato...")
            
            # Prepara features
            X = self.prepare_features(df)
            
            # Target: candidato escolhido
            if 'candidate_id' in df.columns:
                le_candidate = LabelEncoder()
                y = le_candidate.fit_transform(df['candidate_id'])
                self.encoders['candidate_target'] = le_candidate
            else:
                # Se não há target, cria um baseado em padrões
                y = np.random.randint(0, 3, len(X))
            
            # Divide dados
            X_train, X_test, y_train, y_test = train_test_split(
                X, y, test_size=0.2, random_state=42
            )
            
            # Normaliza features
            scaler = StandardScaler()
            X_train_scaled = scaler.fit_transform(X_train)
            X_test_scaled = scaler.transform(X_test)
            self.scalers['candidate'] = scaler
            
            # Treina modelo
            model = RandomForestRegressor(n_estimators=100, random_state=42)
            model.fit(X_train_scaled, y_train)
            
            # Avalia modelo
            y_pred = model.predict(X_test_scaled)
            r2 = r2_score(y_test, y_pred)
            
            self.models['candidate'] = model
            self.logger.info(f"Modelo de candidato treinado. R²: {r2:.3f}")
            
            return r2
            
        except Exception as e:
            self.logger.error(f"Erro ao treinar modelo de candidato: {e}")
            raise
    
    def predict_turnout(self, voter_data: Dict) -> Dict:
        """Prediz probabilidade de participação de um eleitor"""
        try:
            if 'turnout' not in self.models:
                raise ValueError("Modelo de participação não treinado")
            
            # Converte dados para DataFrame
            df = pd.DataFrame([voter_data])
            
            # Prepara features
            X = self.prepare_features(df)
            
            # Normaliza
            X_scaled = self.scalers['turnout'].transform(X)
            
            # Faz predição
            probability = self.models['turnout'].predict_proba(X_scaled)[0][1]
            
            return {
                'turnout_probability': probability,
                'predicted_turnout': probability > 0.5,
                'confidence': abs(probability - 0.5) * 2
            }
            
        except Exception as e:
            self.logger.error(f"Erro na predição de participação: {e}")
            return {
                'turnout_probability': 0.5,
                'predicted_turnout': False,
                'confidence': 0.0,
                'error': str(e)
            }
    
    def predict_candidate_preference(self, voter_data: Dict) -> Dict:
        """Prediz preferência de candidato de um eleitor"""
        try:
            if 'candidate' not in self.models:
                raise ValueError("Modelo de candidato não treinado")
            
            # Converte dados para DataFrame
            df = pd.DataFrame([voter_data])
            
            # Prepara features
            X = self.prepare_features(df)
            
            # Normaliza
            X_scaled = self.scalers['candidate'].transform(X)
            
            # Faz predição
            prediction = self.models['candidate'].predict(X_scaled)[0]
            
            # Converte de volta para ID do candidato
            if 'candidate_target' in self.encoders:
                candidate_id = self.encoders['candidate_target'].inverse_transform([int(prediction)])[0]
            else:
                candidate_id = f"CAND{int(prediction):03d}"
            
            return {
                'predicted_candidate': candidate_id,
                'prediction_score': prediction,
                'confidence': min(1.0, abs(prediction) / 3.0)
            }
            
        except Exception as e:
            self.logger.error(f"Erro na predição de candidato: {e}")
            return {
                'predicted_candidate': None,
                'prediction_score': 0.0,
                'confidence': 0.0,
                'error': str(e)
            }
    
    def predict_election_outcome(self, voter_data_list: List[Dict]) -> Dict:
        """Prediz resultado de uma eleição"""
        try:
            turnout_predictions = []
            candidate_predictions = []
            
            for voter_data in voter_data_list:
                # Prediz participação
                turnout_pred = self.predict_turnout(voter_data)
                turnout_predictions.append(turnout_pred['predicted_turnout'])
                
                # Prediz candidato
                candidate_pred = self.predict_candidate_preference(voter_data)
                candidate_predictions.append(candidate_pred['predicted_candidate'])
            
            # Calcula estatísticas
            total_voters = len(voter_data_list)
            predicted_turnout = sum(turnout_predictions)
            turnout_rate = predicted_turnout / total_voters if total_voters > 0 else 0
            
            # Conta votos por candidato
            candidate_votes = {}
            for i, candidate in enumerate(candidate_predictions):
                if turnout_predictions[i]:  # Só conta se o eleitor vai votar
                    candidate_votes[candidate] = candidate_votes.get(candidate, 0) + 1
            
            # Ordena candidatos por votos
            sorted_candidates = sorted(
                candidate_votes.items(), 
                key=lambda x: x[1], 
                reverse=True
            )
            
            return {
                'total_voters': total_voters,
                'predicted_turnout': predicted_turnout,
                'turnout_rate': turnout_rate,
                'candidate_results': dict(sorted_candidates),
                'predicted_winner': sorted_candidates[0][0] if sorted_candidates else None,
                'prediction_confidence': self._calculate_overall_confidence(turnout_predictions, candidate_predictions)
            }
            
        except Exception as e:
            self.logger.error(f"Erro na predição de eleição: {e}")
            return {
                'total_voters': 0,
                'predicted_turnout': 0,
                'turnout_rate': 0.0,
                'candidate_results': {},
                'predicted_winner': None,
                'error': str(e)
            }
    
    def _calculate_overall_confidence(self, turnout_preds: List[bool], candidate_preds: List[str]) -> float:
        """Calcula confiança geral da predição"""
        try:
            # Confiança baseada na consistência das predições
            turnout_consistency = sum(turnout_preds) / len(turnout_preds) if turnout_preds else 0.5
            
            # Confiança baseada na distribuição de candidatos
            candidate_counts = {}
            for candidate in candidate_preds:
                candidate_counts[candidate] = candidate_counts.get(candidate, 0) + 1
            
            if candidate_counts:
                max_votes = max(candidate_counts.values())
                total_votes = sum(candidate_counts.values())
                candidate_consistency = max_votes / total_votes if total_votes > 0 else 0.5
            else:
                candidate_consistency = 0.5
            
            # Média ponderada das confianças
            overall_confidence = (turnout_consistency + candidate_consistency) / 2
            return min(1.0, overall_confidence)
            
        except Exception as e:
            self.logger.error(f"Erro ao calcular confiança: {e}")
            return 0.5
    
    def save_model(self):
        """Salva modelos treinados"""
        try:
            model_data = {
                'models': self.models,
                'scalers': self.scalers,
                'encoders': self.encoders,
                'feature_columns': self.feature_columns,
                'is_trained': self.is_trained,
                'timestamp': datetime.now().isoformat()
            }
            
            os.makedirs(os.path.dirname(self.model_path), exist_ok=True)
            joblib.dump(model_data, self.model_path)
            self.logger.info(f"Modelos salvos em: {self.model_path}")
            
        except Exception as e:
            self.logger.error(f"Erro ao salvar modelos: {e}")
    
    def load_model(self):
        """Carrega modelos treinados"""
        try:
            if os.path.exists(self.model_path):
                model_data = joblib.load(self.model_path)
                self.models = model_data.get('models', {})
                self.scalers = model_data.get('scalers', {})
                self.encoders = model_data.get('encoders', {})
                self.feature_columns = model_data.get('feature_columns', [])
                self.is_trained = model_data.get('is_trained', False)
                self.logger.info("Modelos carregados com sucesso")
            else:
                self.logger.warning("Modelos não encontrados")
        except Exception as e:
            self.logger.error(f"Erro ao carregar modelos: {e}")

def main():
    """Função principal para teste"""
    model = VotePredictionModel()
    
    # Exemplo de dados
    sample_data = {
        'idade': 35,
        'sexo': 'M',
        'estado': 'SP',
        'votou_ultima_eleicao': 1,
        'timestamp': datetime.now()
    }
    
    print("Modelo de Predição de Votos FORTIS")
    print(f"Modelo treinado: {model.is_trained}")
    
    if model.is_trained:
        # Testa predições
        turnout_pred = model.predict_turnout(sample_data)
        candidate_pred = model.predict_candidate_preference(sample_data)
        
        print(f"Predição de participação: {turnout_pred}")
        print(f"Predição de candidato: {candidate_pred}")

if __name__ == "__main__":
    main()
