# Psychly

Journal intime personnel boosté à l'intelligence artificielle, avec assistant thérapeutique intégré. 100% local, 100% privé.

## Fonctionnalités

- **Journal intime** — Créer, éditer, supprimer des entrées. Recherche plein texte (FTS5).
- **Chat thérapeutique** — Conversations avec un assistant IA adopant une posture de psychologue (ACT, TCC, TCD, Thérapie des Schémas, Mindfulness, etc.).
- **Analyse automatique** — Chaque entrée est analysée pour extraire le ton émotionnel, les thèmes et les patterns cognitifs.
- **Détection de crise** — Redirection vers le 3114 et recommandation de consulter un professionnel en cas de contenu sensible.
- **Portabilité** — Toute l'application tient dans un répertoire copiable sur clé USB.

## Architecture

```
psychly/
├── src-tauri/src/         # Backend Rust (Tauri v2)
│   ├── db/                # SQLite (rusqlite, FTS5)
│   ├── journal/           # Bounded Context : Journal
│   │   ├── domain/        # JournalEntry, JournalRepository
│   │   ├── application/   # Commandes IPC Tauri
│   │   └── infrastructure/# SqliteJournalRepository
│   ├── therapy/           # Bounded Context : Thérapie
│   │   ├── domain/        # ChatSession, ChatMessage, ChatSessionRepository
│   │   ├── application/   # System prompt, prompt assembly, crisis detection, IPC
│   │   └── infrastructure/# SqliteChatSessionRepository, OllamaClient
│   └── analysis/          # Bounded Context : Analyse
│       ├── domain/        # EntryAnalysis, AnalysisRepository
│       ├── application/   # Pipeline, trends, context builder, IPC
│       └── infrastructure/# SqliteAnalysisRepository
├── src/                   # Frontend (Vanilla TypeScript + Web Components)
│   ├── components/        # app-shell, journal-*, chat-*, disclaimer
│   ├── api/               # Wrappers TypeScript pour les commandes IPC
│   └── styles.css
├── start.sh               # Lanceur portable
└── openspec/              # Spécifications OpenSpec
```

**DDD** — 3 bounded contexts (Journal, Therapy, Analysis), chacun avec domain / application / infrastructure.

## Stack technique

| Composant | Technologie |
|-----------|-------------|
| Desktop | Tauri v2 (Rust + WebView) |
| Frontend | Vanilla TypeScript, Web Components, Vite |
| Base de données | SQLite (rusqlite, bundled, FTS5) |
| LLM | Ollama + Qwen 2.5 14B (Q5_K_M) |
| Runtime LLM | Ollama (localhost:11434) |

## Prérequis

- macOS avec Apple Silicon (M1/M2/M3/M4)
- 24 Go de RAM recommandés (pour Qwen 2.5 14B)
- [Ollama](https://ollama.com) installé ou inclus dans le répertoire portable
- Node.js 18+ et Rust 1.70+

## Installation

```bash
# Cloner le projet
git clone <repo-url>
cd psychly

# Installer les dépendances frontend
npm install

# Télécharger le modèle LLM
ollama pull qwen2.5:14b-instruct-q5_K_M

# Lancer en développement
npm run tauri dev

# Installer en local en tant qu'app
npm run tauri build
Copie du .app dans le répertoire local Application
```

## Utilisation portable

```bash
# Lancer avec le script portable
./start.sh
```

Le script :
1. Démarre Ollama avec les chemins portables (`OLLAMA_MODELS`, `OLLAMA_HOST`)
2. Attend qu'Ollama soit prêt
3. Lance l'application Tauri
4. Arrête proprement Ollama à la fermeture

### Structure portable

```
psychly/
├── app/           # Application Tauri compilée (.app)
├── data/          # Base SQLite (psychly.db)
├── models/        # Modèles LLM (GGUF via Ollama)
├── ollama/        # Binaire Ollama (optionnel)
└── start.sh       # Lanceur
```

## Avertissement

Psychly est un outil d'accompagnement personnel. **Il ne remplace pas un professionnel de santé mentale.** En cas d'urgence, contactez le **3114** (numéro national de prévention du suicide, 24h/24, gratuit).
