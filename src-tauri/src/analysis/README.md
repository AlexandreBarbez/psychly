# Analysis — Bounded Context

Analyse les entrées du journal pour enrichir le contexte des conversations thérapeutiques.

## Responsabilités

- Analyse automatique des entrées (ton émotionnel, thèmes, patterns cognitifs)
- Suivi des tendances émotionnelles dans le temps
- Construction du résumé contextuel pour injection dans le prompt de chat
- Exécution asynchrone sans bloquer l'interface

## Structure

- `domain/` — Entité `EntryAnalysis`, trait `AnalysisRepository`
- `application/` — Pipeline d'analyse, agrégation de tendances
- `infrastructure/` — Implémentation SQLite du repository, appels Ollama pour l'analyse
