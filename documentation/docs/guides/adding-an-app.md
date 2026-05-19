# Adding an App

Developers can extend Samaris OS by adding web applications via the VOLT app lifecycle.

<br>

## App Manifest

Create a JSON manifest for your app:

```json
{
  "name": "MyApp",
  "version": "1.0.0",
  "icon": "/opt/myapp/icon.svg",
  "entry": "/opt/myapp/index.html",
  "permissions": ["fs.read", "fs.write", "network.connect"],
  "window": {
    "width": 800,
    "height": 600,
    "resizable": true
  }
}
```

<br>

## Directory Structure

```
/opt/myapp/
├── index.html      ← App entry point
├── icon.svg        ← App icon (SVG or PNG, 128×128 minimum)
├── manifest.json   ← Registration manifest
└── app.js          ← App logic
```

<br>

## Registration

Place the manifest in `/opt/volt/apps/` and restart the desktop session, or use the **App Store** to install from a URL. Apps registered in this directory are automatically loaded by the kernel's `AppRegistry` on boot.

<br>

## Kernel Integration

Apps communicate with the kernel via WebSocket on `ws://localhost:9999`:

```js
const ws = new WebSocket("ws://localhost:9999");

ws.send(JSON.stringify({
  type: "fs.list",
  data: { path: "/User/Desktop" },
  appId: "myapp"
}));
```

<br>

## Permissions

Apps must declare required permissions in their manifest. The kernel enforces namespace-based access control:

| Permission | Access |
|------------|--------|
| `fs.read` | Read any file the user can access |
| `fs.write` | Write to user-writable paths |
| `network.connect` | Make outbound network connections |
| `audio.output` | Play audio through system output |
| `clipboard.read` | Read system clipboard |

See [Security](../architecture/security.md) for details.

<br>

## Related

- [App Store App](../components/apps/app-store.md)
- [App Manifest Reference](../config/app-manifest.md)
- [Kernel App Registry API](../apis/app-registry.md)
- [VOLT Architecture — App Lifecycle](../architecture/volt-app-lifecycle.md)

<br>

---

[← Back: Documentation Index](../index.md)
