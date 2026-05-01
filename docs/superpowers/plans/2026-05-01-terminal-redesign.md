# Terminal Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the soft therapy-app visual design with a Mac terminal / zsh aesthetic — Dracula palette, JetBrains Mono everywhere, left sidebar, Powerline Unicode icons.

**Architecture:** CSS token replacement in `styles.css` + layout restructure in `app-shell.ts`. No backend changes. No other `.ts` files modified — only the CSS classes they reference get restyled.

**Tech Stack:** Vanilla TypeScript Web Components, CSS custom properties, JetBrains Mono (fonts.bunny.net)

---

## File Map

| File | What changes |
|---|---|
| `src/index.html` | Replace font CDN link (Lora+Inter → JetBrains Mono) |
| `src/styles.css` | Full token replacement + all component styles rewritten |
| `src/components/app-shell.ts` | New `render()` — titlebar, sidebar, breadcrumb, bottom bar; new `updateActiveNav()` and `updateBreadcrumb()` methods |

---

## Task 1: Font + Design Tokens

**Files:**
- Modify: `src/index.html`
- Modify: `src/styles.css` (`:root` block only)

- [ ] **Step 1: Replace font link in index.html**

Open `src/index.html`. Replace:
```html
  <link rel="preconnect" href="https://fonts.bunny.net">
  <link href="https://fonts.bunny.net/css?family=lora:400,600|inter:400,500,600&display=swap" rel="stylesheet">
```
With:
```html
  <link rel="preconnect" href="https://fonts.bunny.net">
  <link href="https://fonts.bunny.net/css?family=jetbrains-mono:300,400,500,600&display=swap" rel="stylesheet">
```

- [ ] **Step 2: Replace all CSS tokens in styles.css**

Replace the entire `:root { ... }` block in `src/styles.css` with:

```css
:root {
  /* Dracula palette */
  --color-bg: #282a36;
  --color-surface: #21222c;
  --color-surface-alt: #2d2e3f;
  --color-sidebar: #191a21;
  --color-border: #44475a;
  --color-text: #f8f8f2;
  --color-text-muted: #6272a4;
  --color-primary: #bd93f9;
  --color-primary-hover: #a97ff0;
  --color-accent-green: #50fa7b;
  --color-accent-cyan: #8be9fd;
  --color-accent-orange: #ffb86c;
  --color-user-bubble: #44475a;
  --color-assistant-bubble: #21222c;
  --color-danger: #ff5555;
  --color-danger-hover: #ff3333;

  /* Typography — monospace everywhere */
  --font-body: 'JetBrains Mono', 'Courier New', monospace;
  --font-heading: 'JetBrains Mono', 'Courier New', monospace;
  --font-size-base: 0.8125rem;
  --font-size-sm: 0.6875rem;
  --font-size-lg: 0.875rem;
  --font-size-xl: 1rem;
  --line-height-body: 1.6;

  /* Spacing (unchanged) */
  --space-xs: 0.375rem;
  --space-sm: 0.625rem;
  --space-md: 1rem;
  --space-lg: 1.5rem;
  --space-xl: 2.5rem;

  /* Border radii — sharp terminal style */
  --radius-sm: 3px;
  --radius-md: 4px;
  --radius-lg: 6px;
  --radius-pill: 3px;

  /* No shadows */
  --shadow-sm: none;
  --shadow-md: none;
  --shadow-lg: none;
}
```

- [ ] **Step 3: Verify compilation**

```bash
npm run build
```

Expected: exits 0, no TypeScript errors. Font change has no TS impact.

- [ ] **Step 4: Commit**

```bash
git add src/index.html src/styles.css
git commit -m "feat: replace design tokens with Dracula palette, JetBrains Mono"
```

---

## Task 2: App Shell Layout — Titlebar, Sidebar, Main Panel

**Files:**
- Modify: `src/styles.css` — add new layout CSS classes, remove old nav classes
- Modify: `src/components/app-shell.ts` — new `render()` structure

- [ ] **Step 1: Replace layout CSS in styles.css**

