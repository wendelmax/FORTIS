// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title CryptoUtils
 * @dev Biblioteca para operações criptográficas
 * @author FORTIS Team
 */
library CryptoUtils {
    /**
     * @dev Calcula hash SHA-256
     * @param data Dados para hash
     * @return Hash SHA-256
     */
    function sha256Hash(bytes memory data) internal pure returns (bytes32) {
        return sha256(data);
    }

    /**
     * @dev Calcula hash Keccak-256
     * @param data Dados para hash
     * @return Hash Keccak-256
     */
    function keccak256Hash(bytes memory data) internal pure returns (bytes32) {
        return keccak256(data);
    }

    /**
     * @dev Calcula hash de múltiplos dados
     * @param data1 Primeiro conjunto de dados
     * @param data2 Segundo conjunto de dados
     * @return Hash combinado
     */
    function combinedHash(bytes memory data1, bytes memory data2) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(data1, data2));
    }

    /**
     * @dev Gera nullifier único
     * @param voter Endereço do eleitor
     * @param electionId ID da eleição
     * @param secret Secret do eleitor
     * @return Nullifier único
     */
    function generateNullifier(
        address voter,
        uint256 electionId,
        bytes32 secret
    ) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(voter, electionId, secret));
    }

    /**
     * @dev Verifica assinatura ECDSA
     * @param message Mensagem assinada
     * @param signature Assinatura
     * @param signer Endereço do assinante
     * @return true se a assinatura for válida
     */
    function verifySignature(
        bytes32 message,
        bytes memory signature,
        address signer
    ) internal pure returns (bool) {
        bytes32 messageHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", message));
        bytes32 r;
        bytes32 s;
        uint8 v;

        if (signature.length != 65) {
            return false;
        }

        assembly {
            r := mload(add(signature, 32))
            s := mload(add(signature, 64))
            v := byte(0, mload(add(signature, 96)))
        }

        if (v < 27) {
            v += 27;
        }

        if (v != 27 && v != 28) {
            return false;
        }

        address recovered = ecrecover(messageHash, v, r, s);
        return recovered == signer;
    }

    /**
     * @dev Converte string para bytes32
     * @param source String de origem
     * @return result bytes32 convertido
     */
    function stringToBytes32(string memory source) internal pure returns (bytes32 result) {
        bytes memory tempBytes = bytes(source);
        if (tempBytes.length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
    }

    /**
     * @dev Converte bytes32 para string
     * @param x bytes32 de origem
     * @return String convertida
     */
    function bytes32ToString(bytes32 x) internal pure returns (string memory) {
        bytes memory bytesString = new bytes(32);
        uint charCount = 0;
        for (uint j = 0; j < 32; j++) {
            bytes1 char = bytes1(bytes32(uint(x) * 2 ** (8 * j)));
            if (char != 0) {
                bytesString[charCount] = char;
                charCount++;
            }
        }
        bytes memory bytesStringTrimmed = new bytes(charCount);
        for (uint j = 0; j < charCount; j++) {
            bytesStringTrimmed[j] = bytesString[j];
        }
        return string(bytesStringTrimmed);
    }
}
