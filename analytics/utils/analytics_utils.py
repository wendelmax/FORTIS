#!/usr/bin/env python3
"""
FORTIS Analytics - Utilitários
Funções auxiliares para análise de dados eleitorais
"""

import pandas as pd
import numpy as np
from typing import Dict, List, Tuple, Optional, Any
import logging
from datetime import datetime, timedelta
import json
import hashlib
import os
from pathlib import Path

class AnalyticsUtils:
    """Utilitários para análise de dados eleitorais"""
    
    def __init__(self):
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
    
    def calculate_participation_metrics(self, data: pd.DataFrame) -> Dict[str, float]:
        """Calcula métricas de participação eleitoral"""
        try:
            metrics = {}
            
            if 'participation_rate' in data.columns:
                participation = data['participation_rate']
                metrics['mean_participation'] = float(participation.mean())
                metrics['median_participation'] = float(participation.median())
                metrics['std_participation'] = float(participation.std())
                metrics['min_participation'] = float(participation.min())
                metrics['max_participation'] = float(participation.max())
                metrics['participation_range'] = float(participation.max() - participation.min())
            
            if 'total_votes' in data.columns and 'unique_voters' in data.columns:
                total_votes = data['total_votes'].sum()
                unique_voters = data['unique_voters'].sum()
                metrics['total_votes'] = int(total_votes)
                metrics['unique_voters'] = int(unique_voters)
                metrics['vote_duplication_rate'] = float((total_votes - unique_voters) / unique_voters) if unique_voters > 0 else 0.0
            
            return metrics
            
        except Exception as e:
            self.logger.error(f"Erro no cálculo de métricas de participação: {e}")
            return {}
    
    def analyze_voting_patterns(self, data: pd.DataFrame) -> Dict[str, Any]:
        """Analisa padrões de votação"""
        try:
            patterns = {}
            
            # Análise temporal
            if 'timestamp' in data.columns:
                data['timestamp'] = pd.to_datetime(data['timestamp'])
                data['hour'] = data['timestamp'].dt.hour
                data['day_of_week'] = data['timestamp'].dt.dayofweek
                
                # Padrão por hora
                hourly_votes = data.groupby('hour').size()
                patterns['peak_hour'] = int(hourly_votes.idxmax())
                patterns['lowest_hour'] = int(hourly_votes.idxmin())
                patterns['hourly_distribution'] = hourly_votes.to_dict()
                
                # Padrão por dia da semana
                daily_votes = data.groupby('day_of_week').size()
                patterns['daily_distribution'] = daily_votes.to_dict()
            
            # Análise geográfica
            if 'state' in data.columns:
                state_votes = data.groupby('state').size()
                patterns['most_active_state'] = state_votes.idxmax()
                patterns['least_active_state'] = state_votes.idxmin()
                patterns['state_distribution'] = state_votes.to_dict()
            
            # Análise de candidatos
            if 'candidate_id' in data.columns:
                candidate_votes = data.groupby('candidate_id').size()
                patterns['most_voted_candidate'] = candidate_votes.idxmax()
                patterns['least_voted_candidate'] = candidate_votes.idxmin()
                patterns['candidate_distribution'] = candidate_votes.to_dict()
                
                # Índice de competitividade
                total_votes = candidate_votes.sum()
                percentages = candidate_votes / total_votes
                patterns['competitiveness_index'] = float(percentages.std())
            
            return patterns
            
        except Exception as e:
            self.logger.error(f"Erro na análise de padrões: {e}")
            return {}
    
    def detect_outliers(self, data: pd.DataFrame, column: str, method: str = 'iqr') -> List[int]:
        """Detecta outliers em uma coluna"""
        try:
            values = data[column].dropna()
            
            if method == 'iqr':
                Q1 = values.quantile(0.25)
                Q3 = values.quantile(0.75)
                IQR = Q3 - Q1
                lower_bound = Q1 - 1.5 * IQR
                upper_bound = Q3 + 1.5 * IQR
                outliers = data[(data[column] < lower_bound) | (data[column] > upper_bound)].index.tolist()
            
            elif method == 'zscore':
                z_scores = np.abs((values - values.mean()) / values.std())
                outliers = data[z_scores > 3].index.tolist()
            
            else:
                raise ValueError(f"Método não suportado: {method}")
            
            return outliers
            
        except Exception as e:
            self.logger.error(f"Erro na detecção de outliers: {e}")
            return []
    
    def calculate_correlation_matrix(self, data: pd.DataFrame, numeric_columns: Optional[List[str]] = None) -> pd.DataFrame:
        """Calcula matriz de correlação"""
        try:
            if numeric_columns is None:
                numeric_columns = data.select_dtypes(include=[np.number]).columns.tolist()
            
            correlation_matrix = data[numeric_columns].corr()
            return correlation_matrix
            
        except Exception as e:
            self.logger.error(f"Erro no cálculo de correlação: {e}")
            return pd.DataFrame()
    
    def generate_time_series_features(self, data: pd.DataFrame, time_column: str, value_column: str) -> pd.DataFrame:
        """Gera features de série temporal"""
        try:
            df = data.copy()
            df[time_column] = pd.to_datetime(df[time_column])
            df = df.sort_values(time_column)
            
            # Features temporais
            df['year'] = df[time_column].dt.year
            df['month'] = df[time_column].dt.month
            df['day'] = df[time_column].dt.day
            df['hour'] = df[time_column].dt.hour
            df['day_of_week'] = df[time_column].dt.dayofweek
            df['is_weekend'] = df['day_of_week'].isin([5, 6]).astype(int)
            
            # Features de tendência
            df['value_lag1'] = df[value_column].shift(1)
            df['value_lag7'] = df[value_column].shift(7)
            df['value_ma7'] = df[value_column].rolling(window=7).mean()
            df['value_ma30'] = df[value_column].rolling(window=30).mean()
            
            # Features de diferença
            df['value_diff'] = df[value_column].diff()
            df['value_pct_change'] = df[value_column].pct_change()
            
            return df
            
        except Exception as e:
            self.logger.error(f"Erro na geração de features temporais: {e}")
            return data
    
    def create_geographic_analysis(self, data: pd.DataFrame, state_column: str, value_column: str) -> Dict[str, Any]:
        """Cria análise geográfica"""
        try:
            analysis = {}
            
            # Agrupa por estado
            state_data = data.groupby(state_column)[value_column].agg(['sum', 'mean', 'count', 'std']).reset_index()
            
            # Calcula métricas
            analysis['total_states'] = len(state_data)
            analysis['total_value'] = state_data['sum'].sum()
            analysis['average_per_state'] = state_data['mean'].mean()
            
            # Estado com maior valor
            max_state = state_data.loc[state_data['sum'].idxmax()]
            analysis['top_state'] = {
                'state': max_state[state_column],
                'value': float(max_state['sum']),
                'percentage': float(max_state['sum'] / state_data['sum'].sum() * 100)
            }
            
            # Estado com menor valor
            min_state = state_data.loc[state_data['sum'].idxmin()]
            analysis['bottom_state'] = {
                'state': min_state[state_column],
                'value': float(min_state['sum']),
                'percentage': float(min_state['sum'] / state_data['sum'].sum() * 100)
            }
            
            # Distribuição
            analysis['distribution'] = state_data.set_index(state_column)['sum'].to_dict()
            
            # Concentração (índice de Gini simplificado)
            sorted_values = sorted(state_data['sum'])
            n = len(sorted_values)
            cumsum = np.cumsum(sorted_values)
            analysis['concentration_index'] = float((n + 1 - 2 * np.sum(cumsum) / cumsum[-1]) / n)
            
            return analysis
            
        except Exception as e:
            self.logger.error(f"Erro na análise geográfica: {e}")
            return {}
    
    def calculate_security_metrics(self, data: pd.DataFrame) -> Dict[str, float]:
        """Calcula métricas de segurança"""
        try:
            metrics = {}
            
            # Taxa de verificação
            if 'is_verified' in data.columns:
                verification_rate = data['is_verified'].mean()
                metrics['verification_rate'] = float(verification_rate)
                metrics['unverified_votes'] = int((data['is_verified'] == False).sum())
            
            # Taxa de auditoria
            if 'is_audited' in data.columns:
                audit_rate = data['is_audited'].mean()
                metrics['audit_rate'] = float(audit_rate)
                metrics['unaudited_votes'] = int((data['is_audited'] == False).sum())
            
            # Votos duplicados
            if 'voter_cpf' in data.columns:
                voter_counts = data['voter_cpf'].value_counts()
                duplicate_voters = voter_counts[voter_counts > 1]
                metrics['duplicate_votes'] = int(duplicate_voters.sum())
                metrics['duplicate_voters'] = len(duplicate_voters)
                metrics['duplication_rate'] = float(duplicate_voters.sum() / len(data))
            
            # Votos suspeitos
            if 'is_suspicious' in data.columns:
                suspicious_votes = (data['is_suspicious'] == True).sum()
                metrics['suspicious_votes'] = int(suspicious_votes)
                metrics['suspicious_rate'] = float(suspicious_votes / len(data))
            
            # Score geral de segurança
            security_factors = []
            if 'verification_rate' in metrics:
                security_factors.append(metrics['verification_rate'])
            if 'audit_rate' in metrics:
                security_factors.append(metrics['audit_rate'])
            if 'suspicious_rate' in metrics:
                security_factors.append(1 - metrics['suspicious_rate'])
            
            if security_factors:
                metrics['overall_security_score'] = float(np.mean(security_factors) * 100)
            
            return metrics
            
        except Exception as e:
            self.logger.error(f"Erro no cálculo de métricas de segurança: {e}")
            return {}
    
    def generate_insights(self, data: pd.DataFrame) -> List[str]:
        """Gera insights automáticos dos dados"""
        try:
            insights = []
            
            # Insights de participação
            if 'participation_rate' in data.columns:
                avg_participation = data['participation_rate'].mean()
                if avg_participation > 80:
                    insights.append(f"Excelente participação eleitoral: {avg_participation:.1f}%")
                elif avg_participation < 70:
                    insights.append(f"Baixa participação eleitoral: {avg_participation:.1f}% - considerar campanhas de conscientização")
                else:
                    insights.append(f"Participação moderada: {avg_participation:.1f}%")
            
            # Insights geográficos
            if 'state' in data.columns and 'participation_rate' in data.columns:
                state_participation = data.groupby('state')['participation_rate'].mean()
                best_state = state_participation.idxmax()
                worst_state = state_participation.idxmin()
                insights.append(f"Maior participação: {best_state} ({state_participation[best_state]:.1f}%)")
                insights.append(f"Menor participação: {worst_state} ({state_participation[worst_state]:.1f}%)")
            
            # Insights de segurança
            security_metrics = self.calculate_security_metrics(data)
            if 'overall_security_score' in security_metrics:
                score = security_metrics['overall_security_score']
                if score > 90:
                    insights.append(f"Excelente segurança: {score:.1f}/100")
                elif score < 70:
                    insights.append(f"Segurança preocupante: {score:.1f}/100 - revisar processos")
                else:
                    insights.append(f"Segurança adequada: {score:.1f}/100")
            
            # Insights de competição
            if 'candidate_id' in data.columns:
                candidate_votes = data['candidate_id'].value_counts()
                if len(candidate_votes) > 1:
                    winner_votes = candidate_votes.iloc[0]
                    total_votes = candidate_votes.sum()
                    winner_percentage = winner_votes / total_votes * 100
                    
                    if winner_percentage > 60:
                        insights.append(f"Vitória esmagadora: {winner_percentage:.1f}% dos votos")
                    elif winner_percentage < 35:
                        insights.append(f"Eleição disputada: {winner_percentage:.1f}% dos votos")
                    else:
                        insights.append(f"Vitória confortável: {winner_percentage:.1f}% dos votos")
            
            return insights
            
        except Exception as e:
            self.logger.error(f"Erro na geração de insights: {e}")
            return [f"Erro na análise: {str(e)}"]
    
    def export_to_json(self, data: Any, filepath: str) -> bool:
        """Exporta dados para JSON"""
        try:
            os.makedirs(os.path.dirname(filepath), exist_ok=True)
            
            if isinstance(data, pd.DataFrame):
                data_dict = data.to_dict('records')
            else:
                data_dict = data
            
            with open(filepath, 'w', encoding='utf-8') as f:
                json.dump(data_dict, f, indent=2, ensure_ascii=False, default=str)
            
            self.logger.info(f"Dados exportados para: {filepath}")
            return True
            
        except Exception as e:
            self.logger.error(f"Erro na exportação JSON: {e}")
            return False
    
    def export_to_csv(self, data: pd.DataFrame, filepath: str) -> bool:
        """Exporta dados para CSV"""
        try:
            os.makedirs(os.path.dirname(filepath), exist_ok=True)
            data.to_csv(filepath, index=False, encoding='utf-8')
            
            self.logger.info(f"Dados exportados para: {filepath}")
            return True
            
        except Exception as e:
            self.logger.error(f"Erro na exportação CSV: {e}")
            return False
    
    def generate_data_hash(self, data: Any) -> str:
        """Gera hash dos dados para verificação de integridade"""
        try:
            if isinstance(data, pd.DataFrame):
                data_str = data.to_string()
            else:
                data_str = json.dumps(data, sort_keys=True, default=str)
            
            hash_object = hashlib.sha256(data_str.encode())
            return hash_object.hexdigest()
            
        except Exception as e:
            self.logger.error(f"Erro na geração de hash: {e}")
            return ""
    
    def validate_data_integrity(self, data: pd.DataFrame) -> Dict[str, Any]:
        """Valida integridade dos dados"""
        try:
            validation = {
                'is_valid': True,
                'issues': [],
                'warnings': []
            }
            
            # Verifica valores nulos
            null_counts = data.isnull().sum()
            if null_counts.any():
                validation['warnings'].append(f"Valores nulos encontrados: {null_counts[null_counts > 0].to_dict()}")
            
            # Verifica duplicatas
            if data.duplicated().any():
                validation['issues'].append(f"Duplicatas encontradas: {data.duplicated().sum()}")
                validation['is_valid'] = False
            
            # Verifica tipos de dados
            for col in data.columns:
                if data[col].dtype == 'object':
                    if data[col].str.contains('^\s*$').any():
                        validation['warnings'].append(f"Valores vazios em {col}")
            
            # Verifica consistência temporal
            if 'timestamp' in data.columns:
                timestamps = pd.to_datetime(data['timestamp'], errors='coerce')
                if timestamps.isnull().any():
                    validation['issues'].append("Timestamps inválidos encontrados")
                    validation['is_valid'] = False
            
            return validation
            
        except Exception as e:
            self.logger.error(f"Erro na validação de dados: {e}")
            return {
                'is_valid': False,
                'issues': [f"Erro na validação: {str(e)}"],
                'warnings': []
            }

def main():
    """Função principal para teste"""
    utils = AnalyticsUtils()
    
    # Dados de exemplo
    sample_data = pd.DataFrame({
        'state': ['SP', 'RJ', 'MG', 'RS', 'PR'],
        'participation_rate': [85.2, 78.5, 82.1, 79.3, 81.7],
        'total_votes': [500000, 300000, 250000, 200000, 180000],
        'unique_voters': [400000, 240000, 200000, 160000, 144000],
        'is_verified': [True, True, True, True, True],
        'is_audited': [True, True, False, True, False]
    })
    
    print("Utilitários de Analytics FORTIS")
    
    # Calcula métricas
    metrics = utils.calculate_participation_metrics(sample_data)
    print(f"Métricas de participação: {metrics}")
    
    # Analisa padrões
    patterns = utils.analyze_voting_patterns(sample_data)
    print(f"Padrões de votação: {patterns}")
    
    # Gera insights
    insights = utils.generate_insights(sample_data)
    print(f"Insights: {insights}")
    
    # Valida dados
    validation = utils.validate_data_integrity(sample_data)
    print(f"Validação: {validation}")

if __name__ == "__main__":
    main()
