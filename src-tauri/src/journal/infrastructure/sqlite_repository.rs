use std::sync::Arc;
use chrono::NaiveDateTime;

use crate::db::Database;
use crate::journal::domain::{JournalEntry, JournalRepository};

const DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S%.f";

pub struct SqliteJournalRepository {
    db: Arc<Database>,
}

impl SqliteJournalRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl JournalRepository for SqliteJournalRepository {
    fn create(&self, entry: &JournalEntry) -> Result<(), String> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                entry.id,
                entry.body,
                entry.created_at.format(DATETIME_FMT).to_string(),
                entry.updated_at.format(DATETIME_FMT).to_string(),
            ],
        )
        .map_err(|e| format!("Failed to create entry: {e}"))?;
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Option<JournalEntry>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, body, created_at, updated_at FROM journal_entries WHERE id = ?1")
            .map_err(|e| format!("Failed to prepare query: {e}"))?;

        let result = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok(JournalEntry {
                    id: row.get(0)?,
                    body: row.get(1)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(2)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                    updated_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(3)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .optional()
            .map_err(|e| format!("Failed to get entry: {e}"))?;

        Ok(result)
    }

    fn update(&self, entry: &JournalEntry) -> Result<(), String> {
        let conn = self.db.conn.lock().unwrap();
        let updated = conn
            .execute(
                "UPDATE journal_entries SET body = ?1, updated_at = ?2 WHERE id = ?3",
                rusqlite::params![
                    entry.body,
                    entry.updated_at.format(DATETIME_FMT).to_string(),
                    entry.id,
                ],
            )
            .map_err(|e| format!("Failed to update entry: {e}"))?;

        if updated == 0 {
            return Err(format!("Entry not found: {}", entry.id));
        }
        Ok(())
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let conn = self.db.conn.lock().unwrap();
        let deleted = conn
            .execute("DELETE FROM journal_entries WHERE id = ?1", rusqlite::params![id])
            .map_err(|e| format!("Failed to delete entry: {e}"))?;

        if deleted == 0 {
            return Err(format!("Entry not found: {id}"));
        }
        Ok(())
    }

    fn list(&self, offset: usize, limit: usize) -> Result<Vec<JournalEntry>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, body, created_at, updated_at FROM journal_entries ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
            )
            .map_err(|e| format!("Failed to prepare list query: {e}"))?;

        let entries = stmt
            .query_map(rusqlite::params![limit as i64, offset as i64], |row| {
                Ok(JournalEntry {
                    id: row.get(0)?,
                    body: row.get(1)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(2)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                    updated_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(3)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .map_err(|e| format!("Failed to list entries: {e}"))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(entries)
    }

    fn search(&self, query: &str) -> Result<Vec<JournalEntry>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT je.id, je.body, je.created_at, je.updated_at
                 FROM journal_entries_fts fts
                 JOIN journal_entries je ON je.rowid = fts.rowid
                 WHERE journal_entries_fts MATCH ?1
                 ORDER BY rank",
            )
            .map_err(|e| format!("Failed to prepare search query: {e}"))?;

        let entries = stmt
            .query_map(rusqlite::params![query], |row| {
                Ok(JournalEntry {
                    id: row.get(0)?,
                    body: row.get(1)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(2)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                    updated_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(3)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .map_err(|e| format!("Failed to search entries: {e}"))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(entries)
    }

    fn get_recent(&self, limit: usize) -> Result<Vec<JournalEntry>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, body, created_at, updated_at FROM journal_entries ORDER BY created_at DESC LIMIT ?1",
            )
            .map_err(|e| format!("Failed to prepare get_recent query: {e}"))?;

        let entries = stmt
            .query_map(rusqlite::params![limit as i64], |row| {
                Ok(JournalEntry {
                    id: row.get(0)?,
                    body: row.get(1)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(2)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                    updated_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(3)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .map_err(|e| format!("Failed to get recent entries: {e}"))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(entries)
    }

    fn get_by_ids(&self, ids: &[&str]) -> Result<Vec<JournalEntry>, String> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.db.conn.lock().unwrap();
        let placeholders = vec!["?"; ids.len()].join(", ");
        let query = format!(
            "SELECT id, body, created_at, updated_at FROM journal_entries WHERE id IN ({})",
            placeholders
        );

        let mut stmt = conn
            .prepare(&query)
            .map_err(|e| format!("Failed to prepare get_by_ids query: {e}"))?;

        let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

        let entries = stmt
            .query_map(params.as_slice(), |row| {
                Ok(JournalEntry {
                    id: row.get(0)?,
                    body: row.get(1)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(2)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                    updated_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(3)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .map_err(|e| format!("Failed to get entries by ids: {e}"))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(entries)
    }
}

use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn setup() -> (Arc<Database>, SqliteJournalRepository) {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let repo = SqliteJournalRepository::new(db.clone());
        (db, repo)
    }

    #[test]
    fn test_crud_roundtrip() {
        let (_db, repo) = setup();
        let entry = JournalEntry::new("Aujourd'hui était une bonne journée".to_string());
        let id = entry.id.clone();

        // Create
        repo.create(&entry).unwrap();

        // Read
        let fetched = repo.get(&id).unwrap().expect("Entry should exist");
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.body, "Aujourd'hui était une bonne journée");

        // Update
        let mut updated = fetched;
        updated.body = "Journée modifiée".to_string();
        updated.updated_at = chrono::Local::now().naive_local();
        repo.update(&updated).unwrap();

        let fetched2 = repo.get(&id).unwrap().unwrap();
        assert_eq!(fetched2.body, "Journée modifiée");
        assert!(fetched2.updated_at >= updated.created_at);

        // Delete
        repo.delete(&id).unwrap();
        let fetched3 = repo.get(&id).unwrap();
        assert!(fetched3.is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let (_db, repo) = setup();
        let result = repo.delete("nonexistent-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_sorted_by_date_desc() {
        let (_db, repo) = setup();

        let mut e1 = JournalEntry::new("First".to_string());
        e1.created_at = chrono::NaiveDateTime::parse_from_str("2026-04-01T10:00:00.0", DATETIME_FMT).unwrap();
        e1.updated_at = e1.created_at;

        let mut e2 = JournalEntry::new("Second".to_string());
        e2.created_at = chrono::NaiveDateTime::parse_from_str("2026-04-03T10:00:00.0", DATETIME_FMT).unwrap();
        e2.updated_at = e2.created_at;

        let mut e3 = JournalEntry::new("Third".to_string());
        e3.created_at = chrono::NaiveDateTime::parse_from_str("2026-04-02T10:00:00.0", DATETIME_FMT).unwrap();
        e3.updated_at = e3.created_at;

        repo.create(&e1).unwrap();
        repo.create(&e2).unwrap();
        repo.create(&e3).unwrap();

        let list = repo.list(0, 10).unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].body, "Second"); // Most recent
        assert_eq!(list[1].body, "Third");
        assert_eq!(list[2].body, "First"); // Oldest
    }

    #[test]
    fn test_list_pagination() {
        let (_db, repo) = setup();

        for i in 0..5 {
            let entry = JournalEntry::new(format!("Entry {i}"));
            repo.create(&entry).unwrap();
        }

        let page1 = repo.list(0, 2).unwrap();
        assert_eq!(page1.len(), 2);

        let page2 = repo.list(2, 2).unwrap();
        assert_eq!(page2.len(), 2);

        let page3 = repo.list(4, 2).unwrap();
        assert_eq!(page3.len(), 1);
    }

    #[test]
    fn test_search_fts5() {
        let (_db, repo) = setup();

        let e1 = JournalEntry::new("Aujourd'hui j'ai ressenti de la joie et du bonheur".to_string());
        let e2 = JournalEntry::new("Tristesse profonde ce soir après la dispute".to_string());
        let e3 = JournalEntry::new("La joie de retrouver un ami".to_string());

        repo.create(&e1).unwrap();
        repo.create(&e2).unwrap();
        repo.create(&e3).unwrap();

        let results = repo.search("joie").unwrap();
        assert_eq!(results.len(), 2);

        let results = repo.search("tristesse").unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].body.contains("Tristesse"));
    }

    #[test]
    fn test_search_no_results() {
        let (_db, repo) = setup();
        let e1 = JournalEntry::new("Journée ordinaire".to_string());
        repo.create(&e1).unwrap();

        let results = repo.search("inexistant").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_recent() {
        let (_db, repo) = setup();

        let mut e1 = JournalEntry::new("First".to_string());
        e1.created_at = chrono::Local::now().naive_local() - chrono::Duration::days(2);
        
        let mut e2 = JournalEntry::new("Second".to_string());
        e2.created_at = chrono::Local::now().naive_local(); // Most recent
        
        let mut e3 = JournalEntry::new("Third".to_string());
        e3.created_at = chrono::Local::now().naive_local() - chrono::Duration::days(1);

        repo.create(&e1).unwrap();
        repo.create(&e2).unwrap();
        repo.create(&e3).unwrap();

        let list = repo.get_recent(2).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].body, "Second"); // Most recent
        assert_eq!(list[1].body, "Third");

        let list2 = repo.get_recent(5).unwrap();
        assert_eq!(list2.len(), 3);
        
        let list_empty = repo.get_recent(0).unwrap();
        assert_eq!(list_empty.len(), 0);
    }

    #[test]
    fn test_get_by_ids() {
        let (_db, repo) = setup();

        let e1 = JournalEntry::new("Entry 1".to_string());
        let e2 = JournalEntry::new("Entry 2".to_string());
        let e3 = JournalEntry::new("Entry 3".to_string());

        repo.create(&e1).unwrap();
        repo.create(&e2).unwrap();
        repo.create(&e3).unwrap();

        let ids = vec![e1.id.as_str(), e3.id.as_str()];
        let mut retrieved = repo.get_by_ids(&ids).unwrap();
        assert_eq!(retrieved.len(), 2);
        
        retrieved.sort_by(|a, b| a.body.cmp(&b.body));
        assert_eq!(retrieved[0].body, "Entry 1");
        assert_eq!(retrieved[1].body, "Entry 3");

        let empty_retrieved = repo.get_by_ids(&[]).unwrap();
        assert_eq!(empty_retrieved.len(), 0);

        let partial_ids = vec![e2.id.as_str(), "non_existent_id"];
        let partial_retrieved = repo.get_by_ids(&partial_ids).unwrap();
        assert_eq!(partial_retrieved.len(), 1);
        assert_eq!(partial_retrieved[0].body, "Entry 2");
    }
}
