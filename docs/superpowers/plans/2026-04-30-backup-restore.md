# Backup & Restore Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the silent export button bug, and add full SQLite DB backup/restore alongside the existing Markdown export/import.

**Architecture:** Fix the `open()` array-guard bug in `export-dialog.ts`. Add a new `src-tauri/src/export/sqlite.rs` module with `do_backup` (file copy) and `do_restore` (merge 4 tables in FK order). Wire up 2 new Tauri commands, extend the frontend API, and update the dialog to show 4 buttons.

**Tech Stack:** Rust (rusqlite, tauri v2), TypeScript (Web Components, @tauri-apps/plugin-dialog v2)

---

### Task 1: Add `dialog:allow-save` capability

**Files:**
- Modify: `src-tauri/capabilities/default.json`

- [ ] **Step 1: Add the permission**

Replace the contents of `src-tauri/capabilities/default.json` with:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "enables the default permissions",
  "windows": [
    "main"
  ],
  "permissions": [
    "core:default",
    "dialog:allow-open",
    "dialog:allow-save"
  ]
}
```

- [ ] **Step 2: Commit**

```bash
git add src-tauri/capabilities/default.json
git commit -m "feat: add dialog:allow-save capability for SQLite backup"
```

---

### Task 2: Fix the silent `open()` bug in export-dialog.ts

The `open()` call silently returns null (no native picker) when something goes wrong. Adding `console.error` and guarding the array case surfaces the real error.

**Files:**
- Modify: `src/components/export-dialog.ts`

- [ ] **Step 1: Fix `handleExport` and `handleImport`**

Replace both private methods in `src/components/export-dialog.ts`:

```ts
private async handleExport() {
  try {
    const raw = await open({ directory: true, multiple: false });
    const dir = Array.isArray(raw) ? raw[0] : raw;
    if (!dir) return;
    const count = await exportJournal(dir);
    this.showFeedback(`${count} entrée(s) exportée(s).`);
  } catch (e) {
    console.error("Export Markdown failed:", e);
    this.showFeedback(`Erreur : ${e}`, true);
  }
}

private async handleImport() {
  try {
    const raw = await open({ directory: true, multiple: false });
    const dir = Array.isArray(raw) ? raw[0] : raw;
    if (!dir) return;
    const result2 = await importJournal(dir);
    let msg = `${result2.inserted} importée(s), ${result2.skipped} ignorée(s).`;
    if (result2.errors.length > 0) {
      msg += ` ${result2.errors.length} erreur(s).`;
    }
    this.showFeedback(msg, result2.errors.length > 0);
  } catch (e) {
    console.error("Import Markdown failed:", e);
    this.showFeedback(`Erreur : ${e}`, true);
  }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/export-dialog.ts
git commit -m "fix: guard open() array result and log errors in export-dialog"
```

---

### Task 3: Implement `do_backup` with tests

**Files:**
- Create: `src-tauri/src/export/sqlite.rs`

- [ ] **Step 1: Write the failing tests**

Create `src-tauri/src/export/sqlite.rs` with tests only:

```rust
use crate::db::Database;
use std::path::Path;
use std::sync::Arc;

use super::ImportResult;

pub fn do_backup(src_path: &Path, dest_path: &Path) -> Result<(), String> {
    todo!()
}

pub fn do_restore(db: &Arc<Database>, src_path: &Path) -> Result<ImportResult, String> {
    todo!()
}

#[tauri::command]
pub fn backup_db(app: tauri::AppHandle, dest_path: String) -> Result<(), String> {
    todo!()
}

#[tauri::command]
pub fn restore_db(
    db: tauri::State<'_, Arc<Database>>,
    src_path: String,
) -> Result<ImportResult, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn make_src_db(tmp_dir: &Path) -> std::path::PathBuf {
        let db = Database::open(tmp_dir).unwrap();
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["entry-1", "Hello backup", "2026-04-30T10:00:00", "2026-04-30T10:00:00"],
        ).unwrap();
        drop(conn);
        drop(db);
        tmp_dir.join("data").join("psychly.db")
    }

    #[test]
    fn test_backup_creates_file() {
        let tmp = std::env::temp_dir().join("psychly_backup_test_1");
        std::fs::create_dir_all(&tmp).unwrap();
        let src = make_src_db(&tmp);
        let dest = tmp.join("backup.db");

        let result = do_backup(&src, &dest);
        assert!(result.is_ok(), "backup failed: {:?}", result);
        assert!(dest.exists(), "backup file not created");

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_backup_dest_is_valid_sqlite() {
        let tmp = std::env::temp_dir().join("psychly_backup_test_2");
        std::fs::create_dir_all(&tmp).unwrap();
        let src = make_src_db(&tmp);
        let dest = tmp.join("backup.db");

        do_backup(&src, &dest).unwrap();

        let conn = rusqlite::Connection::open(&dest).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM journal_entries", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_backup_nonexistent_src_returns_error() {
        let dest = std::env::temp_dir().join("psychly_backup_test_3_dest.db");
        let result = do_backup(Path::new("/nonexistent/path/psychly.db"), &dest);
        assert!(result.is_err());
    }
}
```

- [ ] **Step 2: Add `sqlite` mod to `export/mod.rs`**

Add at the top of `src-tauri/src/export/mod.rs`:

```rust
pub mod sqlite;
```

- [ ] **Step 3: Run tests — verify they fail with `todo!()`**

```bash
cd src-tauri && cargo test export::sqlite::tests 2>&1 | head -40
```

Expected: tests panic with `not yet implemented`.

- [ ] **Step 4: Implement `do_backup`**

Replace `do_backup` in `src-tauri/src/export/sqlite.rs`:

```rust
pub fn do_backup(src_path: &Path, dest_path: &Path) -> Result<(), String> {
    if !src_path.exists() {
        return Err(format!("Source DB not found: {}", src_path.display()));
    }
    std::fs::copy(src_path, dest_path)
        .map_err(|e| format!("Backup failed: {e}"))?;
    Ok(())
}
```

- [ ] **Step 5: Run backup tests — verify they pass**

```bash
cd src-tauri && cargo test export::sqlite::tests::test_backup 2>&1
```

Expected: `test_backup_creates_file ... ok`, `test_backup_dest_is_valid_sqlite ... ok`, `test_backup_nonexistent_src_returns_error ... ok`

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/export/sqlite.rs src-tauri/src/export/mod.rs
git commit -m "feat: add do_backup to sqlite export module"
```

