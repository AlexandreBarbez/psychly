# Export / Import Journal Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add export/import of journal entries as a folder of Markdown files, accessible via a nav-bar button.

**Architecture:** New `src-tauri/src/export/mod.rs` module exposes two Tauri commands (`export_journal`, `import_journal`) with testable pure-function helpers. Frontend adds an `export-dialog` Web Component wired into `app-shell`.

**Tech Stack:** Rust (rusqlite, chrono), Tauri v2, `tauri-plugin-dialog` for directory picker, Vanilla TypeScript Web Components.

---

## File Map

| Action | Path | Responsibility |
|--------|------|---------------|
| Create | `src-tauri/src/export/mod.rs` | Export/import logic, Tauri commands, unit tests |
| Modify | `src-tauri/src/lib.rs` | `pub mod export;`, plugin init, register 2 commands |
| Modify | `src-tauri/Cargo.toml` | Add `tauri-plugin-dialog = "2"` |
| Modify | `src-tauri/capabilities/default.json` | Add `dialog:allow-open` permission |
| Modify | `package.json` | Add `@tauri-apps/plugin-dialog` |
| Create | `src/api/export.ts` | `invoke()` wrappers for export/import commands |
| Create | `src/components/export-dialog.ts` | Modal Web Component with export/import UI |
| Modify | `src/main.ts` | Import `export-dialog` component |
| Modify | `src/components/app-shell.ts` | Add nav button, open dialog on click |

---

## Task 1: Add plugin-dialog dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `package.json`
- Modify: `src-tauri/capabilities/default.json`

- [ ] **Step 1: Add Rust dependency**

In `src-tauri/Cargo.toml`, add to `[dependencies]`:

```toml
tauri-plugin-dialog = "2"
```

- [ ] **Step 2: Add JS dependency**

In `package.json`, add to `"dependencies"`:

```json
"@tauri-apps/plugin-dialog": "^2"
```

- [ ] **Step 3: Add capability permission**

In `src-tauri/capabilities/default.json`, the `"permissions"` array becomes:

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
    "dialog:allow-open"
  ]
}
```

- [ ] **Step 4: Install JS deps**

```bash
npm install
```

- [ ] **Step 5: Verify Rust compiles**

```bash
cd src-tauri && cargo check
```

Expected: no errors (plugin-dialog fetched and compiled).

- [ ] **Step 6: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/capabilities/default.json
git commit -m "chore: add tauri-plugin-dialog dependency"
```

---

## Task 2: Implement Rust export/import module (TDD)

**Files:**
- Create: `src-tauri/src/export/mod.rs`

- [ ] **Step 1: Create the file with failing tests**

Create `src-tauri/src/export/mod.rs`:

