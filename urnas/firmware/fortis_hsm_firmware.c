/*
 * FORTIS HSM Firmware
 * Firmware para Hardware Security Module FORTIS-HSM-001
 * 
 * Copyright (C) 2024 FORTIS Team
 * License: MIT
 */

#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>
#include "fortis_hsm.h"

// HSM Configuration
#define HSM_VERSION_MAJOR    1
#define HSM_VERSION_MINOR    0
#define HSM_VERSION_PATCH    0

// Security Configuration
#define MAX_KEYS             1000
#define KEY_SIZE             32
#define NONCE_SIZE           16
#define TAG_SIZE             16
#define MAX_DATA_SIZE        4096

// Hardware Configuration
#define HSM_MEMORY_SIZE      0x10000  // 64KB
#define HSM_FLASH_SIZE       0x80000  // 512KB
#define HSM_RAM_SIZE         0x2000   // 8KB

// Command Codes
#define CMD_INIT             0x01
#define CMD_GENERATE_KEY     0x02
#define CMD_ENCRYPT          0x03
#define CMD_DECRYPT          0x04
#define CMD_SIGN             0x05
#define CMD_VERIFY           0x06
#define CMD_HASH             0x07
#define CMD_RANDOM           0x08
#define CMD_STATUS           0x09
#define CMD_RESET            0x0A

// Response Codes
#define RESP_SUCCESS         0x00
#define RESP_ERROR           0x01
#define RESP_INVALID_CMD     0x02
#define RESP_INVALID_DATA    0x03
#define RESP_KEY_NOT_FOUND   0x04
#define RESP_MEMORY_FULL     0x05
#define RESP_AUTH_FAILED     0x06

// HSM State
typedef struct {
    bool initialized;
    bool authenticated;
    uint32_t key_count;
    uint8_t master_key[KEY_SIZE];
    uint8_t session_key[KEY_SIZE];
    uint32_t random_seed;
    uint32_t error_count;
    uint32_t operation_count;
} hsm_state_t;

// Key Structure
typedef struct {
    uint32_t id;
    uint8_t key[KEY_SIZE];
    uint32_t permissions;
    bool active;
} hsm_key_t;

// Global Variables
static hsm_state_t g_hsm_state;
static hsm_key_t g_keys[MAX_KEYS];
static uint8_t g_work_buffer[MAX_DATA_SIZE];
static uint8_t g_encryption_buffer[MAX_DATA_SIZE];

// Function Prototypes
static void hsm_init(void);
static uint8_t hsm_process_command(uint8_t cmd, const uint8_t* data, uint16_t len, uint8_t* response);
static uint8_t hsm_generate_key(uint32_t key_id, uint32_t permissions);
static uint8_t hsm_encrypt_data(const uint8_t* data, uint16_t len, uint8_t* encrypted, uint16_t* encrypted_len);
static uint8_t hsm_decrypt_data(const uint8_t* encrypted, uint16_t len, uint8_t* data, uint16_t* data_len);
static uint8_t hsm_sign_data(const uint8_t* data, uint16_t len, uint8_t* signature, uint16_t* signature_len);
static uint8_t hsm_verify_signature(const uint8_t* data, uint16_t len, const uint8_t* signature, uint16_t signature_len);
static uint8_t hsm_hash_data(const uint8_t* data, uint16_t len, uint8_t* hash);
static uint8_t hsm_generate_random(uint8_t* random, uint16_t len);
static uint8_t hsm_get_status(uint8_t* status);
static void hsm_reset(void);
static bool hsm_authenticate(const uint8_t* auth_data, uint16_t len);
static uint8_t hsm_find_key(uint32_t key_id, hsm_key_t** key);
static void hsm_secure_erase(uint8_t* data, uint16_t len);
static uint32_t hsm_crc32(const uint8_t* data, uint16_t len);

// Main HSM Entry Point
int main(void) {
    // Initialize HSM
    hsm_init();
    
    // Main loop
    while (1) {
        // Wait for command from host
        if (usb_data_available()) {
            uint8_t cmd;
            uint8_t data[256];
            uint16_t len;
            uint8_t response[256];
            uint16_t response_len;
            
            // Read command
            if (usb_read(&cmd, 1) == 1) {
                // Read data length
                uint8_t len_bytes[2];
                if (usb_read(len_bytes, 2) == 2) {
                    len = (len_bytes[0] << 8) | len_bytes[1];
                    
                    // Read data
                    if (len > 0 && len <= sizeof(data)) {
                        if (usb_read(data, len) == len) {
                            // Process command
                            uint8_t result = hsm_process_command(cmd, data, len, response);
                            
                            // Send response
                            usb_write(&result, 1);
                            usb_write((uint8_t*)&response_len, 2);
                            if (response_len > 0) {
                                usb_write(response, response_len);
                            }
                        }
                    }
                }
            }
        }
        
        // Perform background tasks
        hsm_background_tasks();
    }
    
    return 0;
}