---

### Task 4: Implement `do_restore` with tests

**Files:**
- Modify: `src-tauri/src/export/sqlite.rs`

- [ ] **Step 1: Add restore tests to `sqlite.rs`**

Append inside the `#[cfg(test)] mod tests` block:

```rust
    fn make_src_db_full(tmp_dir: &Path) -> std::path::PathBuf {
        let db = Database::open(tmp_dir).unwrap();
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["entry-restore-1", "Restore body", "2026-04-30T10:00:00", "2026-04-30T10:00:00"],
        ).unwrap();
        conn.execute(
            "INSERT INTO chat_sessions (id, journal_entry_id, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params!["session-restore-1", "entry-restore-1", "2026-04-30T10:01:00"],
        ).unwrap();
        conn.execute(
            "INSERT INTO chat_messages (id, session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params!["msg-restore-1", "session-restore-1", "user", "Hello", "2026-04-30T10:02:00"],
        ).unwrap();
        conn.execute(
            "INSERT INTO entry_analyses (id, entry_id, emotional_tone, themes, patterns, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["analysis-restore-1", "entry-restore-1", "positive", "joy", "growth", "2026-04-30T10:03:00"],
        ).unwrap();
        drop(conn);
        drop(db);
        tmp_dir.join("data").join("psychly.db")
    }

    #[test]
    fn test_restore_merges_all_tables() {
        let tmp_src = std::env::temp_dir().join("psychly_restore_src_1");
        std::fs::create_dir_all(&tmp_src).unwrap();
        let src_path = make_src_db_full(&tmp_src);

        let dst_db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_restore(&dst_db, &src_path).unwrap();

        assert_eq!(result.inserted, 4, "expected 4 rows inserted (entry+session+message+analysis)");
        assert_eq!(result.skipped, 0);
        assert!(result.errors.is_empty(), "unexpected errors: {:?}", result.errors);

        let conn = dst_db.conn.lock().unwrap();
        let body: String = conn
            .query_row("SELECT body FROM journal_entries WHERE id = ?1", ["entry-restore-1"], |r| r.get(0))
            .unwrap();
        assert_eq!(body, "Restore body");

        std::fs::remove_dir_all(&tmp_src).unwrap();
    }

    #[test]
    fn test_restore_skips_existing_journal_entry() {
        let tmp_src = std::env::temp_dir().join("psychly_restore_src_2");
        std::fs::create_dir_all(&tmp_src).unwrap();
        let src_path = make_src_db(&tmp_src);

        let dst_db = Arc::new(Database::open_in_memory().unwrap());
        // Pre-insert the same entry
        {
            let conn = dst_db.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params!["entry-1", "Pre-existing", "2026-04-30T10:00:00", "2026-04-30T10:00:00"],
            ).unwrap();
        }

        let result = do_restore(&dst_db, &src_path).unwrap();
        assert_eq!(result.inserted, 0);
        assert_eq!(result.skipped, 1);
        assert!(result.errors.is_empty());

        std::fs::remove_dir_all(&tmp_src).unwrap();
    }

    #[test]
    fn test_restore_nonexistent_src_returns_error() {
        let dst_db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_restore(&dst_db, Path::new("/nonexistent/backup.db"));
        assert!(result.is_err());
    }
```

