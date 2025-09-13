# FORTIS Analytics - Dashboard de Eleições
# Dashboard interativo para análise de dados eleitorais

import streamlit as st
import pandas as pd
import plotly.express as px
import plotly.graph_objects as go
from datetime import datetime, timedelta
import numpy as np
import json

class ElectionDashboard:
    """Dashboard interativo para análise de eleições"""
    
    def __init__(self):
        self.setup_page_config()
        
    def setup_page_config(self):
        """Configura a página do Streamlit"""
        st.set_page_config(
            page_title="FORTIS - Dashboard de Eleições",
            page_icon="🗳️",
            layout="wide",
            initial_sidebar_state="expanded"
        )
        
    def load_sample_data(self):
        """Carrega dados de exemplo para demonstração"""
        # Dados de exemplo
        elections_data = {
            'election_id': ['1', '2', '3', '4', '5'],
            'name': [
                'Eleição Municipal 2025',
                'Eleição Estadual 2025', 
                'Eleição Federal 2025',
                'Plebiscito Constitucional',
                'Referendo Popular'
            ],
            'start_date': [
                '2025-10-01 08:00:00',
                '2025-10-15 08:00:00',
                '2025-11-01 08:00:00',
                '2025-11-15 08:00:00',
                '2025-12-01 08:00:00'
            ],
            'end_date': [
                '2025-10-01 17:00:00',
                '2025-10-15 17:00:00',
                '2025-11-01 17:00:00',
                '2025-11-15 17:00:00',
                '2025-12-01 17:00:00'
            ],
            'status': ['completed', 'active', 'scheduled', 'scheduled', 'scheduled'],
            'total_votes': [1500000, 2500000, 0, 0, 0],
            'unique_voters': [1200000, 2000000, 0, 0, 0],
            'participation_rate': [85.2, 78.5, 0, 0, 0]
        }
        
        votes_data = []
        for i in range(10000):
            votes_data.append({
                'vote_id': f'vote_{i}',
                'election_id': np.random.choice(['1', '2']),
                'candidate_id': np.random.choice(['cand_1', 'cand_2', 'cand_3', 'cand_4']),
                'voter_cpf': f'{np.random.randint(10000000000, 99999999999)}',
                'timestamp': datetime.now() - timedelta(hours=np.random.randint(0, 24)),
                'is_verified': np.random.choice([True, False], p=[0.95, 0.05]),
                'is_audited': np.random.choice([True, False], p=[0.8, 0.2])
            })
        
        return pd.DataFrame(elections_data), pd.DataFrame(votes_data)
    
    def render_header(self):
        """Renderiza o cabeçalho do dashboard"""
        st.title("🗳️ FORTIS - Dashboard de Eleições")
        st.markdown("---")
        
        # Métricas principais
        col1, col2, col3, col4 = st.columns(4)
        
        with col1:
            st.metric(
                label="Eleições Ativas",
                value="2",
                delta="+1"
            )
        
        with col2:
            st.metric(
                label="Total de Votos",
                value="4,000,000",
                delta="+12.5%"
            )
        
        with col3:
            st.metric(
                label="Taxa de Participação",
                value="81.8%",
                delta="+3.2%"
            )
        
        with col4:
            st.metric(
                label="Votos Verificados",
                value="99.2%",
                delta="+0.1%"
            )
    
    def render_elections_overview(self, elections_df):
        """Renderiza visão geral das eleições"""
        st.subheader("📊 Visão Geral das Eleições")
        
        # Gráfico de status das eleições
        status_counts = elections_df['status'].value_counts()
        
        col1, col2 = st.columns(2)
        
        with col1:
            fig = px.pie(
                values=status_counts.values,
                names=status_counts.index,
                title="Status das Eleições",
                color_discrete_map={
                    'completed': '#2E8B57',
                    'active': '#FFD700',
                    'scheduled': '#4682B4'
                }
            )
            st.plotly_chart(fig, use_container_width=True)
        
        with col2:
            # Gráfico de participação
            active_elections = elections_df[elections_df['status'] == 'active']
            if not active_elections.empty:
                fig = px.bar(
                    active_elections,
                    x='name',
                    y='participation_rate',
                    title="Taxa de Participação por Eleição",
                    color='participation_rate',
                    color_continuous_scale='Viridis'
                )
                fig.update_layout(xaxis_tickangle=-45)
                st.plotly_chart(fig, use_container_width=True)
    
    def render_voting_timeline(self, votes_df):
        """Renderiza timeline de votação"""
        st.subheader("⏰ Timeline de Votação")
        
        # Agrupar votos por hora
        votes_df['hour'] = pd.to_datetime(votes_df['timestamp']).dt.hour
        hourly_votes = votes_df.groupby('hour').size().reset_index(name='votes')
        
        fig = px.line(
            hourly_votes,
            x='hour',
            y='votes',
            title="Votos por Hora",
            markers=True
        )
        fig.update_layout(
            xaxis_title="Hora do Dia",
            yaxis_title="Número de Votos"
        )
        st.plotly_chart(fig, use_container_width=True)
    
    def render_candidate_performance(self, votes_df):
        """Renderiza performance dos candidatos"""
        st.subheader("👥 Performance dos Candidatos")
        
        # Contar votos por candidato
        candidate_votes = votes_df.groupby('candidate_id').size().reset_index(name='votes')
        candidate_votes['percentage'] = (candidate_votes['votes'] / candidate_votes['votes'].sum()) * 100
        
        col1, col2 = st.columns(2)
        
        with col1:
            fig = px.bar(
                candidate_votes,
                x='candidate_id',
                y='votes',
                title="Votos por Candidato",
                color='votes',
                color_continuous_scale='Blues'
            )
            st.plotly_chart(fig, use_container_width=True)
        
        with col2:
            fig = px.pie(
                candidate_votes,
                values='votes',
                names='candidate_id',
                title="Distribuição de Votos (%)"
            )
            st.plotly_chart(fig, use_container_width=True)
    
    def render_security_metrics(self, votes_df):
        """Renderiza métricas de segurança"""
        st.subheader("🔒 Métricas de Segurança")
        
        # Calcular métricas de segurança
        total_votes = len(votes_df)
        verified_votes = votes_df['is_verified'].sum()
        audited_votes = votes_df['is_audited'].sum()
        
        verification_rate = (verified_votes / total_votes) * 100
        audit_rate = (audited_votes / total_votes) * 100
        
        col1, col2, col3 = st.columns(3)
        
        with col1:
            st.metric(
                label="Taxa de Verificação",
                value=f"{verification_rate:.1f}%",
                delta="+0.2%"
            )
        
        with col2:
            st.metric(
                label="Taxa de Auditoria",
                value=f"{audit_rate:.1f}%",
                delta="+1.5%"
            )
        
        with col3:
            st.metric(
                label="Votos Suspeitos",
                value="23",
                delta="-5"
            )
    
    def render_geographic_analysis(self):
        """Renderiza análise geográfica"""
        st.subheader("🗺️ Análise Geográfica")
        
        # Dados de exemplo para mapa
        states_data = {
            'state': ['SP', 'RJ', 'MG', 'RS', 'PR', 'SC', 'BA', 'GO', 'PE', 'CE'],
            'votes': [500000, 300000, 250000, 200000, 180000, 150000, 140000, 120000, 100000, 90000],
            'participation': [85.2, 78.5, 82.1, 79.3, 81.7, 83.4, 76.8, 80.2, 77.9, 75.6]
        }
        
        states_df = pd.DataFrame(states_data)
        
        fig = px.choropleth(
            states_df,
            locations='state',
            color='participation',
            hover_data=['votes', 'participation'],
            title="Taxa de Participação por Estado",
            color_continuous_scale='Viridis'
        )
        st.plotly_chart(fig, use_container_width=True)
    
    def render_sidebar(self):
        """Renderiza barra lateral com filtros"""
        st.sidebar.title("🔧 Filtros")
        
        # Filtro de período
        st.sidebar.subheader("Período")
        start_date = st.sidebar.date_input(
            "Data Inicial",
            value=datetime.now() - timedelta(days=30)
        )
        end_date = st.sidebar.date_input(
            "Data Final",
            value=datetime.now()
        )
        
        # Filtro de eleição
        st.sidebar.subheader("Eleição")
        election_filter = st.sidebar.selectbox(
            "Selecionar Eleição",
            ["Todas", "Eleição Municipal 2025", "Eleição Estadual 2025"]
        )
        
        # Filtro de status
        st.sidebar.subheader("Status")
        status_filter = st.sidebar.multiselect(
            "Status da Eleição",
            ["active", "completed", "scheduled"],
            default=["active", "completed"]
        )
        
        return {
            'start_date': start_date,
            'end_date': end_date,
            'election': election_filter,
            'status': status_filter
        }
    
    def run(self):
        """Executa o dashboard"""
        # Carregar dados
        elections_df, votes_df = self.load_sample_data()
        
        # Renderizar sidebar
        filters = self.render_sidebar()
        
        # Renderizar header
        self.render_header()
        
        # Renderizar seções
        self.render_elections_overview(elections_df)
        self.render_voting_timeline(votes_df)
        self.render_candidate_performance(votes_df)
        self.render_security_metrics(votes_df)
        self.render_geographic_analysis()
        
        # Rodapé
        st.markdown("---")
        st.markdown("**FORTIS - Sistema de Votação Eletrônica Brasileiro** | Dashboard atualizado em tempo real")

# Executar dashboard
if __name__ == "__main__":
    dashboard = ElectionDashboard()
    dashboard.run()
