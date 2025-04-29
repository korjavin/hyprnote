use crate::{
    error::EncryptionError,
    crypto::{encrypt_bytes, decrypt_bytes},
};

/// Encrypt a database field
///
/// This function encrypts a string value for storage in a database.
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `value` - The string value to encrypt
///
/// # Returns
///
/// A vector containing the encrypted data (IV + ciphertext)
pub fn encrypt_field(key: &[u8], value: &str) -> Result<Vec<u8>, EncryptionError> {
    encrypt_bytes(key, value.as_bytes())
}

/// Decrypt a database field
///
/// This function decrypts a database field that was encrypted with `encrypt_field`.
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `data` - The encrypted data (IV + ciphertext)
///
/// # Returns
///
/// The decrypted string value
pub fn decrypt_field(key: &[u8], data: &[u8]) -> Result<String, EncryptionError> {
    let plaintext = decrypt_bytes(key, data)?;
    
    String::from_utf8(plaintext)
        .map_err(|e| EncryptionError::Decryption(format!("Invalid UTF-8: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::{RngCore, OsRng};

    #[test]
    fn test_encrypt_decrypt_field() {
        // Generate a random key
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // Test data
        let value = "This is a sensitive field value";

        // Encrypt the field
        let encrypted = encrypt_field(&key, value).unwrap();
        
        // Decrypt the field
        let decrypted = decrypt_field(&key, &encrypted).unwrap();
        
        // Verify that the decrypted value matches the original
        assert_eq!(decrypted, value);
    }

    #[test]
    fn test_encrypt_decrypt_field_with_special_chars() {
        // Generate a random key
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // Test data with special characters
        let value = "Special characters: äöüß!@#$%^&*()_+{}|:<>?";

        // Encrypt the field
        let encrypted = encrypt_field(&key, value).unwrap();
        
        // Decrypt the field
        let decrypted = decrypt_field(&key, &encrypted).unwrap();
        
        // Verify that the decrypted value matches the original
        assert_eq!(decrypted, value);
    }
}
