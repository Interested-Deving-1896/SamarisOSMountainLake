import { kernelClient } from "../../os/kernel/kernelClient";

export type FirewallRule = {
  id: string;
  direction: "inbound" | "outbound";
  action: "allow" | "deny";
  port: number;
  label: string;
};

export type FirewallState = {
  enabled: boolean;
  inboundPolicy: "allow" | "deny";
  outboundPolicy: "allow" | "deny";
  rules: FirewallRule[];
  systemEnabled: boolean | null;
  platform: string;
};

export const firewallKernel = {
  async list() {
    const response = await kernelClient.request<FirewallState>({ type: "firewall.list", data: {} });
    if (!response.data) throw new Error("firewall_list_missing");
    return response.data;
  },
  async setEnabled(enabled: boolean) {
    const response = await kernelClient.request<FirewallState>({ type: "firewall.enabled", data: { enabled } });
    if (!response.data) throw new Error("firewall_enabled_missing");
    return response.data;
  },
  async setPolicy(direction: "inbound" | "outbound", action: "allow" | "deny") {
    const response = await kernelClient.request<FirewallState>({ type: "firewall.policy", data: { direction, action } });
    if (!response.data) throw new Error("firewall_policy_missing");
    return response.data;
  },
  async addRule(rule: Partial<FirewallRule>) {
    const response = await kernelClient.request<FirewallState>({ type: "firewall.ruleAdd", data: rule });
    if (!response.data) throw new Error("firewall_rule_add_missing");
    return response.data;
  },
  async removeRule(ruleId: string) {
    const response = await kernelClient.request<FirewallState>({ type: "firewall.ruleRemove", data: { ruleId } });
    if (!response.data) throw new Error("firewall_rule_remove_missing");
    return response.data;
  }
};
