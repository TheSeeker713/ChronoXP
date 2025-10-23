# Data Model

**Project:** ChronoXP™ Journal  
**Date:** October 22, 2025

---

## Database: SQLCipher (Encrypted SQLite)

**Encryption:** AES-256  
**Location:** `%APPDATA%\ChronoXP\data.db` (Windows)  
**Collation:** UTF-8 (supports international characters)

---

## Schema

### Table: `entries`
**Purpose:** Store journal entries (text + metadata)

```sql
CREATE TABLE entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    title TEXT,                           -- Optional (first line used if NULL)
    markdown TEXT NOT NULL,               -- Entry content (Markdown format)
    mood TEXT,                            -- Auto-detected or user-set (e.g., "reflective", "anxious")
    word_count INTEGER DEFAULT 0,         -- Calculated on save
    pinned BOOLEAN DEFAULT 0,             -- User can pin important entries
    deleted_at TEXT                       -- Soft delete (NULL = active)
);

CREATE INDEX idx_entries_created_at ON entries(created_at);
CREATE INDEX idx_entries_pinned ON entries(pinned) WHERE deleted_at IS NULL;
```

**Notes:**
- `title`: If NULL, UI displays first line (up to 50 chars) as title
- `markdown`: Full text (headings, lists, bold, etc.)
- `deleted_at`: Soft delete (enables "Trash" feature in future)

---

### Table: `tags`
**Purpose:** Store unique tags

```sql
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,  -- Case-insensitive
    color TEXT DEFAULT '#6B7280',               -- Hex color for UI (optional)
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_tags_name ON tags(name);
```

**Notes:**
- `COLLATE NOCASE`: "Work" and "work" treated as same tag
- `color`: Optional visual customization (future feature)

---

### Table: `entry_tags`
**Purpose:** Many-to-many relationship between entries and tags

```sql
CREATE TABLE entry_tags (
    entry_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (entry_id, tag_id),
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_entry_tags_tag_id ON entry_tags(tag_id);
```

**Notes:**
- Composite primary key prevents duplicate tags on same entry
- Cascade delete: If entry deleted, tags are unlinked

---

### Table: `embeddings`
**Purpose:** Store vector embeddings for semantic search

```sql
CREATE TABLE embeddings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL UNIQUE,
    vector BLOB NOT NULL,                   -- Serialized float array (e.g., 384 or 768 dims)
    model_id TEXT NOT NULL,                 -- Model used (e.g., "nomic-embed-v1.5")
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE
);

CREATE INDEX idx_embeddings_entry_id ON embeddings(entry_id);
```

**Notes:**
- `vector`: BLOB stores float32 array (e.g., 384 floats × 4 bytes = 1.5 KB)
- `model_id`: Tracks which model generated embedding (for compatibility)
- If model changes, embeddings can be regenerated in background

**Vector search:**
- Option A (MVP): `sqlite-vec` extension for cosine similarity
- Option B (future): Export to FAISS index for faster search

---

### Table: `attachments`
**Purpose:** Track files attached to entries (images, PDFs, etc.)

```sql
CREATE TABLE attachments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL,
    filename TEXT NOT NULL,                 -- Original filename (e.g., "photo.jpg")
    filepath TEXT NOT NULL UNIQUE,          -- UUID-based path (e.g., "a3f2b9c4-5d6e.jpg")
    mime_type TEXT NOT NULL,                -- e.g., "image/jpeg", "application/pdf"
    size_bytes INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE
);

CREATE INDEX idx_attachments_entry_id ON attachments(entry_id);
```

**Notes:**
- Files stored in `%APPDATA%\ChronoXP\attachments\`
- UUID filenames prevent collisions
- Cascade delete: If entry deleted, attachments also deleted (from DB and disk)

---

### Table: `artifacts`
**Purpose:** Track unlocked gamification artifacts

```sql
CREATE TABLE artifacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,               -- e.g., "memory_jar", "ancient_scroll"
    unlocked_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

**Notes:**
- `key`: Matches artifact definitions in code (not human-readable labels)
- Artifact metadata (name, flavor text, icon) stored in JSON config file

---

### Table: `settings`
**Purpose:** Store user preferences and app state

