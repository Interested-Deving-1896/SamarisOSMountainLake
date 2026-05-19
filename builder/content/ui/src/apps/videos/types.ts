export type VideoAsset = {
  id: string;
  path: string;
  fileName: string;
  title: string;
  format: string;
  size: number;
  src?: string;
};

export type SavedVideoProgress = Record<string, number>;

export type SortMode = "name" | "size" | "format";
