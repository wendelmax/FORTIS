import {Election, Vote, VoteRequest, VoteResponse, AppError} from '../types';
import {API_BASE_URL} from '../config/api';

export class VotingService {
  private baseUrl: string;

  constructor() {
    this.baseUrl = API_BASE_URL;
  }

  async getElections(): Promise<Election[]> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/elections`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao carregar eleições');
      }

      const data = await response.json();
      return data.elections || [];
    } catch (error) {
      console.error('Erro ao carregar eleições:', error);
      throw new Error('Falha ao carregar eleições. Verifique sua conexão.');
    }
  }

  async getElectionById(electionId: string): Promise<Election> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/elections/${electionId}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao carregar eleição');
      }

      const data = await response.json();
      return data.election;
    } catch (error) {
      console.error('Erro ao carregar eleição:', error);
      throw new Error('Falha ao carregar eleição.');
    }
  }

  async castVote(voteRequest: VoteRequest): Promise<VoteResponse> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/votes`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(voteRequest),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao registrar voto');
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Erro ao registrar voto:', error);
      throw new Error('Falha ao registrar voto. Tente novamente.');
    }
  }

  async getVoteHistory(): Promise<Vote[]> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/votes/history`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao carregar histórico');
      }

      const data = await response.json();
      return data.votes || [];
    } catch (error) {
      console.error('Erro ao carregar histórico de votos:', error);
      return [];
    }
  }

  async getVoteById(voteId: string): Promise<Vote> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/votes/${voteId}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao carregar voto');
      }

      const data = await response.json();
      return data.vote;
    } catch (error) {
      console.error('Erro ao carregar voto:', error);
      throw new Error('Falha ao carregar voto.');
    }
  }

  async verifyVote(voteId: string): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/votes/${voteId}/verify`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro na verificação do voto');
      }

      const data = await response.json();
      return data.isValid;
    } catch (error) {
      console.error('Erro na verificação do voto:', error);
      throw new Error('Falha na verificação do voto.');
    }
  }

  async getVoteReceipt(voteId: string): Promise<string> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/votes/${voteId}/receipt`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao obter comprovante');
      }

      const data = await response.json();
      return data.receipt;
    } catch (error) {
      console.error('Erro ao obter comprovante:', error);
      throw new Error('Falha ao obter comprovante de votação.');
    }
  }

  async getElectionResults(electionId: string): Promise<any> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/elections/${electionId}/results`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao carregar resultados');
      }

      const data = await response.json();
      return data.results;
    } catch (error) {
      console.error('Erro ao carregar resultados:', error);
      throw new Error('Falha ao carregar resultados da eleição.');
    }
  }

  async getActiveElections(): Promise<Election[]> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/elections/active`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao carregar eleições ativas');
      }

      const data = await response.json();
      return data.elections || [];
    } catch (error) {
      console.error('Erro ao carregar eleições ativas:', error);
      throw new Error('Falha ao carregar eleições ativas.');
    }
  }

  async checkVoterEligibility(electionId: string, voterId: string): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/elections/${electionId}/eligibility/${voterId}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao verificar elegibilidade');
      }

      const data = await response.json();
      return data.isEligible;
    } catch (error) {
      console.error('Erro ao verificar elegibilidade:', error);
      throw new Error('Falha ao verificar elegibilidade para votação.');
    }
  }
}
