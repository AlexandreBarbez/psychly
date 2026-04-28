use std::path::Path;
use std::sync::Arc;
use serde::Serialize;
use tauri::State;
use chrono::NaiveDateTime;

use crate::db::Database;

const DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S%.f";

#[derive(Debug, Serialize, Clone)]
pub struct ImportResult {
    pub inserted: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

fn parse_markdown_entry(content: &str) -> Option<(String, String, String, String)> {
    let content = content.trim();
    let after_first = content.strip_prefix("---\n")?;
    let end_pos = after_first.find("\n---")?;
    let frontmatter = &after_first[..end_pos];
    let rest = &after_first[end_pos + 4..];
    let body = rest.trim_start_matches('\n').trim().to_string();

    let mut id = None;
    let mut created_at = None;
    let mut updated_at = None;

    for line in frontmatter.lines() {
        if let Some(v) = line.strip_prefix("id: ") {
            id = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("created_at: ") {
            created_at = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("updated_at: ") {
            updated_at = Some(v.trim().to_string());
        }
    }

    Some((id?, created_at?, updated_at.unwrap_or_default(), body))
}

pub fn do_export(db: &Arc<Database>, dest_dir: &Path) -> Result<usize, String> {
    if !dest_dir.is_dir() {
        return Err(format!("Not a directory: {}", dest_dir.display()));
    }

    let entries: Vec<(String, String, String, String)> = {
        let conn = db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, body, created_at, updated_at FROM journal_entries ORDER BY created_at ASC",
            )
            .map_err(|e| format!("DB prepare error: {e}"))?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| format!("DB query error: {e}"))?
        .filter_map(|r| r.ok())
        .collect();
        rows
    };

    let mut written = 0;
    for (id, body, created_at_str, updated_at_str) in &entries {
        let created_at = NaiveDateTime::parse_from_str(created_at_str, DATETIME_FMT)
            .map_err(|e| format!("Invalid date in entry {id}: {e}"))?;
        let updated_at = NaiveDateTime::parse_from_str(updated_at_str, DATETIME_FMT)
            .unwrap_or(created_at);

        let filename = format!(
            "{}_{}.md",
            created_at.format("%Y-%m-%d"),
            &id[..8.min(id.len())]
        );
        let content = format!(
            "---\nid: {id}\ncreated_at: {}\nupdated_at: {}\n---\n\n{}",
            created_at.format("%Y-%m-%dT%H:%M:%S"),
            updated_at.format("%Y-%m-%dT%H:%M:%S"),
            body
        );

        std::fs::write(dest_dir.join(&filename), &content)
            .map_err(|e| format!("Write error for {filename}: {e}"))?;
        written += 1;
    }

    Ok(written)
}

pub fn do_import(db: &Arc<Database>, src_dir: &Path) -> Result<ImportResult, String> {
    if !src_dir.is_dir() {
        return Err(format!("Not a directory: {}", src_dir.display()));
    }

    let mut result = ImportResult { inserted: 0, skipped: 0, errors: vec![] };

    let md_files: Vec<_> = std::fs::read_dir(src_dir)
        .map_err(|e| format!("Cannot read directory: {e}"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("md"))
        .collect();

    let conn = db.conn.lock().unwrap();

    for dir_entry in md_files {
        let path = dir_entry.path();
        let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                result.errors.push(format!("{filename}: {e}"));
                continue;
            }
        };

        let (id, created_at_str, updated_at_str, body) = match parse_markdown_entry(&content) {
            Some(v) => v,
            None => {
                result.errors.push(format!("{filename}: invalid frontmatter"));
                continue;
            }
        };

        let exists: bool = conn
            .query_row("SELECT 1 FROM journal_entries WHERE id = ?1", [&id], |_| Ok(true))
            .unwrap_or(false);

        if exists {
            result.skipped += 1;
            continue;
        }

        let updated_at = if updated_at_str.is_empty() { created_at_str.clone() } else { updated_at_str };

        match conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, body, created_at_str, updated_at],
        ) {
            Ok(_) => result.inserted += 1,
            Err(e) => result.errors.push(format!("{filename}: insert failed: {e}")),
        }
    }

    Ok(result)
}

