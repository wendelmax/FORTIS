#!/usr/bin/env python3
"""
FORTIS Analytics - Modelo de Análise de Participação
Modelo para análise e predição de participação eleitoral
"""

import pandas as pd
import numpy as np
from typing import Dict, List, Tuple, Optional
import logging
from datetime import datetime, timedelta
from sklearn.ensemble import RandomForestRegressor
from sklearn.linear_model import LinearRegression
from sklearn.preprocessing import StandardScaler
from sklearn.model_selection import train_test_split
from sklearn.metrics import mean_squared_error, r2_score
import joblib
import json
import os

class ParticipationModel:
    """Modelo para análise e predição de participação eleitoral"""
    
    def __init__(self, model_path: str = "analytics/models/participation_model.pkl"):
        self.model_path = model_path
        self.model = None
        self.scaler = StandardScaler()
        self.feature_columns = []
        self.is_trained = False
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Estatísticas do modelo
        self.model_stats = {
            'training_samples': 0,
            'test_accuracy': 0.0,
            'last_trained': None,
            'feature_importance': {}
        }
    
    def prepare_features(self, data: pd.DataFrame) -> pd.DataFrame:
        """Prepara features para o modelo"""
        try:
            df = data.copy()
            
            # Features demográficas
            if 'population' in df.columns:
                df['population_density'] = df['votes'] / df['population']
            
            # Features temporais
            if 'election_date' in df.columns:
                df['election_date'] = pd.to_datetime(df['election_date'])
                df['day_of_week'] = df['election_date'].dt.dayofweek
                df['is_weekend'] = df['day_of_week'].isin([5, 6]).astype(int)
                df['month'] = df['election_date'].dt.month
                df['is_election_season'] = df['month'].isin([10, 11]).astype(int)
            
            # Features geográficas
            if 'state' in df.columns:
                # Codifica estados
                state_mapping = {
                    'SP': 1, 'RJ': 2, 'MG': 3, 'RS': 4, 'PR': 5,
                    'SC': 6, 'BA': 7, 'GO': 8, 'PE': 9, 'CE': 10
                }
                df['state_encoded'] = df['state'].map(state_mapping).fillna(0)
            
            # Features históricas
            if 'previous_participation' in df.columns:
                df['participation_trend'] = df['previous_participation'] - df['participation_rate']
            else:
                df['participation_trend'] = 0
            
            # Features de competição
            if 'candidates_count' in df.columns:
                df['competition_level'] = df['candidates_count'] / df['candidates_count'].max()
            else:
                df['competition_level'] = 1.0
            
            # Seleciona features numéricas
            numeric_columns = df.select_dtypes(include=[np.number]).columns.tolist()
            self.feature_columns = [col for col in numeric_columns if col != 'participation_rate']
            
            return df[self.feature_columns]
            
        except Exception as e:
            self.logger.error(f"Erro ao preparar features: {e}")
            return pd.DataFrame()
    
    def train_model(self, data: pd.DataFrame) -> Dict[str, float]:
        """Treina o modelo de participação"""
        try:
            self.logger.info("Iniciando treinamento do modelo de participação...")
            
            # Prepara features
            X = self.prepare_features(data)
            y = data['participation_rate']
            
            if X.empty or len(y) == 0:
                raise ValueError("Dados insuficientes para treinamento")
            
            # Divide dados
            X_train, X_test, y_train, y_test = train_test_split(
                X, y, test_size=0.2, random_state=42
            )
            
            # Normaliza features
            X_train_scaled = self.scaler.fit_transform(X_train)
            X_test_scaled = self.scaler.transform(X_test)
            
            # Treina modelo Random Forest
            self.model = RandomForestRegressor(
                n_estimators=100,
                max_depth=10,
                random_state=42,
                n_jobs=-1
            )
            
            self.model.fit(X_train_scaled, y_train)
            
            # Avalia modelo
            y_pred = self.model.predict(X_test_scaled)
            mse = mean_squared_error(y_test, y_pred)
            r2 = r2_score(y_test, y_pred)
            
            # Atualiza estatísticas
            self.model_stats['training_samples'] = len(X_train)
            self.model_stats['test_accuracy'] = r2
            self.model_stats['last_trained'] = datetime.now().isoformat()
            self.model_stats['feature_importance'] = dict(
                zip(self.feature_columns, self.model.feature_importances_)
            )
            
            self.is_trained = True
            
            # Salva modelo
            self.save_model()
            
            self.logger.info(f"Modelo treinado com sucesso. R²: {r2:.3f}")
            
            return {
                'mse': mse,
                'r2': r2,
                'training_samples': len(X_train),
                'test_samples': len(X_test)
            }
            
        except Exception as e:
            self.logger.error(f"Erro no treinamento: {e}")
            raise
    
    def predict_participation(self, data: Dict) -> Dict[str, float]:
        """Prediz taxa de participação"""
        try:
            if not self.is_trained:
                raise ValueError("Modelo não foi treinado")
            
            # Converte dados para DataFrame
            df = pd.DataFrame([data])
            
            # Prepara features
            X = self.prepare_features(df)
            
            if X.empty:
                raise ValueError("Features não puderam ser preparadas")
            
            # Normaliza features
            X_scaled = self.scaler.transform(X)
            
            # Faz predição
            prediction = self.model.predict(X_scaled)[0]
            
            # Calcula intervalo de confiança (simplificado)
            confidence_interval = self._calculate_confidence_interval(X_scaled)
            
            return {
                'predicted_participation': round(prediction, 2),
                'confidence_lower': round(prediction - confidence_interval, 2),
                'confidence_upper': round(prediction + confidence_interval, 2),
                'confidence_level': 0.95
            }
            
        except Exception as e:
            self.logger.error(f"Erro na predição: {e}")
            return {
                'predicted_participation': 0.0,
                'error': str(e)
            }
    
    def _calculate_confidence_interval(self, X_scaled: np.ndarray) -> float:
        """Calcula intervalo de confiança para predição"""
        try:
            # Usa desvio padrão das predições do conjunto de treino
            # Em um modelo mais sofisticado, usaria bootstrap ou quantile regression
            return 5.0  # Intervalo fixo de ±5% para simplificação
        except:
            return 5.0
    
    def analyze_participation_factors(self, data: pd.DataFrame) -> Dict[str, any]:
        """Analisa fatores que influenciam a participação"""
        try:
            if not self.is_trained:
                raise ValueError("Modelo não foi treinado")
            
            # Prepara features
            X = self.prepare_features(data)
            
            if X.empty:
                return {'error': 'Dados insuficientes para análise'}
            
            # Obtém importância das features
            feature_importance = dict(
                zip(self.feature_columns, self.model.feature_importances_)
            )
            
            # Ordena por importância
            sorted_features = sorted(
                feature_importance.items(), 
                key=lambda x: x[1], 
                reverse=True
            )
            
            # Análise de correlação
            correlations = {}
            for feature in self.feature_columns:
                if feature in data.columns:
                    corr = data[feature].corr(data['participation_rate'])
                    correlations[feature] = round(corr, 3)
            
            return {
                'feature_importance': sorted_features,
                'correlations': correlations,
                'top_factors': sorted_features[:5],
                'analysis_timestamp': datetime.now().isoformat()
            }
            
        except Exception as e:
            self.logger.error(f"Erro na análise de fatores: {e}")
            return {'error': str(e)}
    
    def generate_participation_insights(self, data: pd.DataFrame) -> List[str]:
        """Gera insights sobre participação eleitoral"""
        try:
            insights = []
            
            # Análise de tendências
            if 'participation_rate' in data.columns:
                avg_participation = data['participation_rate'].mean()
                if avg_participation > 80:
                    insights.append(f"Alta participação média: {avg_participation:.1f}%")
                elif avg_participation < 70:
                    insights.append(f"Baixa participação média: {avg_participation:.1f}%")
                else:
                    insights.append(f"Participação moderada: {avg_participation:.1f}%")
            
            # Análise geográfica
            if 'state' in data.columns and 'participation_rate' in data.columns:
                state_participation = data.groupby('state')['participation_rate'].mean()
                best_state = state_participation.idxmax()
                worst_state = state_participation.idxmin()
                
                insights.append(f"Maior participação: {best_state} ({state_participation[best_state]:.1f}%)")
                insights.append(f"Menor participação: {worst_state} ({state_participation[worst_state]:.1f}%)")
            
            # Análise temporal
            if 'election_date' in data.columns and 'participation_rate' in data.columns:
                data['election_date'] = pd.to_datetime(data['election_date'])
                data['year'] = data['election_date'].dt.year
                yearly_participation = data.groupby('year')['participation_rate'].mean()
                
                if len(yearly_participation) > 1:
                    trend = yearly_participation.iloc[-1] - yearly_participation.iloc[0]
                    if trend > 0:
                        insights.append(f"Tendência crescente de participação: +{trend:.1f}%")
                    else:
                        insights.append(f"Tendência decrescente de participação: {trend:.1f}%")
            
            # Análise de competição
            if 'candidates_count' in data.columns and 'participation_rate' in data.columns:
                corr = data['candidates_count'].corr(data['participation_rate'])
                if corr > 0.3:
                    insights.append("Mais candidatos correlaciona com maior participação")
                elif corr < -0.3:
                    insights.append("Mais candidatos correlaciona com menor participação")
            
            return insights
            
        except Exception as e:
            self.logger.error(f"Erro na geração de insights: {e}")
            return [f"Erro na análise: {str(e)}"]
    
    def save_model(self):
        """Salva modelo treinado"""
        try:
            model_data = {
                'model': self.model,
                'scaler': self.scaler,
                'feature_columns': self.feature_columns,
                'is_trained': self.is_trained,
                'model_stats': self.model_stats
            }
            
            os.makedirs(os.path.dirname(self.model_path), exist_ok=True)
            joblib.dump(model_data, self.model_path)
            
            self.logger.info(f"Modelo salvo em: {self.model_path}")
            
        except Exception as e:
            self.logger.error(f"Erro ao salvar modelo: {e}")
    
    def load_model(self):
        """Carrega modelo treinado"""
        try:
            if os.path.exists(self.model_path):
                model_data = joblib.load(self.model_path)
                
                self.model = model_data['model']
                self.scaler = model_data['scaler']
                self.feature_columns = model_data['feature_columns']
                self.is_trained = model_data['is_trained']
                self.model_stats = model_data['model_stats']
                
                self.logger.info("Modelo carregado com sucesso")
            else:
                self.logger.warning("Modelo não encontrado")
                
        except Exception as e:
            self.logger.error(f"Erro ao carregar modelo: {e}")
    
    def get_model_info(self) -> Dict[str, any]:
        """Retorna informações do modelo"""
        return {
            'is_trained': self.is_trained,
            'model_stats': self.model_stats,
            'feature_columns': self.feature_columns,
            'model_path': self.model_path
        }

