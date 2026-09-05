import { ref } from 'vue';

const files = ref([]);
const clips = ref([]);
const currentPath = ref('');
const activeTab = ref('files');
const loading = ref(false);
const error = ref(null);
const connected = ref(false);
const uploadProgress = ref(null);

let ws = null;
let wsRetry = null;

function statusOk(res) {
  return res.ok;
}

async function fetchFiles() {
  loading.value = true;
  error.value = null;
  try {
    const q = currentPath.value ? `?path=${encodeURIComponent(currentPath.value)}` : '';
    const res = await fetch(`/api/files${q}`);
    if (!statusOk(res)) throw new Error(`HTTP ${res.status}`);
    const data = await res.json();
    files.value = Array.isArray(data) ? data : [];
    connected.value = true;
  } catch (err) {
    error.value = 'Failed to connect to Relay server: ' + (err.message || err);
    connected.value = false;
  } finally {
    loading.value = false;
  }
}

function openFolder(path) {
  currentPath.value = path;
  fetchFiles();
}

function downloadFile(file) {
  const url = `/api/files/download?path=${encodeURIComponent(file.path)}`;
  const a = document.createElement('a');
  a.href = url;
  a.download = file.name;
  document.body.appendChild(a);
  a.click();
  a.remove();
}

async function deleteFile(file) {
  if (!window.confirm(`Delete "${file.name}"? This cannot be undone.`)) return;
  try {
    error.value = null;
    const res = await fetch(`/api/files?path=${encodeURIComponent(file.path)}`, { method: 'DELETE' });
    if (!statusOk(res)) throw new Error(`HTTP ${res.status}`);
    files.value = files.value.filter((f) => f.path !== file.path);
  } catch (err) {
    error.value = 'Failed to delete: ' + (err.message || err);
  }
}

async function createFolder() {
  const name = window.prompt('New folder name');
  if (!name || !name.trim()) return;
  const fullPath = currentPath.value
    ? `${currentPath.value}/${name.trim()}`
    : name.trim();
  try {
    error.value = null;
    const res = await fetch('/api/files/mkdir', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ path: fullPath }),
    });
    if (!statusOk(res)) throw new Error(`HTTP ${res.status}`);
    fetchFiles();
  } catch (err) {
    error.value = 'Failed to create folder: ' + (err.message || err);
  }
}

function uploadFiles(fileList) {
  const selected = Array.from(fileList || []);
  if (!selected.length) return;
  uploadProgress.value = {
    done: 0,
    total: selected.length,
    name: selected[0].name,
    percent: 0,
  };

  const run = async (index) => {
    if (index >= selected.length) {
      uploadProgress.value = null;
      fetchFiles();
      return;
    }
    const file = selected[index];
    uploadProgress.value = {
      done: index,
      total: selected.length,
      name: file.name,
      percent: 0,
    };

    const form = new FormData();
    form.append('path', currentPath.value);
    form.append('file', file, file.name);

    try {
      error.value = null;
      await new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest();
        xhr.open('POST', '/api/files/upload');
        xhr.upload.onprogress = (e) => {
          if (e.lengthComputable && uploadProgress.value) {
            uploadProgress.value.name = file.name;
            uploadProgress.value.percent = Math.round((e.loaded / e.total) * 90);
          }
        };
        xhr.onload = () =>
          xhr.status >= 200 && xhr.status < 300
            ? resolve()
            : reject(new Error(`Upload failed (HTTP ${xhr.status})`));
        xhr.onerror = () => reject(new Error('Upload failed'));
        xhr.send(form);
      });
      run(index + 1);
    } catch (err) {
      uploadProgress.value = null;
      error.value = err.message || String(err);
    }
  };

  run(0);
}

async function fetchClips() {
  try {
    const res = await fetch('/api/clips');
    if (statusOk(res)) {
      const data = await res.json();
      clips.value = Array.isArray(data) ? data : [];
    }
  } catch (_) {}
}

async function sendClip(content) {
  const text = content.trim();
  if (!text) return null;
  const device = localStorage.getItem('relay_device') || 'Mobile Device';
  const res = await fetch('/api/clips', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ content: text, device_label: device }),
  });
  if (!statusOk(res)) throw new Error(`HTTP ${res.status}`);
  const clip = await res.json();
  if (!clips.value.some((c) => c.id === clip.id)) clips.value.unshift(clip);
  return clip;
}

async function deleteClip(id) {
  const res = await fetch(`/api/clips/${id}`, { method: 'DELETE' });
  if (statusOk(res)) {
    clips.value = clips.value.filter((c) => c.id !== id);
  }
}

function copyText(text) {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(text);
  }
}

function deviceLabel() {
  return localStorage.getItem('relay_device') || 'Mobile Device';
}

function setDeviceLabel(label) {
  localStorage.setItem('relay_device', label.trim() || 'Mobile Device');
}

function connectWs() {
  if (ws) {
    try {
      ws.close();
    } catch (_) {}
  }

  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const wsUrl = `${protocol}//${window.location.host}/ws`;

  try {
    ws = new WebSocket(wsUrl);
    ws.onopen = () => {
      connected.value = true;
    };
    ws.onmessage = (event) => {
      try {
        handleWsMessage(JSON.parse(event.data));
      } catch (_) {}
    };
    ws.onclose = () => {
      connected.value = false;
      if (wsRetry) clearTimeout(wsRetry);
      wsRetry = setTimeout(connectWs, 3000);
    };
  } catch (_) {}
}

function handleWsMessage(msg) {
  if (msg.type === 'clip_added') {
    const exists = clips.value.some((c) => c.id === msg.data.id);
    if (!exists) clips.value.unshift(msg.data);
  } else if (msg.type === 'clip_removed') {
    clips.value = clips.value.filter((c) => c.id !== msg.data.id);
  } else if (msg.type === 'file_added' || msg.type === 'file_removed') {
    const path = msg.data.path || '';
    const dir = currentPath.value;
    const inCurrent = !dir || path.startsWith(dir + '/');
    if (inCurrent) fetchFiles();
  }
}

function init() {
  fetchFiles();
  fetchClips();
  connectWs();
}

export {
  files,
  clips,
  currentPath,
  activeTab,
  loading,
  error,
  connected,
  uploadProgress,
  init,
  fetchFiles,
  openFolder,
  downloadFile,
  deleteFile,
  createFolder,
  uploadFiles,
  fetchClips,
  sendClip,
  deleteClip,
  copyText,
  deviceLabel,
  setDeviceLabel,
};