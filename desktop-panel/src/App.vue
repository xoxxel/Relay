<template>
  <main class="min-h-screen bg-paper text-ink flex flex-col items-center justify-between p-6 max-w-md mx-auto">
    <!-- Header -->
    <header class="w-full text-center pt-2 mb-2">
      <div class="inline-flex items-center gap-2 mb-1">
        <div class="w-6 h-6 rounded-lg bg-signal/15 border border-signal/30 flex items-center justify-center">
          <Radio class="w-3.5 h-3.5 text-signal" />
        </div>
        <h1 class="text-xl font-bold tracking-tight text-ink">Relay</h1>
      </div>
      <p class="text-xs text-ink-muted">Instant local network sharing</p>
    </header>

    <!-- Main Content Area -->
    <div class="w-full flex-1 flex flex-col justify-center py-2">
      <!-- Big Toggle Switch -->
      <ServerToggle />

      <!-- QR & Connection Details -->
      <ConnectionCard />

      <!-- Configuration & Status Card -->
      <div class="w-full bg-surface border border-border rounded-2xl p-4 text-xs space-y-3">
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-2 text-ink-secondary">
            <Folder class="w-3.5 h-3.5 text-ink-muted" />
            <span>Shared Folder</span>
          </div>
          <span
            class="font-mono text-[11px] text-ink truncate max-w-[210px] bg-paper px-2 py-0.5 rounded-md border border-border/50 select-text"
            :title="store.status.shared_folder"
          >
            {{ store.status.shared_folder || 'Loading...' }}
          </span>
        </div>

        <div class="flex items-center justify-between pt-2 border-t border-border-subtle">
          <div class="flex items-center gap-2 text-ink-secondary">
            <Server class="w-3.5 h-3.5 text-ink-muted" />
            <span>Port</span>
          </div>
          <span class="font-mono text-[11px] text-ink bg-paper px-2 py-0.5 rounded-md border border-border/50 select-text">
            {{ store.status.port }}
          </span>
        </div>

        <div class="flex items-center justify-between pt-2 border-t border-border-subtle">
          <div class="flex items-center gap-2 text-ink-secondary">
            <Wifi class="w-3.5 h-3.5 text-ink-muted" />
            <span>Local Hostname</span>
          </div>
          <span class="font-mono text-[11px] text-ink bg-paper px-2 py-0.5 rounded-md border border-border/50 select-text">
            {{ store.status.mdns_hostname }}
          </span>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <footer class="w-full text-center text-[11px] text-ink-muted pt-4">
      <span>Relay v0.1.0 • Fast &amp; Private</span>
    </footer>
  </main>
</template>

<script setup>
import { onMounted } from 'vue';
import { Radio, Folder, Server, Wifi } from 'lucide-vue-next';
import ServerToggle from './components/ServerToggle.vue';
import ConnectionCard from './components/ConnectionCard.vue';
import { useServerStore } from './stores/serverStore';

const store = useServerStore();

onMounted(() => {
  store.fetchStatus();
});
</script>
