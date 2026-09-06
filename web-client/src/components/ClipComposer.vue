<template>
  <form class="sticky top-0 z-20 bg-paper/95 backdrop-blur border-b border-border pb-3 animate-float-in" @submit.prevent="submit">
    <div class="flex gap-2">
      <input
        v-model="text"
        type="text"
        inputmode="text"
        placeholder="Type something to share..."
        class="flex-1 min-w-0 bg-surface border border-border rounded-xl px-3.5 py-2.5 text-sm text-ink placeholder:text-ink-muted/70 focus:outline-none focus:border-signal/60 focus:ring-2 focus:ring-signal/20 transition-all"
        @keydown.meta.enter="submit"
        @keydown.ctrl.enter="submit"
      />
      <button
        type="submit"
        class="px-3.5 rounded-xl bg-signal hover:bg-signal-hover text-paper active:scale-95 transition-all flex items-center justify-center disabled:opacity-40 disabled:cursor-not-allowed"
        :disabled="!text.trim()"
        aria-label="Send"
      >
        <Send class="w-4 h-4" />
      </button>
    </div>
    <p v-if="sendError" class="text-[10px] text-red-400 mt-1.5 px-1">{{ sendError }}</p>
  </form>
</template>

<script setup>
import { ref } from 'vue';
import { Send } from 'lucide-vue-next';
import { sendClip } from '../relay';

const text = ref('');
const sendError = ref('');

async function submit() {
  if (!text.value.trim()) return;
  sendError.value = '';
  try {
    await sendClip(text.value);
    text.value = '';
  } catch (err) {
    sendError.value = 'Failed to send: ' + (err.message || err);
  }
}
</script>
