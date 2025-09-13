#!/usr/bin/env python3
"""
FORTIS - Modelo de Reconhecimento Facial
Implementação do modelo de reconhecimento facial para verificação de eleitores
"""

import cv2
import numpy as np
import face_recognition
from typing import Dict, List, Tuple, Optional
import logging
import pickle
import os
from datetime import datetime

class FaceRecognitionModel:
    """Modelo de reconhecimento facial para o sistema FORTIS"""
    
    def __init__(self, model_path: Optional[str] = None):
        self.known_faces = {}
        self.known_encodings = []
        self.known_cpfs = []
        self.model_path = model_path or "ai/data/models/face_encodings.pkl"
        self.load_model()
        
        # Configuração de logging
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
    
    def load_model(self):
        """Carrega modelo treinado"""
        try:
            if os.path.exists(self.model_path):
                with open(self.model_path, 'rb') as f:
                    data = pickle.load(f)
                    self.known_faces = data.get('faces', {})
                    self.known_encodings = data.get('encodings', [])
                    self.known_cpfs = data.get('cpfs', [])
                self.logger.info(f"Modelo carregado: {len(self.known_faces)} faces conhecidas")
            else:
                self.logger.warning("Modelo não encontrado, iniciando com dados vazios")
        except Exception as e:
            self.logger.error(f"Erro ao carregar modelo: {e}")
            self.known_faces = {}
            self.known_encodings = []
            self.known_cpfs = []
    
    def save_model(self):
        """Salva modelo treinado"""
        try:
            data = {
                'faces': self.known_faces,
                'encodings': self.known_encodings,
                'cpfs': self.known_cpfs,
                'timestamp': datetime.now().isoformat()
            }
            
            os.makedirs(os.path.dirname(self.model_path), exist_ok=True)
            with open(self.model_path, 'wb') as f:
                pickle.dump(data, f)
            
            self.logger.info(f"Modelo salvo em: {self.model_path}")
        except Exception as e:
            self.logger.error(f"Erro ao salvar modelo: {e}")
    
    def preprocess_image(self, image: np.ndarray) -> np.ndarray:
        """Pré-processa imagem para reconhecimento"""
        # Converte para RGB se necessário
        if len(image.shape) == 3 and image.shape[2] == 3:
            image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
        
        # Redimensiona se muito grande
        height, width = image.shape[:2]
        if height > 1000 or width > 1000:
            scale = min(1000/height, 1000/width)
            new_height = int(height * scale)
            new_width = int(width * scale)
            image = cv2.resize(image, (new_width, new_height))
        
        return image
    
    def extract_face_encoding(self, image: np.ndarray) -> Optional[np.ndarray]:
        """Extrai encoding facial de uma imagem"""
        try:
            # Pré-processa imagem
            processed_image = self.preprocess_image(image)
            
            # Encontra faces na imagem
            face_locations = face_recognition.face_locations(processed_image)
            
            if not face_locations:
                self.logger.warning("Nenhuma face encontrada na imagem")
                return None
            
            # Usa a primeira face encontrada
            face_location = face_locations[0]
            
            # Extrai encoding
            face_encodings = face_recognition.face_encodings(
                processed_image, 
                [face_location]
            )
            
            if face_encodings:
                return face_encodings[0]
            else:
                self.logger.warning("Não foi possível extrair encoding facial")
                return None
                
        except Exception as e:
            self.logger.error(f"Erro ao extrair encoding facial: {e}")
            return None
    
    def add_face(self, image: np.ndarray, cpf: str, name: str) -> bool:
        """Adiciona nova face ao modelo"""
        try:
            encoding = self.extract_face_encoding(image)
            if encoding is None:
                return False
            
            # Adiciona aos dados conhecidos
            self.known_faces[cpf] = {
                'name': name,
                'encoding': encoding,
                'added_at': datetime.now().isoformat()
            }
            
            self.known_encodings.append(encoding)
            self.known_cpfs.append(cpf)
            
            # Salva modelo atualizado
            self.save_model()
            
            self.logger.info(f"Face adicionada para CPF: {cpf}")
            return True
            
        except Exception as e:
            self.logger.error(f"Erro ao adicionar face: {e}")
            return False
    
    def recognize_face(self, image: np.ndarray, tolerance: float = 0.6) -> Dict:
        """Reconhece face em uma imagem"""
        try:
            encoding = self.extract_face_encoding(image)
            if encoding is None:
                return {
                    'recognized': False,
                    'confidence': 0.0,
                    'cpf': None,
                    'name': None,
                    'error': 'Nenhuma face encontrada'
                }
            
            if not self.known_encodings:
                return {
                    'recognized': False,
                    'confidence': 0.0,
                    'cpf': None,
                    'name': None,
                    'error': 'Nenhuma face conhecida no modelo'
                }
            
            # Compara com faces conhecidas
            face_distances = face_recognition.face_distance(
                self.known_encodings, 
                encoding
            )
            
            # Encontra a melhor correspondência
            best_match_index = np.argmin(face_distances)
            best_distance = face_distances[best_match_index]
            
            # Calcula confiança (1 - distância)
            confidence = max(0, 1 - best_distance)
            
            if best_distance <= tolerance:
                cpf = self.known_cpfs[best_match_index]
                face_data = self.known_faces[cpf]
                
                return {
                    'recognized': True,
                    'confidence': confidence,
                    'cpf': cpf,
                    'name': face_data['name'],
                    'distance': best_distance
                }
            else:
                return {
                    'recognized': False,
                    'confidence': confidence,
                    'cpf': None,
                    'name': None,
                    'error': 'Face não reconhecida'
                }
                
        except Exception as e:
            self.logger.error(f"Erro ao reconhecer face: {e}")
            return {
                'recognized': False,
                'confidence': 0.0,
                'cpf': None,
                'name': None,
                'error': str(e)
            }
    
    def verify_voter(self, image: np.ndarray, expected_cpf: str) -> Dict:
        """Verifica se a face corresponde ao CPF esperado"""
        try:
            result = self.recognize_face(image)
            
            if not result['recognized']:
                return {
                    'verified': False,
                    'confidence': result['confidence'],
                    'error': result.get('error', 'Face não reconhecida')
                }
            
            # Verifica se o CPF reconhecido corresponde ao esperado
            if result['cpf'] == expected_cpf:
                return {
                    'verified': True,
                    'confidence': result['confidence'],
                    'cpf': result['cpf'],
                    'name': result['name']
                }
            else:
                return {
                    'verified': False,
                    'confidence': result['confidence'],
                    'error': f'CPF não confere. Esperado: {expected_cpf}, Reconhecido: {result["cpf"]}'
                }
                
        except Exception as e:
            self.logger.error(f"Erro na verificação do eleitor: {e}")
            return {
                'verified': False,
                'confidence': 0.0,
                'error': str(e)
            }
    
    def get_model_stats(self) -> Dict:
        """Retorna estatísticas do modelo"""
        return {
            'total_faces': len(self.known_faces),
            'model_path': self.model_path,
            'last_updated': max([
                face_data.get('added_at', '') 
                for face_data in self.known_faces.values()
            ], default='Nunca')
        }

def main():
    """Função principal para teste"""
    model = FaceRecognitionModel()
    
    # Exemplo de uso
    print("Modelo de Reconhecimento Facial FORTIS")
    print(f"Faces conhecidas: {len(model.known_faces)}")
    
    # Estatísticas do modelo
    stats = model.get_model_stats()
    print(f"Estatísticas: {stats}")

if __name__ == "__main__":
    main()
