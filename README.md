# APIPup

A lightweight, cross-platform API testing tool.

Built with **Tauri 2** + **Rust** (reqwest) + **SvelteKit** (SPA) + **Tailwind CSS v4**.

Supports **macOS** and **Windows**.

## Architecture

```
ui/ (SvelteKit SPA)  ──invoke()──>  tauri/ (Rust)
                                      └── reqwest sends HTTP
                                      └── returns response
```

- **Frontend** (`ui/`): SvelteKit in SPA mode with Tailwind CSS v4, Svelte 5 runes
- **Backend** (`tauri/`): Tauri 2 desktop shell + Rust HTTP engine via reqwest
- **Communication**: Tauri `invoke()` — no CORS issues, no extra ports
- **i18n**: Built-in Chinese (zh) and English (en) support

## Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- [Rust](https://rustup.rs/) >= 1.77
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), WebView2 (pre-installed on Windows 10/11)

## Getting Started

```bash
# Clone the repo
git clone <repo-url>
cd APIPup

# Install dependencies
pnpm install

# Start development (launches both frontend dev server and Tauri window)
pnpm dev
```

`pnpm dev` will:
1. Start the SvelteKit Vite dev server on `http://127.0.0.1:1420`
2. Compile the Rust backend
3. Open the APIPup desktop window

## Build for Production

```bash
# Build .app
pnpm build

# Build .dmg (requires: brew install create-dmg)
pnpm build:dmg
```

The output is at `tauri/target/release/bundle/macos/`.

## Project Structure

```
APIPup/
├── tauri/                        # Tauri 2 + Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json           # App config (window, bundle, build commands)
│   ├── capabilities/             # Tauri permission capabilities
│   └── src/
│       ├── main.rs               # Entry point
│       ├── lib.rs                # Tauri builder + command registration
│       └── commands/
│           └── http.rs           # send_request command (reqwest)
├── ui/                           # SvelteKit SPA frontend
│   ├── svelte.config.js          # adapter-static (SPA mode)
│   ├── vite.config.ts            # Vite + Tailwind v4 plugin
│   └── src/
│       ├── app.css               # Tailwind v4 theme
│       ├── lib/
│       │   ├── i18n/             # i18n (zh / en)
│       │   ├── api/http.ts       # Tauri invoke wrapper
│       │   ├── stores/           # Svelte stores (request state)
│       │   └── components/       # UI components
│       └── routes/               # SvelteKit pages
├── package.json                  # Workspace root
└── pnpm-workspace.yaml
```

## Tech Stack

| Layer     | Technology                          |
|-----------|-------------------------------------|
| Desktop   | Tauri 2                             |
| Backend   | Rust + reqwest + serde + tokio      |
| Frontend  | SvelteKit (SPA) + Svelte 5          |
| Styling   | Tailwind CSS v4                     |
| i18n      | Custom Svelte store (zh / en)       |
| Package   | pnpm workspace                      |

## License

MIT