```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,                    -- JSON blob for complex values
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

**Example rows:**
```sql
INSERT INTO settings (key, value) VALUES
('theme', '"dark"'),
('font_family', '"OpenDyslexic"'),
('user_xp', '250'),
('simple_mode', 'false'),
('ai_muse_enabled', 'true'),
('ai_archivist_enabled', 'true');
```

**Notes:**
- `value`: Stored as JSON string (allows nested objects)
- For simple values (numbers, booleans), still JSON-encoded for consistency

---

### Table: `audit`
**Purpose:** Log local-only events (for debugging, not telemetry)

```sql
CREATE TABLE audit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event TEXT NOT NULL,                    -- e.g., "entry_created", "export_completed"
    details TEXT,                           -- JSON blob with event-specific data
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_audit_timestamp ON audit(timestamp);
```

**Notes:**
- **Local only:** Never sent to external servers
- Examples:
  - `{"event": "entry_created", "details": {"word_count": 156}}`
  - `{"event": "export_completed", "details": {"file_size": 2048576}}`
- Used for:
  - Debugging (e.g., "When did autosave last trigger?")
  - Local analytics (if user opts in to anonymous stats)
- Retention: Auto-purge entries older than 90 days (configurable)

---

## Queries (Common Use Cases)

### 1. Get Recent Entries
```sql
SELECT id, created_at, title, markdown, mood
FROM entries
WHERE deleted_at IS NULL
ORDER BY created_at DESC
LIMIT 20;
```

---

### 2. Get Entry with Tags
```sql
SELECT 
    e.id,
    e.created_at,
    e.title,
    e.markdown,
    GROUP_CONCAT(t.name, ', ') AS tags
FROM entries e
LEFT JOIN entry_tags et ON e.id = et.entry_id
LEFT JOIN tags t ON et.tag_id = t.id
WHERE e.id = ?
GROUP BY e.id;
```

---

### 3. Search by Tag
```sql
SELECT DISTINCT e.*
FROM entries e
JOIN entry_tags et ON e.id = et.entry_id
JOIN tags t ON et.tag_id = t.id
WHERE t.name = ? AND e.deleted_at IS NULL
ORDER BY e.created_at DESC;
```

---

### 4. Semantic Search (Simplified)
```sql
-- Step 1: Generate query embedding (via llama.cpp)
-- Step 2: Calculate cosine similarity with stored embeddings
SELECT 
    e.id,
    e.created_at,
    e.title,
    e.markdown,
    -- Cosine similarity calculation (pseudo-code, actual impl in Rust)
    cosine_similarity(query_vector, emb.vector) AS score
FROM entries e
JOIN embeddings emb ON e.id = emb.entry_id
WHERE e.deleted_at IS NULL
ORDER BY score DESC
LIMIT 10;
```

**Actual implementation:**
- Rust code iterates over embeddings, calculates similarity
- Could use `sqlite-vec` extension for native vector search

---

### 5. Get XP Progress
```sql
SELECT value AS xp FROM settings WHERE key = 'user_xp';
```

---

### 6. Count Entries by Month
```sql
SELECT 
    strftime('%Y-%m', created_at) AS month,
    COUNT(*) AS entry_count
FROM entries
WHERE deleted_at IS NULL
GROUP BY month
ORDER BY month DESC;
```

---

## Data Migrations

### Migration Strategy
- **Tool:** Rust crate `rusqlite` with manual migration scripts
- **Version tracking:** `schema_version` in `settings` table

**Example migration (v1 → v2):**
```sql
-- Add `pinned` column to entries
ALTER TABLE entries ADD COLUMN pinned BOOLEAN DEFAULT 0;
UPDATE settings SET value = '2' WHERE key = 'schema_version';
```

**Best practices:**
1. Always backward-compatible (e.g., new columns with defaults)
2. Never drop columns (soft deprecate instead)
3. Test migration on copy of production DB

---

## Backup & Restore

### Backup (Manual or Scheduled)
```sql
-- SQLite built-in backup API
VACUUM INTO '/path/to/backup.db';
```

**Scheduled backups:**
- Optional setting: "Auto-backup every 7 days"
- Stored in `%APPDATA%\ChronoXP\backups\`
- Keep last 3 backups, delete older

---

### Restore
1. User selects backup file
2. App validates schema version
3. If compatible: Replace `data.db` with backup
4. If migration needed: Run migration scripts first

---

## Data Portability (.chrxp Export Format)

### Exported Structure
```
chronoxp_journal_2025-10-22.chrxp (ZIP)
├── manifest.json
├── entries/
│   ├── 2025-10-22_1542.md
│   ├── 2025-10-21_0830.md
│   └── ...
├── attachments/
│   ├── img_001.jpg
│   └── ...
├── meta/
│   ├── tags.json
│   ├── artifacts.json
│   └── settings.json
├── database/
│   └── chronoxp.db           (full SQLite backup)
└── vectors/
    └── embeddings.bin        (optional)
```

### Markdown Entry Format
```markdown
---
id: 123
created_at: 2025-10-22T15:42:00Z
updated_at: 2025-10-22T15:45:00Z
tags: [work, stress, reflection]
mood: reflective
word_count: 156
xp_awarded: 10
---

