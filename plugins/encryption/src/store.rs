use std::sync::{Arc, Mutex};
use hypr_encryption::KeyManager;

/// State for the encryption plugin
#[derive(Default)]
pub struct EncryptionState {
    /// The key manager
    pub key_manager: Arc<Mutex<KeyManager>>,
}
