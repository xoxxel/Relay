<template>
  <section class="flex flex-col items-center justify-center py-6 select-none animate-float-in-delay">
    <button type="button" @click="store.toggleServer" :disabled="store.loading" :class="['chunky-toggle relative inline-flex h-20 w-40 items-center rounded-[2rem] p-2 transition-all duration-500 ease-out focus:outline-none focus:ring-2 focus:ring-signal/60 focus:ring-offset-4 focus:ring-offset-paper disabled:opacity-50 disabled:cursor-not-allowed', store.status.running ? 'bg-signal is-on' : 'bg-idle-surface border border-border hover:border-signal/40']" role="switch" :aria-checked="store.status.running" aria-label="Toggle Relay Server">
      <span aria-hidden="true" :class="['chunky-toggle-knob pointer-events-none inline-flex size-16 transform items-center justify-center rounded-[1.5rem] bg-surface transition-transform duration-500 ease-out', store.status.running ? 'translate-x-20' : 'translate-x-0']">
        <span :class="['size-3 rounded-full transition-all duration-500', store.status.running ? 'bg-signal shadow-[0_0_12px_rgba(45,212,191,.9)]' : 'bg-ink-muted/50']"></span>
      </span>
      <span class="absolute inset-0 flex items-center justify-between px-6 pointer-events-none text-[9px] font-bold uppercase tracking-[.18em]" :class="store.status.running ? 'text-paper/60' : 'text-ink-muted/50'"><span>Off</span><span>On</span></span>
    </button>
    <div class="mt-5 text-center">
      <div class="flex items-center justify-center gap-2">
        <span v-if="store.status.running" class="status-pulse size-2.5 rounded-full bg-signal"></span>
        <span class="text-xs font-semibold uppercase tracking-[.16em] transition-colors duration-300" :class="store.status.running ? 'text-signal' : 'text-ink-muted'">{{ store.status.running ? 'Sharing Active' : 'Server Inactive' }}</span>
      </div>
      <p class="text-[11px] text-ink-muted mt-1.5">{{ store.status.running ? 'Ready for devices on your local network' : 'Turn on to share files and clipboard' }}</p>
      <div v-if="store.error" class="mt-3 px-3 py-1.5 rounded-lg bg-red-950/40 border border-red-800/50 text-[11px] text-red-400 max-w-xs text-center">{{ store.error }}</div>
    </div>
  </section>
</template>
<script setup>
import { useServerStore } from '../stores/serverStore';
const store = useServerStore();
</script>
