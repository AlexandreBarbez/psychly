## 1. Workflow file

- [x] 1.1 Créer le fichier `.github/workflows/ci.yml`
- [x] 1.2 Définir les triggers `pull_request` (tous les branches) et `push` sur `main`
- [x] 1.3 Déclarer deux jobs parallèles : `frontend` et `backend`

## 2. Job frontend

- [x] 2.1 Configurer le runner `ubuntu-latest`
- [x] 2.2 Ajouter l'étape `actions/checkout@v4`
- [x] 2.3 Ajouter l'étape `pnpm/action-setup@v4` avec la version pnpm appropriée
- [x] 2.4 Ajouter l'étape `actions/setup-node@v4` avec la version Node du projet et le cache `pnpm`
- [x] 2.5 Ajouter l'étape `pnpm install --frozen-lockfile`
- [x] 2.6 Ajouter l'étape `pnpm build` (`tsc && vite build`)

## 3. Job backend

- [x] 3.1 Configurer le runner `ubuntu-latest`
- [x] 3.2 Ajouter l'étape `actions/checkout@v4`
- [x] 3.3 Ajouter l'étape `dtolnay/rust-toolchain@1.77.2`
- [x] 3.4 Ajouter l'étape `Swatinem/rust-cache@v2` avec `workspaces: src-tauri`
- [x] 3.5 Ajouter l'étape `cargo test` exécutée dans `src-tauri/`

## 4. Validation

- [ ] 4.1 Pousser la branche et ouvrir une PR pour vérifier que les deux jobs se déclenchent
- [ ] 4.2 Vérifier que le job `frontend` passe (compilation TypeScript OK)
- [ ] 4.3 Vérifier que le job `backend` passe (`cargo test` OK)
- [ ] 4.4 Vérifier que les deux jobs s'exécutent bien en parallèle
