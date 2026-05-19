import { kernelClient } from "../../os/kernel/kernelClient";

export type PermissionEntry = {
  action: string;
  allowed: boolean;
};

export type AppPermissionEntry = {
  appId: string;
  permissions: PermissionEntry[];
};

export const permissionsKernel = {
  async listAll() {
    const response = await kernelClient.request<AppPermissionEntry[]>({
      type: "permission.listAll",
      data: {}
    });
    if (!response.data) throw new Error("permission_list_all_missing");
    return response.data;
  },
  async set(appId: string, action: string, allowed: boolean) {
    const response = await kernelClient.request<PermissionEntry[]>({
      type: "permission.set",
      data: { appId, action, allowed }
    });
    if (!response.data) throw new Error("permission_set_missing");
    return response.data;
  }
};
