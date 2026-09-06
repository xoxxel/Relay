# Relay — Development Roadmap

## Phase 0 — Project Scaffold
**Goal:** Tauri skeleton with both Vue bundles and unified build pipeline.
- [x] Install prerequisites: Rust, Node, WebKitGTK, Tauri CLI
- [x] tauri init with folder structure
- [x] Create desktop-panel/ with Vue 3 + Vite + Tailwind + Pinia + vueuse + qrcode.vue
- [x] Create web-client/ with Vue 3 + Vite + Tailwind
- [x] Configure rust-embed to bundle web-client/dist into the binary
- [x] Dev and build scripts for both bundles

**Readiness:** cargo tauri dev opens a desktop window; cargo tauri build produces .deb/AppImage.

## Phase 1 — Core Server (Axum + IPC)
**Goal:** Server toggleable on/off via IPC, serving a test route to web-client.
- [x] state.rs: AppState with port, shared_folder, server status
- [x] commands.rs: start_server, stop_server, get_server_status
- [x] Test route GET /api/files?path= on a hardcoded folder
- [x] Desktop panel: ServerToggle.vue → invoke IPC + display ServerStatus
- [x] Serve embedded web-client from root route

**Readiness:** Toggle starts server in <1s; GET /api/files responds from mobile browser.

## Phase 2 — File Manager
**Goal:** Full web file manager with path safety and upload/download.
- [x] GET /api/files (list, sort, size/mtime/mime)
- [x] POST /api/files/upload (multipart, size limit)
- [x] GET /api/files/download (stream)
- [x] POST /api/files/mkdir
- [x] DELETE /api/files
- [x] Path safety: normalize + canonicalize + 403 for traversal attempts
- [x] Security middleware: Host/CORS limited to subnet
- [x] web-client: FilesView.vue + BreadcrumbBar.vue + FileRow.vue + UploadFab.vue

**Readiness:** Upload, download, mkdir, delete work on real data; traversal attempts return 403.

## Phase 3 — Real-Time WebSocket
**Goal:** Live updates across all connected devices without page refresh.
- [x] server/ws.rs: /ws endpoint with broadcast channel
- [x] Broadcast file_added / file_removed events from file routes
- [x] web-client connects to /ws (useWebSocket) and auto-refreshes file list
- [x] desktop-panel connects to /ws for ActivityFeed.vue

**Readiness:** Upload from one mobile appears on desktop and other mobiles in <2s without refresh.

## Phase 4 — Shared Clipboard
**Goal:** Instant text broadcast between all devices + SQLite history.
- [x] db.rs: create and migrate clip_items table with rusqlite
- [x] GET /api/clips (last 50 items)
- [x] POST /api/clips (save + broadcast clip_added)
- [x] DELETE /api/clips/:id (delete + broadcast clip_removed)
- [x] ClipComposer.vue in web-client (sticky in ClipboardView)
- [x] ClipboardPanel.vue in desktop-panel
- [x] copy_to_system_clipboard via tauri-plugin-clipboard-manager

**Readiness:** Text sent from mobile appears instantly on desktop and all connected mobiles; history persists across restarts.

## Phase 5 — Discovery & Settings
**Goal:** Connect mobile without manually typing an IP address.
- [x] mdns.rs: register relay.local with mdns-sd on start, unregister on stop
- [x] ConnectionCard.vue: QR code + address + copy button
- [x] Card visible only when server is on (fade+scale animation)
- [x] FolderPicker.vue + pick_shared_folder native dialog + store via tauri-plugin-store
- [x] get_settings / save_settings (port, folder, device name)

**Readiness:** QR scan goes directly to web file manager; lanshare.local resolves; folder change applies immediately.

## Phase 6 — Final Polish & Release
**Goal:** Design system complete + real-device testing + installable package.
- [x] Full design system: color palette, Inter + JetBrains Mono typography
- [x] Desktop panel and web-client layouts
- [x] Empty and error states with guidance text
- [ ] prefers-reduced-motion support and visible focus states
- [ ] Real-device test on Android mobile on same Wi-Fi network
- [ ] Final build and install verification on Linux Mint (.deb / AppImage)

**Readiness:** All acceptance criteria from the proposal met.

---
## Execution Order
```
Phase 0 → Phase 1 → Phase 2 → Phase 3 → Phase 4
              └──── Phase 5 (parallel with 2/3/4)
                                   → Phase 6 (after all)
```
