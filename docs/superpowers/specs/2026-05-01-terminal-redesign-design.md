# Psychly — Terminal Redesign Design Spec

**Date:** 2026-05-01  
**Status:** Approved  
**Approach:** CSS + minimal TS restructure (Approach A)

---

## Summary

Complete visual redesign of Psychly from soft therapy-app aesthetic to Mac terminal / zsh style. Black background, monospace font, Dracula color palette, left sidebar navigation, Powerline Unicode icons. No backend changes. No logic changes. Layout restructure in `app-shell.ts` + full CSS token replacement.

---

## Color Tokens (Dracula)

Replace all existing CSS variables in `src/styles.css`:

| Token | Value | Usage |
|---|---|---|
| `--color-bg` | `#282a36` | Main content background |
| `--color-surface` | `#21222c` | Breadcrumb, bottom bar |
| `--color-surface-alt` | `#2d2e3f` | Hover states, active items |
| `--color-sidebar` | `#191a21` | Sidebar, titlebar, status bar |
| `--color-border` | `#44475a` | All borders, separators |
| `--color-text` | `#f8f8f2` | Primary text |
| `--color-text-muted` | `#6272a4` | Labels, dates muted, comments |
| `--color-primary` | `#bd93f9` | Purple — brand, active accents, breadcrumb |
| `--color-accent-green` | `#50fa7b` | Active nav item, status OK, new-entry CTA |
| `--color-accent-cyan` | `#8be9fd` | Secondary accent (therapy section) |
| `--color-accent-orange` | `#ffb86c` | Entry dates, warnings |
| `--color-danger` | `#ff5555` | Delete, status error |
| `--color-user-bubble` | `#44475a` | Chat user bubble bg |
| `--color-assistant-bubble` | `#21222c` | Chat assistant bubble bg |

---

## Typography

Single font throughout: **JetBrains Mono** (Google Fonts).

```html
<!-- in src/index.html <head> -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;600&display=swap" rel="stylesheet">
```

CSS tokens:

```css
--font-body: 'JetBrains Mono', 'Courier New', monospace;
--font-heading: 'JetBrains Mono', 'Courier New', monospace;
--font-size-base: 0.8125rem;   /* 13px */
--font-size-sm: 0.6875rem;     /* 11px */
--font-size-lg: 0.875rem;      /* 14px */
--font-size-xl: 1rem;          /* 16px */
--line-height-body: 1.6;
```

Remove all `border-radius` pill/lg values — use sharp or minimal radius only:

```css
--radius-sm: 3px;
--radius-md: 4px;
--radius-lg: 6px;
--radius-pill: 3px;  /* neutralized */
```

---

## Layout Structure

### Current
```
app-shell (column)
  ├── <nav class="app-nav">          ← top horizontal bar
  └── <main id="app-content">
```

### New
```
app-shell (column)
  ├── .titlebar                      ← fake macOS traffic lights + path
  └── .app-body (row)
        ├── <nav class="app-sidebar"> ← left sidebar, 200px fixed
        └── .main-panel (column)
              ├── .breadcrumb        ← ~ / section / subsection
              ├── <main id="app-content">   ← journal views
              ├── <div id="chat-container"> ← chat view (display toggled)
              └── .bottom-bar        ← keybinding hints + metadata
```

### `app-shell.ts` changes
- `render()`: replace `<nav class="app-nav">` with `.titlebar` + `.app-sidebar`
- Sidebar sections: `journal` (entries, new entry, search) · `therapy` (new chat, history) · `data` (export)
- Breadcrumb updates on each `navigateTo()` call
- Bottom bar: static keybinding hints, right side shows contextual count (e.g. "5 entrées · mai 2026")
- Ollama status moves to bottom of sidebar (not top nav)

---

## Sidebar Nav Items

| Section | Icon | Label | View |
|---|---|---|---|
| journal | `❯` | entries | `list` |
| journal | `✦` | new entry | `editor` |
| journal | `⌕` | search | `search` |
| therapy | `❯` | new chat | `chat` |
| therapy | `↳` | history | `chat-history` |
| data | `↑` | export | _(opens dialog)_ |

