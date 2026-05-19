import type { FsListResult, FsReadDataUrlResult, FsReadResult, FsService } from "./types";

const BASE = "";

async function j(url: string, init?: RequestInit) {
  const res = await fetch(url, init);
  const data = await res.json().catch(() => null);
  if (!res.ok || !data || data.ok === false) {
    const msg = data && data.error ? String(data.error) : `HTTP ${res.status}`;
    throw new Error(msg);
  }
  return data;
}

export const httpFs: FsService = {
  async read(path: string): Promise<FsReadResult> {
    const data = await j(`${BASE}/api/fs/read?path=${encodeURIComponent(path)}`);
    return { path: data.path, content: data.content };
  },
  async readDataUrl(path: string): Promise<FsReadDataUrlResult> {
    const data = await j(`${BASE}/api/fs/read-data-url?path=${encodeURIComponent(path)}`);
    return { path: data.path, dataUrl: data.dataUrl };
  },
  async write(path: string, content: string): Promise<void> {
    await j(`${BASE}/api/fs/write`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ path, content })
    });
  },
  async writeBase64(path: string, base64: string): Promise<void> {
    await j(`${BASE}/api/fs/write-base64`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ path, base64 })
    });
  },
  async list(path: string): Promise<FsListResult> {
    const data = await j(`${BASE}/api/fs/list?path=${encodeURIComponent(path)}`);
    return { path: data.path, nodes: data.nodes };
  },
  async mkdir(p: string): Promise<void> {
    await j(`${BASE}/api/fs/mkdir`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ path: p })
    });
  },
  async rename(from: string, to: string): Promise<void> {
    await j(`${BASE}/api/fs/rename`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ from, to })
    });
  },
  async copy(from: string, to: string): Promise<void> {
    await j(`${BASE}/api/fs/copy`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ from, to })
    });
  },
  async remove(p: string, opts?: { recursive?: boolean }): Promise<void> {
    await j(`${BASE}/api/fs/delete`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ path: p, recursive: Boolean(opts?.recursive) })
    });
  },
  async delete(p: string, opts?: { recursive?: boolean }): Promise<void> {
    await j(`${BASE}/api/fs/delete`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ path: p, recursive: Boolean(opts?.recursive) })
    });
  }
};

export async function isHttpFsAvailable(): Promise<boolean> {
  try {
    const res = await fetch(`${BASE}/health`, { cache: "no-store" });
    return res.ok;
  } catch {
    return false;
  }
}
