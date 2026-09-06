# نقشه راه توسعه — LAN Share

> برگرفته از `proposal.md` (بخش ۱۳) و گسترش‌یافته با جزئیات اجرایی، وابستگی‌ها و معیارهای آمادگی هر فاز.
> هر فاز فقط وقتی «تمام» حساب می‌شود که معیارهای آمادگی پایین همان فاز برقرار باشد.

---

## فاز ۰ — زیرساخت و پروژه پایه (Scaffold)

**هدف:** پروژه‌ی Tauri خالی با هر دو باندل Vue و چرخه‌ی build یکپارچه.

- [x] نصب پیش‌نیازها (بخش ۱۲ پروپوزال): Rust, Node, وابستگی‌های WebKitGTK, Tauri CLI
- [x] `tauri init` با نام `lan-share` و ساختار پوشه‌ها طبق بخش ۴
- [x] ایجاد `desktop-panel/` با Vue 3 + Vite + Tailwind + Pinia + vueuse + qrcode.vue
- [x] ایجاد `web-client/` با Vue 3 + Vite + Tailwind (بدون Pinia در ابتدا؛ در صورت نیاز اضافه شود)
- [x] پیکربندی `rust-embed` برای جاسازی `web-client/dist` در باینری (کافی است خروجی خالی/placeholder فعلاً باشد)
- [x] اسکریپت توسعه و build که هر دو باندل را می‌سازد:
  - `dev`: `cargo tauri dev`
  - `build`: build هر دو باندل → `cargo tauri build`

**معیار آمادگی:** `cargo tauri dev` پنجره‌ی دسکتاپ را با یک پیام ساده باز کند و `cargo tauri build` خروجی `.deb`/AppImage بدهد.

---

## فاز ۱ — هسته سرور (axum + IPC)

**هدف:** سروری که با toggle روشن/خاموش می‌شود و یک روت تست در اختیار web-client قرار می‌دهد.

- [x] `state.rs`: تعریف `AppState` مشترک (`Arc<...>`) برای port، shared_folder، وضعیت سرور
- [x] `commands.rs`:
  - [x] `start_server` — بررسی آزاد بودن پورت، بالا آوردن axum روی IP واقعی LAN و پورت
  - [x] `stop_server` — خاموش‌کردن graceful
  - [x] `get_server_status` — برگرداندن `ServerStatus` (بخش ۵)
- [x] روت تست `GET /api/files?path=` روی یک پوشه‌ی هاردکد (در `server/routes_files.rs`)
- [x] اتصال پنل دسکتاپ: `ServerToggle.vue` → `invoke('start_server'/'stop_server')` + نمایش `ServerStatus`
- [x] سرو کردن `web-client` از محل embed (مسیر ریشه)

**معیار آمادگی:** دکمه‌ی toggle سرور را زیر ۱ ثانیه روشن کند، وضعیت در پنل نمایش داده شود و `GET /api/files` از مرورگر موبایل پاسخ بدهد.

---

## فاز ۲ — مدیریت فایل‌ها

**هدف:** فایل‌منیجر وب کامل با امنیت مسیر و آپلود/دانلود.

- [x] کامل کردن روت‌های فایل در `server/routes_files.rs`:
  - [x] `GET /api/files` (لیست، سورت، نمایش size/mtime/mime)
  - [x] `POST /api/files/upload` (multipart، محدودیت حجم طبق بخش ۱۱)
  - [x] `GET /api/files/download` (stream)
  - [x] `POST /api/files/mkdir`
  - [x] `DELETE /api/files`
- [x] پیاده‌سازی **Path Safety** (بخش ۶): نرمالایز + canonicalize + رد 403 برای خروج از ریشه
- [x] اضافه کردن middleware امنیتی (بخش ۱۱): فیلتر `Host`/CORS محدود به subnet — در نسخه‌ی اول no-op برای PIN
- [x] `web-client`:
  - [x] `FilesView.vue` + `BreadcrumbBar.vue` + `FileRow.vue`
  - [x] `UploadFab.vue` و فلوی آپلود چند فایل
  - [x] دانلود فایل + ساخت پوشه + حذف

**معیار آمادگی:** آپلود، دانلود، ساخت پوشه و حذف روی داده‌های واقعی کار کند؛ تلاش برای Path Traversal با 403 رد شود.

---

## فاز ۳ — زنده‌سازی (WebSocket)

**هدف:** به‌روزرسانی لحظه‌ای بدون رفرش برای همه‌ی کلاینت‌ها.

- [x] `server/ws.rs`: اتصال `/ws` با broadcast channel (`tokio::sync::broadcast`)
- [x] broadcast رویدادهای `file_added` / `file_removed` از روی روت‌های فایل
- [x] اتصال `web-client` به `/ws` (`useWebSocket`) و آپدیت خودکار لیست
- [x] اتصال `desktop-panel` به `/ws` برای نمایش فعالیت‌ها (`ActivityFeed.vue`)

