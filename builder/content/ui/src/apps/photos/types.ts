export type PhotoAsset = {
  id: string;
  name: string;
  path: string;
  dataUrl: string | null;
  size?: number;
  modifiedAt?: number | string | null;
};

export type ViewMode = "grid" | "single";
export type SortMode = "name" | "size" | "newest";
export type ViewFilter = "all" | "recent";

export type PhotoCollection = {
  sourcePath: string;
  photos: PhotoAsset[];
  loading: boolean;
};
