import { useState, useEffect, useCallback } from "react";
import { downloadStore, type DownloadItem } from "./downloadStore";

export function useDownloads() {
  const [items, setItems] = useState<DownloadItem[]>(() => downloadStore.getItems());

  useEffect(() => downloadStore.subscribe(setItems), []);

  const cancel = useCallback((id: string) => downloadStore.cancel(id), []);
  const clearHistory = useCallback(() => downloadStore.clearHistory(), []);
  const removeItem = useCallback((id: string) => downloadStore.removeItem(id), []);

  const active = items.filter((i) => i.state === "downloading");
  const completed = items.filter((i) => i.state === "completed");
  const failed = items.filter((i) => i.state === "failed");

  return { items, active, completed, failed, cancel, clearHistory, removeItem };
}
