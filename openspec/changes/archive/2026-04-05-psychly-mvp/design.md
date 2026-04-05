## Context

Psychly est un projet greenfield — une application de journal intime avec chat thérapeutique IA, fonctionnant 100% en local sur macOS Apple Silicon (M4, 24 Go RAM). L'application doit être entièrement portable (installable depuis une clé USB), sans aucune dépendance réseau. L'architecture suit les principes DDD avec documentation markdown intégrée au plus près du code.

L'utilisateur est unique (usage personnel). Il n'y a pas besoin d'authentification, de multi-tenancy, ni de gestion d'accès concurrents. La contrainte principale est la portabilité totale et l'exécution locale du LLM.

## Goals / Non-Goals

**Goals:**
- Application desktop native macOS fonctionnelle en local sans internet
- Journal intime avec CRUD complet et persistance durable
- Chat thérapeutique conversationnel utilisant un LLM local
- Analyse contextuelle du journal pour personnaliser les réponses du chat
- Portabilité complète sur clé USB (données + runtime + modèle)
- Architecture DDD avec documentation inline

**Non-Goals:**
- Multi-utilisateur ou authentification
- Déploiement cloud ou synchronisation réseau
- Support d'autres OS que macOS (Apple Silicon)
- Interface mobile
- Remplacement d'un véritable suivi thérapeutique professionnel
- Fine-tuning d'un modèle — on utilise un modèle pré-entraîné avec du prompt engineering

## Decisions

### 1. Framework UI : Tauri v2

**Choix** : Tauri v2 avec frontend web (HTML/CSS/JS ou framework léger)

**Pourquoi** :
- Bundle natif macOS léger (~10-15 Mo vs ~150+ Mo pour Electron)
- Utilise WebView natif (WebKit sur macOS), pas de Chromium embarqué
- Backend en Rust = faible empreinte mémoire, critique quand le LLM consomme déjà une part significative des 24 Go
- Excellent support Apple Silicon natif
- Portable : un seul binaire + dossier de données

**Alternatives considérées** :
- *Electron* : trop lourd en RAM et en taille de bundle, problématique pour la portabilité
- *SwiftUI natif* : bon rendu macOS mais couple au toolchain Apple, moins portable et plus complexe à maintenir

### 2. Frontend : Vanilla TypeScript + Web Components

**Choix** : TypeScript pur avec Web Components natifs, sans framework lourd

**Pourquoi** :
- Application simple (journal + chat), pas besoin de React/Vue/Svelte
- Zéro dépendance framework = portabilité maximale et bundle minimal
- Web Components standards, supportés nativement par WebKit
- Facilite la maintenance long terme (pas de breaking changes de framework)

**Alternatives considérées** :
- *React/Vue/Svelte* : surcharge pour une app single-user avec 2-3 vues
- *Lit* : intéressant mais ajoute une dépendance pour un gain limité ici

### 3. Persistance : SQLite via rusqlite

**Choix** : SQLite embarqué, accédé depuis le backend Rust via `rusqlite`

**Pourquoi** :
- Fichier unique `.db` — parfait pour la portabilité clé USB
- Zéro serveur de base de données, zéro configuration
- Performant pour un usage single-user (pas de contention)
- Supporte les requêtes textuelles (FTS5) pour la recherche dans le journal
- `rusqlite` est la référence Rust pour SQLite, bien maintenue

**Alternatives considérées** :
- *Fichiers JSON/Markdown* : simple mais pas de requêtes structurées, FTS complexe
- *DuckDB* : orienté analytique, surdimensionné pour ce cas d'usage
- *Sled/RocksDB* : key-value store, moins adapté pour des requêtes relationnelles

### 4. LLM local : Ollama

**Choix** : Ollama comme runtime LLM local avec Qwen 2.5 14B (Q5_K_M)

**Pourquoi** :
- Installation simple sur macOS, support natif Apple Silicon (Metal)
- Gestion automatique des modèles (téléchargement, cache, quantization)
- API HTTP locale compatible OpenAI — facilite l'intégration
- Modèle recommandé : **Qwen 2.5 14B (Q5_K_M)** — excellent en français, capacités d'analyse et de raisonnement supérieures aux modèles 7-8B, bien adapté au dialogue thérapeutique nuancé
- Un modèle 14B quantifié Q5 consomme ~10-12 Go de RAM, confortable sur 24 Go M4
- La priorité est la qualité d'analyse et de réponse, pas la taille sur disque

