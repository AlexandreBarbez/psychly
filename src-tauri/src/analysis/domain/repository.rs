use super::entity::EntryAnalysis;

pub trait AnalysisRepository {
    fn store(&self, analysis: &EntryAnalysis) -> Result<(), String>;
    fn get_by_entry_id(&self, entry_id: &str) -> Result<Option<EntryAnalysis>, String>;
    fn get_recent(&self, limit: usize) -> Result<Vec<EntryAnalysis>, String>;
}