- [ ] **Step 2: Run restore tests — verify they fail with `todo!()`**

```bash
cd src-tauri && cargo test export::sqlite::tests::test_restore 2>&1 | head -40
```

Expected: panic `not yet implemented`.

- [ ] **Step 3: Implement `do_restore`**

Replace `do_restore` in `src-tauri/src/export/sqlite.rs`:

```rust
pub fn do_restore(db: &Arc<Database>, src_path: &Path) -> Result<ImportResult, String> {
    if !src_path.exists() {
        return Err(format!("Backup file not found: {}", src_path.display()));
    }

    let src_conn = rusqlite::Connection::open_with_flags(
        src_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .map_err(|e| format!("Cannot open backup: {e}"))?;

    let mut result = ImportResult { inserted: 0, skipped: 0, errors: vec![] };

    // 1. journal_entries (no FK dependencies)
    let journal_rows: Vec<(String, String, String, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, body, created_at, updated_at FROM journal_entries")
            .map_err(|e| format!("Cannot read journal_entries: {e}"))?;
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
            .map_err(|e| format!("journal_entries query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect()
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, body, created_at, updated_at) in journal_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM journal_entries WHERE id = ?1", [&id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, body, created_at, updated_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("journal_entries/{id}: {e}")),
            }
        }
    }

    // 2. chat_sessions (FK → journal_entries)
    let session_rows: Vec<(String, Option<String>, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, journal_entry_id, created_at FROM chat_sessions")
            .map_err(|e| format!("Cannot read chat_sessions: {e}"))?;
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .map_err(|e| format!("chat_sessions query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect()
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, journal_entry_id, created_at) in session_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM chat_sessions WHERE id = ?1", [&id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO chat_sessions (id, journal_entry_id, created_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![id, journal_entry_id, created_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("chat_sessions/{id}: {e}")),
            }
        }
    }

    // 3. chat_messages (FK → chat_sessions)
    let message_rows: Vec<(String, String, String, String, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, session_id, role, content, created_at FROM chat_messages")
            .map_err(|e| format!("Cannot read chat_messages: {e}"))?;
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)))
            .map_err(|e| format!("chat_messages query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect()
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, session_id, role, content, created_at) in message_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM chat_messages WHERE id = ?1", [&id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO chat_messages (id, session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, session_id, role, content, created_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("chat_messages/{id}: {e}")),
            }
        }
    }

    // 4. entry_analyses (FK → journal_entries)
    let analysis_rows: Vec<(String, String, String, String, String, String)> = {
        let mut stmt = src_conn
            .prepare("SELECT id, entry_id, emotional_tone, themes, patterns, created_at FROM entry_analyses")
            .map_err(|e| format!("Cannot read entry_analyses: {e}"))?;
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)))
            .map_err(|e| format!("entry_analyses query failed: {e}"))?
            .filter_map(|r| r.ok())
            .collect()
    };
    {
        let conn = db.conn.lock().unwrap();
        for (id, entry_id, emotional_tone, themes, patterns, created_at) in analysis_rows {
            let exists: bool = conn
                .query_row("SELECT 1 FROM entry_analyses WHERE entry_id = ?1", [&entry_id], |_| Ok(true))
                .unwrap_or(false);
            if exists {
                result.skipped += 1;
                continue;
            }
            match conn.execute(
                "INSERT INTO entry_analyses (id, entry_id, emotional_tone, themes, patterns, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![id, entry_id, emotional_tone, themes, patterns, created_at],
            ) {
                Ok(_) => result.inserted += 1,
                Err(e) => result.errors.push(format!("entry_analyses/{id}: {e}")),
            }
        }
    }

    Ok(result)
}
```

