# Export / Import Journal — Design Spec

**Date:** 2026-04-28
**Scope:** Journal entries only (no chat sessions, no analyses)

## Problème

Lors de réinstallations fréquentes en phase de développement, les entrées de journal sont perdues. De plus, les données stockées dans SQLite ne sont pas lisibles sans l'application si le projet s'arrête.

## Solution retenue

Export/import à la demande via dossier de fichiers Markdown (un par entrée). Couvre les deux besoins : récupération après réinstall et pérennité des données en format ouvert.

## Architecture

### Backend — nouveau module `src-tauri/src/export/`

```
src-tauri/src/export/
└── mod.rs   # 2 commandes Tauri + logique export/import + tests
```

Pas de découpage domain/application/infrastructure : la logique est trop simple pour justifier DDD complet.

Deux commandes Tauri enregistrées dans `lib.rs` :

```rust
export_journal(dest_dir: String) -> Result<usize, String>
import_journal(src_dir: String) -> Result<ImportResult, String>
```

```rust
pub struct ImportResult {
    pub inserted: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}
```

### Frontend

```
src/api/export.ts                # wrappers invoke()
src/components/export-dialog.ts  # modale export/import
```

`app-shell.ts` : bouton "💾 Export" ajouté dans la nav bar, ouvre `export-dialog`.

Sélection de dossier via `@tauri-apps/plugin-dialog` (`open({ directory: true })`). Requiert ajout dans `Cargo.toml` et `tauri.conf.json`.

## Format fichier

**Nom :** `YYYY-MM-DD_<8 premiers chars UUID>.md`
Exemple : `2026-04-28_a3f7c912.md`

**Contenu :**

```markdown
---
id: a3f7c912-1234-5678-abcd-ef0123456789
created_at: 2026-04-28T10:32:00
updated_at: 2026-04-28T14:15:00
---

Corps de l'entrée ici.
```

- Frontmatter délimité par `---`
- Body = tout ce qui suit le second `---` (whitespace trimmé)
- Fichiers lisibles et éditables dans n'importe quel éditeur de texte

## Data flow

### Export

1. Récupérer toutes les entrées via `SqliteJournalRepository`
2. Pour chaque entrée : formater en Markdown, écrire `{dest_dir}/{date}_{id[..8]}.md`
3. Retourner le nombre de fichiers écrits

### Import

1. Lire tous les `.md` du dossier sélectionné (non-récursif)
2. Pour chaque fichier : parser frontmatter → extraire `id`, `created_at`, `updated_at`, body
3. Si `id` déjà présent en DB → skip (pas d'écrasement)
4. `INSERT INTO journal_entries` — le trigger FTS5 existant réindexe automatiquement
5. Retourner `ImportResult { inserted, skipped, errors }`

### UI flow

- Clic "💾 Export" → modale `export-dialog` s'ouvre
- Deux actions dans la modale :
  - **Exporter** : file picker → dossier destination → feedback "N entrées exportées"
  - **Importer** : file picker → dossier source → feedback "N importées, N ignorées, N erreurs"

## Gestion des erreurs

**Fatales** (retournent `Err(String)`, opération annulée) :
- Dossier destination inaccessible en écriture
- Dossier source introuvable
- Échec DB lors de la lecture pour l'export

**Non-fatales** (dans `ImportResult.errors`, opération continue) :
- Fichier `.md` sans frontmatter valide
- Champs `id` ou `created_at` manquants dans le frontmatter
- Échec d'écriture d'un fichier individuel à l'export

## Tests

Dans `src-tauri/src/export/mod.rs` (tests Rust unitaires) :

- Export sur DB in-memory → vérifie fichiers créés + contenu frontmatter correct
- Import depuis dossier temporaire → vérifie `inserted` count
- Import avec ID dupliqué → vérifie `skipped` count incrémenté
- Import fichier `.md` malformé → vérifie `errors` non vide

## Dépendances à ajouter

- `tauri-plugin-dialog` dans `Cargo.toml`
- `@tauri-apps/plugin-dialog` dans `package.json`
- Permission `dialog:default` dans `tauri.conf.json` capabilities
