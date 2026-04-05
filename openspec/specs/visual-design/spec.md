## ADDED Requirements

### Requirement: Design token system
The application SHALL define all visual values (colors, typography, spacing, radii, shadows) as CSS custom properties on `:root` in `src/styles.css`. No hardcoded color, font, or spacing values SHALL appear in component selectors.

#### Scenario: All colors use tokens
- **WHEN** a developer inspects any CSS selector in `styles.css`
- **THEN** all color values reference a `--color-*` custom property, not a hex/rgb literal

#### Scenario: All spacing uses tokens
- **WHEN** a developer inspects any CSS selector in `styles.css`
- **THEN** all padding and margin values reference a `--space-*` custom property

#### Scenario: Dark mode preparation
- **WHEN** a `@media (prefers-color-scheme: dark)` block is added later
- **THEN** it SHALL only need to redefine `--color-*` tokens on `:root` to retheme the entire application

---

### Requirement: Calming blue colour palette
The application SHALL use a palette of desaturated, warm-cool blue tones. The following tokens SHALL be defined and used:

| Token | Required value range | Usage |
|---|---|---|
| `--color-bg` | Very pale blue-grey (`#e8f0f8` to `#f2f6fb`) | Page background |
| `--color-surface` | White or near-white | Cards, dialogs, panels |
| `--color-surface-alt` | Slightly tinted blue | Secondary surfaces |
| `--color-border` | Soft blue-grey | Borders and dividers |
| `--color-primary` | Muted slate blue | Primary actions, active states |
| `--color-primary-hover` | Darker shade of primary | Hover state on primary |
| `--color-user-bubble` | Same as or close to `--color-primary` | User chat bubbles |
| `--color-assistant-bubble` | Very light blue tint | Assistant chat bubbles |
| `--color-text` | Dark blue-night (`#1a2d40` range) | Body text |
| `--color-text-muted` | Mid-range blue-grey | Secondary/placeholder text |
| `--color-accent` | Soft sky blue | Indicators, highlights |

#### Scenario: Primary button uses palette
- **WHEN** the user views a primary button (`btn-primary`)
- **THEN** its background MUST be `--color-primary` and its text MUST contrast sufficiently (WCAG AA, minimum 4.5:1)

#### Scenario: Page background is calming
- **WHEN** the application loads
- **THEN** the `body` background SHALL use `--color-bg` (a pale blue, not pure white or pure grey)

#### Scenario: Chat bubbles are visually distinct
- **WHEN** a conversation is active in the chat view
- **THEN** user bubbles SHALL use `--color-user-bubble` and assistant bubbles SHALL use `--color-assistant-bubble`, clearly distinguishable from each other

---

### Requirement: Expressive typography
The application SHALL load Lora (serif, for headings) and Inter (sans-serif, for body) via Bunny Fonts. The following tokens SHALL be defined:

- `--font-heading`: `'Lora', Georgia, serif`
- `--font-body`: `'Inter', system-ui, sans-serif`
- `--font-size-base`: body reading size (~0.95rem)
- `--font-size-sm`: small labels (~0.85rem)
- `--font-size-lg`: section headers (~1.1rem)
- `--font-size-xl`: page-level titles (~1.3rem)
- `--line-height-body`: comfortable reading line height (≥1.6)

#### Scenario: Font link in HTML head
- **WHEN** `src/index.html` is served
- **THEN** a `<link>` to Bunny Fonts for Lora and Inter SHALL be present in `<head>`, with `font-display: swap`

#### Scenario: Headings use serif font
- **WHEN** the user views any `h1`, `h2` heading in the application
- **THEN** the rendered font SHALL be Lora (or its serif fallback)

#### Scenario: Body text uses sans-serif
- **WHEN** the user reads body text, labels, or button text
- **THEN** the rendered font SHALL be Inter (or its system-ui fallback)

---

### Requirement: Generous spacing and rounded corners
All interactive and content containers SHALL use spacious padding and generous border radii to create a calm, premium feel.

- Container padding SHALL use at minimum `--space-md` (≥1rem)
- Interactive elements (buttons, inputs) SHALL have a border-radius of at minimum `--radius-sm` (≥8px)
- Cards and panels SHALL have a border-radius of at minimum `--radius-md` (≥12px)
- Chat bubbles SHALL have a border-radius of at minimum `--radius-lg` (≥16px)

#### Scenario: Chat bubbles are rounded
- **WHEN** a message appears in the chat thread
- **THEN** the bubble SHALL have a border-radius of at least 16px on all corners

#### Scenario: Buttons are rounded
- **WHEN** the user views any button
- **THEN** its border-radius SHALL be at least 8px

---

### Requirement: Depth via shadows, not hard borders
Components that need visual separation SHALL use box-shadow instead of `border`. Hard 1px borders SHALL only be used where they denote functional separation (e.g., input fields, thread dividers).

Shadows SHALL be defined via tokens:
- `--shadow-sm`: subtle resting shadow for list items
- `--shadow-md`: medium shadow for cards and dialogs
- `--shadow-lg`: prominent shadow for modals/overlays

#### Scenario: Session list items use shadow
- **WHEN** the user views the chat or journal session list
- **THEN** each list item SHALL have a shadow (`--shadow-sm`) rather than a hard bottom border

#### Scenario: Disclaimer dialog uses shadow
- **WHEN** the disclaimer dialog is displayed
- **THEN** it SHALL use `--shadow-lg` and SHALL NOT have a visible border

---

### Requirement: Smooth transitions on interactive states
All hover, focus, and active state changes on interactive elements SHALL use a CSS transition of ≤200ms to feel smooth and premium.

#### Scenario: Button hover transition
- **WHEN** the user hovers over a button
- **THEN** the background colour change SHALL be animated with a transition of ≤200ms

#### Scenario: Input focus transition
- **WHEN** the user focuses a text input or textarea
- **THEN** the border or shadow change SHALL be animated with a transition of ≤200ms

---

### Requirement: Class name backward compatibility
All existing CSS class names used in `src/components/*.ts` SHALL remain valid. No class name SHALL be removed or renamed without a corresponding update to every TypeScript component that uses it.

#### Scenario: Existing components render without missing styles
- **WHEN** the application loads after the CSS redesign
- **THEN** no component SHALL render with unstyled (browser-default) appearance due to a missing class
