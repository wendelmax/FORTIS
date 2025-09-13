#!/usr/bin/env python3
"""
FORTIS Analytics - Análise de Segurança
Modelo para análise de segurança e detecção de anomalias eleitorais
"""

import pandas as pd
import numpy as np
from typing import Dict, List, Tuple, Optional
import logging
from datetime import datetime, timedelta
from sklearn.ensemble import IsolationForest
from sklearn.cluster import DBSCAN
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import silhouette_score
import joblib
import json
import os

class SecurityAnalysisModel:
    """Modelo para análise de segurança eleitoral"""
    
    def __init__(self, model_path: str = "analytics/models/security_model.pkl"):
        self.model_path = model_path
        self.anomaly_detector = None
        self.clusterer = None
        self.scaler = StandardScaler()
        self.feature_columns = []
        self.is_trained = False
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Estatísticas do modelo
        self.model_stats = {
            'training_samples': 0,
            'anomalies_detected': 0,
            'clusters_found': 0,
            'last_trained': None,
            'security_score': 0.0
        }
    
    def prepare_security_features(self, data: pd.DataFrame) -> pd.DataFrame:
        """Prepara features para análise de segurança"""
        try:
            df = data.copy()
            
            # Features temporais
            if 'timestamp' in df.columns:
                df['timestamp'] = pd.to_datetime(df['timestamp'])
                df['hour'] = df['timestamp'].dt.hour
                df['day_of_week'] = df['timestamp'].dt.dayofweek
                df['is_weekend'] = df['day_of_week'].isin([5, 6]).astype(int)
                df['is_night'] = df['hour'].between(0, 5).astype(int)
            
            # Features de votação
            if 'votes_per_hour' in df.columns:
                df['votes_per_hour_normalized'] = (df['votes_per_hour'] - df['votes_per_hour'].mean()) / df['votes_per_hour'].std()
            
            # Features geográficas
            if 'zone_id' in df.columns and 'section_id' in df.columns:
                df['zone_section'] = df['zone_id'].astype(str) + '_' + df['section_id'].astype(str)
                df['votes_per_zone'] = df.groupby('zone_id')['vote_id'].transform('count')
                df['votes_per_section'] = df.groupby('section_id')['vote_id'].transform('count')
            
            # Features de verificação
            if 'is_verified' in df.columns:
                df['verification_rate'] = df['is_verified'].mean()
            else:
                df['verification_rate'] = 1.0
            
            if 'is_audited' in df.columns:
                df['audit_rate'] = df['is_audited'].mean()
            else:
                df['audit_rate'] = 1.0
            
            # Features de duplicação
            if 'voter_cpf' in df.columns:
                df['voter_vote_count'] = df.groupby('voter_cpf')['vote_id'].transform('count')
                df['has_duplicate_votes'] = (df['voter_vote_count'] > 1).astype(int)
            else:
                df['voter_vote_count'] = 1
                df['has_duplicate_votes'] = 0
            
            # Features de candidato
            if 'candidate_id' in df.columns:
                df['candidate_vote_count'] = df.groupby('candidate_id')['vote_id'].transform('count')
                df['candidate_dominance'] = df['candidate_vote_count'] / df['candidate_vote_count'].sum()
            else:
                df['candidate_vote_count'] = 1
                df['candidate_dominance'] = 1.0
            
            # Seleciona features numéricas
            numeric_columns = df.select_dtypes(include=[np.number]).columns.tolist()
            self.feature_columns = [col for col in numeric_columns if col not in ['vote_id', 'election_id']]
            
            return df[self.feature_columns]
            
        except Exception as e:
            self.logger.error(f"Erro ao preparar features de segurança: {e}")
            return pd.DataFrame()
    
    def train_anomaly_detector(self, data: pd.DataFrame) -> Dict[str, float]:
        """Treina detector de anomalias"""
        try:
            self.logger.info("Treinando detector de anomalias...")
            
            # Prepara features
            X = self.prepare_security_features(data)
            
            if X.empty:
                raise ValueError("Dados insuficientes para treinamento")
            
            # Remove valores NaN
            X = X.fillna(X.mean())
            
            # Normaliza features
            X_scaled = self.scaler.fit_transform(X)
            
            # Treina detector de anomalias
            self.anomaly_detector = IsolationForest(
                contamination=0.1,  # Espera 10% de anomalias
                random_state=42,
                n_jobs=-1
            )
            
            self.anomaly_detector.fit(X_scaled)
            
            # Detecta anomalias no conjunto de treino
            anomalies = self.anomaly_detector.predict(X_scaled)
            anomaly_count = np.sum(anomalies == -1)
            
            # Atualiza estatísticas
            self.model_stats['training_samples'] = len(X)
            self.model_stats['anomalies_detected'] = anomaly_count
            self.model_stats['last_trained'] = datetime.now().isoformat()
            
            self.logger.info(f"Detector de anomalias treinado. Anomalias detectadas: {anomaly_count}")
            
            return {
                'training_samples': len(X),
                'anomalies_detected': anomaly_count,
                'anomaly_rate': anomaly_count / len(X)
            }
            
        except Exception as e:
            self.logger.error(f"Erro no treinamento do detector de anomalias: {e}")
            raise
    
    def train_clusterer(self, data: pd.DataFrame) -> Dict[str, float]:
        """Treina clusterer para agrupamento de padrões"""
        try:
            self.logger.info("Treinando clusterer...")
            
            # Prepara features
            X = self.prepare_security_features(data)
            
            if X.empty:
                raise ValueError("Dados insuficientes para treinamento")
            
            # Remove valores NaN
            X = X.fillna(X.mean())
            
            # Normaliza features
            X_scaled = self.scaler.fit_transform(X)
            
            # Treina clusterer
            self.clusterer = DBSCAN(
                eps=0.5,
                min_samples=5,
                n_jobs=-1
            )
            
            clusters = self.clusterer.fit_predict(X_scaled)
            
            # Calcula métricas de clustering
            n_clusters = len(set(clusters)) - (1 if -1 in clusters else 0)
            n_noise = list(clusters).count(-1)
            
            if n_clusters > 1:
                silhouette_avg = silhouette_score(X_scaled, clusters)
            else:
                silhouette_avg = 0.0
            
            # Atualiza estatísticas
            self.model_stats['clusters_found'] = n_clusters
            self.model_stats['noise_points'] = n_noise
            
            self.logger.info(f"Clusterer treinado. Clusters: {n_clusters}, Silhouette: {silhouette_avg:.3f}")
            
            return {
                'n_clusters': n_clusters,
                'n_noise': n_noise,
                'silhouette_score': silhouette_avg
            }
            
        except Exception as e:
            self.logger.error(f"Erro no treinamento do clusterer: {e}")
            raise
    
    def detect_anomalies(self, data: pd.DataFrame) -> Dict[str, any]:
        """Detecta anomalias em dados de votação"""
        try:
            if not self.is_trained:
                raise ValueError("Modelo não foi treinado")
            
            # Prepara features
            X = self.prepare_security_features(data)
            
            if X.empty:
                return {'error': 'Dados insuficientes para análise'}
            
            # Remove valores NaN
            X = X.fillna(X.mean())
            
            # Normaliza features
            X_scaled = self.scaler.transform(X)
            
            # Detecta anomalias
            anomaly_scores = self.anomaly_detector.decision_function(X_scaled)
            anomaly_predictions = self.anomaly_detector.predict(X_scaled)
            
            # Identifica anomalias
            anomalies = data[anomaly_predictions == -1].copy()
            anomalies['anomaly_score'] = anomaly_scores[anomaly_predictions == -1]
            
            # Classifica tipos de anomalias
            anomaly_types = self._classify_anomaly_types(anomalies)
            
            return {
                'total_votes': len(data),
                'anomalies_detected': len(anomalies),
                'anomaly_rate': len(anomalies) / len(data),
                'anomaly_details': anomalies.to_dict('records'),
                'anomaly_types': anomaly_types,
                'security_score': self._calculate_security_score(len(anomalies), len(data))
            }
            
        except Exception as e:
            self.logger.error(f"Erro na detecção de anomalias: {e}")
            return {'error': str(e)}
    
    def _classify_anomaly_types(self, anomalies: pd.DataFrame) -> Dict[str, int]:
        """Classifica tipos de anomalias"""
        try:
            types = {
                'temporal_anomalies': 0,
                'geographic_anomalies': 0,
                'duplicate_votes': 0,
                'verification_anomalies': 0,
                'other_anomalies': 0
            }
            
            for _, row in anomalies.iterrows():
                # Anomalias temporais (votos em horários suspeitos)
                if 'is_night' in row and row['is_night'] == 1:
                    types['temporal_anomalies'] += 1
                
                # Anomalias geográficas (muitos votos em uma zona/seção)
                if 'votes_per_zone' in row and row['votes_per_zone'] > row['votes_per_zone'].quantile(0.95):
                    types['geographic_anomalies'] += 1
                
                # Votos duplicados
                if 'has_duplicate_votes' in row and row['has_duplicate_votes'] == 1:
                    types['duplicate_votes'] += 1
                
                # Anomalias de verificação
                if 'is_verified' in row and row['is_verified'] == False:
                    types['verification_anomalies'] += 1
                
                # Outras anomalias
                if not any([
                    'is_night' in row and row['is_night'] == 1,
                    'has_duplicate_votes' in row and row['has_duplicate_votes'] == 1,
                    'is_verified' in row and row['is_verified'] == False
                ]):
                    types['other_anomalies'] += 1
            
            return types
            
        except Exception as e:
            self.logger.error(f"Erro na classificação de anomalias: {e}")
            return {}
    
    def _calculate_security_score(self, anomalies: int, total_votes: int) -> float:
        """Calcula score de segurança"""
        try:
            if total_votes == 0:
                return 0.0
            
            anomaly_rate = anomalies / total_votes
            
            # Score baseado na taxa de anomalias
            if anomaly_rate < 0.01:  # < 1%
                return 95.0
            elif anomaly_rate < 0.05:  # < 5%
                return 85.0
            elif anomaly_rate < 0.10:  # < 10%
                return 70.0
            else:
                return 50.0
                
        except:
            return 0.0
    
    def analyze_voting_patterns(self, data: pd.DataFrame) -> Dict[str, any]:
        """Analisa padrões de votação"""
        try:
            patterns = {}
            
            # Análise temporal
            if 'timestamp' in data.columns:
                data['timestamp'] = pd.to_datetime(data['timestamp'])
                data['hour'] = data['timestamp'].dt.hour
                
                hourly_votes = data.groupby('hour').size()
                patterns['peak_hour'] = hourly_votes.idxmax()
                patterns['lowest_hour'] = hourly_votes.idxmin()
                patterns['voting_distribution'] = hourly_votes.to_dict()
            
            # Análise geográfica
            if 'zone_id' in data.columns:
                zone_votes = data.groupby('zone_id').size()
                patterns['most_active_zone'] = zone_votes.idxmax()
                patterns['least_active_zone'] = zone_votes.idxmin()
                patterns['zone_distribution'] = zone_votes.to_dict()
            
            # Análise de candidatos
            if 'candidate_id' in data.columns:
                candidate_votes = data.groupby('candidate_id').size()
                patterns['most_voted_candidate'] = candidate_votes.idxmax()
                patterns['least_voted_candidate'] = candidate_votes.idxmin()
                patterns['candidate_distribution'] = candidate_votes.to_dict()
            
            # Análise de verificação
            if 'is_verified' in data.columns:
                verification_rate = data['is_verified'].mean()
                patterns['verification_rate'] = verification_rate
                patterns['unverified_votes'] = len(data[data['is_verified'] == False])
            
            return patterns
            
        except Exception as e:
            self.logger.error(f"Erro na análise de padrões: {e}")
            return {'error': str(e)}
    
    def generate_security_report(self, data: pd.DataFrame) -> Dict[str, any]:
        """Gera relatório de segurança"""
        try:
            # Detecta anomalias
            anomaly_analysis = self.detect_anomalies(data)
            
            # Analisa padrões
            pattern_analysis = self.analyze_voting_patterns(data)
            
            # Calcula métricas gerais
            total_votes = len(data)
            security_score = anomaly_analysis.get('security_score', 0.0)
            
            # Gera recomendações
            recommendations = self._generate_security_recommendations(anomaly_analysis, pattern_analysis)
            
            return {
                'summary': {
                    'total_votes': total_votes,
                    'security_score': security_score,
                    'anomalies_detected': anomaly_analysis.get('anomalies_detected', 0),
                    'anomaly_rate': anomaly_analysis.get('anomaly_rate', 0.0)
                },
                'anomaly_analysis': anomaly_analysis,
                'pattern_analysis': pattern_analysis,
                'recommendations': recommendations,
                'generated_at': datetime.now().isoformat()
            }
            
        except Exception as e:
            self.logger.error(f"Erro na geração do relatório: {e}")
            return {'error': str(e)}
    
    def _generate_security_recommendations(self, anomaly_analysis: Dict, pattern_analysis: Dict) -> List[str]:
        """Gera recomendações de segurança"""
        recommendations = []
        
        try:
            # Recomendações baseadas em anomalias
            anomaly_rate = anomaly_analysis.get('anomaly_rate', 0.0)
            if anomaly_rate > 0.05:
                recommendations.append("Alta taxa de anomalias detectada - investigar imediatamente")
            elif anomaly_rate > 0.02:
                recommendations.append("Taxa moderada de anomalias - monitorar de perto")
            
            # Recomendações baseadas em padrões
            if 'verification_rate' in pattern_analysis:
                if pattern_analysis['verification_rate'] < 0.95:
                    recommendations.append("Taxa de verificação baixa - revisar processo de verificação")
            
            # Recomendações baseadas em tipos de anomalias
            anomaly_types = anomaly_analysis.get('anomaly_types', {})
            if anomaly_types.get('duplicate_votes', 0) > 0:
                recommendations.append("Votos duplicados detectados - implementar validação mais rigorosa")
            
            if anomaly_types.get('temporal_anomalies', 0) > 0:
                recommendations.append("Votos em horários suspeitos - revisar logs de acesso")
            
            if anomaly_types.get('geographic_anomalies', 0) > 0:
                recommendations.append("Concentração anômala de votos - investigar zonas específicas")
            
            if not recommendations:
                recommendations.append("Sistema funcionando dentro dos parâmetros normais de segurança")
            
            return recommendations
            
        except Exception as e:
            self.logger.error(f"Erro na geração de recomendações: {e}")
            return ["Erro na análise de recomendações"]
    
    def save_model(self):
        """Salva modelo treinado"""
        try:
            model_data = {
                'anomaly_detector': self.anomaly_detector,
                'clusterer': self.clusterer,
                'scaler': self.scaler,
                'feature_columns': self.feature_columns,
                'is_trained': self.is_trained,
                'model_stats': self.model_stats
            }
            
            os.makedirs(os.path.dirname(self.model_path), exist_ok=True)
            joblib.dump(model_data, self.model_path)
            
            self.logger.info(f"Modelo de segurança salvo em: {self.model_path}")
            
        except Exception as e:
            self.logger.error(f"Erro ao salvar modelo: {e}")
    
    def load_model(self):
        """Carrega modelo treinado"""
        try:
            if os.path.exists(self.model_path):
                model_data = joblib.load(self.model_path)
                
                self.anomaly_detector = model_data['anomaly_detector']
                self.clusterer = model_data['clusterer']
                self.scaler = model_data['scaler']
                self.feature_columns = model_data['feature_columns']
                self.is_trained = model_data['is_trained']
                self.model_stats = model_data['model_stats']
                
                self.logger.info("Modelo de segurança carregado com sucesso")
            else:
                self.logger.warning("Modelo de segurança não encontrado")
                
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
    model = SecurityAnalysisModel()
    
    # Dados de exemplo
    sample_data = pd.DataFrame({
        'vote_id': [f'vote_{i}' for i in range(1000)],
        'timestamp': pd.date_range('2025-10-01 08:00:00', periods=1000, freq='1min'),
        'zone_id': np.random.choice(['Z001', 'Z002', 'Z003', 'Z004'], 1000),
        'section_id': np.random.choice(['S001', 'S002', 'S003'], 1000),
        'voter_cpf': [f'{np.random.randint(10000000000, 99999999999)}' for _ in range(1000)],
        'candidate_id': np.random.choice(['C001', 'C002', 'C003', 'C004'], 1000),
        'is_verified': np.random.choice([True, False], 1000, p=[0.95, 0.05]),
        'is_audited': np.random.choice([True, False], 1000, p=[0.8, 0.2])
    })
    
    print("Modelo de Análise de Segurança FORTIS")
    
    # Treina modelo
    anomaly_results = model.train_anomaly_detector(sample_data)
    cluster_results = model.train_clusterer(sample_data)
    
    print(f"Resultados do detector de anomalias: {anomaly_results}")
    print(f"Resultados do clusterer: {cluster_results}")
    
    # Gera relatório
    report = model.generate_security_report(sample_data)
    print(f"Relatório de segurança: {report['summary']}")

if __name__ == "__main__":
    main()
