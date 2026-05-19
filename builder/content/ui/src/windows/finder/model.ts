import type { SamarisIconName } from "../../modules/icons";

export type FinderMenuState = {
  x: number;
  y: number;
  targetName: string | null;
};

export type FinderPreview = {
  path: string;
  content: string;
} | null;

export type FinderSearchResult = {
  id: string;
  title: string;
  path: string;
  kind: "dir" | "file";
  modifiedAt?: string | null;
  size?: number | null;
};

export type FinderViewMode = "list" | "grid" | "columns";
export type FinderSortField = "name" | "kind" | "size" | "date";
export type FinderSortOrder = "asc" | "desc";

export type FinderLocation = {
  id: string;
  label: string;
  path: string;
  icon: SamarisIconName;
  disabled?: boolean;
  hint?: string;
  devicePath?: string;
  mounted?: boolean;
  ejectable?: boolean;
};

export type FinderSection = {
  title: string;
  items: FinderLocation[];
};
