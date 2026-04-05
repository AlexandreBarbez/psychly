# Journal — Bounded Context

Gère le cycle de vie complet des entrées du journal intime.

## Responsabilités

- Création, lecture, modification et suppression d'entrées
- Recherche plein texte (FTS5) dans le contenu des entrées
- Listage chronologique avec aperçu

## Structure

- `domain/` — Entité `JournalEntry`, trait `JournalRepository`
- `application/` — Cas d'usage (CRUD, recherche), commandes Tauri IPC
- `infrastructure/` — Implémentation SQLite du repository
