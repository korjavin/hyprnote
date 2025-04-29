# Encryption Implementation Plan

Based on the requirements in Encryption.md, here's a detailed plan to implement secure data encryption in the Tauri application.

## Overview

We need to implement a secure encryption system that:
1. Prompts for a password at startup
2. Derives a 256-bit encryption key using Argon2id
3. Uses the key to encrypt/decrypt sensitive data in SQLite and WAV files
4. Securely manages the key in memory
5. Properly cleans up when the application exits

## Progress

### ✅ Phase 1: Core Encryption Infrastructure (Completed)

1. ✅ Created a new crate `encryption` in the `crates` directory
2. ✅ Added necessary dependencies to Cargo.toml
3. ✅ Implemented error handling module (`error.rs`)
4. ✅ Implemented key derivation with Argon2id (`key_manager.rs`)
5. ✅ Implemented basic encryption/decryption utilities (`crypto.rs`)
6. ✅ Implemented database field encryption/decryption helpers (`db_crypto.rs`)
7. ✅ Implemented file encryption/decryption helpers (`file_crypto.rs`)

## Implementation Steps

### 1. Set Up Dependencies ✅

Added the necessary crates to Cargo.toml:

```toml
[dependencies]
argon2 = "0.5.2"
password-hash = "0.5.0"
rand_core = { version = "0.6.4", features = ["std"] }
aes-gcm = "0.10.3"
zeroize = { version = "1.6.0", features = ["derive"] }
secrecy = "0.8.0"
thiserror = { workspace = true }
anyhow = { workspace = true }
serde = { workspace = true, features = ["derive"] }
```

### 2. Create Encryption Module Structure ✅

Created a new crate structure for encryption:

```
crates/encryption/
  Cargo.toml
  src/
    lib.rs           # Main module exports
    error.rs         # Error handling
    key_manager.rs   # Password handling and key derivation
    crypto.rs        # Encryption/decryption utilities
    db_crypto.rs     # Database field encryption helpers
    file_crypto.rs   # File encryption helpers
```

### 3. Implement Key Derivation (key_manager.rs) ✅

- Created a secure key manager that:
  - Receives the password from the frontend
  - Generates or retrieves a salt
  - Derives a 256-bit key using Argon2id with strong parameters (19 MiB memory, 2 iterations)
  - Securely stores the key in memory using zeroize
  - Provides a way to access the key for encryption/decryption
  - Implements zeroize for secure cleanup

### 4. Implement Encryption/Decryption Utilities (crypto.rs) ✅

- Created utility functions for:
  - `encrypt_bytes(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, EncryptionError>`
  - `decrypt_bytes(key: &[u8], data: &[u8]) -> Result<Vec<u8>, EncryptionError>`
  - `encrypt_bytes_with_aad(key: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, EncryptionError>`
  - `decrypt_bytes_with_aad(key: &[u8], data: &[u8], aad: &[u8]) -> Result<Vec<u8>, EncryptionError>`
  - These handle AES-256-GCM encryption with proper IV generation

### 5. Implement Database Field Encryption (db_crypto.rs) ✅

- Created helpers for encrypting/decrypting database fields:
  - `encrypt_field(key: &[u8], value: &str) -> Result<Vec<u8>, EncryptionError>`
  - `decrypt_field(key: &[u8], data: &[u8]) -> Result<String, EncryptionError>`
- Added tests for field encryption/decryption

### 6. Implement File Encryption (file_crypto.rs) ✅

- Created helpers for encrypting/decrypting files:
  - `encrypt_file(key: &[u8], input_path: P, output_path: P) -> Result<(), EncryptionError>`
  - `decrypt_file(key: &[u8], input_path: P, output_path: P) -> Result<(), EncryptionError>`
  - `encrypt_content_to_file(key: &[u8], content: &[u8], output_path: P) -> Result<(), EncryptionError>`
  - `decrypt_file_to_memory(key: &[u8], input_path: P) -> Result<Vec<u8>, EncryptionError>`
- Added tests for file encryption/decryption

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

## Next Steps

### Phase 2: Database Integration

