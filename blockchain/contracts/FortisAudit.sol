// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title FortisAudit
 * @dev Sistema de auditoria imutável e transparente para eleições
 * @author FORTIS Team
 */
contract FortisAudit is AccessControl, ReentrancyGuard, Pausable {
    // Roles
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    bytes32 public constant AUDITOR_ROLE = keccak256("AUDITOR_ROLE");
    bytes32 public constant MINISTER_ROLE = keccak256("MINISTER_ROLE");

    // Structs
    struct AuditLog {
        uint256 id;
        uint256 electionId;
        address auditor;
        string action;
        string description;
        string dataHash;
        string signature;
        uint256 timestamp;
        bool isVerified;
    }

    struct AuditReport {
        uint256 id;
        uint256 electionId;
        address auditor;
        string reportHash;
        string ipfsHash;
        bool isApproved;
        address approvedBy;
        uint256 createdAt;
        uint256 approvedAt;
    }

    struct MerkleProof {
        uint256 leafIndex;
        bytes32[] proof;
        bytes32 root;
        bytes32 leaf;
    }

    struct VoteVerification {
        uint256 voteId;
        bool isValid;
        string reason;
        address verifiedBy;
        uint256 verifiedAt;
    }

    // State variables
    mapping(uint256 => AuditLog) public auditLogs;
    mapping(uint256 => AuditReport) public auditReports;
    mapping(uint256 => VoteVerification) public voteVerifications;
    mapping(uint256 => mapping(address => bool)) public auditorPermissions;
    mapping(bytes32 => bool) public merkleRoots;
    mapping(string => bool) public usedSignatures;

    uint256 private _auditLogCounter;
    uint256 private _auditReportCounter;

    // Events
    event AuditLogCreated(
        uint256 indexed auditId,
        uint256 indexed electionId,
        address indexed auditor,
        string action,
        string description
    );

    event AuditReportSubmitted(
        uint256 indexed reportId,
        uint256 indexed electionId,
        address indexed auditor,
        string reportHash
    );

    event AuditReportApproved(
        uint256 indexed reportId,
        address indexed approvedBy,
        uint256 approvedAt
    );

    event VoteVerified(
        uint256 indexed voteId,
        bool isValid,
        string reason,
        address verifiedBy
    );

    event MerkleRootRegistered(
        uint256 indexed electionId,
        bytes32 indexed merkleRoot,
        address registeredBy
    );

    event AuditorPermissionGranted(
        uint256 indexed electionId,
        address indexed auditor,
        address grantedBy
    );

    // Modifiers
    modifier onlyAuditorForElection(uint256 _electionId) {
        require(
            hasRole(AUDITOR_ROLE, msg.sender) || auditorPermissions[_electionId][msg.sender],
            "Not authorized auditor for this election"
        );
        _;
    }

    modifier onlyValidAuditId(uint256 _auditId) {
        require(_auditId > 0 && _auditId <= _auditLogCounter, "Invalid audit ID");
        _;
    }

    modifier onlyValidReportId(uint256 _reportId) {
        require(_reportId > 0 && _reportId <= _auditReportCounter, "Invalid report ID");
        _;
    }

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE, msg.sender);
    }

    /**
     * @dev Create an audit log entry
     * @param _electionId Election ID
     * @param _action Action performed
     * @param _description Description of the action
     * @param _dataHash Hash of the data being audited
     * @param _signature Digital signature of the auditor
     */
    function createAuditLog(
        uint256 _electionId,
        string memory _action,
        string memory _description,
        string memory _dataHash,
        string memory _signature
    ) 
        external 
        onlyAuditorForElection(_electionId) 
        nonReentrant 
    {
        require(bytes(_action).length > 0, "Action cannot be empty");
        require(bytes(_description).length > 0, "Description cannot be empty");
        require(bytes(_dataHash).length > 0, "Data hash cannot be empty");
        require(bytes(_signature).length > 0, "Signature cannot be empty");
        require(!usedSignatures[_signature], "Signature already used");

        _auditLogCounter++;
        uint256 auditId = _auditLogCounter;

        auditLogs[auditId] = AuditLog({
            id: auditId,
            electionId: _electionId,
            auditor: msg.sender,
            action: _action,
            description: _description,
            dataHash: _dataHash,
            signature: _signature,
            timestamp: block.timestamp,
            isVerified: false
        });

        usedSignatures[_signature] = true;

        emit AuditLogCreated(auditId, _electionId, msg.sender, _action, _description);
    }

    /**
     * @dev Submit an audit report
     * @param _electionId Election ID
     * @param _reportHash Hash of the audit report
     * @param _ipfsHash IPFS hash of the report
     */
    function submitAuditReport(
        uint256 _electionId,
        string memory _reportHash,
        string memory _ipfsHash
    ) 
        external 
        onlyAuditorForElection(_electionId) 
        nonReentrant 
    {
        require(bytes(_reportHash).length > 0, "Report hash cannot be empty");
        require(bytes(_ipfsHash).length > 0, "IPFS hash cannot be empty");

        _auditReportCounter++;
        uint256 reportId = _auditReportCounter;

        auditReports[reportId] = AuditReport({
            id: reportId,
            electionId: _electionId,
            auditor: msg.sender,
            reportHash: _reportHash,
            ipfsHash: _ipfsHash,
            isApproved: false,
            approvedBy: address(0),
            createdAt: block.timestamp,
            approvedAt: 0
        });

        emit AuditReportSubmitted(reportId, _electionId, msg.sender, _reportHash);
    }

    /**
     * @dev Approve an audit report
     * @param _reportId Report ID
     */
    function approveAuditReport(uint256 _reportId) 
        external 
        onlyRole(MINISTER_ROLE) 
        onlyValidReportId(_reportId) 
    {
        require(!auditReports[_reportId].isApproved, "Report already approved");

        auditReports[_reportId].isApproved = true;
        auditReports[_reportId].approvedBy = msg.sender;
        auditReports[_reportId].approvedAt = block.timestamp;

        emit AuditReportApproved(_reportId, msg.sender, block.timestamp);
    }

    /**
     * @dev Verify a vote
     * @param _voteId Vote ID
     * @param _isValid Whether the vote is valid
     * @param _reason Reason for the verification result
     */
    function verifyVote(
        uint256 _voteId,
        bool _isValid,
        string memory _reason
    ) 
        external 
        onlyRole(AUDITOR_ROLE) 
    {
        require(_voteId > 0, "Invalid vote ID");
        require(bytes(_reason).length > 0, "Reason cannot be empty");

        voteVerifications[_voteId] = VoteVerification({
            voteId: _voteId,
            isValid: _isValid,
            reason: _reason,
            verifiedBy: msg.sender,
            verifiedAt: block.timestamp
        });

        emit VoteVerified(_voteId, _isValid, _reason, msg.sender);
    }

    /**
     * @dev Register a Merkle root for an election
     * @param _electionId Election ID
     * @param _merkleRoot Merkle root hash
     */
    function registerMerkleRoot(uint256 _electionId, bytes32 _merkleRoot) 
        external 
        onlyRole(ADMIN_ROLE) 
    {
        require(_merkleRoot != bytes32(0), "Invalid Merkle root");
        require(!merkleRoots[_merkleRoot], "Merkle root already registered");

        merkleRoots[_merkleRoot] = true;

        emit MerkleRootRegistered(_electionId, _merkleRoot, msg.sender);
    }

    /**
     * @dev Grant auditor permission for a specific election
     * @param _electionId Election ID
     * @param _auditor Auditor address
     */
    function grantAuditorPermission(uint256 _electionId, address _auditor) 
        external 
        onlyRole(ADMIN_ROLE) 
    {
        require(_auditor != address(0), "Invalid auditor address");
        require(!auditorPermissions[_electionId][_auditor], "Permission already granted");

        auditorPermissions[_electionId][_auditor] = true;

        emit AuditorPermissionGranted(_electionId, _auditor, msg.sender);
    }

    /**
     * @dev Verify a Merkle proof
     * @param _proof Merkle proof data
     */
    function verifyMerkleProof(MerkleProof memory _proof) 
        external 
        view 
        returns (bool) 
    {
        require(merkleRoots[_proof.root], "Merkle root not registered");
        
        bytes32 currentHash = _proof.leaf;
        
        for (uint256 i = 0; i < _proof.proof.length; i++) {
            if (_proof.leafIndex % 2 == 0) {
                currentHash = keccak256(abi.encodePacked(currentHash, _proof.proof[i]));
            } else {
                currentHash = keccak256(abi.encodePacked(_proof.proof[i], currentHash));
            }
            _proof.leafIndex /= 2;
        }
        
        return currentHash == _proof.root;
    }

    /**
     * @dev Get audit log
     * @param _auditId Audit log ID
     */
    function getAuditLog(uint256 _auditId) 
        external 
        view 
        onlyValidAuditId(_auditId) 
        returns (AuditLog memory) 
    {
        return auditLogs[_auditId];
    }

    /**
     * @dev Get audit report
     * @param _reportId Report ID
     */
    function getAuditReport(uint256 _reportId) 
        external 
        view 
        onlyValidReportId(_reportId) 
        returns (AuditReport memory) 
    {
        return auditReports[_reportId];
    }

    /**
     * @dev Get vote verification
     * @param _voteId Vote ID
     */
    function getVoteVerification(uint256 _voteId) 
        external 
        view 
        returns (VoteVerification memory) 
    {
        return voteVerifications[_voteId];
    }

    /**
     * @dev Get audit logs for an election
     * @param _electionId Election ID
     * @param _offset Starting index
     * @param _limit Number of logs to return
     */
    function getElectionAuditLogs(
        uint256 _electionId,
        uint256 _offset,
        uint256 _limit
    ) 
        external 
        view 
        returns (AuditLog[] memory) 
    {
        require(_limit > 0 && _limit <= 100, "Invalid limit");
        
        uint256 count = 0;
        uint256 resultCount = 0;
        
        // Count matching logs
        for (uint256 i = 1; i <= _auditLogCounter; i++) {
            if (auditLogs[i].electionId == _electionId) {
                if (count >= _offset) {
                    resultCount++;
                }
                count++;
            }
        }
        
        AuditLog[] memory result = new AuditLog[](resultCount);
        uint256 index = 0;
        count = 0;
        
        // Fill result array
        for (uint256 i = 1; i <= _auditLogCounter; i++) {
            if (auditLogs[i].electionId == _electionId) {
                if (count >= _offset && index < resultCount) {
                    result[index] = auditLogs[i];
                    index++;
                }
                count++;
            }
        }
        
        return result;
    }

    /**
     * @dev Get total audit logs count
     */
    function getAuditLogsCount() external view returns (uint256) {
        return _auditLogCounter;
    }

    /**
     * @dev Get total audit reports count
     */
    function getAuditReportsCount() external view returns (uint256) {
        return _auditReportCounter;
    }

    /**
     * @dev Check if Merkle root is registered
     * @param _merkleRoot Merkle root to check
     */
    function isMerkleRootRegistered(bytes32 _merkleRoot) external view returns (bool) {
        return merkleRoots[_merkleRoot];
    }

    /**
     * @dev Pause the contract
     */
    function pause() external onlyRole(ADMIN_ROLE) {
        _pause();
    }

    /**
     * @dev Unpause the contract
     */
    function unpause() external onlyRole(ADMIN_ROLE) {
        _unpause();
    }
}