Find the `APP SHELL & NAVIGATION` section in `src/styles.css`. Replace everything from `.app-shell` through `.status-disconnected` and `#app-content` and `#chat-container` with:

```css
/* ============================================================
   APP SHELL & LAYOUT
   ============================================================ */
.app-shell {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

/* Fake macOS titlebar */
.titlebar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: var(--color-sidebar);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
  -webkit-app-region: drag;
}

.dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
}
.dot-red    { background: #ff5555; }
.dot-yellow { background: #ffb86c; }
.dot-green  { background: #50fa7b; }

.titlebar-path {
  margin-left: 8px;
  color: var(--color-text-muted);
  font-size: var(--font-size-sm);
  letter-spacing: .04em;
}

/* Row: sidebar + main panel */
.app-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

/* Left sidebar */
.app-sidebar {
  width: 200px;
  flex-shrink: 0;
  background: var(--color-sidebar);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-brand {
  padding: 14px 16px 12px;
  border-bottom: 1px solid var(--color-border);
}

.sidebar-brand-name {
  color: var(--color-primary);
  font-size: var(--font-size-lg);
  font-weight: 600;
  letter-spacing: .05em;
}

.sidebar-brand-sub {
  color: var(--color-text-muted);
  font-size: 10px;
  margin-top: 2px;
}

.nav-section {
  padding: 10px 0 4px;
}

.nav-label {
  color: var(--color-text-muted);
  font-size: 10px;
  letter-spacing: .12em;
  text-transform: uppercase;
  padding: 0 16px 5px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 6px 16px;
  background: none;
  border: none;
  border-left: 2px solid transparent;
  cursor: pointer;
  color: var(--color-text-muted);
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  text-align: left;
  transition: background 100ms, color 100ms, border-color 100ms;
}

.nav-item:hover {
  background: var(--color-surface-alt);
  color: var(--color-text);
}

.nav-item.active {
  color: var(--color-accent-green);
  border-left-color: var(--color-accent-green);
  background: var(--color-surface-alt);
}

.nav-icon {
  width: 14px;
  text-align: center;
  flex-shrink: 0;
}

.sidebar-status {
  margin-top: auto;
  padding: 10px 16px;
  border-top: 1px solid var(--color-border);
}

.status-indicator {
  font-size: 10px;
}

.status-connected    { color: var(--color-accent-green); }
.status-disconnected { color: var(--color-danger); }

/* Right: main panel */
.main-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Breadcrumb */
.breadcrumb {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 6px 18px;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  font-size: var(--font-size-sm);
  flex-shrink: 0;
}

.bc-home    { color: var(--color-primary); }
.bc-sep     { color: var(--color-border); }
.bc-section { color: var(--color-text-muted); }
.bc-current { color: var(--color-text); }

#app-content {
  flex: 1;
  overflow: hidden;
}

#chat-container {
  flex: 1;
  overflow: hidden;
}

/* Bottom bar */
.bottom-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 4px 18px;
  background: var(--color-sidebar);
  border-top: 1px solid var(--color-border);
  font-size: 10px;
  flex-shrink: 0;
}

.bottom-hints {
  display: flex;
  gap: 12px;
  color: var(--color-text-muted);
}

.bottom-meta {
  color: var(--color-primary);
}

.key {
  display: inline-block;
  background: var(--color-border);
  color: var(--color-text);
  padding: 1px 5px;
  border-radius: 2px;
  font-size: 10px;
  margin-right: 3px;
}
```

- [ ] **Step 2: Rewrite render() in app-shell.ts**

Replace the entire `private render()` method in `src/components/app-shell.ts` with:

