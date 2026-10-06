use serde::Serialize;
use tauri::State;

use crate::{
    crypto::{
        derive_wrap_key, encode_recovery_key, generate_master_key,
        load_from_keychain, store_in_keychain, unwrap_master_key, wrap_master_key, zero_key,
    },
    AppState,
};

#[derive(Serialize)]
pub struct VaultStatus {
    pub initialized: bool,
    pub vault_password_enabled: bool,
    pub locked: bool,
}

#[derive(Serialize)]
pub struct RecoveryKey {
    pub key_b32: String,
}

/// Return current vault status without unlocking.
#[tauri::command]
pub async fn get_vault_status(state: State<'_, AppState>) -> Result<VaultStatus, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let crypto = state.crypto_key.lock().map_err(|e| e.to_string())?;

    let initialized = db.is_some();
    let locked = crypto.is_none();

    let vault_password_enabled = if let Some(conn) = db.as_ref() {
        let val: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'vault_password_enabled'",
                [],
                |row| row.get(0),
            )
            .ok();
        val.as_deref() == Some("true")
    } else {
        false
    };

    Ok(VaultStatus {
        initialized,
        vault_password_enabled,
        locked,
    })
}

/// First-time vault initialization. Generates master key, stores it, creates DB.
#[tauri::command]
pub async fn initialize_vault(
    vault_password: Option<String>,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<RecoveryKey, String> {
    use crate::db::{open_db, run_migrations};
    use tauri::Manager;

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not resolve app data dir: {e}"))?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let db_path = data_dir.join("journal.db");

    let master_key = generate_master_key();
    let recovery_key_b32 = encode_recovery_key(&master_key);

    // Store in OS keychain
    store_in_keychain(&master_key).map_err(|e| format!("Keychain error: {e}"))?;

    // Open / create DB and run migrations
    let conn = open_db(&db_path).map_err(|e| format!("DB error: {e}"))?;
    run_migrations(&conn).map_err(|e| format!("Migration error: {e}"))?;

    // If vault password provided, wrap key and store wrapped version
    if let Some(ref pw) = vault_password {
        let (wrap_key, salt_hex) =
            derive_wrap_key(pw, None).map_err(|e| format!("KDF error: {e}"))?;
        let wrapped = wrap_master_key(&master_key, &wrap_key)
            .map_err(|e| format!("Wrap error: {e}"))?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('vault_password_enabled', 'true'),
             ('vault_wrapped_key', ?1),
             ('vault_kdf_salt', ?2),
             ('initialized', 'true')",
            rusqlite::params![wrapped, salt_hex],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            "UPDATE settings SET value = 'true' WHERE key = 'initialized'",
            [],
        )
        .map_err(|e| e.to_string())?;
    }

    // Load into state
    let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let mut crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    *db_guard = Some(conn);
    *crypto_guard = Some(master_key);

    Ok(RecoveryKey {
        key_b32: recovery_key_b32,
    })
}

/// Unlock the vault (load the master key from keychain or derive from password).
#[tauri::command]
pub async fn unlock_vault(
    password: Option<String>,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    use crate::db::{open_db, run_migrations};
    use tauri::Manager;

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Could not resolve app data dir: {e}"))?;
    let db_path = data_dir.join("journal.db");

    let conn = open_db(&db_path).map_err(|e| format!("DB error: {e}"))?;
    run_migrations(&conn).map_err(|e| format!("Migration error: {e}"))?;

    // Determine master key
    let vault_pw_enabled: bool = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'vault_password_enabled'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .as_deref()
        == Some("true");

    let master_key = if vault_pw_enabled {
        let pw = password.ok_or("Vault password required")?;
        let wrapped: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'vault_wrapped_key'",
                [],
                |row| row.get(0),
            )
            .map_err(|_| "No wrapped key found — vault may be corrupted")?;
        let salt: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'vault_kdf_salt'",
                [],
                |row| row.get(0),
            )
            .map_err(|_| "No KDF salt found — vault may be corrupted")?;
        let (wrap_key, _) =
            derive_wrap_key(&pw, Some(&salt)).map_err(|e| format!("KDF error: {e}"))?;
        unwrap_master_key(&wrapped, &wrap_key)
            .map_err(|_| "Incorrect password — wrong vault password".to_string())?
    } else {
        load_from_keychain().map_err(|e| format!("Keychain error: {e}"))?
    };

    let mut db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let mut crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    *db_guard = Some(conn);
    *crypto_guard = Some(master_key);

    Ok(())
}

/// Lock the vault — zeroes the in-memory key.
#[tauri::command]
pub async fn lock_vault(state: State<'_, AppState>) -> Result<(), String> {
    let mut crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut key) = *crypto_guard {
        zero_key(key);
    }
    *crypto_guard = None;
    Ok(())
}

/// Set or change the vault password.
#[tauri::command]
pub async fn set_vault_password(
    current_password: Option<String>,
    new_password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;

    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    let master_key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    // Verify current password if vault password is enabled
    let vault_pw_enabled: bool = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'vault_password_enabled'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .as_deref()
        == Some("true");

    if vault_pw_enabled {
        let cur_pw = current_password.ok_or("Current password required")?;
        let wrapped: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'vault_wrapped_key'",
                [],
                |row| row.get(0),
            )
            .map_err(|_| "No wrapped key found")?;
        let salt: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'vault_kdf_salt'",
                [],
                |row| row.get(0),
            )
            .map_err(|_| "No salt found")?;
        let (wrap_key, _) =
            derive_wrap_key(&cur_pw, Some(&salt)).map_err(|e| format!("KDF: {e}"))?;
        unwrap_master_key(&wrapped, &wrap_key)
            .map_err(|_| "Incorrect current password".to_string())?;
    }

    // Re-wrap with new password
    let (new_wrap_key, new_salt_hex) =
        derive_wrap_key(&new_password, None).map_err(|e| format!("KDF: {e}"))?;
    let new_wrapped = wrap_master_key(master_key, &new_wrap_key)
        .map_err(|e| format!("Wrap error: {e}"))?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES
         ('vault_password_enabled', 'true'),
         ('vault_wrapped_key', ?1),
         ('vault_kdf_salt', ?2)",
        rusqlite::params![new_wrapped, new_salt_hex],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