- [ ] **Step 4: Implement Tauri command wrappers**

Replace the two `todo!()` command stubs in `sqlite.rs`:

```rust
use crate::db::Database;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

use super::ImportResult;

#[tauri::command]
pub fn backup_db(app: AppHandle, dest_path: String) -> Result<(), String> {
    let app_root = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Cannot resolve resource dir: {e}"))?;
    let db_path = crate::db::Database::resolve_path(&app_root);
    do_backup(&db_path, Path::new(&dest_path))
}

#[tauri::command]
pub fn restore_db(
    db: State<'_, Arc<Database>>,
    src_path: String,
) -> Result<ImportResult, String> {
    do_restore(db.inner(), Path::new(&src_path))
}
```

Make sure the imports at the top of `sqlite.rs` are:

```rust
use crate::db::Database;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

use super::ImportResult;
```

- [ ] **Step 5: Run all sqlite tests**

```bash
cd src-tauri && cargo test export::sqlite 2>&1
```

Expected: all 6 tests pass (`test_backup_creates_file`, `test_backup_dest_is_valid_sqlite`, `test_backup_nonexistent_src_returns_error`, `test_restore_merges_all_tables`, `test_restore_skips_existing_journal_entry`, `test_restore_nonexistent_src_returns_error`)

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/export/sqlite.rs
git commit -m "feat: implement do_restore — merge all 4 tables from SQLite backup"
```

---

### Task 5: Register new commands in lib.rs

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add commands to `invoke_handler!`**

In `src-tauri/src/lib.rs`, find the `invoke_handler` block and add the two new commands:

```rust
.invoke_handler(tauri::generate_handler![
  // ... existing commands ...
  export::export_journal,
  export::import_journal,
  export::sqlite::backup_db,      // ADD
  export::sqlite::restore_db,     // ADD
])
```

- [ ] **Step 2: Build to verify no compile errors**

```bash
cd src-tauri && cargo build 2>&1 | grep -E "error|warning\[" | head -20
```

Expected: no errors. Warnings about unused imports are acceptable.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: register backup_db and restore_db tauri commands"
```

---

### Task 6: Extend frontend API

**Files:**
- Modify: `src/api/export.ts`

- [ ] **Step 1: Add `backupDb` and `restoreDb`**

Replace the full contents of `src/api/export.ts`:

```ts
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

export async function backupDb(destPath: string): Promise<void> {
  return invoke("backup_db", { destPath });
}

export async function restoreDb(srcPath: string): Promise<ImportResult> {
  return invoke("restore_db", { srcPath });
}
```

- [ ] **Step 2: Commit**

```bash
git add src/api/export.ts
git commit -m "feat: add backupDb and restoreDb to frontend export API"
```

---

### Task 7: Update export-dialog with 4 buttons

**Files:**
- Modify: `src/components/export-dialog.ts`

- [ ] **Step 1: Replace the full file**