#[tauri::command]
pub fn export_journal(db: State<'_, Arc<Database>>, dest_dir: String) -> Result<usize, String> {
    do_export(db.inner(), Path::new(&dest_dir))
}

#[tauri::command]
pub fn import_journal(db: State<'_, Arc<Database>>, src_dir: String) -> Result<ImportResult, String> {
    do_import(db.inner(), Path::new(&src_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db_with_entry() -> Arc<Database> {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
                "Hello world",
                "2026-04-28T10:00:00.0",
                "2026-04-28T10:00:00.0"
            ],
        ).unwrap();
        drop(conn);
        db
    }

    #[test]
    fn test_export_creates_markdown_file() {
        let db = db_with_entry();
        let tmp = std::env::temp_dir().join("psychly_export_test_1");
        std::fs::create_dir_all(&tmp).unwrap();

        let count = do_export(&db, &tmp).unwrap();
        assert_eq!(count, 1);

        let file = tmp.join("2026-04-28_aaaaaaaa.md");
        assert!(file.exists(), "Expected file {:?} to exist", file);
        let content = std::fs::read_to_string(&file).unwrap();
        assert!(content.starts_with("---\n"));
        assert!(content.contains("id: aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"));
        assert!(content.contains("Hello world"));

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_export_invalid_dir_returns_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_export(&db, Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_export_empty_db_returns_zero() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_export_test_empty");
        std::fs::create_dir_all(&tmp).unwrap();

        let count = do_export(&db, &tmp).unwrap();
        assert_eq!(count, 0);

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_inserts_entry() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_import_test_1");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(
            tmp.join("2026-04-28_aaaaaaaa.md"),
            "---\nid: aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee\ncreated_at: 2026-04-28T10:00:00\nupdated_at: 2026-04-28T10:00:00\n---\n\nHello world",
        ).unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 1);
        assert_eq!(result.skipped, 0);
        assert!(result.errors.is_empty());

        let conn = db.conn.lock().unwrap();
        let body: String = conn.query_row(
            "SELECT body FROM journal_entries WHERE id = ?1",
            ["aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(body, "Hello world");

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_skips_duplicate_id() {
        let db = db_with_entry();
        let tmp = std::env::temp_dir().join("psychly_import_test_2");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(
            tmp.join("2026-04-28_aaaaaaaa.md"),
            "---\nid: aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee\ncreated_at: 2026-04-28T10:00:00\nupdated_at: 2026-04-28T10:00:00\n---\n\nHello world",
        ).unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 0);
        assert_eq!(result.skipped, 1);
        assert!(result.errors.is_empty());

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_invalid_frontmatter_adds_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_import_test_3");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(tmp.join("bad.md"), "No frontmatter here at all").unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 0);
        assert!(!result.errors.is_empty());

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_missing_id_adds_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_import_test_4");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(
            tmp.join("no_id.md"),
            "---\ncreated_at: 2026-04-28T10:00:00\n---\n\nBody without id",
        ).unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 0);
        assert!(!result.errors.is_empty());

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_nonexistent_dir_returns_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_import(&db, Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_markdown_entry_valid() {
        let content = "---\nid: test-uuid\ncreated_at: 2026-04-28T10:00:00\nupdated_at: 2026-04-28T10:30:00\n---\n\nMon journal";
        let result = parse_markdown_entry(content);
        assert!(result.is_some());
        let (id, created_at, updated_at, body) = result.unwrap();
        assert_eq!(id, "test-uuid");
        assert_eq!(created_at, "2026-04-28T10:00:00");
        assert_eq!(updated_at, "2026-04-28T10:30:00");
        assert_eq!(body, "Mon journal");
    }

    #[test]
    fn test_parse_markdown_entry_no_frontmatter() {
        let result = parse_markdown_entry("No frontmatter");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_markdown_entry_missing_id() {
        let result = parse_markdown_entry("---\ncreated_at: 2026-04-28T10:00:00\n---\n\nBody");
        assert!(result.is_none());
    }
}