// Initialize HSM
static void hsm_init(void) {
    // Clear state
    memset(&g_hsm_state, 0, sizeof(g_hsm_state));
    memset(g_keys, 0, sizeof(g_keys));
    memset(g_work_buffer, 0, sizeof(g_work_buffer));
    memset(g_encryption_buffer, 0, sizeof(g_encryption_buffer));
    
    // Initialize hardware
    hsm_hardware_init();
    
    // Generate master key from hardware RNG
    hsm_hardware_random(g_hsm_state.master_key, KEY_SIZE);
    
    // Initialize random seed
    g_hsm_state.random_seed = hsm_hardware_random32();
    
    // Mark as initialized
    g_hsm_state.initialized = true;
    
    // Log initialization
    hsm_log_event("HSM_INIT", "HSM initialized successfully");
}

// Process HSM Command
static uint8_t hsm_process_command(uint8_t cmd, const uint8_t* data, uint16_t len, uint8_t* response) {
    uint16_t response_len = 0;
    
    // Check if HSM is initialized
    if (!g_hsm_state.initialized) {
        return RESP_ERROR;
    }
    
    // Process command
    switch (cmd) {
        case CMD_INIT:
            if (len >= 32) {
                if (hsm_authenticate(data, 32)) {
                    g_hsm_state.authenticated = true;
                    response[0] = RESP_SUCCESS;
                    response_len = 1;
                } else {
                    response[0] = RESP_AUTH_FAILED;
                    response_len = 1;
                }
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_GENERATE_KEY:
            if (len >= 8) {
                uint32_t key_id = (data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3];
                uint32_t permissions = (data[4] << 24) | (data[5] << 16) | (data[6] << 8) | data[7];
                
                uint8_t result = hsm_generate_key(key_id, permissions);
                response[0] = result;
                response_len = 1;
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_ENCRYPT:
            if (len >= 4) {
                uint32_t key_id = (data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3];
                const uint8_t* data_to_encrypt = data + 4;
                uint16_t data_len = len - 4;
                
                // Find key
                hsm_key_t* key;
                if (hsm_find_key(key_id, &key) == RESP_SUCCESS) {
                    uint8_t result = hsm_encrypt_data(data_to_encrypt, data_len, response + 1, &response_len);
                    response[0] = result;
                    response_len += 1;
                } else {
                    response[0] = RESP_KEY_NOT_FOUND;
                    response_len = 1;
                }
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_DECRYPT:
            if (len >= 4) {
                uint32_t key_id = (data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3];
                const uint8_t* data_to_decrypt = data + 4;
                uint16_t data_len = len - 4;
                
                // Find key
                hsm_key_t* key;
                if (hsm_find_key(key_id, &key) == RESP_SUCCESS) {
                    uint8_t result = hsm_decrypt_data(data_to_decrypt, data_len, response + 1, &response_len);
                    response[0] = result;
                    response_len += 1;
                } else {
                    response[0] = RESP_KEY_NOT_FOUND;
                    response_len = 1;
                }
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_SIGN:
            if (len >= 4) {
                uint32_t key_id = (data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3];
                const uint8_t* data_to_sign = data + 4;
                uint16_t data_len = len - 4;
                
                // Find key
                hsm_key_t* key;
                if (hsm_find_key(key_id, &key) == RESP_SUCCESS) {
                    uint8_t result = hsm_sign_data(data_to_sign, data_len, response + 1, &response_len);
                    response[0] = result;
                    response_len += 1;
                } else {
                    response[0] = RESP_KEY_NOT_FOUND;
                    response_len = 1;
                }
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_VERIFY:
            if (len >= 4) {
                uint32_t key_id = (data[0] << 24) | (data[1] << 16) | (data[2] << 8) | data[3];
                const uint8_t* data_to_verify = data + 4;
                uint16_t data_len = len - 4;
                
                // Find key
                hsm_key_t* key;
                if (hsm_find_key(key_id, &key) == RESP_SUCCESS) {
                    uint8_t result = hsm_verify_signature(data_to_verify, data_len, data + 4 + data_len, len - 4 - data_len);
                    response[0] = result;
                    response_len = 1;
                } else {
                    response[0] = RESP_KEY_NOT_FOUND;
                    response_len = 1;
                }
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_HASH:
            if (len > 0) {
                uint8_t result = hsm_hash_data(data, len, response + 1);
                response[0] = result;
                response_len = 33; // 1 byte result + 32 bytes hash
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_RANDOM:
            if (len >= 2) {
                uint16_t random_len = (data[0] << 8) | data[1];
                if (random_len <= MAX_DATA_SIZE) {
                    uint8_t result = hsm_generate_random(response + 1, random_len);
                    response[0] = result;
                    response_len = random_len + 1;
                } else {
                    response[0] = RESP_INVALID_DATA;
                    response_len = 1;
                }
            } else {
                response[0] = RESP_INVALID_DATA;
                response_len = 1;
            }
            break;
            
        case CMD_STATUS:
            {
                uint8_t result = hsm_get_status(response + 1);
                response[0] = result;
                response_len = 17; // 1 byte result + 16 bytes status
            }
            break;
            
        case CMD_RESET:
            hsm_reset();
            response[0] = RESP_SUCCESS;
            response_len = 1;
            break;
            
        default:
            response[0] = RESP_INVALID_CMD;
            response_len = 1;
            break;
    }
    
    return response_len;
}

// Generate Key
static uint8_t hsm_generate_key(uint32_t key_id, uint32_t permissions) {
    // Check if key already exists
    hsm_key_t* existing_key;
    if (hsm_find_key(key_id, &existing_key) == RESP_SUCCESS) {
        return RESP_ERROR; // Key already exists
    }
    
    // Check if we have space for more keys
    if (g_hsm_state.key_count >= MAX_KEYS) {
        return RESP_MEMORY_FULL;
    }
    
    // Find empty slot
    for (uint32_t i = 0; i < MAX_KEYS; i++) {
        if (!g_keys[i].active) {
            // Generate random key
            hsm_hardware_random(g_keys[i].key, KEY_SIZE);
            
            // Set key properties
            g_keys[i].id = key_id;
            g_keys[i].permissions = permissions;
            g_keys[i].active = true;
            
            // Increment key count
            g_hsm_state.key_count++;
            
            // Log key generation
            hsm_log_event("KEY_GENERATED", "Key generated successfully");
            
            return RESP_SUCCESS;
        }
    }
    
    return RESP_MEMORY_FULL;
}

// Encrypt Data
static uint8_t hsm_encrypt_data(const uint8_t* data, uint16_t len, uint8_t* encrypted, uint16_t* encrypted_len) {
    // Check data length
    if (len > MAX_DATA_SIZE) {
        return RESP_INVALID_DATA;
    }
    
    // Generate random nonce
    uint8_t nonce[NONCE_SIZE];
    hsm_hardware_random(nonce, NONCE_SIZE);
    
    // Copy nonce to output
    memcpy(encrypted, nonce, NONCE_SIZE);
    
    // Encrypt data using AES-GCM
    uint16_t ciphertext_len = len;
    if (hsm_aes_gcm_encrypt(data, len, g_hsm_state.session_key, nonce, 
                           encrypted + NONCE_SIZE, &ciphertext_len) != 0) {
        return RESP_ERROR;
    }
    
    // Set output length
    *encrypted_len = NONCE_SIZE + ciphertext_len;
    
    // Increment operation count
    g_hsm_state.operation_count++;
    
    return RESP_SUCCESS;
}

// Decrypt Data
static uint8_t hsm_decrypt_data(const uint8_t* encrypted, uint16_t len, uint8_t* data, uint16_t* data_len) {
    // Check minimum length
    if (len < NONCE_SIZE) {
        return RESP_INVALID_DATA;
    }
    
    // Extract nonce
    const uint8_t* nonce = encrypted;
    const uint8_t* ciphertext = encrypted + NONCE_SIZE;
    uint16_t ciphertext_len = len - NONCE_SIZE;
    
    // Decrypt data using AES-GCM
    if (hsm_aes_gcm_decrypt(ciphertext, ciphertext_len, g_hsm_state.session_key, nonce, 
                           data, data_len) != 0) {
        return RESP_ERROR;
    }
    
    // Increment operation count
    g_hsm_state.operation_count++;
    
    return RESP_SUCCESS;
}

// Sign Data
static uint8_t hsm_sign_data(const uint8_t* data, uint16_t len, uint8_t* signature, uint16_t* signature_len) {
    // Hash data
    uint8_t hash[32];
    if (hsm_hash_data(data, len, hash) != RESP_SUCCESS) {
        return RESP_ERROR;
    }
    
    // Sign hash using ECDSA
    if (hsm_ecdsa_sign(hash, 32, g_hsm_state.master_key, signature, signature_len) != 0) {
        return RESP_ERROR;
    }
    
    // Increment operation count
    g_hsm_state.operation_count++;
    
    return RESP_SUCCESS;
}

// Verify Signature
static uint8_t hsm_verify_signature(const uint8_t* data, uint16_t len, const uint8_t* signature, uint16_t signature_len) {
    // Hash data
    uint8_t hash[32];
    if (hsm_hash_data(data, len, hash) != RESP_SUCCESS) {
        return RESP_ERROR;
    }
    
    // Verify signature using ECDSA
    if (hsm_ecdsa_verify(hash, 32, signature, signature_len, g_hsm_state.master_key) != 0) {
        return RESP_ERROR;
    }
    
    // Increment operation count
    g_hsm_state.operation_count++;
    
    return RESP_SUCCESS;
}

// Hash Data
static uint8_t hsm_hash_data(const uint8_t* data, uint16_t len, uint8_t* hash) {
    // Use SHA-256
    if (hsm_sha256(data, len, hash) != 0) {
        return RESP_ERROR;
    }
    
    return RESP_SUCCESS;
}

// Generate Random
static uint8_t hsm_generate_random(uint8_t* random, uint16_t len) {
    // Generate random data using hardware RNG
    hsm_hardware_random(random, len);
    
    return RESP_SUCCESS;
}

// Get Status
static uint8_t hsm_get_status(uint8_t* status) {
    // Status format: [version(4)] [key_count(4)] [operation_count(4)] [error_count(4)]
    status[0] = HSM_VERSION_MAJOR;
    status[1] = HSM_VERSION_MINOR;
    status[2] = HSM_VERSION_PATCH;
    status[3] = 0;
    
    status[4] = (g_hsm_state.key_count >> 24) & 0xFF;
    status[5] = (g_hsm_state.key_count >> 16) & 0xFF;
    status[6] = (g_hsm_state.key_count >> 8) & 0xFF;
    status[7] = g_hsm_state.key_count & 0xFF;
    
    status[8] = (g_hsm_state.operation_count >> 24) & 0xFF;
    status[9] = (g_hsm_state.operation_count >> 16) & 0xFF;
    status[10] = (g_hsm_state.operation_count >> 8) & 0xFF;
    status[11] = g_hsm_state.operation_count & 0xFF;
    
    status[12] = (g_hsm_state.error_count >> 24) & 0xFF;
    status[13] = (g_hsm_state.error_count >> 16) & 0xFF;
    status[14] = (g_hsm_state.error_count >> 8) & 0xFF;
    status[15] = g_hsm_state.error_count & 0xFF;
    
    return RESP_SUCCESS;
}

// Reset HSM
static void hsm_reset(void) {
    // Secure erase all sensitive data
    hsm_secure_erase((uint8_t*)&g_hsm_state, sizeof(g_hsm_state));
    hsm_secure_erase((uint8_t*)g_keys, sizeof(g_keys));
    hsm_secure_erase(g_work_buffer, sizeof(g_work_buffer));
    hsm_secure_erase(g_encryption_buffer, sizeof(g_encryption_buffer));
    
    // Reinitialize
    hsm_init();
    
    // Log reset
    hsm_log_event("HSM_RESET", "HSM reset performed");
}

// Authenticate
static bool hsm_authenticate(const uint8_t* auth_data, uint16_t len) {
    // Simple authentication using master key
    if (len != KEY_SIZE) {
        return false;
    }
    
    // Compare with master key
    if (memcmp(auth_data, g_hsm_state.master_key, KEY_SIZE) == 0) {
        return true;
    }
    
    // Increment error count
    g_hsm_state.error_count++;
    
    return false;
}

// Find Key
static uint8_t hsm_find_key(uint32_t key_id, hsm_key_t** key) {
    for (uint32_t i = 0; i < MAX_KEYS; i++) {
        if (g_keys[i].active && g_keys[i].id == key_id) {
            *key = &g_keys[i];
            return RESP_SUCCESS;
        }
    }
    
    return RESP_KEY_NOT_FOUND;
}

// Secure Erase
static void hsm_secure_erase(uint8_t* data, uint16_t len) {
    // Overwrite with random data multiple times
    for (int i = 0; i < 3; i++) {
        hsm_hardware_random(data, len);
    }
    
    // Final clear
    memset(data, 0, len);
}

// CRC32 Calculation
static uint32_t hsm_crc32(const uint8_t* data, uint16_t len) {
    uint32_t crc = 0xFFFFFFFF;
    
    for (uint16_t i = 0; i < len; i++) {
        crc ^= data[i];
        for (int j = 0; j < 8; j++) {
            if (crc & 1) {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    
    return ~crc;
}
