<template>
  <nav class="flex items-center gap-0.5 overflow-x-auto py-1 whitespace-nowrap select-none">
    <button
      class="flex items-center gap-1 px-2 py-1 rounded-lg text-ink-secondary hover:text-ink hover:bg-surface-elevated transition-colors font-medium"
      @click="goTo('')"
      title="Shared root"
    >
      <FolderRoot class="w-3.5 h-3.5" />
      <span>Shared</span>
    </button>
    <template v-for="(segment, i) in segments" :key="i">
      <ChevronRight class="w-3 h-3 text-ink-muted flex-shrink-0" />
      <button
        class="px-2 py-1 rounded-lg text-ink-secondary hover:text-ink hover:bg-surface-elevated transition-colors font-medium"
        @click="goTo(segments.slice(0, i + 1).join('/'))"
      >
        {{ segment }}
      </button>
    </template>
  </nav>
</template>

<script setup>
import { computed } from 'vue';
import { FolderRoot, ChevronRight } from 'lucide-vue-next';
import { currentPath, openFolder } from '../relay';

const segments = computed(() =>
  currentPath.value ? currentPath.value.split('/') : []
);

function goTo(path) {
  if (path !== currentPath.value) openFolder(path);
}
</script>