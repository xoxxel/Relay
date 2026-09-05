<template>
  <div class="flex flex-col items-center justify-center py-6 select-none">
    <!-- Toggle Button Container -->
    <button
      type="button"
      @click="store.toggleServer"
      :disabled="store.loading"
      :class="[
        'relative inline-flex h-16 w-32 items-center rounded-full p-1.5 transition-all duration-300 ease-out focus:outline-none focus:ring-2 focus:ring-signal focus:ring-offset-2 focus:ring-offset-paper disabled:opacity-50 disabled:cursor-not-allowed',
        store.status.running
          ? 'bg-signal shadow-glow-signal'
          : 'bg-idle-surface border border-border hover:border-border/80'
      ]"
      role="switch"
      :aria-checked="store.status.running"
      aria-label="Toggle Relay Server"
    >
      <span
        aria-hidden="true"
        :class="[
          'pointer-events-none inline-flex h-13 w-13 transform items-center justify-center rounded-full bg-surface shadow-md transition-transform duration-300 ease-out',
          store.status.running ? 'translate-x-16' : 'translate-x-0'
        ]"
      >
        <span
          :class="[
            'w-3.5 h-3.5 rounded-full transition-all duration-300',
            store.status.running
              ? 'bg-signal shadow-[0_0_8px_#10B981]'
              : 'bg-ink-muted/50'
          ]"
        ></span>
      </span>
    </button>

    <!-- Status Label & Details -->
    <div class="mt-4 text-center">
      <div class="flex items-center justify-center gap-2">
        <span
          class="relative flex h-2.5 w-2.5"
          v-if="store.status.running"
        >
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-signal opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-signal"></span>
        </span>
        <span
          class="text-xs font-semibold uppercase tracking-wider transition-colors duration-200"
          :class="store.status.running ? 'text-signal' : 'text-ink-muted'"
        >
          {{ store.status.running ? 'Sharing Active' : 'Server Inactive' }}
        </span>
      </div>

      <p class="text-[11px] text-ink-muted mt-1">
        {{ store.status.running ? 'Connected to local Wi-Fi network' : 'Turn on to share files and clipboard' }}
      </p>

      <div v-if="store.error" class="mt-2.5 px-3 py-1.5 rounded-lg bg-red-950/40 border border-red-800/50 text-[11px] text-red-400 max-w-xs text-center">
        {{ store.error }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { useServerStore } from '../stores/serverStore';

const store = useServerStore();
</script>
