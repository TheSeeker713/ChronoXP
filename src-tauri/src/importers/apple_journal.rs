//! Apple Journal export importer (stub — Phase 4).
//!
//! Apple Journal (iOS 17+) can export entries as a `.journal` file, which is
//! a zip archive containing JSON and media. The format is undocumented by Apple;
//! this stub will be reverse-engineered from public export samples in Phase 4.
//!
//! Known structure (from community research):
//! ```
//! export.journal  (ZIP)
//! ├── entries/
//! │   ├── <uuid>.json   (entry metadata + text)
//! │   └── ...
//! └── media/
//!     └── <filename>
//! ```
//!
//! Entry JSON (approximate):
//! ```json
//! {
//!   "identifier": "UUID",
//!   "creationDate": "2025-01-01T12:00:00.000Z",
//!   "lastModifiedDate": "2025-01-01T12:05:00.000Z",
//!   "text": "Journal entry content",
//!   "media": [{ "filename": "photo.jpg", "type": "image" }]
//! }
//! ```

use std::path::Path;

use super::{ImportEntry, ImportError, Importer};

pub struct AppleJournalImporter;

impl Importer for AppleJournalImporter {
    fn name(&self) -> &'static str {
        "Apple Journal"
    }

    fn can_handle(&self, path: &Path) -> bool {
        path.extension().map_or(false, |e| e == "journal")
    }

    fn parse(&self, _path: &Path) -> Result<Vec<ImportEntry>, ImportError> {
        // TODO: Phase 4 implementation
        // Steps:
        // 1. Open the .journal (ZIP) archive
        // 2. Enumerate entries/*.json files
        // 3. Parse each entry JSON
        // 4. Map fields to ImportEntry
        // 5. Collect media refs
        Err(ImportError::InvalidFormat(
            "Apple Journal importer not yet implemented (Phase 4)".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn can_handle_journal_extension() {
        let importer = AppleJournalImporter;
        assert!(importer.can_handle(Path::new("export.journal")));
        assert!(!importer.can_handle(Path::new("export.zip")));
        assert!(!importer.can_handle(Path::new("export.json")));
    }

    #[test]
    fn parse_returns_not_implemented() {
        let importer = AppleJournalImporter;
        let result = importer.parse(Path::new("nonexistent.journal"));
        assert!(result.is_err());
        match result.unwrap_err() {
            ImportError::InvalidFormat(msg) => assert!(msg.contains("Phase 4")),
            e => panic!("unexpected: {e}"),
        }
    }
}
