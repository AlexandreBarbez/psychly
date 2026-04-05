## 1. Project Scaffolding

- [x] 1.1 Initialize Tauri v2 project with Rust backend and web frontend
- [x] 1.2 Configure TypeScript build pipeline (tsconfig, bundler) for vanilla TS + Web Components
- [x] 1.3 Set up DDD folder structure in Rust backend: `journal/`, `therapy/`, `analysis/` bounded contexts, each with `domain/`, `application/`, `infrastructure/` layers
- [x] 1.4 Add `rusqlite` (with bundled SQLite + FTS5) and `reqwest` dependencies to Cargo.toml
- [x] 1.5 Create README.md for each bounded context explaining its role and boundaries

## 2. Database & Persistence

- [x] 2.1 Design SQLite schema: `journal_entries` table (id, body, created_at, updated_at), `chat_sessions` table (id, created_at, journal_entry_id nullable), `chat_messages` table (id, session_id, role, content, created_at), `entry_analyses` table (id, entry_id, emotional_tone, themes, patterns, created_at)
- [x] 2.2 Implement database initialization module in Rust with automatic schema creation on first launch
- [x] 2.3 Create FTS5 virtual table for full-text search on journal entry body
- [x] 2.4 Implement portable database path resolution (relative to app root, no hardcoded paths)
- [x] 2.5 Test database initialization: verify schema creation on first launch and idempotence on subsequent launches
- [x] 2.6 Test FTS5 virtual table: verify indexing and search on journal entry body
- [x] 2.7 Test portable path resolution: verify database is created at relative path, no absolute paths used

## 3. Journal Domain (Rust)

- [x] 3.1 Implement `JournalEntry` domain entity with id, body, created_at, updated_at
- [x] 3.2 Implement `JournalRepository` trait and SQLite implementation: create, read, update, delete
- [x] 3.3 Implement full-text search query via FTS5 with relevance ranking
- [x] 3.4 Implement list entries with pagination, sorted by date descending, with text preview
- [x] 3.5 Register Tauri IPC commands: `create_entry`, `get_entry`, `list_entries`, `update_entry`, `delete_entry`, `search_entries`
- [x] 3.6 Test `JournalEntry` domain entity: validate creation, field integrity, timestamp behavior
- [x] 3.7 Test `JournalRepository` SQLite implementation: CRUD round-trip (create → read → update → delete), verify data integrity
- [x] 3.8 Test full-text search: verify FTS5 ranking, partial match, accent handling, empty results
- [x] 3.9 Test list entries: verify pagination, date sorting, text preview truncation

## 4. Journal UI (Frontend)

- [x] 4.1 Create `<journal-list>` Web Component: displays chronological entry list with date and preview, empty state message
- [x] 4.2 Create `<journal-editor>` Web Component: text editor for creating/editing entries with save action
- [x] 4.3 Create `<journal-entry-view>` Web Component: read-only full entry display with date
- [x] 4.4 Create `<journal-search>` Web Component: search input with results display and excerpt highlighting
- [x] 4.5 Implement delete confirmation dialog
- [x] 4.6 Wire all journal components to Tauri IPC commands
- [x] 4.7 Test `<journal-list>`: verify rendering with entries, empty state, correct date/preview display
- [x] 4.8 Test `<journal-editor>`: verify save triggers IPC, validation of empty body
- [x] 4.9 Test `<journal-search>`: verify search triggers on input, results rendering, highlighted excerpts

## 5. Ollama Integration (Rust)

- [x] 5.1 Implement Ollama HTTP client in Rust (`reqwest` to `localhost:11434`): chat completion endpoint with streaming support
- [x] 5.2 Implement Ollama health check (detect if running, attempt to start if not)
- [x] 5.3 Configure Ollama model name (Qwen 2.5 14B) as application setting
- [x] 5.4 Implement streaming response relay via Tauri events (SSE pattern from Rust to frontend)
- [x] 5.5 Handle Ollama unavailability: return structured error to frontend for UI display
- [x] 5.6 Test Ollama HTTP client: verify request formatting, streaming chunk parsing, response deserialization (mock HTTP server)
- [x] 5.7 Test health check: verify detection of running/stopped Ollama (mock responses)
- [x] 5.8 Test unavailability handling: verify structured error returned when Ollama unreachable

## 6. Therapy Domain — Chat (Rust)

