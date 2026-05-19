import { kernelClient } from "../../os/kernel/kernelClient";

export type BatteryStatus = {
  available: boolean;
  percentage: number;
  charging: boolean;
  lowPower: boolean;
  source: string;
};

export const batteryKernel = {
  async status() {
    const response = await kernelClient.request<BatteryStatus>({
      type: "battery.status",
      data: {}
    });
    if (!response.data) throw new Error("battery_status_missing");
    return response.data;
  }
};
