use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{
    crypto::{decrypt_str, encrypt_str},
    AppState,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryTag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

/// Lightweight entry for list views (no decrypted markdown).
#[derive(Debug, Serialize)]
pub struct EntryRow {
    pub id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub title: Option<String>,
    pub content_preview: String,
    pub mood: Option<String>,
    pub word_count: i64,
    pub pinned: bool,
    pub deleted_at: Option<String>,
    pub tags: Vec<EntryTag>,
}

/// Full entry including decrypted markdown (only returned when editing).
#[derive(Debug, Serialize)]
pub struct EntryFull {
    pub id: i64,
    pub created_at: String,
    pub updated_at: String,
    pub title: Option<String>,
    pub markdown: String,
    pub content_preview: String,
    pub mood: Option<String>,
    pub word_count: i64,
    pub pinned: bool,
    pub deleted_at: Option<String>,
    pub tags: Vec<EntryTag>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntryPayload {
    pub title: Option<String>,
    pub markdown: String,
    pub mood: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntryPayload {
    pub id: i64,
    pub title: Option<String>,
    pub markdown: String,
    pub mood: Option<String>,
    pub tags: Option<Vec<String>>,
}

fn count_words(html: &str) -> i64 {
    let stripped = html.replace('<', " ").replace('>', " ");
    stripped
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .count() as i64
}

/// Derive a plain-text preview (first 200 chars, stripped of HTML tags).
fn make_preview(html: &str) -> String {
    let stripped: String = {
        let mut out = String::with_capacity(html.len());
        let mut in_tag = false;
        for c in html.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                c if !in_tag => out.push(c),
                _ => {}
            }
        }
        out
    };
    let trimmed = stripped.trim().to_string();
    if trimmed.len() > 200 {
        trimmed[..200].to_string()
    } else {
        trimmed
    }
}

fn load_tags_for_entry(conn: &rusqlite::Connection, entry_id: i64) -> Vec<EntryTag> {
    let mut stmt = match conn.prepare(
        "SELECT t.id, t.name, t.color, t.created_at
         FROM tags t
         JOIN entry_tags et ON et.tag_id = t.id
         WHERE et.entry_id = ?1
         ORDER BY t.name",
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let mut rows = match stmt.query(rusqlite::params![entry_id]) {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    let mut tags = Vec::new();
    while let Ok(Some(row)) = rows.next() {
        tags.push(EntryTag {
            id: row.get(0).unwrap_or(0),
            name: row.get(1).unwrap_or_default(),
            color: row.get(2).unwrap_or_default(),
            created_at: row.get(3).unwrap_or_default(),
        });
    }
    tags
}

fn sync_tags(conn: &rusqlite::Connection, entry_id: i64, tag_names: &[String]) -> rusqlite::Result<()> {
    // Remove old links
    conn.execute("DELETE FROM entry_tags WHERE entry_id = ?1", [entry_id])?;

    for name in tag_names {
        // Upsert tag
        conn.execute(
            "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
            rusqlite::params![name],
        )?;
        let tag_id: i64 = conn.query_row(
            "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
            rusqlite::params![name],
            |row| row.get(0),
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO entry_tags (entry_id, tag_id) VALUES (?1, ?2)",
            rusqlite::params![entry_id, tag_id],
        )?;
    }
    Ok(())
}

#[tauri::command]
pub async fn list_entries(
    limit: i64,
    offset: i64,
    state: State<'_, AppState>,
) -> Result<Vec<EntryRow>, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;

    type RawRow = (i64, String, String, Option<String>, String, Option<String>, i64, bool, Option<String>);

    let raw_rows: Vec<RawRow> = {
        let mut stmt = conn
            .prepare(
                "SELECT id, created_at, updated_at, title, markdown, mood, word_count, pinned, deleted_at
                 FROM entries
                 WHERE deleted_at IS NULL
                 ORDER BY pinned DESC, created_at DESC
                 LIMIT ?1 OFFSET ?2",
            )
            .map_err(|e| e.to_string())?;
        let mut query_rows = stmt
            .query(rusqlite::params![limit, offset])
            .map_err(|e| e.to_string())?;
        let mut v: Vec<RawRow> = Vec::new();
        while let Some(row) = query_rows.next().map_err(|e| e.to_string())? {
            v.push((
                row.get(0).unwrap_or(0),
                row.get(1).unwrap_or_default(),
                row.get(2).unwrap_or_default(),
                row.get(3).ok(),
                row.get(4).unwrap_or_default(),
                row.get(5).ok(),
                row.get(6).unwrap_or(0),
                row.get(7).unwrap_or(false),
                row.get(8).ok(),
            ));
        }
        v
    };

    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    let key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    let mut entries = Vec::new();
    for (id, created_at, updated_at, title, encrypted_md, mood, word_count, pinned, deleted_at) in raw_rows {
        let markdown = decrypt_str(&encrypted_md, key)
            .unwrap_or_else(|_| "[decryption error]".to_string());
        let content_preview = make_preview(&markdown);
        let tags = load_tags_for_entry(conn, id);

        entries.push(EntryRow {
            id,
            created_at,
            updated_at,
            title,
            content_preview,
            mood,
            word_count,
            pinned,
            deleted_at,
            tags,
        });
    }

    Ok(entries)
}

#[tauri::command]
pub async fn get_entry(id: i64, state: State<'_, AppState>) -> Result<EntryFull, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    let key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    let row = conn
        .query_row(
            "SELECT id, created_at, updated_at, title, markdown, mood, word_count, pinned, deleted_at
             FROM entries WHERE id = ?1",
            [id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, bool>(7)?,
                    row.get::<_, Option<String>>(8)?,
                ))
            },
        )
        .map_err(|e| format!("Entry not found: {e}"))?;

    let (eid, created_at, updated_at, title, encrypted_md, mood, word_count, pinned, deleted_at) = row;
    let markdown =
        decrypt_str(&encrypted_md, key).map_err(|e| format!("Decrypt error: {e}"))?;
    let content_preview = make_preview(&markdown);
    let tags = load_tags_for_entry(conn, eid);

    Ok(EntryFull {
        id: eid,
        created_at,
        updated_at,
        title,
        markdown,
        content_preview,
        mood,
        word_count,
        pinned,
        deleted_at,
        tags,
    })
}

