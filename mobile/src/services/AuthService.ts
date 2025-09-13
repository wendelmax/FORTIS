import {Voter, AuthResponse, BiometricData, AppError} from '../types';
import {API_BASE_URL} from '../config/api';

export class AuthService {
  private baseUrl: string;

  constructor() {
    this.baseUrl = API_BASE_URL;
  }

  async login(cpf: string, password: string): Promise<AuthResponse> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/auth/login`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          cpf: this.formatCpf(cpf),
          password,
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro no login');
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Erro no login:', error);
      throw new Error('Falha na autenticação. Verifique suas credenciais.');
    }
  }

  async loginWithBiometric(biometricData: BiometricData): Promise<AuthResponse> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/auth/biometric`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          biometricData,
          deviceInfo: await this.getDeviceInfo(),
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro na autenticação biométrica');
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Erro na autenticação biométrica:', error);
      throw new Error('Falha na autenticação biométrica. Tente novamente.');
    }
  }

  async refreshToken(refreshToken: string): Promise<AuthResponse> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/auth/refresh`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          refreshToken,
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao renovar token');
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Erro ao renovar token:', error);
      throw new Error('Falha ao renovar token de acesso.');
    }
  }

  async logout(accessToken: string): Promise<void> {
    try {
      await fetch(`${this.baseUrl}/api/v1/auth/logout`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${accessToken}`,
        },
      });
    } catch (error) {
      console.error('Erro no logout:', error);
      // Não lançar erro no logout para não bloquear o usuário
    }
  }

  async validateToken(accessToken: string): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/auth/validate`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${accessToken}`,
        },
      });

      return response.ok;
    } catch (error) {
      console.error('Erro na validação do token:', error);
      return false;
    }
  }

  async getVoterInfo(accessToken: string): Promise<Voter> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/auth/voter`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${accessToken}`,
        },
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao obter dados do eleitor');
      }

      const data = await response.json();
      return data.voter;
    } catch (error) {
      console.error('Erro ao obter dados do eleitor:', error);
      throw new Error('Falha ao obter dados do eleitor.');
    }
  }

  async changePassword(
    accessToken: string,
    currentPassword: string,
    newPassword: string
  ): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/api/v1/auth/change-password`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${accessToken}`,
        },
        body: JSON.stringify({
          currentPassword,
          newPassword,
        }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Erro ao alterar senha');
      }
    } catch (error) {
      console.error('Erro ao alterar senha:', error);
      throw new Error('Falha ao alterar senha. Tente novamente.');
    }
  }

  private formatCpf(cpf: string): string {
    // Remove caracteres não numéricos
    return cpf.replace(/\D/g, '');
  }

  private async getDeviceInfo() {
    const DeviceInfo = require('react-native-device-info');
    
    return {
      deviceId: await DeviceInfo.getUniqueId(),
      model: DeviceInfo.getModel(),
      os: DeviceInfo.getSystemName(),
      version: DeviceInfo.getSystemVersion(),
      isJailbroken: await DeviceInfo.isEmulator(),
      isRooted: await DeviceInfo.isEmulator(),
      hasSecurityPatch: true, // Implementar verificação real
      timestamp: new Date().toISOString(),
    };
  }
}
