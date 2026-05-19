export type NetworkMode = "dhcp" | "manual";

export type NetworkInterfaceModel = {
  interfaceId: string;
  name: string;
  label: string;
  type: "wifi" | "ethernet" | "loopback";
  mac: string;
  address: string;
  netmask: string;
  mode: NetworkMode;
  gateway: string;
  dnsPrimary: string;
  dnsSecondary: string;
  connected: boolean;
};

export type WifiNetwork = {
  id: string;
  label: string;
  strength: number;
  secured: boolean;
  connected?: boolean;
  channel?: string;
  band?: string;
};

export type TabId = "wifi" | "interfaces" | "status";
