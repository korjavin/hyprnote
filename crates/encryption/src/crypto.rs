use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit, Payload},
    Nonce,
};
use rand_core::{RngCore, OsRng};
use crate::error::EncryptionError;

/// The size of the IV (nonce) in bytes
const IV_SIZE: usize = 12;

/// Encrypt data using AES-256-GCM
///
/// This function:
/// 1. Generates a random 12-byte IV
/// 2. Encrypts the data using AES-256-GCM
/// 3. Returns the IV concatenated with the ciphertext
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `plaintext` - The data to encrypt
///
/// # Returns
///
/// A vector containing the IV concatenated with the ciphertext
pub fn encrypt_bytes(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    // Initialize the cipher with the key
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| EncryptionError::Encryption("Invalid key length".into()))?;

    // Generate a random IV
    let mut iv = [0u8; IV_SIZE];
    OsRng.fill_bytes(&mut iv);
    let nonce = Nonce::from_slice(&iv);

    // Encrypt the data
    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|_| EncryptionError::AeadError)?;

    // Concatenate the IV and ciphertext
    let mut result = Vec::with_capacity(IV_SIZE + ciphertext.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Encrypt data with associated data using AES-256-GCM
///
/// This function is similar to `encrypt_bytes` but also authenticates
/// additional data that is not encrypted.
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `plaintext` - The data to encrypt
/// * `aad` - Additional authenticated data
///
/// # Returns
///
/// A vector containing the IV concatenated with the ciphertext
pub fn encrypt_bytes_with_aad(key: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    // Initialize the cipher with the key
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| EncryptionError::Encryption("Invalid key length".into()))?;

    // Generate a random IV
    let mut iv = [0u8; IV_SIZE];
    OsRng.fill_bytes(&mut iv);
    let nonce = Nonce::from_slice(&iv);

    // Create a payload with the plaintext and AAD
    let payload = Payload {
        msg: plaintext,
        aad,
    };

    // Encrypt the data
    let ciphertext = cipher.encrypt(nonce, payload)
        .map_err(|_| EncryptionError::AeadError)?;

    // Concatenate the IV and ciphertext
    let mut result = Vec::with_capacity(IV_SIZE + ciphertext.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt data using AES-256-GCM
///
/// This function:
/// 1. Extracts the IV from the first 12 bytes of the input
/// 2. Decrypts the remaining data using AES-256-GCM
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `data` - The data to decrypt (IV + ciphertext)
///
/// # Returns
///
/// The decrypted plaintext
pub fn decrypt_bytes(key: &[u8], data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    // Check that the data is long enough to contain an IV and ciphertext
    if data.len() <= IV_SIZE {
        return Err(EncryptionError::Decryption("Data too short".into()));
    }

    // Extract the IV and ciphertext
    let (iv, ciphertext) = data.split_at(IV_SIZE);
    let nonce = Nonce::from_slice(iv);

    // Initialize the cipher with the key
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| EncryptionError::Decryption("Invalid key length".into()))?;

    // Decrypt the data
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| EncryptionError::AeadError)?;

    Ok(plaintext)
}

/// Decrypt data with associated data using AES-256-GCM
///
/// This function is similar to `decrypt_bytes` but also verifies
/// the authenticity of additional data that was not encrypted.
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `data` - The data to decrypt (IV + ciphertext)
/// * `aad` - Additional authenticated data
///
/// # Returns
///
/// The decrypted plaintext
pub fn decrypt_bytes_with_aad(key: &[u8], data: &[u8], aad: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    // Check that the data is long enough to contain an IV and ciphertext
    if data.len() <= IV_SIZE {
        return Err(EncryptionError::Decryption("Data too short".into()));
    }

    // Extract the IV and ciphertext
    let (iv, ciphertext) = data.split_at(IV_SIZE);
    let nonce = Nonce::from_slice(iv);

    // Initialize the cipher with the key
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| EncryptionError::Decryption("Invalid key length".into()))?;

    // Create a payload with the ciphertext and AAD
    let payload = Payload {
        msg: ciphertext,
        aad,
    };

    // Decrypt the data
    let plaintext = cipher.decrypt(nonce, payload)
        .map_err(|_| EncryptionError::AeadError)?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        // Generate a random key
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // Test data
        let plaintext = b"This is a test message";

        // Encrypt the data
        let encrypted = encrypt_bytes(&key, plaintext).unwrap();

        // Verify that the encrypted data is longer than the plaintext
        assert!(encrypted.len() > plaintext.len());

        // Decrypt the data
        let decrypted = decrypt_bytes(&key, &encrypted).unwrap();

        // Verify that the decrypted data matches the original plaintext
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_with_aad() {
        // Generate a random key
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // Test data
        let plaintext = b"This is a test message";
        let aad = b"Additional authenticated data";

        // Encrypt the data with AAD
        let encrypted = encrypt_bytes_with_aad(&key, plaintext, aad).unwrap();

        // Decrypt the data with the correct AAD
        let decrypted = decrypt_bytes_with_aad(&key, &encrypted, aad).unwrap();
        assert_eq!(decrypted, plaintext);

        // Attempt to decrypt with incorrect AAD should fail
        let wrong_aad = b"Wrong additional data";
        let result = decrypt_bytes_with_aad(&key, &encrypted, wrong_aad);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_data() {
        // Generate a random key
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // Test data
        let plaintext = b"This is a test message";

        // Encrypt the data
        let mut encrypted = encrypt_bytes(&key, plaintext).unwrap();

        // Tamper with the ciphertext
        if let Some(byte) = encrypted.get_mut(IV_SIZE + 5) {
            *byte ^= 0x01;
        }

        // Attempt to decrypt the tampered data should fail
        let result = decrypt_bytes(&key, &encrypted);
        assert!(result.is_err());
    }
}
