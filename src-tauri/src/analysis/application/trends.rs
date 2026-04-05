use std::collections::HashMap;

use crate::analysis::domain::EntryAnalysis;

/// Aggregates emotional tones from recent analyses into a weekly summary.
///
/// Returns a list of (emotion, count) pairs sorted by frequency descending.
/// Returns empty if fewer than 3 analyses are provided.
pub fn aggregate_emotional_trends(analyses: &[EntryAnalysis]) -> Vec<(String, usize)> {
    if analyses.len() < 3 {
        return Vec::new();
    }

    let mut counts: HashMap<String, usize> = HashMap::new();
    for analysis in analyses {
        *counts.entry(analysis.emotional_tone.clone()).or_insert(0) += 1;
    }

    let mut sorted: Vec<(String, usize)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted
}

/// Formats the emotional trends as a human-readable French summary.
pub fn format_trends_summary(trends: &[(String, usize)]) -> String {
    if trends.is_empty() {
        return String::new();
    }

    let parts: Vec<String> = trends
        .iter()
        .map(|(emotion, count)| format!("{emotion} ({count}x)"))
        .collect();

    format!("Tendances émotionnelles récentes : {}", parts.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::domain::EntryAnalysis;

    fn make_analysis(tone: &str) -> EntryAnalysis {
        EntryAnalysis::new("e".to_string(), tone.to_string(), vec![], vec![])
    }

    #[test]
    fn test_aggregate_insufficient_entries() {
        let analyses = vec![make_analysis("tristesse"), make_analysis("joie")];
        let result = aggregate_emotional_trends(&analyses);
        assert!(result.is_empty());
    }

    #[test]
    fn test_aggregate_sufficient_entries() {
        let analyses = vec![
            make_analysis("tristesse"),
            make_analysis("tristesse"),
            make_analysis("anxiété"),
            make_analysis("tristesse"),
        ];
        let result = aggregate_emotional_trends(&analyses);
        assert_eq!(result[0].0, "tristesse");
        assert_eq!(result[0].1, 3);
        assert_eq!(result[1].0, "anxiété");
        assert_eq!(result[1].1, 1);
    }

    #[test]
    fn test_format_trends_empty() {
        assert_eq!(format_trends_summary(&[]), "");
    }

    #[test]
    fn test_format_trends_summary() {
        let trends = vec![
            ("tristesse".to_string(), 3),
            ("anxiété".to_string(), 1),
        ];
        let summary = format_trends_summary(&trends);
        assert!(summary.contains("tristesse (3x)"));
        assert!(summary.contains("anxiété (1x)"));
    }
}
