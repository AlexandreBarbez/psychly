## Why

Actuellement, l'IA thérapeutique n'a accès au contexte journal que de manière limitée : soit via l'entrée unique depuis laquelle le chat a été lancé (`journalContext` passé par le frontend), soit via un résumé agrégé des analyses récentes (tendances émotionnelles, thèmes, patterns cognitifs via `context_builder`). L'IA ne peut pas accéder au contenu réel des entrées de journal — elle ne voit que des métadonnées extraites par l'analyse.

L'utilisateur s'attend à ce que l'IA puisse lui parler concrètement de ce qu'il a écrit dans son journal : citer des passages, faire des liens entre des situations précises, et enrichir la conversation avec le vécu décrit. Cela nécessite d'injecter le contenu pertinent des entrées de journal dans le contexte du chat, tout en maîtrisant la taille du contexte pour ne pas saturer la fenêtre de tokens du LLM.

## What Changes

- Le backend récupère automatiquement les entrées de journal récentes et/ou pertinentes au moment de l'assemblage du prompt, au lieu de se reposer uniquement sur le `journal_context` passé par le frontend.
- Un mécanisme de sélection de contexte choisit les entrées les plus pertinentes pour la conversation en cours (récence + correspondance thématique via les analyses existantes).
- Le contenu des entrées sélectionnées est condensé et injecté dans le prompt système, avec un budget de tokens dédié pour éviter de saturer la fenêtre de contexte.
- Le frontend n'a plus besoin de passer manuellement le contexte journal — le backend le construit de façon autonome.

## Capabilities

### New Capabilities

- `journal-context-retrieval` : Mécanisme de sélection et d'injection des entrées de journal pertinentes dans le contexte du chat thérapeutique. Couvre la stratégie de sélection (récence, pertinence thématique), la condensation du contenu, et la gestion du budget de tokens.

### Modified Capabilities

- `therapeutic-chat` : Le prompt assembly intègre automatiquement le contenu des entrées de journal pertinentes, au lieu de se reposer sur un contexte optionnel passé par le frontend. Le système prompt et le context builder sont adaptés pour exploiter le contenu réel des entrées.

## Impact

- **Backend Rust** : `therapy::application::prompt_assembly` et `analysis::application::context_builder` — modification de la logique d'assemblage du contexte pour inclure le contenu des entrées.
- **Backend Rust** : `journal::domain::repository` — potentielle nouvelle interface de requête pour récupérer les entrées pertinentes par récence et thème.
- **Frontend** : `chat-view.ts` et `api/chat.ts` — simplification possible (le frontend n'a plus à fournir le `journalContext`).
- **Budget tokens** : Le `RESERVED_TOKENS` dans `prompt_assembly.rs` doit être recalibré pour accueillir le contenu des entrées de journal tout en laissant de la place à l'historique de conversation et à la réponse.
- **Aucune dépendance externe ajoutée** : la sélection de contexte s'appuie sur les analyses déjà stockées localement (pas de vector DB, pas de réseau).