```typescript
  private render() {
    this.innerHTML = `
      <disclaimer-dialog></disclaimer-dialog>
      <div class="app-shell">
        <div class="titlebar">
          <div class="dot dot-red"></div>
          <div class="dot dot-yellow"></div>
          <div class="dot dot-green"></div>
          <span class="titlebar-path">psychly — ~/journal</span>
        </div>
        <div class="app-body">
          <nav class="app-sidebar">
            <div class="sidebar-brand">
              <div class="sidebar-brand-name">ψ psychly</div>
              <div class="sidebar-brand-sub">v0.1.0 · local</div>
            </div>
            <div class="nav-section">
              <div class="nav-label">journal</div>
              <button class="nav-item active" id="nav-journal">
                <span class="nav-icon">❯</span>entries
              </button>
              <button class="nav-item" id="nav-new-entry">
                <span class="nav-icon">✦</span>new entry
              </button>
              <button class="nav-item" id="nav-search">
                <span class="nav-icon">⌕</span>search
              </button>
            </div>
            <div class="nav-section">
              <div class="nav-label">therapy</div>
              <button class="nav-item" id="nav-chat">
                <span class="nav-icon">❯</span>new chat
              </button>
              <button class="nav-item" id="nav-chat-history">
                <span class="nav-icon">↳</span>history
              </button>
            </div>
            <div class="nav-section">
              <div class="nav-label">data</div>
              <button class="nav-item" id="nav-export">
                <span class="nav-icon">↑</span>export
              </button>
            </div>
            <div class="sidebar-status">
              <div id="ollama-status" class="status-indicator status-disconnected">○ ollama indisponible</div>
            </div>
          </nav>
          <div class="main-panel">
            <div class="breadcrumb" id="app-breadcrumb">
              <span class="bc-home">~</span>
              <span class="bc-sep">/</span>
              <span class="bc-section">journal</span>
              <span class="bc-sep">/</span>
              <span class="bc-current">entries</span>
            </div>
            <main id="app-content">
              <journal-list></journal-list>
            </main>
            <div id="chat-container" style="display:none;"></div>
            <div class="bottom-bar">
              <span class="bottom-hints" id="bottom-bar-hints"></span>
              <span class="bottom-meta" id="bottom-bar-meta"></span>
            </div>
          </div>
        </div>
      </div>
    `;

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
      if (document.querySelector("export-dialog")) return;
      const dialog = document.createElement("export-dialog");
      document.body.appendChild(dialog);
    });

    this.querySelector("#chat-container")?.addEventListener("close-chat", () => {
      const chatContainer = this.querySelector<HTMLElement>("#chat-container");
      if (chatContainer) {
        chatContainer.innerHTML = "";
        chatContainer.style.display = "none";
      }
      this.chatMounted = false;
      this.navigateTo({ view: "list" });
    });
  }
```

- [ ] **Step 3: Add updateActiveNav() method to app-shell.ts**

Add this method to the `AppShell` class (after `updateStatusIndicator()`):

```typescript
  private updateActiveNav(view: string) {
    this.querySelectorAll(".nav-item").forEach(el => el.classList.remove("active"));
    const viewToId: Record<string, string> = {
      list: "nav-journal",
      editor: "nav-new-entry",
      search: "nav-search",
      chat: "nav-chat",
      "chat-history": "nav-chat-history",
      entry: "nav-journal",
    };
    const id = viewToId[view];
    if (id) this.querySelector(`#${id}`)?.classList.add("active");
  }
```

- [ ] **Step 4: Add updateBreadcrumb() method to app-shell.ts**

Add this method to the `AppShell` class (after `updateActiveNav()`):

```typescript
  private updateBreadcrumb(view: string, hasId?: boolean) {
    const el = this.querySelector<HTMLElement>("#app-breadcrumb");
    if (!el) return;
    const paths: Record<string, string[]> = {
      list: ["journal", "entries"],
      editor: hasId ? ["journal", "entries", "edit"] : ["journal", "new-entry"],
      entry: ["journal", "entries", "view"],
      search: ["journal", "search"],
      chat: ["therapy", "chat"],
      "chat-history": ["therapy", "history"],
    };
    const segments = paths[view] ?? [view];
    el.innerHTML = `
      <span class="bc-home">~</span>
      ${segments.map((s, i) => `
        <span class="bc-sep">/</span>
        <span class="${i === segments.length - 1 ? "bc-current" : "bc-section"}">${s}</span>
      `).join("")}
    `;
  }
