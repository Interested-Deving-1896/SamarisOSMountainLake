import { kernelClient } from "../../os/kernel/kernelClient";
import type {
  WineLaunchOptions,
  WineSession,
  WineSessionLogEvent,
  WineSessionLogs,
  WineSessionUpdateEvent,
  WineStatus
} from "./WineTypes";

export const wineApi = {
  async status() {
    const response = await kernelClient.request<WineStatus>({ type: "wine.status", data: {} });
    if (!response.data) throw new Error("wine_status_missing");
    return response.data;
  },

  async launch(exePath: string, options?: WineLaunchOptions) {
    const response = await kernelClient.request<WineSession>(
      { type: "wine.launch", data: { exePath, options: options || {} } },
      { timeoutMs: 12000 }
    );
    if (!response.data) throw new Error("wine_launch_missing");
    return response.data;
  },

  async openConfig(options?: WineLaunchOptions) {
    const response = await kernelClient.request<WineSession>(
      { type: "wine.config", data: { options: options || {} } },
      { timeoutMs: 12000 }
    );
    if (!response.data) throw new Error("wine_config_missing");
    return response.data;
  },

  async stop(sessionId: string) {
    const response = await kernelClient.request<{ ok: boolean; session: WineSession }>({
      type: "wine.stop",
      data: { sessionId }
    });
    if (!response.data) throw new Error("wine_stop_missing");
    return response.data;
  },

  async logs(sessionId: string) {
    const response = await kernelClient.request<WineSessionLogs>({ type: "wine.logs", data: { sessionId } });
    if (!response.data) throw new Error("wine_logs_missing");
    return response.data;
  },

  onSessionLog(handler: (event: WineSessionLogEvent) => void) {
    return kernelClient.on<WineSessionLogEvent>("wine.session.log", handler);
  },

  onSessionUpdate(handler: (event: WineSessionUpdateEvent) => void) {
    return kernelClient.on<WineSessionUpdateEvent>("wine.session.update", handler);
  }
};
