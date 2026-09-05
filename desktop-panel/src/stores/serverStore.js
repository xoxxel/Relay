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

  const loading = ref(false);
  const error = ref(null);

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
      return null;
    }
  }

  async function fetchStatus() {
    try {
      loading.value = true;
      error.value = null;
      const res = await invokeTauri('get_server_status');
      if (res) status.value = res;
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
      if (res) status.value = res;
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
      if (res) status.value = res;
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

  return {
    status,
    loading,
    error,
    fetchStatus,
    startServer,
    stopServer,
    toggleServer
  };
});
