use crate::db::Database;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

use super::ImportResult;

pub fn do_backup(src_path: &Path, dest_path: &Path) -> Result<(), String> {
    if !src_path.exists() {
        return Err(format!("Source DB not found: {}", src_path.display()));
    }
    std::fs::copy(src_path, dest_path)
        .map_err(|e| format!("Backup failed: {e}"))?;
    Ok(())
}

pub fn do_restore(db: &Arc<Database>, src_path: &Path) -> Result<ImportResult, String> {
    if !src_path.exists() {
        return Err(format!("Backup file not found: {}", src_path.display()));
    }

    let src_conn = rusqlite::Connection::open_with_flags(
        src_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .map_err(|e| format!("Cannot open backup: {e}"))?;

    let mut result = ImportResult { inserted: 0, skipped: 0, errors: vec![] };

    // 1. journal_entries (no FK dependencies)
    let journal_rows: Vec<(String, String, String, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, body, created_at, updated_at FROM journal_entries")
            .map_err(|e| format!("Cannot read journal_entries: {e}"))?;
        let rows: Vec<_> = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
            .map_err(|e| format!("journal_entries query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect();
        rows
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, body, created_at, updated_at) in journal_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM journal_entries WHERE id = ?1", [&id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, body, created_at, updated_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("journal_entries/{id}: {e}")),
            }
        }
    }

    // 2. chat_sessions (FK → journal_entries)
    let session_rows: Vec<(String, Option<String>, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, journal_entry_id, created_at FROM chat_sessions")
            .map_err(|e| format!("Cannot read chat_sessions: {e}"))?;
        let rows: Vec<_> = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .map_err(|e| format!("chat_sessions query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect();
        rows
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, journal_entry_id, created_at) in session_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM chat_sessions WHERE id = ?1", [&id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO chat_sessions (id, journal_entry_id, created_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![id, journal_entry_id, created_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("chat_sessions/{id}: {e}")),
            }
        }
    }

    // 3. chat_messages (FK → chat_sessions)
    let message_rows: Vec<(String, String, String, String, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, session_id, role, content, created_at FROM chat_messages")
            .map_err(|e| format!("Cannot read chat_messages: {e}"))?;
        let rows: Vec<_> = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)))
            .map_err(|e| format!("chat_messages query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect();
        rows
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, session_id, role, content, created_at) in message_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM chat_messages WHERE id = ?1", [&id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO chat_messages (id, session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, session_id, role, content, created_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("chat_messages/{id}: {e}")),
            }
        }
    }

    // 4. entry_analyses (FK → journal_entries)
    let analysis_rows: Vec<(String, String, String, String, String, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, entry_id, emotional_tone, themes, patterns, created_at FROM entry_analyses")
            .map_err(|e| format!("Cannot read entry_analyses: {e}"))?;
        let rows: Vec<_> = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)))
            .map_err(|e| format!("entry_analyses query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect();
        rows
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, entry_id, emotional_tone, themes, patterns, created_at) in analysis_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM entry_analyses WHERE entry_id = ?1", [&entry_id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO entry_analyses (id, entry_id, emotional_tone, themes, patterns, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![id, entry_id, emotional_tone, themes, patterns, created_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("entry_analyses/{id}: {e}")),
            }
        }
    }

    Ok(result)
}

#[tauri::command]
pub fn backup_db(app: AppHandle, dest_path: String) -> Result<(), String> {
    let app_root = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Cannot resolve resource dir: {e}"))?;
    let db_path = crate::db::Database::resolve_path(&app_root);
    do_backup(&db_path, Path::new(&dest_path))
}

