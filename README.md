# Relay

A minimal, ultra-fast, and secure **Local Network File & Live Clipboard Sharing** desktop application. Share files and sync clipboard in real-time across your local Wi-Fi network without requiring internet access or installing any app on mobile devices.

---

## ✨ Key Characteristics

- **Name**: Relay
- **Theme**: Sleek Dark Theme (`#0B0E14` obsidian canvas, `#141721` elevated surface, emerald signal glow).
- **Language**: English (LTR), minimal, modern typography and clean iconography.
- **Responsive**: Adaptive layouts tailored for both desktop control panel and mobile web clients.

---

## 🚀 Progress & Roadmap

- [x] **Phase 0 — Project Scaffold**: Tauri v2, Vue 3, Vite, Tailwind CSS (Dark Mode), Pinia, and `rust-embed`.
- [x] **Phase 1 — Core Server & IPC**: Axum HTTP server with Tokio background task, graceful shutdown, Path Traversal safety protection, English dark theme UI with live toggle and QR Code connection card.
- [ ] **Phase 2 — File Manager**: Multipart multi-file upload, streaming download, directory creation & deletion.
- [ ] **Phase 3 — Real-Time WebSocket**: Live updates across all connected devices without page refresh.
- [ ] **Phase 4 — Shared Live Clipboard**: Instant text broadcast with SQLite persistent history.
- [ ] **Phase 5 — Discovery & Native Controls**: mDNS broadcasting (`relay.local`), native directory picker, persistent settings.
- [ ] **Phase 6 — Final Polish & Packaging**: System tray integration, `.deb`/AppImage builds.

---

## 💻 How to Run & Test

### Prerequisites
- **Node.js**: v18+
- **Rust & Cargo**: v1.80+

---

### 1. Launch Full Desktop Application (Tauri)

Runs the native desktop window connected to the Rust Axum server:

```bash
# Install dependencies (if not already installed)
npm install
npm --prefix desktop-panel install
npm --prefix web-client install

# Start Tauri in development mode
npm run tauri:dev
```

---

### 2. Preview Desktop Panel in Browser (Quick UI Test)

To inspect and test the desktop control panel components directly in your browser:

```bash
npm run dev:desktop
```
Open in browser: 👉 **`http://localhost:1420`**

---

### 3. Preview Mobile Web Client

To test the mobile-friendly web client (which clients on your local network will see):

```bash
npm run dev:web
```
Open in browser: 👉 **`http://localhost:5173`**

---

### 4. Run Automated Backend & Security Tests

Executes all unit tests, path traversal security validation, and Axum server integration tests:

```bash
npm run test:backend
```

---

## 📁 Repository Structure

```text
Relay/
├── desktop-panel/      # Desktop Vue 3 + Vite + Tailwind Dark Theme UI
├── web-client/         # Mobile Web Client served embedded by Axum
├── src-tauri/          # Rust Backend & Axum HTTP / IPC Core
│   ├── src/
│   │   ├── commands.rs     # Tauri IPC Commands
│   │   ├── state.rs        # AppState & ServerStatus models
│   │   ├── web_assets.rs   # rust-embed bundle serving
│   │   └── server/         # Axum server routes & path safety
│   └── tests/              # Server integration tests
├── ROADMAP.md          # Detailed development roadmap
└── proposal.md         # Technical architecture specification
```
