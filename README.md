# APIPup

A high-performance, cross-platform API testing tool with native desktop performance.

Built with **Tauri** + **Rust** + **SvelteKit** for a fast, lightweight, and responsive experience.

Supports **macOS** and **Windows**.

## Design Philosophy

- **No server, no cloud storage** — All requests are sent directly from the Rust layer, avoiding security issues
- **Local data storage** — Data stays on your disk. Use Git for version control and team collaboration
- **AI-friendly & CLI-friendly** — Designed for automation and integration with AI tools

## Performance

| Metric | APIPup | Typical Electron Apps |
|--------|--------|----------------------|
| Bundle Size | ~10MB | ~450MB |
| Startup Speed | Ultra-fast | Slower |
| 1000 Requests Time | Fast (Rust) | Slower |
| Peak Memory (100 concurrent, 1000 total) | 20–60MB | ~100–200MB |

## Features

- Send HTTP requests (GET, POST, PUT, DELETE, etc.)
- View response headers and body
- Built-in English and Chinese support
- Fast and lightweight

## For Developers

### Architecture

```
ui/ (SvelteKit)  ──invoke()──>  tauri/ (Rust)
                              └── HTTP requests
                              └── returns response
```

- **Frontend** (`ui/`): SvelteKit with Tailwind CSS
- **Backend** (`tauri/`): Tauri + Rust HTTP engine
- **Communication**: Tauri `invoke()` — no CORS issues, no extra ports

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- [Rust](https://rustup.rs/) >= 1.77
- Platform-specific dependencies:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), WebView2 (pre-installed on Windows 10/11)

### Getting Started

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

### Build for Production

```bash
# Build .app
pnpm build

# Build .dmg (requires: brew install create-dmg)
pnpm build:dmg
```

The output is at `tauri/target/release/bundle/macos/`.

## License

MIT
