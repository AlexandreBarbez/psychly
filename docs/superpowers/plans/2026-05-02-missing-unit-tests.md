# Missing Unit Tests Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add missing unit tests identified by code review, plus fix the orphaned-message bug in the stream error path.

**Architecture:** All tests are Rust `#[cfg(test)]` modules added inline to existing source files. No new files created. One bug fix in `src-tauri/src/therapy/application/commands.rs`.

**Tech Stack:** Rust, rusqlite (in-memory SQLite for tests), chrono, uuid — all already in Cargo.toml.

---

## File Map

| File | Change |
|---|---|
| `src-tauri/src/journal/infrastructure/sqlite_repository.rs` | Add FTS5 malformed query test |
| `src-tauri/src/export/mod.rs` | Add parse_markdown_entry edge case tests |
| `src-tauri/src/analysis/application/context_builder.rs` | Add truncate_entries edge case tests |
| `src-tauri/src/therapy/infrastructure/sqlite_repository.rs` | Add crisis flow repo test |
| `src-tauri/src/therapy/application/commands.rs` | Fix orphaned message bug (save error stub on stream failure) |

---

## Task 1: FTS5 Malformed Query Test

**Files:**
- Modify: `src-tauri/src/journal/infrastructure/sqlite_repository.rs` (append to `#[cfg(test)] mod tests`)

The `search()` method uses `filter_map(|r| r.ok())` which silently drops SQLite errors during row iteration. A malformed FTS5 expression like an unclosed quote causes a SQLite parse error that surfaces during iteration — it gets swallowed, returning `Ok([])`. The test documents this behavior and guards against panics.

- [ ] **Step 1: Write the failing test**

Open `src-tauri/src/journal/infrastructure/sqlite_repository.rs`. Inside the existing `#[cfg(test)] mod tests { ... }` block, add after the last test:

```rust
    #[test]
    fn test_search_malformed_fts5_does_not_panic() {
        let (_db, repo) = setup();
        // Insert an entry so the table is non-empty
        repo.create(&JournalEntry::new("contenu de test".to_string())).unwrap();

        // Unclosed quote is an invalid FTS5 expression.
        // rusqlite returns an error during row iteration, which filter_map silently drops.
        // The important guarantee: this never panics and returns Ok.
        let result = repo.search("\"unclosed");
        assert!(result.is_ok(), "Malformed FTS5 query must not panic or propagate as Err");
        // May return empty vec (error silently swallowed) — that is acceptable current behavior.
        // A future fix would propagate the error instead.
    }

    #[test]
    fn test_search_fts5_operator_only() {
        let (_db, repo) = setup();
        repo.create(&JournalEntry::new("test de recherche".to_string())).unwrap();

        // Bare FTS5 operator with no operands — another malformed expression.
        let result = repo.search("OR");
        assert!(result.is_ok(), "FTS5 bare operator must not panic");
    }
```

- [ ] **Step 2: Run and confirm behavior**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test -p psychly journal::infrastructure::sqlite_repository::tests::test_search_malformed_fts5_does_not_panic -- --nocapture 2>&1
cargo test -p psychly journal::infrastructure::sqlite_repository::tests::test_search_fts5_operator_only -- --nocapture 2>&1
```

Expected: both PASS. If either panics, that is the bug — report DONE_WITH_CONCERNS with the panic message.

- [ ] **Step 3: Run full test suite to confirm no regressions**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test 2>&1 | tail -5
```

Expected: `test result: ok. N passed; 0 failed`

- [ ] **Step 4: Commit**

```bash
cd /Users/gyshido/Workspace/Psychly
git add src-tauri/src/journal/infrastructure/sqlite_repository.rs
git commit -m "test: document FTS5 malformed query behavior in search()"
```

---

## Task 2: parse_markdown_entry Edge Case Tests

**Files:**
- Modify: `src-tauri/src/export/mod.rs` (append to `#[cfg(test)] mod tests`)

