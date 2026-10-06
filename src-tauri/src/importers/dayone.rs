//! Day One JSON zip importer (stub — Phase 4).
//!
//! Day One exports a `.zip` containing `Journal.json` (or multiple `.json` files).
//! Format documented at: https://dayoneapp.com/guides/tips-and-tutorials/exporting-entries/
//!
//! Each entry in `Journal.json` looks like:
//! ```json
//! {
//!   "uuid": "ABC123...",
//!   "creationDate": "2025-10-22T12:00:00Z",
//!   "modifiedDate": "2025-10-22T12:05:00Z",
//!   "text": "## Title\n\nContent...",
//!   "tags": ["work", "personal"],
//!   "starred": false,
//!   "userActivity": { "activityName": "" },
//!   "photos": [{ "identifier": "photo1", "type": "jpeg", ... }]
//! }
//! ```

use std::path::Path;

use super::{ImportEntry, ImportError, Importer};

pub struct DayOneImporter;

impl Importer for DayOneImporter {
    fn name(&self) -> &'static str {
        "Day One"
    }

    fn can_handle(&self, path: &Path) -> bool {
        path.extension().map_or(false, |e| e == "zip")
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |n| n.to_lowercase().contains("dayone") || n.to_lowercase().contains("day one"))
    }

    fn parse(&self, _path: &Path) -> Result<Vec<ImportEntry>, ImportError> {
        // TODO: Phase 4 implementation
        // Steps:
        // 1. Open the ZIP archive
        // 2. Find Journal.json (or *.json files in root)
        // 3. Parse the JSON array of entries
        // 4. For each entry:
        //    a. Map `uuid` → `external_id`
        //    b. Map `creationDate` → `created_at`
        //    c. Map `modifiedDate` → `updated_at`
        //    d. Convert `text` (Markdown) to HTML
        //    e. Map `tags` → `tags`
        //    f. Map `photos[].identifier` → `media_refs` (with ZIP path)
        //    g. Map `starred` → pinned (handled separately, not in ImportEntry)
        Err(ImportError::InvalidFormat(
            "Day One importer not yet implemented (Phase 4)".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn can_handle_dayone_zip() {
        let importer = DayOneImporter;
        assert!(importer.can_handle(Path::new("DayOne.zip")));
        assert!(importer.can_handle(Path::new("day one journal.zip")));
        assert!(!importer.can_handle(Path::new("journal.json")));
        assert!(!importer.can_handle(Path::new("other.zip")));
    }

    #[test]
    fn parse_returns_not_implemented() {
        let importer = DayOneImporter;
        let result = importer.parse(Path::new("nonexistent.zip"));
        assert!(result.is_err());
        match result.unwrap_err() {
            ImportError::InvalidFormat(msg) => assert!(msg.contains("Phase 4")),
            e => panic!("unexpected: {e}"),
        }
    }
}
