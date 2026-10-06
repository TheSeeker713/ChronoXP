use serde::{Deserialize, Serialize};
use tauri::State;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagRow {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

#[tauri::command]
pub async fn list_tags(state: State<'_, AppState>) -> Result<Vec<TagRow>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;

    let mut stmt = conn
        .prepare("SELECT id, name, color, created_at FROM tags ORDER BY name")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(TagRow {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .flatten()
        .collect();

    Ok(rows)
}

#[tauri::command]
pub async fn create_tag(
    name: String,
    color: String,
    state: State<'_, AppState>,
) -> Result<TagRow, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;

    conn.execute(
        "INSERT OR IGNORE INTO tags (name, color) VALUES (?1, ?2)",
        rusqlite::params![name.trim(), color],
    )
    .map_err(|e| e.to_string())?;

    let row = conn
        .query_row(
            "SELECT id, name, color, created_at FROM tags WHERE name = ?1 COLLATE NOCASE",
            rusqlite::params![name.trim()],
            |row| {
                Ok(TagRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                    created_at: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(row)
}

#[tauri::command]
pub async fn delete_tag(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    conn.execute("DELETE FROM tags WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
