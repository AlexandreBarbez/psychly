## Context

Le projet Psychly est une application Tauri composée de deux couches :
- **Frontend** : TypeScript compilé avec `tsc` + bundlé avec Vite (`pnpm build`)
- **Backend** : Rust (`cargo`), Rust minimum `1.77.2`, avec des dépendances système (`rusqlite` bundled, `reqwest`)

Actuellement, il n'y a aucune CI. Les PRs peuvent introduire des régressions de compilation ou de tests sans être détectées.

## Goals / Non-Goals

**Goals:**
- Valider la compilation TypeScript sur chaque PR et push sur `main`
- Valider la compilation et les tests Rust sur chaque PR et push sur `main`
- Workflow simple, rapide à maintenir

**Non-Goals:**
- Build complet Tauri (bundle `.app`, `.dmg`, etc.) — coûteux et non nécessaire pour valider le code
- Publication ou déploiement automatique
- Tests end-to-end ou tests d'intégration Tauri
- Cache avancé ou matrix multi-plateforme (dans un premier temps)

## Decisions

### 1. Un seul workflow, deux jobs parallèles

**Décision :** Un fichier `.github/workflows/ci.yml` avec deux jobs indépendants : `frontend` et `backend`.

**Rationale :** Les deux couches sont indépendantes — pas besoin d'ordonner leur exécution. Les faire tourner en parallèle réduit le temps total de la CI.

Alternative écartée : un seul job séquentiel → plus lent, moins lisible.

### 2. Ne pas builder le binaire Tauri complet

**Décision :** Le job `frontend` exécute uniquement `tsc && vite build` (sans `tauri build`).

**Rationale :** `tauri build` requiert des dépendances système lourdes (GTK, WebKit, etc.) sur Linux et produit des artefacts binaires inutiles pour la validation de code. La compilation TypeScript seule suffit à détecter les erreurs de type et de bundling.

### 3. Rust : compiler le lib crate uniquement, pas le binaire Tauri

**Décision :** Le job `backend` exécute `cargo test` dans `src-tauri/`, ce qui compile `psychly_lib` et exécute les tests unitaires.

**Rationale :** La lib contient toute la logique métier (journal, analysis, therapy). Le binaire Tauri (`main.rs`) dépend de la couche native Tauri — difficile à compiler sans les dépendances système complètes.

Alternative : `cargo check` uniquement → écarte les tests, moins de valeur.

### 4. pnpm via `pnpm/action-setup`

**Décision :** Utiliser l'action officielle `pnpm/action-setup@v4` pour installer pnpm avant `actions/setup-node`.

**Rationale :** Le projet utilise `pnpm` (pas `npm` ni `yarn`). L'action officielle est la méthode recommandée et gère le cache automatiquement.

### 5. Rust toolchain via `dtolnay/rust-toolchain`

**Décision :** Utiliser `dtolnay/rust-toolchain@1.77.2` pour épingler la version Rust définie dans `Cargo.toml`.

**Rationale :** Garantit la reproductibilité. La version `1.77.2` est la version minimale déclarée dans `Cargo.toml`.

## Risks / Trade-offs

- **Dépendances système de rusqlite bundled** : `rusqlite` avec `features = ["bundled"]` compile SQLite depuis les sources — nécessite `gcc`/`clang` et `make` sur le runner. Ces outils sont disponibles sur `ubuntu-latest` par défaut → risque faible.

- **Temps de compilation Rust** : Sans cache, `cargo test` peut prendre plusieurs minutes. Mitigation : activer le cache Cargo via `Swatinem/rust-cache` pour accélérer les builds suivants.

- **Pas de tests TypeScript** : Le projet ne dispose pas encore de tests unitaires frontend. La CI valide uniquement la compilation — acceptable pour l'état actuel du projet.
