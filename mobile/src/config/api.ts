// Configuração da API
export const API_BASE_URL = __DEV__ 
  ? 'http://localhost:8080' // Desenvolvimento
  : 'https://api.fortis.gov.br'; // Produção

// Configurações de timeout
export const API_TIMEOUT = 30000; // 30 segundos

// Configurações de retry
export const API_MAX_RETRIES = 3;
export const API_RETRY_DELAY = 1000; // 1 segundo

// Headers padrão
export const DEFAULT_HEADERS = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
  'User-Agent': 'FORTIS-Mobile/1.0.0',
};

// Configurações de segurança
export const SECURITY_CONFIG = {
  minAndroidVersion: 8.0,
  minIOSVersion: 12.0,
  requiredSecurityPatch: true,
  allowEmulator: false,
  allowRoot: false,
  allowJailbreak: false,
};

// Configurações de votação
export const VOTING_CONFIG = {
  maxVoteAttempts: 3,
  voteTimeout: 300000, // 5 minutos
  biometricRequired: true,
  receiptRequired: true,
  auditLogging: true,
};

// Configurações de cache
export const CACHE_CONFIG = {
  electionsCacheTime: 300000, // 5 minutos
  voterCacheTime: 600000, // 10 minutos
  maxCacheSize: 50, // MB
};

// Configurações de logging
export const LOGGING_CONFIG = {
  enableConsoleLog: __DEV__,
  enableFileLog: true,
  enableRemoteLog: !__DEV__,
  logLevel: __DEV__ ? 'debug' : 'error',
};

// Configurações de notificações
export const NOTIFICATION_CONFIG = {
  enablePushNotifications: true,
  enableLocalNotifications: true,
  enableVotingReminders: true,
  enableSecurityAlerts: true,
};

// Configurações de acessibilidade
export const ACCESSIBILITY_CONFIG = {
  enableVoiceOver: true,
  enableLargeText: true,
  enableHighContrast: true,
  enableScreenReader: true,
  enableHapticFeedback: true,
};

// Configurações de blockchain
export const BLOCKCHAIN_CONFIG = {
  network: __DEV__ ? 'testnet' : 'mainnet',
  rpcUrl: __DEV__ 
    ? 'https://polygon-mumbai.g.alchemy.com/v2/demo'
    : 'https://polygon-mainnet.g.alchemy.com/v2/prod',
  contractAddress: __DEV__ 
    ? '0x1234567890123456789012345678901234567890'
    : '0xabcdefabcdefabcdefabcdefabcdefabcdefabcd',
  gasLimit: 500000,
  gasPrice: '20000000000', // 20 gwei
};

// Configurações de criptografia
export const CRYPTO_CONFIG = {
  algorithm: 'AES-256-GCM',
  keyLength: 32,
  ivLength: 12,
  tagLength: 16,
  iterations: 100000,
  saltLength: 16,
};

// Configurações de biometria
export const BIOMETRIC_CONFIG = {
  supportedTypes: ['fingerprint', 'face', 'voice'],
  fallbackEnabled: true,
  maxAttempts: 3,
  timeout: 30000, // 30 segundos
  requireConfirmation: true,
};

// Configurações de rede
export const NETWORK_CONFIG = {
  timeout: 30000,
  retryAttempts: 3,
  retryDelay: 1000,
  enableOfflineMode: false,
  syncOnReconnect: true,
};

// Configurações de UI
export const UI_CONFIG = {
  theme: 'light', // 'light' | 'dark' | 'auto'
  language: 'pt-BR',
  fontSize: 'medium', // 'small' | 'medium' | 'large' | 'extra-large'
  animations: true,
  hapticFeedback: true,
  soundEffects: true,
};

// Configurações de debug
export const DEBUG_CONFIG = {
  enableLogging: __DEV__,
  enablePerformanceMonitoring: __DEV__,
  enableCrashReporting: !__DEV__,
  enableAnalytics: !__DEV__,
  enableRemoteDebugging: __DEV__,
};
