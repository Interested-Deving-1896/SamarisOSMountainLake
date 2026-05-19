export type DockFolderChild = {
  id: string;
  label: string;
  running?: boolean;
};

export type DockItemModel =
  | { id: string; kind: "app"; label: string; running?: boolean }
  | { id: string; kind: "folder"; label: string; running?: boolean; children: DockFolderChild[] };