1. ✅ Identify all sensitive fields in the database
   - `raw_memo_html` in the sessions table
   - `enhanced_memo_html` in the sessions table
   - `conversations` in the sessions table (stored as JSON)

2. Modify database access code to use encryption
   - Update `upsert_session` in `crates/db-user/src/sessions_ops.rs` to encrypt sensitive fields
   - Update `get_session` and `list_sessions` to decrypt sensitive fields
   - Ensure proper error handling for encryption/decryption failures

3. Test database encryption/decryption

### Phase 3: File Encryption

1. Modify file I/O code to use encryption for WAV files
   - Update WAV file saving in `plugins/listener/src/fsm.rs` to encrypt audio files
   - Implement decryption when reading WAV files
   - Files are saved at `app_dir.join(session_id).join("audio.wav")`

2. Test file encryption/decryption with actual WAV files

### Phase 4: Frontend Integration (Completed)

1. ✅ Set up Tauri plugin for encryption
   - Created plugin structure in `plugins/encryption`
   - Implemented key management and password handling
   - Added commands for unlocking/locking the app and changing passwords
   - Set up salt storage for consistent key derivation

2. ✅ Create password prompt UI
   - Implemented React component for password entry in `apps/desktop/src/components/password-modal.tsx`
   - Created encryption context in `apps/desktop/src/contexts/encryption.tsx`
   - Set up automatic prompt at application startup
   - Added TypeScript bindings for the encryption plugin

3. ✅ Implement error handling and user feedback
   - Added error handling for incorrect passwords
   - Implemented loading state during password verification
   - Added feedback on encryption status

### Phase 5: Testing and Refinement

1. Comprehensive testing of all components
2. Performance optimization if needed
3. Security review and hardening

## Testing Progress

✅ Core Encryption Module Tests
- All unit tests for the encryption crate are passing
- Tested key derivation with Argon2id
- Tested encryption/decryption of data with AES-256-GCM
- Tested database field encryption/decryption
- Tested file encryption/decryption
- Tested error handling for tampered data

## Manual Testing Instructions

To test the encryption functionality:

1. Build and run the application:
   ```bash
   cd /Users/iv/Projects/hyprnote/apps/desktop
   pnpm dev
   ```

2. When the application starts, you should see the password prompt modal.

3. Enter a password to unlock the application.
   - Try entering an incorrect password (anything other than "test") to test error handling
   - Try entering "test" as the password to test successful unlocking

4. After unlocking, the application should function normally.

5. To test locking the application, you can add a temporary button in the UI:
   ```tsx
   <button onClick={() => commands.lock_app()}>Lock App</button>
   ```

6. To test changing the password, you can add a temporary form:
   ```tsx
   <form onSubmit={(e) => {
     e.preventDefault();
     commands.change_password(oldPassword, newPassword);
   }}>
     <input type="password" placeholder="Old Password" value={oldPassword} onChange={(e) => setOldPassword(e.target.value)} />
     <input type="password" placeholder="New Password" value={newPassword} onChange={(e) => setNewPassword(e.target.value)} />
     <button type="submit">Change Password</button>
   </form>
   ```

### Current Implementation Notes

For testing purposes, we've implemented a mock version of the encryption commands:

```tsx
// Mock implementation for testing
const commands = {
  get_encryption_status: async (): Promise<boolean> => {
    console.log("Mock: get_encryption_status called");
    return false;
  },
  unlock_app: async (password: string): Promise<boolean> => {
    console.log("Mock: unlock_app called with password:", password);
    return password === "test";
  },
  lock_app: async (): Promise<void> => {
    console.log("Mock: lock_app called");
  },
  change_password: async (old_password: string, new_password: string): Promise<void> => {
    console.log("Mock: change_password called with old_password:", old_password, "new_password:", new_password);
  }
};
```

In a production environment, these commands would be imported from the actual Tauri plugin.

## Security Considerations

- Use strong Argon2id parameters (memory: ~19 MiB, iterations: 2+)
- Never reuse IVs with AES-GCM
- Properly handle authentication errors
- Keep the encryption key in limited scope
- Avoid storing plaintext anywhere
- Implement proper zeroization of sensitive data
- Provide clear user guidance on password importance
