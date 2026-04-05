use std::sync::Arc;
use chrono::Local;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::Database;
use crate::journal::domain::{JournalEntry, JournalRepository};
use crate::journal::infrastructure::SqliteJournalRepository;
use crate::therapy::infrastructure::ollama_client::OllamaClient;
use crate::analysis::application::pipeline::trigger_analysis;

#[derive(Serialize)]
pub struct EntryResponse {
    pub id: String,
    pub body: String,
    pub preview: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<JournalEntry> for EntryResponse {
    fn from(e: JournalEntry) -> Self {
        let preview = e.preview(150);
        Self {
            id: e.id,
            body: e.body,
            preview,
            created_at: e.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            updated_at: e.updated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateEntryInput {
    pub body: String,
}

#[derive(Deserialize)]
pub struct UpdateEntryInput {
    pub id: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct ListEntriesInput {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[tauri::command]
pub async fn create_entry(
    db: State<'_, Arc<Database>>,
    ollama: State<'_, OllamaClient>,
    input: CreateEntryInput,
) -> Result<EntryResponse, String> {
    let repo = SqliteJournalRepository::new(db.inner().clone());
    let entry = JournalEntry::new(input.body);
    repo.create(&entry)?;

    // Trigger async analysis
    trigger_analysis(
        db.inner().clone(),
        ollama.inner().clone(),
        entry.id.clone(),
        entry.body.clone(),
    );

    Ok(EntryResponse::from(entry))
}

#[tauri::command]
pub fn get_entry(
    db: State<'_, Arc<Database>>,
    id: String,
) -> Result<Option<EntryResponse>, String> {
    let repo = SqliteJournalRepository::new(db.inner().clone());
    let entry = repo.get(&id)?;
    Ok(entry.map(EntryResponse::from))
}

#[tauri::command]
pub fn list_entries(
    db: State<'_, Arc<Database>>,
    input: ListEntriesInput,
) -> Result<Vec<EntryResponse>, String> {
    let repo = SqliteJournalRepository::new(db.inner().clone());
    let offset = input.offset.unwrap_or(0);
    let limit = input.limit.unwrap_or(50);
    let entries = repo.list(offset, limit)?;
    Ok(entries.into_iter().map(EntryResponse::from).collect())
}

#[tauri::command]
pub async fn update_entry(
    db: State<'_, Arc<Database>>,
    ollama: State<'_, OllamaClient>,
    input: UpdateEntryInput,
) -> Result<EntryResponse, String> {
    let repo = SqliteJournalRepository::new(db.inner().clone());
    let existing = repo.get(&input.id)?.ok_or_else(|| format!("Entry not found: {}", input.id))?;
    let updated = JournalEntry {
        id: existing.id,
        body: input.body,
        created_at: existing.created_at,
        updated_at: Local::now().naive_local(),
    };
    repo.update(&updated)?;

    // Re-trigger analysis on edit
    trigger_analysis(
        db.inner().clone(),
        ollama.inner().clone(),
        updated.id.clone(),
        updated.body.clone(),
    );

    Ok(EntryResponse::from(updated))
}

#[tauri::command]
pub fn delete_entry(
    db: State<'_, Arc<Database>>,
    id: String,
) -> Result<(), String> {
    let repo = SqliteJournalRepository::new(db.inner().clone());
    repo.delete(&id)
}

#[tauri::command]
pub fn search_entries(
    db: State<'_, Arc<Database>>,
    query: String,
) -> Result<Vec<EntryResponse>, String> {
    let repo = SqliteJournalRepository::new(db.inner().clone());
    let entries = repo.search(&query)?;
    Ok(entries.into_iter().map(EntryResponse::from).collect())
}
