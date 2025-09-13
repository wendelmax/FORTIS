/*
 * FORTIS HSM Header
 * Cabeçalho para Hardware Security Module FORTIS-HSM-001
 * 
 * Copyright (C) 2024 FORTIS Team
 * License: MIT
 */

#ifndef FORTIS_HSM_H
#define FORTIS_HSM_H

#include <stdint.h>
#include <stdbool.h>

// HSM Version
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

// Key Permissions
#define PERM_ENCRYPT         0x01
#define PERM_DECRYPT         0x02
#define PERM_SIGN            0x04
#define PERM_VERIFY          0x08
#define PERM_ALL             0x0F

// HSM State Structure
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

// Function Prototypes

// Hardware Interface
void hsm_hardware_init(void);
void hsm_hardware_random(uint8_t* data, uint16_t len);
uint32_t hsm_hardware_random32(void);
void hsm_hardware_delay(uint32_t ms);
void hsm_hardware_reset(void);

// USB Interface
bool usb_data_available(void);
uint16_t usb_read(uint8_t* data, uint16_t len);
uint16_t usb_write(const uint8_t* data, uint16_t len);

// Cryptographic Functions
int hsm_aes_gcm_encrypt(const uint8_t* plaintext, uint16_t plaintext_len,
                       const uint8_t* key, const uint8_t* nonce,
                       uint8_t* ciphertext, uint16_t* ciphertext_len);
int hsm_aes_gcm_decrypt(const uint8_t* ciphertext, uint16_t ciphertext_len,
                       const uint8_t* key, const uint8_t* nonce,
                       uint8_t* plaintext, uint16_t* plaintext_len);
int hsm_ecdsa_sign(const uint8_t* data, uint16_t len,
                  const uint8_t* private_key, uint8_t* signature, uint16_t* signature_len);
int hsm_ecdsa_verify(const uint8_t* data, uint16_t len,
                    const uint8_t* signature, uint16_t signature_len,
                    const uint8_t* public_key);
int hsm_sha256(const uint8_t* data, uint16_t len, uint8_t* hash);

// Background Tasks
void hsm_background_tasks(void);

// Logging
void hsm_log_event(const char* event, const char* message);

// Utility Functions
uint32_t hsm_crc32(const uint8_t* data, uint16_t len);
void hsm_secure_erase(uint8_t* data, uint16_t len);

#endif // FORTIS_HSM_H
