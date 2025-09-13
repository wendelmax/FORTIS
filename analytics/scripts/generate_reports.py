# FORTIS Analytics - Gerador de Relatórios
# Script para gerar relatórios automatizados de eleições

import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from datetime import datetime, timedelta
import json
import os
from typing import Dict, List, Any
import logging

class ElectionReportGenerator:
    """Gerador de relatórios de eleições"""
    
    def __init__(self, output_dir: str = "analytics/reports"):
        self.output_dir = output_dir
        self.setup_logging()
        self.ensure_output_dir()
        
    def setup_logging(self):
        """Configura logging"""
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s'
        )
        self.logger = logging.getLogger(__name__)
        
    def ensure_output_dir(self):
        """Garante que o diretório de saída existe"""
        os.makedirs(self.output_dir, exist_ok=True)
        
    def load_election_data(self, election_id: str) -> Dict[str, Any]:
        """Carrega dados de uma eleição específica"""
        # Simulação de dados - em produção viria do banco
        data = {
            'election': {
                'id': election_id,
                'name': 'Eleição Municipal 2025',
                'start_date': '2025-10-01T08:00:00Z',
                'end_date': '2025-10-01T17:00:00Z',
                'status': 'completed',
                'total_votes': 1500000,
                'unique_voters': 1200000,
                'participation_rate': 85.2
            },
            'candidates': [
                {'id': 'cand_1', 'name': 'João Silva', 'party': 'PT', 'votes': 450000, 'percentage': 30.0},
                {'id': 'cand_2', 'name': 'Maria Santos', 'party': 'PSDB', 'votes': 400000, 'percentage': 26.7},
                {'id': 'cand_3', 'name': 'Pedro Costa', 'party': 'PSL', 'votes': 350000, 'percentage': 23.3},
                {'id': 'cand_4', 'name': 'Ana Oliveira', 'party': 'PSOL', 'votes': 300000, 'percentage': 20.0}
            ],
            'votes_timeline': self._generate_votes_timeline(),
            'geographic_data': self._generate_geographic_data(),
            'security_metrics': self._generate_security_metrics()
        }
        return data
        
    def _generate_votes_timeline(self) -> List[Dict]:
        """Gera timeline de votos"""
        timeline = []
        base_time = datetime(2025, 10, 1, 8, 0)
        
        for hour in range(9):  # 8h às 17h
            current_time = base_time + timedelta(hours=hour)
            votes_count = np.random.poisson(150000)  # Média de 150k votos por hora
            
            timeline.append({
                'timestamp': current_time.isoformat(),
                'votes_count': votes_count,
                'cumulative_votes': sum(t['votes_count'] for t in timeline) + votes_count
            })
            
        return timeline
        
    def _generate_geographic_data(self) -> List[Dict]:
        """Gera dados geográficos"""
        states = ['SP', 'RJ', 'MG', 'RS', 'PR', 'SC', 'BA', 'GO', 'PE', 'CE']
        data = []
        
        for state in states:
            data.append({
                'state': state,
                'votes': np.random.randint(50000, 500000),
                'participation_rate': np.random.uniform(70, 90),
                'population': np.random.randint(1000000, 50000000)
            })
            
        return data
        
    def _generate_security_metrics(self) -> Dict:
        """Gera métricas de segurança"""
        return {
            'verified_votes': 1485000,
            'audited_votes': 1200000,
            'suspicious_votes': 23,
            'verification_rate': 99.0,
            'audit_rate': 80.0,
            'security_score': 98.5
        }
        
    def generate_summary_report(self, election_id: str) -> str:
        """Gera relatório resumo da eleição"""
        data = self.load_election_data(election_id)
        
        report = f"""
# RELATÓRIO DE ELEIÇÃO - {data['election']['name']}

## Informações Gerais
- **ID da Eleição:** {data['election']['id']}
- **Data de Início:** {data['election']['start_date']}
- **Data de Fim:** {data['election']['end_date']}
- **Status:** {data['election']['status']}

## Resultados
- **Total de Votos:** {data['election']['total_votes']:,}
- **Eleitores Únicos:** {data['election']['unique_voters']:,}
- **Taxa de Participação:** {data['election']['participation_rate']}%

## Resultados por Candidato
"""
        
        for candidate in data['candidates']:
            report += f"- **{candidate['name']} ({candidate['party']}):** {candidate['votes']:,} votos ({candidate['percentage']}%)\n"
            
        report += f"""
## Métricas de Segurança
- **Votos Verificados:** {data['security_metrics']['verified_votes']:,} ({data['security_metrics']['verification_rate']}%)
- **Votos Auditados:** {data['security_metrics']['audited_votes']:,} ({data['security_metrics']['audit_rate']}%)
- **Votos Suspeitos:** {data['security_metrics']['suspicious_votes']}
- **Score de Segurança:** {data['security_metrics']['security_score']}/100

## Análise Temporal
"""
        
        for entry in data['votes_timeline']:
            report += f"- **{entry['timestamp']}:** {entry['votes_count']:,} votos (Total: {entry['cumulative_votes']:,})\n"
            
        report += f"""
## Análise Geográfica
"""
        
        for geo in data['geographic_data']:
            report += f"- **{geo['state']}:** {geo['votes']:,} votos ({geo['participation_rate']:.1f}% de participação)\n"
            
        report += f"""
---
*Relatório gerado em: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*
*Sistema FORTIS - Votação Eletrônica Brasileira*
"""
        
        return report
        
    def generate_detailed_report(self, election_id: str) -> Dict[str, Any]:
        """Gera relatório detalhado com gráficos"""
        data = self.load_election_data(election_id)
        
        # Criar gráficos
        self._create_candidate_chart(data['candidates'])
        self._create_timeline_chart(data['votes_timeline'])
        self._create_geographic_chart(data['geographic_data'])
        self._create_security_chart(data['security_metrics'])
        
        # Gerar relatório JSON
        detailed_report = {
            'election_summary': data['election'],
            'candidates_analysis': self._analyze_candidates(data['candidates']),
            'temporal_analysis': self._analyze_timeline(data['votes_timeline']),
            'geographic_analysis': self._analyze_geographic(data['geographic_data']),
            'security_analysis': self._analyze_security(data['security_metrics']),
            'recommendations': self._generate_recommendations(data),
            'generated_at': datetime.now().isoformat()
        }
        
        return detailed_report
        
    def _create_candidate_chart(self, candidates: List[Dict]):
        """Cria gráfico de candidatos"""
        df = pd.DataFrame(candidates)
        
        plt.figure(figsize=(12, 8))
        bars = plt.bar(df['name'], df['votes'], color=['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728'])
        
        plt.title('Votos por Candidato', fontsize=16, fontweight='bold')
        plt.xlabel('Candidatos', fontsize=12)
        plt.ylabel('Número de Votos', fontsize=12)
        plt.xticks(rotation=45)
        
        # Adicionar valores nas barras
        for bar, votes in zip(bars, df['votes']):
            plt.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 1000,
                    f'{votes:,}', ha='center', va='bottom')
        
        plt.tight_layout()
        plt.savefig(f'{self.output_dir}/candidates_chart.png', dpi=300, bbox_inches='tight')
        plt.close()
        
    def _create_timeline_chart(self, timeline: List[Dict]):
        """Cria gráfico de timeline"""
        df = pd.DataFrame(timeline)
        df['timestamp'] = pd.to_datetime(df['timestamp'])
        
        plt.figure(figsize=(14, 8))
        plt.plot(df['timestamp'], df['cumulative_votes'], marker='o', linewidth=2, markersize=6)
        
        plt.title('Evolução dos Votos ao Longo do Dia', fontsize=16, fontweight='bold')
        plt.xlabel('Horário', fontsize=12)
        plt.ylabel('Votos Acumulados', fontsize=12)
        plt.grid(True, alpha=0.3)
        plt.xticks(rotation=45)
        
        plt.tight_layout()
        plt.savefig(f'{self.output_dir}/timeline_chart.png', dpi=300, bbox_inches='tight')
        plt.close()
        
    def _create_geographic_chart(self, geographic_data: List[Dict]):
        """Cria gráfico geográfico"""
        df = pd.DataFrame(geographic_data)
        
        plt.figure(figsize=(12, 8))
        bars = plt.bar(df['state'], df['participation_rate'], color='skyblue')
        
        plt.title('Taxa de Participação por Estado', fontsize=16, fontweight='bold')
        plt.xlabel('Estados', fontsize=12)
        plt.ylabel('Taxa de Participação (%)', fontsize=12)
        
        # Adicionar valores nas barras
        for bar, rate in zip(bars, df['participation_rate']):
            plt.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.5,
                    f'{rate:.1f}%', ha='center', va='bottom')
        
        plt.tight_layout()
        plt.savefig(f'{self.output_dir}/geographic_chart.png', dpi=300, bbox_inches='tight')
        plt.close()
        
    def _create_security_chart(self, security_metrics: Dict):
        """Cria gráfico de segurança"""
        metrics = ['Verificação', 'Auditoria', 'Segurança']
        values = [
            security_metrics['verification_rate'],
            security_metrics['audit_rate'],
            security_metrics['security_score']
        ]
        
        plt.figure(figsize=(10, 6))
        bars = plt.bar(metrics, values, color=['#2E8B57', '#FFD700', '#DC143C'])
        
        plt.title('Métricas de Segurança', fontsize=16, fontweight='bold')
        plt.ylabel('Score (%)', fontsize=12)
        plt.ylim(0, 100)
        
        # Adicionar valores nas barras
        for bar, value in zip(bars, values):
            plt.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 1,
                    f'{value:.1f}%', ha='center', va='bottom')
        
        plt.tight_layout()
        plt.savefig(f'{self.output_dir}/security_chart.png', dpi=300, bbox_inches='tight')
        plt.close()
        
    def _analyze_candidates(self, candidates: List[Dict]) -> Dict:
        """Analisa dados dos candidatos"""
        df = pd.DataFrame(candidates)
        
        return {
            'total_candidates': len(candidates),
            'winner': df.loc[df['votes'].idxmax(), 'name'],
            'winner_votes': int(df['votes'].max()),
            'winner_percentage': float(df['votes'].max() / df['votes'].sum() * 100),
            'vote_distribution': df['percentage'].tolist(),
            'competitiveness_index': float(df['percentage'].std())
        }
        
    def _analyze_timeline(self, timeline: List[Dict]) -> Dict:
        """Analisa timeline de votos"""
        df = pd.DataFrame(timeline)
        
        return {
            'peak_hour': df.loc[df['votes_count'].idxmax(), 'timestamp'],
            'peak_votes': int(df['votes_count'].max()),
            'average_hourly_votes': float(df['votes_count'].mean()),
            'voting_pattern': 'uniform' if df['votes_count'].std() < df['votes_count'].mean() * 0.3 else 'variable'
        }
        
    def _analyze_geographic(self, geographic_data: List[Dict]) -> Dict:
        """Analisa dados geográficos"""
        df = pd.DataFrame(geographic_data)
        
        return {
            'highest_participation_state': df.loc[df['participation_rate'].idxmax(), 'state'],
            'highest_participation_rate': float(df['participation_rate'].max()),
            'average_participation': float(df['participation_rate'].mean()),
            'participation_variance': float(df['participation_rate'].var())
        }
        
    def _analyze_security(self, security_metrics: Dict) -> Dict:
        """Analisa métricas de segurança"""
        return {
            'overall_security_score': security_metrics['security_score'],
            'verification_effectiveness': security_metrics['verification_rate'],
            'audit_coverage': security_metrics['audit_rate'],
            'threat_level': 'low' if security_metrics['suspicious_votes'] < 50 else 'medium' if security_metrics['suspicious_votes'] < 100 else 'high'
        }
        
    def _generate_recommendations(self, data: Dict) -> List[str]:
        """Gera recomendações baseadas nos dados"""
        recommendations = []
        
        # Análise de participação
        if data['election']['participation_rate'] < 80:
            recommendations.append("Considerar campanhas de conscientização para aumentar a participação")
            
        # Análise de segurança
        if data['security_metrics']['suspicious_votes'] > 100:
            recommendations.append("Investigar votos suspeitos e reforçar medidas de segurança")
            
        # Análise temporal
        timeline_df = pd.DataFrame(data['votes_timeline'])
        if timeline_df['votes_count'].std() > timeline_df['votes_count'].mean() * 0.5:
            recommendations.append("Considerar distribuição mais uniforme dos horários de votação")
            
        return recommendations
        
    def save_report(self, election_id: str, report_type: str = "summary"):
        """Salva relatório em arquivo"""
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        
        if report_type == "summary":
            report = self.generate_summary_report(election_id)
            filename = f"{self.output_dir}/election_{election_id}_summary_{timestamp}.md"
            with open(filename, 'w', encoding='utf-8') as f:
                f.write(report)
        else:
            report = self.generate_detailed_report(election_id)
            filename = f"{self.output_dir}/election_{election_id}_detailed_{timestamp}.json"
            with open(filename, 'w', encoding='utf-8') as f:
                json.dump(report, f, indent=2, ensure_ascii=False)
                
        self.logger.info(f"Relatório salvo: {filename}")
        return filename

# Exemplo de uso
if __name__ == "__main__":
    generator = ElectionReportGenerator()
    
    # Gerar relatório resumo
    summary_file = generator.save_report("eleicao_2025_001", "summary")
    print(f"Relatório resumo gerado: {summary_file}")
    
    # Gerar relatório detalhado
    detailed_file = generator.save_report("eleicao_2025_001", "detailed")
    print(f"Relatório detalhado gerado: {detailed_file}")
