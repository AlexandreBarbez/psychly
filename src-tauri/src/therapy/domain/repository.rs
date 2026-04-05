use super::entity::{ChatSession, ChatMessage};

pub trait ChatSessionRepository {
    fn create_session(&self, session: &ChatSession) -> Result<(), String>;
    fn add_message(&self, message: &ChatMessage) -> Result<(), String>;
    fn get_session(&self, id: &str) -> Result<Option<ChatSession>, String>;
    fn get_session_messages(&self, session_id: &str) -> Result<Vec<ChatMessage>, String>;
    fn list_sessions(&self) -> Result<Vec<ChatSession>, String>;
}
