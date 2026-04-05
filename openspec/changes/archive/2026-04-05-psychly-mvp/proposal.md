## Why

Psychly est un journal intime personnel boosté à l'IA, conçu pour fonctionner entièrement en local sur un Mac M4 (24 Go RAM). L'objectif est de permettre à l'utilisateur de tenir un journal quotidien et d'interagir via un chat avec un agent IA formé en psychologie clinique — capable d'adopter une posture thérapeutique s'appuyant sur des cadres reconnus (ACT, CBT, DBT, Schema Therapy, Mindfulness, théorie de l'attachement, etc.). Aucune solution existante ne combine journal intime + chat thérapeutique + fonctionnement 100% local et portable.

## What Changes

- Création d'une application de journal intime avec persistance locale des entrées
- Ajout d'un chat conversationnel intégré avec un modèle LLM local jouant le rôle de thérapeute
- Analyse automatique des entrées du journal par l'IA pour contextualiser les échanges
- Stockage complet sur disque local (portable sur clé USB) sans dépendance réseau
- Architecture DDD avec documentation inline (markdown au plus près des composants)

## Capabilities

### New Capabilities

- `journal-entries`: Saisie, persistance et consultation des entrées du journal intime. L'utilisateur écrit sa journée, la sauvegarde, et peut la relire plus tard.
- `therapeutic-chat`: Chat conversationnel avec un agent IA local adoptant une posture de psychologue. L'agent utilise les cadres thérapeutiques (ACT, CBT, DBT, Schema Therapy, Mindfulness, etc.) pour guider l'échange.
- `journal-analysis`: Analyse des entrées du journal par l'IA pour enrichir le contexte des conversations thérapeutiques. L'agent a accès à l'historique du journal pour personnaliser ses réponses.
- `local-runtime`: Infrastructure d'exécution 100% locale — LLM local, stockage fichier, aucune dépendance réseau. L'ensemble doit être portable (clé USB) et fonctionner sur Mac M4 24 Go.

### Modified Capabilities

_(Aucune — projet greenfield, pas de specs existantes)_

## Impact

- **Code** : Création complète de l'application (frontend + backend + intégration LLM). Architecture DDD, chaque module documenté en markdown.
- **Dépendances** : Modèle LLM local (ex. llama.cpp / Ollama), framework UI desktop (ex. Tauri / Electron), base de données locale (ex. SQLite).
- **Système** : Fonctionne exclusivement sur macOS (Apple Silicon). Aucun service cloud. Toute la donnée reste locale et portable.
- **Sécurité** : Données sensibles (journal intime) stockées uniquement en local. Pas de transmission réseau.
