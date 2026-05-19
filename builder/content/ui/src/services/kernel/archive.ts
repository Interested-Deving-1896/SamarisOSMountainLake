import { kernelClient } from "../../os/kernel/kernelClient";

export type ArchiveEntry = {
  name: string;
  kind: "file" | "dir";
  size?: number;
};

export const archiveKernel = {
  async list(archivePath: string) {
    const response = await kernelClient.request<{ ok: boolean; entries: ArchiveEntry[]; error?: string }>({
      type: "archive.list",
      data: { archivePath }
    });
    if (!response.data) throw new Error("archive_list_missing");
    if (!response.data.ok) throw new Error(response.data.error || "Failed to list archive");
    return response.data.entries;
  },
  async extract(archivePath: string, destDir: string) {
    const response = await kernelClient.request<{ ok: boolean; error?: string; path?: string; files?: string[] }>({
      type: "archive.extract",
      data: { archivePath, destDir }
    });
    if (!response.data) throw new Error("archive_extract_missing");
    if (!response.data.ok) throw new Error(response.data.error || "Extraction failed");
    return response.data;
  }
};
