import React, {createContext, useContext, useReducer, useEffect} from 'react';
import {Election, Vote, VoteRequest, VoteResponse, AppError} from '../types';
import {VotingService} from '../services/VotingService';

interface VotingState {
  elections: Election[];
  currentElection: Election | null;
  currentVote: Vote | null;
  isVoting: boolean;
  isLoading: boolean;
  error: AppError | null;
  voteHistory: Vote[];
}

type VotingAction =
  | {type: 'VOTING_START'}
  | {type: 'VOTING_SUCCESS'; payload: VoteResponse}
  | {type: 'VOTING_FAILURE'; payload: AppError}
  | {type: 'VOTING_RESET'}
  | {type: 'ELECTIONS_LOAD_START'}
  | {type: 'ELECTIONS_LOAD_SUCCESS'; payload: Election[]}
  | {type: 'ELECTIONS_LOAD_FAILURE'; payload: AppError}
  | {type: 'ELECTION_SELECT'; payload: Election}
  | {type: 'VOTE_CLEAR_ERROR'}
  | {type: 'VOTE_HISTORY_LOAD'; payload: Vote[]};

const initialState: VotingState = {
  elections: [],
  currentElection: null,
  currentVote: null,
  isVoting: false,
  isLoading: false,
  error: null,
  voteHistory: [],
};

const votingReducer = (state: VotingState, action: VotingAction): VotingState => {
  switch (action.type) {
    case 'VOTING_START':
      return {
        ...state,
        isVoting: true,
        error: null,
      };
    case 'VOTING_SUCCESS':
      const newVote: Vote = {
        id: action.payload.voteId,
        electionId: state.currentElection?.id || '',
        candidateId: '', // Será preenchido pelo componente
        voterId: '', // Será preenchido pelo contexto de auth
        timestamp: action.payload.timestamp,
        isVerified: true,
        receiptHash: action.payload.receiptHash,
        nullifier: '',
        proof: '',
      };
      return {
        ...state,
        isVoting: false,
        currentVote: newVote,
        error: null,
      };
    case 'VOTING_FAILURE':
      return {
        ...state,
        isVoting: false,
        error: action.payload,
      };
    case 'VOTING_RESET':
      return {
        ...state,
        currentElection: null,
        currentVote: null,
        isVoting: false,
        error: null,
      };
    case 'ELECTIONS_LOAD_START':
      return {
        ...state,
        isLoading: true,
        error: null,
      };
    case 'ELECTIONS_LOAD_SUCCESS':
      return {
        ...state,
        elections: action.payload,
        isLoading: false,
        error: null,
      };
    case 'ELECTIONS_LOAD_FAILURE':
      return {
        ...state,
        isLoading: false,
        error: action.payload,
      };
    case 'ELECTION_SELECT':
      return {
        ...state,
        currentElection: action.payload,
        error: null,
      };
    case 'VOTE_CLEAR_ERROR':
      return {
        ...state,
        error: null,
      };
    case 'VOTE_HISTORY_LOAD':
      return {
        ...state,
        voteHistory: action.payload,
      };
    default:
      return state;
  }
};

interface VotingContextType {
  state: VotingState;
  loadElections: () => Promise<void>;
  selectElection: (election: Election) => void;
  castVote: (voteRequest: VoteRequest) => Promise<VoteResponse>;
  resetVoting: () => void;
  loadVoteHistory: () => Promise<void>;
  clearError: () => void;
  getActiveElections: () => Election[];
  getElectionById: (id: string) => Election | undefined;
  hasVotedInElection: (electionId: string) => boolean;
}

const VotingContext = createContext<VotingContextType | undefined>(undefined);

export const useVoting = () => {
  const context = useContext(VotingContext);
  if (!context) {
    throw new Error('useVoting must be used within a VotingProvider');
  }
  return context;
};

interface VotingProviderProps {
  children: React.ReactNode;
}

export const VotingProvider: React.FC<VotingProviderProps> = ({children}) => {
  const [state, dispatch] = useReducer(votingReducer, initialState);
  const votingService = new VotingService();

  useEffect(() => {
    loadElections();
    loadVoteHistory();
  }, []);

  const loadElections = async () => {
    dispatch({type: 'ELECTIONS_LOAD_START'});
    
    try {
      const elections = await votingService.getElections();
      dispatch({type: 'ELECTIONS_LOAD_SUCCESS', payload: elections});
    } catch (error) {
      const appError: AppError = {
        code: 'ELECTIONS_LOAD_FAILED',
        message: error instanceof Error ? error.message : 'Erro ao carregar eleições',
        timestamp: new Date().toISOString(),
      };
      dispatch({type: 'ELECTIONS_LOAD_FAILURE', payload: appError});
    }
  };

  const selectElection = (election: Election) => {
    dispatch({type: 'ELECTION_SELECT', payload: election});
  };

  const castVote = async (voteRequest: VoteRequest): Promise<VoteResponse> => {
    dispatch({type: 'VOTING_START'});
    
    try {
      const response = await votingService.castVote(voteRequest);
      dispatch({type: 'VOTING_SUCCESS', payload: response});
      
      // Recarregar eleições para atualizar status de votação
      await loadElections();
      
      return response;
    } catch (error) {
      const appError: AppError = {
        code: 'VOTE_CAST_FAILED',
        message: error instanceof Error ? error.message : 'Erro ao registrar voto',
        timestamp: new Date().toISOString(),
      };
      dispatch({type: 'VOTING_FAILURE', payload: appError});
      throw error;
    }
  };

  const resetVoting = () => {
    dispatch({type: 'VOTING_RESET'});
  };

  const loadVoteHistory = async () => {
    try {
      const history = await votingService.getVoteHistory();
      dispatch({type: 'VOTE_HISTORY_LOAD', payload: history});
    } catch (error) {
      console.error('Erro ao carregar histórico de votos:', error);
    }
  };

  const clearError = () => {
    dispatch({type: 'VOTE_CLEAR_ERROR'});
  };

  const getActiveElections = (): Election[] => {
    return state.elections.filter(election => election.isActive);
  };

  const getElectionById = (id: string): Election | undefined => {
    return state.elections.find(election => election.id === id);
  };

  const hasVotedInElection = (electionId: string): boolean => {
    return state.voteHistory.some(vote => vote.electionId === electionId);
  };

  const value: VotingContextType = {
    state,
    loadElections,
    selectElection,
    castVote,
    resetVoting,
    loadVoteHistory,
    clearError,
    getActiveElections,
    getElectionById,
    hasVotedInElection,
  };

  return <VotingContext.Provider value={value}>{children}</VotingContext.Provider>;
};
