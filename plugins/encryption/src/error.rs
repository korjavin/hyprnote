use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Encryption(#[from] hypr_encryption::EncryptionError),

    #[error("Encryption is not enabled")]
    NotEnabled,

    #[error("Encryption is already enabled")]
    AlreadyEnabled,

    #[error("Incorrect password")]
    IncorrectPassword,

    #[error("Password is required")]
    PasswordRequired,

    #[error("Salt not found")]
    SaltNotFound,

    #[error("Failed to save salt: {0}")]
    SaltSaveFailed(String),

    #[error("Failed to load salt: {0}")]
    SaltLoadFailed(String),

    #[error("Tauri error: {0}")]
    Tauri(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<tauri::Error> for Error {
    fn from(error: tauri::Error) -> Self {
        Self::Tauri(error.to_string())
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self::Other(error)
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Self::Other(error.to_string())
    }
}
