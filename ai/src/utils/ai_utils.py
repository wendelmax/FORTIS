#!/usr/bin/env python3
"""
FORTIS - Utilitários de IA
Funções utilitárias para o sistema de IA do FORTIS
"""

import numpy as np
import pandas as pd
from typing import Dict, List, Optional, Tuple, Any
import logging
from datetime import datetime, timedelta
import json
import hashlib
import cv2
from pathlib import Path

class AIUtils:
    """Utilitários para o sistema de IA do FORTIS"""
    
    def __init__(self):
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
    
    def calculate_confidence_score(self, predictions: List[float], weights: Optional[List[float]] = None) -> float:
        """Calcula score de confiança baseado em múltiplas predições"""
        try:
            if not predictions:
                return 0.0
            
            predictions = np.array(predictions)
            
            if weights is None:
                weights = np.ones(len(predictions)) / len(predictions)
            else:
                weights = np.array(weights)
                weights = weights / np.sum(weights)  # Normaliza pesos
            
            # Calcula média ponderada
            weighted_mean = np.average(predictions, weights=weights)
            
            # Calcula variância para medir consistência
            variance = np.average((predictions - weighted_mean) ** 2, weights=weights)
            
            # Score de confiança (maior quando há consistência)
            confidence = max(0, 1 - variance)
            
            return round(confidence, 3)
            
        except Exception as e:
            self.logger.error(f"Erro ao calcular score de confiança: {e}")
            return 0.0
    
    def detect_outliers(self, data: List[float], method: str = 'iqr', threshold: float = 1.5) -> List[bool]:
        """Detecta outliers em uma lista de dados"""
        try:
            data = np.array(data)
            
            if method == 'iqr':
                # Método IQR (Interquartile Range)
                q1 = np.percentile(data, 25)
                q3 = np.percentile(data, 75)
                iqr = q3 - q1
                
                lower_bound = q1 - threshold * iqr
                upper_bound = q3 + threshold * iqr
                
                outliers = (data < lower_bound) | (data > upper_bound)
                
            elif method == 'zscore':
                # Método Z-Score
                mean = np.mean(data)
                std = np.std(data)
                
                if std == 0:
                    outliers = np.zeros(len(data), dtype=bool)
                else:
                    z_scores = np.abs((data - mean) / std)
                    outliers = z_scores > threshold
                    
            else:
                raise ValueError(f"Método não suportado: {method}")
            
            return outliers.tolist()
            
        except Exception as e:
            self.logger.error(f"Erro na detecção de outliers: {e}")
            return [False] * len(data)
    
    def normalize_features(self, features: np.ndarray, method: str = 'minmax') -> np.ndarray:
        """Normaliza features para treinamento de modelos"""
        try:
            if method == 'minmax':
                # Normalização Min-Max
                min_val = np.min(features, axis=0)
                max_val = np.max(features, axis=0)
                
                # Evita divisão por zero
                range_val = max_val - min_val
                range_val[range_val == 0] = 1
                
                normalized = (features - min_val) / range_val
                
            elif method == 'zscore':
                # Normalização Z-Score
                mean = np.mean(features, axis=0)
                std = np.std(features, axis=0)
                
                # Evita divisão por zero
                std[std == 0] = 1
                
                normalized = (features - mean) / std
                
            else:
                raise ValueError(f"Método de normalização não suportado: {method}")
            
            return normalized
            
        except Exception as e:
            self.logger.error(f"Erro na normalização: {e}")
            return features
    
    def create_feature_matrix(self, data: List[Dict], feature_columns: List[str]) -> np.ndarray:
        """Cria matriz de features a partir de dados"""
        try:
            df = pd.DataFrame(data)
            
            # Seleciona apenas colunas de features
            feature_df = df[feature_columns]
            
            # Converte para array numpy
            feature_matrix = feature_df.values
            
            return feature_matrix
            
        except Exception as e:
            self.logger.error(f"Erro ao criar matriz de features: {e}")
            return np.array([])
    
    def calculate_metrics(self, y_true: List[float], y_pred: List[float]) -> Dict[str, float]:
        """Calcula métricas de avaliação de modelos"""
        try:
            y_true = np.array(y_true)
            y_pred = np.array(y_pred)
            
            # Erro quadrático médio
            mse = np.mean((y_true - y_pred) ** 2)
            
            # Raiz do erro quadrático médio
            rmse = np.sqrt(mse)
            
            # Erro absoluto médio
            mae = np.mean(np.abs(y_true - y_pred))
            
            # R²
            ss_res = np.sum((y_true - y_pred) ** 2)
            ss_tot = np.sum((y_true - np.mean(y_true)) ** 2)
            r2 = 1 - (ss_res / ss_tot) if ss_tot != 0 else 0
            
            # Acurácia (para classificação)
            if len(np.unique(y_true)) <= 2:  # Classificação binária
                accuracy = np.mean(y_true == y_pred)
            else:
                accuracy = None
            
            return {
                'mse': round(mse, 4),
                'rmse': round(rmse, 4),
                'mae': round(mae, 4),
                'r2': round(r2, 4),
                'accuracy': round(accuracy, 4) if accuracy is not None else None
            }
            
        except Exception as e:
            self.logger.error(f"Erro ao calcular métricas: {e}")
            return {}
    
    def generate_data_hash(self, data: Any) -> str:
        """Gera hash SHA-256 de dados para verificação de integridade"""
        try:
            if isinstance(data, (dict, list)):
                data_str = json.dumps(data, sort_keys=True, default=str)
            else:
                data_str = str(data)
            
            hash_object = hashlib.sha256(data_str.encode())
            return hash_object.hexdigest()
            
        except Exception as e:
            self.logger.error(f"Erro ao gerar hash: {e}")
            return ""
    
    def validate_image(self, image: np.ndarray, min_size: Tuple[int, int] = (50, 50)) -> bool:
        """Valida se uma imagem é adequada para processamento"""
        try:
            if image is None:
                return False
            
            if len(image.shape) != 3:
                return False
            
            height, width, channels = image.shape
            
            if height < min_size[0] or width < min_size[1]:
                return False
            
            if channels != 3:
                return False
            
            # Verifica se a imagem não está vazia
            if np.all(image == 0):
                return False
            
            return True
            
        except Exception as e:
            self.logger.error(f"Erro na validação de imagem: {e}")
            return False
    
    def preprocess_image(self, image: np.ndarray, target_size: Tuple[int, int] = (224, 224)) -> np.ndarray:
        """Pré-processa imagem para modelos de IA"""
        try:
            if not self.validate_image(image):
                raise ValueError("Imagem inválida")
            
            # Redimensiona
            resized = cv2.resize(image, target_size)
            
            # Normaliza pixels para [0, 1]
            normalized = resized.astype(np.float32) / 255.0
            
            return normalized
            
        except Exception as e:
            self.logger.error(f"Erro no pré-processamento de imagem: {e}")
            return image
    
    def create_time_series_features(self, timestamps: List[datetime]) -> Dict[str, List[float]]:
        """Cria features temporais a partir de timestamps"""
        try:
            features = {
                'hour': [],
                'day_of_week': [],
                'day_of_month': [],
                'month': [],
                'is_weekend': [],
                'is_holiday': []
            }
            
            for timestamp in timestamps:
                features['hour'].append(timestamp.hour)
                features['day_of_week'].append(timestamp.weekday())
                features['day_of_month'].append(timestamp.day)
                features['month'].append(timestamp.month)
                features['is_weekend'].append(1 if timestamp.weekday() >= 5 else 0)
                
                # Simula feriados (em produção, viria de uma base de dados)
                is_holiday = timestamp.month == 12 and timestamp.day == 25  # Natal
                features['is_holiday'].append(1 if is_holiday else 0)
            
            return features
            
        except Exception as e:
            self.logger.error(f"Erro ao criar features temporais: {e}")
            return {}
    
    def calculate_similarity(self, vec1: np.ndarray, vec2: np.ndarray, method: str = 'cosine') -> float:
        """Calcula similaridade entre dois vetores"""
        try:
            if method == 'cosine':
                # Similaridade do cosseno
                dot_product = np.dot(vec1, vec2)
                norm1 = np.linalg.norm(vec1)
                norm2 = np.linalg.norm(vec2)
                
                if norm1 == 0 or norm2 == 0:
                    return 0.0
                
                similarity = dot_product / (norm1 * norm2)
                
            elif method == 'euclidean':
                # Distância euclidiana (invertida para similaridade)
                distance = np.linalg.norm(vec1 - vec2)
                similarity = 1 / (1 + distance)
                
            else:
                raise ValueError(f"Método de similaridade não suportado: {method}")
            
            return round(similarity, 4)
            
        except Exception as e:
            self.logger.error(f"Erro ao calcular similaridade: {e}")
            return 0.0
    
    def create_confusion_matrix(self, y_true: List[int], y_pred: List[int], labels: Optional[List[str]] = None) -> Dict:
        """Cria matriz de confusão para classificação"""
        try:
            y_true = np.array(y_true)
            y_pred = np.array(y_pred)
            
            # Encontra classes únicas
            classes = np.unique(np.concatenate([y_true, y_pred]))
            n_classes = len(classes)
            
            # Cria matriz de confusão
            confusion_matrix = np.zeros((n_classes, n_classes), dtype=int)
            
            for i in range(len(y_true)):
                true_idx = np.where(classes == y_true[i])[0][0]
                pred_idx = np.where(classes == y_pred[i])[0][0]
                confusion_matrix[true_idx, pred_idx] += 1
            
            # Calcula métricas
            accuracy = np.trace(confusion_matrix) / np.sum(confusion_matrix)
            
            precision = []
            recall = []
            f1_score = []
            
            for i in range(n_classes):
                tp = confusion_matrix[i, i]
                fp = np.sum(confusion_matrix[:, i]) - tp
                fn = np.sum(confusion_matrix[i, :]) - tp
                
                prec = tp / (tp + fp) if (tp + fp) > 0 else 0
                rec = tp / (tp + fn) if (tp + fn) > 0 else 0
                f1 = 2 * (prec * rec) / (prec + rec) if (prec + rec) > 0 else 0
                
                precision.append(prec)
                recall.append(rec)
                f1_score.append(f1)
            
            result = {
                'confusion_matrix': confusion_matrix.tolist(),
                'classes': classes.tolist(),
                'labels': labels or [f"Class {i}" for i in classes],
                'accuracy': round(accuracy, 4),
                'precision': [round(p, 4) for p in precision],
                'recall': [round(r, 4) for r in recall],
                'f1_score': [round(f, 4) for f in f1_score]
            }
            
            return result
            
        except Exception as e:
            self.logger.error(f"Erro ao criar matriz de confusão: {e}")
            return {}
    
    def save_results(self, results: Dict, filepath: str):
        """Salva resultados em arquivo JSON"""
        try:
            filepath = Path(filepath)
            filepath.parent.mkdir(parents=True, exist_ok=True)
            
            with open(filepath, 'w') as f:
                json.dump(results, f, indent=2, default=str)
            
            self.logger.info(f"Resultados salvos em: {filepath}")
            
        except Exception as e:
            self.logger.error(f"Erro ao salvar resultados: {e}")
    
    def load_results(self, filepath: str) -> Dict:
        """Carrega resultados de arquivo JSON"""
        try:
            with open(filepath, 'r') as f:
                results = json.load(f)
            
            return results
            
        except Exception as e:
            self.logger.error(f"Erro ao carregar resultados: {e}")
            return {}

def main():
    """Função principal para teste"""
    utils = AIUtils()
    
    print("Utilitários de IA FORTIS")
    
    # Testa cálculo de confiança
    predictions = [0.8, 0.9, 0.7, 0.85]
    confidence = utils.calculate_confidence_score(predictions)
    print(f"Score de confiança: {confidence}")
    
    # Testa detecção de outliers
    data = [1, 2, 3, 4, 5, 100]  # 100 é outlier
    outliers = utils.detect_outliers(data)
    print(f"Outliers detectados: {outliers}")
    
    # Testa métricas
    y_true = [1, 0, 1, 0, 1]
    y_pred = [1, 0, 0, 0, 1]
    metrics = utils.calculate_metrics(y_true, y_pred)
    print(f"Métricas: {metrics}")

if __name__ == "__main__":
    main()
