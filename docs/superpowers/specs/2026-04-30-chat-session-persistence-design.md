# Chat Session Persistence Design

## Problem

Navigating away from the chat view destroys the `chat-view` Web Component (via `innerHTML` replacement in `app-shell`). Active session state, messages, and in-flight streaming are lost. Returning to chat starts a fresh session.

## Goal

Active chat session persists across tab switches. User must explicitly close it via a dedicated button. Clicking "💬 Chat" while a session is active resumes it.

## Approach

Keep `chat-view` mounted in a dedicated `#chat-container` div that always lives in the DOM alongside `#app-content`. Toggle visibility instead of creating/destroying. No backend changes required.

## Architecture

### `app-shell.ts`

Two sibling content areas in the render template:

```html
<main id="app-content">...</main>
<div id="chat-container" style="display:none"></div>
```

New state: `private chatMounted = false`

`navigateTo` logic:

- **chat view**: hide `#app-content`, show `#chat-container`. If `!chatMounted`, create `<chat-view>` and append to `#chat-container`, set `chatMounted = true`. If already mounted, no-op (session resumes).
- **all other views**: show `#app-content`, hide `#chat-container`. Mount the requested component into `#app-content` as before.

Close handler: listens for `close-chat` custom event on `#chat-container`. On receive: clear `#chat-container.innerHTML`, set `chatMounted = false`, navigate to `list`.

### `chat-view.ts`

Header updated:

- `← Retour` (existing) — dispatches `navigate { view: "list" }` as before. Session stays mounted; user can return via "💬 Chat".
- `✕ Fermer` (new) — dispatches `close-chat` bubbling custom event. `app-shell` handles destruction.

No changes to session loading, streaming, or history read-only behavior.

## Data Flow

```
User clicks nav tab  →  navigateTo(other)  →  hide #chat-container, show #app-content
User clicks "💬 Chat"  →  navigateTo("chat")  →  show #chat-container (chat-view already mounted)
User clicks "✕ Fermer"  →  close-chat event  →  app-shell clears #chat-container, chatMounted=false
```

## Edge Cases

- **Streaming while switching tabs**: stream continues (unlisten stays active, component alive). UI updates land correctly when user returns.
- **Entry-linked chat**: `entry-id` attribute set on first mount only. Switching away and back resumes same session without re-linking.
- **History view**: loads sessions via `session-id` into `#app-content` as before — unaffected.
- **App init**: `chatMounted = false`, `#chat-container` hidden. No chat-view created until first "💬 Chat" click.

## Files Changed

- `src/components/app-shell.ts` — layout, navigateTo logic, close-chat handler
- `src/components/chat-view.ts` — add "✕ Fermer" button and close-chat dispatch
