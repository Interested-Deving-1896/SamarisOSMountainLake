import { kernelClient } from "../../os/kernel/kernelClient";

export type SessionSecurityState = {
  guestMode: boolean;
  lockAfterMinutes: number;
  locked: boolean;
  passwordHint: string;
  displayName: string;
  username: string;
  hasPassword?: boolean;
  ok?: boolean;
  message?: string;
};

export const sessionSecurityKernel = {
  async get() {
    const response = await kernelClient.request<SessionSecurityState>({ type: "session.security.get", data: {} });
    if (!response.data) throw new Error("session_security_get_missing");
    return response.data;
  },
  async set(payload: Partial<SessionSecurityState>) {
    const response = await kernelClient.request<SessionSecurityState>({ type: "session.security.set", data: payload });
    if (!response.data) throw new Error("session_security_set_missing");
    return response.data;
  },
  async lock() {
    const response = await kernelClient.request<SessionSecurityState>({ type: "session.lock", data: {} });
    if (!response.data) throw new Error("session_lock_missing");
    return response.data;
  },
  async unlock(password: string, username?: string) {
    const response = await kernelClient.request<SessionSecurityState>({ type: "session.unlock", data: { password, username } });
    if (!response.data) throw new Error("session_unlock_missing");
    return response.data;
  }
};
