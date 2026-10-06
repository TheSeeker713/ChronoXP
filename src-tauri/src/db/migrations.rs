//! Schema migration system for ChronoXP.
//!
//! Migrations are append-only numbered steps. The current schema version is
//! stored in `settings` with key `schema_version`. Each migration is idempotent
//! (uses IF NOT EXISTS / IF NOT EXISTS guards) and never drops existing columns.

use anyhow::{anyhow, Result};
use rusqlite::Connection;

use super::schema::*;

const CURRENT_VERSION: u32 = 1;

/// Apply any pending migrations, creating the schema from scratch if needed.
pub fn run_migrations(conn: &Connection) -> Result<()> {
    let version = get_schema_version(conn)?;
    for v in (version + 1)..=CURRENT_VERSION {
        apply_migration(conn, v)?;
    }
    Ok(())
}

fn get_schema_version(conn: &Connection) -> Result<u32> {
    // Check if settings table exists yet
    let exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='settings'",
        [],
        |row| row.get::<_, i64>(0),
    )? > 0;

    if !exists {
        return Ok(0);
    }

    let ver: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .ok();

    match ver {
        Some(v) => v.trim_matches('"').parse::<u32>().map_err(|e| anyhow!(e)),
        None => Ok(0),
    }
}

fn apply_migration(conn: &Connection, version: u32) -> Result<()> {
    match version {
        1 => migration_v1(conn),
        v => Err(anyhow!("Unknown migration version: {v}")),
    }
}

/// V1: Initial schema — all base tables + FTS + sync_log stub.
fn migration_v1(conn: &Connection) -> Result<()> {
    conn.execute_batch(CREATE_SETTINGS)?;
    conn.execute_batch(CREATE_ENTRIES)?;
    conn.execute_batch(CREATE_TAGS)?;
    conn.execute_batch(CREATE_ENTRY_TAGS)?;
    conn.execute_batch(CREATE_ATTACHMENTS)?;
    conn.execute_batch(CREATE_SYNC_LOG)?;
    conn.execute_batch(CREATE_AUDIT)?;
    conn.execute_batch(CREATE_FTS)?;

    // Seed default settings
    conn.execute_batch(
        "INSERT OR IGNORE INTO settings (key, value) VALUES
            ('schema_version', '1'),
            ('theme', '\"dark\"'),
            ('font_family', '\"Inter\"'),
            ('auto_lock_minutes', '15'),
            ('vault_password_enabled', 'false'),
            ('simple_mode', 'false'),
            ('initialized', 'false');",
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::open_memory_db;

    #[test]
    fn test_migration_v1_creates_all_tables() {
        let conn = open_memory_db().expect("open in-memory db");
        run_migrations(&conn).expect("migrations should succeed");

        let tables: Vec<String> = {
            let mut stmt = conn
                .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
                .unwrap();
            stmt.query_map([], |row| row.get(0))
                .unwrap()
                .flatten()
                .collect()
        };

        for expected in &[
            "entries",
            "tags",
            "entry_tags",
            "attachments",
            "settings",
            "sync_log",
            "audit",
        ] {
            assert!(
                tables.contains(&expected.to_string()),
                "Missing table: {expected}"
            );
        }
    }

    #[test]
    fn test_migration_idempotent() {
        let conn = open_memory_db().expect("open in-memory db");
        run_migrations(&conn).expect("first migration");
        // Running again must not error
        run_migrations(&conn).expect("second migration must be idempotent");
    }

    #[test]
    fn test_schema_version_set_to_1() {
        let conn = open_memory_db().expect("open in-memory db");
        run_migrations(&conn).expect("migration");

        let ver: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .expect("schema_version must exist");
        assert_eq!(ver, "1");
    }

    #[test]
    fn test_fts_triggers_work() {
        let conn = open_memory_db().expect("open in-memory db");
        run_migrations(&conn).expect("migration");

        conn.execute(
            "INSERT INTO entries (title, markdown, word_count) VALUES ('Hello World', 'encrypted', 2)",
            [],
        )
        .expect("insert entry");

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM entries_fts WHERE title MATCH 'Hello'",
                [],
                |row| row.get(0),
            )
            .expect("FTS query");
        assert_eq!(count, 1, "FTS trigger should have indexed the entry");
    }

    #[test]
    fn test_default_settings_seeded() {
        let conn = open_memory_db().expect("open in-memory db");
        run_migrations(&conn).expect("migration");

        let theme: String = conn
            .query_row(
                "SELECT value FROM settings WHERE key = 'theme'",
                [],
                |row| row.get(0),
            )
            .expect("theme setting");
        assert_eq!(theme, "\"dark\"");
    }

    #[test]
    fn test_foreign_key_cascade() {
        let conn = open_memory_db().expect("open in-memory db");
        run_migrations(&conn).expect("migration");

        conn.execute(
            "INSERT INTO entries (title, markdown, word_count) VALUES ('Test', 'enc', 1)",
            [],
        )
        .unwrap();
        let entry_id: i64 = conn.last_insert_rowid();

        conn.execute("INSERT INTO tags (name) VALUES ('mytag')", []).unwrap();
        let tag_id: i64 = conn.last_insert_rowid();

        conn.execute(
            "INSERT INTO entry_tags (entry_id, tag_id) VALUES (?1, ?2)",
            [entry_id, tag_id],
        )
        .unwrap();

        // Deleting the entry should cascade to entry_tags
        conn.execute("DELETE FROM entries WHERE id = ?1", [entry_id]).unwrap();

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM entry_tags WHERE entry_id = ?1",
                [entry_id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0, "CASCADE delete should remove entry_tags rows");
    }
}