```rust
use std::path::Path;
use std::sync::Arc;
use serde::Serialize;
use tauri::State;
use chrono::NaiveDateTime;

use crate::db::Database;

const DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S%.f";

#[derive(Debug, Serialize, Clone)]
pub struct ImportResult {
    pub inserted: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

fn parse_markdown_entry(content: &str) -> Option<(String, String, String, String)> {
    let content = content.trim();
    let after_first = content.strip_prefix("---\n")?;
    let end_pos = after_first.find("\n---")?;
    let frontmatter = &after_first[..end_pos];
    let rest = &after_first[end_pos + 4..];
    let body = rest.trim_start_matches('\n').trim().to_string();

    let mut id = None;
    let mut created_at = None;
    let mut updated_at = None;

    for line in frontmatter.lines() {
        if let Some(v) = line.strip_prefix("id: ") {
            id = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("created_at: ") {
            created_at = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("updated_at: ") {
            updated_at = Some(v.trim().to_string());
        }
    }

    Some((id?, created_at?, updated_at.unwrap_or_default(), body))
}

pub fn do_export(db: &Arc<Database>, dest_dir: &Path) -> Result<usize, String> {
    todo!()
}

pub fn do_import(db: &Arc<Database>, src_dir: &Path) -> Result<ImportResult, String> {
    todo!()
}

#[tauri::command]
pub fn export_journal(db: State<'_, Arc<Database>>, dest_dir: String) -> Result<usize, String> {
    do_export(db.inner(), Path::new(&dest_dir))
}

#[tauri::command]
pub fn import_journal(db: State<'_, Arc<Database>>, src_dir: String) -> Result<ImportResult, String> {
    do_import(db.inner(), Path::new(&src_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db_with_entry() -> Arc<Database> {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
                "Hello world",
                "2026-04-28T10:00:00.0",
                "2026-04-28T10:00:00.0"
            ],
        ).unwrap();
        drop(conn);
        db
    }

    #[test]
    fn test_export_creates_markdown_file() {
        let db = db_with_entry();
        let tmp = std::env::temp_dir().join("psychly_export_test_1");
        std::fs::create_dir_all(&tmp).unwrap();

        let count = do_export(&db, &tmp).unwrap();
        assert_eq!(count, 1);

        let file = tmp.join("2026-04-28_aaaaaaaa.md");
        assert!(file.exists(), "Expected file {:?} to exist", file);
        let content = std::fs::read_to_string(&file).unwrap();
        assert!(content.starts_with("---\n"));
        assert!(content.contains("id: aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"));
        assert!(content.contains("Hello world"));

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_export_invalid_dir_returns_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_export(&db, Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_export_empty_db_returns_zero() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_export_test_empty");
        std::fs::create_dir_all(&tmp).unwrap();

        let count = do_export(&db, &tmp).unwrap();
        assert_eq!(count, 0);

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_inserts_entry() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_import_test_1");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(
            tmp.join("2026-04-28_aaaaaaaa.md"),
            "---\nid: aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee\ncreated_at: 2026-04-28T10:00:00\nupdated_at: 2026-04-28T10:00:00\n---\n\nHello world",
        ).unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 1);
        assert_eq!(result.skipped, 0);
        assert!(result.errors.is_empty());

        let conn = db.conn.lock().unwrap();
        let body: String = conn.query_row(
            "SELECT body FROM journal_entries WHERE id = ?1",
            ["aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(body, "Hello world");

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_skips_duplicate_id() {
        let db = db_with_entry();
        let tmp = std::env::temp_dir().join("psychly_import_test_2");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(
            tmp.join("2026-04-28_aaaaaaaa.md"),
            "---\nid: aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee\ncreated_at: 2026-04-28T10:00:00\nupdated_at: 2026-04-28T10:00:00\n---\n\nHello world",
        ).unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 0);
        assert_eq!(result.skipped, 1);
        assert!(result.errors.is_empty());

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_invalid_frontmatter_adds_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_import_test_3");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(tmp.join("bad.md"), "No frontmatter here at all").unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 0);
        assert!(!result.errors.is_empty());

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_missing_id_adds_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let tmp = std::env::temp_dir().join("psychly_import_test_4");
        std::fs::create_dir_all(&tmp).unwrap();

        std::fs::write(
            tmp.join("no_id.md"),
            "---\ncreated_at: 2026-04-28T10:00:00\n---\n\nBody without id",
        ).unwrap();

        let result = do_import(&db, &tmp).unwrap();
        assert_eq!(result.inserted, 0);
        assert!(!result.errors.is_empty());

        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn test_import_nonexistent_dir_returns_error() {
        let db = Arc::new(Database::open_in_memory().unwrap());
        let result = do_import(&db, Path::new("/nonexistent/path/xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_markdown_entry_valid() {
        let content = "---\nid: test-uuid\ncreated_at: 2026-04-28T10:00:00\nupdated_at: 2026-04-28T10:30:00\n---\n\nMon journal";
        let result = parse_markdown_entry(content);
        assert!(result.is_some());
        let (id, created_at, updated_at, body) = result.unwrap();
        assert_eq!(id, "test-uuid");
        assert_eq!(created_at, "2026-04-28T10:00:00");
        assert_eq!(updated_at, "2026-04-28T10:30:00");
        assert_eq!(body, "Mon journal");
    }

    #[test]
    fn test_parse_markdown_entry_no_frontmatter() {
        let result = parse_markdown_entry("No frontmatter");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_markdown_entry_missing_id() {
        let result = parse_markdown_entry("---\ncreated_at: 2026-04-28T10:00:00\n---\n\nBody");
        assert!(result.is_none());
    }
}
```

- [ ] **Step 2: Add `pub mod export;` to lib.rs so it compiles**