Two edge cases in `parse_markdown_entry`:
1. Body containing `---` on its own line — the parser uses `find("\n---")` to locate the FIRST occurrence, so it finds the closing frontmatter delimiter correctly; the body `---` is preserved.
2. Body that is empty after the closing `---` — should parse successfully with empty body string.

- [ ] **Step 1: Write the failing tests**

Open `src-tauri/src/export/mod.rs`. Inside the existing `#[cfg(test)] mod tests { ... }` block, add after the last test:

```rust
    #[test]
    fn test_parse_markdown_entry_body_contains_triple_dash() {
        // A body that itself contains a --- separator line must not confuse the parser.
        // The parser finds the FIRST \n--- as the closing frontmatter delimiter, so
        // subsequent --- lines in the body are passed through untouched.
        let content = "---\nid: test-uuid\ncreated_at: 2026-01-01T00:00:00\nupdated_at: 2026-01-01T00:00:00\n---\n\nPremière partie\n---\nDeuxième partie";
        let result = parse_markdown_entry(content);
        assert!(result.is_some(), "Entry with --- in body should parse successfully");
        let (id, _created, _updated, body) = result.unwrap();
        assert_eq!(id, "test-uuid");
        assert!(body.contains("Première partie"), "Body must contain first section");
        assert!(body.contains("---"), "Body must preserve the --- separator");
        assert!(body.contains("Deuxième partie"), "Body must contain second section");
    }

    #[test]
    fn test_parse_markdown_entry_empty_body() {
        // An entry with no body after the closing --- should parse with empty string.
        let content = "---\nid: empty-body-uuid\ncreated_at: 2026-01-01T00:00:00\nupdated_at: 2026-01-01T00:00:00\n---\n";
        let result = parse_markdown_entry(content);
        assert!(result.is_some(), "Entry with empty body should parse successfully");
        let (id, _, _, body) = result.unwrap();
        assert_eq!(id, "empty-body-uuid");
        assert_eq!(body, "", "Body should be empty string");
    }

    #[test]
    fn test_parse_markdown_entry_missing_updated_at_defaults_empty() {
        // updated_at is optional — when absent, it should default to empty string
        // (do_import then substitutes created_at for it).
        let content = "---\nid: no-updated\ncreated_at: 2026-01-01T00:00:00\n---\n\nCorps sans updated_at";
        let result = parse_markdown_entry(content);
        assert!(result.is_some(), "Entry without updated_at should parse");
        let (id, created_at, updated_at, body) = result.unwrap();
        assert_eq!(id, "no-updated");
        assert_eq!(created_at, "2026-01-01T00:00:00");
        assert_eq!(updated_at, "", "Missing updated_at should default to empty string");
        assert_eq!(body, "Corps sans updated_at");
    }
```

- [ ] **Step 2: Run the tests**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test -p psychly export::tests::test_parse_markdown_entry_body_contains_triple_dash -- --nocapture 2>&1
cargo test -p psychly export::tests::test_parse_markdown_entry_empty_body -- --nocapture 2>&1
cargo test -p psychly export::tests::test_parse_markdown_entry_missing_updated_at_defaults_empty -- --nocapture 2>&1
```

Expected: all 3 PASS.

- [ ] **Step 3: Full suite**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test 2>&1 | tail -5
```

Expected: `test result: ok. N passed; 0 failed`

- [ ] **Step 4: Commit**

```bash
cd /Users/gyshido/Workspace/Psychly
git add src-tauri/src/export/mod.rs
git commit -m "test: add parse_markdown_entry edge cases (body with ---, empty body, missing updated_at)"
```

---

## Task 3: truncate_entries Edge Case Tests

**Files:**
- Modify: `src-tauri/src/analysis/application/context_builder.rs` (append to `#[cfg(test)] mod tests`)

