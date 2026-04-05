use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryAnalysis {
    pub id: String,
    pub entry_id: String,
    pub emotional_tone: String,
    pub themes: Vec<String>,
    pub patterns: Vec<String>,
    pub created_at: NaiveDateTime,
}

impl EntryAnalysis {
    pub fn new(entry_id: String, emotional_tone: String, themes: Vec<String>, patterns: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            entry_id,
            emotional_tone,
            themes,
            patterns,
            created_at: Local::now().naive_local(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_analysis_has_id() {
        let analysis = EntryAnalysis::new(
            "entry-1".to_string(),
            "tristesse".to_string(),
            vec!["solitude".to_string()],
            vec!["catastrophisation".to_string()],
        );
        assert!(!analysis.id.is_empty());
        assert_eq!(analysis.entry_id, "entry-1");
        assert_eq!(analysis.emotional_tone, "tristesse");
        assert_eq!(analysis.themes.len(), 1);
        assert_eq!(analysis.patterns.len(), 1);
    }

    #[test]
    fn test_analysis_with_multiple_themes_and_patterns() {
        let analysis = EntryAnalysis::new(
            "entry-2".to_string(),
            "anxiété".to_string(),
            vec!["travail".to_string(), "performance".to_string()],
            vec!["pensée tout-ou-rien".to_string(), "surgénéralisation".to_string()],
        );
        assert_eq!(analysis.themes.len(), 2);
        assert_eq!(analysis.patterns.len(), 2);
    }

    #[test]
    fn test_two_analyses_have_different_ids() {
        let a1 = EntryAnalysis::new("e1".to_string(), "joie".to_string(), vec![], vec![]);
        let a2 = EntryAnalysis::new("e2".to_string(), "joie".to_string(), vec![], vec![]);
        assert_ne!(a1.id, a2.id);
    }
}
