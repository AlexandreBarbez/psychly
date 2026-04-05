import { invoke } from "@tauri-apps/api/core";

export interface EntryResponse {
  id: string;
  body: string;
  preview: string;
  created_at: string;
  updated_at: string;
}

export async function createEntry(body: string): Promise<EntryResponse> {
  return invoke("create_entry", { input: { body } });
}

export async function getEntry(id: string): Promise<EntryResponse | null> {
  return invoke("get_entry", { id });
}

export async function listEntries(
  offset = 0,
  limit = 50
): Promise<EntryResponse[]> {
  return invoke("list_entries", { input: { offset, limit } });
}

export async function updateEntry(
  id: string,
  body: string
): Promise<EntryResponse> {
  return invoke("update_entry", { input: { id, body } });
}

export async function deleteEntry(id: string): Promise<void> {
  return invoke("delete_entry", { id });
}

export async function searchEntries(query: string): Promise<EntryResponse[]> {
  return invoke("search_entries", { query });
}
