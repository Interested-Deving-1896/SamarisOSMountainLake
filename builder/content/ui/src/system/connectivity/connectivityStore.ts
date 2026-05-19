import { kernelClient } from "../../os/kernel/kernelClient";

export type Network = { id: string; label: string; strength: number; secured: boolean; connected?: boolean; channel?: string; band?: string };
export type BluetoothDevice = { id: string; label: string; connected: boolean; paired?: boolean };
type ConnectivityState = {
  platform: string; wifiEnabled: boolean; bluetoothEnabled: boolean;
  currentNetworkId: string; currentNetworkLabel?: string; currentAddress?: string | null;
  networks: Network[]; devices: BluetoothDevice[];
  capabilities: { wifiToggle: boolean; wifiConnect: boolean; bluetoothToggle: boolean; bluetoothConnect: boolean };
};
type Listener = () => void;

class ConnectivityStore {
  private state: ConnectivityState = { platform: "unknown", wifiEnabled: true, bluetoothEnabled: false, currentNetworkId: "", networks: [], devices: [], capabilities: { wifiToggle: false, wifiConnect: false, bluetoothToggle: false, bluetoothConnect: false } };
  private listeners = new Set<Listener>();
  private initialized = false;
  private pollTimer: number | null = null;

  init() {
    if (this.initialized) return;
    this.initialized = true;
    this.pollTimer = window.setInterval(() => { void this.refresh(); }, 30000);
  }

  reset() {
    this.initialized = false;
    if (this.pollTimer) { clearInterval(this.pollTimer); this.pollTimer = null; }
    this.state = { platform: "unknown", wifiEnabled: true, bluetoothEnabled: false, currentNetworkId: "", networks: [], devices: [], capabilities: { wifiToggle: false, wifiConnect: false, bluetoothToggle: false, bluetoothConnect: false } };
    for (const l of this.listeners) l();
  }

  getState() { return this.state; }
  subscribe(listener: Listener) { this.listeners.add(listener); return () => this.listeners.delete(listener); }

  async toggleWifi() {
    if (!this.state.capabilities.wifiToggle) return;
    return kernelClient.request<ConnectivityState>({ type: "device.wifi.toggle", data: { enabled: !this.state.wifiEnabled } }).then((r) => { if (r.data) this.patch(r.data); return r.data; }).catch(() => null);
  }

  async toggleBluetooth() {
    if (!this.state.capabilities.bluetoothToggle) return;
    return kernelClient.request<ConnectivityState>({ type: "device.bluetooth.toggle", data: { enabled: !this.state.bluetoothEnabled } }, { timeoutMs: 25000 }).then((r) => { if (r.data) this.patch(r.data); return r.data; }).catch(() => null);
  }

  async connectNetwork(label: string, password?: string) {
    const network = this.state.networks.find((n) => n.label === label);
    return kernelClient.request<ConnectivityState>({ type: "device.wifi.connect", data: { ssid: label, password } }).then((r) => { if (r.data) this.patch(r.data); return r.data; }).catch(() => null);
  }

  async disconnectNetwork() {
    return kernelClient.request<ConnectivityState>({ type: "device.wifi.disconnect", data: {} }).then((r) => { if (r.data) this.patch(r.data); return r.data; }).catch(() => null);
  }

  async forgetNetwork(ssid: string) {
    return kernelClient.request<{ ok: boolean }>({ type: "device.wifi.forget", data: { ssid } }).catch(() => null);
  }

  async getSavedNetworks() {
    return kernelClient.request<{ ssid: string }[]>({ type: "device.wifi.saved", data: {} }).then((r) => r.data || []).catch(() => []);
  }

  async autoConnect() {
    return kernelClient.request<{ ok: boolean; ssid?: string }>({ type: "device.wifi.autoconnect", data: {} }).catch(() => ({ ok: false }));
  }

  async connectBluetooth(id: string) {
    return kernelClient.request<ConnectivityState>({ type: "device.bluetooth.connect", data: { id } }).then((r) => { if (r.data) this.patch(r.data); return r.data; }).catch(() => null);
  }

  async disconnectBluetooth(id: string) {
    return kernelClient.request<ConnectivityState>({ type: "device.bluetooth.disconnect", data: { id } }).then((r) => { if (r.data) this.patch(r.data); return r.data; }).catch(() => null);
  }

  async unpairBluetooth(id: string) {
    return kernelClient.request<ConnectivityState>({ type: "device.bluetooth.unpair", data: { id } }).then((r) => { if (r.data) this.patch(r.data); return r.data; }).catch(() => null);
  }

  async scanBluetooth() {
    return kernelClient.request<any[]>({ type: "device.bluetooth.scan", data: {} }, { timeoutMs: 20000 }).then((r) => r.data || []).catch(() => []);
  }

  async refresh() {
    try {
      const r = await kernelClient.request<ConnectivityState>({ type: "device.connectivity.status", data: {} }, { timeoutMs: 25000 });
      if (r.data) this.patch(r.data);
    } catch {}
  }

  private patch(partial: Partial<ConnectivityState>) {
    this.state = { ...this.state, ...partial, capabilities: { ...this.state.capabilities, ...(partial.capabilities || {}) } };
    for (const l of this.listeners) l();
  }
}

export const connectivityStore = new ConnectivityStore();
