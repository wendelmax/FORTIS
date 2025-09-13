// FORTIS Frontend - Hook de Autenticação

import { useState, useEffect, useCallback } from 'react';
import { User, LoginRequest, LoginResponse } from '../types';
import apiService from '../services/api';

export const useAuth = () => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const initAuth = async () => {
      const token = localStorage.getItem('access_token');
      if (token) {
        try {
          const response = await apiService.verifyToken(token);
          setUser(response.data);
        } catch (err) {
          localStorage.removeItem('access_token');
          localStorage.removeItem('refresh_token');
        }
      }
      setLoading(false);
    };

    initAuth();
  }, []);

  const login = useCallback(async (credentials: LoginRequest) => {
    try {
      setLoading(true);
      setError(null);
      const response: LoginResponse = await apiService.login(credentials);
      
      localStorage.setItem('access_token', response.access_token);
      localStorage.setItem('refresh_token', response.refresh_token);
      setUser(response.user);
      
      return response;
    } catch (err: any) {
      setError(err.response?.data?.message || 'Erro ao fazer login');
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const logout = useCallback(async () => {
    try {
      await apiService.logout();
    } catch (err) {
      console.error('Erro ao fazer logout:', err);
    } finally {
      setUser(null);
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
    }
  }, []);

  const isAuthenticated = !!user;
  const isAdmin = user?.roles.includes('admin') || false;
  const isMinister = user?.roles.includes('minister') || false;

  return {
    user,
    loading,
    error,
    login,
    logout,
    isAuthenticated,
    isAdmin,
    isMinister,
  };
};
