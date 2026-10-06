//! Markdown-with-YAML-frontmatter importer.
//!
//! Handles exports from: Obsidian, Bear, Day One (markdown export),
//! Notion (markdown), and any app that produces `.md` files with YAML frontmatter.
//!
//! Expected file format:
//! ```markdown
//! ---
//! title: My Entry
//! date: 2025-10-22T12:00:00Z
//! tags: [work, personal]
//! mood: reflective
//! ---
//!
//! # My Entry
//!
//! Content here...
//! ```
//!
//! The importer accepts:
//! - A single `.md` file
//! - A directory of `.md` files
//! - A `.zip` containing `.md` files

use std::path::Path;
use uuid::Uuid;

use super::{ImportEntry, ImportError, Importer};

pub struct MarkdownImporter;

impl Importer for MarkdownImporter {
    fn name(&self) -> &'static str {
        "Markdown"
    }

    fn can_handle(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            return ext == "md" || ext == "markdown";
        }
        // Directory containing .md files
        if path.is_dir() {
            return true;
        }
        false
    }

    fn parse(&self, path: &Path) -> Result<Vec<ImportEntry>, ImportError> {
        if path.is_file() {
            let content = std::fs::read_to_string(path)?;
            let entry = parse_markdown_file(&content, path)?;
            Ok(vec![entry])
        } else if path.is_dir() {
            let mut entries = Vec::new();
            let read_dir = std::fs::read_dir(path)?;
            for item in read_dir {
                let item = item?;
                let item_path = item.path();
                if item_path.extension().map_or(false, |e| e == "md" || e == "markdown") {
                    let content = std::fs::read_to_string(&item_path)?;
                    match parse_markdown_file(&content, &item_path) {
                        Ok(entry) => entries.push(entry),
                        Err(e) => {
                            eprintln!("Skipping {}: {e}", item_path.display());
                        }
                    }
                }
            }
            Ok(entries)
        } else {
            Err(ImportError::NotFound(path.display().to_string()))
        }
    }
}

fn parse_markdown_file(content: &str, path: &Path) -> Result<ImportEntry, ImportError> {
    let (frontmatter, body) = split_frontmatter(content);

    let title = frontmatter
        .as_ref()
        .and_then(|fm| extract_field(fm, "title"));
    let created_at = frontmatter
        .as_ref()
        .and_then(|fm| extract_field(fm, "date").or_else(|| extract_field(fm, "created_at")))
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
    let updated_at = frontmatter
        .as_ref()
        .and_then(|fm| {
            extract_field(fm, "updated_at")
                .or_else(|| extract_field(fm, "modified"))
        })
        .unwrap_or_else(|| created_at.clone());
    let tags = frontmatter
        .as_ref()
        .and_then(|fm| extract_tags(fm))
        .unwrap_or_default();
    let mood = frontmatter
        .as_ref()
        .and_then(|fm| extract_field(fm, "mood"));

    // Use path stem as external_id if no id field
    let external_id = frontmatter
        .as_ref()
        .and_then(|fm| extract_field(fm, "id").or_else(|| extract_field(fm, "uuid")))
        .unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(String::from)
                .unwrap_or_else(|| Uuid::new_v4().to_string())
        });

    // Convert Markdown body to minimal HTML (preserve as-is for now;
    // Phase 4 will add a proper Markdown→HTML renderer).
    let markdown_html = body.trim().to_string();

    Ok(ImportEntry {
        external_id,
        created_at,
        updated_at,
        title,
        markdown: markdown_html,
        tags,
        mood,
        media_refs: vec![],
    })
}

/// Split content into (optional frontmatter, body).
fn split_frontmatter(content: &str) -> (Option<String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content.to_string());
    }

    let after_first = &trimmed[3..];
    if let Some(end_pos) = after_first.find("\n---") {
        let fm = after_first[..end_pos].to_string();
        let body_start = end_pos + 4; // skip past \n---
        let body = after_first[body_start..].trim_start().to_string();
        (Some(fm), body)
    } else {
        (None, content.to_string())
    }
}

fn extract_field(frontmatter: &str, field: &str) -> Option<String> {
    let prefix = format!("{field}:");
    frontmatter
        .lines()
        .find(|l| l.trim_start().starts_with(&prefix))
        .map(|l| {
            l.trim_start()
                .trim_start_matches(&prefix)
                .trim()
                .trim_matches('"')
                .to_string()
        })
}

fn extract_tags(frontmatter: &str) -> Option<Vec<String>> {
    // Handles: `tags: [work, personal]` or `tags: work, personal`
    let line = frontmatter
        .lines()
        .find(|l| l.trim_start().starts_with("tags:"))?;
    let value = line.trim_start().trim_start_matches("tags:").trim();
    let cleaned = value.trim_matches('[').trim_matches(']');
    let tags: Vec<String> = cleaned
        .split(',')
        .map(|t| t.trim().trim_matches('"').to_string())
        .filter(|t| !t.is_empty())
        .collect();
    Some(tags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn can_handle_md_extension() {
        let importer = MarkdownImporter;
        assert!(importer.can_handle(Path::new("entry.md")));
        assert!(importer.can_handle(Path::new("entry.markdown")));
        assert!(!importer.can_handle(Path::new("entry.json")));
        assert!(!importer.can_handle(Path::new("entry.txt")));
    }

    #[test]
    fn parse_simple_markdown_file() {
        let content = "---\ntitle: Test Entry\ndate: 2025-10-22T12:00:00Z\ntags: [work, life]\nmood: reflective\n---\n\n# Test Entry\n\nContent here.";
        let mut f = NamedTempFile::with_suffix(".md").unwrap();
        f.write_all(content.as_bytes()).unwrap();
        let path = f.path();

        let importer = MarkdownImporter;
        let entries = importer.parse(path).unwrap();
        assert_eq!(entries.len(), 1);

        let e = &entries[0];
        assert_eq!(e.title, Some("Test Entry".into()));
        assert_eq!(e.created_at, "2025-10-22T12:00:00Z");
        assert_eq!(e.tags, vec!["work", "life"]);
        assert_eq!(e.mood, Some("reflective".into()));
        assert!(e.markdown.contains("Content here."), "body: {:?}", e.markdown);
    }

    #[test]
    fn parse_no_frontmatter() {
        let content = "# Just a title\n\nSome content.";
        let mut f = NamedTempFile::with_suffix(".md").unwrap();
        f.write_all(content.as_bytes()).unwrap();

        let importer = MarkdownImporter;
        let entries = importer.parse(f.path()).unwrap();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].markdown.contains("Some content."));
        assert!(entries[0].title.is_none());
    }

    #[test]
    fn split_frontmatter_correctly() {
        let content = "---\nid: 1\ntags: [a]\n---\n\nBody";
        let (fm, body) = split_frontmatter(content);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("id: 1"));
        assert_eq!(body.trim(), "Body");
    }

    #[test]
    fn extract_tags_bracket_format() {
        let fm = "tags: [work, personal, health]";
        let tags = extract_tags(fm).unwrap();
        assert_eq!(tags, vec!["work", "personal", "health"]);
    }

    #[test]
    fn parse_directory_of_md_files() {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..3 {
            let path = dir.path().join(format!("entry_{i}.md"));
            std::fs::write(&path, format!("---\ntitle: Entry {i}\ndate: 2025-01-0{i}T00:00:00Z\n---\n\nContent {i}")).unwrap();
        }

        let importer = MarkdownImporter;
        let entries = importer.parse(dir.path()).unwrap();
        assert_eq!(entries.len(), 3);
    }
}
