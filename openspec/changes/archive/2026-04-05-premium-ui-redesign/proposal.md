## Why

L'interface actuelle de Psychly est fonctionnelle mais manque de personnalité : palette neutre (#fafafa, iOS blue), aucune typographie expressive, mise en page générique. Pour une app de bien-être mental, l'esthétique est un vecteur de confiance — l'utilisateur doit se sentir en sécurité et bienveillant dès la première seconde, comme en ouvrant son journal intime. Un design premium et intimiste renforce aussi la valeur perçue de l'application.

## What Changes

- **Palette de couleurs** : remplacement des neutres froids et du bleu iOS par une palette de bleus désaturés, chaleureux et apaisants (teintes ardoise, brume, indigo doux) avec des surfaces légèrement teintées plutôt que blanc pur / gris pur.
- **Typographie** : remplacement de la font système générique par une police serif élégante pour les titres (intimité, journal) et une sans-serif douce pour le corps (lisibilité).
- **Espacements & arrondis** : augmentation des paddings, border-radius plus généreux, moins de densité visuelle pour une respiration accrue.
- **Composants** : refonte des bulles de chat, des cartes de journal, de la navigation, des boutons et des états vides pour un registre premium & doux.
- **Effets visuels** : ombres subtiles (pas de border bruts), dégradés discrets sur les surfaces principales, transitions douces sur hover/focus.
- **Mode sombre** : les variables CSS seront prêtes pour un dark mode ultérieur (non implémenté dans ce changement).

## Capabilities

### New Capabilities
- `visual-design`: Système de design Psychly — tokens de couleur, typographie, espacements, et guidelines visuels pour tous les composants UI. Couvre les règles de style applicables à `styles.css` et aux composants web.

### Modified Capabilities
<!-- Aucun changement de comportement fonctionnel — uniquement le rendu visuel. Les specs existantes (journal-entries, therapeutic-chat, journal-analysis, local-runtime) ne sont pas affectées au niveau des exigences. -->

## Impact

- **`src/styles.css`** : fichier principal à refondre intégralement — variables CSS globales (tokens), tous les sélecteurs de composants.
- **`src/index.html`** : possible ajout d'un lien vers une font Google/Bunny Fonts (si approuvé).
- **`src/components/*.ts`** : vérification que les noms de classes CSS existants sont préservés OU mis à jour de manière cohérente (aucun changement de logique JS).
- Aucun changement aux APIs Rust/Tauri ni à la logique métier.
