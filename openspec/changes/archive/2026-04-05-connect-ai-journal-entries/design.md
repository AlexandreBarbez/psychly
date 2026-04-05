## Context

L'IA thérapeutique dispose déjà d'un mécanisme de contexte journal (`context_builder.rs`) qui injecte un résumé agrégé des analyses (tendances émotionnelles, thèmes récurrents, patterns cognitifs) dans le prompt système. Le frontend peut aussi passer le body d'une entrée unique via `journalContext` quand le chat est lancé depuis une entrée.

Le problème : l'IA ne voit jamais le contenu réel des entrées. Elle ne peut pas citer ce que l'utilisateur a écrit, faire des liens entre situations concrètes, ou réagir au vécu décrit — seulement aux métadonnées extraites.

L'infrastructure existante offre :
- `JournalRepository::list(offset, limit)` et `search(query)` pour récupérer les entrées
- `AnalysisRepository::get_recent(limit)` pour les analyses avec `entry_id`, `emotional_tone`, `themes`, `patterns`
- `prompt_assembly.rs` avec un budget de tokens (`MAX_CONTEXT_TOKENS = 16_000`, `RESERVED_TOKENS = 4_000`) et un mécanisme de gestion de fenêtre de contexte existant
- `build_chat_context()` qui construit déjà un résumé pour le prompt

## Goals / Non-Goals

**Goals:**
- L'IA accède au contenu réel des entrées de journal récentes lors de chaque échange
- La sélection des entrées est pertinente (récence + correspondance thématique avec la conversation)
- Le contenu injecté respecte un budget de tokens pour ne pas saturer la fenêtre de contexte
- Le backend gère tout de façon autonome — le frontend n'a plus besoin de passer le contexte journal
- Aucune dépendance externe ajoutée (pas de vector DB, pas d'embeddings)

**Non-Goals:**
- Recherche sémantique par embeddings — on s'appuie sur la correspondance de thèmes via les analyses existantes
- Résumé automatique des entrées longues par le LLM (coût en inférence trop élevé par message)
- Modification de l'UI du chat (pas de visualisation des entrées injectées côté frontend)
- RAG complet avec chunking et scoring — la simplicité est prioritaire

## Decisions

### 1. Enrichir `build_chat_context()` plutôt que créer un nouveau module

**Choix** : Étendre `context_builder.rs` pour inclure le contenu des entrées en plus des métadonnées actuelles.

**Alternatives considérées** :
- Nouveau module `journal_context` séparé → duplication de la logique de connexion DB et du concept de "contexte pour le chat"
- Injecter les entrées comme messages utilisateur dans l'historique → pollue l'historique et casse la logique de gestion de fenêtre

**Rationale** : `build_chat_context()` est déjà le point unique de construction du contexte journal pour le prompt. L'enrichir maintient la cohésion et évite de disperser la logique.

### 2. Stratégie de sélection : récence + correspondance thématique

**Choix** : Sélectionner les entrées en deux passes :
1. **Toujours** : les N entrées les plus récentes (ex: 5 dernières) — elles reflètent l'état actuel de l'utilisateur
2. **En complément** : les entrées dont les thèmes d'analyse correspondent aux thèmes de la conversation en cours, limitées par le budget restant

Pour la passe thématique, on exploite les `themes` déjà extraits par l'analyse de chaque entrée et on les compare aux thèmes des entrées récentes ou au contenu du dernier message utilisateur (correspondance simple par mots-clés, pas d'embeddings).

**Alternatives considérées** :
- Récence seule → manque les entrées anciennes très pertinentes thématiquement
- FTS5 SQLite sur le contenu → plus lourd à mettre en place, overkill pour le MVP quand on a déjà les thèmes extraits
- Embeddings + cosine similarity → dépendance externe, complexité de maintenance

**Rationale** : Les analyses existent déjà pour chaque entrée. Les utiliser pour la correspondance thématique est quasi gratuit et ne nécessite aucune infrastructure supplémentaire.

