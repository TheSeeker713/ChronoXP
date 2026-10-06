//! SQL schema for ChronoXP — matches the data model in docs/05-architecture/data-model.md.
//!
//! The `markdown` column in `entries` stores AES-256-GCM ciphertext (base64).
//! All other columns are plaintext for indexing and search.

pub const CREATE_ENTRIES: &str = "
CREATE TABLE IF NOT EXISTS entries (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    title       TEXT,
    markdown    TEXT NOT NULL,
    mood        TEXT,
    word_count  INTEGER NOT NULL DEFAULT 0,
    pinned      INTEGER NOT NULL DEFAULT 0,
    deleted_at  TEXT
);

CREATE INDEX IF NOT EXISTS idx_entries_created_at ON entries(created_at);
CREATE INDEX IF NOT EXISTS idx_entries_pinned ON entries(pinned) WHERE deleted_at IS NULL;
";

pub const CREATE_TAGS: &str = "
CREATE TABLE IF NOT EXISTS tags (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL UNIQUE COLLATE NOCASE,
    color       TEXT NOT NULL DEFAULT '#6B7280',
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
";

pub const CREATE_ENTRY_TAGS: &str = "
CREATE TABLE IF NOT EXISTS entry_tags (
    entry_id    INTEGER NOT NULL,
    tag_id      INTEGER NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    PRIMARY KEY (entry_id, tag_id),
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id)   REFERENCES tags(id)    ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_entry_tags_tag_id ON entry_tags(tag_id);
";

pub const CREATE_ATTACHMENTS: &str = "
CREATE TABLE IF NOT EXISTS attachments (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id    INTEGER NOT NULL,
    filename    TEXT NOT NULL,
    filepath    TEXT NOT NULL UNIQUE,
    mime_type   TEXT NOT NULL,
    size_bytes  INTEGER NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_attachments_entry_id ON attachments(entry_id);
";

pub const CREATE_SETTINGS: &str = "
CREATE TABLE IF NOT EXISTS settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
";

pub const CREATE_SYNC_LOG: &str = "
-- Stub table for future LAN/cloud sync. Not used in MVP.
CREATE TABLE IF NOT EXISTS sync_log (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id    INTEGER NOT NULL,
    sync_hash   TEXT NOT NULL,
    last_sync   TEXT NOT NULL,
    device_id   TEXT NOT NULL,
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE
);
";

pub const CREATE_AUDIT: &str = "
CREATE TABLE IF NOT EXISTS audit (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    event       TEXT NOT NULL,
    details     TEXT,
    timestamp   TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit(timestamp);
";

pub const CREATE_FTS: &str = "
-- FTS5 virtual table over plaintext title for keyword search.
-- Markdown content is searched in memory (decrypted) via the search command.
CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts USING fts5(
    title,
    content='entries',
    content_rowid='id'
);

CREATE TRIGGER IF NOT EXISTS entries_ai AFTER INSERT ON entries BEGIN
    INSERT INTO entries_fts(rowid, title) VALUES (new.id, new.title);
END;

CREATE TRIGGER IF NOT EXISTS entries_au AFTER UPDATE ON entries BEGIN
    INSERT INTO entries_fts(entries_fts, rowid, title) VALUES ('delete', old.id, old.title);
    INSERT INTO entries_fts(rowid, title) VALUES (new.id, new.title);
END;

CREATE TRIGGER IF NOT EXISTS entries_ad AFTER DELETE ON entries BEGIN
    INSERT INTO entries_fts(entries_fts, rowid, title) VALUES ('delete', old.id, old.title);
END;
";
