# Encryption Implementation Plan

Based on the requirements in Encryption.md, here's a detailed plan to implement secure data encryption in the Tauri application.

## Overview

We need to implement a secure encryption system that:
1. Prompts for a password at startup
2. Derives a 256-bit encryption key using Argon2id
3. Uses the key to encrypt/decrypt sensitive data in SQLite and WAV files
4. Securely manages the key in memory
5. Properly cleans up when the application exits

## Implementation Steps

### 1. Set Up Dependencies

Add the necessary crates to Cargo.toml:

```toml
[dependencies]
# Existing dependencies...
argon2 = "0.5.2"
password-hash = "0.5.0"
rand_core = "0.6.4"
aes-gcm = "0.10.3"
zeroize = "1.6.0"
secrecy = "0.8.0"
```

### 2. Create Encryption Module Structure

Create a new module structure for encryption:

```
src/
  encryption/
    mod.rs           # Main module exports
    key_manager.rs   # Password handling and key derivation
    crypto.rs        # Encryption/decryption utilities
    db_crypto.rs     # Database field encryption helpers
    file_crypto.rs   # File encryption helpers
```

### 3. Implement Key Derivation (key_manager.rs)

- Create a secure key manager that:
  - Receives the password from the frontend
  - Generates or retrieves a salt
  - Derives a 256-bit key using Argon2id
  - Securely stores the key in memory
  - Provides a way to access the key for encryption/decryption
  - Implements zeroize for secure cleanup

### 4. Implement Encryption/Decryption Utilities (crypto.rs)

- Create utility functions for:
  - `encrypt_bytes(key: &[u8], plaintext: &[u8]) -> Vec<u8>`
  - `decrypt_bytes(key: &[u8], data: &[u8]) -> Result<Vec<u8>, DecryptError>`
  - These will handle AES-256-GCM encryption with proper IV generation

### 5. Implement Database Field Encryption (db_crypto.rs)

- Create helpers for encrypting/decrypting database fields
- Identify sensitive fields in the database schema (conversations, raw_memo_html, enhanced_memo_html)
- Implement functions to transparently encrypt on write and decrypt on read

### 6. Implement File Encryption (file_crypto.rs)

- Create helpers for encrypting/decrypting WAV files
- Implement functions to:
  - Encrypt WAV files before saving to disk
  - Decrypt WAV files when loading for playback
  - Handle temporary decrypted files securely

### 7. Create Frontend Password Prompt

- Implement a password prompt dialog in the frontend
- Set up Tauri command to send the password to the backend
- Handle password validation and error states

### 8. Integrate with Existing Code

- Modify database access code to use encryption for sensitive fields
- Modify file I/O code to use encryption for WAV files
- Ensure proper error handling throughout

### 9. Implement Secure Cleanup

- Set up proper cleanup when the application exits
- Ensure the key is securely wiped from memory

### 10. Testing

- Create tests for each encryption component
- Test key derivation with various passwords
- Test encryption/decryption of database fields
- Test encryption/decryption of WAV files
- Test error handling (wrong password, corrupted data)
- Test secure cleanup

## Detailed Implementation Plan

### Phase 1: Core Encryption Infrastructure

1. Set up the encryption module structure
2. Implement key derivation with Argon2id
3. Implement basic encryption/decryption utilities
4. Create a secure key manager

### Phase 2: Database Integration

1. Identify all sensitive fields in the database
2. Implement database field encryption/decryption
3. Modify database access code to use encryption
4. Test database encryption/decryption

### Phase 3: File Encryption

1. Implement WAV file encryption/decryption
2. Modify file I/O code to use encryption
3. Test file encryption/decryption

### Phase 4: Frontend Integration

1. Create password prompt UI
2. Set up Tauri command for password handling
3. Implement error handling and user feedback

### Phase 5: Testing and Refinement

1. Comprehensive testing of all components
2. Performance optimization if needed
3. Security review and hardening

## Security Considerations

- Use strong Argon2id parameters (memory: ~19 MiB, iterations: 2+)
- Never reuse IVs with AES-GCM
- Properly handle authentication errors
- Keep the encryption key in limited scope
- Avoid storing plaintext anywhere
- Implement proper zeroization of sensitive data
- Provide clear user guidance on password importance
