#!/usr/bin/env python3
"""
FORTIS - Limpeza de Dados
Sistema de limpeza e validação de dados eleitorais
"""

import pandas as pd
import numpy as np
from typing import Dict, List, Tuple, Optional, Any
import logging
import re
from datetime import datetime
import hashlib

class DataCleaner:
    """Sistema de limpeza de dados para o FORTIS"""
    
    def __init__(self):
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Estatísticas de limpeza
        self.cleaning_stats = {
            'total_records': 0,
            'cleaned_records': 0,
            'removed_records': 0,
            'errors_found': 0,
            'warnings': []
        }
    
    def clean_voter_data(self, df: pd.DataFrame) -> pd.DataFrame:
        """Limpa dados de eleitores"""
        self.logger.info("Iniciando limpeza de dados de eleitores...")
        
        original_count = len(df)
        self.cleaning_stats['total_records'] = original_count
        
        # Remove duplicatas por CPF
        df_clean = df.drop_duplicates(subset=['cpf'], keep='first')
        removed_duplicates = original_count - len(df_clean)
        if removed_duplicates > 0:
            self.cleaning_stats['warnings'].append(f"Removidas {removed_duplicates} duplicatas por CPF")
        
        # Valida CPF
        df_clean = self._validate_cpf(df_clean)
        
        # Limpa nomes
        df_clean = self._clean_names(df_clean)
        
        # Valida datas de nascimento
        df_clean = self._validate_birth_dates(df_clean)
        
        # Limpa dados geográficos
        df_clean = self._clean_geographic_data(df_clean)
        
        # Remove registros com dados críticos faltantes
        df_clean = df_clean.dropna(subset=['cpf', 'nome', 'data_nascimento'])
        
        self.cleaning_stats['cleaned_records'] = len(df_clean)
        self.cleaning_stats['removed_records'] = original_count - len(df_clean)
        
        self.logger.info(f"Limpeza concluída: {len(df_clean)}/{original_count} registros válidos")
        return df_clean
    
    def clean_vote_data(self, df: pd.DataFrame) -> pd.DataFrame:
        """Limpa dados de votação"""
        self.logger.info("Iniciando limpeza de dados de votação...")
        
        original_count = len(df)
        self.cleaning_stats['total_records'] = original_count
        
        # Valida IDs de voto únicos
        df_clean = df.drop_duplicates(subset=['vote_id'], keep='first')
        
        # Valida timestamps
        df_clean = self._validate_timestamps(df_clean)
        
        # Valida IDs de eleitor
        df_clean = self._validate_voter_ids(df_clean)
        
        # Valida IDs de candidato
        df_clean = self._validate_candidate_ids(df_clean)
        
        # Remove votos duplicados por eleitor
        df_clean = df_clean.drop_duplicates(subset=['voter_id'], keep='first')
        
        self.cleaning_stats['cleaned_records'] = len(df_clean)
        self.cleaning_stats['removed_records'] = original_count - len(df_clean)
        
        self.logger.info(f"Limpeza de votos concluída: {len(df_clean)}/{original_count} registros válidos")
        return df_clean
    
    def _validate_cpf(self, df: pd.DataFrame) -> pd.DataFrame:
        """Valida e limpa CPFs"""
        def is_valid_cpf(cpf: str) -> bool:
            """Valida CPF usando algoritmo oficial"""
            if not cpf or pd.isna(cpf):
                return False
            
            # Remove caracteres não numéricos
            cpf = re.sub(r'[^0-9]', '', str(cpf))
            
            # Verifica se tem 11 dígitos
            if len(cpf) != 11:
                return False
            
            # Verifica se não são todos iguais
            if cpf == cpf[0] * 11:
                return False
            
            # Calcula primeiro dígito verificador
            sum1 = sum(int(cpf[i]) * (10 - i) for i in range(9))
            digit1 = 11 - (sum1 % 11)
            if digit1 >= 10:
                digit1 = 0
            
            # Calcula segundo dígito verificador
            sum2 = sum(int(cpf[i]) * (11 - i) for i in range(10))
            digit2 = 11 - (sum2 % 11)
            if digit2 >= 10:
                digit2 = 0
            
            return int(cpf[9]) == digit1 and int(cpf[10]) == digit2
        
        # Aplica validação
        valid_cpf_mask = df['cpf'].apply(is_valid_cpf)
        invalid_cpfs = (~valid_cpf_mask).sum()
        
        if invalid_cpfs > 0:
            self.cleaning_stats['errors_found'] += invalid_cpfs
            self.cleaning_stats['warnings'].append(f"Encontrados {invalid_cpfs} CPFs inválidos")
        
        return df[valid_cpf_mask]
    
    def _clean_names(self, df: pd.DataFrame) -> pd.DataFrame:
        """Limpa nomes de eleitores"""
        def clean_name(name: str) -> str:
            if pd.isna(name):
                return name
            
            # Remove caracteres especiais exceto espaços e acentos
            name = re.sub(r'[^a-zA-ZÀ-ÿ\s]', '', str(name))
            
            # Remove espaços extras
            name = ' '.join(name.split())
            
            # Capitaliza primeira letra de cada palavra
            name = name.title()
            
            return name
        
        df['nome'] = df['nome'].apply(clean_name)
        
        # Remove nomes muito curtos ou suspeitos
        df = df[df['nome'].str.len() >= 3]
        
        return df
    
    def _validate_birth_dates(self, df: pd.DataFrame) -> pd.DataFrame:
        """Valida datas de nascimento"""
        def is_valid_birth_date(date_str: str) -> bool:
            if pd.isna(date_str):
                return False
            
            try:
                birth_date = pd.to_datetime(date_str)
                today = datetime.now()
                
                # Verifica se a data é válida
                if birth_date > today:
                    return False
                
                # Verifica se a pessoa tem pelo menos 16 anos
                age = (today - birth_date).days // 365
                if age < 16:
                    return False
                
                # Verifica se a pessoa não tem mais de 120 anos
                if age > 120:
                    return False
                
                return True
            except:
                return False
        
        valid_dates_mask = df['data_nascimento'].apply(is_valid_birth_date)
        invalid_dates = (~valid_dates_mask).sum()
        
        if invalid_dates > 0:
            self.cleaning_stats['errors_found'] += invalid_dates
            self.cleaning_stats['warnings'].append(f"Encontradas {invalid_dates} datas de nascimento inválidas")
        
        return df[valid_dates_mask]
    
    def _clean_geographic_data(self, df: pd.DataFrame) -> pd.DataFrame:
        """Limpa dados geográficos"""
        # Limpa municípios
        if 'municipio' in df.columns:
            df['municipio'] = df['municipio'].str.strip().str.title()
        
        # Valida estados
        if 'estado' in df.columns:
            valid_states = ['AC', 'AL', 'AP', 'AM', 'BA', 'CE', 'DF', 'ES', 'GO', 
                          'MA', 'MT', 'MS', 'MG', 'PA', 'PB', 'PR', 'PE', 'PI', 
                          'RJ', 'RN', 'RS', 'RO', 'RR', 'SC', 'SP', 'SE', 'TO']
            
            df['estado'] = df['estado'].str.upper().str.strip()
            valid_state_mask = df['estado'].isin(valid_states)
            invalid_states = (~valid_state_mask).sum()
            
            if invalid_states > 0:
                self.cleaning_stats['errors_found'] += invalid_states
                self.cleaning_stats['warnings'].append(f"Encontrados {invalid_states} estados inválidos")
            
            df = df[valid_state_mask]
        
        return df
    
    def _validate_timestamps(self, df: pd.DataFrame) -> pd.DataFrame:
        """Valida timestamps de votação"""
        if 'timestamp' not in df.columns:
            return df
        
        # Converte para datetime
        df['timestamp'] = pd.to_datetime(df['timestamp'], errors='coerce')
        
        # Remove timestamps inválidos
        valid_timestamps = df['timestamp'].notna()
        invalid_timestamps = (~valid_timestamps).sum()
        
        if invalid_timestamps > 0:
            self.cleaning_stats['errors_found'] += invalid_timestamps
            self.cleaning_stats['warnings'].append(f"Encontrados {invalid_timestamps} timestamps inválidos")
        
        return df[valid_timestamps]
    
    def _validate_voter_ids(self, df: pd.DataFrame) -> pd.DataFrame:
        """Valida IDs de eleitores"""
        if 'voter_id' not in df.columns:
            return df
        
        # Remove IDs vazios ou inválidos
        valid_voter_ids = df['voter_id'].notna() & (df['voter_id'] != '')
        invalid_voter_ids = (~valid_voter_ids).sum()
        
        if invalid_voter_ids > 0:
            self.cleaning_stats['errors_found'] += invalid_voter_ids
            self.cleaning_stats['warnings'].append(f"Encontrados {invalid_voter_ids} IDs de eleitor inválidos")
        
        return df[valid_voter_ids]
    
    def _validate_candidate_ids(self, df: pd.DataFrame) -> pd.DataFrame:
        """Valida IDs de candidatos"""
        if 'candidate_id' not in df.columns:
            return df
        
        # Remove IDs vazios ou inválidos
        valid_candidate_ids = df['candidate_id'].notna() & (df['candidate_id'] != '')
        invalid_candidate_ids = (~valid_candidate_ids).sum()
        
        if invalid_candidate_ids > 0:
            self.cleaning_stats['errors_found'] += invalid_candidate_ids
            self.cleaning_stats['warnings'].append(f"Encontrados {invalid_candidate_ids} IDs de candidato inválidos")
        
        return df[valid_candidate_ids]
    
    def generate_data_hash(self, df: pd.DataFrame) -> str:
        """Gera hash dos dados para verificação de integridade"""
        try:
            # Converte DataFrame para string
            data_str = df.to_string()
            
            # Gera hash SHA-256
            hash_object = hashlib.sha256(data_str.encode())
            return hash_object.hexdigest()
        except Exception as e:
            self.logger.error(f"Erro ao gerar hash: {e}")
            return ""
    
    def get_cleaning_report(self) -> Dict:
        """Gera relatório de limpeza"""
        return {
            'statistics': self.cleaning_stats,
            'data_quality_score': self._calculate_quality_score(),
            'recommendations': self._generate_recommendations(),
            'timestamp': datetime.now().isoformat()
        }
    
    def _calculate_quality_score(self) -> float:
        """Calcula score de qualidade dos dados"""
        if self.cleaning_stats['total_records'] == 0:
            return 0.0
        
        # Score baseado na proporção de registros válidos
        valid_ratio = self.cleaning_stats['cleaned_records'] / self.cleaning_stats['total_records']
        
        # Penaliza por erros encontrados
        error_penalty = min(0.2, self.cleaning_stats['errors_found'] / self.cleaning_stats['total_records'])
        
        quality_score = max(0.0, valid_ratio - error_penalty)
        return round(quality_score * 100, 2)
    
    def _generate_recommendations(self) -> List[str]:
        """Gera recomendações baseadas na análise"""
        recommendations = []
        
        if self.cleaning_stats['removed_records'] > 0:
            recommendations.append(f"Considere revisar {self.cleaning_stats['removed_records']} registros removidos")
        
        if self.cleaning_stats['errors_found'] > 0:
            recommendations.append(f"Implemente validação mais rigorosa para {self.cleaning_stats['errors_found']} erros encontrados")
        
        if self.cleaning_stats['warnings']:
            recommendations.append("Revise os warnings gerados durante a limpeza")
        
        if not recommendations:
            recommendations.append("Dados de alta qualidade - continue com o processo atual")
        
        return recommendations

def main():
    """Função principal para teste"""
    cleaner = DataCleaner()
    
    # Exemplo de dados com problemas
    sample_data = pd.DataFrame({
        'cpf': ['12345678901', '11111111111', '98765432109', '00000000000'],
        'nome': ['João Silva', 'Maria@Santos', 'Pedro Oliveira', ''],
        'data_nascimento': ['1985-03-15', '1990-07-22', '2010-01-01', '1980-12-12'],
        'estado': ['SP', 'RJ', 'MG', 'XX']
    })
    
    print("Sistema de Limpeza de Dados FORTIS")
    print(f"Dados originais: {len(sample_data)} registros")
    
    # Limpa dados
    cleaned_data = cleaner.clean_voter_data(sample_data)
    print(f"Dados limpos: {len(cleaned_data)} registros")
    
    # Gera relatório
    report = cleaner.get_cleaning_report()
    print(f"Score de qualidade: {report['data_quality_score']}%")
    print(f"Recomendações: {report['recommendations']}")

if __name__ == "__main__":
    main()
