use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle, Emitter};

use crate::db::Database;
use crate::therapy::domain::{ChatSession, ChatMessage, ChatSessionRepository};
use crate::therapy::infrastructure::sqlite_repository::SqliteChatSessionRepository;
use crate::therapy::infrastructure::ollama_client::OllamaClient;
use crate::therapy::application::prompt_assembly::assemble_prompt;
use crate::therapy::application::crisis_detection::{detect_crisis, crisis_safety_response};
use crate::analysis::application::context_builder::build_chat_context;

#[derive(Serialize)]
pub struct SessionResponse {
    pub id: String,
    pub journal_entry_id: Option<String>,
    pub created_at: String,
}

impl From<ChatSession> for SessionResponse {
    fn from(s: ChatSession) -> Self {
        Self {
            id: s.id,
            journal_entry_id: s.journal_entry_id,
            created_at: s.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
}

impl From<ChatMessage> for MessageResponse {
    fn from(m: ChatMessage) -> Self {
        Self {
            id: m.id,
            session_id: m.session_id,
            role: m.role,
            content: m.content,
            created_at: m.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ChatStreamEvent {
    pub session_id: String,
    pub chunk: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct StartChatSessionInput {
    pub journal_entry_id: Option<String>,
}

#[derive(Deserialize)]
pub struct SendMessageInput {
    pub session_id: String,
    pub content: String,
    pub journal_context: Option<String>,
}

#[tauri::command]
pub fn start_chat_session(
    db: State<'_, Arc<Database>>,
    input: StartChatSessionInput,
) -> Result<SessionResponse, String> {
    let repo = SqliteChatSessionRepository::new(db.inner().clone());
    let session = ChatSession::new(input.journal_entry_id);
    repo.create_session(&session)?;
    Ok(SessionResponse::from(session))
}

#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    db: State<'_, Arc<Database>>,
    ollama: State<'_, OllamaClient>,
    input: SendMessageInput,
) -> Result<MessageResponse, String> {
    let repo = SqliteChatSessionRepository::new(db.inner().clone());

    // Check crisis detection first
    if detect_crisis(&input.content) {
        let crisis_response = crisis_safety_response();

        // Save user message
        let user_msg = ChatMessage::new(
            input.session_id.clone(),
            "user".to_string(),
            input.content,
        );
        repo.add_message(&user_msg)?;

        // Save crisis response as assistant message
        let assistant_msg = ChatMessage::new(
            input.session_id.clone(),
            "assistant".to_string(),
            crisis_response,
        );
        repo.add_message(&assistant_msg)?;

        return Ok(MessageResponse::from(assistant_msg));
    }

    // Save user message
    let user_msg = ChatMessage::new(
        input.session_id.clone(),
        "user".to_string(),
        input.content.clone(),
    );
    repo.add_message(&user_msg)?;

    // Get conversation history
    let history = repo.get_session_messages(&input.session_id)?;

    // Build context dynamically
    let journal_context = build_chat_context(db.inner().clone()).unwrap_or_default();

    // Assemble prompt (excludes the current user message since it's already in history)
    let history_without_last = &history[..history.len().saturating_sub(1)];
    let prompt_messages = assemble_prompt(
        if journal_context.is_empty() { None } else { Some(&journal_context) },
        history_without_last,
        &input.content,
    );

    // Stream response from Ollama
    let mut rx = ollama.chat_stream(prompt_messages).await?;

    let session_id = input.session_id.clone();
    let mut full_response = String::new();

    while let Some(chunk_result) = rx.recv().await {
        match chunk_result {
            Ok(chunk) => {
                full_response.push_str(&chunk);
                let _ = app.emit("chat-stream", ChatStreamEvent {
                    session_id: session_id.clone(),
                    chunk,
                    done: false,
                });
            }
            Err(e) => {
                // Save a stub assistant message so the user message is not orphaned in the DB.
                // An unpaired user message would pollute future LLM context.
                let error_stub = ChatMessage::new(
                    input.session_id.clone(),
                    "assistant".to_string(),
                    "Désolé, une erreur de connexion est survenue. Veuillez réessayer.".to_string(),
                );
                let _ = repo.add_message(&error_stub);
                return Err(format!("Stream error: {e}"));
            }
        }
    }

    // Emit done event
    let _ = app.emit("chat-stream", ChatStreamEvent {
        session_id: session_id.clone(),
        chunk: String::new(),
        done: true,
    });

    // Save assistant response
    let assistant_msg = ChatMessage::new(
        input.session_id,
        "assistant".to_string(),
        full_response,
    );
    repo.add_message(&assistant_msg)?;

    Ok(MessageResponse::from(assistant_msg))
}

#[tauri::command]
pub fn list_chat_sessions(
    db: State<'_, Arc<Database>>,
) -> Result<Vec<SessionResponse>, String> {
    let repo = SqliteChatSessionRepository::new(db.inner().clone());
    let sessions = repo.list_sessions()?;
    Ok(sessions.into_iter().map(SessionResponse::from).collect())
}

#[tauri::command]
pub fn get_chat_session(
    db: State<'_, Arc<Database>>,
    session_id: String,
) -> Result<(SessionResponse, Vec<MessageResponse>), String> {
    let repo = SqliteChatSessionRepository::new(db.inner().clone());
    let session = repo.get_session(&session_id)?
        .ok_or_else(|| format!("Session not found: {session_id}"))?;
    let messages = repo.get_session_messages(&session_id)?;
    Ok((
        SessionResponse::from(session),
        messages.into_iter().map(MessageResponse::from).collect(),
    ))
}

#[tauri::command]
pub async fn check_ollama_status(
    ollama: State<'_, OllamaClient>,
) -> Result<bool, String> {
    ollama.health_check().await
}
