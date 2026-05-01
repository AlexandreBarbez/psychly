# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Dev (starts Vite + Tauri)
npm run tauri dev

# Frontend only (Vite on :1420)
npm run dev

# Frontend build
npm run build          # tsc + vite build → dist/

# Rust tests
cd src-tauri && cargo test

# Full app build
npm run tauri build
```

**CI uses pnpm** (not npm). Locally either works, but CI pipeline installs with `pnpm install` / `pnpm build`.

Ollama must be running before `tauri dev`. Override host via `OLLAMA_HOST` env var (default `http://localhost:11434`).

## Architecture

Tauri v2 desktop app: Rust backend + Vanilla TypeScript frontend (Web Components, no framework).

### Frontend (`src/`)

- `src/index.html` → `src/main.ts` bootstraps app
- `src/components/` — Web Components (no framework). `app-shell.ts` is the root shell, routes between journal and therapy views.
- `src/api/` — thin wrappers around `@tauri-apps/api/core` `invoke()` calls, one file per bounded context (`journal.ts`, `chat.ts`)

### Backend (`src-tauri/src/`)

Three DDD bounded contexts, each with `domain/`, `application/`, `infrastructure/`:

| Context | Domain | Infra |
|---------|--------|-------|
| `journal/` | `JournalEntry` | `SqliteJournalRepository`, FTS5 full-text search |
| `therapy/` | `ChatSession`, `ChatMessage` | `SqliteChatSessionRepository`, `OllamaClient` |
| `analysis/` | `EntryAnalysis` | `SqliteAnalysisRepository` |

IPC commands registered in `lib.rs` via `invoke_handler!`. All Tauri commands live in `*/application/commands.rs`.

### Key wiring

- `lib.rs` → wires `Arc<Database>` and `OllamaClient` into Tauri app state; both shared via `app.manage()`
- `db/mod.rs` → opens/migrates SQLite, resolves path relative to `resource_dir()` for portability
- `OllamaClient` reads `OLLAMA_HOST` env var; model hardcoded to `qwen2.5:14b-instruct-q5_K_M`
- `therapy/application/crisis_detection.rs` — detects crisis keywords, triggers 3114 redirect
- `analysis/application/pipeline.rs` — auto-analysis of journal entries on save

### Data

SQLite db at `data/psychly.db` (portable). LLM models at `models/` (Ollama manifests + blobs).