In `src-tauri/src/lib.rs`, add after `pub mod analysis;`:

```rust
pub mod export;
```

- [ ] **Step 3: Run tests to confirm they fail (todo!() panics)**

```bash
cd src-tauri && cargo test export
```

Expected: FAILED — `not yet implemented`

- [ ] **Step 4: Implement `do_export`**

Replace `pub fn do_export(db: &Arc<Database>, dest_dir: &Path) -> Result<usize, String> { todo!() }` with:

```rust
pub fn do_export(db: &Arc<Database>, dest_dir: &Path) -> Result<usize, String> {
    if !dest_dir.is_dir() {
        return Err(format!("Not a directory: {}", dest_dir.display()));
    }

    let entries: Vec<(String, String, String, String)> = {
        let conn = db.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, body, created_at, updated_at FROM journal_entries ORDER BY created_at ASC",
            )
            .map_err(|e| format!("DB prepare error: {e}"))?;

        stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| format!("DB query error: {e}"))?
        .filter_map(|r| r.ok())
        .collect()
    };

    let mut written = 0;
    for (id, body, created_at_str, updated_at_str) in &entries {
        let created_at = NaiveDateTime::parse_from_str(created_at_str, DATETIME_FMT)
            .map_err(|e| format!("Invalid date in entry {id}: {e}"))?;
        let updated_at = NaiveDateTime::parse_from_str(updated_at_str, DATETIME_FMT)
            .unwrap_or(created_at);

        let filename = format!(
            "{}_{}.md",
            created_at.format("%Y-%m-%d"),
            &id[..8.min(id.len())]
        );
        let content = format!(
            "---\nid: {id}\ncreated_at: {}\nupdated_at: {}\n---\n\n{}",
            created_at.format("%Y-%m-%dT%H:%M:%S"),
            updated_at.format("%Y-%m-%dT%H:%M:%S"),
            body
        );

        std::fs::write(dest_dir.join(&filename), &content)
            .map_err(|e| format!("Write error for {filename}: {e}"))?;
        written += 1;
    }

    Ok(written)
}
```

- [ ] **Step 5: Implement `do_import`**

Replace `pub fn do_import(db: &Arc<Database>, src_dir: &Path) -> Result<ImportResult, String> { todo!() }` with:

```rust
pub fn do_import(db: &Arc<Database>, src_dir: &Path) -> Result<ImportResult, String> {
    if !src_dir.is_dir() {
        return Err(format!("Not a directory: {}", src_dir.display()));
    }

    let mut result = ImportResult { inserted: 0, skipped: 0, errors: vec![] };

    let md_files: Vec<_> = std::fs::read_dir(src_dir)
        .map_err(|e| format!("Cannot read directory: {e}"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("md"))
        .collect();

    let conn = db.conn.lock().unwrap();

    for dir_entry in md_files {
        let path = dir_entry.path();
        let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                result.errors.push(format!("{filename}: {e}"));
                continue;
            }
        };

        let (id, created_at_str, updated_at_str, body) = match parse_markdown_entry(&content) {
            Some(v) => v,
            None => {
                result.errors.push(format!("{filename}: invalid frontmatter"));
                continue;
            }
        };

        let exists: bool = conn
            .query_row("SELECT 1 FROM journal_entries WHERE id = ?1", [&id], |_| Ok(true))
            .unwrap_or(false);

        if exists {
            result.skipped += 1;
            continue;
        }

        let updated_at = if updated_at_str.is_empty() { created_at_str.clone() } else { updated_at_str };

        match conn.execute(
            "INSERT INTO journal_entries (id, body, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, body, created_at_str, updated_at],
        ) {
            Ok(_) => result.inserted += 1,
            Err(e) => result.errors.push(format!("{filename}: insert failed: {e}")),
        }
    }

    Ok(result)
}
```

- [ ] **Step 6: Run tests and confirm they pass**

```bash
cd src-tauri && cargo test export
```

Expected: all 10 tests PASS.

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/export/mod.rs
git commit -m "feat: add export/import journal Rust module"
```

---

## Task 3: Register commands in lib.rs and init plugin

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Add plugin init and register commands**

In `src-tauri/src/lib.rs`, add `.plugin(tauri_plugin_dialog::init())` before `.setup(...)`, and add the two export commands to `invoke_handler`. Full updated `lib.rs`:

```rust
pub mod db;
pub mod journal;
pub mod therapy;
pub mod analysis;
pub mod export;

