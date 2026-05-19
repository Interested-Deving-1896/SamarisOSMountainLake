function p(msg) { return msg.payload || msg.data || {}; }

async function handle(message, kernel) {
  const data = p(message);
  const c = kernel.connectivity;

  switch (message.type) {
    case "device.list":
      return { type: "device.list.result", data: [...kernel.devices] };
    case "device.status":
      return { type: "device.status.result", data: { count: kernel.devices.length, devices: [...kernel.devices] } };
    case "device.connectivity.status":
      return { type: "device.connectivity.status.result", data: await c.getStatus() };
    case "device.wifi.toggle":
      return { type: "device.wifi.toggle.result", data: await c.toggleWifi(Boolean(data.enabled)) };
    case "device.wifi.connect":
      return { type: "device.wifi.connect.result", data: await c.connectWifi(data.ssid || data.id || data.label, data.password) };
    case "device.wifi.disconnect":
      return { type: "device.wifi.disconnect.result", data: await c.disconnectWifi() };
    case "device.wifi.forget":
      return { type: "device.wifi.forget.result", data: await c.forgetNetwork(data.ssid) };
    case "device.wifi.saved":
      return { type: "device.wifi.saved.result", data: await c.getSavedNetworks() };
    case "device.wifi.autoconnect":
      return { type: "device.wifi.autoconnect.result", data: await c.autoConnect() };
    case "device.bluetooth.toggle":
      return { type: "device.bluetooth.toggle.result", data: await c.toggleBluetooth(Boolean(data.enabled)) };
    case "device.bluetooth.connect":
      return { type: "device.bluetooth.connect.result", data: await c.connectBluetooth(data.id || data.deviceId) };
    case "device.bluetooth.disconnect":
      return { type: "device.bluetooth.disconnect.result", data: await c.disconnectBluetooth(data.id || data.deviceId) };
    case "device.bluetooth.unpair":
      return { type: "device.bluetooth.unpair.result", data: await c.unpairBluetooth(data.id || data.deviceId) };
    case "device.bluetooth.scan":
      return { type: "device.bluetooth.scan.result", data: await c.scanBluetooth() };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
