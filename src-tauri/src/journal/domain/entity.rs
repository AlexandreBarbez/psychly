use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl JournalEntry {
    /// Creates a new JournalEntry with auto-generated id and timestamps.
    pub fn new(body: String) -> Self {
        let now = Local::now().naive_local();
        Self {
            id: Uuid::new_v4().to_string(),
            body,
            created_at: now,
            updated_at: now,
        }
    }

    /// Returns a text preview (first N chars) of the entry body.
    pub fn preview(&self, max_chars: usize) -> String {
        if self.body.len() <= max_chars {
            self.body.clone()
        } else {
            let truncated: String = self.body.chars().take(max_chars).collect();
            format!("{truncated}…")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry_has_id_and_timestamps() {
        let entry = JournalEntry::new("Test body".to_string());
        assert!(!entry.id.is_empty());
        assert_eq!(entry.body, "Test body");
        assert_eq!(entry.created_at, entry.updated_at);
    }

    #[test]
    fn test_two_entries_have_different_ids() {
        let e1 = JournalEntry::new("A".to_string());
        let e2 = JournalEntry::new("B".to_string());
        assert_ne!(e1.id, e2.id);
    }

    #[test]
    fn test_preview_short_body() {
        let entry = JournalEntry::new("Court".to_string());
        assert_eq!(entry.preview(100), "Court");
    }

    #[test]
    fn test_preview_long_body() {
        let entry = JournalEntry::new("Ceci est un texte assez long pour le test".to_string());
        let preview = entry.preview(10);
        assert!(preview.ends_with('…'));
        assert!(preview.chars().count() <= 11); // 10 + ellipsis
    }
}
