pub mod backup;
pub mod commands;
pub mod crypto;
pub mod db;
pub mod importers;

use std::sync::Mutex;

use rusqlite::Connection;

/// Shared application state injected into all Tauri commands.
pub struct AppState {
    /// The open database connection (None when locked or not yet initialized).
    pub db: Mutex<Option<Connection>>,
    /// The in-memory master key (None when locked).
    /// Zeroed via `zeroize` on lock.
    pub crypto_key: Mutex<Option<[u8; 32]>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            db: Mutex::new(None),
            crypto_key: Mutex::new(None),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // Security
            commands::security::get_vault_status,
            commands::security::initialize_vault,
            commands::security::unlock_vault,
            commands::security::lock_vault,
            commands::security::set_vault_password,
            // Entries
            commands::entries::list_entries,
            commands::entries::get_entry,
            commands::entries::create_entry,
            commands::entries::update_entry,
            commands::entries::delete_entry,
            commands::entries::pin_entry,
            // Tags
            commands::tags::list_tags,
            commands::tags::create_tag,
            commands::tags::delete_tag,
            // Search
            commands::search::search_entries,
            // Export / Import
            commands::export::export_journal,
            commands::export::import_journal,
            // Settings
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ChronoXP");
}
