<template>
  <transition
    enter-active-class="transition duration-200 ease-out"
    enter-from-class="opacity-0 scale-95"
    enter-to-class="opacity-100 scale-100"
    leave-active-class="transition duration-150 ease-in"
    leave-from-class="opacity-100 scale-100"
    leave-to-class="opacity-0 scale-95"
  >
    <div
      v-if="store.status.running"
      class="w-full bg-surface border border-border rounded-2xl p-5 mb-5 shadow-glow-subtle flex flex-col sm:flex-row items-center justify-between gap-5"
    >
      <!-- QR Frame -->
      <div class="bg-white p-2.5 rounded-xl shadow-sm flex items-center justify-center flex-shrink-0">
        <qrcode-vue
          :value="store.status.lan_url || 'http://127.0.0.1:8420'"
          :size="124"
          level="M"
          render-as="svg"
        />
      </div>

      <!-- Connection Info -->
      <div class="flex-1 w-full text-center sm:text-left space-y-2.5">
        <div>
          <span class="text-[11px] uppercase tracking-wider text-ink-muted font-medium block">
            Scan QR or Open in Browser
          </span>
          <div class="font-mono text-sm font-semibold text-ink break-all mt-0.5 select-text">
            {{ store.status.lan_url }}
          </div>
        </div>

        <div class="flex items-center justify-center sm:justify-start gap-2 pt-0.5">
          <button
            @click="copyUrl"
            type="button"
            class="inline-flex items-center gap-1.5 px-3.5 py-1.5 text-xs font-medium rounded-lg border border-border bg-surface-elevated hover:bg-border/60 hover:text-white text-ink transition-all active:scale-95"
          >
            <component :is="copied ? Check : Copy" class="w-3.5 h-3.5" :class="copied ? 'text-signal' : 'text-ink-muted'" />
            <span>{{ copied ? 'Copied' : 'Copy URL' }}</span>
          </button>
        </div>
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

async function copyUrl() {
  if (!store.status.lan_url) return;
  try {
    if (navigator.clipboard) {
      await navigator.clipboard.writeText(store.status.lan_url);
    }
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (e) {
    console.error('Failed to copy URL:', e);
  }
}
</script>
