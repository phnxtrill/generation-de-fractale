## Carré de Cantor – Fractale Interactive

### Description
Application Rust dédiée à la génération et à l’exploration interactive du carré de Cantor. L’utilisateur peut zoomer, se déplacer, réinitialiser la vue et bénéficier d’un niveau de détail adapté au zoom. L’évolution du projet suit trois jalons (statique, GIF animé, interface temps réel) visibles dans l’historique Git fourni via le bundle.

### Fonctionnalités
- Génération récursive du carré de Cantor
- Affichage temps réel avec `minifb`
- Zoom fluide à la molette
- Déplacements clavier
- Touche `R` pour réinitialiser la vue
- Nombre d’itérations ajusté dynamiquement
- Culling pour ignorer les zones hors écran
- Couleurs personnalisables
- Historique complet des versions

### Commandes
| Action                 | Touche            |
|------------------------|-------------------|
| Zoom avant             | Molette haut      |
| Zoom arrière           | Molette bas       |
| Déplacement haut       | `Z` ou `↑`        |
| Déplacement bas        | `S` ou `↓`        |
| Déplacement gauche     | `Q` ou `←`        |
| Déplacement droite     | `D` ou `→`        |
| Réinitialisation       | `R`               |
| Quitter                | `Échap`           |

### Dépendances
`Cargo.toml` :
```toml
[dependencies]
image = "0.24"
minifb = "0.25"
```

### Compilation et exécution
```bash
cargo build
cargo run
```

### Organisation
- `src/main.rs` : version finale avec interface graphique interactive.
- Historique Git : versions précédente (fractale statique, GIF animé, GIF infini, interface progressive).

### Optimisations
- Culling pour éviter les rendus hors écran.
- Arrêt de récursion quand un carré devient plus petit qu’un pixel.
- Itérations adaptatives selon le niveau de zoom.
- Interpolation du zoom pour conserver une animation fluide.

### Objectifs pédagogiques
Le projet illustre la récursivité, la modélisation de fractales, la manipulation d’images, l’optimisation graphique, la gestion d’une caméra virtuelle, l’interaction utilisateur et l’utilisation de Git pour le suivi de versions.

### Rendu
- Git bundle contenant l’intégralité de l’historique.
- Code final fonctionnel retraçant les étapes (statique → GIF → interactif).

### Auteurs
- Quentin Retory
- Thimothée Drapin
- Steven Pajoul

Projet réalisé en Rust dans le cadre d’un TP sur la génération de fractales.

