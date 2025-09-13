export type RootStackParamList = {
  Login: undefined;
  BiometricAuth: {
    cpf: string;
    accessToken: string;
  };
  ElectionList: undefined;
  Voting: {
    electionId: string;
    electionTitle: string;
  };
  VoteConfirmation: {
    electionId: string;
    candidateId: string;
    candidateName: string;
    candidateNumber: string;
  };
  VoteReceipt: {
    voteId: string;
    electionId: string;
    candidateName: string;
    candidateNumber: string;
    timestamp: string;
    receiptHash: string;
  };
  Settings: undefined;
  Help: undefined;
};

export type AuthStackParamList = {
  Login: undefined;
  BiometricAuth: {
    cpf: string;
    accessToken: string;
  };
};

export type VotingStackParamList = {
  ElectionList: undefined;
  Voting: {
    electionId: string;
    electionTitle: string;
  };
  VoteConfirmation: {
    electionId: string;
    candidateId: string;
    candidateName: string;
    candidateNumber: string;
  };
  VoteReceipt: {
    voteId: string;
    electionId: string;
    candidateName: string;
    candidateNumber: string;
    timestamp: string;
    receiptHash: string;
  };
};
