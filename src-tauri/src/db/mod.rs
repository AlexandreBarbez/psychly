use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// Opens or creates the SQLite database at a path relative to the app root.
    /// Creates the data directory if it doesn't exist.
    pub fn open(app_root: &std::path::Path) -> Result<Self> {
        let data_dir = app_root.join("data");
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| rusqlite::Error::InvalidParameterName(format!("Failed to create data dir: {e}")))?;

        let db_path = data_dir.join("psychly.db");
        let conn = Connection::open(&db_path)?;

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        let db = Database {
            conn: Mutex::new(conn),
        };
        db.initialize_schema()?;
        Ok(db)
    }

    /// Opens an in-memory database for testing.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.initialize_schema()?;
        Ok(db)
    }

    /// Resolves the portable database path relative to the app root.
    pub fn resolve_path(app_root: &std::path::Path) -> PathBuf {
        app_root.join("data").join("psychly.db")
    }

    /// Creates all tables if they don't exist (idempotent).
    fn initialize_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS journal_entries (
                id TEXT PRIMARY KEY NOT NULL,
                body TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS chat_sessions (
                id TEXT PRIMARY KEY NOT NULL,
                journal_entry_id TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (journal_entry_id) REFERENCES journal_entries(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS chat_messages (
                id TEXT PRIMARY KEY NOT NULL,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (session_id) REFERENCES chat_sessions(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS entry_analyses (
                id TEXT PRIMARY KEY NOT NULL,
                entry_id TEXT NOT NULL UNIQUE,
                emotional_tone TEXT NOT NULL,
                themes TEXT NOT NULL,
                patterns TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (entry_id) REFERENCES journal_entries(id) ON DELETE CASCADE
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS journal_entries_fts USING fts5(
                body,
                content='journal_entries',
                content_rowid='rowid'
            );

            CREATE TRIGGER IF NOT EXISTS journal_entries_ai AFTER INSERT ON journal_entries BEGIN
                INSERT INTO journal_entries_fts(rowid, body) VALUES (new.rowid, new.body);
            END;

            CREATE TRIGGER IF NOT EXISTS journal_entries_ad AFTER DELETE ON journal_entries BEGIN
                INSERT INTO journal_entries_fts(journal_entries_fts, rowid, body) VALUES('delete', old.rowid, old.body);
            END;

            CREATE TRIGGER IF NOT EXISTS journal_entries_au AFTER UPDATE ON journal_entries BEGIN
                INSERT INTO journal_entries_fts(journal_entries_fts, rowid, body) VALUES('delete', old.rowid, old.body);
                INSERT INTO journal_entries_fts(rowid, body) VALUES (new.rowid, new.body);
            END;
            ",
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_schema_creation_in_memory() {
        let db = Database::open_in_memory().expect("Failed to open in-memory db");
        let conn = db.conn.lock().unwrap();

        // Verify all tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();

        assert!(tables.contains(&"journal_entries".to_string()));
        assert!(tables.contains(&"chat_sessions".to_string()));
        assert!(tables.contains(&"chat_messages".to_string()));
        assert!(tables.contains(&"entry_analyses".to_string()));
    }

    #[test]
    fn test_schema_creation_idempotent() {
        let db = Database::open_in_memory().expect("Failed to open in-memory db");
        // Re-initialize should not fail
        db.initialize_schema().expect("Second init should succeed");
        db.initialize_schema().expect("Third init should succeed");
    }

    #[test]
    fn test_fts5_index() {
        let db = Database::open_in_memory().expect("Failed to open in-memory db");
        let conn = db.conn.lock().unwrap();

        // Insert a journal entry
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["test-1", "Aujourd'hui j'ai ressenti de la joie", "2026-04-05T10:00:00", "2026-04-05T10:00:00"],
        ).unwrap();

        // Search via FTS5
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM journal_entries_fts WHERE journal_entries_fts MATCH ?1",
                ["joie"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Search for non-matching term
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM journal_entries_fts WHERE journal_entries_fts MATCH ?1",
                ["tristesse"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_fts5_update_reindex() {
        let db = Database::open_in_memory().expect("Failed to open in-memory db");
        let conn = db.conn.lock().unwrap();

        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["test-1", "joie", "2026-04-05T10:00:00", "2026-04-05T10:00:00"],
        ).unwrap();

        // Update the entry
        conn.execute(
            "UPDATE journal_entries SET body = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params!["tristesse", "2026-04-05T11:00:00", "test-1"],
        ).unwrap();

        // Old term should not match
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM journal_entries_fts WHERE journal_entries_fts MATCH ?1",
                ["joie"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);

        // New term should match
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM journal_entries_fts WHERE journal_entries_fts MATCH ?1",
                ["tristesse"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_portable_path_resolution() {
        let root = Path::new("/some/portable/root");
        let db_path = Database::resolve_path(root);
        assert_eq!(db_path, root.join("data").join("psychly.db"));
        // No absolute path components outside root
        assert!(db_path.starts_with(root));
    }

    #[test]
    fn test_open_with_real_file() {
        let tmp_dir = std::env::temp_dir().join("psychly_test_db");
        let _ = std::fs::remove_dir_all(&tmp_dir);
        std::fs::create_dir_all(&tmp_dir).unwrap();

        let db = Database::open(&tmp_dir).expect("Failed to open file-backed db");
        let expected_path = tmp_dir.join("data").join("psychly.db");
        assert!(expected_path.exists());

        // Verify it's usable
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["t1", "test", "2026-04-05T10:00:00", "2026-04-05T10:00:00"],
        ).unwrap();

        drop(conn);
        drop(db);

        // Reopen — data should persist
        let db2 = Database::open(&tmp_dir).expect("Failed to reopen db");
        let conn2 = db2.conn.lock().unwrap();
        let body: String = conn2
            .query_row("SELECT body FROM journal_entries WHERE id = ?1", ["t1"], |row| row.get(0))
            .unwrap();
        assert_eq!(body, "test");

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
}
