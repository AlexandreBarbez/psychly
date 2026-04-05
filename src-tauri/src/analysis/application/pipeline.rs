use std::sync::Arc;

use crate::db::Database;
use crate::therapy::infrastructure::ollama_client::OllamaClient;
use crate::analysis::domain::{EntryAnalysis, AnalysisRepository};
use crate::analysis::infrastructure::sqlite_repository::SqliteAnalysisRepository;
use super::analysis_prompt::{build_analysis_prompt, parse_analysis_response};

/// Triggers an asynchronous analysis of a journal entry.
/// Spawns a background task that calls Ollama and stores the result.
pub fn trigger_analysis(db: Arc<Database>, ollama: OllamaClient, entry_id: String, entry_body: String) {
    tokio::spawn(async move {
        if let Err(e) = run_analysis(db, &ollama, &entry_id, &entry_body).await {
            log::error!("Analysis failed for entry {entry_id}: {e}");
        }
    });
}

/// Runs the analysis pipeline: build prompt, call LLM, parse, store.
async fn run_analysis(
    db: Arc<Database>,
    ollama: &OllamaClient,
    entry_id: &str,
    entry_body: &str,
) -> Result<(), String> {
    let (system, user) = build_analysis_prompt(entry_body);
    let response = ollama.generate(user, Some(system)).await?;
    let (emotional_tone, themes, patterns) = parse_analysis_response(&response)?;

    let analysis = EntryAnalysis::new(entry_id.to_string(), emotional_tone, themes, patterns);
    let repo = SqliteAnalysisRepository::new(db);
    repo.store(&analysis)?;

    Ok(())
}