- [x] 6.1 Implement `ChatSession` and `ChatMessage` domain entities
- [x] 6.2 Implement `ChatSessionRepository` trait and SQLite implementation: create session, add message, list sessions, get session with messages
- [x] 6.3 Build therapeutic system prompt incorporating ACT, CBT, DBT, Schema Therapy, Mindfulness, attachment theory, cognitive distortions, emotional regulation, defense mechanisms, mentalization, exposure/avoidance — in French
- [x] 6.4 Implement prompt assembly: system prompt + journal context (from analysis) + conversation history + user message
- [x] 6.5 Implement context window management: summarize older messages when conversation exceeds token limit
- [x] 6.6 Implement crisis detection keywords/patterns with safety response (empathy + 3114 helpline + professional referral)
- [x] 6.7 Register Tauri IPC commands: `start_chat_session`, `send_message` (streaming), `list_chat_sessions`, `get_chat_session`
- [x] 6.8 Test `ChatSession` / `ChatMessage` domain entities: validate creation, relationships, field integrity
- [x] 6.9 Test `ChatSessionRepository` SQLite implementation: create session, add messages, retrieve session with messages, list sessions
- [x] 6.10 Test prompt assembly: verify system prompt + context + history + user message are correctly composed
- [x] 6.11 Test context window management: verify summarization triggers when conversation exceeds token limit
- [x] 6.12 Test crisis detection: verify keywords trigger safety response with 3114 helpline

## 7. Chat UI (Frontend)

- [x] 7.1 Create `<chat-view>` Web Component: message thread with streaming text display, input field, send button
- [x] 7.2 Create `<chat-session-list>` Web Component: past sessions with date and preview, read-only reopening
- [x] 7.3 Implement streaming message rendering (progressive text display via Tauri events)
- [x] 7.4 Add "Chat" action button on journal entries to start contextual chat
- [x] 7.5 Implement first-use disclaimer dialog (AI is not a licensed therapist, recommendation to consult professionals)
- [x] 7.6 Wire chat components to Tauri IPC commands
- [x] 7.7 Test `<chat-view>`: verify message rendering, streaming text display, send action
- [x] 7.8 Test `<chat-session-list>`: verify past sessions display, read-only mode on reopen
- [x] 7.9 Test disclaimer dialog: verify display on first use, not shown on subsequent uses

## 8. Analysis Domain (Rust)

- [x] 8.1 Implement `EntryAnalysis` domain entity: emotional_tone, themes (list), cognitive_patterns (list)
- [x] 8.2 Implement `AnalysisRepository` trait and SQLite implementation: store, get by entry_id, get recent analyses
- [x] 8.3 Build analysis prompt: instruct LLM to extract emotional tone, key themes, and cognitive patterns from a journal entry — structured JSON output
- [x] 8.4 Implement async analysis pipeline: trigger on entry save/edit, call Ollama, parse response, store result — non-blocking
- [x] 8.5 Implement emotional trend aggregation: summarize emotional tones across recent entries (weekly dominant emotions)
- [x] 8.6 Implement context builder for therapy chat: compile recent analyses into a concise summary for injection into chat prompt
- [x] 8.7 Register Tauri IPC command: `get_entry_analysis` (for optional future UI display)
- [x] 8.8 Test `EntryAnalysis` domain entity: validate creation, field integrity
- [x] 8.9 Test `AnalysisRepository` SQLite implementation: store, retrieve by entry_id, get recent analyses, verify re-analysis replaces previous result
- [x] 8.10 Test analysis prompt: verify LLM response parsing into structured JSON (emotional_tone, themes, patterns) with mock LLM response
- [x] 8.11 Test async analysis pipeline: verify non-blocking execution, result storage after completion
- [x] 8.12 Test emotional trend aggregation: verify weekly summary with sufficient entries (≥3), empty result with fewer
- [x] 8.13 Test context builder: verify compiled analysis summary format for chat prompt injection

## 9. Application Navigation & Layout

- [x] 9.1 Create `<app-shell>` Web Component: main layout with navigation between Journal and Chat views
- [x] 9.2 Implement client-side routing between journal list, entry view/edit, chat, and chat history
- [x] 9.3 Add Ollama status indicator in the UI (connected/unavailable)
- [x] 9.4 Test `<app-shell>`: verify navigation between Journal and Chat views
- [x] 9.5 Test client-side routing: verify direct URL access to each view, back/forward navigation
- [x] 9.6 Test Ollama status indicator: verify correct state display (connected vs unavailable)

## 10. Portability & Launcher

- [x] 10.1 Create `start.sh` launcher script: set `OLLAMA_MODELS` and `OLLAMA_HOST` to portable paths, start Ollama, wait for readiness, launch Tauri app
- [x] 10.2 Implement clean shutdown: trap signals in `start.sh` to stop Ollama when app exits
- [x] 10.3 Document portable directory structure (`app/`, `data/`, `models/`, `ollama/`)
- [x] 10.4 Verify all path resolutions are relative to application root (no absolute paths)
- [x] 10.5 Test full copy-to-USB-and-run workflow on a separate Mac

## 11. Documentation

- [x] 11.1 Write project root README.md with setup, usage, and architecture overview
- [x] 11.2 Add inline markdown documentation for each DDD bounded context and key modules
- [x] 11.3 Document system prompt design decisions and therapeutic framework choices
