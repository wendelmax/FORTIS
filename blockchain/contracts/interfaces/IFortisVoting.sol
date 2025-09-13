// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title IFortisVoting
 * @dev Interface para o contrato FortisVoting
 * @author FORTIS Team
 */
interface IFortisVoting {
    // Events
    event ElectionCreated(
        uint256 indexed electionId,
        string title,
        uint256 startTime,
        uint256 endTime,
        address createdBy
    );
    
    event ElectionUpdated(
        uint256 indexed electionId,
        string title,
        bool isActive,
        bool isCompleted
    );
    
    event ElectionCompleted(
        uint256 indexed electionId,
        string merkleRoot,
        string ipfsHash
    );
    
    event CandidateAdded(
        uint256 indexed electionId,
        uint256 indexed candidateId,
        string name,
        string party,
        string number
    );
    
    event VoteCast(
        uint256 indexed electionId,
        uint256 indexed candidateId,
        address voter,
        string encryptedVote,
        string zkProof
    );
    
    event VoterRegistered(address indexed voter);
    event VoterRemoved(address indexed voter);

    // Structs
    struct Election {
        uint256 id;
        string title;
        string description;
        uint256 startTime;
        uint256 endTime;
        bool isActive;
        bool isCompleted;
        uint256 totalVotes;
        string merkleRoot;
        string ipfsHash;
        address createdBy;
        uint256 createdAt;
    }

    struct Candidate {
        uint256 id;
        uint256 electionId;
        string name;
        string party;
        string position;
        string number;
        string photoUrl;
        string bio;
        bool isActive;
        uint256 votesCount;
    }

    // Functions
    function createElection(
        string memory _title,
        string memory _description,
        uint256 _startTime,
        uint256 _endTime
    ) external returns (uint256);

    function activateElection(uint256 _electionId) external;
    
    function completeElection(
        uint256 _electionId,
        string memory _merkleRoot,
        string memory _ipfsHash
    ) external;

    function addCandidate(
        uint256 _electionId,
        string memory _name,
        string memory _party,
        string memory _position,
        string memory _number,
        string memory _photoUrl,
        string memory _bio
    ) external returns (uint256);

    function castVote(
        uint256 _electionId,
        uint256 _candidateId,
        string memory _encryptedVote,
        string memory _zkProof,
        string memory _nullifier
    ) external;

    function registerVoter(address _voter) external;
    function removeVoter(address _voter) external;

    function getElection(uint256 _electionId) external view returns (Election memory);
    function getCandidate(uint256 _candidateId) external view returns (Candidate memory);
    function getElectionCandidates(uint256 _electionId) external view returns (uint256[] memory);
    function getElectionResults(uint256 _electionId) external view returns (Candidate[] memory);
    
    function isVoterRegistered(address _voter) external view returns (bool);
    function hasVoted(uint256 _electionId, address _voter) external view returns (bool);
    function getElectionCount() external view returns (uint256);
    function getCandidateCount() external view returns (uint256);
}