# Work-life balance thoughts

Today I talked to Mom about family expectations and how work is taking over my life...
```

### `tags.json`
```json
[
  {"id": 1, "name": "work", "color": "#3B82F6"},
  {"id": 2, "name": "family", "color": "#10B981"},
  {"id": 3, "name": "stress", "color": "#EF4444"}
]
```

### `artifacts.json`
```json
[
  {"key": "first_journal", "unlocked_at": "2025-10-01T12:00:00Z"},
  {"key": "memory_jar", "unlocked_at": "2025-10-15T18:30:00Z"}
]
```

---

## Performance Optimizations

### Indexing
- Primary keys (automatic): Fast lookups by ID
- Foreign keys: Enable cascade deletes
- Custom indexes: `created_at`, `tag_id`, etc. (see schema)

### Batch Operations
- **Bulk insert tags:**
  ```sql
  INSERT OR IGNORE INTO tags (name) VALUES ('work'), ('family'), ('health');
  ```
- **Bulk update XP:**
  ```sql
  UPDATE settings SET value = value + 10 WHERE key = 'user_xp';
  ```

### Vacuuming (Reclaim Space)
```sql
PRAGMA auto_vacuum = INCREMENTAL;
PRAGMA incremental_vacuum;
```

---

## Data Retention & Privacy

### What We Keep
- All entries (until user explicitly deletes)
- All tags, attachments, embeddings (linked to entries)
- XP and artifacts (unless "Simple Mode" + user deletes)
- Settings preferences

### What We Don't Keep
- **No telemetry:** Event logs are local-only
- **No cloud sync:** Data never leaves device
- **No search history:** Queries not logged (unless user opts in to local analytics)

### Deletion
- **Soft delete:** `deleted_at` column (enables "Trash" feature)
- **Hard delete:** After 30 days in Trash, permanent delete (`DELETE FROM entries WHERE deleted_at < date('now', '-30 days')`)
- **Attachments:** Deleted from disk when entry is hard-deleted

---

## Security

### Encryption
- **SQLCipher:** Database encrypted at rest (AES-256)
- **Key management:** OS keychain (Windows Credential Manager, macOS Keychain)
- **Optional vault password:** Second encryption layer (PBKDF2 key derivation)

### SQL Injection Prevention
- **Prepared statements:** All queries use parameterized SQL (via `rusqlite`)
- **No raw string concatenation**

---

## Testing Data

### Seed Data (For Development)
```sql
INSERT INTO entries (markdown, mood, word_count) VALUES
('First journal entry. Feeling optimistic about this new tool!', 'hopeful', 10),
('Long day at work. Need to find better balance.', 'tired', 9),
('Spent time with family today. Reminded me what matters.', 'grateful', 10);

INSERT INTO tags (name) VALUES ('work'), ('family'), ('health');

INSERT INTO entry_tags (entry_id, tag_id) VALUES
(2, 1), (3, 2);
```

---

## Future Enhancements

### 1. Full-Text Search (FTS5)
```sql
CREATE VIRTUAL TABLE entries_fts USING fts5(markdown, content=entries, content_rowid=id);
-- Enables fast keyword search with ranking
```

### 2. Mood Trends
```sql
SELECT mood, COUNT(*) AS count
FROM entries
WHERE created_at > date('now', '-30 days')
GROUP BY mood;
```

### 3. Sync Metadata (Future: LAN Sync)
```sql
CREATE TABLE sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL,
    sync_hash TEXT NOT NULL,     -- SHA-256 of entry content
    last_sync TEXT NOT NULL,
    device_id TEXT NOT NULL
);
```

---

## Schema Diagram (Simplified)

```
┌─────────────┐       ┌─────────────┐       ┌─────────────┐
│   entries   │───┬───│ entry_tags  │───────│    tags     │
│             │   │   │             │       │             │
│ • id        │   │   │ • entry_id  │       │ • id        │
│ • markdown  │   │   │ • tag_id    │       │ • name      │
│ • mood      │   │   └─────────────┘       │ • color     │
│ • word_count│   │                         └─────────────┘
└─────────────┘   │
       │          │   ┌─────────────┐
       ├──────────┴───│ embeddings  │
       │              │             │
       │              │ • entry_id  │
       │              │ • vector    │
       │              └─────────────┘
       │
       │          ┌─────────────┐
       └──────────│ attachments │
                  │             │
                  │ • entry_id  │
                  │ • filepath  │
                  └─────────────┘
```

---

## References
- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [SQLCipher](https://www.zetetic.net/sqlcipher/)
- [Rusqlite (Rust SQLite bindings)](https://docs.rs/rusqlite/)
- [sqlite-vec (Vector Extension)](https://github.com/asg017/sqlite-vec)
