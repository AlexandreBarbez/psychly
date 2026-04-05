## 1. Journal Repository — Nouvelles méthodes de requête

- [x] 1.1 Ajouter `get_recent(limit: usize) -> Result<Vec<JournalEntry>, String>` au trait `JournalRepository`
- [x] 1.2 Implémenter `get_recent` dans `SqliteJournalRepository` (SQL: `SELECT * FROM journal_entries ORDER BY created_at DESC LIMIT ?`)
- [x] 1.3 Ajouter `get_by_ids(ids: &[&str]) -> Result<Vec<JournalEntry>, String>` au trait `JournalRepository`
- [x] 1.4 Implémenter `get_by_ids` dans `SqliteJournalRepository` (SQL: `SELECT * FROM journal_entries WHERE id IN (...)`)
- [x] 1.5 Écrire les tests unitaires pour `get_recent` et `get_by_ids` (cas nominal, aucune entrée, IDs partiellement valides)

## 2. Sélection d'entrées — Récence + correspondance thématique

- [x] 2.1 Créer la fonction `select_recent_entries(db, limit) -> Vec<JournalEntry>` dans `context_builder.rs` qui récupère les N entrées les plus récentes via `JournalRepository::get_recent`
- [x] 2.2 Créer la fonction `select_thematic_entries(db, recent_entry_ids, limit) -> Vec<JournalEntry>` qui récupère les analyses récentes, extrait les thèmes des entrées récentes, cherche les analyses d'anciennes entrées partageant ces thèmes, puis récupère ces entrées via `get_by_ids` (dédupliquées par rapport aux entrées récentes)
- [x] 2.3 Écrire les tests unitaires pour la sélection récente (0 entrées, < 5 entrées, >= 5 entrées)
- [x] 2.4 Écrire les tests unitaires pour la sélection thématique (pas de match, match trouvé, déduplication avec entrées récentes)

## 3. Budget de tokens et troncature

- [x] 3.1 Ajouter la constante `JOURNAL_CONTENT_BUDGET = 3_000` dans `prompt_assembly.rs` et ajuster `RESERVED_TOKENS` en conséquence
- [x] 3.2 Créer la fonction `truncate_entries(entries: &[JournalEntry], budget_tokens: usize) -> Vec<(JournalEntry, String)>` qui alloue le budget par entrée (`budget / len`), tronque chaque entrée à sa part, et ajoute `[...]` aux entrées tronquées
- [x] 3.3 Écrire les tests unitaires pour la troncature (contenu dans le budget, dépassement, une seule entrée longue)

## 4. Formatage et injection dans le prompt

- [x] 4.1 Créer la fonction `format_journal_entries(entries: &[(JournalEntry, String)]) -> String` qui formate chaque entrée avec sa date comme header et son contenu (potentiellement tronqué), ordonnées chronologiquement (plus ancienne en premier)
- [x] 4.2 Modifier `build_chat_context(db)` pour appeler `select_recent_entries`, `select_thematic_entries`, `truncate_entries`, et `format_journal_entries`, puis ajouter le bloc de contenu au contexte existant (tendances + thèmes + patterns)
- [x] 4.3 Écrire les tests unitaires pour le formatage (ordre chronologique, format date + contenu, marqueur `[...]`)

## 5. Intégration dans le prompt assembly et le command handler

- [x] 5.1 Modifier `send_message` dans `commands.rs` pour appeler `build_chat_context(db)` côté backend au lieu de se reposer sur `input.journal_context`
- [x] 5.2 Conserver le champ `journal_context` dans `SendMessageInput` pour backward compatibility mais ne plus l'utiliser pour l'assemblage du prompt
- [x] 5.3 Vérifier que `assemble_prompt` reçoit le contexte enrichi (contenu des entrées + métadonnées) et l'injecte correctement dans le prompt système

## 6. Simplification frontend

- [x] 6.1 Retirer le passage de `journalContext` dans `chat-view.ts` lors de l'appel à `sendMessage` (passer `undefined` ou retirer le paramètre)
- [x] 6.2 Mettre à jour `api/chat.ts` pour rendre le paramètre `journalContext` optionnel avec valeur par défaut `null`

## 7. Tests d'intégration

- [x] 7.1 Écrire un test d'intégration vérifiant que `build_chat_context` retourne le contenu des entrées récentes quand des entrées et analyses existent
- [x] 7.2 Écrire un test d'intégration vérifiant que le contexte inclut des entrées thématiquement pertinentes en plus des récentes
- [x] 7.3 Écrire un test d'intégration vérifiant que le budget de tokens est respecté quand les entrées sont volumineuses
