import { kernelClient } from "../../os/kernel/kernelClient";

export type SearchResult = {
  kind: "app" | "file" | "settings";
  id: string;
  title: string;
  subtitle: string;
  path?: string;
  fileKind?: "dir" | "file";
  modifiedAt?: string | null;
  size?: number | null;
};

export const searchKernel = {
  async query(term: string) {
    const response = await kernelClient.request<SearchResult[]>({
      type: "search.query",
      data: { term }
    });
    if (!response.data) throw new Error("search_query_missing");
    return response.data;
  },
  async queryFiles(term: string) {
    const response = await kernelClient.request<SearchResult[]>({
      type: "search.query",
      data: { term, scope: "files" }
    });
    if (!response.data) throw new Error("search_query_missing");
    return response.data;
  }
};
