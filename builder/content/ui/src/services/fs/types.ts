export type FsNode = {
  name: string;
  kind: "dir" | "file";
  size?: number;
  modifiedAt?: string;
};

export type FsListResult = {
  path: string;
  nodes: FsNode[];
};

export type FsReadResult = {
  path: string;
  content: string;
};

export type FsReadDataUrlResult = {
  path: string;
  dataUrl: string;
};

export type FsStatResult = {
  path: string;
  name: string;
  kind: "dir" | "file";
  size?: number;
  modifiedAt?: string;
  exists: boolean;
};

export interface FsService {
  read(path: string): Promise<FsReadResult>;
  readDataUrl(path: string): Promise<FsReadDataUrlResult>;
  write(path: string, content: string): Promise<void>;
  writeBase64(path: string, base64: string): Promise<void>;
  list(path: string): Promise<FsListResult>;
  stat?(path: string): Promise<FsStatResult>;
  exists?(path: string): Promise<FsStatResult | { exists: false; path: string }>;
  mkdir(path: string): Promise<void>;
  rename(from: string, to: string): Promise<void>;
  copy(from: string, to: string): Promise<void>;
  delete(path: string, opts?: { recursive?: boolean }): Promise<void>;
  remove(path: string, opts?: { recursive?: boolean }): Promise<void>;
}
