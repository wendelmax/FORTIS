// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title IFortisAudit
 * @dev Interface para o contrato FortisAudit
 * @author FORTIS Team
 */
interface IFortisAudit {
    // Events
    event AuditLogCreated(
        uint256 indexed logId,
        uint256 indexed electionId,
        address auditor,
        string action,
        string description,
        uint256 timestamp
    );
    
    event AuditReportCreated(
        uint256 indexed reportId,
        uint256 indexed electionId,
        address auditor,
        string reportHash,
        string ipfsHash,
        uint256 createdAt
    );
    
    event AuditReportApproved(
        uint256 indexed reportId,
        address approvedBy,
        uint256 approvedAt
    );
    
    event MerkleProofVerified(
        uint256 indexed electionId,
        bytes32 root,
        bool isValid
    );

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

    // Functions
    function createAuditLog(
        uint256 _electionId,
        string memory _action,
        string memory _description,
        string memory _dataHash,
        string memory _signature
    ) external returns (uint256);

    function createAuditReport(
        uint256 _electionId,
        string memory _reportHash,
        string memory _ipfsHash
    ) external returns (uint256);

    function approveAuditReport(uint256 _reportId) external;
    
    function verifyMerkleProof(
        uint256 _electionId,
        MerkleProof memory _proof
    ) external returns (bool);

    function getAuditLog(uint256 _logId) external view returns (AuditLog memory);
    function getAuditReport(uint256 _reportId) external view returns (AuditReport memory);
    function getElectionAuditLogs(uint256 _electionId) external view returns (uint256[] memory);
    function getElectionAuditReports(uint256 _electionId) external view returns (uint256[] memory);
    
    function getAuditLogCount() external view returns (uint256);
    function getAuditReportCount() external view returns (uint256);
}
