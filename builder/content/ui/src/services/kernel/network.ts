import { kernelClient } from "../../os/kernel/kernelClient";

export type NetworkInterface = {
  id: string;
  name: string;
  type: "wifi" | "ethernet" | "loopback";
  mac: string;
  address: string;
  netmask: string;
  internal: boolean;
  mode: "dhcp" | "manual";
  gateway: string;
  dnsPrimary: string;
  dnsSecondary: string;
  connected: boolean;
  label: string;
};

export const networkKernel = {
  async list() {
    const response = await kernelClient.request<NetworkInterface[]>({
      type: "network.list",
      data: {}
    });
    if (!response.data) throw new Error("network_list_missing");
    return response.data;
  },
  async setConfig(config: Partial<NetworkInterface> & { interfaceId: string }) {
    const response = await kernelClient.request<{
      applied: boolean;
      note: string;
      interfaces: NetworkInterface[];
    }>({
      type: "network.setConfig",
      data: config
    });
    if (!response.data) throw new Error("network_set_config_missing");
    return response.data;
  }
};
