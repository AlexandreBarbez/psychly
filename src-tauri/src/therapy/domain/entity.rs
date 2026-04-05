use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub journal_entry_id: Option<String>,
    pub created_at: NaiveDateTime,
}

impl ChatSession {
    pub fn new(journal_entry_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            journal_entry_id,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

impl ChatMessage {
    pub fn new(session_id: String, role: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            role,
            content,
            created_at: Local::now().naive_local(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session_has_id() {
        let session = ChatSession::new(None);
        assert!(!session.id.is_empty());
        assert!(session.journal_entry_id.is_none());
    }

    #[test]
    fn test_session_with_entry_id() {
        let session = ChatSession::new(Some("entry-123".to_string()));
        assert_eq!(session.journal_entry_id.as_deref(), Some("entry-123"));
    }

    #[test]
    fn test_new_message_has_fields() {
        let msg = ChatMessage::new("sess-1".to_string(), "user".to_string(), "Bonjour".to_string());
        assert!(!msg.id.is_empty());
        assert_eq!(msg.session_id, "sess-1");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Bonjour");
    }

    #[test]
    fn test_two_messages_have_different_ids() {
        let m1 = ChatMessage::new("s".to_string(), "user".to_string(), "A".to_string());
        let m2 = ChatMessage::new("s".to_string(), "user".to_string(), "B".to_string());
        assert_ne!(m1.id, m2.id);
    }
}
