#!/usr/bin/env python3
"""
FORTIS - Modelos de Processamento de Linguagem Natural
Modelos especializados para análise de texto eleitoral
"""

import numpy as np
import pandas as pd
from typing import Dict, List, Optional, Tuple, Any
import logging
from datetime import datetime
import re
import json
from collections import Counter
import pickle
import os
from pathlib import Path

class ElectionTextAnalyzer:
    """Analisador de texto especializado em eleições"""
    
    def __init__(self, model_path: Optional[str] = None):
        self.model_path = model_path or "ai/data/models/nlp_models.pkl"
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
        # Vocabulário especializado em eleições
        self.election_vocabulary = {
            'positive': [
                'democracia', 'transparência', 'confiança', 'segurança', 'integridade',
                'justiça', 'igualdade', 'participação', 'cidadania', 'voto',
                'eleição', 'urna', 'candidato', 'partido', 'governo', 'povo',
                'direito', 'dever', 'responsabilidade', 'honestidade', 'ética'
            ],
            'negative': [
                'fraude', 'corrupção', 'manipulação', 'injustiça', 'desigualdade',
                'exclusão', 'violência', 'ameaça', 'intimidação', 'coação',
                'compra', 'venda', 'tráfico', 'lavagem', 'sonegação', 'evasão',
                'irregularidade', 'ilegalidade', 'inconstitucional', 'arbitrário'
            ],
            'technical': [
                'sistema', 'software', 'hardware', 'programa', 'aplicativo',
                'interface', 'banco', 'dados', 'informação', 'tecnologia',
                'digital', 'eletrônico', 'computador', 'rede', 'internet',
                'código', 'algoritmo', 'criptografia', 'segurança', 'backup'
            ],
            'logistical': [
                'local', 'seção', 'zona', 'mesário', 'fiscal', 'observador',
                'transporte', 'distribuição', 'organização', 'coordenador',
                'supervisor', 'treinamento', 'capacitação', 'instrução',
                'manual', 'procedimento', 'protocolo', 'norma', 'regra'
            ]
        }
        
        # Padrões de regex para extração
        self.patterns = {
            'cpf': r'\b\d{3}\.?\d{3}\.?\d{3}-?\d{2}\b',
            'titulo': r'\b\d{12}\b',
            'secao': r'seção\s*(\d+)',
            'zona': r'zona\s*(\d+)',
            'candidato': r'candidato\s+([A-Za-z\s]+)',
            'partido': r'partido\s+([A-Za-z\s]+)',
            'numero': r'número\s*(\d+)',
            'data': r'\b\d{1,2}/\d{1,2}/\d{4}\b',
            'hora': r'\b\d{1,2}:\d{2}\b'
        }
        
        # Modelos treinados
        self.sentiment_model = None
        self.classification_model = None
        self.entity_model = None
        
        # Estatísticas
        self.analysis_stats = {
            'total_texts': 0,
            'sentiment_analyses': 0,
            'entity_extractions': 0,
            'classifications': 0
        }
    
    def preprocess_text(self, text: str) -> str:
        """Pré-processa texto para análise"""
        try:
            # Converte para minúsculas
            text = text.lower()
            
            # Remove caracteres especiais exceto acentos
            text = re.sub(r'[^a-záàâãéèêíìîóòôõúùûç\s]', ' ', text)
            
            # Remove espaços extras
            text = ' '.join(text.split())
            
            # Remove stopwords básicas
            stopwords = {
                'a', 'ao', 'aos', 'aquela', 'aquelas', 'aquele', 'aqueles', 'aquilo',
                'as', 'até', 'com', 'da', 'das', 'do', 'dos', 'e', 'ela', 'elas',
                'ele', 'eles', 'em', 'na', 'nas', 'no', 'nos', 'o', 'os', 'ou',
                'para', 'pela', 'pelas', 'pelo', 'pelos', 'por', 'que', 'se',
                'sua', 'suas', 'são', 'um', 'uma', 'uns', 'umas'
            }
            
            words = text.split()
            filtered_words = [word for word in words if word not in stopwords]
            
            return ' '.join(filtered_words)
            
        except Exception as e:
            self.logger.error(f"Erro no pré-processamento: {e}")
            return text
    
    def extract_entities(self, text: str) -> Dict[str, List[str]]:
        """Extrai entidades do texto"""
        try:
            entities = {
                'cpfs': [],
                'titulos': [],
                'secoes': [],
                'zonas': [],
                'candidatos': [],
                'partidos': [],
                'numeros': [],
                'datas': [],
                'horas': []
            }
            
            # Extrai CPFs
            cpf_matches = re.findall(self.patterns['cpf'], text)
            entities['cpfs'] = list(set(cpf_matches))
            
            # Extrai títulos de eleitor
            titulo_matches = re.findall(self.patterns['titulo'], text)
            entities['titulos'] = list(set(titulo_matches))
            
            # Extrai seções
            secao_matches = re.findall(self.patterns['secao'], text, re.IGNORECASE)
            entities['secoes'] = list(set(secao_matches))
            
            # Extrai zonas
            zona_matches = re.findall(self.patterns['zona'], text, re.IGNORECASE)
            entities['zonas'] = list(set(zona_matches))
            
            # Extrai candidatos
            candidato_matches = re.findall(self.patterns['candidato'], text, re.IGNORECASE)
            entities['candidatos'] = [match.strip() for match in candidato_matches]
            
            # Extrai partidos
            partido_matches = re.findall(self.patterns['partido'], text, re.IGNORECASE)
            entities['partidos'] = [match.strip() for match in partido_matches]
            
            # Extrai números
            numero_matches = re.findall(self.patterns['numero'], text, re.IGNORECASE)
            entities['numeros'] = list(set(numero_matches))
            
            # Extrai datas
            data_matches = re.findall(self.patterns['data'], text)
            entities['datas'] = list(set(data_matches))
            
            # Extrai horas
            hora_matches = re.findall(self.patterns['hora'], text)
            entities['horas'] = list(set(hora_matches))
            
            self.analysis_stats['entity_extractions'] += 1
            
            return entities
            
        except Exception as e:
            self.logger.error(f"Erro na extração de entidades: {e}")
            return {}
    
    def analyze_sentiment(self, text: str) -> Dict[str, Any]:
        """Analisa sentimento do texto"""
        try:
            processed_text = self.preprocess_text(text)
            words = processed_text.split()
            
            if not words:
                return {
                    'sentiment': 'NEUTRO',
                    'confidence': 0.5,
                    'positive_score': 0.0,
                    'negative_score': 0.0,
                    'neutral_score': 1.0
                }
            
            # Conta palavras positivas e negativas
            positive_count = sum(1 for word in words if word in self.election_vocabulary['positive'])
            negative_count = sum(1 for word in words if word in self.election_vocabulary['negative'])
            
            # Calcula scores
            total_words = len(words)
            positive_score = positive_count / total_words
            negative_score = negative_count / total_words
            neutral_score = 1 - positive_score - negative_score
            
            # Determina sentimento
            if positive_score > negative_score and positive_score > 0.1:
                sentiment = 'POSITIVO'
                confidence = min(0.9, positive_score * 2)
            elif negative_score > positive_score and negative_score > 0.1:
                sentiment = 'NEGATIVO'
                confidence = min(0.9, negative_score * 2)
            else:
                sentiment = 'NEUTRO'
                confidence = 0.5
            
            self.analysis_stats['sentiment_analyses'] += 1
            
            return {
                'sentiment': sentiment,
                'confidence': round(confidence, 3),
                'positive_score': round(positive_score, 3),
                'negative_score': round(negative_score, 3),
                'neutral_score': round(neutral_score, 3),
                'word_count': total_words,
                'positive_words': positive_count,
                'negative_words': negative_count
            }
            
        except Exception as e:
            self.logger.error(f"Erro na análise de sentimento: {e}")
            return {
                'sentiment': 'NEUTRO',
                'confidence': 0.0,
                'error': str(e)
            }
    
    def classify_text_type(self, text: str) -> Dict[str, Any]:
        """Classifica o tipo de texto"""
        try:
            processed_text = self.preprocess_text(text)
            words = processed_text.split()
            
            if not words:
                return {
                    'type': 'UNKNOWN',
                    'confidence': 0.0,
                    'scores': {}
                }
            
            # Calcula scores para cada categoria
            scores = {}
            for category, keywords in self.election_vocabulary.items():
                count = sum(1 for word in words if word in keywords)
                scores[category] = count / len(words) if words else 0
            
            # Determina tipo
            max_score = max(scores.values())
            if max_score > 0.05:  # Threshold mínimo
                text_type = max(scores, key=scores.get).upper()
                confidence = min(0.9, max_score * 5)
            else:
                text_type = 'GENERAL'
                confidence = 0.5
            
            self.analysis_stats['classifications'] += 1
            
            return {
                'type': text_type,
                'confidence': round(confidence, 3),
                'scores': {k: round(v, 3) for k, v in scores.items()},
                'word_count': len(words)
            }
            
        except Exception as e:
            self.logger.error(f"Erro na classificação: {e}")
            return {
                'type': 'UNKNOWN',
                'confidence': 0.0,
                'error': str(e)
            }
    
    def extract_keywords(self, text: str, top_n: int = 10) -> List[Tuple[str, int]]:
        """Extrai palavras-chave do texto"""
        try:
            processed_text = self.preprocess_text(text)
            words = processed_text.split()
            
            # Remove palavras muito curtas
            words = [word for word in words if len(word) > 2]
            
            # Conta frequência
            word_freq = Counter(words)
            
            # Retorna top N palavras
            return word_freq.most_common(top_n)
            
        except Exception as e:
            self.logger.error(f"Erro na extração de palavras-chave: {e}")
            return []
    
    def detect_anomalies(self, text: str) -> Dict[str, Any]:
        """Detecta anomalias no texto"""
        try:
            anomalies = []
            
            # Verifica padrões suspeitos
            if re.search(r'\b\d{11}\b', text):  # Possível CPF sem formatação
                anomalies.append({
                    'type': 'unformatted_cpf',
                    'description': 'CPF encontrado sem formatação adequada',
                    'severity': 'MEDIUM'
                })
            
            if re.search(r'\b\d{4,}\b', text):  # Números muito longos
                anomalies.append({
                    'type': 'long_number',
                    'description': 'Número muito longo encontrado',
                    'severity': 'LOW'
                })
            
            # Verifica repetições excessivas
            words = text.split()
            if len(words) > 0:
                word_freq = Counter(words)
                max_freq = max(word_freq.values())
                if max_freq > len(words) * 0.3:  # Mais de 30% da mesma palavra
                    anomalies.append({
                        'type': 'excessive_repetition',
                        'description': 'Repetição excessiva de palavras',
                        'severity': 'HIGH'
                    })
            
            # Verifica caracteres especiais excessivos
            special_chars = len(re.findall(r'[^a-zA-Záàâãéèêíìîóòôõúùûç\s\d]', text))
            if special_chars > len(text) * 0.1:  # Mais de 10% de caracteres especiais
                anomalies.append({
                    'type': 'excessive_special_chars',
                    'description': 'Muitos caracteres especiais',
                    'severity': 'MEDIUM'
                })
            
            return {
                'anomalies_found': len(anomalies),
                'anomalies': anomalies,
                'text_length': len(text),
                'word_count': len(words)
            }
            
        except Exception as e:
            self.logger.error(f"Erro na detecção de anomalias: {e}")
            return {
                'anomalies_found': 0,
                'anomalies': [],
                'error': str(e)
            }
    
    def analyze_text_complexity(self, text: str) -> Dict[str, Any]:
        """Analisa complexidade do texto"""
        try:
            sentences = re.split(r'[.!?]+', text)
            sentences = [s.strip() for s in sentences if s.strip()]
            
            words = text.split()
            
            if not sentences or not words:
                return {
                    'complexity': 'UNKNOWN',
                    'metrics': {}
                }
            
            # Calcula métricas
            avg_sentence_length = len(words) / len(sentences)
            avg_word_length = sum(len(word) for word in words) / len(words)
            
            # Palavras únicas
            unique_words = len(set(words))
            vocabulary_richness = unique_words / len(words)
            
            # Determina complexidade
            if avg_sentence_length > 20 and avg_word_length > 5:
                complexity = 'HIGH'
            elif avg_sentence_length > 10 and avg_word_length > 4:
                complexity = 'MEDIUM'
            else:
                complexity = 'LOW'
            
            return {
                'complexity': complexity,
                'metrics': {
                    'sentence_count': len(sentences),
                    'word_count': len(words),
                    'unique_words': unique_words,
                    'avg_sentence_length': round(avg_sentence_length, 2),
                    'avg_word_length': round(avg_word_length, 2),
                    'vocabulary_richness': round(vocabulary_richness, 3)
                }
            }
            
        except Exception as e:
            self.logger.error(f"Erro na análise de complexidade: {e}")
            return {
                'complexity': 'UNKNOWN',
                'error': str(e)
            }
    
    def comprehensive_analysis(self, text: str) -> Dict[str, Any]:
        """Análise completa do texto"""
        try:
            self.analysis_stats['total_texts'] += 1
            
            # Executa todas as análises
            entities = self.extract_entities(text)
            sentiment = self.analyze_sentiment(text)
            classification = self.classify_text_type(text)
            keywords = self.extract_keywords(text)
            anomalies = self.detect_anomalies(text)
            complexity = self.analyze_text_complexity(text)
            
            return {
                'text': text,
                'entities': entities,
                'sentiment': sentiment,
                'classification': classification,
                'keywords': keywords,
                'anomalies': anomalies,
                'complexity': complexity,
                'analysis_timestamp': datetime.now().isoformat(),
                'text_length': len(text),
                'word_count': len(text.split())
            }
            
        except Exception as e:
            self.logger.error(f"Erro na análise completa: {e}")
            return {
                'text': text,
                'error': str(e),
                'analysis_timestamp': datetime.now().isoformat()
            }
    
    def get_analysis_statistics(self) -> Dict[str, Any]:
        """Retorna estatísticas de análise"""
        return {
            'total_texts': self.analysis_stats['total_texts'],
            'sentiment_analyses': self.analysis_stats['sentiment_analyses'],
            'entity_extractions': self.analysis_stats['entity_extractions'],
            'classifications': self.analysis_stats['classifications'],
            'vocabulary_size': {
                'positive': len(self.election_vocabulary['positive']),
                'negative': len(self.election_vocabulary['negative']),
                'technical': len(self.election_vocabulary['technical']),
                'logistical': len(self.election_vocabulary['logistical'])
            }
        }
    
    def save_model(self):
        """Salva modelo treinado"""
        try:
            model_data = {
                'election_vocabulary': self.election_vocabulary,
                'patterns': self.patterns,
                'analysis_stats': self.analysis_stats,
                'timestamp': datetime.now().isoformat()
            }
            
            os.makedirs(os.path.dirname(self.model_path), exist_ok=True)
            with open(self.model_path, 'wb') as f:
                pickle.dump(model_data, f)
            
            self.logger.info(f"Modelo NLP salvo em: {self.model_path}")
            
        except Exception as e:
            self.logger.error(f"Erro ao salvar modelo: {e}")
    
    def load_model(self):
        """Carrega modelo treinado"""
        try:
            if os.path.exists(self.model_path):
                with open(self.model_path, 'rb') as f:
                    model_data = pickle.load(f)
                
                self.election_vocabulary = model_data.get('election_vocabulary', self.election_vocabulary)
                self.patterns = model_data.get('patterns', self.patterns)
                self.analysis_stats = model_data.get('analysis_stats', self.analysis_stats)
                
                self.logger.info("Modelo NLP carregado com sucesso")
            else:
                self.logger.warning("Modelo NLP não encontrado")
        except Exception as e:
            self.logger.error(f"Erro ao carregar modelo: {e}")

def main():
    """Função principal para teste"""
    analyzer = ElectionTextAnalyzer()
    
    # Exemplo de texto eleitoral
    sample_text = """
    O sistema eleitoral digital está funcionando muito bem. 
    A transparência e segurança das urnas eletrônicas são excelentes.
    Candidato João Silva do Partido Democrático está liderando.
    Seção 123, Zona 45. CPF: 123.456.789-01
    """
    
    print("Analisador de Texto Eleitoral FORTIS")
    
    # Análise completa
    result = analyzer.comprehensive_analysis(sample_text)
    
    print(f"Sentimento: {result['sentiment']['sentiment']}")
    print(f"Classificação: {result['classification']['type']}")
    print(f"Entidades: {result['entities']}")
    print(f"Palavras-chave: {result['keywords'][:5]}")
    print(f"Anomalias: {result['anomalies']['anomalies_found']}")
    
    # Estatísticas
    stats = analyzer.get_analysis_statistics()
    print(f"Estatísticas: {stats}")

if __name__ == "__main__":
    main()
