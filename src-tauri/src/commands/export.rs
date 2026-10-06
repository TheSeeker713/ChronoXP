//! Export and import of `.chrxp` archives.
//!
//! Format: ZIP archive with an optional AES-256-GCM encryption envelope.
//!
//! File layout on disk:
//!   4 bytes  magic  b"CHRX"
//!   1 byte   version  0=plain, 1=encrypted
//!   [v1] 16 bytes  PBKDF2 salt
//!   [v1] 12 bytes  AES-GCM nonce
//!   [v1] N bytes   ciphertext + 16-byte GCM tag
//!   [v0] N bytes   raw ZIP bytes

use std::io::{Cursor, Read, Write};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::State;

use crate::{
    crypto::{decrypt, decrypt_str, derive_export_key, encrypt, encrypt_str},
    AppState,
};

const MAGIC: &[u8; 4] = b"CHRX";

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    pub include_entries: bool,
    pub include_attachments: bool,
    pub include_settings: bool,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExportResult {
    pub path: String,
    pub entry_count: usize,
}

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub imported: usize,
    pub skipped: usize,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Build the inner ZIP bytes for a `.chrxp` export.
fn build_zip(
    conn: &rusqlite::Connection,
    crypto_key: &[u8; 32],
    options: &ExportOptions,
) -> anyhow::Result<(Vec<u8>, usize)> {
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    let mut zip_buf = Vec::new();
    let cursor = Cursor::new(&mut zip_buf);
    let mut zip = ZipWriter::new(cursor);
    let file_opts =
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut entry_count = 0usize;

    if options.include_entries {
        // Collect entries first to release the borrow on conn
        type EntryRow = (i64, String, String, Option<String>, String, Option<String>, i64);
        let entry_rows: Vec<EntryRow> = {
            let mut stmt = conn.prepare(
                "SELECT id, created_at, updated_at, title, markdown, mood, word_count
                 FROM entries WHERE deleted_at IS NULL ORDER BY created_at",
            )?;
            let mut rows = stmt.query([])?;
            let mut v: Vec<EntryRow> = Vec::new();
            while let Some(row) = rows.next()? {
                v.push((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ));
            }
            v
        };

        for (id, created_at, updated_at, title, encrypted_md, mood, word_count) in entry_rows {
            let markdown = decrypt_str(&encrypted_md, crypto_key)
                .unwrap_or_else(|_| "[decryption error]".into());

            // Collect tags for this entry (separate query, conn is free now)
            let tags: Vec<String> = {
                let mut s = conn.prepare(
                    "SELECT t.name FROM tags t
                     JOIN entry_tags et ON et.tag_id = t.id
                     WHERE et.entry_id = ?1",
                )?;
                let mut tag_rows = s.query(rusqlite::params![id])?;
                let mut tv: Vec<String> = Vec::new();
                while let Some(r) = tag_rows.next()? {
                    if let Ok(name) = r.get::<_, String>(0) { tv.push(name); }
                }
                tv
            };

            let date_slug = created_at.get(..10).unwrap_or("unknown").replace('-', "");
            let filename = format!("entries/{id}_{date_slug}.md");

            let frontmatter = format!(
                "---\nid: {id}\ncreated_at: {created_at}\nupdated_at: {updated_at}\ntags: [{}]\nmood: {}\nword_count: {word_count}\n---\n\n",
                tags.join(", "),
                mood.as_deref().unwrap_or("")
            );
            let title_block = title
                .as_deref()
                .map(|t| format!("# {t}\n\n"))
                .unwrap_or_default();
            let content = format!("{frontmatter}{title_block}{markdown}");

            zip.start_file(&filename, file_opts)?;
            zip.write_all(content.as_bytes())?;
            entry_count += 1;
        }

        // Tags metadata
        let all_tags: Vec<serde_json::Value> = {
            let mut s = conn.prepare("SELECT id, name, color FROM tags ORDER BY name")?;
            let mut tag_rows = s.query([])?;
            let mut tv: Vec<serde_json::Value> = Vec::new();
            while let Some(r) = tag_rows.next()? {
                let id: i64 = r.get(0)?;
                let name: String = r.get(1)?;
                let color: String = r.get(2)?;
                tv.push(json!({"id": id, "name": name, "color": color}));
            }
            tv
        };

        zip.start_file("meta/tags.json", file_opts)?;
        zip.write_all(serde_json::to_string_pretty(&all_tags)?.as_bytes())?;
    }

    if options.include_settings {
        let settings: Vec<(String, String)> = {
            let mut s = conn.prepare("SELECT key, value FROM settings")?;
            let mut setting_rows = s.query([])?;
            let mut sv: Vec<(String, String)> = Vec::new();
            while let Some(r) = setting_rows.next()? {
                let k: String = r.get(0)?;
                let v: String = r.get(1)?;
                if !k.starts_with("vault_") && k != "initialized" {
                    sv.push((k, v));
                }
            }
            sv
        };
        let settings_map: serde_json::Map<String, serde_json::Value> = settings
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::String(v)))
            .collect();
        zip.start_file("meta/settings.json", file_opts)?;
        zip.write_all(serde_json::to_string_pretty(&settings_map)?.as_bytes())?;
    }

    // Manifest
    let manifest = json!({
        "version": "1.0.0",
        "export_date": chrono::Utc::now().to_rfc3339(),
        "entry_count": entry_count,
        "encrypted": options.password.is_some(),
        "encryption_method": if options.password.is_some() { "AES-256-GCM+PBKDF2" } else { "none" },
        "includes": {
            "entries": options.include_entries,
            "attachments": options.include_attachments,
            "settings": options.include_settings,
        },
        "chronoxp_version": "0.1.0",
    });
    zip.start_file("manifest.json", file_opts)?;
    zip.write_all(serde_json::to_string_pretty(&manifest)?.as_bytes())?;

    drop(zip);
    Ok((zip_buf, entry_count))
}

