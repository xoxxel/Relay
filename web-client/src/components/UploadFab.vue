<template>
  <div class="pointer-events-none fixed inset-x-0 bottom-20 z-30 max-w-lg mx-auto px-6 flex justify-end">
    <input
      ref="input"
      type="file"
      multiple
      class="hidden"
      @change="onChange"
    />

    <button
      class="pointer-events-auto w-14 h-14 rounded-2xl bg-signal hover:bg-signal-hover text-paper shadow-glow-signal flex items-center justify-center active:scale-90 transition-all"
      aria-label="Upload files"
      :disabled="!!uploadProgress"
      @click="openPicker"
    >
      <Upload class="w-6 h-6" :class="{ 'animate-pulse': uploadProgress }" />
    </button>

    <div
      v-if="uploadProgress"
      class="absolute bottom-0 right-6 mb-16 w-64 bg-surface border border-border rounded-xl p-3 shadow-glow-subtle"
    >
      <p class="text-[10px] text-ink-muted font-mono truncate mb-1.5">
        {{ uploadProgress.name }} ({{ uploadProgress.done + 1 }}/{{ uploadProgress.total }})
      </p>
      <div class="h-1.5 rounded-full bg-idle-surface overflow-hidden">
        <div
          class="h-full bg-signal rounded-full transition-all duration-200"
          :style="{ width: uploadProgress.percent + '%' }"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { Upload } from 'lucide-vue-next';
import { uploadProgress, uploadFiles } from '../relay';

const input = ref(null);

function openPicker() {
  if (input.value) input.value.click();
}

function onChange(event) {
  uploadFiles(event.target.files);
  event.target.value = '';
}
</script>