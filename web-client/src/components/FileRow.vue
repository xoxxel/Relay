<template>
  <div class="flex items-center gap-1 py-1 pr-2 select-none">
    <button
      class="flex-1 flex items-center gap-3 min-w-0 p-1.5 rounded-lg hover:bg-surface-elevated/60 transition-colors text-left"
      @click="onOpen"
    >
      <div
        class="w-10 h-10 rounded-xl bg-paper border border-border/80 flex items-center justify-center flex-shrink-0 transition-transform group-hover:scale-105"
      >
        <component
          :is="icon"
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
    </button>

    <div class="flex items-center gap-0.5 flex-shrink-0">
      <button
        v-if="!file.is_dir"
        class="p-2 rounded-lg text-ink-muted hover:text-signal hover:bg-signal/10 active:scale-95 transition-all"
        title="Download"
        @click.stop="downloadFile(file)"
      >
        <Download class="w-3.5 h-3.5" />
      </button>
      <button
        class="p-2 rounded-lg text-ink-muted hover:text-red-400 hover:bg-red-500/10 active:scale-95 transition-all"
        title="Delete"
        @click.stop="handleDelete"
      >
        <Trash2 class="w-3.5 h-3.5" />
      </button>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import {
  Folder,
  FileText,
  FileImage,
  FileCode,
  FileMusic,
  File,
  Download,
  Trash2,
} from 'lucide-vue-next';
import { openFolder, downloadFile, deleteFile } from '../relay';

const props = defineProps({
  file: { type: Object, required: true },
});

const emit = defineEmits(['deleted']);

const icon = computed(() => {
  if (props.file.is_dir) return Folder;
  const mime = props.file.mime_type || '';
  if (mime.startsWith('image/')) return FileImage;
  if (mime.startsWith('audio/')) return FileMusic;
  if (
    mime.includes('json') ||
    mime.includes('javascript') ||
    mime.includes('html')
  )
    return FileCode;
  if (mime.startsWith('text/') || mime.includes('pdf')) return FileText;
  return File;
});

function onOpen() {
  if (props.file.is_dir) openFolder(props.file.path);
}

async function handleDelete() {
  await deleteFile(props.file);
  emit('deleted');
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
</script>
