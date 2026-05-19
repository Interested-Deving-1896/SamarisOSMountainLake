# Filesystem API

The filesystem is accessed through a unified `FsService` interface, exposed via the `filesystem` handler domain of Kernel A (one of 26 handler domains).

<br>

## Interface

```typescript
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

## Path Whitelist

File access is restricted to a whitelist of allowed directories, enforced by path resolution comparison:

| Allowed Path | Purpose |
|-------------|---------|
| `/User/*` | User home directory |
| `/opt/volt/*` | System configuration and data |
| `/tmp/samaris/*` | Temporary files |
| `/run/samaris/*` | Runtime state |

Any path traversal attempt (`..`) outside these directories is **blocked**.

<br>

## Usage in React Components

```tsx
import { useFs } from "../services/fs/useFs";

function MyComponent() {
  const fs = useFs();
  const [files, setFiles] = useState([]);

  useEffect(() => {
    fs.list("/User/Desktop").then(result => setFiles(result.nodes));
  }, [fs]);
}
```

<br>

## Related

- [Kernel WebSocket Protocol](kernel-ws.md)
- [Kernel A — Node.js](../architecture/kernel-node.md)
- [Filesystem Layout](../system/filesystem-layout.md)

<br>

---

[← Back: Documentation Index](../index.md)
