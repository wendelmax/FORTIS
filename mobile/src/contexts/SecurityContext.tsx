import React, {createContext, useContext, useReducer, useEffect} from 'react';
import {SecurityCheck, SecurityIssue, DeviceInfo, AppError} from '../types';
import {SecurityService} from '../services/SecurityService';

interface SecurityState {
  securityCheck: SecurityCheck | null;
  deviceInfo: DeviceInfo | null;
  isSecure: boolean;
  isLoading: boolean;
  error: AppError | null;
  lastCheckTime: string | null;
}

type SecurityAction =
  | {type: 'SECURITY_CHECK_START'}
  | {type: 'SECURITY_CHECK_SUCCESS'; payload: {check: SecurityCheck; deviceInfo: DeviceInfo}}
  | {type: 'SECURITY_CHECK_FAILURE'; payload: AppError}
  | {type: 'SECURITY_CLEAR_ERROR'}
  | {type: 'SECURITY_UPDATE_DEVICE'; payload: DeviceInfo};

const initialState: SecurityState = {
  securityCheck: null,
  deviceInfo: null,
  isSecure: false,
  isLoading: false,
  error: null,
  lastCheckTime: null,
};

const securityReducer = (state: SecurityState, action: SecurityAction): SecurityState => {
  switch (action.type) {
    case 'SECURITY_CHECK_START':
      return {
        ...state,
        isLoading: true,
        error: null,
      };
    case 'SECURITY_CHECK_SUCCESS':
      return {
        ...state,
        securityCheck: action.payload.check,
        deviceInfo: action.payload.deviceInfo,
        isSecure: action.payload.check.isSecure,
        isLoading: false,
        error: null,
        lastCheckTime: new Date().toISOString(),
      };
    case 'SECURITY_CHECK_FAILURE':
      return {
        ...state,
        isLoading: false,
        error: action.payload,
        isSecure: false,
      };
    case 'SECURITY_CLEAR_ERROR':
      return {
        ...state,
        error: null,
      };
    case 'SECURITY_UPDATE_DEVICE':
      return {
        ...state,
        deviceInfo: action.payload,
      };
    default:
      return state;
  }
};

interface SecurityContextType {
  state: SecurityState;
  performSecurityCheck: () => Promise<void>;
  updateDeviceInfo: (deviceInfo: DeviceInfo) => void;
  clearError: () => void;
  getSecurityScore: () => number;
  getSecurityLevel: () => 'low' | 'medium' | 'high' | 'critical';
  getCriticalIssues: () => SecurityIssue[];
  isDeviceSecure: () => boolean;
}

const SecurityContext = createContext<SecurityContextType | undefined>(undefined);

export const useSecurity = () => {
  const context = useContext(SecurityContext);
  if (!context) {
    throw new Error('useSecurity must be used within a SecurityProvider');
  }
  return context;
};

interface SecurityProviderProps {
  children: React.ReactNode;
}

export const SecurityProvider: React.FC<SecurityProviderProps> = ({children}) => {
  const [state, dispatch] = useReducer(securityReducer, initialState);
  const securityService = new SecurityService();

  useEffect(() => {
    performSecurityCheck();
    
    // Verificar segurança periodicamente
    const interval = setInterval(() => {
      performSecurityCheck();
    }, 30000); // A cada 30 segundos

    return () => clearInterval(interval);
  }, []);

  const performSecurityCheck = async () => {
    dispatch({type: 'SECURITY_CHECK_START'});
    
    try {
      const {check, deviceInfo} = await securityService.performFullSecurityCheck();
      dispatch({type: 'SECURITY_CHECK_SUCCESS', payload: {check, deviceInfo}});
    } catch (error) {
      const appError: AppError = {
        code: 'SECURITY_CHECK_FAILED',
        message: error instanceof Error ? error.message : 'Erro na verificação de segurança',
        timestamp: new Date().toISOString(),
      };
      dispatch({type: 'SECURITY_CHECK_FAILURE', payload: appError});
    }
  };

  const updateDeviceInfo = (deviceInfo: DeviceInfo) => {
    dispatch({type: 'SECURITY_UPDATE_DEVICE', payload: deviceInfo});
  };

  const clearError = () => {
    dispatch({type: 'SECURITY_CLEAR_ERROR'});
  };

  const getSecurityScore = (): number => {
    return state.securityCheck?.score || 0;
  };

  const getSecurityLevel = (): 'low' | 'medium' | 'high' | 'critical' => {
    const score = getSecurityScore();
    if (score >= 90) return 'high';
    if (score >= 70) return 'medium';
    if (score >= 50) return 'low';
    return 'critical';
  };

  const getCriticalIssues = (): SecurityIssue[] => {
    return state.securityCheck?.issues.filter(issue => issue.severity === 'critical') || [];
  };

  const isDeviceSecure = (): boolean => {
    return state.isSecure && getCriticalIssues().length === 0;
  };

  const value: SecurityContextType = {
    state,
    performSecurityCheck,
    updateDeviceInfo,
    clearError,
    getSecurityScore,
    getSecurityLevel,
    getCriticalIssues,
    isDeviceSecure,
  };

  return <SecurityContext.Provider value={value}>{children}</SecurityContext.Provider>;
};
