<template>
  <div class="min-h-screen bg-paper text-ink flex flex-col max-w-lg mx-auto p-4 sm:p-6 select-none">
    <!-- Header -->
    <header class="py-4 border-b border-border flex items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-xl bg-signal/15 border border-signal/30 flex items-center justify-center">
          <Radio class="w-4 h-4 text-signal" />
        </div>
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
    <main class="flex-1 py-5">
      <!-- Section Title & Refresh -->
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-2">
          <FolderClosed class="w-4 h-4 text-ink-muted" />
          <h2 class="text-xs font-semibold uppercase tracking-wider text-ink-secondary">
            Shared Files
          </h2>
        </div>

        <button
          @click="fetchFiles"
          :disabled="loading"
          class="inline-flex items-center gap-1.5 px-2.5 py-1 text-xs font-medium text-ink-secondary hover:text-ink bg-surface border border-border rounded-lg transition-all active:scale-95 disabled:opacity-50"
        >
          <RefreshCw class="w-3 h-3" :class="{ 'animate-spin': loading }" />
          <span>{{ loading ? 'Updating...' : 'Refresh' }}</span>
        </button>
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
          Files added to the shared folder on host device will appear here instantly.
        </p>
      </div>

      <!-- File List -->
      <div
        v-else-if="files.length > 0"
        class="bg-surface border border-border rounded-2xl divide-y divide-border-subtle overflow-hidden shadow-sm"
      >
        <div
          v-for="file in files"
          :key="file.path"
          class="p-3.5 flex items-center justify-between hover:bg-surface-elevated/60 transition-colors"
        >
          <div class="flex items-center gap-3 min-w-0">
            <div class="w-9 h-9 rounded-xl bg-paper border border-border/80 flex items-center justify-center flex-shrink-0">
              <component
                :is="getFileIcon(file)"
                class="w-4 h-4"
                :class="file.is_dir ? 'text-amber-400' : 'text-signal'"
              />
            </div>
            <div class="min-w-0">
              <p class="text-xs font-semibold text-ink truncate select-text">{{ file.name }}</p>
              <p class="text-[10px] text-ink-muted font-mono mt-0.5">
                {{ file.is_dir ? 'Folder' : formatSize(file.size_bytes) }}
              </p>
            </div>
          </div>

          <span class="text-[10px] text-ink-muted font-mono flex-shrink-0 pl-2">
            {{ formatDate(file.modified_at) }}
          </span>
        </div>
      </div>
    </main>

    <!-- Footer -->
    <footer class="py-3 text-center border-t border-border-subtle text-[11px] text-ink-muted">
      <span>Relay Local Network Client</span>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import {
  Radio,
  FolderClosed,
  FolderOpen,
  Folder,
  FileText,
  FileImage,
  FileCode,
  FileMusic,
  File,
  RefreshCw,
  AlertCircle
} from 'lucide-vue-next';

const files = ref([]);
const loading = ref(false);
const connected = ref(false);
const error = ref(null);

async function fetchFiles() {
  loading.value = true;
  error.value = null;
  try {
    const res = await fetch('/api/files?path=');
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    files.value = Array.isArray(data) ? data : [];
    connected.value = true;
  } catch (err) {
    error.value = 'Failed to connect to Relay server: ' + (err.message || err);
    connected.value = false;
  } finally {
    loading.value = false;
  }
}

function getFileIcon(file) {
  if (file.is_dir) return Folder;
  const mime = file.mime_type || '';
  if (mime.startsWith('image/')) return FileImage;
  if (mime.startsWith('audio/')) return FileMusic;
  if (mime.includes('json') || mime.includes('javascript') || mime.includes('html')) return FileCode;
  if (mime.startsWith('text/') || mime.includes('pdf')) return FileText;
  return File;
}

function formatSize(bytes) {
  if (!bytes) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB'];
  let i = 0;
  let size = bytes;
  while (size >= 1024 && i < units.length - 1) {
    size /= 1024;
    i++;
  }
  return `${size.toFixed(1)} ${units[i]}`;
}

function formatDate(iso) {
  if (!iso) return '';
  try {
    const d = new Date(iso);
    return d.toLocaleDateString([], { month: 'short', day: 'numeric' }) + ' ' +
      d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  } catch {
    return '';
  }
}

onMounted(() => {
  fetchFiles();
});
</script>