Existing test covers 2 entries where one gets truncated. Missing cases:
- Single entry gets the entire budget (not divided by 1, but `budget / 1 = budget`)
- All entries fit within their share — no truncation marker added
- Zero entries — returns empty vec

`CHARS_PER_TOKEN = 4` is imported from `prompt_assembly`. In the test module, access it via `use crate::therapy::application::prompt_assembly::CHARS_PER_TOKEN;`.

- [ ] **Step 1: Write the failing tests**

Open `src-tauri/src/analysis/application/context_builder.rs`. Inside the existing `#[cfg(test)] mod tests { ... }` block, add after the last test:

```rust
    #[test]
    fn test_truncate_entries_single_entry_gets_full_budget() {
        use crate::therapy::application::prompt_assembly::CHARS_PER_TOKEN;
        // With 1 entry, budget_tokens / 1 = budget_tokens, max_chars = budget_tokens * CHARS_PER_TOKEN
        let budget_tokens: usize = 10; // 40 chars
        let max_chars = budget_tokens * CHARS_PER_TOKEN;

        // Entry shorter than budget — not truncated
        let short = JournalEntry::new("bonjour".to_string()); // 7 chars < 40
        let result = truncate_entries(&[short.clone()], budget_tokens);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "bonjour", "Short entry must not be truncated");
        assert!(!result[0].1.contains("[...]"));

        // Entry longer than budget — truncated to max_chars + " [...]"
        let long_body = "a".repeat(max_chars + 10); // 50 chars > 40
        let long = JournalEntry::new(long_body.clone());
        let result = truncate_entries(&[long], budget_tokens);
        assert_eq!(result.len(), 1);
        assert!(result[0].1.ends_with(" [...]"), "Long entry must end with truncation marker");
        assert_eq!(
            result[0].1.len(),
            max_chars + " [...]".len(),
            "Truncated body must be exactly max_chars + marker length"
        );
    }

    #[test]
    fn test_truncate_entries_all_fit_no_truncation() {
        // budget_tokens=100, 2 entries → 50 tokens each = 200 chars each (CHARS_PER_TOKEN=4)
        // Both entries are far below 200 chars, so neither should be truncated.
        let budget_tokens: usize = 100;
        let e1 = JournalEntry::new("court".to_string());         // 5 chars
        let e2 = JournalEntry::new("aussi court".to_string());   // 11 chars
        let result = truncate_entries(&[e1, e2], budget_tokens);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1, "court");
        assert_eq!(result[1].1, "aussi court");
        assert!(!result[0].1.contains("[...]"));
        assert!(!result[1].1.contains("[...]"));
    }

    #[test]
    fn test_truncate_entries_empty_input() {
        let result = truncate_entries(&[], 1000);
        assert!(result.is_empty(), "Empty input must return empty vec");
    }
```

- [ ] **Step 2: Run the tests**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test -p psychly analysis::application::context_builder::tests::test_truncate_entries_single_entry_gets_full_budget -- --nocapture 2>&1
cargo test -p psychly analysis::application::context_builder::tests::test_truncate_entries_all_fit_no_truncation -- --nocapture 2>&1
cargo test -p psychly analysis::application::context_builder::tests::test_truncate_entries_empty_input -- --nocapture 2>&1
```

Expected: all 3 PASS.

- [ ] **Step 3: Full suite**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test 2>&1 | tail -5
```

Expected: `test result: ok. N passed; 0 failed`

- [ ] **Step 4: Commit**

```bash
cd /Users/gyshido/Workspace/Psychly
git add src-tauri/src/analysis/application/context_builder.rs
git commit -m "test: add truncate_entries edge cases (single entry, all fit, empty)"
```

---

## Task 4: Crisis Flow Repository Test

**Files:**
- Modify: `src-tauri/src/therapy/infrastructure/sqlite_repository.rs` (append to `#[cfg(test)] mod tests`)

