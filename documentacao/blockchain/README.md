# FORTIS - Smart Contracts e Blockchain
## Backend Architect Perspective

### 🎯 **Visão Geral da Blockchain**

O FORTIS implementa uma solução blockchain híbrida baseada em Polygon (Ethereum L2) com smart contracts Solidity para garantir transparência, imutabilidade e auditabilidade total do processo eleitoral brasileiro.

---

## 🏗️ **Arquitetura Blockchain**

### **1. Camada de Blockchain**
```
┌─────────────────────────────────────────────────────────┐
│                   BLOCKCHAIN LAYER                      │
├─────────────────────────────────────────────────────────┤
│ • Polygon Mainnet (Ethereum L2)                         │
│ • Smart Contracts Solidity                              │
│ • IPFS para armazenamento imutável                      │
│ • Zero-Knowledge Proofs (SnarkJS)                       │
│ • Multi-signature wallets                               │
│ • Cross-chain bridges (Ethereum ↔ Polygon)              │
└─────────────────────────────────────────────────────────┘
```

### **2. Contratos Principais**
- **FortisVoting.sol** - Lógica principal de votação
- **FortisElection.sol** - Gestão de eleições
- **FortisAudit.sol** - Auditoria e verificação
- **FortisIdentity.sol** - Identidade digital
- **FortisGovernance.sol** - Governança do sistema

---

## 🗳️ **Contrato Principal: FortisVoting.sol**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

