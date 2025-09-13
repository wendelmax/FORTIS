// FORTIS Frontend - Serviço de API

import axios, { AxiosInstance, AxiosResponse, AxiosError } from 'axios';
import { ApiResponse, ApiError } from '../types';

class ApiService {
  private api: AxiosInstance;

  constructor() {
    this.api = axios.create({
      baseURL: process.env.REACT_APP_API_URL || 'http://localhost:8080/api/v1',
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    this.setupInterceptors();
  }

  private setupInterceptors() {
    // Request interceptor
    this.api.interceptors.request.use(
      (config) => {
        const token = localStorage.getItem('access_token');
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
      },
      (error) => {
        return Promise.reject(error);
      }
    );

    // Response interceptor
    this.api.interceptors.response.use(
      (response: AxiosResponse) => {
        return response;
      },
      async (error: AxiosError) => {
        if (error.response?.status === 401) {
          // Token expirado, tentar refresh
          const refreshToken = localStorage.getItem('refresh_token');
          if (refreshToken) {
            try {
              const response = await this.refreshToken(refreshToken);
              const newToken = response.data.access_token;
              localStorage.setItem('access_token', newToken);
              
              // Retry da requisição original
              if (error.config) {
                error.config.headers.Authorization = `Bearer ${newToken}`;
                return this.api.request(error.config);
              }
            } catch (refreshError) {
              // Refresh falhou, redirecionar para login
              localStorage.removeItem('access_token');
              localStorage.removeItem('refresh_token');
              window.location.href = '/login';
            }
          } else {
            // Sem refresh token, redirecionar para login
            window.location.href = '/login';
          }
        }
        return Promise.reject(error);
      }
    );
  }

  // Métodos de autenticação
  async login(credentials: { cpf: string; password: string }) {
    const response = await this.api.post('/auth/login', credentials);
    return response.data;
  }

  async refreshToken(refreshToken: string) {
    const response = await this.api.post('/auth/refresh', { refresh_token: refreshToken });
    return response.data;
  }

  async logout() {
    const refreshToken = localStorage.getItem('refresh_token');
    if (refreshToken) {
      await this.api.post('/auth/logout', { refresh_token: refreshToken });
    }
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
  }

  async verifyToken(token: string) {
    const response = await this.api.post('/auth/verify', { token });
    return response.data;
  }

  // Métodos de eleições
  async getElections(params?: any) {
    const response = await this.api.get('/elections', { params });
    return response.data;
  }

  async getElection(id: string) {
    const response = await this.api.get(`/elections/${id}`);
    return response.data;
  }

  async createElection(data: any) {
    const response = await this.api.post('/elections', data);
    return response.data;
  }

  async updateElection(id: string, data: any) {
    const response = await this.api.put(`/elections/${id}`, data);
    return response.data;
  }

  async deleteElection(id: string) {
    const response = await this.api.delete(`/elections/${id}`);
    return response.data;
  }

  async getElectionCandidates(electionId: string) {
    const response = await this.api.get(`/elections/${electionId}/candidates`);
    return response.data;
  }

  async addElectionCandidate(electionId: string, data: any) {
    const response = await this.api.post(`/elections/${electionId}/candidates`, data);
    return response.data;
  }

  // Métodos de votos
  async getVotes(electionId: string) {
    const response = await this.api.get(`/votes/${electionId}`);
    return response.data;
  }

  async verifyVote(voteId: string) {
    const response = await this.api.get(`/votes/verify/${voteId}`);
    return response.data;
  }

  async auditVotes(electionId: string) {
    const response = await this.api.get(`/votes/audit/${electionId}`);
    return response.data;
  }

  // Métodos de nós
  async getNodes() {
    const response = await this.api.get('/nodes');
    return response.data;
  }

  async getNode(id: string) {
    const response = await this.api.get(`/nodes/${id}`);
    return response.data;
  }

  async createNode(data: any) {
    const response = await this.api.post('/nodes', data);
    return response.data;
  }

  async updateNode(id: string, data: any) {
    const response = await this.api.put(`/nodes/${id}`, data);
    return response.data;
  }

  async deleteNode(id: string) {
    const response = await this.api.delete(`/nodes/${id}`);
    return response.data;
  }

  async getNodeStatus(id: string) {
    const response = await this.api.get(`/nodes/${id}/status`);
    return response.data;
  }

  async syncNodes() {
    const response = await this.api.post('/nodes/sync');
    return response.data;
  }

  // Métodos de auditoria
  async getAudits() {
    const response = await this.api.get('/audit');
    return response.data;
  }

  async getAudit(id: string) {
    const response = await this.api.get(`/audit/${id}`);
    return response.data;
  }

  async createAudit(data: any) {
    const response = await this.api.post('/audit', data);
    return response.data;
  }

  async verifyAudit(id: string, data: any) {
    const response = await this.api.post(`/audit/${id}/verify`, data);
    return response.data;
  }

  async getElectionAudits(electionId: string) {
    const response = await this.api.get(`/audit/election/${electionId}`);
    return response.data;
  }

  // Métodos de health check
  async getHealth() {
    const response = await this.api.get('/health');
    return response.data;
  }

  async getReady() {
    const response = await this.api.get('/health/ready');
    return response.data;
  }
}

export const apiService = new ApiService();
export default apiService;
