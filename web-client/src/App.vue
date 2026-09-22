<script setup>
import { computed, ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue';
import { Folder, Files, Clipboard, Search, Upload, FolderPlus, RefreshCw, Download, Trash2, Copy, Check, X, FileText, Image, Music, Film, ChevronRight, Save, Send, Pencil, Laptop, LayoutGrid, List } from 'lucide-vue-next';
import ClipboardCard from './components/ClipboardCard.vue';
import { lockPageScroll, trackViewport } from './viewport';
import { files, clips, currentPath, activeTab, loading, error, connected, uploadProgress, init, fetchFiles, openFolder, downloadFile, deleteFile, createFolder, uploadFiles, fetchClips, sendClip, deleteClip, copyText, deviceLabel, setDeviceLabel } from './relay';
const composerInput = ref(null);
let unlockPage, stopViewportTracking;
function fitComposer() {
  const input = composerInput.value;
  if (!input) return;
  input.style.height = 'auto';
  input.style.height = `${input.scrollHeight}px`;
}
const query = ref(''), sort = ref('name'), grid = ref(false), draft = ref(''), sending = ref(false);
const deviceName = ref(deviceLabel()), notice = ref(''), manualCopy = ref(null), picker = ref(null);
const viewer = ref(null), selected = ref(null), busy = ref(false), saving = ref(false), previewError = ref('');
const content = ref(''), original = ref(''), mediaUrl = ref(''), kind = ref('unsupported');
let timer, request = 0;
watch(draft, () => nextTick(fitComposer));
const dirty = computed(() => content.value !== original.value);
const segments = computed(() => currentPath.value.split('/').filter(Boolean));
const visibleFiles = computed(() => files.value.filter(f => f.name.toLowerCase().includes(query.value.toLowerCase())).slice().sort((a,b) => {
  if (a.is_dir !== b.is_dir) return a.is_dir ? -1 : 1;
  if (sort.value === 'size') return b.size_bytes - a.size_bytes;
  if (sort.value === 'date') return new Date(b.modified_at || 0) - new Date(a.modified_at || 0);
  return a.name.localeCompare(b.name, undefined, { numeric: true });
}));
const visibleClips = computed(() => clips.value.filter(c => c.content.toLowerCase().includes(query.value.toLowerCase())));
watch(activeTab, () => { query.value = ''; error.value = null; if (activeTab.value === 'clipboard') fetchClips(); });
watch(currentPath, () => { query.value = ''; });
function toast(text) { notice.value = text; clearTimeout(timer); timer = setTimeout(() => notice.value = '', 4500); }
function size(n) { if (!n) return '0 B'; const i = Math.min(3, Math.floor(Math.log(n)/Math.log(1024))); return `${(n/1024**i).toFixed(i ? 1 : 0)} ${['B','KB','MB','GB'][i]}`; }
function date(s) { return s ? new Date(s).toLocaleString([], { month:'short', day:'numeric', hour:'2-digit', minute:'2-digit' }) : '—'; }
function fileKind(f) {
  const mime = f.mime_type || '', ext = f.name.split('.').pop().toLowerCase();
  if (mime.startsWith('image/')) return 'image';
  if (mime.startsWith('audio/')) return 'audio';
  if (mime.startsWith('video/')) return 'video';
  if (mime.startsWith('text/') || ['txt','md','json','js','ts','vue','css','html','xml','yaml','yml','toml','csv','log','rs','py','sh','ini','conf','sql','svg'].includes(ext)) return 'text';
  return 'unsupported';
}
function icon(f) { return f.is_dir ? Folder : ({image:Image,audio:Music,video:Film}[fileKind(f)] || FileText); }
async function copy(clip) { try { await copyText(clip.content); toast('Copied to this device'); } catch (e) { manualCopy.value = clip.content; error.value = e.message; } }
async function removeClip(clip) { if (!confirm('Delete this shared clipboard item?')) return; try { await deleteClip(clip.id); toast('Clipboard item deleted'); } catch(e) { error.value = e.message; } }
async function send() { if (!draft.value.trim() || sending.value) return; sending.value = true; try { await sendClip(draft.value); draft.value = ''; toast('Shared with your devices'); } catch(e) { error.value = `Could not send: ${e.message}`; } finally { sending.value = false; } }
function renameDevice() { const name = prompt('Name this device', deviceName.value); if(name?.trim()) { setDeviceLabel(name); deviceName.value = deviceLabel(); } }
function chooseFiles(e) { uploadFiles(e.target.files); e.target.value = ''; }
function dropFiles(e) { if (activeTab.value === 'files') uploadFiles(e.dataTransfer.files); }
function releaseMedia() { if (mediaUrl.value) URL.revokeObjectURL(mediaUrl.value); mediaUrl.value = ''; }
async function open(f) {
  if (f.is_dir) return openFolder(f.path);
  selected.value = f; content.value = ''; original.value = ''; previewError.value = ''; kind.value = fileKind(f); releaseMedia();
  await nextTick();
  unlockPage = lockPageScroll();
  viewer.value.showModal();
  const ticket = ++request;
  if (kind.value === 'unsupported') return;
  const limit = kind.value === 'text' ? 1024*1024 : 25*1024*1024;
  if (f.size_bytes > limit) { previewError.value = `Preview is limited to ${size(limit)}. Download this file to open it.`; return; }
  busy.value = true;
  try {
    const endpoint = kind.value === 'text' ? 'text' : 'download';
    const res = await fetch(`/api/files/${endpoint}?path=${encodeURIComponent(f.path)}`);
    if (!res.ok) throw new Error(res.status === 415 ? 'This file is not editable UTF-8 text.' : `Could not open file (HTTP ${res.status}).`);
    if (kind.value === 'text') { const data = await res.json(); if (ticket === request) content.value = original.value = data.content; }
    else { const blob = await res.blob(); if (ticket === request) mediaUrl.value = URL.createObjectURL(blob); }
  } catch(e) { if(ticket === request) previewError.value = e.message; }
  finally { if(ticket === request) busy.value = false; }
}
function closeViewer() { if (saving.value || (dirty.value && !confirm('Discard your unsaved changes?'))) return; ++request; busy.value = false; viewer.value.close(); unlockPage?.(); unlockPage = null; selected.value = null; releaseMedia(); }
async function save() {
  if (!dirty.value || saving.value) return;
  saving.value = true; previewError.value = '';
  const updated = content.value;
  try {
    const res = await fetch('/api/files/text', {method:'PUT', headers:{'Content-Type':'application/json'}, body:JSON.stringify({path:selected.value.path, content:updated, original:original.value})});
    if (!res.ok) throw new Error(res.status === 409 ? 'This file changed on another device. Copy your edits, close this window and reopen the file before saving.' : `Could not save (HTTP ${res.status}). Your edits are still here.`);
    original.value = updated; toast('File saved'); fetchFiles();
  } catch(e) { previewError.value = e.message; } finally { saving.value = false; }
}
function beforeUnload(e) { if (dirty.value && selected.value) { e.preventDefault(); e.returnValue = ''; } }
onMounted(() => { stopViewportTracking = trackViewport(); init(); window.addEventListener('beforeunload', beforeUnload); });
onBeforeUnmount(() => { unlockPage?.(); stopViewportTracking?.(); releaseMedia(); clearTimeout(timer); window.removeEventListener('beforeunload', beforeUnload); });
</script>

<template>
  <div class="workspace" @dragover.prevent @drop.prevent="dropFiles">
    <aside class="sidebar">
      <a class="brand" href="/" aria-label="Relay home"><img src="/relay-icon.svg" alt=""/><span>relay<span class="brand-dot">.</span></span></a>
      <p class="eyebrow nav-label">YOUR WORKSPACE</p>
      <span class="mobile-connection"><span :class="['status-dot',{online:connected}]"></span>{{ connected ? 'Connected' : 'Offline' }}</span>
      <nav aria-label="Workspace">
        <button :class="['nav-item',{active:activeTab==='files'}]" @click="activeTab='files'"><Files/> Files <span>{{ files.length }}</span></button>
        <button :class="['nav-item',{active:activeTab==='clipboard'}]" @click="activeTab='clipboard'"><Clipboard/> Clipboard <span>{{ clips.length }}</span></button>
      </nav>
      <div class="sidebar-bottom"><div class="network-card"><span :class="['status-dot',{online:connected}]"></span><div><strong>{{ connected ? 'Connected locally' : 'Reconnecting…' }}</strong><small>Your files stay on your network</small></div></div><button class="device" @click="renameDevice"><Laptop/><span>{{ deviceName }}</span><Pencil/></button></div>
    </aside>
    <div class="main-shell">
      <header class="topbar"><span>Workspace <ChevronRight/> <strong>{{ activeTab==='files'?'Files':'Clipboard' }}</strong></span><span class="connection"><span :class="['status-dot',{online:connected}]"></span>{{ connected?'Live sync':'Offline' }}</span></header>
      <main>
        <section class="page-heading"><div><p class="eyebrow">{{ activeTab==='files'?'A PLACE FOR EVERYTHING':'BETWEEN YOUR DEVICES' }}</p><h1>{{ activeTab==='files'?'Your files':'Shared clipboard' }}</h1><p>{{ activeTab==='files'?'Browse, preview and edit. Everything, close at hand.':'Send a thought, a link or a snippet. Pick it up anywhere.' }}</p></div><button v-if="activeTab==='files'" class="primary" :disabled="!!uploadProgress" @click="picker.click()"><Upload/> Upload files</button></section>
        <input ref="picker" type="file" multiple hidden @change="chooseFiles"/>
        <div v-if="error" class="message error" role="alert"><span>{{ error }}</span><button aria-label="Dismiss error" @click="error=null"><X/></button></div>
        <section v-if="manualCopy !== null" class="manual-copy"><label for="manual">Select this text and use your device’s Copy command</label><textarea id="manual" readonly :value="manualCopy" @focus="$event.target.select()"></textarea><button @click="manualCopy=null">Done</button></section>
        <template v-if="activeTab==='files'">
          <div class="toolbar files-toolbar"><div class="search"><Search/><input v-model="query" aria-label="Search files" placeholder="Search this folder…"/></div><select v-model="sort" aria-label="Sort files"><option value="name">Name A–Z</option><option value="date">Recently modified</option><option value="size">Largest first</option></select><div class="view-switch"><button :class="{chosen:!grid}" aria-label="List view" :aria-pressed="!grid" @click="grid=false"><List/></button><button :class="{chosen:grid}" aria-label="Grid view" :aria-pressed="grid" @click="grid=true"><LayoutGrid/></button></div></div>
          <div class="folderbar"><nav class="breadcrumbs" aria-label="Folder path"><button @click="openFolder('')"><Folder/> Shared</button><template v-for="(part,i) in segments" :key="i"><ChevronRight/><button @click="openFolder(segments.slice(0,i+1).join('/'))">{{ part }}</button></template></nav><div class="actions"><button title="New folder" aria-label="New folder" @click="createFolder"><FolderPlus/></button><button title="Refresh" aria-label="Refresh files" :disabled="loading" @click="fetchFiles"><RefreshCw :class="{spin:loading}"/></button></div></div>
          <div v-if="uploadProgress" class="upload-status" role="status"><div><Upload/><strong>{{ uploadProgress.name }}</strong><span>{{ uploadProgress.done+1 }} / {{ uploadProgress.total }}</span></div><progress max="100" :value="uploadProgress.percent"></progress></div>
          <div v-if="loading && !files.length" class="empty" role="status"><RefreshCw class="spin"/><h2>Loading your files…</h2></div>
          <div v-else-if="!visibleFiles.length" class="empty"><Folder/><h2>{{ query?'No matching files':'Make room for your next idea' }}</h2><p>{{ query?'Try a different file name.':'Drop files here or use Upload files to get started.' }}</p></div>
          <section v-else :class="['file-list',{grid}]" aria-label="Files">
            <div v-if="!grid" class="list-heading"><span>Name</span><span>Size</span><span>Modified</span><span>Actions</span></div>
            <article v-for="file in visibleFiles" :key="file.path" class="file-row">
              <button class="file-open" @click="open(file)"><span :class="['file-icon',file.is_dir?'folder':fileKind(file)]"><component :is="icon(file)"/></span><span class="file-label"><strong>{{ file.name }}</strong><small>{{ file.is_dir?'Folder':(file.name.split('.').pop().toUpperCase()+' file') }}<span v-if="!file.is_dir" class="mobile-file-size"> · {{ size(file.size_bytes) }}</span></small></span></button>
              <span class="file-size">{{ file.is_dir?'—':size(file.size_bytes) }}</span><span class="file-date">{{ date(file.modified_at) }}</span>
              <div class="actions"><button v-if="!file.is_dir" :aria-label="`Download ${file.name}`" title="Download" @click="downloadFile(file)"><Download/></button><button class="danger" :aria-label="`Delete ${file.name}`" title="Delete" @click="deleteFile(file)"><Trash2/></button></div>
            </article>
          </section>
          <footer class="file-footer"><span>{{ visibleFiles.length }} {{ visibleFiles.length===1?'item':'items' }}</span><span>Drag & drop to upload · Click a file to preview</span></footer>
        </template>
        <template v-else>
          <form class="composer" @submit.prevent="send"><label for="clip-draft">Share something</label><textarea ref="composerInput" id="clip-draft" rows="4" v-model="draft" :disabled="sending" placeholder="Paste a link, write a note, or share a code snippet…" @keydown.ctrl.enter.prevent="send" @keydown.meta.enter.prevent="send"></textarea><div><span>From {{ deviceName }} · Ctrl / ⌘ + Enter to send</span><button class="primary" :disabled="sending || !draft.trim()"><Send/>{{ sending?'Sending…':'Share text' }}</button></div></form>
          <div class="toolbar clipboard-toolbar"><h2>Recent notes <span>{{ clips.length }}</span></h2><div class="search"><Search/><input v-model="query" placeholder="Search clipboard…" aria-label="Search clipboard"/></div><button aria-label="Refresh clipboard" @click="fetchClips"><RefreshCw/></button></div>
          <div v-if="!visibleClips.length" class="empty"><Clipboard/><h2>{{ query?'No matching notes':'Your clipboard, in sync' }}</h2><p>Share text above to make it available on your devices.</p></div>
          <section class="clip-grid" aria-label="Shared notes">
            <ClipboardCard v-for="clip in visibleClips" :key="clip.id" :clip="clip" @copy="copy" @delete="removeClip" />
          </section>
        </template>
      </main>
    </div>
    <div v-if="notice" class="toast" role="status"><Check/>{{ notice }}</div>
    <dialog ref="viewer" class="viewer" aria-labelledby="preview-title" @cancel.prevent="closeViewer" @click="($event.target===viewer) && closeViewer()">
      <template v-if="selected"><header><div><span class="file-icon"><component :is="icon(selected)"/></span><div><h2 id="preview-title">{{ selected.name }}</h2><small>{{ size(selected.size_bytes) }} · {{ kind==='text'?'Text editor':'Preview' }}{{ dirty?' · Unsaved changes':'' }}</small></div></div><button aria-label="Close preview" :disabled="saving" @click="closeViewer"><X/></button></header><div :class="['viewer-body', {'text-preview':kind==='text' && !busy} ]"><p v-if="busy" class="empty">Loading preview…</p><div v-if="previewError" class="message error" role="alert">{{ previewError }}</div><textarea v-if="kind==='text' && !busy && (!previewError || content || dirty)" v-model="content" :disabled="saving" class="editor" spellcheck="false" aria-label="File content" @keydown.ctrl.s.prevent="save" @keydown.meta.s.prevent="save"></textarea><img v-if="kind==='image' && mediaUrl" :src="mediaUrl" :alt="selected.name" @error="previewError='This image format cannot be previewed. Download it to open.'"/><video v-if="kind==='video' && mediaUrl" :src="mediaUrl" controls @error="previewError='Your browser cannot play this format. Download it to open.'"></video><audio v-if="kind==='audio' && mediaUrl" :src="mediaUrl" controls @error="previewError='Your browser cannot play this format. Download it to open.'"></audio><div v-if="kind==='unsupported'" class="empty"><FileText/><h2>Open with your favorite app</h2><p>Download this file to view or edit it on your device.</p></div></div><footer><button @click="downloadFile(selected)"><Download/> Download</button><span v-if="kind==='text'">UTF-8 · Up to 1 MB</span><button v-if="kind==='text'" class="primary" :disabled="busy || saving || !dirty" @click="save"><Save/>{{ saving?'Saving…':'Save changes' }}</button></footer></template>
    </dialog>
  </div>
</template>
