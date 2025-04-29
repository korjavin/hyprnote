//! Secure data encryption module for Hyprnote
//! 
//! This module provides encryption and decryption functionality for sensitive data
//! using Argon2id for key derivation and AES-256-GCM for encryption.

mod error;
mod key_manager;
mod crypto;
mod db_crypto;
mod file_crypto;

pub use error::EncryptionError;
pub use key_manager::{KeyManager, KeyState};
pub use crypto::{encrypt_bytes, decrypt_bytes};
pub use db_crypto::{encrypt_field, decrypt_field};
pub use file_crypto::{encrypt_file, decrypt_file};

/// Re-export types that are used in the public API
pub use aes_gcm::aead::Error as AeadError;
