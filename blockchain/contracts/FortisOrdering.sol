// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * @title FortisOrdering
 * @dev Sistema de Ordenação de Eventos Críticos - Blockchain Minimalista
 * 
 * Este contrato implementa os princípios do Prof. Marcos Simplicio:
 * - Blockchain é para ORDENAÇÃO de eventos, não validação de conteúdo
 * - Usar blockchain apenas onde a ordenação é ESSENCIAL
 * - Evitar replicação desnecessária de dados
 * 
 * @author FORTIS Team
 */
contract FortisOrdering is AccessControl, ReentrancyGuard {
    
    // Roles - Apenas 27 nós TSE autorizados
    bytes32 public constant TSE_NODE_ROLE = keccak256("TSE_NODE_ROLE");
    bytes32 public constant AUDITOR_ROLE = keccak256("AUDITOR_ROLE");
    
    // Eventos críticos que REQUEREM ordenação global
    enum CriticalEventType {
        ELECTION_CREATED,    // Criação de eleição
        ELECTION_STARTED,    // Início de eleição
        ELECTION_ENDED,      // Fim de eleição
        AUDIT_TRIGGERED,     // Auditoria iniciada
        SECURITY_ALERT,      // Alerta de segurança
        SYSTEM_MAINTENANCE,  // Manutenção do sistema
        NODE_SYNC,           // Sincronização de nó
        EMERGENCY_STOP       // Parada de emergência
    }
    
    // Estrutura de evento crítico
    struct CriticalEvent {
        uint256 eventId;
        CriticalEventType eventType;
        string eventHash;        // Hash do evento (não os dados)
        uint256 timestamp;
        address nodeAddress;     // Nó que registrou o evento
        string merkleRoot;       // Root do Merkle tree do evento
        string ipfsHash;         // Hash IPFS dos dados (se necessário)
        bool isVerified;
        uint256 blockNumber;
    }
    
    // Estado do contrato
    uint256 private _eventCounter;
    mapping(uint256 => CriticalEvent) public criticalEvents;
    mapping(CriticalEventType => uint256[]) public eventsByType;
    mapping(string => bool) public eventHashes; // Prevenir duplicatas
    
    // Estatísticas
    mapping(address => uint256) public nodeEventCount;
    mapping(CriticalEventType => uint256) public eventTypeCount;
    
    // Eventos
    event CriticalEventRecorded(
        uint256 indexed eventId,
        CriticalEventType indexed eventType,
        string eventHash,
        address indexed nodeAddress,
        uint256 timestamp
    );
    
    event EventVerified(
        uint256 indexed eventId,
        address indexed verifier,
        bool isValid
    );
    
    event NodeRegistered(
        address indexed nodeAddress,
        string nodeId,
        uint256 timestamp
    );
    
    // Modificadores
    modifier onlyTSENode() {
        require(hasRole(TSE_NODE_ROLE, msg.sender), "Only TSE nodes can perform this action");
        _;
    }
    
    modifier onlyValidEventType(CriticalEventType _eventType) {
        require(uint8(_eventType) <= uint8(CriticalEventType.EMERGENCY_STOP), "Invalid event type");
        _;
    }
    
    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(TSE_NODE_ROLE, msg.sender);
    }
    
    /**
     * @dev Registra um evento crítico que requer ordenação global
     * @param _eventType Tipo do evento crítico
     * @param _eventHash Hash do evento (não os dados completos)
     * @param _merkleRoot Root do Merkle tree do evento
     * @param _ipfsHash Hash IPFS dos dados (opcional)
     */
    function recordCriticalEvent(
        CriticalEventType _eventType,
        string memory _eventHash,
        string memory _merkleRoot,
        string memory _ipfsHash
    ) 
        external 
        onlyTSENode 
        onlyValidEventType(_eventType)
        nonReentrant 
    {
        // Verificar se evento já foi registrado
        require(!eventHashes[_eventHash], "Event already recorded");
        
        // Incrementar contador
        _eventCounter++;
        
        // Criar evento crítico
        CriticalEvent memory newEvent = CriticalEvent({
            eventId: _eventCounter,
            eventType: _eventType,
            eventHash: _eventHash,
            timestamp: block.timestamp,
            nodeAddress: msg.sender,
            merkleRoot: _merkleRoot,
            ipfsHash: _ipfsHash,
            isVerified: false,
            blockNumber: block.number
        });
        
        // Armazenar evento
        criticalEvents[_eventCounter] = newEvent;
        eventsByType[_eventType].push(_eventCounter);
        eventHashes[_eventHash] = true;
        
        // Atualizar estatísticas
        nodeEventCount[msg.sender]++;
        eventTypeCount[_eventType]++;
        
        // Emitir evento
        emit CriticalEventRecorded(
            _eventCounter,
            _eventType,
            _eventHash,
            msg.sender,
            block.timestamp
        );
    }
    
    /**
     * @dev Verifica um evento crítico (apenas auditores)
     * @param _eventId ID do evento
     * @param _isValid Se o evento é válido
     */
    function verifyEvent(uint256 _eventId, bool _isValid) 
        external 
        onlyRole(AUDITOR_ROLE) 
    {
        require(_eventId > 0 && _eventId <= _eventCounter, "Invalid event ID");
        
        criticalEvents[_eventId].isVerified = _isValid;
        
        emit EventVerified(_eventId, msg.sender, _isValid);
    }
    
    /**
     * @dev Registra um nó TSE
     * @param _nodeAddress Endereço do nó
     * @param _nodeId ID do nó
     */
    function registerTSENode(address _nodeAddress, string memory _nodeId) 
        external 
        onlyRole(DEFAULT_ADMIN_ROLE) 
    {
        _grantRole(TSE_NODE_ROLE, _nodeAddress);
        
        emit NodeRegistered(_nodeAddress, _nodeId, block.timestamp);
    }
    
    /**
     * @dev Obtém evento crítico por ID
     * @param _eventId ID do evento
     */
    function getCriticalEvent(uint256 _eventId) 
        external 
        view 
        returns (CriticalEvent memory) 
    {
        require(_eventId > 0 && _eventId <= _eventCounter, "Invalid event ID");
        return criticalEvents[_eventId];
    }
    
    /**
     * @dev Obtém eventos por tipo
     * @param _eventType Tipo do evento
     * @param _offset Offset para paginação
     * @param _limit Limite de resultados
     */
    function getEventsByType(
        CriticalEventType _eventType, 
        uint256 _offset, 
        uint256 _limit
    ) 
        external 
        view 
        returns (uint256[] memory eventIds, CriticalEvent[] memory events) 
    {
        uint256[] storage typeEvents = eventsByType[_eventType];
        uint256 totalEvents = typeEvents.length;
        
        if (_offset >= totalEvents) {
            return (new uint256[](0), new CriticalEvent[](0));
        }
        
        uint256 endIndex = _offset + _limit;
        if (endIndex > totalEvents) {
            endIndex = totalEvents;
        }
        
        uint256 resultCount = endIndex - _offset;
        eventIds = new uint256[](resultCount);
        events = new CriticalEvent[](resultCount);
        
        for (uint256 i = 0; i < resultCount; i++) {
            uint256 eventIndex = typeEvents[_offset + i];
            eventIds[i] = eventIndex;
            events[i] = criticalEvents[eventIndex];
        }
    }
    
    /**
     * @dev Obtém eventos em um intervalo de tempo
     * @param _startTime Timestamp inicial
     * @param _endTime Timestamp final
     */
    function getEventsByTimeRange(
        uint256 _startTime, 
        uint256 _endTime
    ) 
        external 
        view 
        returns (uint256[] memory eventIds, CriticalEvent[] memory events) 
    {
        uint256[] memory matchingEvents = new uint256[](_eventCounter);
        uint256 matchCount = 0;
        
        for (uint256 i = 1; i <= _eventCounter; i++) {
            CriticalEvent memory event = criticalEvents[i];
            if (event.timestamp >= _startTime && event.timestamp <= _endTime) {
                matchingEvents[matchCount] = i;
                matchCount++;
            }
        }
        
        eventIds = new uint256[](matchCount);
        events = new CriticalEvent[](matchCount);
        
        for (uint256 i = 0; i < matchCount; i++) {
            eventIds[i] = matchingEvents[i];
            events[i] = criticalEvents[matchingEvents[i]];
        }
    }
    
    /**
     * @dev Obtém estatísticas do sistema
     */
    function getSystemStats() 
        external 
        view 
        returns (
            uint256 totalEvents,
            uint256 verifiedEvents,
            uint256 totalNodes,
            mapping(CriticalEventType => uint256) memory eventsByTypeCount
        ) 
    {
        totalEvents = _eventCounter;
        verifiedEvents = 0;
        
        for (uint256 i = 1; i <= _eventCounter; i++) {
            if (criticalEvents[i].isVerified) {
                verifiedEvents++;
            }
        }
        
        totalNodes = getRoleMemberCount(TSE_NODE_ROLE);
        eventsByTypeCount = eventTypeCount;
    }
    
    /**
     * @dev Obtém contagem de eventos por nó
     * @param _nodeAddress Endereço do nó
     */
    function getNodeEventCount(address _nodeAddress) 
        external 
        view 
        returns (uint256) 
    {
        return nodeEventCount[_nodeAddress];
    }
    
    /**
     * @dev Obtém total de eventos registrados
     */
    function getTotalEvents() external view returns (uint256) {
        return _eventCounter;
    }
    
    /**
     * @dev Verifica se um hash de evento já foi registrado
     * @param _eventHash Hash do evento
     */
    function isEventHashRegistered(string memory _eventHash) 
        external 
        view 
        returns (bool) 
    {
        return eventHashes[_eventHash];
    }
    
    /**
     * @dev Obtém eventos não verificados
     */
    function getUnverifiedEvents() 
        external 
        view 
        returns (uint256[] memory eventIds, CriticalEvent[] memory events) 
    {
        uint256[] memory unverifiedEvents = new uint256[](_eventCounter);
        uint256 unverifiedCount = 0;
        
        for (uint256 i = 1; i <= _eventCounter; i++) {
            if (!criticalEvents[i].isVerified) {
                unverifiedEvents[unverifiedCount] = i;
                unverifiedCount++;
            }
        }
        
        eventIds = new uint256[](unverifiedCount);
        events = new CriticalEvent[](unverifiedCount);
        
        for (uint256 i = 0; i < unverifiedCount; i++) {
            eventIds[i] = unverifiedEvents[i];
            events[i] = criticalEvents[unverifiedEvents[i]];
        }
    }
    
    /**
     * @dev Obtém último evento de um tipo específico
     * @param _eventType Tipo do evento
     */
    function getLastEventOfType(CriticalEventType _eventType) 
        external 
        view 
        returns (CriticalEvent memory) 
    {
        uint256[] storage typeEvents = eventsByType[_eventType];
        if (typeEvents.length == 0) {
            revert("No events of this type found");
        }
        
        uint256 lastEventId = typeEvents[typeEvents.length - 1];
        return criticalEvents[lastEventId];
    }
}