```ts
import { open, save } from "@tauri-apps/plugin-dialog";
import { exportJournal, importJournal, backupDb, restoreDb } from "../api/export";

export class ExportDialog extends HTMLElement {
  connectedCallback() {
    this.render();
  }

  private render() {
    this.innerHTML = `
      <div class="export-overlay">
        <div class="export-dialog">
          <h2>Export / Import</h2>

          <section class="export-section">
            <h3>📝 Journal (Markdown)</h3>
            <p>Exportez vos entrées vers des fichiers Markdown lisibles, ou importez depuis un dossier existant.</p>
            <div class="export-actions">
              <button class="btn-primary" id="btn-export-md">💾 Exporter</button>
              <button class="btn-secondary" id="btn-import-md">📥 Importer</button>
            </div>
          </section>

          <section class="export-section">
            <h3>🗄️ Base de données complète</h3>
            <p>Sauvegardez ou restaurez l'ensemble de vos données (journal, thérapie, analyses).</p>
            <div class="export-actions">
              <button class="btn-primary" id="btn-backup-db">💾 Sauvegarder</button>
              <button class="btn-secondary" id="btn-restore-db">📥 Restaurer</button>
            </div>
          </section>

          <div id="export-feedback" class="export-feedback"></div>
          <button class="btn-close" id="btn-close">✕ Fermer</button>
        </div>
      </div>
    `;

    this.querySelector("#btn-close")?.addEventListener("click", () => this.remove());
    this.querySelector("#btn-export-md")?.addEventListener("click", () => this.handleExportMd());
    this.querySelector("#btn-import-md")?.addEventListener("click", () => this.handleImportMd());
    this.querySelector("#btn-backup-db")?.addEventListener("click", () => this.handleBackupDb());
    this.querySelector("#btn-restore-db")?.addEventListener("click", () => this.handleRestoreDb());
  }

  private showFeedback(msg: string, isError = false) {
    const el = this.querySelector("#export-feedback");
    if (el) {
      el.textContent = msg;
      el.className = `export-feedback ${isError ? "feedback-error" : "feedback-success"}`;
    }
  }

  private async handleExportMd() {
    try {
      const raw = await open({ directory: true, multiple: false });
      const dir = Array.isArray(raw) ? raw[0] : raw;
      if (!dir) return;
      const count = await exportJournal(dir);
      this.showFeedback(`${count} entrée(s) exportée(s).`);
    } catch (e) {
      console.error("Export Markdown failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }

  private async handleImportMd() {
    try {
      const raw = await open({ directory: true, multiple: false });
      const dir = Array.isArray(raw) ? raw[0] : raw;
      if (!dir) return;
      const result = await importJournal(dir);
      let msg = `${result.inserted} importée(s), ${result.skipped} ignorée(s).`;
      if (result.errors.length > 0) msg += ` ${result.errors.length} erreur(s).`;
      this.showFeedback(msg, result.errors.length > 0);
    } catch (e) {
      console.error("Import Markdown failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }

  private async handleBackupDb() {
    try {
      const dest = await save({
        defaultPath: "psychly-backup.db",
        filters: [{ name: "SQLite Database", extensions: ["db"] }],
      });
      if (!dest) return;
      await backupDb(dest);
      this.showFeedback("Sauvegarde créée.");
    } catch (e) {
      console.error("DB backup failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }

  private async handleRestoreDb() {
    try {
      const raw = await open({
        multiple: false,
        filters: [{ name: "SQLite Database", extensions: ["db"] }],
      });
      const src = Array.isArray(raw) ? raw[0] : raw;
      if (!src) return;
      const result = await restoreDb(src);
      let msg = `${result.inserted} entrée(s) restaurée(s), ${result.skipped} ignorée(s).`;
      if (result.errors.length > 0) msg += ` ${result.errors.length} erreur(s).`;
      this.showFeedback(msg, result.errors.length > 0);
    } catch (e) {
      console.error("DB restore failed:", e);
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }
}

customElements.define("export-dialog", ExportDialog);
```

- [ ] **Step 2: Type-check**

```bash
npm run build 2>&1 | grep -E "error TS|Error" | head -20
```

Expected: no TypeScript errors.

- [ ] **Step 3: Commit**

```bash
git add src/components/export-dialog.ts
git commit -m "feat: add 4-button export dialog with SQLite backup/restore"
```

---

### Task 8: Smoke test in dev

- [ ] **Step 1: Start dev server**

```bash
npm run tauri dev
```

Ollama must be running. If not available, start it or skip therapy-related actions.

- [ ] **Step 2: Test Markdown export**

Open export dialog → click "Exporter" (Markdown) → native folder picker appears → pick a folder → feedback shows count.

- [ ] **Step 3: Test SQLite backup**

Click "Sauvegarder" → native save dialog appears with default name `psychly-backup.db` → pick location → feedback shows "Sauvegarde créée."

- [ ] **Step 4: Test SQLite restore**

Click "Restaurer" → native open dialog with `.db` filter → pick the backup created in Step 3 → feedback shows inserted/skipped counts.

- [ ] **Step 5: Test cancel behavior**

Cancel each dialog → no feedback message shown (silent return).

- [ ] **Step 6: If any button still does nothing**

Open browser devtools (Tauri dev window → right-click → Inspect) → check Console for errors logged by `console.error`. The error will identify the root cause.
