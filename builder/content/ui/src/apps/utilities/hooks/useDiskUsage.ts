import React from "react";
import { useFs } from "../../../services/fs/useFs";
import type { DiskUsageNode } from "../types";

async function summarizePath(
  fs: ReturnType<typeof useFs>,
  path: string
): Promise<DiskUsageNode> {
  const result = await fs.list(path);
  let size = 0;
  let files = 0;
  let directories = 0;
  for (const node of result.nodes) {
    const childPath = `${path.replace(/\/+$/, "")}/${node.name}`.replace(/\/+/g, "/");
    if (node.kind === "dir") {
      directories += 1;
      const child = await summarizePath(fs, childPath);
      size += child.size;
      files += child.files;
      directories += child.directories;
    } else {
      files += 1;
      size += node.size ?? 0;
    }
  }
  return { path, size, files, directories };
}

export function useDiskUsage() {
  const fs = useFs();
  const fsRef = React.useRef(fs);
  fsRef.current = fs;
  const [nodes, setNodes] = React.useState<DiskUsageNode[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [reloadToken, setReloadToken] = React.useState(0);

  React.useEffect(() => {
    let cancelled = false;
    async function load() {
      setLoading(true);
      try {
        const f = fsRef.current;
        const root = await f.list("/User");
        const folders = root.nodes.filter((node) => node.kind === "dir");
        const summaries = await Promise.all(
          folders.map((folder) => summarizePath(f, `/User/${folder.name}`.replace(/\/+/g, "/")))
        );
        if (!cancelled) setNodes(summaries.sort((l, r) => r.size - l.size));
      } catch {
        if (!cancelled) setNodes([]);
      } finally {
        if (!cancelled) setLoading(false);
      }
    }
    void load();
    return () => { cancelled = true; };
  }, [reloadToken]);

  const totalSize = React.useMemo(() => nodes.reduce((sum, n) => sum + n.size, 0), [nodes]);

  return { nodes, totalSize, loading, refresh: () => setReloadToken((c) => c + 1) };
}
