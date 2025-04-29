use thiserror::Error;

/// Errors that can occur during encryption operations
#[derive(Debug, Error)]
pub enum EncryptionError {
    /// Error during key derivation
    #[error("Key derivation failed: {0}")]
    KeyDerivation(String),

    /// Error during encryption
    #[error("Encryption failed: {0}")]
    Encryption(String),

    /// Error during decryption
    #[error("Decryption failed: {0}")]
    Decryption(String),

    /// Error when the key is not available
    #[error("Encryption key not available")]
    KeyNotAvailable,

    /// Error when the salt is not available
    #[error("Salt not available")]
    SaltNotAvailable,

    /// Error during file operations
    #[error("File operation failed: {0}")]
    FileOperation(#[from] std::io::Error),

    /// Error during AES-GCM operations
    #[error("AES-GCM operation failed")]
    AeadError,

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl From<argon2::Error> for EncryptionError {
    fn from(err: argon2::Error) -> Self {
        Self::KeyDerivation(err.to_string())
    }
}

impl From<password_hash::Error> for EncryptionError {
    fn from(err: password_hash::Error) -> Self {
        Self::KeyDerivation(err.to_string())
    }
}

impl From<String> for EncryptionError {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

impl From<&str> for EncryptionError {
    fn from(err: &str) -> Self {
        Self::Other(err.to_string())
    }
}

impl From<aes_gcm::aead::Error> for EncryptionError {
    fn from(_: aes_gcm::aead::Error) -> Self {
        Self::AeadError
    }
}