#[tauri::command]
pub fn restore_db(
    db: State<'_, Arc<Database>>,
    src_path: String,
) -> Result<ImportResult, String> {
    do_restore(db.inner(), Path::new(&src_path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn make_src_db(tmp_dir: &Path) -> std::path::PathBuf {
        let db = Database::open(tmp_dir).unwrap();
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["entry-1", "Hello backup", "2026-04-30T10:00:00", "2026-04-30T10:00:00"],
        ).unwrap();
        drop(conn);
        drop(db);
        tmp_dir.join("data").join("psychly.db")
    }

    #[test]
    fn test_backup_creates_file() {
        let tmp = std::env::temp_dir().join("psychly_backup_test_1");
        std::fs::create_dir_all(&tmp).unwrap();
        let src = make_src_db(&tmp);
        let dest = tmp.join("backup.db");

        let result = do_backup(&src, &dest);
        assert!(result.is_ok(), "backup failed: {:?}", result);
        assert!(dest.exists(), "backup file not created");

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_backup_dest_is_valid_sqlite() {
        let tmp = std::env::temp_dir().join("psychly_backup_test_2");
        std::fs::create_dir_all(&tmp).unwrap();
        let src = make_src_db(&tmp);
        let dest = tmp.join("backup.db");

        do_backup(&src, &dest).unwrap();

        let conn = rusqlite::Connection::open(&dest).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM journal_entries", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_backup_nonexistent_src_returns_error() {
        let dest = std::env::temp_dir().join("psychly_backup_test_3_dest.db");
        let result = do_backup(Path::new("/nonexistent/path/psychly.db"), &dest);
        assert!(result.is_err());
    }

    fn make_src_db_full(tmp_dir: &Path) -> std::path::PathBuf {
        let db = Database::open(tmp_dir).unwrap();
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["entry-restore-1", "Restore body", "2026-04-30T10:00:00", "2026-04-30T10:00:00"],
        ).unwrap();
        conn.execute(
            "INSERT INTO chat_sessions (id, journal_entry_id, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params!["session-restore-1", "entry-restore-1", "2026-04-30T10:01:00"],
        ).unwrap();
        conn.execute(
            "INSERT INTO chat_messages (id, session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params!["msg-restore-1", "session-restore-1", "user", "Hello", "2026-04-30T10:02:00"],
        ).unwrap();
        conn.execute(
            "INSERT INTO entry_analyses (id, entry_id, emotional_tone, themes, patterns, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["analysis-restore-1", "entry-restore-1", "positive", "joy", "growth", "2026-04-30T10:03:00"],
        ).unwrap();
        drop(conn);
        drop(db);
        tmp_dir.join("data").join("psychly.db")
    }

    #[test]
    fn test_restore_merges_all_tables() {
        let tmp_src = std::env::temp_dir().join("psychly_restore_src_1");
        std::fs::create_dir_all(&tmp_src).unwrap();
        let src_path = make_src_db_full(&tmp_src);

        let dst_db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_restore(&dst_db, &src_path).unwrap();

        assert_eq!(result.inserted, 4, "expected 4 rows inserted (entry+session+message+analysis)");
        assert_eq!(result.skipped, 0);
        assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);

        let conn = dst_db.conn.lock().unwrap();
        let body: String = conn
            .query_row("SELECT body FROM journal_entries WHERE id = ?1", ["entry-restore-1"], |r| r.get(0))
            .unwrap();
        assert_eq!(body, "Restore body");

        std::fs::remove_dir_all(&tmp_src).unwrap();
    }

    #[test]
    fn test_restore_skips_existing_journal_entry() {
        let tmp_src = std::env::temp_dir().join("psychly_restore_src_2");
        std::fs::create_dir_all(&tmp_src).unwrap();
        let src_path = make_src_db(&tmp_src);

        let dst_db = Arc::new(Database::open_in_memory().unwrap());
        // Pre-insert the same entry
        {
            let conn = dst_db.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params!["entry-1", "Pre-existing", "2026-04-30T10:00:00", "2026-04-30T10:00:00"],
            ).unwrap();
        }

        let result = do_restore(&dst_db, &src_path).unwrap();
        assert_eq!(result.inserted, 0);
        assert_eq!(result.skipped, 1);
        assert!(result.errors.is_empty());

        std::fs::remove_dir_all(&tmp_src).unwrap();
    }

    #[test]
    fn test_restore_nonexistent_src_returns_error() {
        let dst_db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_restore(&dst_db, Path::new("/nonexistent/backup.db"));
        assert!(result.is_err());
    }
}
