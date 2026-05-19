# App Store

Browse, install, and manage web applications for the Samaris desktop environment.

<br>

## Features

- Curated app catalog with categories (Productivity, Media, Development, Games, Utilities)
- One-click install to desktop with automatic icon registration
- Automatic app registration with the kernel
- Version tracking and update notifications
- App uninstall with cleanup
- Search by name or category

<br>

## Architecture

```
AppStore (React)
├── AppGrid (catalog browser with category filters)
├── AppDetail (screenshots, description, version, install button)
├── InstalledApps (manage installed apps — update, uninstall)
├── SearchBar (filter by name/category)
└── useAppRegistry (hook for install/uninstall lifecycle)
```

<br>

## App Manifest

Apps are defined as JSON manifests and stored in `/opt/volt/apps/`:

```json
{
  "name": "My App",
  "version": "1.0.0",
  "icon": "/opt/apps/myapp/icon.png",
  "entry": "/opt/apps/myapp/index.html",
  "permissions": ["fs.read", "network.connect"]
}
```

<br>

## Related

- [Adding an App Guide](../guides/adding-an-app.md)
- [App Manifest Reference](../config/app-manifest.md)
- [Kernel App Registry API](../apis/app-registry.md)
- [VOLT Architecture — App Lifecycle](../architecture/volt-app-lifecycle.md)

<br>

---

[← Back: Documentation Index](../index.md)
