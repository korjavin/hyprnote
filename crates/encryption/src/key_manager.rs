use argon2::{Argon2, Params, password_hash::SaltString};
use rand_core::OsRng;
use zeroize::{Zeroize, ZeroizeOnDrop};
use std::sync::{Arc, RwLock};
use crate::error::EncryptionError;

/// The state of the encryption key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    /// The key is not available
    Unavailable,
    /// The key is available and ready for use
    Available,
}

/// A secure container for the encryption key
#[derive(Zeroize, ZeroizeOnDrop)]
struct SecureKey {
    /// The 256-bit encryption key
    #[zeroize(skip)]
    key: [u8; 32],
}

impl SecureKey {
    /// Create a new secure key
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    /// Get a reference to the key
    fn key(&self) -> &[u8; 32] {
        &self.key
    }
}

/// Manages the encryption key for the application
pub struct KeyManager {
    /// The encryption key, wrapped in a secure container
    key: Arc<RwLock<Option<SecureKey>>>,
    /// The salt used for key derivation
    salt: Option<String>,
}

impl KeyManager {
    /// Create a new key manager
    pub fn new() -> Self {
        Self {
            key: Arc::new(RwLock::new(None)),
            salt: None,
        }
    }

    /// Get the current state of the key
    pub fn state(&self) -> KeyState {
        if self.key.read().unwrap().is_some() {
            KeyState::Available
        } else {
            KeyState::Unavailable
        }
    }

    /// Set the salt for key derivation
    pub fn set_salt(&mut self, salt: String) {
        self.salt = Some(salt);
    }

    /// Get the salt for key derivation
    pub fn salt(&self) -> Option<&str> {
        self.salt.as_deref()
    }

    /// Generate a new random salt
    pub fn generate_salt(&mut self) -> String {
        let salt = SaltString::generate(&mut OsRng).to_string();
        self.salt = Some(salt.clone());
        salt
    }

    /// Derive a key from a password using Argon2id
    pub fn derive_key(&mut self, password: &[u8]) -> Result<(), EncryptionError> {
        // Get or generate the salt
        let salt = match &self.salt {
            Some(s) => s.clone(),
            None => self.generate_salt(),
        };

        // Configure Argon2id with strong parameters
        // Memory: 19 MiB (19456 KiB), Iterations: 2, Parallelism: 1
        let params = Params::new(19456, 2, 1, None)?;
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        // Derive the key
        let mut key_bytes = [0u8; 32];
        argon2.hash_password_into(password, salt.as_bytes(), &mut key_bytes)?;

        // Store the key securely
        let secure_key = SecureKey::new(key_bytes);
        *self.key.write().unwrap() = Some(secure_key);

        Ok(())
    }

    /// Get a reference to the key, if available
    pub fn get_key(&self) -> Result<[u8; 32], EncryptionError> {
        match &*self.key.read().unwrap() {
            Some(secure_key) => Ok(*secure_key.key()),
            None => Err(EncryptionError::KeyNotAvailable),
        }
    }

    /// Clear the key from memory
    pub fn clear_key(&self) {
        *self.key.write().unwrap() = None;
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let mut key_manager = KeyManager::new();
        let password = b"test-password";

        // Derive a key
        key_manager.derive_key(password).unwrap();
        assert_eq!(key_manager.state(), KeyState::Available);

        // Get the key
        let key = key_manager.get_key().unwrap();
        assert_eq!(key.len(), 32);

        // Clear the key
        key_manager.clear_key();
        assert_eq!(key_manager.state(), KeyState::Unavailable);
        assert!(key_manager.get_key().is_err());
    }

    #[test]
    fn test_salt_generation() {
        let mut key_manager = KeyManager::new();

        // Generate a salt
        let salt = key_manager.generate_salt();
        assert!(!salt.is_empty());
        assert_eq!(key_manager.salt(), Some(salt.as_str()));

        // Set a custom salt
        let custom_salt = "custom-salt".to_string();
        key_manager.set_salt(custom_salt.clone());
        assert_eq!(key_manager.salt(), Some(custom_salt.as_str()));
    }
}
