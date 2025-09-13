// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title FortisIdentity
 * @dev Sistema de identidade digital para eleitores brasileiros
 * @author FORTIS Team
 */
contract FortisIdentity is AccessControl, ReentrancyGuard, Pausable {
    // Roles
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    bytes32 public constant TSE_ROLE = keccak256("TSE_ROLE");
    bytes32 public constant AUDITOR_ROLE = keccak256("AUDITOR_ROLE");

    // Structs
    struct VoterIdentity {
        address voterAddress;
        string cpf;
        string name;
        string birthDate;
        string documentHash;
        bool isActive;
        bool isEligible;
        uint256 registeredAt;
        uint256 lastUpdated;
        address registeredBy;
    }

    struct DigitalCertificate {
        string certificateId;
        address owner;
        string publicKey;
        string certificateData;
        bool isValid;
        uint256 issuedAt;
        uint256 expiresAt;
        address issuedBy;
    }

    struct BiometricData {
        string fingerprintHash;
        string facialHash;
        string voiceHash;
        bool isVerified;
        uint256 lastUpdated;
    }

    // State variables
    mapping(address => VoterIdentity) public voterIdentities;
    mapping(string => address) public cpfToAddress;
    mapping(string => DigitalCertificate) public certificates;
    mapping(address => BiometricData) public biometricData;
    mapping(address => bool) public isRegistered;
    mapping(string => bool) public usedCPFs;
    mapping(string => bool) public usedCertificates;

    // Events
    event VoterRegistered(
        address indexed voter,
        string cpf,
        string name,
        address registeredBy
    );

    event VoterUpdated(
        address indexed voter,
        string cpf,
        string name,
        bool isEligible
    );

    event CertificateIssued(
        string indexed certificateId,
        address indexed owner,
        address issuedBy,
        uint256 expiresAt
    );

    event CertificateRevoked(
        string indexed certificateId,
        address revokedBy
    );

    event BiometricDataUpdated(
        address indexed voter,
        bool fingerprintVerified,
        bool facialVerified,
        bool voiceVerified
    );

    event VoterDeactivated(
        address indexed voter,
        address deactivatedBy
    );

    // Modifiers
    modifier onlyRegisteredVoter(address _voter) {
        require(isRegistered[_voter], "Voter not registered");
        _;
    }

    modifier onlyValidCPF(string memory _cpf) {
        require(bytes(_cpf).length == 11, "Invalid CPF length");
        require(!usedCPFs[_cpf], "CPF already registered");
        _;
    }

    modifier onlyValidCertificate(string memory _certificateId) {
        require(bytes(_certificateId).length > 0, "Certificate ID cannot be empty");
        require(!usedCertificates[_certificateId], "Certificate already used");
        _;
    }

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(ADMIN_ROLE, msg.sender);
    }

    /**
     * @dev Register a new voter
     * @param _voter Voter address
     * @param _cpf CPF number
     * @param _name Full name
     * @param _birthDate Birth date
     * @param _documentHash Hash of identity document
     */
    function registerVoter(
        address _voter,
        string memory _cpf,
        string memory _name,
        string memory _birthDate,
        string memory _documentHash
    ) 
        external 
        onlyRole(TSE_ROLE) 
        onlyValidCPF(_cpf) 
        nonReentrant 
    {
        require(_voter != address(0), "Invalid voter address");
        require(!isRegistered[_voter], "Voter already registered");
        require(bytes(_name).length > 0, "Name cannot be empty");
        require(bytes(_documentHash).length > 0, "Document hash cannot be empty");

        voterIdentities[_voter] = VoterIdentity({
            voterAddress: _voter,
            cpf: _cpf,
            name: _name,
            birthDate: _birthDate,
            documentHash: _documentHash,
            isActive: true,
            isEligible: true,
            registeredAt: block.timestamp,
            lastUpdated: block.timestamp,
            registeredBy: msg.sender
        });

        cpfToAddress[_cpf] = _voter;
        isRegistered[_voter] = true;
        usedCPFs[_cpf] = true;

        emit VoterRegistered(_voter, _cpf, _name, msg.sender);
    }

    /**
     * @dev Update voter eligibility
     * @param _voter Voter address
     * @param _isEligible Whether voter is eligible to vote
     */
    function updateVoterEligibility(
        address _voter,
        bool _isEligible
    ) 
        external 
        onlyRole(TSE_ROLE) 
        onlyRegisteredVoter(_voter) 
    {
        voterIdentities[_voter].isEligible = _isEligible;
        voterIdentities[_voter].lastUpdated = block.timestamp;

        emit VoterUpdated(
            _voter,
            voterIdentities[_voter].cpf,
            voterIdentities[_voter].name,
            _isEligible
        );
    }

    /**
     * @dev Issue a digital certificate
     * @param _certificateId Certificate ID
     * @param _owner Certificate owner
     * @param _publicKey Public key
     * @param _certificateData Certificate data
     * @param _expiresAt Expiration timestamp
     */
    function issueCertificate(
        string memory _certificateId,
        address _owner,
        string memory _publicKey,
        string memory _certificateData,
        uint256 _expiresAt
    ) 
        external 
        onlyRole(TSE_ROLE) 
        onlyValidCertificate(_certificateId) 
        onlyRegisteredVoter(_owner) 
    {
        require(_expiresAt > block.timestamp, "Expiration must be in the future");
        require(bytes(_publicKey).length > 0, "Public key cannot be empty");

        certificates[_certificateId] = DigitalCertificate({
            certificateId: _certificateId,
            owner: _owner,
            publicKey: _publicKey,
            certificateData: _certificateData,
            isValid: true,
            issuedAt: block.timestamp,
            expiresAt: _expiresAt,
            issuedBy: msg.sender
        });

        usedCertificates[_certificateId] = true;

        emit CertificateIssued(_certificateId, _owner, msg.sender, _expiresAt);
    }

    /**
     * @dev Revoke a digital certificate
     * @param _certificateId Certificate ID
     */
    function revokeCertificate(string memory _certificateId) 
        external 
        onlyRole(TSE_ROLE) 
    {
        require(bytes(_certificateId).length > 0, "Certificate ID cannot be empty");
        require(certificates[_certificateId].isValid, "Certificate not valid");

        certificates[_certificateId].isValid = false;

        emit CertificateRevoked(_certificateId, msg.sender);
    }

    /**
     * @dev Update biometric data
     * @param _voter Voter address
     * @param _fingerprintHash Fingerprint hash
     * @param _facialHash Facial recognition hash
     * @param _voiceHash Voice recognition hash
     */
    function updateBiometricData(
        address _voter,
        string memory _fingerprintHash,
        string memory _facialHash,
        string memory _voiceHash
    ) 
        external 
        onlyRole(TSE_ROLE) 
        onlyRegisteredVoter(_voter) 
    {
        require(bytes(_fingerprintHash).length > 0, "Fingerprint hash cannot be empty");
        require(bytes(_facialHash).length > 0, "Facial hash cannot be empty");
        require(bytes(_voiceHash).length > 0, "Voice hash cannot be empty");

        biometricData[_voter] = BiometricData({
            fingerprintHash: _fingerprintHash,
            facialHash: _facialHash,
            voiceHash: _voiceHash,
            isVerified: true,
            lastUpdated: block.timestamp
        });

        emit BiometricDataUpdated(_voter, true, true, true);
    }

    /**
     * @dev Deactivate a voter
     * @param _voter Voter address
     */
    function deactivateVoter(address _voter) 
        external 
        onlyRole(TSE_ROLE) 
        onlyRegisteredVoter(_voter) 
    {
        voterIdentities[_voter].isActive = false;
        voterIdentities[_voter].lastUpdated = block.timestamp;

        emit VoterDeactivated(_voter, msg.sender);
    }

    /**
     * @dev Verify voter identity
     * @param _voter Voter address
     * @param _cpf CPF to verify
     */
    function verifyVoterIdentity(address _voter, string memory _cpf) 
        external 
        view 
        returns (bool) 
    {
        if (!isRegistered[_voter]) return false;
        if (!voterIdentities[_voter].isActive) return false;
        if (!voterIdentities[_voter].isEligible) return false;
        if (keccak256(bytes(voterIdentities[_voter].cpf)) != keccak256(bytes(_cpf))) return false;
        
        return true;
    }

    /**
     * @dev Verify digital certificate
     * @param _certificateId Certificate ID
     * @param _owner Expected owner
     */
    function verifyCertificate(string memory _certificateId, address _owner) 
        external 
        view 
        returns (bool) 
    {
        if (bytes(_certificateId).length == 0) return false;
        if (!certificates[_certificateId].isValid) return false;
        if (certificates[_certificateId].owner != _owner) return false;
        if (block.timestamp > certificates[_certificateId].expiresAt) return false;
        
        return true;
    }

    /**
     * @dev Verify biometric data
     * @param _voter Voter address
     * @param _fingerprintHash Fingerprint hash to verify
     * @param _facialHash Facial hash to verify
     */
    function verifyBiometricData(
        address _voter,
        string memory _fingerprintHash,
        string memory _facialHash
    ) 
        external 
        view 
        returns (bool) 
    {
        if (!isRegistered[_voter]) return false;
        if (!biometricData[_voter].isVerified) return false;
        
        bool fingerprintMatch = keccak256(bytes(biometricData[_voter].fingerprintHash)) == 
                               keccak256(bytes(_fingerprintHash));
        bool facialMatch = keccak256(bytes(biometricData[_voter].facialHash)) == 
                          keccak256(bytes(_facialHash));
        
        return fingerprintMatch && facialMatch;
    }

    /**
     * @dev Get voter identity
     * @param _voter Voter address
     */
    function getVoterIdentity(address _voter) 
        external 
        view 
        onlyRegisteredVoter(_voter) 
        returns (VoterIdentity memory) 
    {
        return voterIdentities[_voter];
    }

    /**
     * @dev Get voter by CPF
     * @param _cpf CPF number
     */
    function getVoterByCPF(string memory _cpf) 
        external 
        view 
        returns (address) 
    {
        return cpfToAddress[_cpf];
    }

    /**
     * @dev Get digital certificate
     * @param _certificateId Certificate ID
     */
    function getCertificate(string memory _certificateId) 
        external 
        view 
        returns (DigitalCertificate memory) 
    {
        return certificates[_certificateId];
    }

    /**
     * @dev Get biometric data
     * @param _voter Voter address
     */
    function getBiometricData(address _voter) 
        external 
        view 
        onlyRegisteredVoter(_voter) 
        returns (BiometricData memory) 
    {
        return biometricData[_voter];
    }

    /**
     * @dev Check if voter is eligible
     * @param _voter Voter address
     */
    function isVoterEligible(address _voter) external view returns (bool) {
        if (!isRegistered[_voter]) return false;
        return voterIdentities[_voter].isActive && voterIdentities[_voter].isEligible;
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
