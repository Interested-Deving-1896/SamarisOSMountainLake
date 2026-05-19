import type { FsListResult, FsNode, FsReadDataUrlResult, FsReadResult, FsService } from "./types";

type TreeNode = FsNode & { children?: TreeNode[] };

const MOCK_FS: TreeNode = {
  name: "/",
  kind: "dir",
  children: [
    {
      name: "opt",
      kind: "dir",
      children: [{ name: "volt", kind: "dir", children: [{ name: "boot.html", kind: "file", size: 4096 }] }]
    },
    {
      name: "home",
      kind: "dir",
      children: [{ name: "user", kind: "dir", children: [{ name: "Desktop", kind: "dir", children: [] }] }]
    }
  ]
};

function normalize(path: string): string[] {
  const cleaned = path.trim();
  if (cleaned === "/" || cleaned === "") return [];
  return cleaned.replace(/^\//, "").split("/").filter(Boolean);
}

function findDir(root: TreeNode, segs: string[]): TreeNode | null {
  let cur: TreeNode = root;
  for (const seg of segs) {
    if (cur.kind !== "dir") return null;
    const next = (cur.children ?? []).find((c) => c.kind === "dir" && c.name === seg);
    if (!next) return null;
    cur = next;
  }
  return cur;
}

export const mockFs: FsService = {
  async read(path: string): Promise<FsReadResult> {
    const segs = normalize(path);
    const fileName = segs[segs.length - 1] || "";
    return { path: `/${segs.join("/")}`.replace(/\/+$/, "") || "/", content: `Mock file: ${fileName}` };
  },
  async readDataUrl(path: string): Promise<FsReadDataUrlResult> {
    const segs = normalize(path);
    return {
      path: `/${segs.join("/")}`.replace(/\/+$/, "") || "/",
      dataUrl: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNDAiIGhlaWdodD0iMTgwIiB2aWV3Qm94PSIwIDAgMjQwIDE4MCI+PHJlY3Qgd2lkdGg9IjI0MCIgaGVpZ2h0PSIxODAiIGZpbGw9IiNlMmU4ZjAiLz48dGV4dCB4PSI1MCUiIHk9IjUwJSIgZG9taW5hbnQtYmFzZWxpbmU9Im1pZGRsZSIgdGV4dC1hbmNob3I9Im1pZGRsZSIgZm9udC1zaXplPSIxNiIgZmlsbD0iIzMzNDE1NSI+TW9jayBQaG90bzwvdGV4dD48L3N2Zz4="
    };
  },
  async write(_path: string, _content: string): Promise<void> {
    // no-op for mock
  },
  async writeBase64(_path: string, _base64: string): Promise<void> {
    // no-op for mock
  },
  async list(path: string): Promise<FsListResult> {
    const segs = normalize(path);
    const dir = findDir(MOCK_FS, segs) ?? MOCK_FS;
    const nodes = (dir.children ?? []).map((n) => ({ name: n.name, kind: n.kind, size: n.size, modifiedAt: n.modifiedAt }));
    return { path: `/${segs.join("/")}`.replace(/\/+$/, "") || "/", nodes };
  },
  async mkdir(_path: string): Promise<void> {
    // no-op for mock
  },
  async rename(_from: string, _to: string): Promise<void> {
    // no-op for mock
  },
  async copy(_from: string, _to: string): Promise<void> {
    // no-op for mock
  },
  async remove(_path: string): Promise<void> {
    // no-op for mock
  },
  async delete(_path: string, _opts?: { recursive?: boolean }): Promise<void> {
    // no-op for mock
  }
};
