import React, {createContext, useContext, useReducer, useEffect} from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';
import {Voter, AuthResponse, AppError} from '../types';
import {AuthService} from '../services/AuthService';

interface AuthState {
  isAuthenticated: boolean;
  isLoading: boolean;
  voter: Voter | null;
  accessToken: string | null;
  refreshToken: string | null;
  error: AppError | null;
}

type AuthAction =
  | {type: 'AUTH_START'}
  | {type: 'AUTH_SUCCESS'; payload: AuthResponse}
  | {type: 'AUTH_FAILURE'; payload: AppError}
  | {type: 'AUTH_LOGOUT'}
  | {type: 'AUTH_CLEAR_ERROR'}
  | {type: 'AUTH_UPDATE_VOTER'; payload: Voter};

const initialState: AuthState = {
  isAuthenticated: false,
  isLoading: false,
  voter: null,
  accessToken: null,
  refreshToken: null,
  error: null,
};

const authReducer = (state: AuthState, action: AuthAction): AuthState => {
  switch (action.type) {
    case 'AUTH_START':
      return {
        ...state,
        isLoading: true,
        error: null,
      };
    case 'AUTH_SUCCESS':
      return {
        ...state,
        isAuthenticated: true,
        isLoading: false,
        voter: action.payload.voter,
        accessToken: action.payload.accessToken,
        refreshToken: action.payload.refreshToken,
        error: null,
      };
    case 'AUTH_FAILURE':
      return {
        ...state,
        isAuthenticated: false,
        isLoading: false,
        voter: null,
        accessToken: null,
        refreshToken: null,
        error: action.payload,
      };
    case 'AUTH_LOGOUT':
      return {
        ...state,
        isAuthenticated: false,
        isLoading: false,
        voter: null,
        accessToken: null,
        refreshToken: null,
        error: null,
      };
    case 'AUTH_CLEAR_ERROR':
      return {
        ...state,
        error: null,
      };
    case 'AUTH_UPDATE_VOTER':
      return {
        ...state,
        voter: action.payload,
      };
    default:
      return state;
  }
};

interface AuthContextType {
  state: AuthState;
  login: (cpf: string, password: string) => Promise<void>;
  loginWithBiometric: (biometricData: any) => Promise<void>;
  logout: () => Promise<void>;
  refreshToken: () => Promise<void>;
  clearError: () => void;
  updateVoter: (voter: Voter) => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: React.ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({children}) => {
  const [state, dispatch] = useReducer(authReducer, initialState);
  const authService = new AuthService();

  useEffect(() => {
    checkStoredAuth();
  }, []);

  const checkStoredAuth = async () => {
    try {
      const storedToken = await AsyncStorage.getItem('accessToken');
      const storedVoter = await AsyncStorage.getItem('voter');
      
      if (storedToken && storedVoter) {
        const voter = JSON.parse(storedVoter);
        dispatch({
          type: 'AUTH_SUCCESS',
          payload: {
            success: true,
            accessToken: storedToken,
            refreshToken: '',
            expiresIn: 3600,
            voter,
          },
        });
      }
    } catch (error) {
      console.error('Erro ao verificar autenticação armazenada:', error);
    }
  };

  const login = async (cpf: string, password: string) => {
    dispatch({type: 'AUTH_START'});
    
    try {
      const response = await authService.login(cpf, password);
      
      // Armazenar dados de autenticação
      await AsyncStorage.setItem('accessToken', response.accessToken);
      await AsyncStorage.setItem('refreshToken', response.refreshToken);
      await AsyncStorage.setItem('voter', JSON.stringify(response.voter));
      
      dispatch({type: 'AUTH_SUCCESS', payload: response});
    } catch (error) {
      const appError: AppError = {
        code: 'LOGIN_FAILED',
        message: error instanceof Error ? error.message : 'Erro desconhecido no login',
        timestamp: new Date().toISOString(),
      };
      dispatch({type: 'AUTH_FAILURE', payload: appError});
      throw error;
    }
  };

  const loginWithBiometric = async (biometricData: any) => {
    dispatch({type: 'AUTH_START'});
    
    try {
      const response = await authService.loginWithBiometric(biometricData);
      
      // Armazenar dados de autenticação
      await AsyncStorage.setItem('accessToken', response.accessToken);
      await AsyncStorage.setItem('refreshToken', response.refreshToken);
      await AsyncStorage.setItem('voter', JSON.stringify(response.voter));
      
      dispatch({type: 'AUTH_SUCCESS', payload: response});
    } catch (error) {
      const appError: AppError = {
        code: 'BIOMETRIC_LOGIN_FAILED',
        message: error instanceof Error ? error.message : 'Erro na autenticação biométrica',
        timestamp: new Date().toISOString(),
      };
      dispatch({type: 'AUTH_FAILURE', payload: appError});
      throw error;
    }
  };

  const logout = async () => {
    try {
      if (state.accessToken) {
        await authService.logout(state.accessToken);
      }
      
      // Limpar dados armazenados
      await AsyncStorage.multiRemove(['accessToken', 'refreshToken', 'voter']);
      
      dispatch({type: 'AUTH_LOGOUT'});
    } catch (error) {
      console.error('Erro no logout:', error);
      // Mesmo com erro, fazer logout local
      dispatch({type: 'AUTH_LOGOUT'});
    }
  };

  const refreshToken = async () => {
    if (!state.refreshToken) {
      throw new Error('Nenhum refresh token disponível');
    }
    
    try {
      const response = await authService.refreshToken(state.refreshToken);
      
      // Atualizar tokens armazenados
      await AsyncStorage.setItem('accessToken', response.accessToken);
      await AsyncStorage.setItem('refreshToken', response.refreshToken);
      
      dispatch({
        type: 'AUTH_SUCCESS',
        payload: {
          success: true,
          accessToken: response.accessToken,
          refreshToken: response.refreshToken,
          expiresIn: response.expiresIn,
          voter: state.voter!,
        },
      });
    } catch (error) {
      // Se refresh falhar, fazer logout
      await logout();
      throw error;
    }
  };

  const clearError = () => {
    dispatch({type: 'AUTH_CLEAR_ERROR'});
  };

  const updateVoter = (voter: Voter) => {
    dispatch({type: 'AUTH_UPDATE_VOTER', payload: voter});
  };

  const value: AuthContextType = {
    state,
    login,
    loginWithBiometric,
    logout,
    refreshToken,
    clearError,
    updateVoter,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};
