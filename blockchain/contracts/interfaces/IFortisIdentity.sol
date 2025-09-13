// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title IFortisIdentity
 * @dev Interface para o contrato FortisIdentity
 * @author FORTIS Team
 */
interface IFortisIdentity {
    // Events
    event VoterRegistered(
        address indexed voter,
        string cpfHash,
        string nameEncrypted,
        bool isEligible,
        uint256 timestamp
    );
    
    event VoterUpdated(
        address indexed voter,
        string cpfHash,
        bool isEligible,
        uint256 timestamp
    );
    
    event VoterRemoved(address indexed voter);
    
    event CertificateValidated(
        address indexed voter,
        string certificateHash,
        bool isValid,
        uint256 timestamp
    );
    
    event BiometricDataUpdated(
        address indexed voter,
        string biometricHash,
        uint256 timestamp
    );

    // Structs
    struct VoterIdentity {
        address voter;
        string cpfHash;
        string nameEncrypted;
        bool isEligible;
        string certificateHash;
        string biometricHash;
        uint256 registeredAt;
        uint256 lastUpdated;
        bool isActive;
    }

    struct CertificateData {
        string certificateHash;
        string publicKey;
        string issuer;
        uint256 validFrom;
        uint256 validTo;
        bool isRevoked;
    }

    struct BiometricData {
        string fingerprintHash;
        string facialHash;
        string voiceHash;
        uint256 lastUpdated;
    }

    // Functions
    function registerVoter(
        address _voter,
        string memory _cpfHash,
        string memory _nameEncrypted,
        string memory _certificateHash,
        string memory _biometricHash
    ) external;

    function updateVoter(
        address _voter,
        string memory _cpfHash,
        bool _isEligible
    ) external;

    function removeVoter(address _voter) external;
    
    function validateCertificate(
        address _voter,
        string memory _certificateHash
    ) external returns (bool);

    function updateBiometricData(
        address _voter,
        string memory _biometricHash
    ) external;

    function isVoterEligible(address _voter) external view returns (bool);
    function isVoterRegistered(address _voter) external view returns (bool);
    function getVoterIdentity(address _voter) external view returns (VoterIdentity memory);
    function getCertificateData(string memory _certificateHash) external view returns (CertificateData memory);
    function getBiometricData(address _voter) external view returns (BiometricData memory);
    
    function getVoterCount() external view returns (uint256);
    function getEligibleVoterCount() external view returns (uint256);
}
