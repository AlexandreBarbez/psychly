use std::sync::Arc;
use chrono::NaiveDateTime;
use rusqlite::OptionalExtension;

use crate::db::Database;
use crate::analysis::domain::{EntryAnalysis, AnalysisRepository};

const DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S%.f";

pub struct SqliteAnalysisRepository {
    db: Arc<Database>,
}

impl SqliteAnalysisRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl AnalysisRepository for SqliteAnalysisRepository {
    fn store(&self, analysis: &EntryAnalysis) -> Result<(), String> {
        let conn = self.db.conn.lock().unwrap();
        let themes_json = serde_json::to_string(&analysis.themes)
            .map_err(|e| format!("Failed to serialize themes: {e}"))?;
        let patterns_json = serde_json::to_string(&analysis.patterns)
            .map_err(|e| format!("Failed to serialize patterns: {e}"))?;

        // Upsert: replace existing analysis for the same entry
        conn.execute(
            "INSERT INTO entry_analyses (id, entry_id, emotional_tone, themes, patterns, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(entry_id) DO UPDATE SET
               id = excluded.id,
               emotional_tone = excluded.emotional_tone,
               themes = excluded.themes,
               patterns = excluded.patterns,
               created_at = excluded.created_at",
            rusqlite::params![
                analysis.id,
                analysis.entry_id,
                analysis.emotional_tone,
                themes_json,
                patterns_json,
                analysis.created_at.format(DATETIME_FMT).to_string(),
            ],
        )
        .map_err(|e| format!("Failed to store analysis: {e}"))?;
        Ok(())
    }

    fn get_by_entry_id(&self, entry_id: &str) -> Result<Option<EntryAnalysis>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, entry_id, emotional_tone, themes, patterns, created_at FROM entry_analyses WHERE entry_id = ?1")
            .map_err(|e| format!("Failed to prepare query: {e}"))?;

        let result = stmt
            .query_row(rusqlite::params![entry_id], |row| {
                let themes_str: String = row.get(3)?;
                let patterns_str: String = row.get(4)?;
                Ok(EntryAnalysis {
                    id: row.get(0)?,
                    entry_id: row.get(1)?,
                    emotional_tone: row.get(2)?,
                    themes: serde_json::from_str(&themes_str).unwrap_or_default(),
                    patterns: serde_json::from_str(&patterns_str).unwrap_or_default(),
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(5)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .optional()
            .map_err(|e| format!("Failed to get analysis: {e}"))?;

        Ok(result)
    }

    fn get_recent(&self, limit: usize) -> Result<Vec<EntryAnalysis>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, entry_id, emotional_tone, themes, patterns, created_at FROM entry_analyses ORDER BY created_at DESC LIMIT ?1")
            .map_err(|e| format!("Failed to prepare query: {e}"))?;

        let analyses = stmt
            .query_map(rusqlite::params![limit as i64], |row| {
                let themes_str: String = row.get(3)?;
                let patterns_str: String = row.get(4)?;
                Ok(EntryAnalysis {
                    id: row.get(0)?,
                    entry_id: row.get(1)?,
                    emotional_tone: row.get(2)?,
                    themes: serde_json::from_str(&themes_str).unwrap_or_default(),
                    patterns: serde_json::from_str(&patterns_str).unwrap_or_default(),
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(5)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .map_err(|e| format!("Failed to list analyses: {e}"))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(analyses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Arc<Database> {
        let db = Arc::new(Database::open_in_memory().unwrap());
        // Insert a journal entry for FK constraint
        {
            let conn = db.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES ('e1', 'test', '2025-01-01T00:00:00.0', '2025-01-01T00:00:00.0')",
                [],
            ).unwrap();
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES ('e2', 'test2', '2025-01-02T00:00:00.0', '2025-01-02T00:00:00.0')",
                [],
            ).unwrap();
        }
        db
    }

    #[test]
    fn test_store_and_get_analysis() {
        let db = setup();
        let repo = SqliteAnalysisRepository::new(db);
        let analysis = EntryAnalysis::new(
            "e1".to_string(),
            "tristesse".to_string(),
            vec!["solitude".to_string()],
            vec!["catastrophisation".to_string()],
        );
        repo.store(&analysis).unwrap();

        let found = repo.get_by_entry_id("e1").unwrap().unwrap();
        assert_eq!(found.entry_id, "e1");
        assert_eq!(found.emotional_tone, "tristesse");
        assert_eq!(found.themes, vec!["solitude"]);
        assert_eq!(found.patterns, vec!["catastrophisation"]);
    }

    #[test]
    fn test_get_nonexistent_analysis() {
        let db = setup();
        let repo = SqliteAnalysisRepository::new(db);
        let found = repo.get_by_entry_id("nope").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_upsert_replaces_analysis() {
        let db = setup();
        let repo = SqliteAnalysisRepository::new(db);

        let a1 = EntryAnalysis::new("e1".to_string(), "tristesse".to_string(), vec![], vec![]);
        repo.store(&a1).unwrap();

        let a2 = EntryAnalysis::new("e1".to_string(), "joie".to_string(), vec!["progrès".to_string()], vec![]);
        repo.store(&a2).unwrap();

        let found = repo.get_by_entry_id("e1").unwrap().unwrap();
        assert_eq!(found.emotional_tone, "joie");
        assert_eq!(found.themes, vec!["progrès"]);
    }

    #[test]
    fn test_get_recent() {
        let db = setup();
        let repo = SqliteAnalysisRepository::new(db);

        let a1 = EntryAnalysis::new("e1".to_string(), "tristesse".to_string(), vec![], vec![]);
        let a2 = EntryAnalysis::new("e2".to_string(), "joie".to_string(), vec![], vec![]);
        repo.store(&a1).unwrap();
        repo.store(&a2).unwrap();

        let recent = repo.get_recent(10).unwrap();
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_get_recent_with_limit() {
        let db = setup();
        let repo = SqliteAnalysisRepository::new(db);

        let a1 = EntryAnalysis::new("e1".to_string(), "tristesse".to_string(), vec![], vec![]);
        let a2 = EntryAnalysis::new("e2".to_string(), "joie".to_string(), vec![], vec![]);
        repo.store(&a1).unwrap();
        repo.store(&a2).unwrap();

        let recent = repo.get_recent(1).unwrap();
        assert_eq!(recent.len(), 1);
    }
}
