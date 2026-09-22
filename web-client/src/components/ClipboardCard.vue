<script setup>
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue';
import { Laptop, Copy, Trash2, ChevronDown, ChevronUp } from 'lucide-vue-next';
const props = defineProps({ clip: { type: Object, required: true } });
defineEmits(['copy', 'delete']);
const paragraph = ref(null);
const expanded = ref(false);
const truncated = ref(false);
let observer;
function measure() {
  if (!expanded.value && paragraph.value) {
    truncated.value = paragraph.value.scrollHeight > paragraph.value.clientHeight + 1;
  }
}
watch(() => props.clip.content, async () => { await nextTick(); measure(); });
onMounted(() => {
  measure();
  observer = new ResizeObserver(measure);
  observer.observe(paragraph.value);
});
onBeforeUnmount(() => observer?.disconnect());
function date(value) {
  return value ? new Date(value).toLocaleString([], {
    month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit',
  }) : '';
}
</script>

<template>
  <article class="clip-card">
    <div class="clip-meta">
      <span><Laptop />{{ clip.device_label || 'Device' }}</span>
      <time :datetime="clip.created_at">{{ date(clip.created_at) }}</time>
    </div>
    <p ref="paragraph" :id="`clip-${clip.id}`" :class="['clip-content', { collapsed: !expanded }]" dir="auto">{{ clip.content }}</p>
    <button v-if="truncated || expanded" class="expand-note" :aria-expanded="expanded" :aria-controls="`clip-${clip.id}`" @click="expanded = !expanded">
      <component :is="expanded ? ChevronUp : ChevronDown" />
      {{ expanded ? 'Show less' : 'Show more' }}
    </button>
    <footer>
      <button @click="$emit('copy', clip)"><Copy /> Copy text</button>
      <button class="danger icon-button" aria-label="Delete note" @click="$emit('delete', clip)"><Trash2 /></button>
    </footer>
  </article>
</template>
