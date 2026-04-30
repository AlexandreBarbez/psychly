# Chat Session Persistence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Keep `chat-view` mounted across tab switches so the active session persists until the user explicitly closes it.

**Architecture:** Add a permanent `#chat-container` div alongside `#app-content` in `app-shell`. Toggle CSS visibility instead of destroying the component. A new "✕ Fermer" button in `chat-view` fires a `close-chat` event that `app-shell` handles to tear down the session.

**Tech Stack:** TypeScript, Web Components, Vite, Tauri v2

---

## File Map

| File | Change |
|------|--------|
| `src/components/app-shell.ts` | Add `chatMounted` state, `#chat-container` div, update `navigateTo`, add `close-chat` handler |
| `src/components/chat-view.ts` | Add "✕ Fermer" button and `close-chat` dispatch |
| `src/styles.css` | Add `#chat-container` layout rule |

---

### Task 1: Add `#chat-container` to app-shell layout

**Files:**
- Modify: `src/components/app-shell.ts`
- Modify: `src/styles.css`

No test framework exists for frontend — verify manually in dev server.

- [ ] **Step 1: Add `chatMounted` property to `AppShell` class**

In `src/components/app-shell.ts`, update the class properties section (lines 12-14):

```typescript
export class AppShell extends HTMLElement {
  private currentView = "list";
  private ollamaConnected = false;
  private chatMounted = false;
```

- [ ] **Step 2: Add `#chat-container` to render template**

In `render()`, replace the `<main id="app-content">` block (around line 108-110) with:

```html
<main id="app-content">
  <journal-list></journal-list>
</main>
<div id="chat-container"></div>
```

- [ ] **Step 3: Add CSS rule for `#chat-container`**

In `src/styles.css`, after the `#app-content` rule (after line 161):

```css
#chat-container {
  display: none;
  flex: 1;
  overflow: hidden;
}
```

- [ ] **Step 4: Update `navigateTo` to toggle visibility instead of replacing content**

Replace the entire `navigateTo` method body with:

```typescript
private navigateTo(detail: NavigateDetail) {
  this.currentView = detail.view;
  const appContent = this.querySelector<HTMLElement>("#app-content");
  const chatContainer = this.querySelector<HTMLElement>("#chat-container");
  if (!appContent || !chatContainer) return;

  if (detail.view === "chat") {
    appContent.style.display = "none";
    chatContainer.style.display = "block";
    if (!this.chatMounted) {
      if (detail.sessionId) {
        chatContainer.innerHTML = `<chat-view session-id="${detail.sessionId}"></chat-view>`;
      } else if (detail.entryId) {
        chatContainer.innerHTML = `<chat-view entry-id="${detail.entryId}"></chat-view>`;
      } else {
        chatContainer.innerHTML = `<chat-view></chat-view>`;
      }
      this.chatMounted = true;
    }
    return;
  }

  appContent.style.display = "";
  chatContainer.style.display = "none";

  switch (detail.view) {
    case "list":
      appContent.innerHTML = `<journal-list></journal-list>`;
      break;
    case "editor":
      if (detail.id) {
        appContent.innerHTML = `<journal-editor entry-id="${detail.id}"></journal-editor>`;
      } else {
        appContent.innerHTML = `<journal-editor></journal-editor>`;
      }
      break;
    case "entry":
      appContent.innerHTML = `<journal-entry-view entry-id="${detail.id}"></journal-entry-view>`;
      break;
    case "search":
      appContent.innerHTML = `<journal-search></journal-search>`;
      break;
    case "chat-history":
      appContent.innerHTML = `<chat-session-list></chat-session-list>`;
      break;
  }
}
```

- [ ] **Step 5: Add `close-chat` event handler in `render()`**

At the end of `render()`, after all existing `addEventListener` calls (before the closing `}`), add:

```typescript
this.querySelector("#chat-container")?.addEventListener("close-chat", () => {
  const chatContainer = this.querySelector<HTMLElement>("#chat-container");
  if (chatContainer) chatContainer.innerHTML = "";
  this.chatMounted = false;
  this.navigateTo({ view: "list" });
});
```

- [ ] **Step 6: Commit**

```bash
git add src/components/app-shell.ts src/styles.css
git commit -m "feat: add persistent chat container to app-shell"
```

---

### Task 2: Add "✕ Fermer" button to chat-view

**Files:**
- Modify: `src/components/chat-view.ts`

- [ ] **Step 1: Update chat header template in `render()`**

Replace the `chat-header` div (lines 162-166):

```typescript
<div class="chat-header">
  <button class="btn-back" id="chat-back-btn">← Retour</button>
  <h2>Conversation</h2>
  <button class="btn-secondary" id="chat-close-btn">✕ Fermer</button>
</div>
```

- [ ] **Step 2: Add event listener for close button**

After the existing `#chat-back-btn` listener (around line 177), add:

```typescript
this.querySelector("#chat-close-btn")?.addEventListener("click", () => {
  this.dispatchEvent(
    new CustomEvent("close-chat", { bubbles: true, composed: true })
  );
});
```

- [ ] **Step 3: Verify in dev server**

Run: `npm run dev`

Check:
1. Open chat → start typing a message → send it
2. Switch to "📓 Journal" tab
3. Switch back to "💬 Chat" → conversation still visible, input enabled
4. Click "✕ Fermer" → lands on journal list, chat state cleared
5. Click "💬 Chat" again → fresh empty session

- [ ] **Step 4: Commit**

```bash
git add src/components/chat-view.ts
git commit -m "feat: add close button to chat-view, dispatch close-chat event"
```
