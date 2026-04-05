use super::entity::JournalEntry;

/// Defines the contract for journal entry persistence.
pub trait JournalRepository {
    fn create(&self, entry: &JournalEntry) -> Result<(), String>;
    fn get(&self, id: &str) -> Result<Option<JournalEntry>, String>;
    fn update(&self, entry: &JournalEntry) -> Result<(), String>;
    fn delete(&self, id: &str) -> Result<(), String>;
    fn list(&self, offset: usize, limit: usize) -> Result<Vec<JournalEntry>, String>;
    fn search(&self, query: &str) -> Result<Vec<JournalEntry>, String>;
    fn get_recent(&self, limit: usize) -> Result<Vec<JournalEntry>, String>;
    fn get_by_ids(&self, ids: &[&str]) -> Result<Vec<JournalEntry>, String>;
}
