#!/usr/bin/env bash
# ══════════════════════════════════════════════════════════════
#  Relay — اسکریپت نصب و ساخت تعاملی
#  سیستم تستشده: Linux Mint 22.3 / Ubuntu 24.04
# ══════════════════════════════════════════════════════════════

set -uo pipefail   # note: no -e so apt warnings don't abort
export PATH="$HOME/.cargo/bin:$PATH"

# ── رنگ‌ها ─────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

# ── توابع کمکی ─────────────────────────────────────────────────
header()  { echo -e "\n${BOLD}${CYAN}══ $* ══${RESET}"; }
info()    { echo -e "  ${GREEN}✔${RESET}  $*"; }
warn()    { echo -e "  ${YELLOW}⚠${RESET}  $*"; }
fail()    { echo -e "  ${RED}✘  $*${RESET}"; exit 1; }
expected(){ echo -e "  ${CYAN}↳ خروجی انتظاری:${RESET} $*"; }

# ── تأیید کاربر (fix: echo -en + read -r جدا) ─────────────────
confirm() {
    local msg="${1:-ادامه دهم؟}"
    echo ""
    echo -en "  ${YELLOW}▶ ${msg} [Enter=بله / Ctrl+C=لغو]${RESET} "
    read -r _
}

confirm_check() {
    local msg="${1:-خروجی درست بود؟}"
    echo ""
    echo -en "  ${YELLOW}▶ ${msg} [y=بله / n=لغو]${RESET}  "
    read -r ans
    [[ "${ans,,}" == "n" ]] && fail "کاربر لغو کرد."
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEB_PATH="$SCRIPT_DIR/src-tauri/target/release/bundle/deb/relay_0.1.0_amd64.deb"

# ══════════════════════════════════════════════════════════════
echo -e "\n${BOLD}🔌  Relay — نصب‌کنندهٔ تعاملی${RESET}"
echo    "    هر مرحله قبل از اجرا توضیح داده می‌شود."
echo    "    Ctrl+C را در هر لحظه بزن تا لغو شود."
# ══════════════════════════════════════════════════════════════


# ─────────────────────────────────────────────────────────────
header "قدم ۱ — نصب پیش‌نیازهای سیستم"
# ─────────────────────────────────────────────────────────────
echo "    بسته‌های لازم برای کامپایل Tauri روی سیستم نصب می‌شوند:"
echo "    libwebkit2gtk-4.1-dev, libgtk-3-dev, librsvg2-dev, patchelf,"
echo "    build-essential, libssl-dev, libxdo-dev, libayatana-appindicator3-dev"
warn "خطاهای GPG ریپوهای ربط‌نداشته (مثل Outline VPN) را نادیده بگیر — مشکل Relay نیست."
confirm "نصب پیش‌نیازها را شروع کنم؟ (نیاز به رمز sudo)"

# apt update: خطاهای ریپوهای ثالث را نادیده می‌گیریم
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
  || fail "نصب بسته‌ها ناموفق بود — خروجی بالا را چک کن."

info "پیش‌نیازها نصب شدند."


# ─────────────────────────────────────────────────────────────
header "قدم ۲ — بررسی Rust و Node"
# ─────────────────────────────────────────────────────────────
RUST_VER=$(cargo --version 2>/dev/null || echo "NOT FOUND")
NODE_VER=$(node  --version 2>/dev/null || echo "NOT FOUND")

echo -e "    cargo : ${GREEN}${RUST_VER}${RESET}"
echo -e "    node  : ${GREEN}${NODE_VER}${RESET}"
expected "cargo X.YY.Z  |  node vXX.Y.Z"

[[ "$RUST_VER" == "NOT FOUND" ]] && fail "Rust پیدا نشد — از https://rustup.rs نصب کن."
[[ "$NODE_VER" == "NOT FOUND" ]] && fail "Node پیدا نشد — از https://nodejs.org نصب کن."

confirm_check "نسخه‌ها درست به نظر می‌رسند؟"


# ─────────────────────────────────────────────────────────────
header "قدم ۳ — نصب وابستگی‌های Node"
# ─────────────────────────────────────────────────────────────
echo "    npm install برای root، desktop-panel و web-client اجرا می‌شود."
confirm "شروع کنم؟"

cd "$SCRIPT_DIR"
npm install --silent         || fail "npm install (root) ناموفق"
npm --prefix desktop-panel install --silent || fail "npm install (desktop-panel) ناموفق"
npm --prefix web-client install --silent    || fail "npm install (web-client) ناموفق"

info "node_modules نصب شدند."


# ─────────────────────────────────────────────────────────────
header "قدم ۴ — ساخت باندل‌های Vue (web-client + desktop-panel)"
# ─────────────────────────────────────────────────────────────
echo "    هر دو پروژه‌ی Vue با Vite build می‌شوند."
echo "    خروجی: desktop-panel/dist  و  web-client/dist"
confirm "Build را شروع کنم؟"

npm run build:all 2>&1 | tail -25
[[ ${PIPESTATUS[0]} -ne 0 ]] && fail "build:all ناموفق بود."

expected "✓ built in ...ms  (برای هر دو باندل)"
confirm_check "خروجی build موفق بود؟"


# ─────────────────────────────────────────────────────────────
header "قدم ۵ — ساخت باینری Rust + بسته‌ی نصبی"
# ─────────────────────────────────────────────────────────────
echo "    این مرحله چند دقیقه طول می‌کشد (کامپایل Release Rust)."
echo "    خروجی نهایی:"
echo "      .deb     →  src-tauri/target/release/bundle/deb/"
echo "      AppImage →  src-tauri/target/release/bundle/appimage/"
confirm "tauri build را شروع کنم؟"

set +o pipefail
npm run tauri:build 2>&1 | grep --line-buffered -E "(Compiling relay|Finished|Bundling|Built|^error)" || true
BUILD_EXIT=${PIPESTATUS[0]}
set -o pipefail

[[ $BUILD_EXIT -ne 0 ]] && fail "tauri:build ناموفق بود — خروجی بالا را چک کن."

# جستجوی فایل .deb
if [[ ! -f "$DEB_PATH" ]]; then
    DEB_PATH=$(find "$SCRIPT_DIR/src-tauri/target/release/bundle/deb/" -name "*.deb" 2>/dev/null | head -1 || true)
    [[ -z "$DEB_PATH" ]] && fail "هیچ فایل .deb پیدا نشد."
fi

info "بسته ساخته شد: $(basename "$DEB_PATH")"
confirm_check "Build موفق بود؟"


# ─────────────────────────────────────────────────────────────
header "قدم ۶ — نصب .deb روی سیستم"
# ─────────────────────────────────────────────────────────────
echo -e "    دستور: ${CYAN}sudo apt install $DEB_PATH${RESET}"
confirm "نصب را شروع کنم؟ (نیاز به رمز sudo)"

sudo apt install -y "$DEB_PATH" || fail "نصب .deb ناموفق بود."

info "Relay روی سیستم نصب شد."
expected "برنامه‌ای به نام Relay در منوی برنامه‌ها ظاهر شود"


# ─────────────────────────────────────────────────────────────
header "قدم ۷ — ساخت میانبر روی دسکتاپ"
# ─────────────────────────────────────────────────────────────
DESKTOP_FILE=$(find /usr/share/applications -iname "relay.desktop" 2>/dev/null | head -1 || true)

if [[ -z "$DESKTOP_FILE" ]]; then
    warn "فایل relay.desktop پیدا نشد. فهرست فایل‌های موجود:"
    ls /usr/share/applications/ | grep -i relay || echo "    (هیچ کدام)"
    confirm "بدون میانبر ادامه دهم؟"
else
    echo -e "    فایل: ${CYAN}$DESKTOP_FILE${RESET}"
    confirm "میانبر روی دسکتاپ بسازم؟"

    ln -sf "$DESKTOP_FILE" ~/Desktop/Relay.desktop
    chmod +x ~/Desktop/Relay.desktop
    gio set ~/Desktop/Relay.desktop metadata::trusted true 2>/dev/null \
        || warn "gio set ناموفق — کلیک راست روی آیکون و 'Allow Launching' را بزن."

    info "میانبر ساخته شد: ~/Desktop/Relay.desktop"
    expected "آیکون Relay روی دسکتاپ ظاهر شود"
    confirm_check "آیکون روی دسکتاپ دیده می‌شود؟"
fi


# ══════════════════════════════════════════════════════════════
echo ""
echo -e "${BOLD}${GREEN}🎉  همه مراحل با موفقیت انجام شدند!${RESET}"
echo ""
echo "  ➤  برنامه را از منوی برنامه‌ها یا دسکتاپ باز کن"
echo "  ➤  Toggle را روشن کن → QR Code نمایش داده می‌شود"
echo "  ➤  از موبایل (همان Wi-Fi) QR را اسکن کن"
echo "  ➤  آپلود / دانلود / کلیپ‌بورد را تست کن"
echo ""
echo "  حذف برنامه:  sudo apt remove relay"
echo ""