contract FortisVoting is ReentrancyGuard, Ownable {
    using ECDSA for bytes32;
    
    // Eventos
    event ElectionCreated(
        bytes32 indexed electionId,
        string name,
        uint256 startTime,
        uint256 endTime,
        address creator
    );
    
    event VoteCast(
        bytes32 indexed electionId,
        bytes32 indexed voteId,
        address indexed voter,
        uint256 candidateId,
        bytes32 zkProof,
        uint256 timestamp
    );
    
    event ElectionResults(
        bytes32 indexed electionId,
        uint256[] candidateVotes,
        uint256 totalVotes,
        bytes32 merkleRoot
    );
    
    // Estruturas
    struct Election {
        bytes32 id;
        string name;
        string description;
        uint256 startTime;
        uint256 endTime;
        bool active;
        bool resultsPublished;
        address creator;
        mapping(uint256 => Candidate) candidates;
        uint256 candidateCount;
        mapping(address => bool) voters;
        uint256 totalVotes;
        bytes32 merkleRoot;
    }
    
    struct Candidate {
        uint256 id;
        string name;
        string party;
        string position;
        uint256 number;
        uint256 votes;
        bool active;
    }
    
    struct Vote {
        bytes32 id;
        bytes32 electionId;
        address voter;
        uint256 candidateId;
        bytes32 zkProof;
        uint256 timestamp;
        bool verified;
    }
    
    // Estado do contrato
    mapping(bytes32 => Election) public elections;
    mapping(bytes32 => Vote) public votes;
    mapping(address => bool) public authorizedNodes;
    mapping(address => bool) public auditors;
    
    uint256 public totalElections;
    uint256 public totalVotes;
    
    // Modificadores
    modifier onlyAuthorizedNode() {
        require(authorizedNodes[msg.sender], "Not authorized node");
        _;
    }
    
    modifier onlyAuditor() {
        require(auditors[msg.sender], "Not authorized auditor");
        _;
    }
    
    modifier electionActive(bytes32 _electionId) {
        require(elections[_electionId].active, "Election not active");
        require(
            block.timestamp >= elections[_electionId].startTime &&
            block.timestamp <= elections[_electionId].endTime,
            "Election not in voting period"
        );
        _;
    }
    
    // Funções principais
    
    /**
     * @dev Criar nova eleição
     * @param _name Nome da eleição
     * @param _description Descrição da eleição
     * @param _startTime Timestamp de início
     * @param _endTime Timestamp de fim
     * @param _candidates Array de candidatos
     */
    function createElection(
        string memory _name,
        string memory _description,
        uint256 _startTime,
        uint256 _endTime,
        Candidate[] memory _candidates
    ) external onlyOwner returns (bytes32) {
        require(_startTime > block.timestamp, "Start time must be in future");
        require(_endTime > _startTime, "End time must be after start time");
        require(_candidates.length > 0, "Must have at least one candidate");
        
        bytes32 electionId = keccak256(abi.encodePacked(
            _name,
            _startTime,
            _endTime,
            block.timestamp
        ));
        
        Election storage election = elections[electionId];
        election.id = electionId;
        election.name = _name;
        election.description = _description;
        election.startTime = _startTime;
        election.endTime = _endTime;
        election.active = true;
        election.creator = msg.sender;
        
        for (uint256 i = 0; i < _candidates.length; i++) {
            election.candidates[_candidates[i].id] = _candidates[i];
            election.candidateCount++;
        }
        
        totalElections++;
        
        emit ElectionCreated(electionId, _name, _startTime, _endTime, msg.sender);
        
        return electionId;
    }
    
    /**
     * @dev Registrar voto
     * @param _electionId ID da eleição
     * @param _candidateId ID do candidato
     * @param _zkProof Prova Zero-Knowledge
     * @param _voterSignature Assinatura do eleitor
     */
    function castVote(
        bytes32 _electionId,
        uint256 _candidateId,
        bytes32 _zkProof,
        bytes memory _voterSignature
    ) external 
        nonReentrant 
        electionActive(_electionId) 
        returns (bytes32) 
    {
        require(!elections[_electionId].voters[msg.sender], "Already voted");
        require(
            elections[_electionId].candidates[_candidateId].active,
            "Candidate not active"
        );
        
        // Verificar assinatura do eleitor
        bytes32 messageHash = keccak256(abi.encodePacked(
            _electionId,
            _candidateId,
            msg.sender,
            block.timestamp
        ));
        address signer = messageHash.recover(_voterSignature);
        require(signer == msg.sender, "Invalid signature");
        
        // Criar voto
        bytes32 voteId = keccak256(abi.encodePacked(
            _electionId,
            msg.sender,
            block.timestamp,
            block.number
        ));
        
        Vote storage vote = votes[voteId];
        vote.id = voteId;
        vote.electionId = _electionId;
        vote.voter = msg.sender;
        vote.candidateId = _candidateId;
        vote.zkProof = _zkProof;
        vote.timestamp = block.timestamp;
        vote.verified = true;
        
        // Atualizar estado
        elections[_electionId].voters[msg.sender] = true;
        elections[_electionId].candidates[_candidateId].votes++;
        elections[_electionId].totalVotes++;
        totalVotes++;
        
        emit VoteCast(_electionId, voteId, msg.sender, _candidateId, _zkProof, block.timestamp);
        
        return voteId;
    }
    
    /**
     * @dev Verificar voto
     * @param _voteId ID do voto
     * @return vote Dados do voto
     */
    function getVote(bytes32 _voteId) external view returns (Vote memory) {
        require(votes[_voteId].id != bytes32(0), "Vote not found");
        return votes[_voteId];
    }
    
    /**
     * @dev Obter resultados da eleição
     * @param _electionId ID da eleição
     * @return candidateVotes Array com votos por candidato
     * @return totalVotes Total de votos
     */
    function getElectionResults(bytes32 _electionId) 
        external 
        view 
        returns (uint256[] memory candidateVotes, uint256 totalVotes) 
    {
        require(elections[_electionId].id != bytes32(0), "Election not found");
        require(
            block.timestamp > elections[_electionId].endTime,
            "Election not finished"
        );
        
        Election storage election = elections[_electionId];
        candidateVotes = new uint256[](election.candidateCount);
        
        for (uint256 i = 0; i < election.candidateCount; i++) {
            candidateVotes[i] = election.candidates[i].votes;
        }
        
        totalVotes = election.totalVotes;
    }
    
    /**
     * @dev Publicar resultados (apenas após eleição)
     * @param _electionId ID da eleição
     * @param _merkleRoot Root da árvore Merkle
     */
    function publishResults(
        bytes32 _electionId,
        bytes32 _merkleRoot
    ) external onlyOwner {
        require(elections[_electionId].id != bytes32(0), "Election not found");
        require(
            block.timestamp > elections[_electionId].endTime,
            "Election not finished"
        );
        require(!elections[_electionId].resultsPublished, "Results already published");
        
        elections[_electionId].resultsPublished = true;
        elections[_electionId].merkleRoot = _merkleRoot;
        
        // Emitir evento com resultados
        uint256[] memory candidateVotes = new uint256[](elections[_electionId].candidateCount);
        for (uint256 i = 0; i < elections[_electionId].candidateCount; i++) {
            candidateVotes[i] = elections[_electionId].candidates[i].votes;
        }
        
        emit ElectionResults(
            _electionId,
            candidateVotes,
            elections[_electionId].totalVotes,
            _merkleRoot
        );
    }
    
    /**
     * @dev Verificar integridade do voto usando Merkle Proof
     * @param _voteId ID do voto
     * @param _proof Prova Merkle
     * @return isValid Se o voto é válido
     */
    function verifyVoteIntegrity(
        bytes32 _voteId,
        bytes32[] memory _proof
    ) external view returns (bool isValid) {
        require(votes[_voteId].id != bytes32(0), "Vote not found");
        
        Vote memory vote = votes[_voteId];
        bytes32 leaf = keccak256(abi.encodePacked(
            vote.id,
            vote.electionId,
            vote.voter,
            vote.candidateId,
            vote.timestamp
        ));
        
        return MerkleProof.verify(_proof, elections[vote.electionId].merkleRoot, leaf);
    }
    
    // Funções administrativas
    
    /**
     * @dev Autorizar nó da rede
     * @param _node Endereço do nó
     */
    function authorizeNode(address _node) external onlyOwner {
        authorizedNodes[_node] = true;
    }
    
    /**
     * @dev Desautorizar nó da rede
     * @param _node Endereço do nó
     */
    function deauthorizeNode(address _node) external onlyOwner {
        authorizedNodes[_node] = false;
    }
    
    /**
     * @dev Adicionar auditor
     * @param _auditor Endereço do auditor
     */
    function addAuditor(address _auditor) external onlyOwner {
        auditors[_auditor] = true;
    }
    
    /**
     * @dev Remover auditor
     * @param _auditor Endereço do auditor
     */
    function removeAuditor(address _auditor) external onlyOwner {
        auditors[_auditor] = false;
    }
    
    // Funções de auditoria
    
    /**
     * @dev Auditoria completa da eleição
     * @param _electionId ID da eleição
     * @return auditData Dados de auditoria
     */
    function auditElection(bytes32 _electionId) 
        external 
        view 
        onlyAuditor 
        returns (
            uint256 totalVotes,
            uint256 uniqueVoters,
            uint256 candidateCount,
            bool resultsPublished,
            bytes32 merkleRoot
        ) 
    {
        require(elections[_electionId].id != bytes32(0), "Election not found");
        
        Election storage election = elections[_electionId];
        
        return (
            election.totalVotes,
            election.totalVotes, // Assumindo 1 voto por eleitor
            election.candidateCount,
            election.resultsPublished,
            election.merkleRoot
        );
    }
}
```

---

## 🔐 **Contrato de Identidade: FortisIdentity.sol**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract FortisIdentity is Ownable {
    using ECDSA for bytes32;
    
    // Eventos
    event IdentityRegistered(
        address indexed user,
        string cpf,
        bytes32 biometricHash,
        uint256 timestamp
    );
    
    event IdentityVerified(
        address indexed user,
        bool verified,
        uint256 timestamp
    );
    
    // Estruturas
    struct Identity {
        string cpf;
        bytes32 biometricHash;
        bool verified;
        bool active;
        uint256 registrationTime;
        uint256 lastVerification;
    }
    
    // Estado do contrato
    mapping(address => Identity) public identities;
    mapping(string => address) public cpfToAddress;
    mapping(address => bool) public authorizedValidators;
    
    uint256 public totalIdentities;
    
    // Modificadores
    modifier onlyAuthorizedValidator() {
        require(authorizedValidators[msg.sender], "Not authorized validator");
        _;
    }
    
    /**
     * @dev Registrar identidade digital
     * @param _cpf CPF do usuário
     * @param _biometricHash Hash biométrico
     * @param _signature Assinatura digital
     */
    function registerIdentity(
        string memory _cpf,
        bytes32 _biometricHash,
        bytes memory _signature
    ) external {
        require(identities[msg.sender].registrationTime == 0, "Identity already registered");
        require(cpfToAddress[_cpf] == address(0), "CPF already registered");
        
        // Verificar assinatura
        bytes32 messageHash = keccak256(abi.encodePacked(
            _cpf,
            _biometricHash,
            msg.sender,
            block.timestamp
        ));
        address signer = messageHash.recover(_signature);
        require(signer == msg.sender, "Invalid signature");
        
        identities[msg.sender] = Identity({
            cpf: _cpf,
            biometricHash: _biometricHash,
            verified: false,
            active: true,
            registrationTime: block.timestamp,
            lastVerification: 0
        });
        
        cpfToAddress[_cpf] = msg.sender;
        totalIdentities++;
        
        emit IdentityRegistered(msg.sender, _cpf, _biometricHash, block.timestamp);
    }
    
    /**
     * @dev Verificar identidade
     * @param _user Endereço do usuário
     * @param _biometricHash Hash biométrico atual
     * @param _validatorSignature Assinatura do validador
     */
    function verifyIdentity(
        address _user,
        bytes32 _biometricHash,
        bytes memory _validatorSignature
    ) external onlyAuthorizedValidator {
        require(identities[_user].registrationTime > 0, "Identity not registered");
        require(identities[_user].active, "Identity not active");
        
        // Verificar assinatura do validador
        bytes32 messageHash = keccak256(abi.encodePacked(
            _user,
            _biometricHash,
            block.timestamp
        ));
        address signer = messageHash.recover(_validatorSignature);
        require(authorizedValidators[signer], "Invalid validator signature");
        
        // Verificar hash biométrico
        require(
            identities[_user].biometricHash == _biometricHash,
            "Biometric hash mismatch"
        );
        
        identities[_user].verified = true;
        identities[_user].lastVerification = block.timestamp;
        
        emit IdentityVerified(_user, true, block.timestamp);
    }
    
    /**
     * @dev Obter identidade
     * @param _user Endereço do usuário
     * @return identity Dados da identidade
     */
    function getIdentity(address _user) external view returns (Identity memory) {
        return identities[_user];
    }
    
    /**
     * @dev Autorizar validador
     * @param _validator Endereço do validador
     */
    function authorizeValidator(address _validator) external onlyOwner {
        authorizedValidators[_validator] = true;
    }
}
```

