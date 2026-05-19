import { kernelClient } from "../../os/kernel/kernelClient";

export type Printer = {
  id: string;
  name: string;
  status: string;
  source: string;
  uri?: string;
  protocol?: string;
};

export type PrintQueueJob = {
  printerId: string;
  jobId: string;
  summary: string;
};

export type PrintState = {
  printers: Printer[];
  queue: PrintQueueJob[];
};

export const printKernel = {
  async list() {
    const response = await kernelClient.request<PrintState>({ type: "print.list", data: {} });
    if (!response.data) throw new Error("print_list_missing");
    return response.data;
  },
  async add(config: Record<string, unknown>) {
    const response = await kernelClient.request<{ ok: boolean; note: string; printer: Printer }>({
      type: "print.add",
      data: config
    });
    if (!response.data) throw new Error("print_add_missing");
    return response.data;
  },
  async remove(printerId: string) {
    const response = await kernelClient.request<{ ok: boolean }>({ type: "print.remove", data: { printerId } });
    if (!response.data) throw new Error("print_remove_missing");
    return response.data;
  },
  async submit(data: Record<string, unknown>) {
    const response = await kernelClient.request<{ ok: boolean; note: string; log: string }>({
      type: "print.submit",
      data
    });
    if (!response.data) throw new Error("print_submit_missing");
    return response.data;
  }
};