Active item: `color: #50fa7b`, `border-left: 2px solid #50fa7b`, `background: #2d2e3f`.  
Inactive: `color: #6272a4`, transparent border.

---

## Component Style Changes

### Buttons
- `btn-primary`: `background: #bd93f9`, `color: #191a21`, `border-radius: 3px`, no shadow
- `btn-secondary`: `background: transparent`, `border: 1px solid #44475a`, `color: #f8f8f2`
- `btn-delete`: `color: #ff5555` on hover, no background fill
- Remove all `box-shadow` from buttons

### Entry list items
- Remove card shadow and rounded corners
- `border-left: 2px solid transparent` → `#bd93f9` on hover, `#50fa7b` on active
- Date column: `color: #ffb86c`
- Background hover: `#2d2e3f`

### Chat bubbles
- User: `background: #44475a`, `color: #f8f8f2`, `border-radius: 3px`, no pill shape
- Assistant: `background: #21222c`, `border-left: 2px solid #bd93f9`, no bubble shape
- Role label: `color: #6272a4`, small caps

### Inputs / Textareas
- `background: #191a21`, `border: 1px solid #44475a`, `color: #f8f8f2`, `border-radius: 3px`
- Focus: `border-color: #bd93f9`, no box-shadow glow

### Dialogs (disclaimer, export)
- `background: #21222c`, `border: 1px solid #44475a`, `border-radius: 6px`
- No backdrop blur softness — keep `backdrop-filter: none`

---

## Titlebar

Purely decorative fake macOS titlebar:

```html
<div class="titlebar">
  <div class="dot dot-red"></div>
  <div class="dot dot-yellow"></div>
  <div class="dot dot-green"></div>
  <span class="titlebar-path">psychly — ~/journal</span>
</div>
```

Dots: 12px circles, `#ff5555` / `#ffb86c` / `#50fa7b`. Not functional — no close/minimize behavior needed (Tauri handles the real window chrome).

---

## Breadcrumb

Updates dynamically in `navigateTo()`:

| View | Breadcrumb |
|---|---|
| `list` | `~ / journal / entries` |
| `editor` (new) | `~ / journal / new-entry` |
| `editor` (existing) | `~ / journal / entries / edit` |
| `entry` | `~ / journal / entries / view` |
| `search` | `~ / journal / search` |
| `chat` | `~ / therapy / chat` |
| `chat-history` | `~ / therapy / history` |

Separator: `<span class="sep">/</span>` in `#44475a`. Active segment: `#f8f8f2`. Preceding: `#6272a4`.

---

## Bottom Bar

Static keybinding hints, context-sensitive right side. Example for `list` view:

```
[n] new  [↵] open  [s] search  [e] export          5 entrées · mai 2026
```

Keys styled as `<span class="key">n</span>` — dark bg `#44475a`, light text.

Right-side metadata (e.g. "5 entrées · mai 2026") is **static hardcoded text per view** — no backend fetch. Deferred to a future iteration.

---

## Shadows & Borders

Remove all `box-shadow`. Use borders only:
- Sidebar right: `border-right: 1px solid #44475a`
- Breadcrumb bottom: `border-bottom: 1px solid #44475a`
- Bottom bar top: `border-top: 1px solid #44475a`
- Titlebar bottom: `border-bottom: 1px solid #44475a`

---

## Files Changed

| File | Change |
|---|---|
| `src/index.html` | Add JetBrains Mono font link |
| `src/styles.css` | Full token replacement, all component styles |
| `src/components/app-shell.ts` | New `render()` — titlebar, sidebar, breadcrumb, bottom bar |

No other files change. All component `.ts` files untouched (only CSS classes they reference will be re-styled).

---

## Out of Scope

- Keyboard navigation shortcuts (keybindings shown in bottom bar are decorative only)
- Tauri window chrome customization
- Dark/light mode toggle
- Animations beyond existing CSS transitions
