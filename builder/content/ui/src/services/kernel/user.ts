import { kernelClient } from "../../os/kernel/kernelClient";

export type SamarisUser = {
  username: string;
  displayName: string;
  guest: boolean;
  hasPassword: boolean;
  createdAt?: string | null;
};

export const userKernel = {
  async create(username: string, displayName: string, password: string): Promise<SamarisUser> {
    const res = await kernelClient.request<SamarisUser>({ type: "user.create", data: { username, displayName, password } });
    if (!res.data) throw new Error("user_create_failed");
    return res.data;
  },

  async login(username: string, password: string): Promise<SamarisUser> {
    const res = await kernelClient.request<SamarisUser>({ type: "user.login", data: { username, password } });
    if (!res.data) throw new Error("user_login_failed");
    return res.data;
  },

  async list(): Promise<SamarisUser[]> {
    const res = await kernelClient.request<SamarisUser[]>({ type: "user.list", data: {} });
    return res.data || [];
  },

  async active(): Promise<SamarisUser | null> {
    const res = await kernelClient.request<SamarisUser>({ type: "user.active", data: {} });
    return res.data || null;
  },

  async delete(username: string): Promise<void> {
    await kernelClient.request({ type: "user.delete", data: { username } });
  },

  async update(username: string, updates: { displayName?: string; password?: string }): Promise<SamarisUser> {
    const res = await kernelClient.request<SamarisUser>({ type: "user.update", data: { username, ...updates } });
    if (!res.data) throw new Error("user_update_failed");
    return res.data;
  },
};