**Alternatives considérées** :
- *Mistral 7B / Llama 3.1 8B* : plus légers mais trop généralistes, qualité insuffisante pour des échanges thérapeutiques nuancés en français
- *llama.cpp directement* : plus léger mais gestion manuelle des modèles, API moins ergonomique
- *LM Studio* : UI-oriented, moins scriptable et portable
- *MLX* : natif Apple mais écosystème plus jeune, moins de modèles disponibles

### 5. Communication Tauri ↔ LLM : API interne Rust

**Choix** : Le backend Rust gère directement les appels HTTP vers Ollama (localhost). Le frontend communique avec le backend Rust via les commandes Tauri (IPC).

**Pourquoi** :
- Flux : Frontend → Tauri IPC → Rust backend → HTTP localhost → Ollama
- Le backend Rust orchestre le prompt engineering (injection du contexte journal, system prompt thérapeutique)
- Pas d'exposition d'API au-delà de localhost
- Le streaming des réponses LLM passe par les événements Tauri (Server-Sent Events côté Rust)

### 6. Architecture DDD : Bounded Contexts

**Choix** : 3 bounded contexts dans le backend Rust

| Context | Responsabilité |
|---------|---------------|
| `journal` | Entrées du journal (CRUD, recherche FTS) |
| `therapy` | Chat thérapeutique, gestion des sessions, prompt engineering |
| `analysis` | Analyse des entrées, extraction de contexte pour le chat |

Chaque context a sa propre structure `domain/`, `application/`, `infrastructure/` avec un `README.md` expliquant son rôle.

**Pourquoi** :
- Séparation claire des responsabilités
- Le context `analysis` sert de pont entre `journal` et `therapy`
- Facilite l'évolution indépendante de chaque domaine

### 7. Portabilité : Structure de répertoire autonome

**Choix** : Tout est contenu dans un répertoire racine unique

```
psychly/
├── app/                  # Binaire Tauri
├── data/                 # SQLite DB + fichiers
├── models/               # Modèles LLM (Ollama GGUF)
├── ollama/               # Runtime Ollama portable
└── start.sh              # Script de lancement
```

**Pourquoi** :
- Copie d'un seul dossier sur clé USB = installation complète
- `start.sh` lance Ollama puis l'application
- Aucun chemin absolu, tout est relatif au répertoire racine
- Les données et le modèle voyagent ensemble

## Risks / Trade-offs

- **Taille du modèle LLM** (~10 Go pour un 14B Q5) → La taille sur disque n'est pas une contrainte. La priorité est la qualité des réponses.

- **Qualité des réponses thérapeutiques en français** → Qwen 2.5 14B offre une bien meilleure maîtrise du français et des capacités de raisonnement supérieures aux modèles 7-8B. Combiné à un prompt engineering soigné, le résultat attendu est significativement meilleur pour le dialogue thérapeutique.

- **RAM partagée entre app et LLM** → Sur 24 Go, Ollama avec Qwen 2.5 14B Q5 consomme ~10-12 Go. L'application Tauri est légère (~100-200 Mo). Il reste ~12 Go pour le système. Mitigation : monitoring de la mémoire, possibilité de descendre en Q4 si nécessaire.

- **Dépendance à Ollama** → Si Ollama cesse d'être maintenu, migration possible vers llama.cpp directement (même format GGUF). L'abstraction via HTTP locale facilite le remplacement.

- **Portabilité Ollama sur clé USB** → Ollama utilise un répertoire home configurable (`OLLAMA_MODELS`). Script de lancement qui configure les variables d'environnement pour pointer vers le dossier portable.

- **Rust learning curve** → Le backend Rust est plus complexe à écrire qu'un backend Node.js/Python mais offre performance et sécurité mémoire. Trade-off accepté pour la qualité du runtime.

## Open Questions

- Valider la consommation mémoire effective de Qwen 2.5 14B Q5_K_M sur M4 24 Go avec Ollama en conditions réelles.
- Faut-il chiffrer la base SQLite (SQLCipher) pour protéger le journal intime, ou la sécurité du disque macOS (FileVault) suffit-elle ?
- Format du system prompt thérapeutique : définir les cadres à activer par défaut vs ceux activables à la demande par l'utilisateur.
