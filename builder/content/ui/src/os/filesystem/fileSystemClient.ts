import type { FsListResult, FsReadDataUrlResult, FsReadResult, FsStatResult } from "../../services/fs/types";
import { kernelClient } from "../kernel/kernelClient";
import { resolvePath } from "./pathResolver";

class FileSystemClient {
  async read(path: string): Promise<FsReadResult> {
    const resolved = resolvePath(path);
    const response = await kernelClient.request<FsReadResult>({
      type: "fs.read",
      data: { path: resolved }
    });
    if (!response.data) {
      throw new Error("fs_read_missing");
    }
    return response.data;
  }

  async readDataUrl(path: string): Promise<FsReadDataUrlResult> {
    const resolved = resolvePath(path);
    const response = await kernelClient.request<FsReadDataUrlResult>({
      type: "fs.readDataUrl",
      data: { path: resolved }
    });
    if (!response.data) {
      throw new Error("fs_read_data_url_missing");
    }
    return response.data;
  }

  async write(path: string, content: string) {
    const resolved = resolvePath(path);
    await kernelClient.request({
      type: "fs.write",
      data: { path: resolved, content }
    });
  }

  async writeBase64(path: string, base64: string) {
    const resolved = resolvePath(path);
    await kernelClient.request({
      type: "fs.writeBase64",
      data: { path: resolved, base64 }
    });
  }

  async list(path: string): Promise<FsListResult> {
    const resolved = resolvePath(path);
    const response = await kernelClient.request<FsListResult>({
      type: "fs.list",
      data: { path: resolved }
    });
    if (!response.data) {
      throw new Error("fs_list_missing");
    }
    return response.data;
  }

  async stat(path: string): Promise<FsStatResult> {
    const resolved = resolvePath(path);
    const response = await kernelClient.request<FsStatResult>({
      type: "fs.stat",
      data: { path: resolved }
    });
    if (!response.data) {
      throw new Error("fs_stat_missing");
    }
    return response.data;
  }

  async exists(path: string): Promise<FsStatResult | { exists: false; path: string }> {
    const resolved = resolvePath(path);
    const response = await kernelClient.request<FsStatResult | { exists: false; path: string }>({
      type: "fs.exists",
      data: { path: resolved }
    });
    if (!response.data) {
      return { exists: false, path: resolved };
    }
    return response.data;
  }

  async delete(path: string, recursive = false) {
    await kernelClient.request({
      type: "fs.delete",
      data: {
        path: resolvePath(path),
        recursive
      }
    });
  }

  async mkdir(path: string) {
    const resolved = resolvePath(path);
    await kernelClient.request({
      type: "fs.mkdir",
      data: { path: resolved }
    });
  }

  async rename(from: string, to: string) {
    await kernelClient.request({
      type: "fs.rename",
      data: {
        from: resolvePath(from),
        to: resolvePath(to)
      }
    });
  }

  async copy(from: string, to: string) {
    await kernelClient.request({
      type: "fs.copy",
      data: {
        from: resolvePath(from),
        to: resolvePath(to)
      }
    });
  }

  async planTransfer(payload: Record<string, unknown>) {
    const response = await kernelClient.request({
      type: "fs.planTransfer",
      data: payload
    });
    return response.data;
  }

  async commitTransfer(payload: Record<string, unknown>) {
    const response = await kernelClient.request({
      type: "fs.commitTransfer",
      data: payload
    });
    return response.data;
  }

  async remove(path: string, opts?: { recursive?: boolean }) {
    await this.delete(path, Boolean(opts?.recursive));
  }

  async watch(path: string) {
    const resolved = resolvePath(path);
    const response = await kernelClient.request<{ ok: boolean; id: string; path: string }>({
      type: "fs.watch",
      data: { path: resolved }
    });
    return response.data;
  }

  async unwatch(path: string) {
    const resolved = resolvePath(path);
    const response = await kernelClient.request<{ ok: boolean; id: string }>({
      type: "fs.unwatch",
      data: { path: resolved }
    });
    return response.data;
  }
}

export const fileSystemClient = new FileSystemClient();
