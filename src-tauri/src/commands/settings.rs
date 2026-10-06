use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::State;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub font_family: String,
    pub auto_lock_minutes: i64,
    pub vault_password_enabled: bool,
    pub simple_mode: bool,
    pub schema_version: i64,
}

fn get_setting(conn: &rusqlite::Connection, key: &str) -> Option<String> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        rusqlite::params![key],
        |row| row.get(0),
    )
    .ok()
}

fn parse_bool(s: &str) -> bool {
    matches!(s, "true" | "1")
}

fn parse_i64(s: &str) -> i64 {
    s.trim_matches('"').parse().unwrap_or(0)
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;

    Ok(AppSettings {
        theme: get_setting(conn, "theme")
            .unwrap_or_else(|| "\"dark\"".into())
            .trim_matches('"')
            .to_string(),
        font_family: get_setting(conn, "font_family")
            .unwrap_or_else(|| "\"Inter\"".into())
            .trim_matches('"')
            .to_string(),
        auto_lock_minutes: get_setting(conn, "auto_lock_minutes")
            .as_deref()
            .map(parse_i64)
            .unwrap_or(15),
        vault_password_enabled: get_setting(conn, "vault_password_enabled")
            .as_deref()
            .map(parse_bool)
            .unwrap_or(false),
        simple_mode: get_setting(conn, "simple_mode")
            .as_deref()
            .map(parse_bool)
            .unwrap_or(false),
        schema_version: get_setting(conn, "schema_version")
            .as_deref()
            .map(parse_i64)
            .unwrap_or(1),
    })
}

#[tauri::command]
pub async fn update_settings(
    settings: serde_json::Map<String, Value>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;

    for (key, value) in &settings {
        let str_value = match value {
            Value::String(s) => format!("\"{}\"", s),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            other => other.to_string(),
        };

        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value,
             updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')",
            rusqlite::params![key, str_value],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
