// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Votes.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title FortisToken
 * @dev Token ERC20 com funcionalidade de votação para governança do FORTIS
 * @author FORTIS Team
 */
contract FortisToken is ERC20, ERC20Votes, AccessControl, Pausable {
    // Roles
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    // State variables
    uint256 public constant MAX_SUPPLY = 1000000000 * 10**18; // 1 bilhão de tokens
    uint256 public constant INITIAL_SUPPLY = 100000000 * 10**18; // 100 milhões de tokens iniciais
    
    mapping(address => bool) public authorizedMinters;
    mapping(address => uint256) public mintingLimits;
    mapping(address => uint256) public mintedAmount;

    // Events
    event TokensMinted(address indexed to, uint256 amount, address minter);
    event TokensBurned(address indexed from, uint256 amount, address burner);
    event MinterAuthorized(address indexed minter, uint256 limit);
    event MinterDeauthorized(address indexed minter);

    constructor() ERC20("FORTIS Token", "FORTIS") ERC20Permit("FORTIS Token") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
        _grantRole(BURNER_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);
        
        // Mint tokens iniciais para o deployer
        _mint(msg.sender, INITIAL_SUPPLY);
    }

    /**
     * @dev Mint tokens to an address
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     */
    function mint(address to, uint256 amount) 
        external 
        onlyRole(MINTER_ROLE) 
        whenNotPaused 
    {
        require(to != address(0), "Cannot mint to zero address");
        require(amount > 0, "Amount must be greater than 0");
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds maximum supply");
        
        _mint(to, amount);
        emit TokensMinted(to, amount, msg.sender);
    }

    /**
     * @dev Burn tokens from an address
     * @param from Address to burn tokens from
     * @param amount Amount of tokens to burn
     */
    function burn(address from, uint256 amount) 
        external 
        onlyRole(BURNER_ROLE) 
        whenNotPaused 
    {
        require(from != address(0), "Cannot burn from zero address");
        require(amount > 0, "Amount must be greater than 0");
        require(balanceOf(from) >= amount, "Insufficient balance");
        
        _burn(from, amount);
        emit TokensBurned(from, amount, msg.sender);
    }

    /**
     * @dev Authorize a minter with a specific limit
     * @param minter Address to authorize
     * @param limit Maximum amount this minter can mint
     */
    function authorizeMinter(address minter, uint256 limit) 
        external 
        onlyRole(DEFAULT_ADMIN_ROLE) 
    {
        require(minter != address(0), "Invalid minter address");
        require(limit > 0, "Limit must be greater than 0");
        require(!authorizedMinters[minter], "Minter already authorized");

        authorizedMinters[minter] = true;
        mintingLimits[minter] = limit;
        _grantRole(MINTER_ROLE, minter);

        emit MinterAuthorized(minter, limit);
    }

    /**
     * @dev Deauthorize a minter
     * @param minter Address to deauthorize
     */
    function deauthorizeMinter(address minter) 
        external 
        onlyRole(DEFAULT_ADMIN_ROLE) 
    {
        require(authorizedMinters[minter], "Minter not authorized");

        authorizedMinters[minter] = false;
        mintingLimits[minter] = 0;
        _revokeRole(MINTER_ROLE, minter);

        emit MinterDeauthorized(minter);
    }

    /**
     * @dev Mint tokens with limit check
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     */
    function mintWithLimit(address to, uint256 amount) 
        external 
        whenNotPaused 
    {
        require(authorizedMinters[msg.sender], "Not authorized minter");
        require(to != address(0), "Cannot mint to zero address");
        require(amount > 0, "Amount must be greater than 0");
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds maximum supply");
        require(mintedAmount[msg.sender] + amount <= mintingLimits[msg.sender], "Exceeds minting limit");

        mintedAmount[msg.sender] += amount;
        _mint(to, amount);
        emit TokensMinted(to, amount, msg.sender);
    }

    /**
     * @dev Get remaining minting limit for a minter
     * @param minter Minter address
     */
    function getRemainingMintingLimit(address minter) 
        external 
        view 
        returns (uint256) 
    {
        if (!authorizedMinters[minter]) return 0;
        return mintingLimits[minter] - mintedAmount[minter];
    }

    /**
     * @dev Get minting statistics for a minter
     * @param minter Minter address
     */
    function getMintingStats(address minter) 
        external 
        view 
        returns (
            bool isAuthorized,
            uint256 limit,
            uint256 minted,
            uint256 remaining
        ) 
    {
        isAuthorized = authorizedMinters[minter];
        limit = mintingLimits[minter];
        minted = mintedAmount[minter];
        remaining = limit > minted ? limit - minted : 0;
    }

    /**
     * @dev Pause the contract
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @dev Unpause the contract
     */
    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    // Required overrides for ERC20Votes
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override(ERC20) whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }

    function _afterTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override(ERC20, ERC20Votes) {
        super._afterTokenTransfer(from, to, amount);
    }

    function _mint(address to, uint256 amount) 
        internal 
        override(ERC20, ERC20Votes) 
    {
        super._mint(to, amount);
    }

    function _burn(address account, uint256 amount) 
        internal 
        override(ERC20, ERC20Votes) 
    {
        super._burn(account, amount);
    }
}