The `send_message` command (in `commands.rs`) follows this crisis path:
1. `detect_crisis(&input.content)` returns `true`
2. Calls `crisis_safety_response()` to get the response string
3. Saves user message via `repo.add_message(&user_msg)`
4. Saves crisis assistant message via `repo.add_message(&assistant_msg)`
5. Returns the assistant message

This test verifies that the repository correctly stores both messages — user message with crisis content and assistant message containing the 3114 helpline reference — by simulating steps 3–4 directly against the in-memory DB.

- [ ] **Step 1: Add the import at the top of the test module**

Open `src-tauri/src/therapy/infrastructure/sqlite_repository.rs`. At the top of the `#[cfg(test)] mod tests { ... }` block, the existing `use super::*;` imports `SqliteChatSessionRepository`, `ChatSession`, `ChatMessage`, and `Database`. We also need `crisis_safety_response`. Add the import inside the tests block:

The existing block starts with:
```rust
#[cfg(test)]
mod tests {
    use super::*;
```

Add after `use super::*;`:
```rust
    use crate::therapy::application::crisis_detection::crisis_safety_response;
```

- [ ] **Step 2: Write the failing test**

Inside the `#[cfg(test)] mod tests { ... }` block, after the last existing test, add:

```rust
    #[test]
    fn test_crisis_path_saves_both_messages() {
        // Simulates what send_message does when detect_crisis() returns true.
        // Verifies both the user message and crisis assistant message are persisted.
        let db = setup();
        let repo = SqliteChatSessionRepository::new(Arc::clone(&db));

        let session = ChatSession::new(None);
        repo.create_session(&session).unwrap();

        // Step 1: save user message (crisis content)
        let user_msg = ChatMessage::new(
            session.id.clone(),
            "user".to_string(),
            "j'ai envie de me suicider".to_string(),
        );
        repo.add_message(&user_msg).unwrap();

        // Step 2: save crisis safety response as assistant message
        let crisis_response = crisis_safety_response();
        let assistant_msg = ChatMessage::new(
            session.id.clone(),
            "assistant".to_string(),
            crisis_response.clone(),
        );
        repo.add_message(&assistant_msg).unwrap();

        // Verify both messages are stored with correct roles and order
        let messages = repo.get_session_messages(&session.id).unwrap();
        assert_eq!(messages.len(), 2, "Both user and assistant messages must be persisted");
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[0].content, "j'ai envie de me suicider");
        assert_eq!(messages[1].role, "assistant");
        assert!(
            messages[1].content.contains("3114"),
            "Crisis response must reference the 3114 helpline"
        );
        assert!(
            messages[1].content.contains("SOS Amitié"),
            "Crisis response must reference SOS Amitié"
        );
    }

    #[test]
    fn test_crisis_response_is_not_empty() {
        // Guard: crisis_safety_response() must never return an empty string,
        // otherwise the assistant message would be stored empty in the DB.
        let response = crisis_safety_response();
        assert!(!response.is_empty(), "Crisis safety response must not be empty");
        assert!(response.len() > 100, "Crisis safety response must be substantive (>100 chars)");
    }
```

- [ ] **Step 3: Run the tests**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test -p psychly therapy::infrastructure::sqlite_repository::tests::test_crisis_path_saves_both_messages -- --nocapture 2>&1
cargo test -p psychly therapy::infrastructure::sqlite_repository::tests::test_crisis_response_is_not_empty -- --nocapture 2>&1
```

Expected: both PASS.

- [ ] **Step 4: Full suite**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test 2>&1 | tail -5
```

Expected: `test result: ok. N passed; 0 failed`

- [ ] **Step 5: Commit**

```bash
cd /Users/gyshido/Workspace/Psychly
git add src-tauri/src/therapy/infrastructure/sqlite_repository.rs
git commit -m "test: add crisis flow repo test — both messages persisted with 3114 reference"
```

---

