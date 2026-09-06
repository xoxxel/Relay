<template>
  <div class="min-h-screen bg-paper text-ink flex flex-col max-w-lg mx-auto select-none">
    <!-- Header -->
    <header class="sticky top-0 z-20 bg-paper/95 backdrop-blur px-4 py-3.5 border-b border-border flex items-center justify-between animate-float-in">
      <div class="flex items-center gap-2.5">
<<<<<<< HEAD
        <div class="w-9 h-9 rounded-xl bg-signal/15 border border-signal/30 flex items-center justify-center overflow-hidden shadow-glow-signal">
          <img src="/relay-icon.svg" alt="Relay" class="w-7 h-7" />
        </div>
=======
        <img :src="relayLogo" class="w-7 h-7 opacity-90" alt="" draggable="false" />
>>>>>>> 34ca5f1 (feat: UI updates, new icons, fix tauri path)
        <div>
          <h1 class="text-base font-bold tracking-tight text-ink leading-none">Relay</h1>
          <p class="text-[11px] text-ink-muted mt-0.5">Local Network Share</p>
        </div>
      </div>

      <div
        class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-medium border"
        :class="connected
          ? 'bg-signal/10 border-signal/30 text-signal'
          : 'bg-idle-surface border-border text-ink-muted'"
      >
        <span
          class="w-2 h-2 rounded-full"
          :class="connected ? 'bg-signal animate-pulse' : 'bg-idle'"
        ></span>
        <span>{{ connected ? 'Connected' : 'Offline' }}</span>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1 px-4 pb-28 pt-3">
      <!-- Files View -->
      <template v-if="activeTab === 'files'">
        <div class="flex items-center justify-between gap-2 mb-1">
          <BreadcrumbBar />
          <div class="flex items-center gap-0.5 flex-shrink-0">
            <button
              class="p-2 rounded-lg text-ink-muted hover:text-ink hover:bg-surface-elevated active:scale-95 transition-all"
              title="New folder"
              @click="createFolder"
            >
              <FolderPlus class="w-4 h-4" />
            </button>
            <button
              class="p-2 rounded-lg text-ink-muted hover:text-ink hover:bg-surface-elevated active:scale-95 transition-all disabled:opacity-40 disabled:cursor-not-allowed"
              title="Refresh"
              :disabled="loading"
              @click="fetchFiles"
            >
              <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': loading }" />
            </button>
          </div>
        </div>

        <!-- Error State -->
        <div
          v-if="error"
          class="p-3.5 bg-red-950/40 border border-red-800/40 rounded-xl text-xs text-red-400 mb-4 flex items-center gap-2"
        >
          <AlertCircle class="w-4 h-4 flex-shrink-0" />
          <span>{{ error }}</span>
        </div>

        <!-- Empty State -->
        <div
          v-if="files.length === 0 && !loading"
          class="text-center py-14 px-4 bg-surface border border-border rounded-2xl flex flex-col items-center justify-center space-y-2"
        >
          <div class="w-12 h-12 rounded-2xl bg-surface-elevated border border-border flex items-center justify-center text-ink-muted mb-1">
            <FolderOpen class="w-6 h-6" />
          </div>
          <p class="text-sm font-medium text-ink">This folder is empty</p>
          <p class="text-xs text-ink-muted max-w-xs">
            Tap the upload button below to add a file here.
          </p>
        </div>

        <!-- File List -->
        <div
          v-else
          class="bg-surface border border-border rounded-2xl divide-y divide-border-subtle overflow-hidden"
        >
          <FileRow
            v-for="file in files"
            :key="file.path"
            :file="file"
          />
        </div>
      </template>

      <!-- Clipboard View -->
      <template v-else>
        <ClipComposer />

        <button
          class="mt-3 flex items-center gap-2 text-[11px] text-ink-muted bg-surface border border-border rounded-xl px-3 py-2"
          @click="changeDeviceName"
        >
          <Smartphone class="w-3.5 h-3.5 text-ink-muted" />
          <span>
            Sending as
            <span class="text-ink-secondary font-medium select-text">{{ deviceName }}</span>
          </span>
          <Pencil class="w-3 h-3 ml-auto text-ink-muted/70" />
        </button>

        <!-- Empty State -->
        <div
          v-if="clips.length === 0"
          class="text-center py-14 px-4 bg-surface border border-border rounded-2xl flex flex-col items-center space-y-2 mt-3"
        >
          <div class="w-12 h-12 rounded-2xl bg-surface-elevated border border-border flex items-center justify-center text-ink-muted mb-1">
            <ClipboardList class="w-6 h-6" />
          </div>
          <p class="text-sm font-medium text-ink">Clipboard is empty</p>
          <p class="text-xs text-ink-muted max-w-xs">
            Text you send from any connected device appears here instantly.
          </p>
        </div>

        <!-- Clip List -->
        <div
          v-else
          class="mt-3 bg-surface border border-border rounded-2xl divide-y divide-border-subtle overflow-hidden"
        >
          <div
            v-for="clip in clips"
            :key="clip.id"
            class="px-4 py-3 cursor-pointer hover:bg-surface-elevated/60 transition-colors"
            @click="copyText(clip.content)"
          >
            <div class="flex items-start gap-3">
              <div class="min-w-0 flex-1">
                <p class="text-sm text-ink leading-snug select-text whitespace-pre-wrap break-words">
                  {{ clip.content }}
                </p>
                <p class="flex items-center gap-1.5 text-[10px] text-ink-muted mt-1.5">
                  <UserRound class="w-3 h-3 flex-shrink-0" />
                  <span class="truncate">{{ clip.device_label }}</span>
                  <span class="text-ink-muted/50">•</span>
                  <span>{{ timeAgo(clip.created_at) }}</span>
                </p>
              </div>
              <div class="flex items-center gap-1 flex-shrink-0">
                <button
                  class="p-1.5 rounded-lg text-ink-muted/70 hover:text-signal hover:bg-signal/10 active:scale-95 transition-all"
                  title="Copy to this device"
                  @click.stop="copyText(clip.content)"
                >
                  <Copy class="w-3.5 h-3.5" />
                </button>
                <button
                  class="p-1.5 rounded-lg text-ink-muted/70 hover:text-red-400 hover:bg-red-500/10 active:scale-95 transition-all"
                  title="Delete"
                  @click.stop="deleteClip(clip.id)"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
          </div>
        </div>
      </template>
    </main>

    <!-- Bottom Tab Bar -->
    <nav
      class="fixed bottom-0 inset-x-0 z-30 bg-paper/95 backdrop-blur border-t border-border"
    >
      <div class="max-w-lg mx-auto px-4 py-2 grid grid-cols-2 gap-1">
        <button
          class="flex items-center justify-center gap-2 py-2.5 rounded-xl text-xs font-semibold transition-all active:scale-[0.98]"
          :class="activeTab === 'files'
            ? 'bg-signal/10 text-signal'
            : 'text-ink-muted hover:text-ink'"
          @click="setTab('files')"
        >
          <FolderClosed class="w-4 h-4" />
          Files
        </button>
        <button
          class="flex items-center justify-center gap-2 py-2.5 rounded-xl text-xs font-semibold transition-all active:scale-[0.98]"
          :class="activeTab === 'clipboard'
            ? 'bg-signal/10 text-signal'
            : 'text-ink-muted hover:text-ink'"
          @click="setTab('clipboard')"
        >
          <ClipboardList class="w-4 h-4" />
          Clipboard
          <span
            v-if="clips.length"
            class="min-w-[16px] h-4 px-1 rounded-full bg-surface-elevated border border-border text-[9px] text-ink-secondary flex items-center justify-center"
          >
            {{ clips.length }}
          </span>
        </button>
      </div>
    </nav>

    <!-- Upload Floating Button (Files tab only) -->
    <UploadFab v-if="activeTab === 'files'" />
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue';
import {
  FolderClosed,
  FolderOpen,
  FolderPlus,
  RefreshCw,
  AlertCircle,
  ClipboardList,
  Copy,
  Trash2,
  Smartphone,
  Pencil,
  UserRound,
} from 'lucide-vue-next';
import relayLogo from './assets/relay-icon.svg';
import BreadcrumbBar from './components/BreadcrumbBar.vue';
import FileRow from './components/FileRow.vue';
import UploadFab from './components/UploadFab.vue';
import ClipComposer from './components/ClipComposer.vue';
import {
  files,
  clips,
  activeTab,
  loading,
  error,
  connected,
  init,
  fetchFiles,
  createFolder,
  deleteClip,
  copyText,
  deviceLabel,
  setDeviceLabel,
} from './relay';

const deviceName = computed(() => deviceLabel());

function setTab(tab) {
  activeTab.value = tab;
}

function changeDeviceName() {
  const name = window.prompt('Device name shown to others', deviceName.value);
  if (name) {
    setDeviceLabel(name);
    deviceName.value = deviceLabel();
  }
}

function timeAgo(iso) {
  if (!iso) return '';
  const diff = Date.now() - new Date(iso).getTime();
  if (diff < 60_000) return 'just now';
  if (diff < 3_600_000) return `${Math.floor(diff / 60_000)} min ago`;
  if (diff < 86_400_000) return `${Math.floor(diff / 3_600_000)} hr ago`;
  return new Date(iso).toLocaleDateString([], { month: 'short', day: 'numeric' });
}

onMounted(() => {
  init();
});
</script>
