import {SecurityCheck, SecurityIssue, DeviceInfo, AppError} from '../types';
import DeviceInfo from 'react-native-device-info';
import {Platform} from 'react-native';

export class SecurityService {
  async performSecurityCheck(): Promise<SecurityCheck> {
    try {
      const deviceInfo = await this.getDeviceInfo();
      const issues = await this.detectSecurityIssues(deviceInfo);
      const score = this.calculateSecurityScore(issues);
      const isSecure = score >= 70 && issues.filter(i => i.severity === 'critical').length === 0;

      return {
        isSecure,
        issues,
        score,
        recommendations: this.generateRecommendations(issues),
      };
    } catch (error) {
      console.error('Erro na verificação de segurança:', error);
      throw new Error('Falha na verificação de segurança do dispositivo.');
    }
  }

  async performFullSecurityCheck(): Promise<{check: SecurityCheck; deviceInfo: DeviceInfo}> {
    const deviceInfo = await this.getDeviceInfo();
    const check = await this.performSecurityCheck();
    return {check, deviceInfo};
  }

  private async getDeviceInfo(): Promise<DeviceInfo> {
    try {
      const [
        deviceId,
        model,
        systemName,
        systemVersion,
        isEmulator,
        isJailbroken,
        hasNotch,
        brand,
        buildNumber,
        bundleId,
        carrier,
        deviceName,
        deviceType,
        firstInstallTime,
        installReferrer,
        lastUpdateTime,
        manufacturer,
        readableVersion,
        serialNumber,
        totalDiskCapacity,
        totalMemory,
        usedMemory,
        userAgent,
        version,
        hasSystemFeature,
        isLocationEnabled,
        isAirplaneMode,
        isBatteryCharging,
        batteryLevel,
        isPinOrFingerprintSet,
        isLandscape,
        isTablet,
        supportedAbis,
        supported32BitAbis,
        supported64BitAbis,
        hasGms,
        hasHms,
        hasNotch,
        isLocationEnabled,
        isAirplaneMode,
        isBatteryCharging,
        batteryLevel,
        isPinOrFingerprintSet,
        isLandscape,
        isTablet,
        supportedAbis,
        supported32BitAbis,
        supported64BitAbis,
        hasGms,
        hasHms,
      ] = await Promise.all([
        DeviceInfo.getUniqueId(),
        DeviceInfo.getModel(),
        DeviceInfo.getSystemName(),
        DeviceInfo.getSystemVersion(),
        DeviceInfo.isEmulator(),
        DeviceInfo.isEmulator(), // Placeholder para jailbreak detection
        DeviceInfo.hasNotch(),
        DeviceInfo.getBrand(),
        DeviceInfo.getBuildNumber(),
        DeviceInfo.getBundleId(),
        DeviceInfo.getCarrier(),
        DeviceInfo.getDeviceName(),
        DeviceInfo.getDeviceType(),
        DeviceInfo.getFirstInstallTime(),
        DeviceInfo.getInstallReferrer(),
        DeviceInfo.getLastUpdateTime(),
        DeviceInfo.getManufacturer(),
        DeviceInfo.getReadableVersion(),
        DeviceInfo.getSerialNumber(),
        DeviceInfo.getTotalDiskCapacity(),
        DeviceInfo.getTotalMemory(),
        DeviceInfo.getUsedMemory(),
        DeviceInfo.getUserAgent(),
        DeviceInfo.getVersion(),
        DeviceInfo.hasSystemFeature('android.hardware.fingerprint'),
        DeviceInfo.isLocationEnabled(),
        DeviceInfo.isAirplaneMode(),
        DeviceInfo.isBatteryCharging(),
        DeviceInfo.getBatteryLevel(),
        DeviceInfo.isPinOrFingerprintSet(),
        DeviceInfo.isLandscape(),
        DeviceInfo.isTablet(),
        DeviceInfo.getSupportedAbis(),
        DeviceInfo.getSupported32BitAbis(),
        DeviceInfo.getSupported64BitAbis(),
        DeviceInfo.hasGms(),
        DeviceInfo.hasHms(),
      ]);

      return {
        deviceId,
        model,
        os: systemName,
        version: systemVersion,
        isJailbroken: isEmulator || isJailbroken,
        isRooted: isEmulator,
        hasSecurityPatch: await this.checkSecurityPatch(systemName, systemVersion),
        timestamp: new Date().toISOString(),
      };
    } catch (error) {
      console.error('Erro ao obter informações do dispositivo:', error);
      throw new Error('Falha ao obter informações do dispositivo.');
    }
  }

