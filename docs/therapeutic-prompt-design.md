# Choix de conception du système prompt thérapeutique

## Approche multi-cadre

Le system prompt intègre intentionnellement plusieurs approches thérapeutiques plutôt qu'une seule. Cette approche intégrative reflète la pratique clinique moderne où les thérapeutes puisent dans différents cadres selon le besoin du patient.

### Cadres inclus

| Cadre | Raison d'inclusion |
|-------|-------------------|
| **ACT** | Acceptation des émotions difficiles, engagement dans les valeurs |
| **TCC** | Identification des distorsions cognitives (le plus testé empiriquement) |
| **TCD** | Régulation émotionnelle, tolérance à la détresse |
| **Thérapie des Schémas** | Patterns profonds et récurrents |
| **Mindfulness** | Ancrage, observation sans jugement |
| **Théorie de l'attachement** | Patterns relationnels |
| **Mentalisation** | Comprendre ses propres états mentaux et ceux des autres |
| **Exposition/Évitement** | Identifier les comportements d'évitement |

## Posture thérapeutique

Le prompt définit une posture spécifique :

1. **Validation d'abord** — Toujours accueillir l'émotion avant d'analyser
2. **Honnêteté bienveillante** — Confronter quand nécessaire (pas d'acquiescement systématique)
3. **Questions ouvertes** — Favoriser la réflexion plutôt que le conseil
4. **Pas de diagnostic** — Jamais d'étiquette clinique
5. **Langue naturelle** — Français courant, pas de jargon psychiatrique

## Sécurité

- Détection de crise par mots-clés (regex simple, pas de ML)
- Réponse immédiate avec numéro 3114
- Disclaimer au premier lancement
- Rappels réguliers que l'IA n'est pas un thérapeute

## Choix du modèle : Qwen 2.5 14B Q5_K_M

- **Taille** : 14B paramètres — bon compromis qualité/mémoire pour 24 Go RAM
- **Quantification** : Q5_K_M — compromise qualité/taille supérieure au Q4
- **Qualité** : Excellente compréhension du français, meilleure que les modèles plus petits
- **Contexte** : 128K tokens natifs, suffisant pour de longues conversations