**معیار آمادگی:** آپلود از یک موبایل، در کمتر از ۲ ثانیه بدون رفرش روی پنل دسکتاپ و سایر موبایل‌ها ظاهر شود.

---

## فاز ۴ — کلیپ‌بورد مشترک

**هدف:** ارسال زنده‌ی متن بین همه‌ی دیوایس‌ها + تاریخچه در SQLite.

- [x] `db.rs`: ایجاد و migration جدول `clip_items` با `rusqlite`
- [x] روت‌های `/api/clips`:
  - [x] `GET /api/clips` (۵۰ آیتم آخر)
  - [x] `POST /api/clips` (ذخیره + broadcast `clip_added`)
  - [x] `DELETE /api/clips/:id` (حذف + broadcast `clip_removed`)
- [x] `ClipComposer.vue` در `web-client` (دمای sticky در `ClipboardView.vue`)
- [x] `ClipboardPanel.vue` در `desktop-panel`
- [x] `copy_to_system_clipboard` (پلاگین `tauri-plugin-clipboard-manager`) با کلیک روی آیتم در پنل دسکتاپ

**معیار آمادگی:** ارسال متن از موبایل، همان لحظه در پنل دسکتاپ و موبایل‌های متصل دیده شود؛ کلیک روی آیتم در دسکتاپ متن را در کلیپ‌بورد سیستم مینت کپی کند؛ بعد از ری‌استارت اپ، تاریخچه حفظ شده باشد.

---

## فاز ۵ — کشف آدرس و پنل کنترل

**هدف:** اتصال موبایل بدون تایپ دستی آدرس.

- [x] `mdns.rs`: ثبت `lanshare.local` با `mdns-sd` هنگام start و حذف هنگام stop
- [x] `ConnectionCard.vue`: نمایش QR (`qrcode.vue`) + آدرس + دکمه‌ی کپی سریع آدرس
- [x] نمایش کارت فقط هنگام روشن بودن سرور (fade+scale ~۲۰۰ms طبق بخش ۹)
- [x] `FolderPicker.vue` + `pick_shared_folder` (دیالوگ بومی) + ذخیره در تنظیمات (`tauri-plugin-store`)
- [x] `get_settings` / `save_settings` (پورت، پوشه، نام دستگاه)

**معیار آمادگی:** اسکن QR مستقیماً به فایل‌منیجر وب برسد؛ `lanshare.local` (یا معادل IP در صورت عدم پشتیبانی) کار کند؛ تغییر پوشه‌ی اشتراکی از تنظیمات اعمال شود.

---

## فاز ۶ — پرداخت طراحی نهایی و انتشار

**هدف:** ظاهر مطابق بخش ۹ + تست واقعی + بسته‌ی نصبی.

- [x] پیاده‌سازی کامل سیستم طراحی: پالت رنگ (Paper/Surface/Border/Ink/Signal/Idle)، فونت Inter + JetBrains Mono (فقط داده‌های فنی)
- [x] اجرای دقیق چیدمان پنل دسکتاپ و web-client از بخش ۹ (Toggle بزرگ به‌عنوان المان اصلی، بدون کارت‌های تکراری)
- [x] حالت‌های خالی و خطا با متن راهنما (مثلاً «این پوشه خالیه — یه فایل از پایین اضافه کن»)
- [ ] `prefers-reduced-motion` و focus state قابل مشاهده
- [ ] تست روی یک موبایل اندروید واقعی در همان شبکه‌ی وای‌فای
- [ ] تست خاموش‌کردن toggle و عدم دسترسی موبایل‌ها بعد از آن
- [ ] build نهایی و تایید نصب روی Linux Mint (`.deb`/AppImage)

**معیار آمادگی:** همه‌ی معیارهای پذیرش بخش ۱۴ پروپوزال برقرار باشد.

---

## سفارش اجرا و وابستگی‌ها

```
فاز ۰ ← فاز ۱ ← فاز ۲ ← فاز ۳ ← فاز ۴
              └──── فاز ۵ (مستقل از ۲/۳/۴، میتواند موازی)
                                   ← فاز ۶ (منوط به همه)
```

- فاز ۵ فقط به فاز ۱ وابسته است؛ می‌تواند هم‌زمان با فازهای ۲–۴ جلو برود.
- فازهای ۲، ۳ و ۴ پشت‌سر هم هستند (۳ به ۲ نیاز دارد چون رویداد file رو broadcast می‌کند؛ ۴ مستقل از ۳ ولی به ۱ بند است).
- پیشنهاد: هر فاز را با یک commit در گیت ببند و بعد از هر فاز، معیار آمادگی همان فاز را دستی تست کن....