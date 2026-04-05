import { invoke } from "@tauri-apps/api/core";

export interface SessionResponse {
  id: string;
  journal_entry_id: string | null;
  created_at: string;
}

export interface MessageResponse {
  id: string;
  session_id: string;
  role: string;
  content: string;
  created_at: string;
}

export interface ChatStreamEvent {
  session_id: string;
  chunk: string;
  done: boolean;
}

export async function startChatSession(
  journalEntryId?: string
): Promise<SessionResponse> {
  return invoke("start_chat_session", {
    input: { journal_entry_id: journalEntryId ?? null },
  });
}

export async function sendMessage(
  sessionId: string,
  content: string,
  journalContext?: string
): Promise<MessageResponse> {
  return invoke("send_message", {
    input: {
      session_id: sessionId,
      content,
      journal_context: journalContext ?? null,
    },
  });
}

export async function listChatSessions(): Promise<SessionResponse[]> {
  return invoke("list_chat_sessions");
}

export async function getChatSession(
  sessionId: string
): Promise<[SessionResponse, MessageResponse[]]> {
  return invoke("get_chat_session", { sessionId });
}
