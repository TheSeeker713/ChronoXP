pub mod migrations;
pub mod schema;

use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;

pub use migrations::run_migrations;

/// Open (or create) the ChronoXP SQLite database at `path`.
pub fn open_db(path: &Path) -> Result<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    // Performance & safety settings
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA foreign_keys = ON;
         PRAGMA synchronous = NORMAL;
         PRAGMA temp_store = MEMORY;",
    )?;
    Ok(conn)
}

/// Open an in-memory database (for tests).
pub fn open_memory_db() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA foreign_keys = ON;",
    )?;
    Ok(conn)
}
