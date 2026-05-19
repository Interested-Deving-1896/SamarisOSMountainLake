import { kernelClient } from "../../os/kernel/kernelClient";

export type InstalledApp = {
  appId: string;
  repoName: string;
  url: string;
  path: string;
  installedAt: string;
  status: string;
  lastBuiltAt?: string;
  source?: "app-store";
  manifest?: {
    name?: string | null;
    displayName?: string | null;
    version?: string | null;
    description?: string | null;
    icon?: string | null;
    samaris?: {
      entry?: string | null;
      preferredWidth?: number | null;
      preferredHeight?: number | null;
    } | null;
  } | null;
  launchable?: boolean;
  launchStrategy?: "static-site" | null;
  launchRoot?: string | null;
  launchEntry?: string | null;
  launchUrl?: string | null;
  launchError?: string | null;
};

export type AppStoreActionResult = {
  ok: boolean;
  stage: string;
  logs: string;
  entry?: InstalledApp;
};

export const appStoreKernel = {
  async listInstalled() {
    const response = await kernelClient.request<InstalledApp[]>({ type: "app.listInstalled", data: {} });
    if (!response.data) throw new Error("app_list_installed_missing");
    return response.data;
  },
  async clone(url: string) {
    const response = await kernelClient.request<AppStoreActionResult>({ type: "app.clone", data: { url } }, { timeoutMs: 120000 });
    if (!response.data) throw new Error("app_clone_missing");
    return response.data;
  },
  async build(appId: string) {
    const response = await kernelClient.request<AppStoreActionResult>({ type: "app.build", data: { appId } }, { timeoutMs: 180000 });
    if (!response.data) throw new Error("app_build_missing");
    return response.data;
  },
  async update(appId: string) {
    const response = await kernelClient.request<AppStoreActionResult>({ type: "app.update", data: { appId } }, { timeoutMs: 180000 });
    if (!response.data) throw new Error("app_update_missing");
    return response.data;
  },
  async remove(appId: string) {
    const response = await kernelClient.request<{ ok: boolean }>({ type: "app.remove", data: { appId } }, { timeoutMs: 120000 });
    if (!response.data) throw new Error("app_remove_missing");
    return response.data;
  },
  async startApp(appId: string) {
    const response = await kernelClient.request<{ ok: boolean; port?: number; launchUrl?: string; error?: string }>({ type: "app.start", data: { appId } }, { timeoutMs: 10000 });
    if (!response.data) throw new Error("app_start_missing");
    return response.data;
  },
  async stopApp(appId: string) {
    const response = await kernelClient.request<{ ok: boolean }>({ type: "app.stop", data: { appId } }, { timeoutMs: 10000 });
    if (!response.data) throw new Error("app_stop_missing");
    return response.data;
  }
};
