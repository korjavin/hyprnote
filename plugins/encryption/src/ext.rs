use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime, State};

use crate::{store::EncryptionState, Error};
use hypr_encryption::{KeyManager, KeyState};

/// Extension trait for the encryption plugin
pub trait EncryptionPluginExt<R: Runtime> {
    /// Unlock the app with a password
    fn unlock_app(&self, password: String) -> Result<bool, Error>;

    /// Lock the app
    fn lock_app(&self) -> Result<(), Error>;

    /// Get the encryption status
    fn get_encryption_status(&self) -> Result<bool, Error>;

    /// Change the encryption password
    fn change_password(&self, old_password: String, new_password: String) -> Result<(), Error>;

    /// Get the key manager
    fn key_manager(&self) -> Result<KeyManager, Error>;

    /// Get the app data directory
    fn app_data_dir(&self) -> Result<PathBuf, Error>;

    /// Get the salt file path
    fn salt_file_path(&self) -> Result<PathBuf, Error>;

    /// Save the salt to a file
    fn save_salt(&self, salt: &str) -> Result<(), Error>;

    /// Load the salt from a file
    fn load_salt(&self) -> Result<Option<String>, Error>;
}

impl<R: Runtime, T: Manager<R>> EncryptionPluginExt<R> for T {
    fn unlock_app(&self, password: String) -> Result<bool, Error> {
        let state: State<EncryptionState> = self.state();
        let mut key_manager = state.key_manager.lock().unwrap();

        // Try to load the salt
        if let Some(salt) = self.load_salt()? {
            key_manager.set_salt(salt);
        } else {
            // If no salt exists, generate a new one and save it
            let salt = key_manager.generate_salt();
            self.save_salt(&salt)?;
        }

        // Derive the key from the password
        key_manager.derive_key(password.as_bytes())?;

        // Return true if the key is available
        Ok(key_manager.state() == KeyState::Available)
    }

    fn lock_app(&self) -> Result<(), Error> {
        let state: State<EncryptionState> = self.state();
        let key_manager = state.key_manager.lock().unwrap();

        // Clear the key
        key_manager.clear_key();

        Ok(())
    }

    fn get_encryption_status(&self) -> Result<bool, Error> {
        let state: State<EncryptionState> = self.state();
        let key_manager = state.key_manager.lock().unwrap();

        // Return true if the key is available
        Ok(key_manager.state() == KeyState::Available)
    }

    fn change_password(&self, old_password: String, new_password: String) -> Result<(), Error> {
        let state: State<EncryptionState> = self.state();
        let mut key_manager = state.key_manager.lock().unwrap();

        // First, verify the old password
        let old_salt = key_manager.salt().ok_or(Error::SaltNotFound)?.to_string();
        key_manager.derive_key(old_password.as_bytes())?;

        if key_manager.state() != KeyState::Available {
            return Err(Error::IncorrectPassword);
        }

        // Generate a new salt for the new password
        let new_salt = key_manager.generate_salt();

        // Derive the key from the new password
        key_manager.derive_key(new_password.as_bytes())?;

        // Save the new salt
        self.save_salt(&new_salt)?;

        Ok(())
    }

    fn key_manager(&self) -> Result<KeyManager, Error> {
        let state: State<EncryptionState> = self.state();
        let key_manager = state.key_manager.lock().unwrap().clone();
        Ok(key_manager)
    }

    fn app_data_dir(&self) -> Result<PathBuf, Error> {
        let app_handle = self.app_handle();
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| Error::Other(format!("Failed to get app data directory: {}", e)))?;
        Ok(app_data_dir)
    }

    fn salt_file_path(&self) -> Result<PathBuf, Error> {
        let app_data_dir = self.app_data_dir()?;
        let salt_file_path = app_data_dir.join("encryption_salt");
        Ok(salt_file_path)
    }

    fn save_salt(&self, salt: &str) -> Result<(), Error> {
        let salt_file_path = self.salt_file_path()?;

        // Create the directory if it doesn't exist
        if let Some(parent) = salt_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write the salt to the file
        std::fs::write(&salt_file_path, salt)
            .map_err(|e| Error::SaltSaveFailed(format!("Failed to save salt: {}", e)))?;

        Ok(())
    }

    fn load_salt(&self) -> Result<Option<String>, Error> {
        let salt_file_path = self.salt_file_path()?;

        // Check if the salt file exists
        if !salt_file_path.exists() {
            return Ok(None);
        }

        // Read the salt from the file
        let salt = std::fs::read_to_string(&salt_file_path)
            .map_err(|e| Error::SaltLoadFailed(format!("Failed to load salt: {}", e)))?;

        Ok(Some(salt))
    }
}
