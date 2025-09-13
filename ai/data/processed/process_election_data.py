#!/usr/bin/env python3
"""
FORTIS - Processamento de Dados de Eleições
Script para processar dados de eleições e resultados
"""

import pandas as pd
import numpy as np
from typing import Dict, List, Tuple
import logging
from datetime import datetime
import json

class ElectionDataProcessor:
    """Processador de dados de eleições para o sistema FORTIS"""
    
    def __init__(self):
        self.processed_data = {}
        self.election_stats = {}
    
    def load_election_data(self, file_path: str) -> pd.DataFrame:
        """Carrega dados de eleições"""
        try:
            if file_path.endswith('.csv'):
                df = pd.read_csv(file_path)
            elif file_path.endswith('.json'):
                df = pd.read_json(file_path)
            else:
                raise ValueError("Formato de arquivo não suportado")
            
            logging.info(f"Dados de eleição carregados: {len(df)} registros")
            return df
        except Exception as e:
            logging.error(f"Erro ao carregar dados de eleição: {e}")
            raise
    
    def process_vote_data(self, df: pd.DataFrame) -> pd.DataFrame:
        """Processa dados de votação"""
        # Converte timestamps
        df['timestamp'] = pd.to_datetime(df['timestamp'])
        
        # Calcula métricas de tempo
        df['hora_voto'] = df['timestamp'].dt.hour
        df['dia_semana'] = df['timestamp'].dt.day_name()
        
        # Agrupa por candidato
        vote_counts = df.groupby('candidate_id').size().reset_index(name='total_votos')
        
        return vote_counts
    
    def calculate_turnout_metrics(self, df: pd.DataFrame) -> Dict:
        """Calcula métricas de participação"""
        total_eligible = df['voter_id'].nunique()
        total_voted = df['voter_id'].nunique()
        
        turnout_rate = (total_voted / total_eligible) * 100 if total_eligible > 0 else 0
        
        return {
            'total_eligible_voters': total_eligible,
            'total_voted': total_voted,
            'turnout_rate': turnout_rate,
            'participation_level': self._classify_participation(turnout_rate)
        }
    
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
    
    def detect_anomalies(self, df: pd.DataFrame) -> List[Dict]:
        """Detecta anomalias nos dados de votação"""
        anomalies = []
        
        # Anomalia: Votos em horário suspeito (madrugada)
        night_votes = df[df['hora_voto'].between(0, 5)]
        if len(night_votes) > 0:
            anomalies.append({
                'type': 'night_voting',
                'count': len(night_votes),
                'description': 'Votos registrados durante madrugada'
            })
        
        # Anomalia: Muitos votos do mesmo eleitor
        voter_counts = df['voter_id'].value_counts()
        duplicate_votes = voter_counts[voter_counts > 1]
        if len(duplicate_votes) > 0:
            anomalies.append({
                'type': 'duplicate_votes',
                'count': len(duplicate_votes),
                'description': 'Eleitores com múltiplos votos'
            })
        
        return anomalies
    
    def generate_election_report(self, df: pd.DataFrame) -> Dict:
        """Gera relatório completo da eleição"""
        report = {
            'election_summary': {
                'total_votes': len(df),
                'unique_voters': df['voter_id'].nunique(),
                'unique_candidates': df['candidate_id'].nunique(),
                'election_duration_hours': (df['timestamp'].max() - df['timestamp'].min()).total_seconds() / 3600
            },
            'turnout_metrics': self.calculate_turnout_metrics(df),
            'vote_distribution': df['candidate_id'].value_counts().to_dict(),
            'temporal_analysis': {
                'votes_by_hour': df['hora_voto'].value_counts().sort_index().to_dict(),
                'votes_by_day': df['dia_semana'].value_counts().to_dict()
            },
            'anomalies': self.detect_anomalies(df),
            'processing_timestamp': datetime.now().isoformat()
        }
        
        return report

def main():
    """Função principal"""
    processor = ElectionDataProcessor()
    
    # Exemplo de uso
    input_file = "ai/data/raw/election_votes.csv"
    output_dir = "ai/data/processed"
    
    try:
        df = processor.load_election_data(input_file)
        report = processor.generate_election_report(df)
        
        # Salva relatório
        with open(f"{output_dir}/election_report.json", 'w') as f:
            json.dump(report, f, indent=2, default=str)
        
        print("Relatório de eleição gerado com sucesso!")
        print(f"Total de votos: {report['election_summary']['total_votes']}")
        print(f"Taxa de participação: {report['turnout_metrics']['turnout_rate']:.2f}%")
        
    except Exception as e:
        logging.error(f"Erro no processamento: {e}")

if __name__ == "__main__":
    main()
