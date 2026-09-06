#!/usr/bin/env bash
# ══════════════════════════════════════════════════════════════
#  Relay — Interactive Build & Install Script
#  Tested on: Linux Mint 22.3 / Ubuntu 24.04
# ══════════════════════════════════════════════════════════════

set -uo pipefail   # note: no -e so apt warnings don't abort
export PATH="$HOME/.cargo/bin:$PATH"

# ── Colors ─────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

# ── Helper functions ─────────────────────────────────────────────
header()  { echo -e "\n${BOLD}${CYAN}══ $* ══${RESET}"; }
info()    { echo -e "  ${GREEN}✔${RESET}  $*"; }
warn()    { echo -e "  ${YELLOW}⚠${RESET}  $*"; }
fail()    { echo -e "  ${RED}✘  $*${RESET}"; exit 1; }
expected(){ echo -e "  ${CYAN}↳ Expected output:${RESET} $*"; }

# ── User confirmation (fix: echo -en + read -r separately) ─────────────────
confirm() {
    local msg="${1:-Continue?}"
    echo ""
    echo -en "  ${YELLOW}▶ ${msg} [Enter=yes / Ctrl+C=cancel]${RESET} "
    read -r _
}

confirm_check() {
    local msg="${1:-Output looks correct?}"
    echo ""
    echo -en "  ${YELLOW}▶ ${msg} [y=yes / n=cancel]${RESET}  "
    read -r ans
    [[ "${ans,,}" == "n" ]] && fail "Cancelled by user."
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEB_PATH="$SCRIPT_DIR/src-tauri/target/release/bundle/deb/relay_0.1.0_amd64.deb"

# ══════════════════════════════════════════════════════════════
echo -e "\n${BOLD}🔌  Relay — Interactive Installer${RESET}"
echo    "    Each step will be explained before it runs."
echo    "    Press Ctrl+C at any time to cancel."
# ══════════════════════════════════════════════════════════════


# ─────────────────────────────────────────────────────────────
header "Step 1 — Install system dependencies"
# ─────────────────────────────────────────────────────────────
echo "    The following packages required to compile Tauri will be installed:"
echo "    libwebkit2gtk-4.1-dev, libgtk-3-dev, librsvg2-dev, patchelf,"
echo "    build-essential, libssl-dev, libxdo-dev, libayatana-appindicator3-dev"
warn "GPG errors from unrelated repos (e.g. Outline VPN) can be ignored — they are not a Relay issue."
confirm "Install system dependencies? (sudo password required)"

# apt update: ignore errors from third-party repos
sudo apt update 2>&1 | grep -v "^W:\|^Ign:" || true

sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  librsvg2-dev \
  patchelf \
  build-essential \
  libssl-dev \
  libxdo-dev \
  libayatana-appindicator3-dev \
  || fail "Package installation failed — check the output above."

info "System dependencies installed."


# ─────────────────────────────────────────────────────────────
header "Step 2 — Verify Rust & Node"
# ─────────────────────────────────────────────────────────────
RUST_VER=$(cargo --version 2>/dev/null || echo "NOT FOUND")
NODE_VER=$(node  --version 2>/dev/null || echo "NOT FOUND")

echo -e "    cargo : ${GREEN}${RUST_VER}${RESET}"
echo -e "    node  : ${GREEN}${NODE_VER}${RESET}"
expected "cargo X.YY.Z  |  node vXX.Y.Z"

[[ "$RUST_VER" == "NOT FOUND" ]] && fail "Rust not found — install from https://rustup.rs"
[[ "$NODE_VER" == "NOT FOUND" ]] && fail "Node not found — install from https://nodejs.org"

confirm_check "Versions look correct?"


# ─────────────────────────────────────────────────────────────
header "Step 3 — Install Node dependencies"
# ─────────────────────────────────────────────────────────────
echo "    Running npm install for root, desktop-panel, and web-client."
confirm "Proceed?"

cd "$SCRIPT_DIR"
npm install --silent         || fail "npm install (root) failed"
npm --prefix desktop-panel install --silent || fail "npm install (desktop-panel) failed"
npm --prefix web-client install --silent    || fail "npm install (web-client) failed"

info "node_modules installed."


# ─────────────────────────────────────────────────────────────
header "Step 4 — Build Vue bundles (web-client + desktop-panel)"
# ─────────────────────────────────────────────────────────────
echo "    Both Vue projects will be built with Vite."
echo "    Output: desktop-panel/dist  and  web-client/dist"
confirm "Start build?"

npm run build:all 2>&1 | tail -25
[[ ${PIPESTATUS[0]} -ne 0 ]] && fail "build:all failed."

expected "✓ built in ...ms  (for each bundle)"
confirm_check "Build succeeded?"


# ─────────────────────────────────────────────────────────────
header "Step 5 — Build Rust binary + installer package"
# ─────────────────────────────────────────────────────────────
echo "    This step takes a few minutes (Release Rust compilation)."
echo "    Final output:"
echo "      .deb     →  src-tauri/target/release/bundle/deb/"
echo "      AppImage →  src-tauri/target/release/bundle/appimage/"
confirm "Start tauri build?"

set +o pipefail
npm run tauri:build 2>&1 | grep --line-buffered -E "(Compiling relay|Finished|Bundling|Built|^error)" || true
BUILD_EXIT=${PIPESTATUS[0]}
set -o pipefail

[[ $BUILD_EXIT -ne 0 ]] && fail "tauri:build failed — check the output above."

# Search for .deb file
if [[ ! -f "$DEB_PATH" ]]; then
    DEB_PATH=$(find "$SCRIPT_DIR/src-tauri/target/release/bundle/deb/" -name "*.deb" 2>/dev/null | head -1 || true)
    [[ -z "$DEB_PATH" ]] && fail "No .deb file found."
fi

info "Package built: $(basename "$DEB_PATH")"
confirm_check "Build succeeded?"


# ─────────────────────────────────────────────────────────────
header "Step 6 — Install .deb on system"
# ─────────────────────────────────────────────────────────────
echo -e "    Command: ${CYAN}sudo apt install $DEB_PATH${RESET}"
confirm "Start installation? (sudo password required)"

sudo apt install -y "$DEB_PATH" || fail "Installation of .deb failed."

info "Relay installed on system."
expected "An application named Relay should appear in the application menu"


# ─────────────────────────────────────────────────────────────
header "Step 7 — Create desktop shortcut"
# ─────────────────────────────────────────────────────────────
DESKTOP_FILE=$(find /usr/share/applications -iname "relay.desktop" 2>/dev/null | head -1 || true)

if [[ -z "$DESKTOP_FILE" ]]; then
    warn "relay.desktop not found. Available files:"
    ls /usr/share/applications/ | grep -i relay || echo "    (none)"
    confirm "Continue without a shortcut?"
else
    echo -e "    File: ${CYAN}$DESKTOP_FILE${RESET}"
    confirm "Create desktop shortcut?"

    ln -sf "$DESKTOP_FILE" ~/Desktop/Relay.desktop
    chmod +x ~/Desktop/Relay.desktop
    gio set ~/Desktop/Relay.desktop metadata::trusted true 2>/dev/null \
        || warn "gio set failed — right-click the icon and select 'Allow Launching'."

    info "Shortcut created: ~/Desktop/Relay.desktop"
    expected "Relay icon should appear on the desktop"
    confirm_check "Desktop icon is visible?"
fi


# ══════════════════════════════════════════════════════════════
echo ""
echo -e "${BOLD}${GREEN}🎉  All steps completed successfully!${RESET}"
echo ""
echo "  ➤  Open Relay from the application menu or desktop"
echo "  ➤  Turn on the toggle → QR Code will be displayed"
echo "  ➤  Scan the QR code from your mobile (same Wi-Fi)"
echo "  ➤  Test upload / download / clipboard"
echo ""
echo "  Uninstall:  sudo apt remove relay"
echo ""