use std::sync::Arc;
use tauri::Manager;

use therapy::infrastructure::ollama_client::OllamaClient;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      let app_root = app.path().resource_dir()
        .unwrap_or_else(|_| std::env::current_dir().unwrap());
      let database = Arc::new(
        db::Database::open(&app_root)
          .expect("Failed to initialize database"),
      );
      app.manage(database);

      let ollama = OllamaClient::new("qwen2.5:14b-instruct-q5_K_M".to_string());
      app.manage(ollama);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      journal::application::commands::create_entry,
      journal::application::commands::get_entry,
      journal::application::commands::list_entries,
      journal::application::commands::update_entry,
      journal::application::commands::delete_entry,
      journal::application::commands::search_entries,
      therapy::application::commands::start_chat_session,
      therapy::application::commands::send_message,
      therapy::application::commands::list_chat_sessions,
      therapy::application::commands::get_chat_session,
      therapy::application::commands::check_ollama_status,
      analysis::application::commands::get_entry_analysis,
      export::export_journal,
      export::import_journal,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

- [ ] **Step 2: Verify full test suite passes**

```bash
cd src-tauri && cargo test
```

Expected: all tests PASS.

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: register export/import commands and dialog plugin"
```

---

## Task 4: Frontend API wrappers

**Files:**
- Create: `src/api/export.ts`

- [ ] **Step 1: Create API wrapper file**

Create `src/api/export.ts`:

```typescript
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
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
npm run build
```

Expected: build succeeds, no TypeScript errors.

- [ ] **Step 3: Commit**

```bash
git add src/api/export.ts
git commit -m "feat: add export/import API wrappers"
```

---

## Task 5: Export dialog Web Component

**Files:**
- Create: `src/components/export-dialog.ts`
- Modify: `src/main.ts`

- [ ] **Step 1: Create the component**

Create `src/components/export-dialog.ts`:

```typescript
import { open } from "@tauri-apps/plugin-dialog";
import { exportJournal, importJournal } from "../api/export";

export class ExportDialog extends HTMLElement {
  connectedCallback() {
    this.render();
  }

  private render() {
    this.innerHTML = `
      <div class="export-overlay">
        <div class="export-dialog">
          <h2>Export / Import</h2>
          <p>Exportez vos entrées vers un dossier de fichiers Markdown lisibles, ou importez depuis un export précédent.</p>
          <div class="export-actions">
            <button class="btn-primary" id="btn-export">💾 Exporter vers un dossier</button>
            <button class="btn-secondary" id="btn-import">📥 Importer depuis un dossier</button>
          </div>
          <div id="export-feedback" class="export-feedback"></div>
          <button class="btn-close" id="btn-close">✕ Fermer</button>
        </div>
      </div>
    `;

    this.querySelector("#btn-close")?.addEventListener("click", () => this.remove());
    this.querySelector("#btn-export")?.addEventListener("click", () => this.handleExport());
    this.querySelector("#btn-import")?.addEventListener("click", () => this.handleImport());
  }

  private showFeedback(msg: string, isError = false) {
    const el = this.querySelector("#export-feedback");
    if (el) {
      el.textContent = msg;
      el.className = `export-feedback ${isError ? "feedback-error" : "feedback-success"}`;
    }
  }

