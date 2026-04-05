# Therapy — Bounded Context

Gère le chat thérapeutique conversationnel avec l'IA locale.

## Responsabilités

- Sessions de chat et historique des messages
- Construction du prompt thérapeutique (system prompt + contexte journal + historique)
- Gestion de la fenêtre de contexte (résumé des anciens messages)
- Détection de crise et réponse de sécurité
- Streaming des réponses LLM vers le frontend

## Structure

- `domain/` — Entités `ChatSession`, `ChatMessage`, trait `ChatSessionRepository`
- `application/` — Cas d'usage (démarrer session, envoyer message), commandes Tauri IPC
- `infrastructure/` — Implémentation SQLite du repository, client Ollama