---

## 📊 **Contrato de Auditoria: FortisAudit.sol**

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";

contract FortisAudit is Ownable {
    // Eventos
    event AuditLogCreated(
        bytes32 indexed logId,
        string eventType,
        bytes32 indexed electionId,
        address indexed auditor,
        uint256 timestamp
    );
    
    event AuditReportGenerated(
        bytes32 indexed reportId,
        bytes32 indexed electionId,
        address indexed auditor,
        uint256 timestamp
    );
    
    // Estruturas
    struct AuditLog {
        bytes32 id;
        string eventType;
        bytes32 electionId;
        address auditor;
        string description;
        bytes32 dataHash;
        uint256 timestamp;
    }
    
    struct AuditReport {
        bytes32 id;
        bytes32 electionId;
        address auditor;
        uint256 totalVotes;
        uint256 verifiedVotes;
        uint256 integrityScore;
        string findings;
        uint256 timestamp;
    }
    
    // Estado do contrato
    mapping(bytes32 => AuditLog) public auditLogs;
    mapping(bytes32 => AuditReport) public auditReports;
    mapping(address => bool) public authorizedAuditors;
    
    uint256 public totalLogs;
    uint256 public totalReports;
    
    // Modificadores
    modifier onlyAuthorizedAuditor() {
        require(authorizedAuditors[msg.sender], "Not authorized auditor");
        _;
    }
    
    /**
     * @dev Criar log de auditoria
     * @param _eventType Tipo do evento
     * @param _electionId ID da eleição
     * @param _description Descrição do evento
     * @param _dataHash Hash dos dados
     */
    function createAuditLog(
        string memory _eventType,
        bytes32 _electionId,
        string memory _description,
        bytes32 _dataHash
    ) external onlyAuthorizedAuditor returns (bytes32) {
        bytes32 logId = keccak256(abi.encodePacked(
            _eventType,
            _electionId,
            msg.sender,
            block.timestamp
        ));
        
        auditLogs[logId] = AuditLog({
            id: logId,
            eventType: _eventType,
            electionId: _electionId,
            auditor: msg.sender,
            description: _description,
            dataHash: _dataHash,
            timestamp: block.timestamp
        });
        
        totalLogs++;
        
        emit AuditLogCreated(logId, _eventType, _electionId, msg.sender, block.timestamp);
        
        return logId;
    }
    
    /**
     * @dev Gerar relatório de auditoria
     * @param _electionId ID da eleição
     * @param _totalVotes Total de votos
     * @param _verifiedVotes Votos verificados
     * @param _integrityScore Score de integridade
     * @param _findings Achados da auditoria
     */
    function generateAuditReport(
        bytes32 _electionId,
        uint256 _totalVotes,
        uint256 _verifiedVotes,
        uint256 _integrityScore,
        string memory _findings
    ) external onlyAuthorizedAuditor returns (bytes32) {
        bytes32 reportId = keccak256(abi.encodePacked(
            _electionId,
            msg.sender,
            block.timestamp
        ));
        
        auditReports[reportId] = AuditReport({
            id: reportId,
            electionId: _electionId,
            auditor: msg.sender,
            totalVotes: _totalVotes,
            verifiedVotes: _verifiedVotes,
            integrityScore: _integrityScore,
            findings: _findings,
            timestamp: block.timestamp
        });
        
        totalReports++;
        
        emit AuditReportGenerated(reportId, _electionId, msg.sender, block.timestamp);
        
        return reportId;
    }
    
    /**
     * @dev Autorizar auditor
     * @param _auditor Endereço do auditor
     */
    function authorizeAuditor(address _auditor) external onlyOwner {
        authorizedAuditors[_auditor] = true;
    }
}
```

---

## 🚀 **Deploy e Configuração**

### **Scripts de Deploy**
```javascript
// deploy.js
const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  
  console.log("Deploying contracts with account:", deployer.address);
  
  // Deploy FortisVoting
  const FortisVoting = await ethers.getContractFactory("FortisVoting");
  const fortisVoting = await FortisVoting.deploy();
  await fortisVoting.deployed();
  
  console.log("FortisVoting deployed to:", fortisVoting.address);
  
  // Deploy FortisIdentity
  const FortisIdentity = await ethers.getContractFactory("FortisIdentity");
  const fortisIdentity = await FortisIdentity.deploy();
  await fortisIdentity.deployed();
  
  console.log("FortisIdentity deployed to:", fortisIdentity.address);
  
  // Deploy FortisAudit
  const FortisAudit = await ethers.getContractFactory("FortisAudit");
  const fortisAudit = await FortisAudit.deploy();
  await fortisAudit.deployed();
  
  console.log("FortisAudit deployed to:", fortisAudit.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
```

### **Configuração Hardhat**
```javascript
// hardhat.config.js
require("@nomicfoundation/hardhat-toolbox");

module.exports = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    polygon: {
      url: process.env.POLYGON_RPC_URL,
      accounts: [process.env.PRIVATE_KEY],
      gasPrice: 30000000000, // 30 gwei
    },
    polygonMumbai: {
      url: process.env.MUMBAI_RPC_URL,
      accounts: [process.env.PRIVATE_KEY],
    }
  },
  etherscan: {
    apiKey: process.env.POLYGONSCAN_API_KEY
  }
};
```

---

## 🧪 **Testes de Smart Contracts**

### **Testes Unitários**
```javascript
// test/FortisVoting.test.js
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("FortisVoting", function () {
  let fortisVoting;
  let owner;
  let voter1;
  let voter2;

  beforeEach(async function () {
    [owner, voter1, voter2] = await ethers.getSigners();
    
    const FortisVoting = await ethers.getContractFactory("FortisVoting");
    fortisVoting = await FortisVoting.deploy();
    await fortisVoting.deployed();
  });

  describe("Election Creation", function () {
    it("Should create election successfully", async function () {
      const candidates = [
        {
          id: 1,
          name: "João Silva",
          party: "PT",
          position: "Presidente",
          number: 13,
          votes: 0,
          active: true
        }
      ];
      
      const tx = await fortisVoting.createElection(
        "Eleição Presidencial 2026",
        "Primeiro turno",
        Math.floor(Date.now() / 1000) + 3600, // 1 hora
        Math.floor(Date.now() / 1000) + 7200, // 2 horas
        candidates
      );
      
      await expect(tx)
        .to.emit(fortisVoting, "ElectionCreated")
        .withArgs(anyValue, "Eleição Presidencial 2026", anyValue, anyValue, owner.address);
    });
  });

  describe("Voting", function () {
    it("Should cast vote successfully", async function () {
      // Setup election first
      // ... (código de setup)
      
      const zkProof = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("zk_proof"));
      const signature = await voter1.signMessage(
        ethers.utils.arrayify(
          ethers.utils.keccak256(
            ethers.utils.defaultAbiCoder.encode(
              ["bytes32", "uint256", "address", "uint256"],
              [electionId, 1, voter1.address, Math.floor(Date.now() / 1000)]
            )
          )
        )
      );
      
      const tx = await fortisVoting.connect(voter1).castVote(
        electionId,
        1,
        zkProof,
        signature
      );
      
      await expect(tx)
        .to.emit(fortisVoting, "VoteCast")
        .withArgs(electionId, anyValue, voter1.address, 1, zkProof, anyValue);
    });
  });
});
```

---

## 📈 **Monitoramento e Analytics**

### **Métricas Blockchain**
```javascript
// analytics/blockchain-metrics.js
const { ethers } = require("ethers");

class BlockchainMetrics {
  constructor(contractAddress, provider) {
    this.contract = new ethers.Contract(contractAddress, abi, provider);
  }
  
  async getElectionStats(electionId) {
    const results = await this.contract.getElectionResults(electionId);
    return {
      totalVotes: results.totalVotes,
      candidateVotes: results.candidateVotes,
      participationRate: (results.totalVotes / 150000000) * 100
    };
  }
  
  async getVoteVerification(voteId) {
    const vote = await this.contract.getVote(voteId);
    return {
      verified: vote.verified,
      timestamp: vote.timestamp,
      candidateId: vote.candidateId
    };
  }
}
```

---

*Documentação de Blockchain FORTIS - Desenvolvida pelo Backend Architect Agent*
