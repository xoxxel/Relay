<template>
  <div class="flex flex-col items-center justify-center py-8 select-none">
    <!-- Toggle Button -->
    <button
      type="button"
      @click="store.toggleServer"
      :disabled="store.loading"
      :class="[
        'relative inline-flex items-center rounded-full p-1.5 transition-colors duration-300 ease-out focus:outline-none focus:ring-2 focus:ring-signal/60 focus:ring-offset-2 focus:ring-offset-paper disabled:opacity-50 disabled:cursor-not-allowed',
        store.status.running
          ? 'bg-signal'
          : 'bg-idle-surface border border-border hover:border-signal/40'
      ]"
      style="width: 152px; height: 76px"
      role="switch"
      :aria-checked="store.status.running"
      aria-label="Toggle Relay Server"
    >
      <span
        aria-hidden="true"
        :class="[
          'pointer-events-none inline-flex items-center justify-center rounded-full bg-paper shadow-lg transition-transform duration-300 ease-out',
          store.status.running ? 'translate-x-[74px]' : 'translate-x-0'
        ]"
        style="width: 66px; height: 66px"
      >
        <Power
          class="w-5 h-5 transition-colors duration-300"
          :class="store.status.running ? 'text-signal' : 'text-ink-muted/60'"
        />
      </span>
    </button>

    <!-- Status Label -->
    <div class="mt-5 text-center">
      <div class="flex items-center justify-center gap-2">
        <span v-if="store.status.running" class="relative flex h-2.5 w-2.5">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-signal opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-signal"></span>
        </span>
        <span
          class="text-xs font-semibold tracking-wide transition-colors duration-200"
          :class="store.status.running ? 'text-signal' : 'text-ink-muted'"
        >
          {{ store.status.running ? 'Sharing Active' : 'Server Inactive' }}
        </span>
      </div>

      <p class="text-[11px] text-ink-muted mt-1.5">
        {{ store.status.running ? 'Connected to local Wi-Fi network' : 'Turn on to share files and clipboard' }}
      </p>

      <div v-if="store.error" class="mt-3 px-3 py-1.5 rounded-lg bg-red-950/40 border border-red-800/50 text-[11px] text-red-400 max-w-xs text-center">
        {{ store.error }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { Power } from 'lucide-vue-next';
import { useServerStore } from '../stores/serverStore';

const store = useServerStore();
</script>