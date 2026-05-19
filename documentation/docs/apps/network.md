# Network App

Full network management with three tabs: **Wi-Fi**, **Interfaces**, and **Status**. Communicates with the kernel's `connectivityService` for all wireless operations and `networkKernel` for interface configuration.

<br>

## Wi-Fi Tab

| Feature | Detail |
|---------|--------|
| Toggle | ON / OFF switch (hardware RF kill) |
| Current network | SSID, signal bars, security type, band (2.4/5/6 GHz) |
| Actions | Disconnect, Forget |
| Scan | Sorted by signal strength, refreshed on open |
| Connect | 1-click open, password modal for WPA/WPA2/WPA3 secured |
| Saved networks | List with Forget option per network |

<br>

## Interfaces Tab

- List of all interfaces (WiFi, Ethernet, Loopback)
- Configure IP mode: **DHCP** (default) / **Manual**
- Set IP address, netmask, gateway, DNS servers
- Interface status indicators (link state, speed, MAC address)

<br>

## Status Tab

- Overview: WiFi state, current network, signal strength, band, IP address
- All interfaces listed with addresses and link status
- Connection uptime and data transfer stats

<br>

## Backend

| Layer | Technology |
|-------|-----------|
| Store | `connectivityStore` (WebSocket) |
| Service | `connectivityService.js` (WiFi scan/connect/disconnect ops) |
| Network config | `networkKernel` (IP mode, interface configuration) |
| Kernel WS channels | `network:scan`, `network:connect`, `network:disconnect`, `network:status` |

<br>

## Related

- [AirBar System Panel](../architecture/airbar.md)
- [Kernel WebSocket — WiFi Commands](../apis/kernel-ws.md#wifi)
- [Kernel WebSocket — Network Configuration](../apis/kernel-ws.md#network)
- [VOLT Architecture — Connectivity Service](../architecture/volt-connectivity.md)

<br>

---

[← Back: Documentation Index](../index.md)