```

- [ ] **Step 5: Wire updateActiveNav() and updateBreadcrumb() into navigateTo()**

In `navigateTo()`, add calls at the start of the method, right after `this.currentView = detail.view;`:

```typescript
  private navigateTo(detail: NavigateDetail) {
    this.currentView = detail.view;
    this.updateActiveNav(detail.view);
    this.updateBreadcrumb(detail.view, !!detail.id);
    // ... rest of method unchanged
```

Also fix the chat-container toggle in `navigateTo()` — the current code uses `display: "block"` but the new layout uses flex. Replace the chat show/hide lines:

```typescript
    if (detail.view === "chat") {
      appContent.style.display = "none";
      chatContainer.style.display = "block";
      // ... rest unchanged
    }

    appContent.style.display = "";
    chatContainer.style.display = "none";
```

- [ ] **Step 6: Verify compilation**

```bash
npm run build
```

Expected: exits 0, no TypeScript errors.

- [ ] **Step 7: Smoke test in dev mode**

```bash
npm run dev
```

Open `http://localhost:1420`. Verify:
- Titlebar with 3 colored dots
- Left sidebar with journal/therapy/data sections
- Breadcrumb `~ / journal / entries` at top of content
- Clicking each sidebar item updates breadcrumb + active nav highlight
- Ollama status shows at bottom of sidebar
- Bottom bar visible

- [ ] **Step 8: Commit**

```bash
git add src/styles.css src/components/app-shell.ts
git commit -m "feat: replace top nav with terminal sidebar layout"
```

---

## Task 3: Component Styles Overhaul

**Files:**
- Modify: `src/styles.css` — all component sections below the layout section

- [ ] **Step 1: Replace shared button styles**

Find and replace the `SHARED BUTTONS` section in `src/styles.css`:

```css
/* ============================================================
   SHARED BUTTONS
   ============================================================ */
.btn-primary {
  background: var(--color-primary);
  color: var(--color-sidebar);
  border: none;
  border-radius: var(--radius-sm);
  padding: var(--space-sm) var(--space-md);
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  font-weight: 500;
  cursor: pointer;
  transition: background 100ms;
}

.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-primary:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-back {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-muted);
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  padding: 0;
  transition: color 100ms;
}

.btn-back:hover {
  color: var(--color-primary);
}

.btn-secondary {
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: var(--space-xs) var(--space-sm);
  cursor: pointer;
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  color: var(--color-text);
  transition: background 100ms, border-color 100ms;
}

.btn-secondary:hover {
  background: var(--color-surface-alt);
  border-color: var(--color-primary);
}

.btn-delete {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-muted);
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  padding: var(--space-xs);
  border-radius: var(--radius-sm);
  transition: color 100ms;
  flex-shrink: 0;
}

.btn-delete:hover {
  color: var(--color-danger);
}
```

- [ ] **Step 2: Replace empty/no-results state styles**

Find and replace the `EMPTY / NO RESULTS STATES` section:

```css
/* ============================================================
   EMPTY / NO RESULTS STATES
   ============================================================ */
.empty-state,
.journal-list-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  padding: var(--space-xl);
  color: var(--color-text-muted);
  font-size: var(--font-size-sm);
  text-align: center;
  height: 100%;
}

.empty-state p,
.journal-list-empty p {
  margin: 0;
  color: var(--color-text-muted);
}
```

- [ ] **Step 3: Replace chat view styles**

Find and replace the `CHAT VIEW` section:

```css
/* ============================================================
   CHAT VIEW
   ============================================================ */
.chat-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg);
}

.chat-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.chat-header h2 {
  margin: 0;
  font-size: var(--font-size-base);
  color: var(--color-text-muted);
  font-weight: 400;
}

.chat-thread {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-md);
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.chat-bubble {
  max-width: 82%;
  padding: var(--space-sm) var(--space-md);
  border-radius: var(--radius-sm);
  line-height: var(--line-height-body);
}

.bubble-user {
  align-self: flex-end;
  background: var(--color-user-bubble);
  color: var(--color-text);
  border-radius: var(--radius-sm);
}

.bubble-assistant {
  align-self: flex-start;
  background: var(--color-assistant-bubble);
  color: var(--color-text);
  border-left: 2px solid var(--color-primary);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}

.bubble-role {
  display: block;
  font-size: 10px;
  font-weight: 500;
  margin-bottom: var(--space-xs);
  color: var(--color-text-muted);
  text-transform: lowercase;
  letter-spacing: .06em;
}

.bubble-content {
  white-space: pre-wrap;
  word-break: break-word;
  font-size: var(--font-size-sm);
}

.chat-input-area {
  display: flex;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  flex-shrink: 0;
}

.chat-input-area textarea {
  flex: 1;
  resize: none;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: var(--space-sm);
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  color: var(--color-text);
  background: var(--color-sidebar);
  transition: border-color 100ms;
}

.chat-input-area textarea:focus {
  outline: none;
  border-color: var(--color-primary);
}
```

- [ ] **Step 4: Replace chat session list styles**

Find and replace the `CHAT SESSION LIST` section:

```css
/* ============================================================
   CHAT SESSION LIST
   ============================================================ */
.chat-session-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg);
}

.chat-sessions-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.chat-sessions-header h2 {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
  font-weight: 400;
}

.session-list {
  list-style: none;
  padding: var(--space-md);
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.session-item {
  padding: var(--space-sm) var(--space-md);
  background: transparent;
  border-left: 2px solid transparent;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: 2px;
  transition: background 100ms, border-color 100ms;
}

.session-item:hover {
  background: var(--color-surface-alt);
  border-left-color: var(--color-primary);
}

.session-date {
  font-size: var(--font-size-sm);
  color: var(--color-accent-orange);
}

.session-context {
  font-size: 10px;
  color: var(--color-text-muted);
}
```

- [ ] **Step 5: Replace export dialog styles**

Find and replace the `EXPORT / IMPORT PANEL` section:

```css
/* ============================================================
   EXPORT / IMPORT PANEL
   ============================================================ */
export-dialog {
  position: fixed;
  top: 52px;
  right: var(--space-md);
  z-index: 500;
  width: 340px;
}

.export-overlay {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--space-lg);
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.export-overlay h2 {
  margin: 0;
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--color-text);
}

.export-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.export-section h3 {
  margin: 0;
  font-size: 10px;
  font-weight: 500;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: .08em;
}

.export-section p {
  margin: 0;
  font-size: 10px;
  color: var(--color-text-muted);
}

.export-actions {
  display: flex;
  gap: var(--space-xs);
}

.export-feedback {
  font-size: 10px;
  min-height: 1.2em;
}

.feedback-success { color: var(--color-accent-green); }
.feedback-error   { color: var(--color-danger); }
```

- [ ] **Step 6: Replace disclaimer dialog styles**

Find and replace the `DISCLAIMER DIALOG` section:

```css
/* ============================================================
   DISCLAIMER DIALOG
   ============================================================ */
.disclaimer-overlay {
  position: fixed;
  inset: 0;
  background: rgba(25, 26, 33, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.disclaimer-dialog {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--space-xl);
  max-width: 500px;
  width: 100%;
  margin: var(--space-md);
}

.disclaimer-dialog h2 {
  margin-top: 0;
  font-size: var(--font-size-base);
  color: var(--color-primary);
}

.disclaimer-dialog p {
  line-height: var(--line-height-body);
  color: var(--color-text-muted);
  margin: 0 0 var(--space-md);
  font-size: var(--font-size-sm);
}
```

- [ ] **Step 7: Replace journal list styles**

Find and replace the `JOURNAL LIST` section:

```css
/* ============================================================
   JOURNAL LIST
   ============================================================ */
.journal-list {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg);
}

.journal-list-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.journal-list-header h2 {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
  font-weight: 400;
}

.entry-list {
  list-style: none;
  padding: var(--space-sm) 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0;
  overflow-y: auto;
}

.entry-item {
  display: flex;
  align-items: baseline;
  gap: var(--space-md);
  padding: var(--space-sm) var(--space-md);
  background: transparent;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: background 100ms, border-color 100ms;
}

.entry-item:hover {
  background: var(--color-surface-alt);
  border-left-color: var(--color-primary);
}

.entry-date {
  font-size: var(--font-size-sm);
  color: var(--color-accent-orange);
  white-space: nowrap;
  flex-shrink: 0;
}

.entry-preview {
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
  flex: 1;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
```

- [ ] **Step 8: Replace journal editor styles**

Find and replace the `JOURNAL EDITOR` section:

```css
/* ============================================================
   JOURNAL EDITOR
   ============================================================ */
journal-editor {
  display: block;
  height: 100%;
}

.journal-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg);
}

.editor-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.editor-header h2 {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
  font-weight: 400;
}

#entry-body {
  flex: 1;
  width: 100%;
  resize: none;
  border: none;
  padding: var(--space-lg);
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  color: var(--color-text);
  background: var(--color-bg);
  line-height: var(--line-height-body);
  box-sizing: border-box;
  caret-color: var(--color-primary);
}

#entry-body:focus {
  outline: none;
}

.editor-actions {
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: flex-end;
  gap: var(--space-sm);
  flex-shrink: 0;
}
```

- [ ] **Step 9: Replace journal entry view styles**

Find and replace the `JOURNAL ENTRY VIEW` section:

```css
/* ============================================================
   JOURNAL ENTRY VIEW
   ============================================================ */
.journal-entry-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg);
}

.entry-view-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;
}

.entry-view-actions {
  display: flex;
  gap: var(--space-sm);
}

.entry-body {
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  line-height: var(--line-height-body);
  color: var(--color-text);
  padding: var(--space-lg);
  overflow-y: auto;
  flex: 1;
  white-space: pre-wrap;
  word-break: break-word;
}
```

- [ ] **Step 10: Replace journal search styles**

Find and replace the `JOURNAL SEARCH` section:

```css
/* ============================================================
   JOURNAL SEARCH
   ============================================================ */
.journal-search {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg);
  padding: var(--space-md);
  gap: var(--space-md);
}

.journal-search input[type="text"] {
  width: 100%;
  padding: var(--space-sm) var(--space-md);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background: var(--color-sidebar);
  color: var(--color-text);
  font-family: var(--font-body);
  font-size: var(--font-size-sm);
  transition: border-color 100ms;
}

.journal-search input[type="text"]:focus {
  outline: none;
  border-color: var(--color-primary);
}

.search-results-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0;
}

.search-result-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: var(--space-sm) var(--space-md);
  background: transparent;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: background 100ms, border-color 100ms;
}

.search-result-item:hover {
  background: var(--color-surface-alt);
  border-left-color: var(--color-primary);
}

.no-results {
  text-align: center;
  color: var(--color-text-muted);
  padding: var(--space-xl);
  margin: 0;
  font-size: var(--font-size-sm);
}
```

- [ ] **Step 11: Verify build**

```bash
npm run build
```

Expected: exits 0.

- [ ] **Step 12: Visual verification in dev**

```bash
npm run dev
```

Check each view (`http://localhost:1420`):
- Journal list: orange dates, hover border-left highlight
- New entry: monospace editor, dark caret
- Entry view: readable monospace text
- Search: dark input field, result hover effect
- Chat: user bubble dark gray, assistant bubble with purple left border
- Chat history: session list with orange dates
- Disclaimer dialog on first launch: dark themed, purple heading

- [ ] **Step 13: Rust tests still pass**

```bash
cd src-tauri && cargo test
```

Expected: all tests pass (no backend changes).

- [ ] **Step 14: Commit**

```bash
git add src/styles.css
git commit -m "feat: restyle all components for terminal aesthetic"
```

---

## Task 4: Gitignore .superpowers/

**Files:**
- Modify: `.gitignore`

- [ ] **Step 1: Add .superpowers to gitignore**

Open `.gitignore` (create if missing). Add:

```
.superpowers/
```

- [ ] **Step 2: Commit**

```bash
git add .gitignore
git commit -m "chore: ignore .superpowers brainstorm artifacts"
```

---

## Done

After Task 4, the app has full terminal aesthetic:
- Dracula palette throughout
- JetBrains Mono everywhere
- Sidebar with Powerline icons, active highlight, Ollama status
- Breadcrumb navigation
- No shadows, sharp corners, border-based separators
- All components restyled consistently
