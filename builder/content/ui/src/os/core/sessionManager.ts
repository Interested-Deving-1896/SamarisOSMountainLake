import { kernelClient } from "../kernel/kernelClient";
import { osStore } from "./osStore";

class SessionManager {
  sync(session: Record<string, unknown>) {
    osStore.patch({ session });
  }

  async saveState() {
    const snapshot = osStore.getState();
    const response = await kernelClient.request<{ ok: boolean; session: Record<string, unknown> }>({
      type: "session.save",
      data: { snapshot }
    });
    if (response.data?.session) {
      osStore.patch({ session: response.data.session });
    }
    return response.data;
  }

  async restoreState() {
    const response = await kernelClient.request<{ ok: boolean; session: Record<string, unknown> }>({
      type: "session.restore",
      data: {}
    });
    if (response.data?.session) {
      osStore.patch({ session: response.data.session });
    }
    return response.data;
  }

  getSessionMetrics() {
    const state = osStore.getState();
    const cpu = state.processes.reduce((total, process) => total + process.cpu, 0);
    const memory = state.processes.reduce((total, process) => total + process.memory, 0);
    return {
      cpu: Math.round(cpu * 10) / 10,
      memory: Math.round(memory * 10) / 10,
      windows: state.windows.length,
      processes: state.processes.length
    };
  }
}

export const sessionManager = new SessionManager();
