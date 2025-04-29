use std::fs;
use std::path::Path;
use crate::{
    error::EncryptionError,
    crypto::{encrypt_bytes, decrypt_bytes},
};

/// Encrypt a file
///
/// This function:
/// 1. Reads the file content
/// 2. Encrypts the content using AES-256-GCM
/// 3. Writes the encrypted content to a new file
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `input_path` - Path to the file to encrypt
/// * `output_path` - Path where the encrypted file will be written
///
/// # Returns
///
/// `Ok(())` if successful, or an error
pub fn encrypt_file<P: AsRef<Path>>(
    key: &[u8],
    input_path: P,
    output_path: P,
) -> Result<(), EncryptionError> {
    // Read the file content
    let plaintext = fs::read(input_path)
        .map_err(|e| EncryptionError::FileOperation(e))?;

    // Encrypt the content
    let encrypted = encrypt_bytes(key, &plaintext)?;

    // Write the encrypted content to the output file
    fs::write(output_path, encrypted)
        .map_err(|e| EncryptionError::FileOperation(e))?;

    Ok(())
}

/// Decrypt a file
///
/// This function:
/// 1. Reads the encrypted file content
/// 2. Decrypts the content using AES-256-GCM
/// 3. Writes the decrypted content to a new file
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `input_path` - Path to the encrypted file
/// * `output_path` - Path where the decrypted file will be written
///
/// # Returns
///
/// `Ok(())` if successful, or an error
pub fn decrypt_file<P: AsRef<Path>>(
    key: &[u8],
    input_path: P,
    output_path: P,
) -> Result<(), EncryptionError> {
    // Read the encrypted file content
    let encrypted = fs::read(input_path)
        .map_err(|e| EncryptionError::FileOperation(e))?;

    // Decrypt the content
    let plaintext = decrypt_bytes(key, &encrypted)?;

    // Write the decrypted content to the output file
    fs::write(output_path, plaintext)
        .map_err(|e| EncryptionError::FileOperation(e))?;

    Ok(())
}

/// Encrypt a file in memory
///
/// This function:
/// 1. Encrypts the provided content using AES-256-GCM
/// 2. Writes the encrypted content to a file
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `content` - The content to encrypt
/// * `output_path` - Path where the encrypted file will be written
///
/// # Returns
///
/// `Ok(())` if successful, or an error
pub fn encrypt_content_to_file<P: AsRef<Path>>(
    key: &[u8],
    content: &[u8],
    output_path: P,
) -> Result<(), EncryptionError> {
    // Encrypt the content
    let encrypted = encrypt_bytes(key, content)?;

    // Write the encrypted content to the output file
    fs::write(output_path, encrypted)
        .map_err(|e| EncryptionError::FileOperation(e))?;

    Ok(())
}

/// Decrypt a file to memory
///
/// This function:
/// 1. Reads the encrypted file content
/// 2. Decrypts the content using AES-256-GCM
/// 3. Returns the decrypted content
///
/// # Arguments
///
/// * `key` - The 32-byte encryption key
/// * `input_path` - Path to the encrypted file
///
/// # Returns
///
/// The decrypted content as a vector of bytes
pub fn decrypt_file_to_memory<P: AsRef<Path>>(
    key: &[u8],
    input_path: P,
) -> Result<Vec<u8>, EncryptionError> {
    // Read the encrypted file content
    let encrypted = fs::read(input_path)
        .map_err(|e| EncryptionError::FileOperation(e))?;

    // Decrypt the content
    decrypt_bytes(key, &encrypted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::{RngCore, OsRng};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_encrypt_decrypt_file() {
        // Generate a random key
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // Create a temporary file with test content
        let mut input_file = NamedTempFile::new().unwrap();
        let test_content = b"This is a test file content";
        input_file.write_all(test_content).unwrap();
        input_file.flush().unwrap();

        // Create temporary files for the encrypted and decrypted content
        let encrypted_file = NamedTempFile::new().unwrap();
        let decrypted_file = NamedTempFile::new().unwrap();

        // Encrypt the file
        encrypt_file(&key, input_file.path(), encrypted_file.path()).unwrap();

        // Verify that the encrypted file exists and is different from the original
        let encrypted_content = fs::read(encrypted_file.path()).unwrap();
        assert!(encrypted_content.len() > test_content.len());
        assert_ne!(encrypted_content, test_content);

        // Decrypt the file
        decrypt_file(&key, encrypted_file.path(), decrypted_file.path()).unwrap();

        // Verify that the decrypted content matches the original
        let decrypted_content = fs::read(decrypted_file.path()).unwrap();
        assert_eq!(decrypted_content, test_content);
    }

    #[test]
    fn test_encrypt_decrypt_memory() {
        // Generate a random key
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);

        // Test content
        let test_content = b"This is test content for memory encryption";

        // Create a temporary file for the encrypted content
        let encrypted_file = NamedTempFile::new().unwrap();

        // Encrypt the content to a file
        encrypt_content_to_file(&key, test_content, encrypted_file.path()).unwrap();

        // Decrypt the file to memory
        let decrypted_content = decrypt_file_to_memory(&key, encrypted_file.path()).unwrap();

        // Verify that the decrypted content matches the original
        assert_eq!(decrypted_content, test_content);
    }
}
