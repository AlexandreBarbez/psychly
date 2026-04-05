/// Builds the prompt to instruct the LLM to analyze a journal entry.
/// Returns a system prompt and the user prompt.
pub fn build_analysis_prompt(entry_body: &str) -> (String, String) {
    let system = r#"Tu es un assistant spécialisé en analyse psychologique de textes personnels. Tu dois analyser le texte fourni et retourner un résultat structuré en JSON avec exactement ce format :

{
  "emotional_tone": "une émotion principale identifiée (en un ou deux mots)",
  "themes": ["thème 1", "thème 2", ...],
  "patterns": ["pattern cognitif 1", "pattern cognitif 2", ...]
}

Règles :
- emotional_tone : l'émotion dominante du texte (ex: tristesse, anxiété, colère, joie, frustration, sérénité, culpabilité, honte, peur, espoir)
- themes : les sujets abordés (ex: travail, relations, famille, santé, estime de soi, solitude, avenir)
- patterns : les distorsions cognitives ou patterns observés (ex: catastrophisation, pensée tout-ou-rien, surgénéralisation, personnalisation, filtre mental, raisonnement émotionnel, lecture de pensée)
- Retourne UNIQUEMENT le JSON, sans texte avant ou après
- Les listes peuvent être vides si rien de significatif n'est identifié
- Tout en français"#.to_string();

    let user = format!("Analyse cette entrée de journal :\n\n{entry_body}");

    (system, user)
}

/// Parses the LLM response into structured analysis fields.
pub fn parse_analysis_response(response: &str) -> Result<(String, Vec<String>, Vec<String>), String> {
    // Try to extract JSON from the response (LLM might add text around it)
    let json_str = extract_json(response)?;

    let parsed: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse analysis JSON: {e}"))?;

    let emotional_tone = parsed["emotional_tone"]
        .as_str()
        .unwrap_or("indéterminé")
        .to_string();

    let themes = parsed["themes"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let patterns = parsed["patterns"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok((emotional_tone, themes, patterns))
}

/// Extracts a JSON object from a string that may contain surrounding text.
fn extract_json(text: &str) -> Result<String, String> {
    let trimmed = text.trim();

    // If it starts with {, try to use it directly
    if trimmed.starts_with('{') {
        if let Some(end) = find_matching_brace(trimmed) {
            return Ok(trimmed[..=end].to_string());
        }
    }

    // Otherwise search for { within the text
    if let Some(start) = trimmed.find('{') {
        let rest = &trimmed[start..];
        if let Some(end) = find_matching_brace(rest) {
            return Ok(rest[..=end].to_string());
        }
    }

    Err("No valid JSON object found in response".to_string())
}

/// Finds the index of the closing brace matching the opening brace at position 0.
fn find_matching_brace(text: &str) -> Option<usize> {
    let mut depth = 0;
    let mut in_string = false;
    let mut escape = false;

    for (i, ch) in text.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        match ch {
            '\\' if in_string => escape = true,
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_analysis_prompt() {
        let (system, user) = build_analysis_prompt("Je me sens triste aujourd'hui.");
        assert!(system.contains("emotional_tone"));
        assert!(system.contains("themes"));
        assert!(system.contains("patterns"));
        assert!(user.contains("Je me sens triste"));
    }

    #[test]
    fn test_parse_clean_json() {
        let response = r#"{"emotional_tone": "tristesse", "themes": ["solitude", "travail"], "patterns": ["catastrophisation"]}"#;
        let (tone, themes, patterns) = parse_analysis_response(response).unwrap();
        assert_eq!(tone, "tristesse");
        assert_eq!(themes, vec!["solitude", "travail"]);
        assert_eq!(patterns, vec!["catastrophisation"]);
    }

    #[test]
    fn test_parse_json_with_surrounding_text() {
        let response = r#"Voici l'analyse :
{"emotional_tone": "anxiété", "themes": ["avenir"], "patterns": []}
C'est mon analyse."#;
        let (tone, themes, patterns) = parse_analysis_response(response).unwrap();
        assert_eq!(tone, "anxiété");
        assert_eq!(themes, vec!["avenir"]);
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_parse_empty_fields() {
        let response = r#"{"emotional_tone": "neutre", "themes": [], "patterns": []}"#;
        let (tone, themes, patterns) = parse_analysis_response(response).unwrap();
        assert_eq!(tone, "neutre");
        assert!(themes.is_empty());
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_parse_invalid_json() {
        let result = parse_analysis_response("not json at all");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_json_direct() {
        let json = extract_json(r#"{"key": "value"}"#).unwrap();
        assert_eq!(json, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_json_nested() {
        let json = extract_json(r#"text {"a": {"b": 1}} end"#).unwrap();
        assert_eq!(json, r#"{"a": {"b": 1}}"#);
    }
}
