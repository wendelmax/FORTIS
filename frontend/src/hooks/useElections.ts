// FORTIS Frontend - Hook de Eleições

import { useState, useEffect, useCallback } from 'react';
import { Election, CreateElectionRequest, UpdateElectionRequest } from '../types';
import apiService from '../services/api';

export const useElections = () => {
  const [elections, setElections] = useState<Election[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchElections = useCallback(async (params?: any) => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiService.getElections(params);
      setElections(response.data || []);
    } catch (err: any) {
      setError(err.response?.data?.message || 'Erro ao carregar eleições');
    } finally {
      setLoading(false);
    }
  }, []);

  const createElection = useCallback(async (data: CreateElectionRequest) => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiService.createElection(data);
      setElections(prev => [response.data, ...prev]);
      return response;
    } catch (err: any) {
      setError(err.response?.data?.message || 'Erro ao criar eleição');
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const updateElection = useCallback(async (id: string, data: UpdateElectionRequest) => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiService.updateElection(id, data);
      setElections(prev => prev.map(election => 
        election.id === id ? response.data : election
      ));
      return response;
    } catch (err: any) {
      setError(err.response?.data?.message || 'Erro ao atualizar eleição');
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const deleteElection = useCallback(async (id: string) => {
    try {
      setLoading(true);
      setError(null);
      await apiService.deleteElection(id);
      setElections(prev => prev.filter(election => election.id !== id));
    } catch (err: any) {
      setError(err.response?.data?.message || 'Erro ao deletar eleição');
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchElections();
  }, [fetchElections]);

  return {
    elections,
    loading,
    error,
    fetchElections,
    createElection,
    updateElection,
    deleteElection,
  };
};
