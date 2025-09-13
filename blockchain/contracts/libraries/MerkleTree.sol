// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title MerkleTree
 * @dev Biblioteca para operações com árvores Merkle
 * @author FORTIS Team
 */
library MerkleTree {
    /**
     * @dev Verifica uma prova Merkle
     * @param leaf O hash da folha
     * @param proof Array de hashes da prova
     * @param root O hash da raiz
     * @return true se a prova for válida
     */
    function verify(
        bytes32 leaf,
        bytes32[] memory proof,
        bytes32 root
    ) internal pure returns (bool) {
        bytes32 computedHash = leaf;

        for (uint256 i = 0; i < proof.length; i++) {
            bytes32 proofElement = proof[i];

            if (computedHash <= proofElement) {
                // Hash atual é o filho esquerdo
                computedHash = keccak256(abi.encodePacked(computedHash, proofElement));
            } else {
                // Hash atual é o filho direito
                computedHash = keccak256(abi.encodePacked(proofElement, computedHash));
            }
        }

        return computedHash == root;
    }

    /**
     * @dev Calcula o hash de uma folha
     * @param data Dados da folha
     * @return Hash da folha
     */
    function leafHash(bytes memory data) internal pure returns (bytes32) {
        return keccak256(data);
    }

    /**
     * @dev Calcula o hash de duas folhas
     * @param left Hash da folha esquerda
     * @param right Hash da folha direita
     * @return Hash combinado
     */
    function nodeHash(bytes32 left, bytes32 right) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(left, right));
    }

    /**
     * @dev Calcula a raiz de uma árvore Merkle a partir de um array de dados
     * @param data Array de dados
     * @return Hash da raiz
     */
    function calculateRoot(bytes[] memory data) internal pure returns (bytes32) {
        if (data.length == 0) {
            return bytes32(0);
        }

        if (data.length == 1) {
            return leafHash(data[0]);
        }

        // Criar array de hashes das folhas
        bytes32[] memory hashes = new bytes32[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            hashes[i] = leafHash(data[i]);
        }

        // Calcular raiz recursivamente
        return _calculateRoot(hashes);
    }

    /**
     * @dev Função interna para calcular a raiz recursivamente
     * @param hashes Array de hashes
     * @return Hash da raiz
     */
    function _calculateRoot(bytes32[] memory hashes) private pure returns (bytes32) {
        if (hashes.length == 1) {
            return hashes[0];
        }

        uint256 nextLevelLength = (hashes.length + 1) / 2;
        bytes32[] memory nextLevel = new bytes32[](nextLevelLength);

        for (uint256 i = 0; i < nextLevelLength; i++) {
            if (i * 2 + 1 < hashes.length) {
                nextLevel[i] = nodeHash(hashes[i * 2], hashes[i * 2 + 1]);
            } else {
                nextLevel[i] = hashes[i * 2];
            }
        }

        return _calculateRoot(nextLevel);
    }
}
