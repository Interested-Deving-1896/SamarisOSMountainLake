import { kernelClient } from "../../os/kernel/kernelClient";

export type EncryptionStatus = {
  available: boolean;
  encrypted: boolean;
  platform: string;
  note: string;
};

export const encryptionKernel = {
  async status() {
    const response = await kernelClient.request<EncryptionStatus>({ type: "encryption.status", data: {} });
    if (!response.data) throw new Error("encryption_status_missing");
    return response.data;
  },
  async changePassphrase() {
    const response = await kernelClient.request<{ ok: boolean; note: string }>({
      type: "encryption.luksChangePassphrase",
      data: {}
    });
    if (!response.data) throw new Error("encryption_change_missing");
    return response.data;
  },
  async backupRecoveryPhrase() {
    const response = await kernelClient.request<{ ok: boolean; note: string }>({
      type: "encryption.backupRecoveryPhrase",
      data: {}
    });
    if (!response.data) throw new Error("encryption_backup_missing");
    return response.data;
  },
  async integrityCheck() {
    const response = await kernelClient.request<{ ok: boolean; note: string }>({
      type: "encryption.integrityCheck",
      data: {}
    });
    if (!response.data) throw new Error("encryption_integrity_missing");
    return response.data;
  }
};
