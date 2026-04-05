use crate::therapy::infrastructure::ollama_client::ChatMessage as OllamaChatMessage;
use crate::therapy::domain::ChatMessage;
use super::system_prompt::therapeutic_system_prompt;

/// Approximate token count: ~4 chars per token for French text.
pub const CHARS_PER_TOKEN: usize = 4;

/// Maximum context window in tokens (Qwen 2.5 14B supports 128K, we use a conservative limit).
pub const MAX_CONTEXT_TOKENS: usize = 16_000;

/// Reserve tokens for system prompt + journal context + new response.
pub const RESERVED_TOKENS: usize = 7_000;

/// Budget specifically allocated for journal entry content injection
pub const JOURNAL_CONTENT_BUDGET: usize = 3_000;

/// Assembles the full message list for the Ollama chat API.
///
/// Structure: [system_prompt, journal_context?, ...history, user_message]
pub fn assemble_prompt(
    journal_context: Option<&str>,
    history: &[ChatMessage],
    user_message: &str,
) -> Vec<OllamaChatMessage> {
    let mut messages = Vec::new();

    // System prompt
    let mut system_content = therapeutic_system_prompt();

    // Append journal context to system prompt if available
    if let Some(context) = journal_context {
        system_content.push_str("\n\n## Contexte du journal de l'utilisateur\n\n");
        system_content.push_str(context);
    }

    messages.push(OllamaChatMessage {
        role: "system".to_string(),
        content: system_content,
    });

    // Manage context window: summarize older messages if needed
    let managed_history = manage_context_window(history, user_message);

    // Add conversation history
    for msg in &managed_history {
        messages.push(OllamaChatMessage {
            role: msg.role.clone(),
            content: msg.content.clone(),
        });
    }

    // Add current user message
    messages.push(OllamaChatMessage {
        role: "user".to_string(),
        content: user_message.to_string(),
    });

    messages
}

/// Manages the context window by summarizing older messages when the
/// conversation exceeds the token budget.
fn manage_context_window(history: &[ChatMessage], user_message: &str) -> Vec<ChatMessage> {
    let available_tokens = MAX_CONTEXT_TOKENS - RESERVED_TOKENS;
    let user_msg_tokens = estimate_tokens(user_message);
    let budget = available_tokens.saturating_sub(user_msg_tokens);

    // Calculate total history tokens
    let total_history_tokens: usize = history.iter().map(|m| estimate_tokens(&m.content)).sum();

    if total_history_tokens <= budget {
        return history.to_vec();
    }

    // Need to trim: keep recent messages, summarize older ones
    let mut kept: Vec<ChatMessage> = Vec::new();
    let mut kept_tokens = 0;

    // Walk backwards from most recent, keeping messages until we exhaust budget
    // Reserve some budget for a summary of older messages
    let summary_budget = budget / 4;
    let recent_budget = budget - summary_budget;

    let mut recent_start = history.len();
    for (i, msg) in history.iter().enumerate().rev() {
        let msg_tokens = estimate_tokens(&msg.content);
        if kept_tokens + msg_tokens > recent_budget {
            recent_start = i + 1;
            break;
        }
        kept_tokens += msg_tokens;
        if i == 0 {
            recent_start = 0;
        }
    }

    // If we have older messages to summarize, create a summary
    if recent_start > 0 {
        let older = &history[..recent_start];
        let summary = summarize_messages(older);

        let summary_msg = ChatMessage {
            id: String::new(),
            session_id: String::new(),
            role: "system".to_string(),
            content: format!("[Résumé des échanges précédents]\n{summary}"),
            created_at: chrono::Local::now().naive_local(),
        };
        kept.push(summary_msg);
    }

    // Add the recent messages we kept
    kept.extend_from_slice(&history[recent_start..]);
    kept
}

/// Creates a simple extractive summary of older messages.
fn summarize_messages(messages: &[ChatMessage]) -> String {
    let mut summary_parts: Vec<String> = Vec::new();

    for msg in messages {
        let role_label = match msg.role.as_str() {
            "user" => "Utilisateur",
            "assistant" => "Assistant",
            _ => continue,
        };

        // Take first ~200 chars of each message as a summary line
        let content_preview = if msg.content.len() > 200 {
            let boundary = msg.content[..200]
                .rfind(' ')
                .unwrap_or(200);
            format!("{}...", &msg.content[..boundary])
        } else {
            msg.content.clone()
        };

        summary_parts.push(format!("- {role_label}: {content_preview}"));
    }

    summary_parts.join("\n")
}

/// Estimates the token count for a string of text.
pub fn estimate_tokens(text: &str) -> usize {
    text.len().div_ceil(CHARS_PER_TOKEN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::therapy::domain::ChatMessage;

    fn make_message(role: &str, content: &str) -> ChatMessage {
        ChatMessage {
            id: "test-id".to_string(),
            session_id: "test-session".to_string(),
            role: role.to_string(),
            content: content.to_string(),
            created_at: chrono::Local::now().naive_local(),
        }
    }

    #[test]
    fn test_assemble_prompt_basic() {
        let messages = assemble_prompt(None, &[], "Bonjour");
        assert_eq!(messages.len(), 2); // system + user
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
        assert_eq!(messages[1].content, "Bonjour");
    }

    #[test]
    fn test_assemble_prompt_with_journal_context() {
        let messages = assemble_prompt(
            Some("Aujourd'hui j'ai eu une journée difficile."),
            &[],
            "Comment gérer mon stress ?",
        );
        assert!(messages[0].content.contains("Contexte du journal"));
        assert!(messages[0].content.contains("journée difficile"));
    }

    #[test]
    fn test_assemble_prompt_with_history() {
        let history = vec![
            make_message("user", "Je me sens triste"),
            make_message("assistant", "Je comprends que tu traverses un moment difficile."),
        ];
        let messages = assemble_prompt(None, &history, "Que faire ?");
        assert_eq!(messages.len(), 4); // system + 2 history + user
        assert_eq!(messages[1].content, "Je me sens triste");
        assert_eq!(messages[3].content, "Que faire ?");
    }

    #[test]
    fn test_estimate_tokens() {
        assert_eq!(estimate_tokens(""), 0);
        assert_eq!(estimate_tokens("abcd"), 1);
        assert_eq!(estimate_tokens("abcde"), 2);
    }

    #[test]
    fn test_manage_context_window_fits() {
        let history = vec![
            make_message("user", "Court message"),
            make_message("assistant", "Réponse courte"),
        ];
        let result = manage_context_window(&history, "Nouveau message");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_manage_context_window_overflow() {
        // Generate a very long history that exceeds the budget
        let long_content = "A".repeat(MAX_CONTEXT_TOKENS * CHARS_PER_TOKEN);
        let history = vec![
            make_message("user", &long_content),
            make_message("assistant", "Réponse à un long message"),
            make_message("user", "Message récent"),
            make_message("assistant", "Réponse récente"),
        ];
        let result = manage_context_window(&history, "Nouveau");

        // Should have a summary + at least the recent messages
        assert!(result.len() < history.len() + 1); // +1 for potential summary
        // The last messages should be the recent ones
        let last = result.last().unwrap();
        assert_eq!(last.content, "Réponse récente");
    }

    #[test]
    fn test_summarize_messages() {
        let messages = vec![
            make_message("user", "Je me sens anxieux"),
            make_message("assistant", "L'anxiété est une émotion naturelle"),
        ];
        let summary = summarize_messages(&messages);
        assert!(summary.contains("Utilisateur"));
        assert!(summary.contains("Assistant"));
        assert!(summary.contains("anxieux"));
    }
}
