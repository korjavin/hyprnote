use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod commands;
mod error;
mod ext;
mod store;

pub use error::Error;
pub use ext::EncryptionPluginExt;
pub use store::EncryptionState;

const PLUGIN_NAME: &str = "encryption";

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            commands::unlock_app,
            commands::lock_app,
            commands::get_encryption_status,
            commands::change_password,
        ])
        .setup(|app| {
            app.manage(store::EncryptionState::default());
            Ok(())
        })
        .build()
}

fn make_specta_builder<R: tauri::Runtime>() -> tauri_specta::Builder<R> {
    tauri_specta::Builder::<R>::new()
        .plugin_name(PLUGIN_NAME)
        .commands(tauri_specta::collect_commands![
            commands::unlock_app::<tauri::Wry>,
            commands::lock_app::<tauri::Wry>,
            commands::get_encryption_status::<tauri::Wry>,
            commands::change_password::<tauri::Wry>,
        ])
}
