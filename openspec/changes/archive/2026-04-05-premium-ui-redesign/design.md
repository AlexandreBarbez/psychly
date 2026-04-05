## Context

Psychly est une app de thérapie personnelle (Tauri + Web Components + Rust). Son UI actuelle est entièrement définie dans `src/styles.css` — un fichier CSS monolithique sans tokens ni système de design. Les composants (`src/components/*.ts`) émettent du HTML avec des classes CSS hardcodées ; aucun CSS-in-JS ni framework CSS n'est utilisé.

Le redesign doit rester 100% CSS vanilla (pas de framework) pour rester cohérent avec l'architecture existante et ne pas introduire de dépendance build supplémentaire.

## Goals / Non-Goals

**Goals:**
- Définir un système de tokens CSS (custom properties) pour la couleur, la typographie et l'espacement
- Produire une palette bleu ardoise/brume apaisante, intimiste, premium
- Introduire une typographie expressive (serif pour les titres, sans-serif douce pour le corps)
- Refondre tous les sélecteurs de composants dans `styles.css` pour utiliser les nouveaux tokens
- Garantir la compatibilité avec les noms de classes existants dans les composants TypeScript
- Préparer les variables pour un future dark mode (variables distinctes pour light/dark)

**Non-Goals:**
- Implémentation du dark mode dans ce changement
- Changement des composants TypeScript (logique, structure HTML, noms de classes)
- Changement des APIs Rust/Tauri ou de la logique métier
- Responsive / mobile-first (l'app est desktop Tauri)
- Remplacement par un framework CSS (Tailwind, UnoCSS, etc.)

## Decisions

### D1 — CSS Custom Properties comme unique source de vérité des tokens

**Décision** : Toutes les valeurs de design (couleurs, typographie, spacing, border-radius, ombres) sont définies comme variables CSS sur `:root`. Les sélecteurs n'utilisent jamais de valeurs hardcodées.

**Rationale** : Permet un dark mode trivial ultérieur (redéfinir les variables dans `@media (prefers-color-scheme: dark)`), maintient la cohérence, et documente le système de design dans le code lui-même.

**Alternative écartée** : Variables Sass/Less — introduit un outil de build supplémentaire sans bénéfice ici.

---

### D2 — Palette bleu ardoise / brume (cool-warm hybrid)

**Décision** : Palette principale centrée sur des bleus désaturés froids-chauds :

| Token | Valeur | Usage |
|---|---|---|
| `--color-bg` | `#f0f4f8` | Fond principal (bleu très pâle) |
| `--color-surface` | `#ffffff` | Cards, dialogs, surfaces |
| `--color-surface-alt` | `#e8eef5` | Surfaces légèrement teintées |
| `--color-border` | `#d0dae6` | Bordures douces |
| `--color-primary` | `#3d6b9e` | Bleu ardoise — actions primaires |
| `--color-primary-hover` | `#2f5580` | État hover |
| `--color-user-bubble` | `#3d6b9e` | Bulle utilisateur dans le chat |
| `--color-assistant-bubble` | `#edf2f7` | Bulle assistant |
| `--color-text` | `#1e2d3d` | Texte principal (bleu nuit) |
| `--color-text-muted` | `#5a7490` | Texte secondaire |
| `--color-accent` | `#7aa7c7` | Accents doux, indicateurs |

**Rationale** : Les bleus désaturés sont associés au calme, à la confiance et à l'introspection dans la littérature sur la psychologie des couleurs. Éviter le bleu iOS pur (#007aff) trop vibrant pour un contexte intimiste.

**Alternative écartée** : Palette lavande/mauve — plus féminisée, moins universelle.

---

### D3 — Typographie : Lora (serif) + Inter (sans-serif)

**Décision** : 
- **Lora** (Google Fonts) — serif humaniste pour les titres H1/H2 (journal, app name). Chaleureux, littéraire.
- **Inter** — sans-serif pour tout le corps de texte, labels, boutons. Excellente lisibilité à petite taille.

**Chargement** : `<link>` Bunny Fonts (RGPD-friendly, même catalogue que Google Fonts) dans `index.html`.

```html
<link rel="preconnect" href="https://fonts.bunny.net">
<link href="https://fonts.bunny.net/css?family=lora:400,600|inter:400,500,600&display=swap" rel="stylesheet">
```

**Tokens typographiques** :
```css
--font-heading: 'Lora', Georgia, serif;
--font-body: 'Inter', system-ui, sans-serif;
--font-size-base: 0.95rem;
--font-size-sm: 0.85rem;
--font-size-lg: 1.1rem;
--font-size-xl: 1.3rem;
--line-height-body: 1.65;
```

**Alternative écartée** : Utiliser uniquement la font système — ne donne pas l'effet "journal intime premium" voulu.

---

### D4 — Ombres portées à la place des bordures brutes

**Décision** : Les composants utilisent des ombres subtiles plutôt que des `border: 1px solid`. Les bordures restent uniquement là où elles séparent des flux (ex: input, thread).

```css
--shadow-sm: 0 1px 3px rgba(30, 45, 61, 0.08);
--shadow-md: 0 4px 16px rgba(30, 45, 61, 0.12);
--shadow-lg: 0 8px 32px rgba(30, 45, 61, 0.16);
```

**Rationale** : Les ombres donnent de la profondeur sans la dureté visuelle des bordures, ce qui renforce le sentiment de douceur et de premium.

---

### D5 — Spacing scale & border-radius généreux

```css
--space-xs: 0.375rem;
--space-sm: 0.625rem;
--space-md: 1rem;
--space-lg: 1.5rem;
--space-xl: 2.5rem;

--radius-sm: 8px;
--radius-md: 14px;
--radius-lg: 20px;
--radius-pill: 999px;
```

**Rationale** : Plus d'espace = moins de densité = sentiment de confort. Border-radius généreux = formes douces = registre émotionnel positif.

## Risks / Trade-offs

- **[Risque] Dépendance réseau pour les fonts Bunny Fonts** → Mitigation : `font-display: swap` pour ne jamais bloquer le rendu ; les fonts système en fallback sont acceptables.
- **[Risque] Noms de classes CSS non exhaustifs** → Mitigation : audit complet des classes dans `src/components/*.ts` avant de modifier `styles.css`, pour ne rien casser.
- **[Trade-off] Lora + Inter = 2 familles à charger** → Poids ~80KB. Acceptable pour une app desktop Tauri (pas de contrainte réseau mobile).
- **[Trade-off] Refonte monolithique de styles.css** → Pas de migration progressive possible vu l'architecture actuelle. Tout ou rien.
