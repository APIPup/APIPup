# APIPup

Un outil de test API multiplateforme haute performance avec des performances natives de bureau.

Construit avec **Tauri** + **Rust** + **SvelteKit** pour une expérience rapide, légère et réactive.

Prend en charge **macOS** et **Windows**.

## Philosophie de conception

- **Pas de serveur, pas de stockage cloud** — Toutes les requêtes sont envoyées directement depuis la couche Rust, évitant les problèmes de sécurité
- **Stockage de données local** — Les données restent sur votre disque. Utilisez Git pour le contrôle de version et la collaboration en équipe
- **Convivial pour l'IA et la CLI** — Conçu pour l'automatisation et l'intégration avec les outils d'IA

## Performances

| Métrique | APIPup | Applications Electron typiques |
|----------|--------|-------------------------------|
| Taille du bundle | ~10MB | ~450MB |
| Vitesse de démarrage | Ultra-rapide | Plus lente |
| Temps pour 1000 requêtes | Rapide (Rust) | Plus lent |
| Mémoire de pointe (100 simultanées, 1000 au total) | 20–60MB | ~100–200MB |

## Fonctionnalités

- Envoyer des requêtes HTTP (GET, POST, PUT, DELETE, etc.)
- Voir les en-têtes et le corps de la réponse
- Support intégré de l'anglais, du chinois, du japonais, du français et de l'allemand
- Rapide et léger

## Pour les développeurs

### Architecture

```
ui/ (SvelteKit)  ──invoke()──>  tauri/ (Rust)
                              └── Requêtes HTTP
                              └── Retourne la réponse
```

- **Frontend** (`ui/`): SvelteKit avec Tailwind CSS
- **Backend** (`tauri/`): Tauri + moteur HTTP Rust
- **Communication**: Tauri `invoke()` — pas de problèmes CORS, pas de ports supplémentaires

### Prérequis

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- [Rust](https://rustup.rs/) >= 1.77
- Dépendances spécifiques à la plateforme:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), WebView2 (préinstallé sur Windows 10/11)

### Démarrage

```bash
# Cloner le dépôt
git clone <repo-url>
cd APIPup

# Installer les dépendances
pnpm install

# Démarrer le développement (lance le serveur de développement frontend et la fenêtre Tauri)
pnpm dev
```

`pnpm dev` va:
1. Démarrer le serveur de développement SvelteKit Vite sur `http://127.0.0.1:1420`
2. Compiler le backend Rust
3. Ouvrir la fenêtre de bureau APIPup

### Build pour la production

```bash
# Build .app
pnpm build

# Build .dmg (nécessite: brew install create-dmg)
pnpm build:dmg
```

La sortie se trouve dans `tauri/target/release/bundle/macos/`.

## Licence

MIT