  private async handleExport() {
    const dir = await open({ directory: true, multiple: false }) as string | null;
    if (!dir) return;
    try {
      const count = await exportJournal(dir);
      this.showFeedback(`${count} entrée(s) exportée(s).`);
    } catch (e) {
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }

  private async handleImport() {
    const dir = await open({ directory: true, multiple: false }) as string | null;
    if (!dir) return;
    try {
      const result = await importJournal(dir);
      let msg = `${result.inserted} importée(s), ${result.skipped} ignorée(s).`;
      if (result.errors.length > 0) {
        msg += ` ${result.errors.length} erreur(s).`;
      }
      this.showFeedback(msg, result.errors.length > 0);
    } catch (e) {
      this.showFeedback(`Erreur : ${e}`, true);
    }
  }
}

customElements.define("export-dialog", ExportDialog);
```

- [ ] **Step 2: Register component in main.ts**

In `src/main.ts`, add after the last existing import:

```typescript
import "./components/export-dialog";
```

Full `src/main.ts`:

```typescript
// Psychly - Main entry point
import "./components/app-shell";
import "./components/journal-list";
import "./components/journal-editor";
import "./components/journal-entry-view";
import "./components/journal-search";
import "./components/chat-view";
import "./components/chat-session-list";
import "./components/disclaimer-dialog";
import "./components/export-dialog";
```

- [ ] **Step 3: Verify TypeScript compiles**

```bash
npm run build
```

Expected: build succeeds.

- [ ] **Step 4: Commit**

```bash
git add src/components/export-dialog.ts src/main.ts
git commit -m "feat: add export-dialog component"
```

---

## Task 6: Wire export button into app-shell

**Files:**
- Modify: `src/components/app-shell.ts`

- [ ] **Step 1: Add nav button and click handler**

In `src/components/app-shell.ts`, inside the `render()` method:

1. In the `nav-links` div, add the export button after `#nav-chat-history`:

```html
<button class="nav-btn" id="nav-export">💾 Export</button>
```

2. After the existing click handler for `#nav-chat-history`, add:

```typescript
this.querySelector("#nav-export")?.addEventListener("click", () => {
  const dialog = document.createElement("export-dialog");
  document.body.appendChild(dialog);
});
```

The updated `nav-links` section in `render()`:

```typescript
<div class="nav-links">
  <button class="nav-btn" id="nav-journal">📓 Journal</button>
  <button class="nav-btn" id="nav-new-entry">✏️ Nouvelle entrée</button>
  <button class="nav-btn" id="nav-search">🔍 Rechercher</button>
  <button class="nav-btn" id="nav-chat">💬 Chat</button>
  <button class="nav-btn" id="nav-chat-history">📋 Historique</button>
  <button class="nav-btn" id="nav-export">💾 Export</button>
</div>
```

The updated event listeners at the end of `render()`:

```typescript
this.querySelector("#nav-journal")?.addEventListener("click", () => {
  this.navigateTo({ view: "list" });
});
this.querySelector("#nav-new-entry")?.addEventListener("click", () => {
  this.navigateTo({ view: "editor" });
});
this.querySelector("#nav-search")?.addEventListener("click", () => {
  this.navigateTo({ view: "search" });
});
this.querySelector("#nav-chat")?.addEventListener("click", () => {
  this.navigateTo({ view: "chat" });
});
this.querySelector("#nav-chat-history")?.addEventListener("click", () => {
  this.navigateTo({ view: "chat-history" });
});
this.querySelector("#nav-export")?.addEventListener("click", () => {
  const dialog = document.createElement("export-dialog");
  document.body.appendChild(dialog);
});
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
npm run build
```

Expected: build succeeds with no errors.

- [ ] **Step 3: Commit**

```bash
git add src/components/app-shell.ts
git commit -m "feat: add export button to nav bar"
```

---

## Self-Review

**Spec coverage:**
- ✅ Export to folder of `.md` files — Task 2 + Task 4 + Task 5
- ✅ One `.md` per entry with YAML frontmatter — Task 2 `do_export` + `format!(...)`
- ✅ Import skips existing entries by ID — Task 2 `do_import` duplicate check
- ✅ Nav bar button — Task 6
- ✅ Directory picker via plugin-dialog — Task 1 + Task 5
- ✅ Fatal errors surfaced — Task 2 `Err(...)` on invalid dirs
- ✅ Non-fatal errors in `ImportResult.errors` — Task 2 `result.errors.push(...)`
- ✅ Feedback in UI — Task 5 `showFeedback()`
- ✅ FTS5 re-indexed automatically — existing DB triggers handle INSERT

**Type consistency:**
- `ImportResult` defined once in `export/mod.rs`, serialized by serde, deserialized in `src/api/export.ts` as `ImportResult` interface — field names match (`inserted`, `skipped`, `errors`)
- `exportJournal(destDir)` → Rust `dest_dir: String` (Tauri v2 camelCase→snake_case auto-mapping)
- `importJournal(srcDir)` → Rust `src_dir: String` — same

**Placeholder scan:** None found.
