// FORTIS Frontend - Tipos TypeScript

export interface User {
  id: string;
  cpf: string;
  name: string;
  roles: string[];
  election_eligible: boolean;
  last_login?: string;
  created_at: string;
  updated_at: string;
}

export interface Election {
  id: string;
  title: string;
  description?: string;
  start_date: string;
  end_date: string;
  status: ElectionStatus;
  created_by: string;
  created_at: string;
  updated_at: string;
  blockchain_hash?: string;
  merkle_root?: string;
  ipfs_hash?: string;
  candidates?: Candidate[];
  total_votes?: number;
}

export type ElectionStatus = 'draft' | 'active' | 'paused' | 'completed' | 'cancelled';

export interface Candidate {
  id: string;
  election_id: string;
  name: string;
  party?: string;
  position: string;
  number: string;
  photo_url?: string;
  bio?: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
  votes_count?: number;
}

export interface Vote {
  id: string;
  election_id: string;
  voter_id: string;
  candidate_id: string;
  encrypted_vote: string;
  zk_proof: string;
  nullifier: string;
  transaction_hash?: string;
  block_number?: number;
  merkle_proof?: any;
  ipfs_hash?: string;
  created_at: string;
}

export interface Node {
  id: string;
  name: string;
  url: string;
  public_key: string;
  is_active: boolean;
  last_sync?: string;
  created_at: string;
  updated_at: string;
  status: NodeStatus;
  region?: string;
  votes_processed?: number;
}

export type NodeStatus = 'online' | 'offline' | 'syncing' | 'error';

export interface Auditor {
  id: string;
  name: string;
  email: string;
  organization?: string;
  public_key: string;
  is_active: boolean;
  permissions: string[];
  last_audit_date?: string;
  created_at: string;
  updated_at: string;
}

export interface AuditLog {
  id: string;
  auditor_id: string;
  election_id?: string;
  action: string;
  description: string;
  data_hash: string;
  signature: string;
  created_at: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
  timestamp: string;
  request_id: string;
}

export interface ApiError {
  code: string;
  message: string;
  details?: any;
}

export interface PaginationParams {
  page: number;
  limit: number;
  sort?: string;
  order?: 'asc' | 'desc';
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    pages: number;
  };
}

export interface DashboardStats {
  total_elections: number;
  active_elections: number;
  total_votes: number;
  total_voters: number;
  total_nodes: number;
  online_nodes: number;
  total_auditors: number;
  recent_audits: number;
}

export interface ElectionStats {
  election_id: string;
  total_votes: number;
  votes_by_candidate: {
    candidate_id: string;
    candidate_name: string;
    votes: number;
    percentage: number;
  }[];
  participation_rate: number;
  last_vote_time?: string;
}

export interface SystemMetrics {
  cpu_usage: number;
  memory_usage: number;
  disk_usage: number;
  network_usage: number;
  active_connections: number;
  response_time: number;
  error_rate: number;
}

export interface Notification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  read: boolean;
  created_at: string;
  action_url?: string;
}

export interface LoginRequest {
  cpf: string;
  password: string;
  remember_me?: boolean;
}

export interface LoginResponse {
  access_token: string;
  refresh_token: string;
  expires_in: number;
  token_type: string;
  user: User;
}

export interface CreateElectionRequest {
  title: string;
  description?: string;
  start_date: string;
  end_date: string;
  candidates: CreateCandidateRequest[];
}

export interface CreateCandidateRequest {
  name: string;
  party?: string;
  position: string;
  number: string;
  photo_url?: string;
  bio?: string;
}

export interface UpdateElectionRequest extends Partial<CreateElectionRequest> {
  id: string;
}

export interface CreateNodeRequest {
  name: string;
  url: string;
  public_key: string;
  region?: string;
}

export interface UpdateNodeRequest extends Partial<CreateNodeRequest> {
  id: string;
}

export interface CreateAuditorRequest {
  name: string;
  email: string;
  organization?: string;
  public_key: string;
  permissions: string[];
}

export interface UpdateAuditorRequest extends Partial<CreateAuditorRequest> {
  id: string;
}

export interface FilterParams {
  search?: string;
  status?: string;
  date_from?: string;
  date_to?: string;
  sort_by?: string;
  sort_order?: 'asc' | 'desc';
}

export interface TableColumn<T> {
  key: keyof T;
  label: string;
  sortable?: boolean;
  render?: (value: any, item: T) => React.ReactNode;
}

export interface TableProps<T> {
  data: T[];
  columns: TableColumn<T>[];
  loading?: boolean;
  pagination?: PaginationParams;
  onPageChange?: (page: number) => void;
  onSort?: (key: keyof T, order: 'asc' | 'desc') => void;
  onRowClick?: (item: T) => void;
  emptyMessage?: string;
}

export interface ChartData {
  labels: string[];
  datasets: {
    label: string;
    data: number[];
    backgroundColor?: string | string[];
    borderColor?: string | string[];
    borderWidth?: number;
  }[];
}

export interface ChartProps {
  data: ChartData;
  type: 'line' | 'bar' | 'pie' | 'doughnut';
  title?: string;
  height?: number;
  options?: any;
}
