# Other Apps

Additional system applications bundled with Samaris OS.

<br>

## Application Catalog

| App | Description |
|-----|-------------|
| **Notes** | Simple markdown text editor with live preview |
| **Videos** | Video file playback with subtitle support |
| **Print** | Printer management and job queue |
| **App Store** | Browse and install web applications |
| **Utilities** | System tools collection (disk usage analyzer, font viewer, archive manager) |
| **Task Manager** | Process and memory monitoring with kill/renice |
| **System Monitor** | CPU, RAM, disk usage graphs and live metrics |
| **Disk Utility** | Disk partitioning, format, mount/unmount, SMART status |
| **Firewall** | Network security rules via iptables/nftables frontend |
| **Encryption** | LUKS disk encryption setup and key management |
| **About** | System information and credits |
| **Trash** | Recently deleted files with restore/empty options |

<br>

## Integration

All apps listed here follow the standard Samaris app lifecycle:
1. Registered in `/opt/volt/apps/` via JSON manifest
2. Launched by the kernel's `AppRegistry` into a dedicated Electron BrowserWindow
3. Communicate via kernel WebSocket channels
4. Subject to permission-based access control

<br>

## Related

- [App Manifest Reference](../config/app-manifest.md)
- [Kernel App Registry API](../apis/app-registry.md)
- [Adding an App Guide](../guides/adding-an-app.md)

<br>

---

[← Back: Documentation Index](../index.md)
