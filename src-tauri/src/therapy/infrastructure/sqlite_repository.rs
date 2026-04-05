use std::sync::Arc;
use chrono::NaiveDateTime;
use rusqlite::OptionalExtension;

use crate::db::Database;
use crate::therapy::domain::{ChatSession, ChatMessage, ChatSessionRepository};

const DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S%.f";

pub struct SqliteChatSessionRepository {
    db: Arc<Database>,
}

impl SqliteChatSessionRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl ChatSessionRepository for SqliteChatSessionRepository {
    fn create_session(&self, session: &ChatSession) -> Result<(), String> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO chat_sessions (id, journal_entry_id, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                session.id,
                session.journal_entry_id,
                session.created_at.format(DATETIME_FMT).to_string(),
            ],
        )
        .map_err(|e| format!("Failed to create chat session: {e}"))?;
        Ok(())
    }

    fn add_message(&self, message: &ChatMessage) -> Result<(), String> {
        let conn = self.db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO chat_messages (id, session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                message.id,
                message.session_id,
                message.role,
                message.content,
                message.created_at.format(DATETIME_FMT).to_string(),
            ],
        )
        .map_err(|e| format!("Failed to add chat message: {e}"))?;
        Ok(())
    }

    fn get_session(&self, id: &str) -> Result<Option<ChatSession>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, journal_entry_id, created_at FROM chat_sessions WHERE id = ?1")
            .map_err(|e| format!("Failed to prepare query: {e}"))?;

        let result = stmt
            .query_row(rusqlite::params![id], |row| {
                Ok(ChatSession {
                    id: row.get(0)?,
                    journal_entry_id: row.get(1)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(2)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .optional()
            .map_err(|e| format!("Failed to get session: {e}"))?;

        Ok(result)
    }

    fn get_session_messages(&self, session_id: &str) -> Result<Vec<ChatMessage>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, session_id, role, content, created_at FROM chat_messages WHERE session_id = ?1 ORDER BY created_at ASC",
            )
            .map_err(|e| format!("Failed to prepare query: {e}"))?;

        let messages = stmt
            .query_map(rusqlite::params![session_id], |row| {
                Ok(ChatMessage {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    role: row.get(2)?,
                    content: row.get(3)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(4)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .map_err(|e| format!("Failed to query messages: {e}"))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(messages)
    }

    fn list_sessions(&self) -> Result<Vec<ChatSession>, String> {
        let conn = self.db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, journal_entry_id, created_at FROM chat_sessions ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare list query: {e}"))?;

        let sessions = stmt
            .query_map([], |row| {
                Ok(ChatSession {
                    id: row.get(0)?,
                    journal_entry_id: row.get(1)?,
                    created_at: NaiveDateTime::parse_from_str(
                        &row.get::<_, String>(2)?,
                        DATETIME_FMT,
                    )
                    .unwrap(),
                })
            })
            .map_err(|e| format!("Failed to list sessions: {e}"))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(sessions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Arc<Database> {
        Arc::new(Database::open_in_memory().unwrap())
    }

    #[test]
    fn test_create_and_get_session() {
        let db = setup();
        let repo = SqliteChatSessionRepository::new(db);
        let session = ChatSession::new(None);
        repo.create_session(&session).unwrap();

        let found = repo.get_session(&session.id).unwrap().unwrap();
        assert_eq!(found.id, session.id);
        assert!(found.journal_entry_id.is_none());
    }

    #[test]
    fn test_create_session_with_entry_id() {
        let db = setup();
        let repo = SqliteChatSessionRepository::new(Arc::clone(&db));

        // Create a journal entry first (FK constraint)
        {
            let conn = db.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES ('e1', 'test', '2025-01-01T00:00:00.0', '2025-01-01T00:00:00.0')",
                [],
            ).unwrap();
        }

        let session = ChatSession::new(Some("e1".to_string()));
        repo.create_session(&session).unwrap();

        let found = repo.get_session(&session.id).unwrap().unwrap();
        assert_eq!(found.journal_entry_id.as_deref(), Some("e1"));
    }

    #[test]
    fn test_get_nonexistent_session() {
        let db = setup();
        let repo = SqliteChatSessionRepository::new(db);
        let found = repo.get_session("nope").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_add_and_get_messages() {
        let db = setup();
        let repo = SqliteChatSessionRepository::new(db);
        let session = ChatSession::new(None);
        repo.create_session(&session).unwrap();

        let m1 = ChatMessage::new(session.id.clone(), "user".to_string(), "Bonjour".to_string());
        let m2 = ChatMessage::new(session.id.clone(), "assistant".to_string(), "Bonjour, comment allez-vous ?".to_string());
        repo.add_message(&m1).unwrap();
        repo.add_message(&m2).unwrap();

        let messages = repo.get_session_messages(&session.id).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[1].role, "assistant");
    }

    #[test]
    fn test_list_sessions() {
        let db = setup();
        let repo = SqliteChatSessionRepository::new(db);

        let s1 = ChatSession::new(None);
        let s2 = ChatSession::new(None);
        repo.create_session(&s1).unwrap();
        repo.create_session(&s2).unwrap();

        let sessions = repo.list_sessions().unwrap();
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn test_messages_ordered_by_created_at() {
        let db = setup();
        let repo = SqliteChatSessionRepository::new(db);
        let session = ChatSession::new(None);
        repo.create_session(&session).unwrap();

        let m1 = ChatMessage::new(session.id.clone(), "user".to_string(), "Premier".to_string());
        let m2 = ChatMessage::new(session.id.clone(), "assistant".to_string(), "Deuxième".to_string());
        let m3 = ChatMessage::new(session.id.clone(), "user".to_string(), "Troisième".to_string());
        repo.add_message(&m1).unwrap();
        repo.add_message(&m2).unwrap();
        repo.add_message(&m3).unwrap();

        let messages = repo.get_session_messages(&session.id).unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].content, "Premier");
        assert_eq!(messages[2].content, "Troisième");
    }
}
