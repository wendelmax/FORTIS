#!/usr/bin/env python3
"""
FORTIS - Processamento de Dados de Eleitores
Script para processar e limpar dados de eleitores para análise de IA
"""

import pandas as pd
import numpy as np
from typing import Dict, List, Tuple
import logging
from datetime import datetime
import json

# Configuração de logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class VoterDataProcessor:
    """Processador de dados de eleitores para o sistema FORTIS"""
    
    def __init__(self):
        self.processed_data = {}
        self.stats = {}
    
    def load_raw_data(self, file_path: str) -> pd.DataFrame:
        """Carrega dados brutos de eleitores"""
        try:
            if file_path.endswith('.csv'):
                df = pd.read_csv(file_path)
            elif file_path.endswith('.json'):
                df = pd.read_json(file_path)
            else:
                raise ValueError("Formato de arquivo não suportado")
            
            logger.info(f"Dados carregados: {len(df)} registros")
            return df
        except Exception as e:
            logger.error(f"Erro ao carregar dados: {e}")
            raise
    
    def clean_voter_data(self, df: pd.DataFrame) -> pd.DataFrame:
        """Limpa e valida dados de eleitores"""
        logger.info("Iniciando limpeza de dados...")
        
        # Remove duplicatas
        df_clean = df.drop_duplicates(subset=['cpf'])
        
        # Valida CPF
        df_clean = df_clean[df_clean['cpf'].str.len() == 11]
        
        # Remove registros com dados faltantes críticos
        df_clean = df_clean.dropna(subset=['cpf', 'nome', 'data_nascimento'])
        
        # Converte data de nascimento
        df_clean['data_nascimento'] = pd.to_datetime(df_clean['data_nascimento'], errors='coerce')
        
        # Calcula idade
        df_clean['idade'] = (datetime.now() - df_clean['data_nascimento']).dt.days // 365
        
        # Remove eleitores menores de 16 anos
        df_clean = df_clean[df_clean['idade'] >= 16]
        
        logger.info(f"Dados limpos: {len(df_clean)} registros válidos")
        return df_clean
    
    def extract_features(self, df: pd.DataFrame) -> pd.DataFrame:
        """Extrai features para análise de IA"""
        logger.info("Extraindo features...")
        
        # Features demográficas
        df['faixa_etaria'] = pd.cut(df['idade'], 
                                  bins=[0, 25, 35, 45, 55, 65, 100], 
                                  labels=['16-25', '26-35', '36-45', '46-55', '56-65', '65+'])
        
        # Features geográficas (se disponível)
        if 'municipio' in df.columns:
            df['regiao'] = df['municipio'].apply(self._classify_region)
        
        # Features de participação eleitoral
        if 'votou_ultima_eleicao' in df.columns:
            df['participacao_historica'] = df['votou_ultima_eleicao'].astype(int)
        
        logger.info("Features extraídas com sucesso")
        return df
    
    def _classify_region(self, municipio: str) -> str:
        """Classifica município por região"""
        # Mapeamento simplificado de regiões
        regioes = {
            'Norte': ['Manaus', 'Belém', 'Porto Velho', 'Rio Branco'],
            'Nordeste': ['Salvador', 'Recife', 'Fortaleza', 'Natal'],
            'Centro-Oeste': ['Brasília', 'Goiânia', 'Campo Grande', 'Cuiabá'],
            'Sudeste': ['São Paulo', 'Rio de Janeiro', 'Belo Horizonte', 'Vitória'],
            'Sul': ['Curitiba', 'Porto Alegre', 'Florianópolis']
        }
        
        for regiao, municipios in regioes.items():
            if any(m in municipio for m in municipios):
                return regiao
        return 'Outras'
    
    def generate_statistics(self, df: pd.DataFrame) -> Dict:
        """Gera estatísticas dos dados processados"""
        stats = {
            'total_eleitores': len(df),
            'distribuicao_idade': df['idade'].describe().to_dict(),
            'distribuicao_faixa_etaria': df['faixa_etaria'].value_counts().to_dict(),
            'participacao_media': df.get('participacao_historica', pd.Series([0])).mean(),
            'data_processamento': datetime.now().isoformat()
        }
        
        if 'regiao' in df.columns:
            stats['distribuicao_regional'] = df['regiao'].value_counts().to_dict()
        
        self.stats = stats
        return stats
    
    def save_processed_data(self, df: pd.DataFrame, output_path: str):
        """Salva dados processados"""
        try:
            # Salva dados em CSV
            df.to_csv(f"{output_path}/voters_processed.csv", index=False)
            
            # Salva estatísticas em JSON
            with open(f"{output_path}/processing_stats.json", 'w') as f:
                json.dump(self.stats, f, indent=2, default=str)
            
            logger.info(f"Dados processados salvos em: {output_path}")
        except Exception as e:
            logger.error(f"Erro ao salvar dados: {e}")
            raise
    
    def process_pipeline(self, input_path: str, output_path: str):
        """Pipeline completo de processamento"""
        logger.info("Iniciando pipeline de processamento...")
        
        # 1. Carregar dados
        df = self.load_raw_data(input_path)
        
        # 2. Limpar dados
        df_clean = self.clean_voter_data(df)
        
        # 3. Extrair features
        df_features = self.extract_features(df_clean)
        
        # 4. Gerar estatísticas
        stats = self.generate_statistics(df_features)
        
        # 5. Salvar resultados
        self.save_processed_data(df_features, output_path)
        
        logger.info("Pipeline de processamento concluído!")
        return df_features, stats

def main():
    """Função principal"""
    processor = VoterDataProcessor()
    
    # Exemplo de uso
    input_file = "ai/data/raw/voters_sample.csv"
    output_dir = "ai/data/processed"
    
    try:
        df, stats = processor.process_pipeline(input_file, output_dir)
        print(f"Processamento concluído: {len(df)} eleitores processados")
        print(f"Estatísticas: {json.dumps(stats, indent=2, default=str)}")
    except Exception as e:
        logger.error(f"Erro no processamento: {e}")

if __name__ == "__main__":
    main()
