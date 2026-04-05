## 1. Preparation & Audit

- [x] 1.1 Audit `src/styles.css` and list all selectors to confirm they map to the class names in `src/components/*.ts` (no orphans)
- [x] 1.2 Verify Bunny Fonts availability for `lora:400,600` and `inter:400,500,600`

## 2. HTML — Load Fonts

- [x] 2.1 Add `<link rel="preconnect" href="https://fonts.bunny.net">` to `<head>` in `src/index.html`
- [x] 2.2 Add `<link href="https://fonts.bunny.net/css?family=lora:400,600|inter:400,500,600&display=swap" rel="stylesheet">` to `<head>` in `src/index.html`

## 3. CSS — Design Token System (`:root`)

- [x] 3.1 Replace the existing `:root` block with the full token set: all `--color-*`, `--font-*`, `--font-size-*`, `--line-height-*`, `--space-*`, `--radius-*`, and `--shadow-*` custom properties per the design decisions
- [x] 3.2 Set `body` and `:root` font-family to `var(--font-body)` and background to `var(--color-bg)`

## 4. CSS — Global & Typography

- [x] 4.1 Apply `font-family: var(--font-heading)` to all `h1`, `h2` elements globally
- [x] 4.2 Apply `font-family: var(--font-body)` to `body` and `input`, `textarea`, `button` elements
- [x] 4.3 Set `line-height: var(--line-height-body)` on `body`

## 5. CSS — Navigation & App Shell

- [x] 5.1 Restyle `.app-shell` and `.app-nav` using `--color-surface`, `--shadow-sm`, and token-based padding/spacing (remove hardcoded values)
- [x] 5.2 Restyle `.nav-brand` with `--font-heading`, `--color-primary`, and `--font-size-lg`
- [x] 5.3 Restyle `.nav-btn` with token border-radius, hover using `--color-surface-alt`, transition ≤200ms
- [x] 5.4 Restyle `.status-indicator`, `.status-connected`, `.status-disconnected` with token colors

## 6. CSS — Buttons

- [x] 6.1 Restyle `.btn-primary` using `--color-primary`, `--color-surface`, `--radius-sm`, padding tokens, transition ≤200ms on hover/focus
- [x] 6.2 Restyle `.btn-primary:hover` using `--color-primary-hover`
- [x] 6.3 Restyle `.btn-secondary` using `--color-surface-alt`, `--color-border`, `--radius-sm`, transition ≤200ms
- [x] 6.4 Restyle `.btn-back` with `--color-primary` and transition ≤200ms
- [x] 6.5 Restyle `.btn-delete` with a muted red token or system red, transition ≤200ms

## 7. CSS — Chat View

- [x] 7.1 Restyle `.chat-view` background with `--color-bg`
- [x] 7.2 Restyle `.chat-header` using `--color-surface`, `--shadow-sm` (remove hard border), token padding
- [x] 7.3 Restyle `.chat-thread` background with `--color-bg`, token gap/padding
- [x] 7.4 Restyle `.chat-bubble` with `--radius-lg` (≥16px), token padding
- [x] 7.5 Restyle `.bubble-user` using `--color-user-bubble` background, white text
- [x] 7.6 Restyle `.bubble-assistant` using `--color-assistant-bubble` background, `--color-text`
- [x] 7.7 Restyle `.bubble-role` with `--font-size-sm`, `--color-text-muted`
- [x] 7.8 Restyle `.chat-input-area` using `--color-surface`, `--shadow-sm` (remove hard border-top), token padding
- [x] 7.9 Restyle the `textarea` in `.chat-input-area` with token border, `--radius-sm`, focus transition ≤200ms using `--color-primary`

## 8. CSS — Chat Session List

- [x] 8.1 Restyle `.chat-session-list` and `.chat-sessions-header` with `--color-surface`, token padding
- [x] 8.2 Restyle `.session-list` removing hard borders; use `--space-sm` gap
- [x] 8.3 Restyle `.session-item` with `--shadow-sm`, `--radius-md`, token padding, hover using `--color-surface-alt`, transition ≤200ms
- [x] 8.4 Restyle `.session-date` with `--font-body`, `--color-text`, `font-weight: 500`
- [x] 8.5 Restyle `.session-context` with `--font-size-sm`, `--color-text-muted`

## 9. CSS — Journal List

- [x] 9.1 Restyle `.journal-list` and `.journal-list-header` with `--color-surface`, token padding, `--shadow-sm`
- [x] 9.2 Restyle `.entry-list` removing hard borders; use token gap
- [x] 9.3 Restyle `.entry-item` with `--shadow-sm`, `--radius-md`, `--color-surface`, token padding, hover transition ≤200ms
- [x] 9.4 Restyle `.entry-date` with `--font-size-sm`, `--color-text-muted`
- [x] 9.5 Restyle `.entry-preview` with `--font-size-sm`, `--color-text-muted`, line-clamp or truncation
- [x] 9.6 Restyle `.journal-list-empty` and `.empty-state` with `--color-text-muted`, `--font-size-lg`, centered token padding

## 10. CSS — Journal Editor

- [x] 10.1 Restyle `.journal-editor` container with `--color-bg`
- [x] 10.2 Restyle `.editor-header` with `--color-surface`, `--shadow-sm` (remove hard border), token padding
- [x] 10.3 Restyle `.editor-actions` with token gap/spacing
- [x] 10.4 Restyle the editor `textarea` (if present) with `--font-body`, `--font-size-base`, `--color-text`, `--color-surface`, no hard border on focus, transition ≤200ms

## 11. CSS — Journal Entry View

- [x] 11.1 Restyle `.journal-entry-view` with `--color-bg`, token padding
- [x] 11.2 Restyle `.entry-view-header` with `--color-surface`, `--shadow-sm`, token padding
- [x] 11.3 Restyle `.entry-view-actions` with token spacing
- [x] 11.4 Restyle `.entry-body` with `--font-body`, `--line-height-body`, `--color-text`, token padding

## 12. CSS — Journal Search

- [x] 12.1 Restyle `.journal-search` container with `--color-bg`, token padding
- [x] 12.2 Restyle the search input with `--radius-sm`, `--color-border`, focus transition ≤200ms using `--color-primary`
- [x] 12.3 Restyle `.search-results-list` with token gap
- [x] 12.4 Restyle `.search-result-item` with `--shadow-sm`, `--radius-md`, token padding, hover transition ≤200ms
- [x] 12.5 Restyle `.no-results` with `--color-text-muted`, centered layout

## 13. CSS — Disclaimer Dialog

- [x] 13.1 Restyle `.disclaimer-overlay` backdrop with `rgba` using `--color-text` base
- [x] 13.2 Restyle `.disclaimer-dialog` with `--color-surface`, `--radius-lg`, `--shadow-lg` (remove hard border), generous token padding
- [x] 13.3 Apply `--font-heading` to the dialog `h2`
- [x] 13.4 Apply `--color-text-muted` to dialog paragraphs

## 14. Verification

- [x] 14.1 Confirm no hardcoded hex/rgb/rem values remain in component selectors in `styles.css` (all use tokens)
- [ ] 14.2 Build and launch the app (`npm run tauri dev` or equivalent); visually verify all views render correctly with the new design
- [x] 14.3 Verify each class from the component audit (steps 1.1) has a restyled rule in `styles.css`
- [x] 14.4 Check WCAG AA contrast ratio (≥4.5:1) for `--color-primary` button text and `--color-text` on `--color-bg`