/// Write an unencrypted archive.
fn write_plain_archive(zip_bytes: &[u8], path: &str) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(MAGIC)?;
    file.write_all(&[0u8])?;
    file.write_all(zip_bytes)?;
    Ok(())
}

/// Write an encrypted archive.
fn write_encrypted_archive(zip_bytes: &[u8], path: &str, password: &str) -> anyhow::Result<()> {
    use rand::RngCore;
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);

    let key = derive_export_key(password, &salt);
    let ciphertext_b64 = encrypt(zip_bytes, &key)?;

    use base64::{engine::general_purpose::STANDARD as B64, Engine};
    let raw = B64.decode(&ciphertext_b64)?;

    let mut file = std::fs::File::create(path)?;
    file.write_all(MAGIC)?;
    file.write_all(&[1u8])?;
    file.write_all(&salt)?;
    file.write_all(&raw)?;
    Ok(())
}

/// Read and optionally decrypt a `.chrxp` file, returning raw ZIP bytes.
pub fn read_archive(file_path: &str, password: Option<&str>) -> anyhow::Result<Vec<u8>> {
    use base64::{engine::general_purpose::STANDARD as B64, Engine};

    let mut file = std::fs::File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    if data.len() < 5 || &data[..4] != MAGIC {
        anyhow::bail!("Not a valid .chrxp file");
    }

    let version = data[4];
    let payload = &data[5..];

    match version {
        0 => Ok(payload.to_vec()),
        1 => {
            let pw = password.ok_or_else(|| anyhow::anyhow!("Password required for encrypted archive"))?;
            if payload.len() < 16 {
                anyhow::bail!("File too short to contain salt");
            }
            let (salt, raw_cipher) = payload.split_at(16);
            let key = derive_export_key(pw, salt);
            let b64_cipher = B64.encode(raw_cipher);
            let zip_bytes = decrypt(&b64_cipher, &key)
                .map_err(|_| anyhow::anyhow!("Decryption failed — wrong password?"))?;
            Ok(zip_bytes)
        }
        v => anyhow::bail!("Unknown .chrxp version: {v}"),
    }
}

// ── Tauri commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn export_journal(
    options: ExportOptions,
    save_path: String,
    state: State<'_, AppState>,
) -> Result<ExportResult, String> {
    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    let key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    let (zip_bytes, entry_count) = build_zip(conn, key, &options).map_err(|e| e.to_string())?;

    if let Some(ref pw) = options.password {
        write_encrypted_archive(&zip_bytes, &save_path, pw).map_err(|e| e.to_string())?;
    } else {
        write_plain_archive(&zip_bytes, &save_path).map_err(|e| e.to_string())?;
    }

    conn.execute(
        "INSERT INTO audit (event, details) VALUES ('export_completed', json_object('entry_count', ?1))",
        rusqlite::params![entry_count as i64],
    )
    .ok();

    Ok(ExportResult {
        path: save_path,
        entry_count,
    })
}

