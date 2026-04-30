use crate::db::Database;
use std::path::Path;
use std::sync::Arc;

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
    todo!()
}

#[tauri::command]
pub fn backup_db(app: tauri::AppHandle, dest_path: String) -> Result<(), String> {
    todo!()
}

#[tauri::command]
pub fn restore_db(
    db: tauri::State<'_, Arc<Database>>,
    src_path: String,
) -> Result<ImportResult, String> {
    todo!()
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
}
