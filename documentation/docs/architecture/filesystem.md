# Filesystem

A **virtual filesystem abstraction** providing a unified API over multiple storage backends. It is one of the 26+ handler domains managed by Kernel A.

<br>

## Backends

| Backend | Usage | Description |
|---------|-------|-------------|
| **kernelFs** | Production | Routes all FS operations through Kernel A via WebSocket |
| **httpFs** | Web fallback | HTTP bridge for remote access |
| **mockFs** | Development | Stub filesystem for testing without kernel |

<br>

## API

```ts
interface FsService {
  read(path: string): Promise<{ path: string; content: string }>;
  readDataUrl(path: string): Promise<{ path: string; dataUrl: string }>;
  write(path: string, content: string): Promise<void>;
  writeBase64(path: string, base64: string): Promise<void>;
  list(path: string): Promise<{ path: string; nodes: FsNode[] }>;
  mkdir(path: string): Promise<void>;
  rename(from: string, to: string): Promise<void>;
  copy(from: string, to: string): Promise<void>;
  delete(path: string, opts?: { recursive?: boolean }): Promise<void>;
  remove(path: string, opts?: { recursive?: boolean }): Promise<void>;
}

interface FsNode {
  name: string;
  kind: "dir" | "file";
  size?: number;
  modifiedAt?: string;
}
```

<br>

## File Watching

The filesystem supports change detection via `watch` / `unwatch` methods with polling-based monitoring.

<br>

## Path Resolution

Paths are resolved by the kernel's `fileSystem.js` through the virtual root system:

| Path | Resolves To |
|------|-------------|
| `/User/desktop` | User's home desktop directory |
| `/opt/volt/ai-models/` | AI model storage |
| Path traversal | Blocked by `path.resolve` comparison |

<br>

## Path Whitelist

The Electron IPC layer restricts filesystem access to approved paths:
- `~/.volt` — application data
- `~/Downloads` — user download directory
- User home directories under `/home/samaris/`
- Protected paths (`/etc`, `/boot`, `/sys`, `/proc`) are never accessible through UI APIs

<br>

## Related

- [Filesystem API Reference](../apis/fs-api.md)
- [Filesystem Layout](../system/filesystem-layout.md)
- [Kernel A — Node.js](kernel-node.md)

<br>

---

[← Back: Architecture Overview](overview.md)
