// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @title FortisVoting
 * @dev Sistema de votação eletrônica brasileiro baseado em blockchain
 * @author FORTIS Team
 */
contract FortisVoting is AccessControl, ReentrancyGuard, Pausable {
    using Counters for Counters.Counter;

    // Roles
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    bytes32 public constant MINISTER_ROLE = keccak256("MINISTER_ROLE");
    bytes32 public constant AUDITOR_ROLE = keccak256("AUDITOR_ROLE");
    bytes32 public constant NODE_ROLE = keccak256("NODE_ROLE");

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

    struct Vote {
        uint256 id;
        uint256 electionId;
        address voter;
        uint256 candidateId;
        string encryptedVote;
        string zkProof;
        string nullifier;
        uint256 timestamp;
        bool isVerified;
    }

    // State variables
    Counters.Counter private _electionIds;
    Counters.Counter private _candidateIds;
    Counters.Counter private _voteIds;

    mapping(uint256 => Election) public elections;
    mapping(uint256 => Candidate) public candidates;
    mapping(uint256 => Vote) public votes;
    mapping(uint256 => mapping(uint256 => bool)) public candidateExists;
    mapping(uint256 => mapping(address => bool)) public hasVoted;
    mapping(address => bool) public registeredVoters;
    mapping(string => bool) public nullifiers;

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

    event CandidateAdded(
        uint256 indexed electionId,
        uint256 indexed candidateId,
        string name,
        string party,
        string number
    );

    event VoteCast(
        uint256 indexed electionId,
        uint256 indexed voteId,
        address indexed voter,
        uint256 candidateId,
        string nullifier
    );

    event VoterRegistered(address indexed voter, address registeredBy);
    event VoterRemoved(address indexed voter);
    event ElectionCompleted(uint256 indexed electionId, string merkleRoot, string ipfsHash);

    // Modifiers
    modifier onlyActiveElection(uint256 _electionId) {
        require(elections[_electionId].isActive, "Election is not active");
        require(block.timestamp >= elections[_electionId].startTime, "Election has not started");
        require(block.timestamp <= elections[_electionId].endTime, "Election has ended");
        _;
    }

    modifier onlyRegisteredVoter() {
        require(registeredVoters[msg.sender], "Voter not registered");
        _;
    }

    modifier onlyValidElection(uint256 _electionId) {
        require(_electionId > 0 && _electionId <= _electionIds.current(), "Invalid election ID");
        _;
    }

    modifier onlyValidCandidate(uint256 _candidateId) {
        require(_candidateId > 0 && _candidateId <= _candidateIds.current(), "Invalid candidate ID");
        _;
    }

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE, msg.sender);
    }

    /**
     * @dev Create a new election
     * @param _title Election title
     * @param _description Election description
     * @param _startTime Election start timestamp
     * @param _endTime Election end timestamp
     */
    function createElection(
        string memory _title,
        string memory _description,
        uint256 _startTime,
        uint256 _endTime
    ) external returns (uint256) {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        require(_startTime > block.timestamp, "Start time must be in the future");
        require(_endTime > _startTime, "End time must be after start time");
        require(bytes(_title).length > 0, "Title cannot be empty");

        _electionIds.increment();
        uint256 electionId = _electionIds.current();

        elections[electionId] = Election({
            id: electionId,
            title: _title,
            description: _description,
            startTime: _startTime,
            endTime: _endTime,
            isActive: false,
            isCompleted: false,
            totalVotes: 0,
            merkleRoot: "",
            ipfsHash: "",
            createdBy: msg.sender,
            createdAt: block.timestamp
        });

        emit ElectionCreated(electionId, _title, _startTime, _endTime, msg.sender);
        return electionId;
    }

    /**
     * @dev Activate an election
     * @param _electionId Election ID
     */
    function activateElection(uint256 _electionId) 
        external 
        onlyValidElection(_electionId) 
    {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        require(!elections[_electionId].isActive, "Election already active");
        require(!elections[_electionId].isCompleted, "Election already completed");
        require(block.timestamp >= elections[_electionId].startTime, "Election start time not reached");

        elections[_electionId].isActive = true;

        emit ElectionUpdated(_electionId, elections[_electionId].title, true, false);
    }

    /**
     * @dev Complete an election
     * @param _electionId Election ID
     * @param _merkleRoot Merkle root of all votes
     * @param _ipfsHash IPFS hash of election data
     */
    function completeElection(
        uint256 _electionId,
        string memory _merkleRoot,
        string memory _ipfsHash
    ) 
        external 
        onlyValidElection(_electionId) 
    {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        require(elections[_electionId].isActive, "Election not active");
        require(block.timestamp >= elections[_electionId].endTime, "Election not ended");

        elections[_electionId].isActive = false;
        elections[_electionId].isCompleted = true;
        elections[_electionId].merkleRoot = _merkleRoot;
        elections[_electionId].ipfsHash = _ipfsHash;

        emit ElectionCompleted(_electionId, _merkleRoot, _ipfsHash);
    }

    /**
     * @dev Add a candidate to an election
     * @param _electionId Election ID
     * @param _name Candidate name
     * @param _party Candidate party
     * @param _position Candidate position
     * @param _number Candidate number
     * @param _photoUrl Candidate photo URL
     * @param _bio Candidate biography
     */
    function addCandidate(
        uint256 _electionId,
        string memory _name,
        string memory _party,
        string memory _position,
        string memory _number,
        string memory _photoUrl,
        string memory _bio
    ) 
        external 
        onlyValidElection(_electionId) 
    {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        require(!elections[_electionId].isCompleted, "Cannot add candidates to completed election");
        require(bytes(_name).length > 0, "Name cannot be empty");
        require(bytes(_number).length > 0, "Number cannot be empty");

        _candidateIds.increment();
        uint256 candidateId = _candidateIds.current();

        candidates[candidateId] = Candidate({
            id: candidateId,
            electionId: _electionId,
            name: _name,
            party: _party,
            position: _position,
            number: _number,
            photoUrl: _photoUrl,
            bio: _bio,
            isActive: true,
            votesCount: 0
        });

        candidateExists[_electionId][candidateId] = true;

        emit CandidateAdded(_electionId, candidateId, _name, _party, _number);
    }

    /**
     * @dev Register a voter
     * @param _voter Voter address
     */
    function registerVoter(address _voter) external {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        require(_voter != address(0), "Invalid voter address");
        require(!registeredVoters[_voter], "Voter already registered");

        registeredVoters[_voter] = true;

        emit VoterRegistered(_voter, msg.sender);
    }

    /**
     * @dev Cast a vote
     * @param _electionId Election ID
     * @param _candidateId Candidate ID
     * @param _encryptedVote Encrypted vote data
     * @param _zkProof Zero-knowledge proof
     * @param _nullifier Nullifier to prevent double voting
     */
    function castVote(
        uint256 _electionId,
        uint256 _candidateId,
        string memory _encryptedVote,
        string memory _zkProof,
        string memory _nullifier
    ) 
        external 
        onlyRegisteredVoter 
        onlyActiveElection(_electionId) 
        onlyValidCandidate(_candidateId) 
        nonReentrant 
    {
        require(!hasVoted[_electionId][msg.sender], "Already voted in this election");
        require(candidateExists[_electionId][_candidateId], "Candidate not in this election");
        require(!nullifiers[_nullifier], "Nullifier already used");
        require(bytes(_nullifier).length > 0, "Nullifier cannot be empty");

        _voteIds.increment();
        uint256 voteId = _voteIds.current();

        votes[voteId] = Vote({
            id: voteId,
            electionId: _electionId,
            voter: msg.sender,
            candidateId: _candidateId,
            encryptedVote: _encryptedVote,
            zkProof: _zkProof,
            nullifier: _nullifier,
            timestamp: block.timestamp,
            isVerified: false
        });

        hasVoted[_electionId][msg.sender] = true;
        nullifiers[_nullifier] = true;
        elections[_electionId].totalVotes++;
        candidates[_candidateId].votesCount++;

        emit VoteCast(_electionId, voteId, msg.sender, _candidateId, _nullifier);
    }

    /**
     * @dev Verify a vote
     * @param _voteId Vote ID
     * @param _isValid Whether the vote is valid
     */
    function verifyVote(uint256 _voteId, bool _isValid) 
        external 
    {
        require(hasRole(AUDITOR_ROLE, msg.sender), "Not authorized");
        require(_voteId > 0 && _voteId <= _voteIds.current(), "Invalid vote ID");
        
        votes[_voteId].isVerified = _isValid;
    }

    /**
     * @dev Get election details
     * @param _electionId Election ID
     */
    function getElection(uint256 _electionId) 
        external 
        view 
        onlyValidElection(_electionId) 
        returns (Election memory) 
    {
        return elections[_electionId];
    }

    /**
     * @dev Get candidate details
     * @param _candidateId Candidate ID
     */
    function getCandidate(uint256 _candidateId) 
        external 
        view 
        onlyValidCandidate(_candidateId) 
        returns (Candidate memory) 
    {
        return candidates[_candidateId];
    }

    /**
     * @dev Get vote details
     * @param _voteId Vote ID
     */
    function getVote(uint256 _voteId) 
        external 
        view 
        returns (Vote memory) 
    {
        require(_voteId > 0 && _voteId <= _voteIds.current(), "Invalid vote ID");
        return votes[_voteId];
    }

    /**
     * @dev Get election results
     * @param _electionId Election ID
     */
    function getElectionResults(uint256 _electionId) 
        external 
        view 
        onlyValidElection(_electionId) 
        returns (uint256[] memory candidateIds, uint256[] memory voteCounts) 
    {
        uint256 candidateCount = 0;
        
        // Count candidates for this election
        for (uint256 i = 1; i <= _candidateIds.current(); i++) {
            if (candidateExists[_electionId][i]) {
                candidateCount++;
            }
        }

        candidateIds = new uint256[](candidateCount);
        voteCounts = new uint256[](candidateCount);

        uint256 index = 0;
        for (uint256 i = 1; i <= _candidateIds.current(); i++) {
            if (candidateExists[_electionId][i]) {
                candidateIds[index] = i;
                voteCounts[index] = candidates[i].votesCount;
                index++;
            }
        }
    }

    /**
     * @dev Get total elections count
     */
    function getElectionsCount() external view returns (uint256) {
        return _electionIds.current();
    }

    /**
     * @dev Get total candidates count
     */
    function getCandidatesCount() external view returns (uint256) {
        return _candidateIds.current();
    }

    /**
     * @dev Get total votes count
     */
    function getVotesCount() external view returns (uint256) {
        return _voteIds.current();
    }

    /**
     * @dev Check if voter has voted in election
     * @param _electionId Election ID
     * @param _voter Voter address
     */
    function hasVoterVoted(uint256 _electionId, address _voter) 
        external 
        view 
        returns (bool) 
    {
        return hasVoted[_electionId][_voter];
    }

    /**
     * @dev Check if voter is registered
     * @param _voter Voter address
     */
    function isVoterRegistered(address _voter) external view returns (bool) {
        return registeredVoters[_voter];
    }

    /**
     * @dev Get total number of elections
     */
    function getElectionCount() external view returns (uint256) {
        return _electionIds.current();
    }

    /**
     * @dev Get total number of candidates
     */
    function getCandidateCount() external view returns (uint256) {
        return _candidateIds.current();
    }

    /**
     * @dev Remove voter from system
     * @param _voter Voter address to remove
     */
    function removeVoter(address _voter) external {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        require(registeredVoters[_voter], "Voter not registered");
        
        registeredVoters[_voter] = false;
        
        emit VoterRemoved(_voter);
    }

    /**
     * @dev Pause the contract
     */
    function pause() external {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        _pause();
    }

    /**
     * @dev Unpause the contract
     */
    function unpause() external {
        require(hasRole(ADMIN_ROLE, msg.sender), "Not authorized");
        _unpause();
    }
}
