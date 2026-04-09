# APIPup

Ein hochleistungsfähiges, plattformübergreifendes API-Test-Tool mit nativer Desktop-Leistung.

Erstellt mit **Tauri** + **Rust** + **SvelteKit** für ein schnelles, leichtes und reaktionsschnelles Erlebnis.

Unterstützt **macOS** und **Windows**.

## Design-Philosophie

- **Kein Server, kein Cloud-Speicher** — Alle Anfragen werden direkt von der Rust-Schicht gesendet, was Sicherheitsprobleme vermeidet
- **Lokale Datenspeicherung** — Daten verbleiben auf Ihrer Festplatte. Verwenden Sie Git für die Versionskontrolle und Team-Zusammenarbeit
- **KI-freundlich und CLI-freundlich** — Entwickelt für Automatisierung und Integration mit KI-Tools

## Leistung

| Metrik | APIPup | Typische Electron-Apps |
|--------|--------|----------------------|
| Bundle-Größe | ~10MB | ~450MB |
| Startgeschwindigkeit | Ultra-schnell | Langsamer |
| Zeit für 1000 Anfragen | Schnell (Rust) | Langsamer |
| Spitzen-Speicher (100 gleichzeitige, 1000 insgesamt) | 20–60MB | ~100–200MB |

## Funktionen

- HTTP-Anfragen senden (GET, POST, PUT, DELETE usw.)
- Antwortheader und -text anzeigen
- Integrierte Unterstützung für Englisch, Chinesisch, Japanisch, Französisch und Deutsch
- Schnell und leicht

## Für Entwickler

### Architektur

```
ui/ (SvelteKit)  ──invoke()──>  tauri/ (Rust)
                              └── HTTP-Anfragen
                              └── Gibt Antwort zurück
```

- **Frontend** (`ui/`): SvelteKit mit Tailwind CSS
- **Backend** (`tauri/`): Tauri + Rust HTTP-Engine
- **Kommunikation**: Tauri `invoke()` — keine CORS-Probleme, keine zusätzlichen Ports

### Voraussetzungen

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- [Rust](https://rustup.rs/) >= 1.77
- Plattformspezifische Abhängigkeiten:
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/), WebView2 (unter Windows 10/11 vorinstalliert)

### Erste Schritte

```bash
# Repository klonen
git clone <repo-url>
cd APIPup

# Abhängigkeiten installieren
pnpm install

# Entwicklung starten (startet Frontend-Dev-Server und Tauri-Fenster)
pnpm dev
```

`pnpm dev` wird:
1. Den SvelteKit Vite-Entwicklungsserver auf `http://127.0.0.1:1420` starten
2. Das Rust-Backend kompilieren
3. Das APIPup-Desktop-Fenster öffnen

### Build für die Produktion

```bash
# .app erstellen
pnpm build

# .dmg erstellen (erfordert: brew install create-dmg)
pnpm build:dmg
```

Die Ausgabe befindet sich unter `tauri/target/release/bundle/macos/`.

## Lizenz

MIT
