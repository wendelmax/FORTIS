// Tipos de usuário e autenticação
export interface Voter {
  id: string;
  cpf: string;
  name: string;
  birthDate: string;
  voterId: string;
  zone: string;
  section: string;
  isEligible: boolean;
  hasVoted: boolean;
  lastVoteDate?: string;
}

export interface AuthResponse {
  success: boolean;
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
  voter: Voter;
}

export interface BiometricData {
  type: 'fingerprint' | 'face' | 'voice';
  data: string;
  timestamp: string;
}

// Tipos de eleição
export interface Election {
  id: string;
  title: string;
  description: string;
  startDate: string;
  endDate: string;
  isActive: boolean;
  isVoted: boolean;
  candidates: Candidate[];
  totalVoters: number;
  totalVotes: number;
  type: 'presidential' | 'congressional' | 'state' | 'municipal';
  year: number;
}

export interface Candidate {
  id: string;
  name: string;
  number: string;
  party: string;
  coalition?: string;
  photo?: string;
  bio?: string;
  isActive: boolean;
  votes: number;
}

// Tipos de voto
export interface Vote {
  id: string;
  electionId: string;
  candidateId: string;
  voterId: string;
  timestamp: string;
  isVerified: boolean;
  receiptHash: string;
  nullifier: string;
  proof: string;
}

export interface VoteRequest {
  electionId: string;
  candidateId: string;
  biometricData: BiometricData;
  deviceInfo: DeviceInfo;
}

export interface VoteResponse {
  success: boolean;
  voteId: string;
  receiptHash: string;
  timestamp: string;
  message: string;
}

// Tipos de dispositivo e segurança
export interface DeviceInfo {
  deviceId: string;
  model: string;
  os: string;
  version: string;
  isJailbroken: boolean;
  isRooted: boolean;
  hasSecurityPatch: boolean;
  timestamp: string;
}

export interface SecurityCheck {
  isSecure: boolean;
  issues: SecurityIssue[];
  score: number;
  recommendations: string[];
}

export interface SecurityIssue {
  type: 'jailbreak' | 'root' | 'outdated' | 'debug' | 'emulator';
  severity: 'low' | 'medium' | 'high' | 'critical';
  description: string;
  recommendation: string;
}

// Tipos de auditoria
export interface AuditEvent {
  id: string;
  type: 'vote_cast' | 'vote_verified' | 'authentication' | 'error' | 'security_alert';
  timestamp: string;
  voterId?: string;
  electionId?: string;
  voteId?: string;
  details: Record<string, any>;
  isVerified: boolean;
}

// Tipos de configuração
export interface AppConfig {
  apiBaseUrl: string;
  blockchainRpcUrl: string;
  encryptionKey: string;
  biometricRequired: boolean;
  debugMode: boolean;
  timeout: number;
  maxRetries: number;
}

// Tipos de erro
export interface AppError {
  code: string;
  message: string;
  details?: Record<string, any>;
  timestamp: string;
}

// Tipos de estado
export interface AppState {
  isAuthenticated: boolean;
  currentVoter?: Voter;
  currentElection?: Election;
  isVoting: boolean;
  lastError?: AppError;
  securityLevel: 'low' | 'medium' | 'high' | 'critical';
}

// Tipos de notificação
export interface Notification {
  id: string;
  title: string;
  message: string;
  type: 'info' | 'warning' | 'error' | 'success';
  timestamp: string;
  isRead: boolean;
  action?: {
    type: 'navigate' | 'url' | 'action';
    data: string;
  };
}

// Tipos de ajuda e suporte
export interface HelpItem {
  id: string;
  title: string;
  content: string;
  category: 'voting' | 'security' | 'technical' | 'general';
  isExpanded: boolean;
}

export interface FAQ {
  question: string;
  answer: string;
  category: string;
  tags: string[];
}
