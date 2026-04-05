use std::sync::Arc;
use serde::Serialize;
use tauri::State;

use crate::db::Database;
use crate::analysis::domain::{EntryAnalysis, AnalysisRepository};
use crate::analysis::infrastructure::sqlite_repository::SqliteAnalysisRepository;

#[derive(Serialize)]
pub struct AnalysisResponse {
    pub id: String,
    pub entry_id: String,
    pub emotional_tone: String,
    pub themes: Vec<String>,
    pub patterns: Vec<String>,
    pub created_at: String,
}

impl From<EntryAnalysis> for AnalysisResponse {
    fn from(a: EntryAnalysis) -> Self {
        Self {
            id: a.id,
            entry_id: a.entry_id,
            emotional_tone: a.emotional_tone,
            themes: a.themes,
            patterns: a.patterns,
            created_at: a.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

#[tauri::command]
pub fn get_entry_analysis(
    db: State<'_, Arc<Database>>,
    entry_id: String,
) -> Result<Option<AnalysisResponse>, String> {
    let repo = SqliteAnalysisRepository::new(db.inner().clone());
    let analysis = repo.get_by_entry_id(&entry_id)?;
    Ok(analysis.map(AnalysisResponse::from))
}
