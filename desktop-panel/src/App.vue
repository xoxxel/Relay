<template>
  <main class="min-h-screen bg-paper text-ink flex flex-col items-center justify-between p-6 max-w-md mx-auto">
    <!-- Header -->
    <header class="w-full flex items-center justify-between pt-1 mb-2">
      <div class="flex items-center gap-3">
        <img src="/relay-icon.svg" alt="Relay" class="w-8 h-8 opacity-90" draggable="false" />
        <div>
          <h1 class="text-lg font-bold tracking-tight text-ink leading-none">Relay</h1>
          <p class="text-[11px] text-ink-muted mt-1">Private sharing, locally.</p>
        </div>
      </div>
      <span class="text-[10px] font-mono text-ink-muted border border-border rounded-full px-2 py-1">v0.1</span>
    </header>

    <div class="w-full flex-1 flex flex-col justify-center py-3">
      <ServerToggle />
      <ConnectionCard />
      <div class="w-full bg-surface border border-border rounded-2xl p-4 text-xs space-y-3">
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-2 text-ink-secondary"><Folder class="w-3.5 h-3.5 text-signal" /><span>Shared Folder</span></div>
          <span class="font-mono text-[11px] text-ink truncate max-w-[210px] bg-paper px-2 py-1 rounded-md border border-border/50 select-text" :title="store.status.shared_folder">{{ store.status.shared_folder || 'Loading...' }}</span>
        </div>
        <div class="flex items-center justify-between pt-2 border-t border-border-subtle">
          <div class="flex items-center gap-2 text-ink-secondary"><Server class="w-3.5 h-3.5 text-ink-muted" /><span>Port</span></div>
          <span class="font-mono text-[11px] text-ink bg-paper px-2 py-1 rounded-md border border-border/50 select-text">{{ store.status.port }}</span>
        </div>
        <div class="flex items-center justify-between pt-2 border-t border-border-subtle">
          <div class="flex items-center gap-2 text-ink-secondary"><Wifi class="w-3.5 h-3.5 text-ink-muted" /><span>Local Hostname</span></div>
          <span class="font-mono text-[11px] text-ink bg-paper px-2 py-1 rounded-md border border-border/50 select-text">{{ store.status.mdns_hostname }}</span>
        </div>
      </div>
    </div>

    <footer class="w-full flex items-center justify-center gap-2 text-[10px] text-ink-muted pt-4"><span class="w-1.5 h-1.5 rounded-full bg-signal/60"></span><span>Fast, private, and local by design</span></footer>
  </main>
</template>

<script setup>
import { onMounted } from 'vue';
import { Folder, Server, Wifi } from 'lucide-vue-next';
import ServerToggle from './components/ServerToggle.vue';
import ConnectionCard from './components/ConnectionCard.vue';
import { useServerStore } from './stores/serverStore';
const store = useServerStore();
onMounted(() => { store.fetchStatus(); });
</script>