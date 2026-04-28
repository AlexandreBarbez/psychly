import { invoke } from "@tauri-apps/api/core";

export interface ImportResult {
  inserted: number;
  skipped: number;
  errors: string[];
}

export async function exportJournal(destDir: string): Promise<number> {
  return invoke("export_journal", { destDir });
}

export async function importJournal(srcDir: string): Promise<ImportResult> {
  return invoke("import_journal", { srcDir });
}