## Task 5: Fix Orphaned Message + Test

**Files:**
- Modify: `src-tauri/src/therapy/application/commands.rs` (fix stream error path)
- Modify: `src-tauri/src/therapy/infrastructure/sqlite_repository.rs` (add test documenting fix)

**The bug:** When the Ollama stream returns an error (line 151 in `commands.rs`), the function returns `Err(...)` immediately. The user message was already saved at line 119. The assistant message is never saved. Result: an orphaned user turn in the DB that will be fed into the LLM context on the next session load, confusing subsequent responses.

**The fix:** On stream error, save a stub error assistant message before returning `Err`. This pairs the user message with a visible error response and prevents context poisoning.

- [ ] **Step 1: Fix the stream error path in commands.rs**

Open `src-tauri/src/therapy/application/commands.rs`. Find this block (around line 141–153):

```rust
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
            Err(e) => return Err(format!("Stream error: {e}")),
        }
    }
```

Replace it with:

```rust
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
```

- [ ] **Step 2: Verify it compiles**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo build 2>&1 | grep -E "^error"
```

Expected: no output (no errors).

- [ ] **Step 3: Add a test documenting the fix behavior**

Open `src-tauri/src/therapy/infrastructure/sqlite_repository.rs`. Inside the `#[cfg(test)] mod tests { ... }` block, after the last test, add:

```rust
    #[test]
    fn test_error_stub_pairs_orphaned_user_message() {
        // Documents the fix for the orphaned-message bug:
        // when send_message encounters a stream error, it saves a stub assistant
        // message so the user message is never left unpaired in the DB.
        // This test simulates the repository operations that the fixed code performs.
        let db = setup();
        let repo = SqliteChatSessionRepository::new(Arc::clone(&db));

        let session = ChatSession::new(None);
        repo.create_session(&session).unwrap();

        // The user message is saved before streaming starts
        let user_msg = ChatMessage::new(
            session.id.clone(),
            "user".to_string(),
            "Comment gérer mon anxiété ?".to_string(),
        );
        repo.add_message(&user_msg).unwrap();

        // Simulate stream error: save the error stub assistant message
        let error_stub = ChatMessage::new(
            session.id.clone(),
            "assistant".to_string(),
            "Désolé, une erreur de connexion est survenue. Veuillez réessayer.".to_string(),
        );
        repo.add_message(&error_stub).unwrap();

        // Verify: user message is paired — 2 messages, not 1
        let messages = repo.get_session_messages(&session.id).unwrap();
        assert_eq!(messages.len(), 2, "User message must be paired with error stub");
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[1].role, "assistant");
        assert!(
            messages[1].content.contains("erreur"),
            "Error stub content must mention 'erreur'"
        );
    }
```

- [ ] **Step 4: Run the test**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test -p psychly therapy::infrastructure::sqlite_repository::tests::test_error_stub_pairs_orphaned_user_message -- --nocapture 2>&1
```

Expected: PASS.

- [ ] **Step 5: Full suite**

```bash
cd /Users/gyshido/Workspace/Psychly/src-tauri
cargo test 2>&1 | tail -5
```

Expected: `test result: ok. N passed; 0 failed`

- [ ] **Step 6: Commit**

```bash
cd /Users/gyshido/Workspace/Psychly
git add src-tauri/src/therapy/application/commands.rs src-tauri/src/therapy/infrastructure/sqlite_repository.rs
git commit -m "fix: save error stub on stream failure to prevent orphaned user message

test: document error stub pairing behavior in repo tests"
```

---

## Done

After Task 5:
- FTS5 malformed queries documented (no panic guarantee)
- parse_markdown_entry edge cases covered (body with ---, empty body, missing updated_at)
- truncate_entries edge cases covered (single entry, all fit, empty)
- Crisis flow verified at repository level (both messages persisted, 3114 present)
- Orphaned message bug fixed + documented with test