#[tauri::command]
pub async fn create_entry(
    payload: CreateEntryPayload,
    state: State<'_, AppState>,
) -> Result<EntryFull, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    let key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    let word_count = count_words(&payload.markdown);
    let encrypted = encrypt_str(&payload.markdown, key)
        .map_err(|e| format!("Encrypt error: {e}"))?;

    conn.execute(
        "INSERT INTO entries (title, markdown, mood, word_count) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![payload.title, encrypted, payload.mood, word_count],
    )
    .map_err(|e| e.to_string())?;
    let entry_id = conn.last_insert_rowid();

    if let Some(ref tags) = payload.tags {
        sync_tags(conn, entry_id, tags).map_err(|e| e.to_string())?;
    }

    conn.execute(
        "INSERT INTO audit (event, details) VALUES ('entry_created', json_object('id', ?1, 'word_count', ?2))",
        rusqlite::params![entry_id, word_count],
    )
    .ok();

    // Re-fetch the full entry to return consistent data
    let tags = load_tags_for_entry(conn, entry_id);
    let created_at: String = conn.query_row(
        "SELECT created_at FROM entries WHERE id = ?1",
        [entry_id],
        |r| r.get(0),
    ).map_err(|e| e.to_string())?;
    let preview = make_preview(&payload.markdown);

    Ok(EntryFull {
        id: entry_id,
        created_at: created_at.clone(),
        updated_at: created_at,
        title: payload.title,
        markdown: payload.markdown,
        content_preview: preview,
        mood: payload.mood,
        word_count,
        pinned: false,
        deleted_at: None,
        tags,
    })
}

#[tauri::command]
pub async fn update_entry(
    payload: UpdateEntryPayload,
    state: State<'_, AppState>,
) -> Result<EntryFull, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    let key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    let word_count = count_words(&payload.markdown);
    let encrypted = encrypt_str(&payload.markdown, key)
        .map_err(|e| format!("Encrypt error: {e}"))?;

    conn.execute(
        "UPDATE entries SET
            title = ?1,
            markdown = ?2,
            mood = ?3,
            word_count = ?4,
            updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         WHERE id = ?5",
        rusqlite::params![payload.title, encrypted, payload.mood, word_count, payload.id],
    )
    .map_err(|e| e.to_string())?;

    if let Some(ref tags) = payload.tags {
        sync_tags(conn, payload.id, tags).map_err(|e| e.to_string())?;
    }

    // Return the updated entry directly without cloning state
    let tags = load_tags_for_entry(conn, payload.id);
    let (created_at, updated_at): (String, String) = conn.query_row(
        "SELECT created_at, updated_at FROM entries WHERE id = ?1",
        [payload.id],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).map_err(|e| e.to_string())?;
    let preview = make_preview(&payload.markdown);

    Ok(EntryFull {
        id: payload.id,
        created_at,
        updated_at,
        title: payload.title,
        markdown: payload.markdown,
        content_preview: preview,
        mood: payload.mood,
        word_count,
        pinned: false,
        deleted_at: None,
        tags,
    })
}

#[tauri::command]
pub async fn delete_entry(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    // Soft-delete
    conn.execute(
        "UPDATE entries SET deleted_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now') WHERE id = ?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn pin_entry(id: i64, pinned: bool, state: State<'_, AppState>) -> Result<(), String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    conn.execute(
        "UPDATE entries SET pinned = ?1 WHERE id = ?2",
        rusqlite::params![pinned as i64, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
