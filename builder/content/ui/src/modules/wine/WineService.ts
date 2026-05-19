import type {
  WineLaunchOptions,
  WineSession,
  WineSessionLogEvent,
  WineSessionLogs,
  WineSessionUpdateEvent,
  WineStatus
} from "./WineTypes";
import { wineApi } from "./wineApi";

class WineService {
  async getStatus(): Promise<WineStatus> {
    return wineApi.status();
  }

  async launchExe(exePath: string, options?: WineLaunchOptions): Promise<WineSession> {
    return wineApi.launch(exePath, options);
  }

  async openConfig(options?: WineLaunchOptions): Promise<WineSession> {
    return wineApi.openConfig(options);
  }

  async stopSession(sessionId: string) {
    return wineApi.stop(sessionId);
  }

  async getSessionLogs(sessionId: string): Promise<WineSessionLogs> {
    return wineApi.logs(sessionId);
  }

  onSessionLog(handler: (event: WineSessionLogEvent) => void) {
    return wineApi.onSessionLog(handler);
  }

  onSessionUpdate(handler: (event: WineSessionUpdateEvent) => void) {
    return wineApi.onSessionUpdate(handler);
  }
}

export const wineService = new WineService();
export { WineService };