  private async detectSecurityIssues(deviceInfo: DeviceInfo): Promise<SecurityIssue[]> {
    const issues: SecurityIssue[] = [];

    // Verificar se é emulador
    if (deviceInfo.isJailbroken) {
      issues.push({
        type: 'emulator',
        severity: 'critical',
        description: 'Dispositivo emulador detectado',
        recommendation: 'Use um dispositivo físico para votação',
      });
    }

    // Verificar se está rootado/jailbroken
    if (deviceInfo.isRooted) {
      issues.push({
        type: 'root',
        severity: 'critical',
        description: 'Dispositivo com root/jailbreak detectado',
        recommendation: 'Remova o root/jailbreak para votar',
      });
    }

    // Verificar versão do sistema
    if (this.isOutdatedSystem(deviceInfo.os, deviceInfo.version)) {
      issues.push({
        type: 'outdated',
        severity: 'high',
        description: 'Sistema operacional desatualizado',
        recommendation: 'Atualize o sistema operacional',
      });
    }

    // Verificar patch de segurança
    if (!deviceInfo.hasSecurityPatch) {
      issues.push({
        type: 'outdated',
        severity: 'high',
        description: 'Patch de segurança desatualizado',
        recommendation: 'Instale as atualizações de segurança',
      });
    }

    return issues;
  }

  private calculateSecurityScore(issues: SecurityIssue[]): number {
    let score = 100;

    issues.forEach(issue => {
      switch (issue.severity) {
        case 'critical':
          score -= 50;
          break;
        case 'high':
          score -= 25;
          break;
        case 'medium':
          score -= 15;
          break;
        case 'low':
          score -= 5;
          break;
      }
    });

    return Math.max(0, score);
  }

  private generateRecommendations(issues: SecurityIssue[]): string[] {
    const recommendations: string[] = [];

    if (issues.some(i => i.type === 'emulator')) {
      recommendations.push('Use um dispositivo físico para votação');
    }

    if (issues.some(i => i.type === 'root')) {
      recommendations.push('Remova o root/jailbreak do dispositivo');
    }

    if (issues.some(i => i.type === 'outdated')) {
      recommendations.push('Atualize o sistema operacional e patches de segurança');
    }

    if (recommendations.length === 0) {
      recommendations.push('Dispositivo seguro para votação');
    }

    return recommendations;
  }

  private isOutdatedSystem(os: string, version: string): boolean {
    if (os === 'Android') {
      const androidVersion = parseFloat(version);
      return androidVersion < 8.0; // Android 8.0+
    } else if (os === 'iOS') {
      const iosVersion = parseFloat(version);
      return iosVersion < 12.0; // iOS 12.0+
    }
    return false;
  }

  private async checkSecurityPatch(os: string, version: string): Promise<boolean> {
    // Implementar verificação real de patch de segurança
    // Por enquanto, retorna true para dispositivos com versões recentes
    if (os === 'Android') {
      const androidVersion = parseFloat(version);
      return androidVersion >= 8.0;
    } else if (os === 'iOS') {
      const iosVersion = parseFloat(version);
      return iosVersion >= 12.0;
    }
    return true;
  }

  async isDeviceSecure(): Promise<boolean> {
    const check = await this.performSecurityCheck();
    return check.isSecure;
  }

  async getSecurityLevel(): Promise<'low' | 'medium' | 'high' | 'critical'> {
    const check = await this.performSecurityCheck();
    const score = check.score;
    
    if (score >= 90) return 'high';
    if (score >= 70) return 'medium';
    if (score >= 50) return 'low';
    return 'critical';
  }
}
