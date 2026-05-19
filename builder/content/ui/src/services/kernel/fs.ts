import type { FsListResult, FsReadDataUrlResult, FsReadResult, FsService, FsStatResult } from "../fs/types";
import { fileSystemClient } from "../../os/filesystem/fileSystemClient";

export const kernelFs: FsService = {
  async read(path: string): Promise<FsReadResult> {
    return fileSystemClient.read(path);
  },
  async readDataUrl(path: string): Promise<FsReadDataUrlResult> {
    return fileSystemClient.readDataUrl(path);
  },
  async write(path: string, content: string): Promise<void> {
    await fileSystemClient.write(path, content);
  },
  async writeBase64(path: string, base64: string): Promise<void> {
    await fileSystemClient.writeBase64(path, base64);
  },
  async list(path: string): Promise<FsListResult> {
    return fileSystemClient.list(path);
  },
  async stat(path: string): Promise<FsStatResult> {
    return fileSystemClient.stat(path);
  },
  async exists(path: string): Promise<FsStatResult | { exists: false; path: string }> {
    return fileSystemClient.exists(path);
  },
  async mkdir(path: string): Promise<void> {
    await fileSystemClient.mkdir(path);
  },
  async rename(from: string, to: string): Promise<void> {
    await fileSystemClient.rename(from, to);
  },
  async copy(from: string, to: string): Promise<void> {
    await fileSystemClient.copy(from, to);
  },
  async delete(path: string, opts?: { recursive?: boolean }): Promise<void> {
    await fileSystemClient.delete(path, Boolean(opts?.recursive));
  },
  async remove(path: string, opts?: { recursive?: boolean }): Promise<void> {
    await fileSystemClient.remove(path, opts);
  }
};
