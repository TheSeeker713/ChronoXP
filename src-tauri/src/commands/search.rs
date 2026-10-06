//! Full-text search over decrypted entry content held in memory.
//!
//! Strategy:
//! 1. Fetch all non-deleted entries (encrypted markdown + metadata).
//! 2. Decrypt each entry's markdown in memory.
//! 3. Case-insensitive substring match against the query.
//! 4. Return ranked results (title matches scored higher).
//!
//! For Phase 5, this will be replaced with llama.cpp semantic search +
//! sqlite-vec vector similarity. The query interface is identical.

use serde::Serialize;
use tauri::State;

use crate::{
    commands::entries::EntryTag,
    crypto::decrypt_str,
    AppState,
};

#[derive(Serialize)]
pub struct SearchResult {
    pub id: i64,
    pub created_at: String,
    pub title: Option<String>,
    pub snippet: String,
    pub tags: Vec<EntryTag>,
    pub score: i32,
}

fn extract_snippet(text: &str, query: &str, window: usize) -> String {
    let lower_text = text.to_lowercase();
    let lower_query = query.to_lowercase();
    if let Some(pos) = lower_text.find(&lower_query) {
        let start = pos.saturating_sub(window / 2);
        let end = (pos + lower_query.len() + window / 2).min(text.len());
        let mut snippet = text[start..end].to_string();
        if start > 0 {
            snippet = format!("…{snippet}");
        }
        if end < text.len() {
            snippet.push('…');
        }
        snippet
    } else {
        text.chars().take(150).collect()
    }
}

#[tauri::command]
pub async fn search_entries(query: String, state: State<'_, AppState>) -> Result<Vec<SearchResult>, String> {
    if query.trim().is_empty() {
        return Ok(vec![]);
    }

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    let key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    // Fetch all active entries
    let mut stmt = conn
        .prepare(
            "SELECT id, created_at, title, markdown FROM entries
             WHERE deleted_at IS NULL
             ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows: Vec<(i64, String, Option<String>, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .flatten()
        .collect();

    let lower_query = query.to_lowercase();
    let mut results: Vec<SearchResult> = Vec::new();

    for (id, created_at, title, encrypted_md) in rows {
        let markdown = match decrypt_str(&encrypted_md, key) {
            Ok(m) => m,
            Err(_) => continue,
        };

        // Strip HTML tags for text search
        let plain: String = {
            let mut out = String::with_capacity(markdown.len());
            let mut in_tag = false;
            for c in markdown.chars() {
                match c {
                    '<' => in_tag = true,
                    '>' => in_tag = false,
                    c if !in_tag => out.push(c),
                    _ => {}
                }
            }
            out
        };

        let lower_plain = plain.to_lowercase();
        let lower_title = title.as_deref().unwrap_or("").to_lowercase();

        let mut score = 0i32;
        if lower_title.contains(&lower_query) {
            score += 10;
        }
        if lower_plain.contains(&lower_query) {
            score += 1;
        }

        if score > 0 {
            let snippet = extract_snippet(&plain, &query, 200);

            // Load tags
            let mut tag_stmt = conn.prepare(
                "SELECT t.id, t.name, t.color, t.created_at
                 FROM tags t JOIN entry_tags et ON et.tag_id = t.id
                 WHERE et.entry_id = ?1",
            ).ok();
            let tags: Vec<EntryTag> = tag_stmt
                .as_mut()
                .and_then(|s| {
                    s.query_map([id], |row| {
                        Ok(EntryTag {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            color: row.get(2)?,
                            created_at: row.get(3)?,
                        })
                    })
                    .ok()
                    .map(|iter| iter.flatten().collect())
                })
                .unwrap_or_default();

            results.push(SearchResult {
                id,
                created_at,
                title,
                snippet,
                tags,
                score,
            });
        }
    }

    results.sort_by(|a, b| b.score.cmp(&a.score).then(b.created_at.cmp(&a.created_at)));
    results.truncate(50);
    Ok(results)
}
