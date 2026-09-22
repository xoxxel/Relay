# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] — 2026-09-22

### Added

- Text file editor in the web client with read, edit, save, and cancel actions.
- Optimistic concurrency checks that prevent overwriting a file changed by another client.
- UTF-8 validation for text editing, with binary files rejected from the editor.
- Clipboard copy support using the secure Clipboard API with a browser-compatible fallback.
- Backend integration tests for text routes, conflict handling, path traversal protection, and binary-file rejection.
- Frontend tests for clipboard success, fallback, and failure behavior.

### Fixed

- Safer text-file handling by rejecting unsupported binary content and unsafe paths before access.

---

## [0.1.0] — 2026-09-06

### Added

#### Project Scaffold
- Tauri v2 + Vue 3 + Vite project scaffold with `rust-embed` for bundling the web-client static assets directly into the Rust binary.
- Workspace-level `package.json` with unified scripts for dev, build, and test across `desktop-panel`, `web-client`, and `src-tauri`.
- Obsidian-inspired dark design system (deep backgrounds, violet/purple accent palette) applied consistently to both the desktop panel and the mobile web client.

#### HTTP Server (Axum)
- Axum 0.8 HTTP + WebSocket server running on port `4444`, driven by a fully async Tokio runtime.
- Tauri IPC commands (`start_server`, `stop_server`, `server_status`) for toggling the server from the desktop panel with graceful shutdown via `tokio::sync::oneshot`.
- CORS middleware via `tower-http` to allow cross-origin requests from any device on the local network.

#### File Manager
- `GET /api/files?path=` — list directory contents (name, size, type, modified timestamp).
- `POST /api/files/upload?path=` — multipart file upload supporting multiple files simultaneously.
- `GET /api/files/download?path=` — streaming file download with correct `Content-Type` (detected via `mime_guess`).
- `POST /api/files/mkdir` — create a new directory.
- `DELETE /api/files?path=` — delete files or directories.
- Path traversal security: every incoming path is resolved with `canonicalize` and checked against the configured root directory; requests outside the root are rejected with `403 Forbidden`.

#### Real-Time Updates (WebSocket)
- `GET /ws` — WebSocket endpoint with a broadcast hub (`tokio::sync::broadcast`) that pushes JSON events to all connected clients.
- Events emitted on: file upload, file delete, directory creation, and clipboard write.
- Web-client auto-reconnects on disconnect with exponential back-off.

#### Shared Clipboard
- `GET /api/clipboard` — read the current shared clipboard entry.
- `POST /api/clipboard` — write a new clipboard entry.
- SQLite persistence (via bundled `rusqlite`) so the clipboard survives server restarts.
- Native system clipboard integration via `arboard`; desktop panel syncs host clipboard with the shared clipboard on write.

#### mDNS Service Discovery
- `relay.local` service announced over mDNS using `mdns-sd` on every server start.
- Service is cleanly unregistered on server stop or application exit.

#### QR Code Connection Card
- Desktop panel generates a QR code embedding the server's local IP address (`http://<ip>:4444`) using the detected primary network interface (`local-ip-address`).
- One-click **Copy** button copies the URL to the system clipboard.
- QR card is hidden when the server is stopped.

#### Desktop Panel (Vue 3)
- Native folder picker via `rfd` exposed through a Tauri IPC command; selected path is displayed in the panel and passed to the server as the share root.
- Persistent settings (last-used folder, server port) stored via `tauri-plugin-store`.
- Server toggle button with live status indicator (running / stopped).
- Minimalist window: 460 × 660 px, minimum 380 × 560 px, `#0A0A0C` background.

#### Mobile Web Client (Vue 3)
- Fully responsive file-manager UI served directly by the Axum server from embedded assets.
- Breadcrumb navigation, drag-and-drop upload zone, per-file download buttons, and mkdir / delete actions.
- Live clipboard panel with read/write support; updates pushed via WebSocket.
- No installation required on mobile — works in any modern mobile browser.

#### Tooling & CI
- Interactive `install.sh` build & install script: checks prerequisites (Rust, Node, Tauri CLI), builds both frontends, compiles the Tauri bundle, and optionally installs the `.deb` on Debian/Ubuntu.
- GitHub Actions **CI workflow**: runs `cargo test` and `cargo clippy` on every push and pull request.
- GitHub Actions **release workflow**: builds platform bundles (Linux deb/AppImage, macOS dmg, Windows msi) on version tags and publishes them to GitHub Releases.

---

[0.2.0]: https://github.com/xoxxel/Relay/releases/tag/v0.2.0
[0.1.0]: https://github.com/xoxxel/Relay/releases/tag/v0.1.0
