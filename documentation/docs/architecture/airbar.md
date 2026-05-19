# AirBar System Panel

The premium **glass-design** system bar providing quick access to settings, powered by the AirBarBridge in the Volt Unifier.

<br>

## Panels

| Panel | Features |
|-------|----------|
| **WiFi** | Toggle, scan, signal bars, connect (open/secured), disconnect, forget, saved networks, auto-refresh 10s |
| **Bluetooth** | Toggle, paired devices, connect/disconnect, unpair, scan |
| **Sound** | Volume slider (gradient track), mute toggle, output device selection |
| **Battery** | Percentage, charging status, source info |
| **Sidebar** | Notification center, quiet mode, WiFi/BT/Sound status |

<br>

## Architecture

```
AirBar.tsx
├── AirBarProvider (context: activePanel, anchors, theme)
├── AirBarStatusCluster (WiFi / BT / Sound / Battery icons)
├── Panels
│   ├── WifiPanel
│   ├── BluetoothPanel
│   ├── SoundPanel
│   ├── BatteryPanel
│   └── SidebarPanel
```

<br>

## Theme

- **Glass design** with adaptive background sampling
- Dark / light mode via CSS custom properties
- Colors controlled by `createAirBarVars()` which adapts to theme

<br>

## Related

- [Network App](../components/apps/network.md)
- [Theme System](../components/modules/theme-system.md)
- [Styling Guide](../guides/styling-guide.md)

<br>

---

[← Back: Architecture Overview](overview.md)
