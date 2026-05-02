use std::collections::HashSet;
use std::sync::Arc;
use crate::db::Database;
use crate::analysis::domain::AnalysisRepository;
use crate::analysis::infrastructure::sqlite_repository::SqliteAnalysisRepository;
use crate::journal::domain::{JournalEntry, JournalRepository};
use crate::journal::infrastructure::sqlite_repository::SqliteJournalRepository;
use crate::therapy::application::prompt_assembly::{JOURNAL_CONTENT_BUDGET, CHARS_PER_TOKEN};
use super::trends::{aggregate_emotional_trends, format_trends_summary};

/// Retrieves the N most recent journal entries.
fn select_recent_entries(db: Arc<Database>, limit: usize) -> Vec<JournalEntry> {
    let repo = SqliteJournalRepository::new(db);
    repo.get_recent(limit).unwrap_or_default()
}

/// Recovers recent analyses, extracts themes from recent entries, finds older entries with the same themes,
/// and retrieves them (deduplicated against recent_entry_ids).
fn select_thematic_entries(db: Arc<Database>, recent_entry_ids: &HashSet<String>, limit: usize) -> Vec<JournalEntry> {
    let analysis_repo = SqliteAnalysisRepository::new(Arc::clone(&db));
    let analyses = analysis_repo.get_recent(20).unwrap_or_default();
    
    // Find themes from recent entries
    let mut recent_themes = HashSet::new();
    for a in &analyses {
        if recent_entry_ids.contains(&a.entry_id) {
            for t in &a.themes {
                recent_themes.insert(t.clone());
            }
        }
    }

    // Find older entries sharing these themes
    let mut thematic_ids = Vec::new();
    for a in &analyses {
        if !recent_entry_ids.contains(&a.entry_id) {
            let has_match = a.themes.iter().any(|t| recent_themes.contains(t));
            if has_match {
                thematic_ids.push(a.entry_id.clone());
                if thematic_ids.len() >= limit {
                    break;
                }
            }
        }
    }

    if thematic_ids.is_empty() {
        return Vec::new();
    }

    let repo = SqliteJournalRepository::new(db);
    let ids_refs: Vec<&str> = thematic_ids.iter().map(|s| s.as_str()).collect();
    repo.get_by_ids(&ids_refs).unwrap_or_default()
}

/// Truncates each entry so they all fit in the token budget, appending '[...]' to truncated entries.
fn truncate_entries(entries: &[JournalEntry], budget_tokens: usize) -> Vec<(JournalEntry, String)> {
    if entries.is_empty() {
        return Vec::new();
    }
    
    // Allow each entry a fair share of the budget, but recent entries could use unused budget (simplification: strict fair share)
    let tokens_per_entry = budget_tokens / entries.len();
    let max_chars_per_entry = tokens_per_entry * CHARS_PER_TOKEN;
    
    entries.iter().map(|e| {
        if e.body.len() <= max_chars_per_entry {
            (e.clone(), e.body.clone())
        } else {
            let mut truncated = e.body.chars().take(max_chars_per_entry).collect::<String>();
            truncated.push_str(" [...]");
            (e.clone(), truncated)
        }
    }).collect()
}

/// Formats the selected entries into a chronological block.
fn format_journal_entries(entries: &[(JournalEntry, String)]) -> String {
    if entries.is_empty() {
        return String::new();
    }
    
    let mut sorted: Vec<&(JournalEntry, String)> = entries.iter().collect();
    // Oldest to most recent
    sorted.sort_by(|a, b| a.0.created_at.cmp(&b.0.created_at));
    
    let mut parts = Vec::new();
    parts.push("=== Entrées de journal récentes / pertinentes ===".to_string());
    
    for (entry, content) in sorted {
        let date_str = entry.created_at.format("%Y-%m-%d %H:%M").to_string();
        parts.push(format!("[Date : {}]\n{}", date_str, content));
    }
    
    parts.join("\n\n")
}

