import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useServerStore = defineStore('server', () => {
  const status = ref({
    running: false,
    port: 8420,
    lan_ip: '',
    lan_url: '',
    mdns_hostname: 'relay.local',
    shared_folder: '',
    connected_clients: 0
  });

  const clips = ref([]);
  const activities = ref([]);
  const loading = ref(false);
  const error = ref(null);
  let ws = null;

  async function invokeTauri(cmd, args = {}) {
    if (typeof window !== 'undefined' && (window.__TAURI_INTERNALS__ || window.__TAURI__)) {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke(cmd, args);
    } else {
      console.warn(`Tauri invoke "${cmd}" called outside Tauri environment.`);
      if (cmd === 'get_server_status') return status.value;
      if (cmd === 'start_server') {
        return {
          ...status.value,
          running: true,
          lan_ip: '192.168.1.100',
          lan_url: `http://192.168.1.100:${status.value.port}`
        };
      }
      if (cmd === 'stop_server') {
        return { ...status.value, running: false };
      }
      if (cmd === 'pick_shared_folder') {
        return '/home/user/Shared';
      }
      if (cmd === 'copy_to_system_clipboard') {
        if (navigator.clipboard) await navigator.clipboard.writeText(args.text);
        return null;
      }
      return null;
    }
  }

  function addActivity(text, type = 'info') {
    activities.value.unshift({
      id: Date.now() + Math.random(),
      text,
      type,
      time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })
    });
    if (activities.value.length > 50) activities.value.pop();
  }

  function connectWs() {
    if (ws) {
      try { ws.close(); } catch (_) {}
    }

    const host = status.value.running ? `127.0.0.1:${status.value.port}` : '127.0.0.1:8420';
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${host}/ws`;

    try {
      ws = new WebSocket(wsUrl);

      ws.onopen = () => {
        addActivity('Connected to real-time engine', 'system');
      };

      ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data);
          handleWsMessage(msg);
        } catch (e) {
          console.error('Failed to parse WS event:', e);
        }
      };

      ws.onclose = () => {
        if (status.value.running) {
          setTimeout(connectWs, 3000);
        }
      };
    } catch (e) {
      console.error('WebSocket connection error:', e);
    }
  }

  function handleWsMessage(msg) {
    if (msg.type === 'client_count') {
      status.value.connected_clients = msg.data.count;
      addActivity(`Active connections: ${msg.data.count}`, 'system');
    } else if (msg.type === 'clip_added') {
      const exists = clips.value.some(c => c.id === msg.data.id);
      if (!exists) {
        clips.value.unshift(msg.data);
        addActivity(`New clipboard text from ${msg.data.device_label || 'device'}`, 'clip');
      }
    } else if (msg.type === 'clip_removed') {
      clips.value = clips.value.filter(c => c.id !== msg.data.id);
      addActivity('Clipboard item removed', 'clip');
    } else if (msg.type === 'file_added') {
      addActivity(`File uploaded: ${msg.data.name}`, 'file');
    } else if (msg.type === 'file_removed') {
      addActivity(`File deleted: ${msg.data.path}`, 'file');
    }
  }

  async function fetchClips() {
    try {
      const port = status.value.port || 8420;
      const res = await fetch(`http://127.0.0.1:${port}/api/clips`);
      if (res.ok) {
        const data = await res.json();
        clips.value = Array.isArray(data) ? data : [];
      }
    } catch (_) {}
  }

  async function sendClip(content) {
    if (!content || !content.trim()) return;
    try {
      const port = status.value.port || 8420;
      const res = await fetch(`http://127.0.0.1:${port}/api/clips`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content, device_label: 'Desktop Host' })
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const newClip = await res.json();
      const exists = clips.value.some(c => c.id === newClip.id);
      if (!exists) clips.value.unshift(newClip);
      return newClip;
    } catch (err) {
      console.error('Failed to send clip:', err);
      throw err;
    }
  }

  async function deleteClip(id) {
    try {
      const port = status.value.port || 8420;
      await fetch(`http://127.0.0.1:${port}/api/clips/${id}`, { method: 'DELETE' });
      clips.value = clips.value.filter(c => c.id !== id);
    } catch (err) {
      console.error('Failed to delete clip:', err);
    }
  }

  async function copyToClipboard(text) {
    try {
      await invokeTauri('copy_to_system_clipboard', { text });
      if (navigator.clipboard) {
        await navigator.clipboard.writeText(text);
      }
      return true;
    } catch (err) {
      console.error('Failed to copy to clipboard:', err);
      return false;
    }
  }

  async function fetchStatus() {
    try {
      loading.value = true;
      error.value = null;
      const res = await invokeTauri('get_server_status');
      if (res) {
        status.value = res;
        if (res.running) {
          connectWs();
          fetchClips();
        }
      }
    } catch (err) {
      error.value = err?.message || String(err);
      console.error('Failed to get server status:', err);
    } finally {
      loading.value = false;
    }
  }

  async function startServer(port = undefined) {
    try {
      loading.value = true;
      error.value = null;
      const res = await invokeTauri('start_server', { port });
      if (res) {
        status.value = res;
        addActivity(`Server started on port ${res.port}`, 'system');
        connectWs();
        fetchClips();
      }
    } catch (err) {
      error.value = err?.message || String(err);
      console.error('Failed to start server:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function stopServer() {
    try {
      loading.value = true;
      error.value = null;
      const res = await invokeTauri('stop_server');
      if (res) {
        status.value = res;
        addActivity('Server stopped', 'system');
        if (ws) {
          try { ws.close(); } catch (_) {}
          ws = null;
        }
      }
    } catch (err) {
      error.value = err?.message || String(err);
      console.error('Failed to stop server:', err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function toggleServer() {
    if (status.value.running) {
      await stopServer();
    } else {
      await startServer();
    }
  }

  async function pickFolder() {
    try {
      const selected = await invokeTauri('pick_shared_folder');
      if (selected) {
        status.value.shared_folder = selected;
        addActivity(`Shared folder changed: ${selected}`, 'system');
      }
    } catch (err) {
      console.error('Failed to pick shared folder:', err);
    }
  }

  return {
    status,
    clips,
    activities,
    loading,
    error,
    fetchStatus,
    startServer,
    stopServer,
    toggleServer,
    pickFolder,
    copyToClipboard,
    sendClip,
    deleteClip,
    fetchClips,
    addActivity
  };
});
