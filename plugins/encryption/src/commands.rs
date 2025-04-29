use tauri::{command, AppHandle, Runtime, State};
use specta::Type;

use crate::{ext::EncryptionPluginExt, Error};

/// Unlock the app with a password
#[command]
#[specta::specta]
pub async fn unlock_app<R: Runtime>(
    app: AppHandle<R>,
    password: String,
) -> Result<bool, Error> {
    if password.is_empty() {
        return Err(Error::PasswordRequired);
    }

    app.unlock_app(password)
}

/// Lock the app
#[command]
#[specta::specta]
pub async fn lock_app<R: Runtime>(app: AppHandle<R>) -> Result<(), Error> {
    app.lock_app()
}

/// Get the encryption status
#[command]
#[specta::specta]
pub async fn get_encryption_status<R: Runtime>(app: AppHandle<R>) -> Result<bool, Error> {
    app.get_encryption_status()
}

/// Change the encryption password
#[command]
#[specta::specta]
pub async fn change_password<R: Runtime>(
    app: AppHandle<R>,
    old_password: String,
    new_password: String,
) -> Result<(), Error> {
    if old_password.is_empty() || new_password.is_empty() {
        return Err(Error::PasswordRequired);
    }

    app.change_password(old_password, new_password)
}