/// Builds a concise context summary from recent journal analyses
/// for injection into the therapy chat prompt.
///
/// Returns None if no context can be built.
pub fn build_chat_context(db: Arc<Database>) -> Option<String> {
    let repo = SqliteAnalysisRepository::new(Arc::clone(&db));
    let analyses = repo.get_recent(10).unwrap_or_default();

    let mut parts: Vec<String> = Vec::new();

    // 1. Fetch entries blocks
    let recent_entries = select_recent_entries(Arc::clone(&db), 5);
    let recent_ids: HashSet<String> = recent_entries.iter().map(|e| e.id.clone()).collect();
    let mut thematic_entries = select_thematic_entries(Arc::clone(&db), &recent_ids, 5);
    
    let mut all_entries = recent_entries;
    all_entries.append(&mut thematic_entries);
    
    if !all_entries.is_empty() {
        let truncated = truncate_entries(&all_entries, JOURNAL_CONTENT_BUDGET);
        let formatted = format_journal_entries(&truncated);
        if !formatted.is_empty() {
            parts.push(formatted);
        }
    }

    if analyses.is_empty() && parts.is_empty() {
        return None;
    }

    // Emotional trends
    let trends = aggregate_emotional_trends(&analyses);
    let trends_text = format_trends_summary(&trends);
    if !trends_text.is_empty() {
        parts.push(trends_text);
    }

    // Recurring themes
    let mut all_themes: Vec<String> = Vec::new();
    for a in &analyses {
        all_themes.extend(a.themes.clone());
    }
    if !all_themes.is_empty() {
        // Deduplicate and take top themes
        all_themes.sort();
        all_themes.dedup();
        let themes_preview: Vec<&str> = all_themes.iter().take(8).map(|s| s.as_str()).collect();
        parts.push(format!("Thèmes récurrents : {}", themes_preview.join(", ")));
    }

    // Cognitive patterns
    let mut all_patterns: Vec<String> = Vec::new();
    for a in &analyses {
        all_patterns.extend(a.patterns.clone());
    }
    if !all_patterns.is_empty() {
        all_patterns.sort();
        all_patterns.dedup();
        let patterns_preview: Vec<&str> = all_patterns.iter().take(6).map(|s| s.as_str()).collect();
        parts.push(format!("Patterns cognitifs observés : {}", patterns_preview.join(", ")));
    }

    if parts.is_empty() {
        return None;
    }

    Some(parts.join("\n\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::domain::EntryAnalysis;

    fn setup_with_analyses(analyses: &[EntryAnalysis]) -> Arc<Database> {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let repo = SqliteAnalysisRepository::new(Arc::clone(&db));

        // Create journal entries for FK constraints
        {
            let conn = db.conn.lock().unwrap();
            for a in analyses.iter() {
                conn.execute(
                    "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, 'test', '2025-01-01T00:00:00.0', '2025-01-01T00:00:00.0')",
                    rusqlite::params![a.entry_id],
                ).unwrap_or_else(|_| 0);
            }
        }

        for a in analyses {
            repo.store(a).unwrap();
        }
        db
    }

    #[test]
    fn test_build_context_empty() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        assert!(build_chat_context(db).is_none());
    }

    #[test]
    fn test_build_context_with_analyses() {
        let analyses = vec![
            EntryAnalysis::new("e1".to_string(), "tristesse".to_string(), vec!["solitude".to_string()], vec!["catastrophisation".to_string()]),
            EntryAnalysis::new("e2".to_string(), "tristesse".to_string(), vec!["travail".to_string()], vec!["pensée tout-ou-rien".to_string()]),
            EntryAnalysis::new("e3".to_string(), "anxiété".to_string(), vec!["avenir".to_string()], vec![]),
        ];
        let db = setup_with_analyses(&analyses);
        let context = build_chat_context(db).unwrap();
        assert!(context.contains("Tendances émotionnelles"));
        assert!(context.contains("Thèmes récurrents"));
        assert!(context.contains("Patterns cognitifs"));
    }

    #[test]
    fn test_build_context_insufficient_for_trends() {
        let analyses = vec![
            EntryAnalysis::new("e1".to_string(), "joie".to_string(), vec!["famille".to_string()], vec![]),
        ];
        let db = setup_with_analyses(&analyses);
        let context = build_chat_context(db).unwrap();
        // Should still have themes, but no trends (< 3 entries)
        assert!(!context.contains("Tendances"));
        assert!(context.contains("Thèmes"));
    }

    #[test]
    fn test_select_recent_entries() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let mut e1 = JournalEntry::new("test1".to_string());
        e1.created_at = chrono::Local::now().naive_local() - chrono::Duration::days(1);
        let mut e2 = JournalEntry::new("test2".to_string());
        e2.created_at = chrono::Local::now().naive_local();
        {
            let journal_repo = SqliteJournalRepository::new(Arc::clone(&db));
            journal_repo.create(&e1).unwrap();
            journal_repo.create(&e2).unwrap();
        }
        
        let recent = select_recent_entries(Arc::clone(&db), 1);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].body, "test2");
        
        let all_recent = select_recent_entries(Arc::clone(&db), 5);
        assert_eq!(all_recent.len(), 2);
    }

    #[test]
    fn test_select_thematic_entries() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let e1 = JournalEntry::new("recent theme X".to_string());
        let e2 = JournalEntry::new("older theme X".to_string());
        let e3 = JournalEntry::new("older theme Y".to_string());
        
        {
            let journal_repo = SqliteJournalRepository::new(Arc::clone(&db));
            journal_repo.create(&e1).unwrap();
            journal_repo.create(&e2).unwrap();
            journal_repo.create(&e3).unwrap();
            
            let analysis_repo = SqliteAnalysisRepository::new(Arc::clone(&db));
            let a1 = EntryAnalysis::new(e1.id.clone(), "tone".to_string(), vec!["X".to_string()], vec![]);
            let a2 = EntryAnalysis::new(e2.id.clone(), "tone".to_string(), vec!["X".to_string()], vec![]);
            let a3 = EntryAnalysis::new(e3.id.clone(), "tone".to_string(), vec!["Y".to_string()], vec![]);
            
            let conn = db.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, 'test', '2025-01-01T00:00:00.0', '2025-01-01T00:00:00.0')",
                rusqlite::params![a1.entry_id],
            ).unwrap_or(0);
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, 'test', '2025-01-01T00:00:00.0', '2025-01-01T00:00:00.0')",
                rusqlite::params![a2.entry_id],
            ).unwrap_or(0);
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, 'test', '2025-01-01T00:00:00.0', '2025-01-01T00:00:00.0')",
                rusqlite::params![a3.entry_id],
            ).unwrap_or(0);
            drop(conn);

            analysis_repo.store(&a1).unwrap();
            analysis_repo.store(&a2).unwrap();
            analysis_repo.store(&a3).unwrap();
        }
        
        let mut recent_ids = HashSet::new();
        recent_ids.insert(e1.id.clone());
        
        let thematic = select_thematic_entries(Arc::clone(&db), &recent_ids, 5);
        assert_eq!(thematic.len(), 1);
        assert_eq!(thematic[0].body, "older theme X");
    }

    #[test]
    fn test_truncate_entries() {
        let e1 = JournalEntry::new("short".to_string());
        let e2 = JournalEntry::new("this is a very long entry ".to_string());
        
        let entries = vec![e1.clone(), e2.clone()];
        let truncated = truncate_entries(&entries, 4);
        assert_eq!(truncated.len(), 2);
        assert_eq!(truncated[0].1, "short");
        assert!(truncated[1].1.contains("[...]"));
    }

    #[test]
    fn test_format_journal_entries() {
        let mut e1 = JournalEntry::new("body1".to_string());
        let mut e2 = JournalEntry::new("body2".to_string());
        e1.created_at = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap();
        e2.created_at = chrono::NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(12, 0, 0).unwrap();

        let entries = vec![(e2, "body2".to_string()), (e1, "body1".to_string())];
        let formatted = format_journal_entries(&entries);

        assert!(formatted.contains("=== Entrées de journal récentes / pertinentes ==="));
        assert!(formatted.contains("[Date : 2025-01-01 12:00]\nbody1"));
        assert!(formatted.contains("[Date : 2025-01-02 12:00]\nbody2"));
    }

    #[test]
    fn test_truncate_entries_single_entry_gets_full_budget() {
        // With 1 entry, budget_tokens / 1 = budget_tokens, max_chars = budget_tokens * CHARS_PER_TOKEN
        let budget_tokens: usize = 10; // 40 chars
        let max_chars = budget_tokens * CHARS_PER_TOKEN;

        // Entry shorter than budget — not truncated
        let short = JournalEntry::new("bonjour".to_string()); // 7 chars < 40
        let result = truncate_entries(&[short.clone()], budget_tokens);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "bonjour", "Short entry must not be truncated");
        assert!(!result[0].1.contains("[...]"));

        // Entry longer than budget — truncated to max_chars + " [...]"
        let long_body = "a".repeat(max_chars + 10); // 50 chars > 40
        let long = JournalEntry::new(long_body.clone());
        let result = truncate_entries(&[long], budget_tokens);
        assert_eq!(result.len(), 1);
        assert!(result[0].1.ends_with(" [...]"), "Long entry must end with truncation marker");
        assert_eq!(
            result[0].1.len(),
            max_chars + " [...]".len(),
            "Truncated body must be exactly max_chars + marker length"
        );
    }

    #[test]
    fn test_truncate_entries_all_fit_no_truncation() {
        // budget_tokens=100, 2 entries → 50 tokens each = 200 chars each (CHARS_PER_TOKEN=4)
        // Both entries are far below 200 chars, so neither should be truncated.
        let budget_tokens: usize = 100;
        let e1 = JournalEntry::new("court".to_string());         // 5 chars
        let e2 = JournalEntry::new("aussi court".to_string());   // 11 chars
        let result = truncate_entries(&[e1, e2], budget_tokens);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1, "court");
        assert_eq!(result[1].1, "aussi court");
        assert!(!result[0].1.contains("[...]"));
        assert!(!result[1].1.contains("[...]"));
    }

    #[test]
    fn test_truncate_entries_empty_input() {
        let result = truncate_entries(&[], 1000);
        assert!(result.is_empty(), "Empty input must return empty vec");
    }
}
