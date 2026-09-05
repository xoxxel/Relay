<template>
  <transition enter-active-class="transition duration-300 ease-out" enter-from-class="opacity-0 translate-y-2" enter-to-class="opacity-100 translate-y-0" leave-active-class="transition duration-150 ease-in" leave-from-class="opacity-100" leave-to-class="opacity-0">
    <div v-if="store.status.running" class="w-full bg-surface border border-signal/20 rounded-2xl p-4 mb-5 shadow-glow-subtle flex items-center gap-4 animate-float-in-delay-2">
      <div class="qr-shell bg-white p-2 rounded-xl shadow-sm flex items-center justify-center flex-shrink-0"><qrcode-vue :value="store.status.lan_url || 'http://127.0.0.1:8420'" :size="104" level="M" render-as="svg" /></div>
      <div class="flex-1 min-w-0">
        <span class="text-[10px] uppercase tracking-[.14em] text-signal font-semibold block">Scan to connect</span>
        <div class="font-mono text-xs font-semibold text-ink break-all mt-1 select-text">{{ store.status.lan_url }}</div>
        <button @click="copyUrl" type="button" class="inline-flex items-center gap-1.5 mt-3 px-3 py-1.5 text-[11px] font-medium rounded-lg border border-border bg-surface-elevated hover:border-signal/40 hover:text-signal text-ink transition-all active:scale-95"><component :is="copied ? Check : Copy" class="w-3.5 h-3.5" :class="copied ? 'text-signal' : 'text-ink-muted'" /><span>{{ copied ? 'Copied' : 'Copy URL' }}</span></button>
      </div>
    </div>
  </transition>
</template>
<script setup>
import { ref } from 'vue';
import QrcodeVue from 'qrcode.vue';
import { Copy, Check } from 'lucide-vue-next';
import { useServerStore } from '../stores/serverStore';
const store = useServerStore();
const copied = ref(false);
async function copyUrl() { if (!store.status.lan_url) return; try { await navigator.clipboard?.writeText(store.status.lan_url); copied.value = true; setTimeout(() => { copied.value = false; }, 2000); } catch (e) { console.error('Failed to copy URL:', e); } }
</script>
