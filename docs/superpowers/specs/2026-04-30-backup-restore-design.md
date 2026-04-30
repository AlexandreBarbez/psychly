# Backup & Restore — Design Spec

**Date:** 2026-04-30  
**Status:** Approved

## Problem

Export dialog button does nothing (no native picker, no feedback). Existing export covers journal entries as Markdown only. No full DB backup exists.

## Goals

1. Fix silent `open()` bug in export dialog
2. Keep Markdown export/import (journal entries, human-readable)
3. Add SQLite backup (full DB copy) with user-chosen destination
4. Add SQLite restore (merge strategy — skip existing IDs)
5. One dialog, 4 buttons

## Architecture

### New file: `src-tauri/src/export/sqlite.rs`

Two Tauri commands:

**`backup_db(app: AppHandle, dest_path: String) -> Result<(), String>`**
- Resolve current DB path via `app.path().app_data_dir()` + `"data/psychly.db"`
- `fs::copy(db_path, dest_path)`
- No lock needed — SQLite WAL mode makes file copy safe

**`restore_db(db: State<Arc<Database>>, src_path: String) -> Result<ImportResult, String>`**
- Open `src_path` as separate read-only SQLite connection
- Merge 4 tables in FK order:
  1. `journal_entries` — skip if `id` exists
  2. `chat_sessions` — skip if `id` exists
  3. `chat_messages` — skip if `id` exists
  4. `entry_analyses` — skip if `entry_id` exists
- Return existing `ImportResult` struct (inserted, skipped, errors)

### Modified: `src-tauri/src/export/mod.rs`

Add `pub mod sqlite;`

### Modified: `src-tauri/src/lib.rs`

Register `sqlite::backup_db` and `sqlite::restore_db` in `invoke_handler!`

### Modified: `capabilities/default.json`

Add `dialog:allow-save` permission for native save dialog.

### Modified: `src/api/export.ts`

```ts
export async function backupDb(destPath: string): Promise<void> {
  return invoke("backup_db", { destPath });
}

export async function restoreDb(srcPath: string): Promise<ImportResult> {
  return invoke("restore_db", { srcPath });
}
```

### Modified: `src/components/export-dialog.ts`

**Bug fix:** Replace `typeof result === "string"` with array guard:
```ts
const raw = await open({ directory: true, multiple: false });
const dir = Array.isArray(raw) ? raw[0] : raw;
```
Add `console.error(e)` in all catch blocks alongside UI feedback.

**Layout — 4 buttons:**
```
┌─────────────────────────────────┐
│  Export / Import                │
│                                 │
│  📝 Journal (Markdown)          │
│  [💾 Exporter]  [📥 Importer]  │
│                                 │
│  🗄️ Base de données complète    │
│  [💾 Sauvegarder]  [📥 Restaurer] │
│                                 │
│  [feedback zone]                │
│  [✕ Fermer]                     │
└─────────────────────────────────┘
```

- Export MD → `open({ directory: true })` → `exportJournal(dir)`
- Import MD → `open({ directory: true })` → `importJournal(dir)`
- Backup DB → `save({ defaultPath: "psychly-backup.db", filters: [{name:"DB", extensions:["db"]}] })` → `backupDb(path)`
- Restore DB → `open({ filters: [{name:"DB", extensions:["db"]}] })` → `restoreDb(path)`

## Error Handling

| Event | Message |
|---|---|
| Backup OK | `"Sauvegarde créée."` |
| Restore OK | `"${inserted} entrée(s) restaurée(s), ${skipped} ignorée(s)."` |
| Error | `"Erreur : ${e}"` |
| User cancels dialog | Silent return, no message |

All catch blocks also `console.error(e)` for dev debugging.

## Tests (Rust)

In `export/sqlite.rs`:
- `test_backup_creates_file`
- `test_restore_merges_entries`
- `test_restore_skips_duplicates`
- `test_restore_invalid_path_errors`

Same pattern as existing tests in `export/mod.rs`.

## Files Changed

| File | Change |
|---|---|
| `src-tauri/src/export/sqlite.rs` | NEW |
| `src-tauri/src/export/mod.rs` | +`pub mod sqlite;` |
| `src-tauri/src/lib.rs` | +2 commands in `invoke_handler!` |
| `src-tauri/capabilities/default.json` | +`dialog:allow-save` |
| `src/api/export.ts` | +`backupDb`, +`restoreDb` |
| `src/components/export-dialog.ts` | Bug fix + 4-button layout |
