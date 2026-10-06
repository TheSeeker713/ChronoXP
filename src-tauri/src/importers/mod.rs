//! Importer plugin interface (groundwork — stubs provided, full implementations in Phase 4).
//!
//! Pipeline: parse → normalize → deduplicate → encrypt → insert
//!
//! Each importer receives a source file path (zip, directory, or single file),
//! parses the format, normalises entries into `ImportEntry`, then the caller
//! deduplicates against the existing DB by `external_id` and encrypts the
//! `markdown` field before writing.

pub mod apple_journal;
pub mod dayone;
pub mod markdown;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A normalised entry ready for de-duplication + encryption + DB insert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEntry {
    /// Original ID from the source (Day One UUID, Apple identifier, etc.).
    pub external_id: String,
    /// ISO-8601 creation time.
    pub created_at: String,
    /// ISO-8601 last-modified time.
    pub updated_at: String,
    /// Optional explicit title.
    pub title: Option<String>,
    /// Entry body. May be Markdown, HTML, or plain text depending on importer.
    /// The importer should normalise to HTML (TipTap-compatible) where possible.
    pub markdown: String,
    /// Tags/labels from source.
    pub tags: Vec<String>,
    /// Mood string from source (mapped to ChronoXP mood vocabulary).
    pub mood: Option<String>,
    /// Relative paths to media files inside the source archive.
    pub media_refs: Vec<String>,
}

/// Errors from import operations.
#[derive(Debug, Error)]
pub enum ImportError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Unsupported version: {0}")]
    UnsupportedVersion(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Statistics returned after a successful import.
#[derive(Debug, Default, Serialize)]
pub struct ImportStats {
    pub parsed: usize,
    pub deduplicated: usize,
    pub imported: usize,
    pub media_copied: usize,
    pub errors: Vec<String>,
}

/// Every importer implements this trait.
pub trait Importer {
    /// Short name for logging / UI.
    fn name(&self) -> &'static str;

    /// Return true if this importer can handle the file at `path`.
    ///
    /// Checked by extension, magic bytes, or presence of a well-known
    /// internal file (e.g., `journal.json` in a Day One zip).
    fn can_handle(&self, path: &std::path::Path) -> bool;

    /// Parse the source into a flat list of normalised entries.
    ///
    /// Does NOT write to DB. The caller is responsible for
    /// deduplication and encryption before inserting.
    fn parse(&self, path: &std::path::Path) -> Result<Vec<ImportEntry>, ImportError>;
}
