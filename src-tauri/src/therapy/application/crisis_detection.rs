/// Crisis keywords and patterns that indicate potential self-harm or acute crisis.
const CRISIS_KEYWORDS: &[&str] = &[
    "suicid",
    "me tuer",
    "me suicider",
    "envie de mourir",
    "en finir",
    "finir avec la vie",
    "plus envie de vivre",
    "ne veux plus vivre",
    "veux mourir",
    "automutilation",
    "me faire du mal",
    "me couper",
    "sauter du pont",
    "avaler des médicaments",
    "overdose",
    "plus de raison de vivre",
    "personne ne m'aime",
    "mieux sans moi",
    "le monde serait mieux sans moi",
    "je suis un fardeau",
    "je ne sers à rien",
    "tout est foutu",
    "je n'en peux plus",
];

/// The safety response message returned when crisis is detected.
pub fn crisis_safety_response() -> String {
    r#"Je t'entends, et ce que tu ressens est important. Je veux que tu saches que tu n'es pas seul(e).

Ce que tu décris m'inquiète et je tiens à être honnête avec toi : je suis un outil d'accompagnement, pas un professionnel de santé mentale. Dans cette situation, il est essentiel que tu puisses parler à quelqu'un de qualifié.

**Appelle le 3114** — c'est le numéro national de prévention du suicide. Il est gratuit, confidentiel, et disponible 24h/24, 7j/7. Des professionnels formés sont là pour t'écouter.

Tu peux aussi contacter :
- **SOS Amitié** : 09 72 39 40 50
- **Fil Santé Jeunes** (si tu as moins de 25 ans) : 0 800 235 236
- Les **urgences** : 15 (SAMU) ou 112

Tu mérites d'être accompagné(e) par un vrai professionnel. Prendre soin de toi, c'est un acte de courage."#.to_string()
}

/// Detects if the user's message contains crisis indicators.
///
/// Returns `true` if the message matches any crisis pattern.
pub fn detect_crisis(message: &str) -> bool {
    let lower = message.to_lowercase();
    CRISIS_KEYWORDS.iter().any(|keyword| lower.contains(keyword))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_crisis_positive() {
        assert!(detect_crisis("J'ai envie de me suicider"));
        assert!(detect_crisis("Je veux en finir avec la vie"));
        assert!(detect_crisis("Je n'ai plus envie de vivre"));
        assert!(detect_crisis("je veux me tuer"));
        assert!(detect_crisis("le monde serait mieux sans moi"));
        assert!(detect_crisis("j'ai envie de mourir"));
    }

    #[test]
    fn test_detect_crisis_case_insensitive() {
        assert!(detect_crisis("JE VEUX EN FINIR"));
        assert!(detect_crisis("Me Suicider"));
    }

    #[test]
    fn test_detect_crisis_negative() {
        assert!(!detect_crisis("Je me sens triste aujourd'hui"));
        assert!(!detect_crisis("J'ai eu une mauvaise journée"));
        assert!(!detect_crisis("Je suis stressé par le travail"));
        assert!(!detect_crisis("Je me sens seul"));
    }

    #[test]
    fn test_crisis_response_contains_helpline() {
        let response = crisis_safety_response();
        assert!(response.contains("3114"));
        assert!(response.contains("SOS Amitié"));
        assert!(response.contains("professionnel"));
    }

    #[test]
    fn test_detect_crisis_partial_match() {
        // "suicid" matches "suicidaire", "suicide", etc.
        assert!(detect_crisis("J'ai des pensées suicidaires"));
        assert!(detect_crisis("Le suicide me semble une option"));
    }

    #[test]
    fn test_detect_crisis_automutilation() {
        assert!(detect_crisis("Je fais de l'automutilation"));
        assert!(detect_crisis("J'ai envie de me faire du mal"));
        assert!(detect_crisis("J'ai envie de me couper"));
    }
}