def main():
    """Função principal para teste"""
    model = ParticipationModel()
    
    # Dados de exemplo
    sample_data = pd.DataFrame({
        'state': ['SP', 'RJ', 'MG', 'RS', 'PR'],
        'population': [46000000, 17000000, 21000000, 11000000, 11000000],
        'votes': [500000, 300000, 250000, 200000, 180000],
        'participation_rate': [85.2, 78.5, 82.1, 79.3, 81.7],
        'election_date': ['2025-10-01', '2025-10-01', '2025-10-01', '2025-10-01', '2025-10-01'],
        'candidates_count': [4, 4, 4, 4, 4],
        'previous_participation': [84.0, 77.0, 81.0, 78.0, 80.0]
    })
    
    print("Modelo de Análise de Participação FORTIS")
    
    # Treina modelo
    results = model.train_model(sample_data)
    print(f"Resultados do treinamento: {results}")
    
    # Testa predição
    test_data = {
        'state': 'SC',
        'population': 7000000,
        'votes': 150000,
        'election_date': '2025-10-01',
        'candidates_count': 4,
        'previous_participation': 82.0
    }
    
    prediction = model.predict_participation(test_data)
    print(f"Predição: {prediction}")
    
    # Gera insights
    insights = model.generate_participation_insights(sample_data)
    print(f"Insights: {insights}")

if __name__ == "__main__":
    main()