#[tauri::command]
pub async fn import_journal(
    file_path: String,
    password: Option<String>,
    state: State<'_, AppState>,
) -> Result<ImportResult, String> {
    use zip::ZipArchive;

    let db_guard = state.db.lock().map_err(|e| e.to_string())?;
    let conn = db_guard.as_ref().ok_or("Vault is locked")?;
    let crypto_guard = state.crypto_key.lock().map_err(|e| e.to_string())?;
    let key = crypto_guard.as_ref().ok_or("Vault is locked")?;

    let zip_bytes =
        read_archive(&file_path, password.as_deref()).map_err(|e| e.to_string())?;

    let cursor = Cursor::new(zip_bytes);
    let mut zip = ZipArchive::new(cursor).map_err(|e| format!("Invalid ZIP: {e}"))?;

    let mut imported = 0usize;
    let mut skipped = 0usize;

    // Collect file contents first to avoid borrow issues
    let mut files_to_import: Vec<(String, String)> = Vec::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(|e| e.to_string())?;
        let name = file.name().to_string();
        if name.starts_with("entries/") && name.ends_with(".md") {
            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|e| e.to_string())?;
            files_to_import.push((name, content));
        }
    }

    for (_name, content) in files_to_import {
        let entry_id = parse_frontmatter_id(&content);

        if let Some(eid) = entry_id {
            let exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) FROM entries WHERE id = ?1",
                    [eid],
                    |r| r.get::<_, i64>(0),
                )
                .unwrap_or(0)
                > 0;

            if exists {
                skipped += 1;
                continue;
            }

            let body = extract_body(&content);
            let encrypted = encrypt_str(&body, key).map_err(|e| e.to_string())?;
            let word_count = body.split_whitespace().count() as i64;

            let created_at = parse_frontmatter_field(&content, "created_at")
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
            let title = parse_frontmatter_field(&content, "title");
            let mood = parse_frontmatter_field(&content, "mood");

            conn.execute(
                "INSERT INTO entries (id, created_at, updated_at, title, markdown, mood, word_count)
                 VALUES (?1, ?2, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![eid, created_at, title, encrypted, mood, word_count],
            )
            .map_err(|e| e.to_string())?;

            if let Some(tags_str) = parse_frontmatter_field(&content, "tags") {
                let tags: Vec<&str> = tags_str
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .collect();

                for tag_name in tags {
                    conn.execute(
                        "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
                        rusqlite::params![tag_name],
                    )
                    .ok();
                    let tag_id: Result<i64, _> = conn.query_row(
                        "SELECT id FROM tags WHERE name = ?1 COLLATE NOCASE",
                        rusqlite::params![tag_name],
                        |r| r.get(0),
                    );
                    if let Ok(tid) = tag_id {
                        conn.execute(
                            "INSERT OR IGNORE INTO entry_tags (entry_id, tag_id) VALUES (?1, ?2)",
                            rusqlite::params![eid, tid],
                        )
                        .ok();
                    }
                }
            }

            imported += 1;
        }
    }

    conn.execute(
        "INSERT INTO audit (event, details) VALUES ('import_completed', json_object('imported', ?1, 'skipped', ?2))",
        rusqlite::params![imported as i64, skipped as i64],
    )
    .ok();

    Ok(ImportResult { imported, skipped })
}

// ── Frontmatter helpers ───────────────────────────────────────────────────────

fn parse_frontmatter_id(content: &str) -> Option<i64> {
    parse_frontmatter_field(content, "id")?.parse().ok()
}

fn parse_frontmatter_field(content: &str, field: &str) -> Option<String> {
    let prefix = format!("{field}: ");
    for line in content.lines() {
        if line.starts_with(&prefix) {
            return Some(line[prefix.len()..].trim().to_string());
        }
    }
    None
}