### 3. Troncature par entrée avec budget de tokens

**Choix** : Allouer un budget de tokens dédié au contenu des entrées de journal (ex: `JOURNAL_CONTENT_TOKENS = 3_000`) pris sur les `RESERVED_TOKENS`. Chaque entrée est tronquée individuellement si elle dépasse sa part du budget (`budget / nombre_entrées`).

Structure du budget révisée :
- `MAX_CONTEXT_TOKENS = 16_000` (inchangé)
- `JOURNAL_CONTENT_BUDGET = 3_000` tokens pour le contenu des entrées
- `ANALYSIS_SUMMARY_BUDGET = 500` tokens pour les métadonnées (trends, themes, patterns — ce qui existe déjà)
- Le reste pour l'historique de conversation + réponse

**Alternatives considérées** :
- Pas de troncature, simplement limiter le nombre d'entrées → risque qu'une entrée très longue consomme tout le budget
- Résumé par LLM de chaque entrée → coût en inférence à chaque message, latence inacceptable

**Rationale** : La troncature est déterministe, instantanée, et prévisible. Les entrées les plus récentes ont priorité sur le budget.

### 4. Le backend construit le contexte, le frontend ne le passe plus

**Choix** : `send_message` appelle `build_chat_context(db)` côté backend au lieu de recevoir `journal_context` du frontend. Le paramètre `journal_context` dans `SendMessageInput` est conservé temporairement mais ignoré si le backend produit son propre contexte.

**Alternatives considérées** :
- Garder le frontend comme source → le frontend n'a pas accès aux analyses, il ne peut pas faire de sélection thématique
- Double contexte (frontend + backend) → confusion et duplication

**Rationale** : Le backend a accès à tout (entrées + analyses + historique de conversation). C'est le seul endroit où une sélection intelligente peut se faire.

### 5. Ajouter `get_recent_entries` au `JournalRepository`

**Choix** : Ajouter une méthode `get_recent(limit: usize) -> Result<Vec<JournalEntry>, String>` au trait `JournalRepository` et son implémentation SQLite. Ajouter aussi `get_by_ids(ids: &[&str]) -> Result<Vec<JournalEntry>, String>` pour récupérer les entrées par correspondance thématique (les `entry_id` viennent des analyses).

**Rationale** : `list(offset, limit)` existe déjà mais n'est pas optimisée pour l'usage "dernières N entrées" sans pagination. `get_by_ids` permet de récupérer les entrées identifiées par la passe thématique en un seul appel SQL.

## Risks / Trade-offs

- **[Correspondance thématique trop basique]** → La comparaison par thèmes extraits peut manquer des entrées pertinentes si l'analyse a mal extrait les thèmes. Mitigation : les entrées récentes sont toujours incluses indépendamment de la correspondance thématique.

- **[Troncature perd du contexte]** → Couper une entrée longue peut retirer des informations importantes. Mitigation : on tronque à la fin (les premiers paragraphes sont généralement les plus informatifs dans un journal), et on ajoute un marqueur `[...]` pour que l'IA sache que le texte est incomplet.

- **[Budget tokens trop petit pour des entrées longues]** → 3 000 tokens ≈ 12 000 caractères, ce qui permet ~5 entrées de taille moyenne. Si l'utilisateur écrit des entrées très longues, seules les premières lignes seront visibles. Mitigation : on pourra augmenter le budget si le modèle le supporte (Qwen 2.5 14B a 128K de contexte, on en utilise 16K).

- **[Performance de la passe thématique]** → Récupérer les analyses puis les entrées correspondantes ajoute des requêtes SQLite. Mitigation : SQLite local est très rapide, et on limite à 10 analyses + 5-10 entrées max.

- **[Backward compatibility frontend]** → Le paramètre `journal_context` du frontend devient inutile. Mitigation : on le conserve dans l'API mais le backend le déprécie silencieusement — pas de breaking change immédiat.
