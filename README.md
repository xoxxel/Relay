<div align="center">

<img src="relay-icon-1024.svg" alt="Relay Logo" width="96" />

# Relay

**Minimal, ultra-fast local-network file & clipboard sharing — no internet, no app install.**

[![Tauri v2](https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri&logoColor=white)](https://tauri.app)
[![Vue 3](https://img.shields.io/badge/Vue-3-42b883?logo=vue.js&logoColor=white)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-2021-CE4A00?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/xoxxel/Relay?color=6E56CF)](https://github.com/xoxxel/Relay/releases)

</div>

---

## Overview

**Relay** is a lightweight desktop application that turns your computer into a local file server and shared clipboard hub. Once the server is running, any device on the same Wi-Fi — phone, tablet, laptop — can browse, upload, and download files or read/write the shared clipboard through a plain browser. No app installation, no cloud account, no internet connection required.

---

## Key Features

- 🚀 **One-click server** — toggle the HTTP/WebSocket server on and off from the desktop panel with a single button click.
- 📱 **Instant mobile access** — a QR code appears the moment the server starts; scan it with your phone camera to open the web UI immediately.
- 📁 **Full file manager** — browse directories, create folders, upload multiple files (multipart), and stream downloads — all from the browser.
- 📋 **Live shared clipboard** — paste on your laptop, read on your phone (and vice versa) in real time, persisted in an embedded SQLite database.
- 🔄 **Real-time updates** — file and clipboard changes are pushed to all connected clients over WebSocket; no manual refresh needed.
- 🔒 **Path traversal protection** — every file-system path is canonicalized and validated against the configured root before serving.
- 📡 **mDNS broadcasting** — the service is announced as `relay.local` so devices on the same network can discover it automatically.
- 🗂️ **Persistent settings** — chosen share folder is remembered across sessions via `tauri-plugin-store`.
- 🎨 **Obsidian dark theme** — clean, distraction-free dark UI for both the desktop panel and the mobile web client.

---

## How It Works

```
┌──────────────────────────────────────────────────────┐
│  Desktop (Relay app)                                 │
│                                                      │
│  1. Pick a folder  →  2. Toggle server ON            │
│                              │                       │
│                     Axum HTTP + WebSocket             │
│                     server starts on :4444           │
│                              │                       │
│  3. QR code appears  ←  Local IP detected            │
└──────────────────────────────┬───────────────────────┘
                               │  Local Wi-Fi
                    ┌──────────▼──────────┐
                    │  Mobile / Browser   │
                    │                     │
                    │  4. Scan QR code    │
                    │  5. Browser opens   │
                    │     web-client UI   │
                    │                     │
                    │  • Browse files     │
                    │  • Upload / Download│
                    │  • Shared clipboard │
                    └─────────────────────┘
```

| Step | Action |
|------|--------|
| **1** | Open Relay and choose the folder you want to share using the native folder picker. |
| **2** | Click **Start Server** — Relay starts an Axum HTTP + WebSocket server on port `4444` and begins broadcasting the service over mDNS. |
| **3** | A QR code containing the server's local IP address appears in the desktop panel. Tap **Copy** to copy the URL to your clipboard. |
| **4** | Scan the QR code with your phone camera (or paste the URL into any browser). |
| **5** | The mobile-optimised web client loads fully from the embedded server — browse folders, upload files, or use the shared clipboard, all in real time. |

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Desktop shell | [Tauri v2](https://tauri.app) | Native window, IPC, OS integrations |
| Desktop UI | [Vue 3](https://vuejs.org) + [Vite](https://vitejs.dev) + Tailwind CSS | Reactive desktop control panel |
| Web client UI | [Vue 3](https://vuejs.org) + [Vite](https://vitejs.dev) + Tailwind CSS | Mobile-friendly browser file manager |
| HTTP server | [Axum 0.8](https://github.com/tokio-rs/axum) + [Tokio](https://tokio.rs) | Async HTTP, multipart upload, streaming download |
| WebSocket | Axum `ws` feature | Real-time push events to all clients |
| Static embedding | [rust-embed](https://github.com/pyros2097/rust-embed) | Bundle web-client dist into the binary |
| Database | [SQLite](https://www.sqlite.org) via `rusqlite` (bundled) | Clipboard persistence |
| Service discovery | [mdns-sd](https://github.com/keepsimple1/mdns-sd) | Announce `relay.local` on the LAN |
| Clipboard access | [arboard](https://github.com/1Password/arboard) | Read/write system clipboard |
| File dialogs | [rfd](https://github.com/PolyMeilex/rfd) | Native folder picker |
| Settings | tauri-plugin-store | Persist user preferences |

---

## Repository Structure

```
Relay/
├── desktop-panel/          # Vue 3 desktop control-panel UI
│   ├── src/
│   │   ├── components/     # ServerCard, QRCard, FilePanel, ClipboardPanel …
│   │   ├── stores/         # Pinia stores
│   │   └── App.vue
│   └── vite.config.js
│
├── web-client/             # Vue 3 mobile browser UI (served by Axum)
│   ├── src/
│   │   ├── components/     # FileList, Uploader, ClipboardView …
│   │   ├── relay.js        # Relay API + WebSocket client
│   │   └── App.vue
│   └── vite.config.js
│
├── src-tauri/              # Rust / Tauri backend
│   ├── src/
│   │   ├── server/
│   │   │   ├── mod.rs          # Router assembly, server lifecycle
│   │   │   ├── routes_files.rs # File-manager endpoints
│   │   │   ├── routes_clip.rs  # Clipboard endpoints
│   │   │   └── ws.rs           # WebSocket broadcast hub
│   │   ├── commands.rs     # Tauri IPC commands (start/stop/status …)
│   │   ├── state.rs        # AppState shared across async tasks
│   │   ├── db.rs           # SQLite initialisation & queries
│   │   ├── mdns.rs         # mDNS service registration
│   │   ├── net.rs          # Local IP detection utilities
│   │   └── web_assets.rs   # rust-embed asset handler
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── install.sh              # Interactive build & install script
├── package.json            # Workspace scripts
└── README.md
```

---

## Getting Started

### Prerequisites

| Tool | Version |
|------|---------|
| [Rust](https://rustup.rs) | 1.77+ (stable) |
| [Node.js](https://nodejs.org) | 18+ |
| [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites) | v2 (`cargo install tauri-cli`) |

Install system dependencies for Tauri on Linux:

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

### Development

```bash
# 1. Clone the repository
git clone https://github.com/xoxxel/Relay.git
cd Relay

# 2. Install Node dependencies
npm install
cd desktop-panel && npm install && cd ..
cd web-client && npm install && cd ..

# 3. Build the web-client (served by Axum at runtime)
npm run build:web

# 4. Launch Tauri in dev mode (hot-reloads the desktop panel)
npm run tauri:dev
```

### Build for Production

```bash
# Build both frontends, then compile and bundle the Tauri app
npm run build:all
npm run tauri:build
```

The installer / binary is placed in `src-tauri/target/release/bundle/`.

Alternatively, use the interactive install script:

```bash
chmod +x install.sh
./install.sh
```

### Run Backend Tests

```bash
npm run test:backend
# or directly:
cargo test --manifest-path src-tauri/Cargo.toml
```

---

## Download

Pre-built binaries for Linux, macOS, and Windows are available on the [**Releases**](https://github.com/xoxxel/Relay/releases) page.

| Platform | Package |
|----------|---------|
| Linux (deb) | `relay_*.deb` |
| Linux (AppImage) | `relay_*.AppImage` |
| macOS | `relay_*.dmg` |
| Windows | `relay_*.msi` |

---

## License

Distributed under the [MIT License](LICENSE).
