## Why

Il n'existe aucune validation automatisée sur les pull requests : le code TypeScript et Rust peut casser sans que cela soit détecté avant la fusion. Mettre en place une CI via GitHub Actions garantit que chaque PR valide la compilation et les tests avant d'être mergée.

## What Changes

- Ajout d'un workflow GitHub Actions déclenché sur les pull requests (`pull_request`) et les pushes sur `main`
- La CI compile le frontend TypeScript (`tsc && vite build`)
- La CI compile et exécute les tests Rust (`cargo test`) sur le crate `psychly`
- Le workflow installe les dépendances Node.js (via `pnpm`) et la toolchain Rust appropriée

## Capabilities

### New Capabilities
- `ci-pipeline`: Workflow GitHub Actions qui valide la compilation TypeScript et les tests Rust sur chaque PR

### Modified Capabilities
<!-- Aucune spec existante n'est impactée — il s'agit d'une infrastructure pure, sans changement de comportement applicatif -->

## Impact

- Nouveau fichier `.github/workflows/ci.yml`
- Aucun changement applicatif (Rust, TypeScript, Tauri config)
- Dépendance sur les runners GitHub-hosted (`ubuntu-latest`)
- Rust version cible : `1.77.2` (définie dans `Cargo.toml`)
- Package manager Node.js : `pnpm`
