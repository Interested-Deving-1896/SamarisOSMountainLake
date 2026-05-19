import type { AppWindow } from "../../shell/windowing/types";
import { kernelClient } from "./client";

type KernelState = {
  processes: unknown[];
  windows: Array<AppWindow & { focused?: boolean }>;
  runtimes: unknown[];
  devices: unknown[];
  session: Record<string, unknown>;
};

export async function fetchKernelState(): Promise<KernelState> {
  const response = await kernelClient.request<KernelState>({ type: "system.state", data: {} });
  if (!response.data) {
    throw new Error("kernel_state_missing");
  }
  return response.data;
}

export async function focusKernelWindow(id: string): Promise<void> {
  await kernelClient.request({ type: "window.focus", data: { id } });
}

export async function openKernelWindow(input: {
  id?: string;
  appId: string;
  title: string;
  x: number;
  y: number;
  w: number;
  h: number;
}): Promise<AppWindow> {
  const response = await kernelClient.request<AppWindow>({ type: "window.open", data: input });
  if (!response.data) {
    throw new Error("kernel_window_missing");
  }
  return response.data;
}
