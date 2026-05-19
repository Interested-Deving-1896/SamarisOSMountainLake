import { kernelClient } from "../../os/kernel/kernelClient";

export type StorageDevice = {
  id: string;
  label: string;
  path: string;
  filesystem: string;
  size: string;
  mounted: boolean;
  mountPath: string;
  removable: boolean;
};

export type StorageStatus = {
  available: boolean;
  mode: "dry-run" | "live";
  firstBootCompleted: boolean;
  mounted: boolean;
  encrypted: boolean;
  mountPoint: string;
  userRoot: string;
  note: string;
  devices: StorageDevice[];
};

export const storageKernel = {
  async status() {
    const response = await kernelClient.request<StorageStatus>({ type: "storage.status", data: {} });
    if (!response.data) throw new Error("storage_status_missing");
    return response.data;
  },
  async devices() {
    const response = await kernelClient.request<StorageDevice[]>({ type: "storage.devices", data: {} });
    return response.data || [];
  },
  async mount(path: string) {
    const response = await kernelClient.request<{ ok: boolean; message: string; devices?: StorageDevice[] }>({
      type: "storage.mount",
      data: { path }
    });
    if (!response.data) throw new Error("storage_mount_missing");
    return response.data;
  },
  async unmount(path: string) {
    const response = await kernelClient.request<{ ok: boolean; message: string; devices?: StorageDevice[] }>({
      type: "storage.unmount",
      data: { path }
    });
    if (!response.data) throw new Error("storage_unmount_missing");
    return response.data;
  }
};