fn extract_body(content: &str) -> String {
    let mut count = 0;
    let mut char_pos = 0;
    for line in content.lines() {
        char_pos += line.len() + 1;
        if line.trim() == "---" {
            count += 1;
            if count == 2 {
                return content[char_pos..].trim().to_string();
            }
        }
    }
    content.to_string()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        crypto::generate_master_key,
        db::{open_memory_db, run_migrations},
    };

    fn setup_db_with_entry() -> (rusqlite::Connection, [u8; 32]) {
        let conn = open_memory_db().expect("open db");
        run_migrations(&conn).expect("migrate");
        let key = generate_master_key();
        let markdown = "<p>Hello export world!</p>";
        let encrypted = encrypt_str(markdown, &key).expect("encrypt");
        conn.execute(
            "INSERT INTO entries (title, markdown, word_count) VALUES ('Test Entry', ?1, 3)",
            rusqlite::params![encrypted],
        )
        .unwrap();
        (conn, key)
    }

    #[test]
    fn test_export_import_roundtrip_plaintext() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.chrxp").to_string_lossy().to_string();

        let (conn, key) = setup_db_with_entry();
        let options = ExportOptions {
            include_entries: true,
            include_attachments: false,
            include_settings: false,
            password: None,
        };

        let (zip_bytes, entry_count) = build_zip(&conn, &key, &options).unwrap();
        assert_eq!(entry_count, 1);

        write_plain_archive(&zip_bytes, &path).unwrap();
        let recovered = read_archive(&path, None).unwrap();
        assert_eq!(zip_bytes, recovered);
    }

    #[test]
    fn test_export_import_roundtrip_encrypted() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("enc.chrxp").to_string_lossy().to_string();

        let (conn, key) = setup_db_with_entry();
        let options = ExportOptions {
            include_entries: true,
            include_attachments: false,
            include_settings: false,
            password: Some("correct horse".into()),
        };

        let (zip_bytes, _) = build_zip(&conn, &key, &options).unwrap();
        write_encrypted_archive(&zip_bytes, &path, "correct horse").unwrap();
        let recovered = read_archive(&path, Some("correct horse")).unwrap();
        assert_eq!(zip_bytes, recovered);
    }

    #[test]
    fn test_wrong_password_fails() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("wrong.chrxp").to_string_lossy().to_string();

        let (conn, key) = setup_db_with_entry();
        let options = ExportOptions {
            include_entries: true,
            include_attachments: false,
            include_settings: false,
            password: Some("correct".into()),
        };

        let (zip_bytes, _) = build_zip(&conn, &key, &options).unwrap();
        write_encrypted_archive(&zip_bytes, &path, "correct").unwrap();

        let result = read_archive(&path, Some("wrong"));
        assert!(result.is_err(), "Wrong password must fail");
    }

    #[test]
    fn test_no_password_on_encrypted_fails() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nopw.chrxp").to_string_lossy().to_string();

        let (conn, key) = setup_db_with_entry();
        let options = ExportOptions {
            include_entries: true,
            include_attachments: false,
            include_settings: false,
            password: Some("secret".into()),
        };

        let (zip_bytes, _) = build_zip(&conn, &key, &options).unwrap();
        write_encrypted_archive(&zip_bytes, &path, "secret").unwrap();

        let result = read_archive(&path, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_magic_fails() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.chrxp").to_string_lossy().to_string();
        std::fs::write(&path, b"BADM\x00hello").unwrap();
        assert!(read_archive(&path, None).is_err());
    }

    #[test]
    fn test_manifest_in_zip() {
        let (conn, key) = setup_db_with_entry();
        let options = ExportOptions {
            include_entries: true,
            include_attachments: false,
            include_settings: false,
            password: None,
        };
        let (zip_bytes, _) = build_zip(&conn, &key, &options).unwrap();
        let cursor = Cursor::new(zip_bytes);
        let mut zip = zip::ZipArchive::new(cursor).unwrap();
        let names: Vec<String> =
            (0..zip.len()).map(|i| zip.by_index(i).unwrap().name().to_string()).collect();
        assert!(names.contains(&"manifest.json".to_string()));
        assert!(names.iter().any(|n| n.starts_with("entries/")));
    }

    #[test]
    fn test_extract_body() {
        let content = "---\nid: 1\ntags: []\n---\n\n# Title\n\nContent here.";
        let body = extract_body(content);
        assert!(body.contains("Content here."), "body: {body:?}");
        assert!(!body.contains("id: 1"));
    }

    #[test]
    fn test_parse_frontmatter_field() {
        let content = "---\nid: 42\ncreated_at: 2026-01-01T00:00:00Z\n---\n\nContent";
        assert_eq!(parse_frontmatter_field(content, "id"), Some("42".into()));
        assert_eq!(
            parse_frontmatter_field(content, "created_at"),
            Some("2026-01-01T00:00:00Z".into())
        );
        assert_eq!(parse_frontmatter_field(content, "missing"), None);
    }
}
