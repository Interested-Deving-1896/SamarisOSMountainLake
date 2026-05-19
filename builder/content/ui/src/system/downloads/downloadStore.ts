import { kernelClient } from "../../os/kernel/kernelClient";

export interface DownloadItem {
  id: string;
  filename: string;
  url: string;
  totalBytes: number;
  received: number;
  state: "downloading" | "completed" | "failed" | "cancelled";
  savePath?: string;
  startTime: number;
  error?: string;
}

type Listener = (items: DownloadItem[]) => void;

const STORAGE_KEY = "samaris/downloads/history";
const MAX_HISTORY = 100;

let items: DownloadItem[] = [];
let listeners = new Set<Listener>();

function loadHistory(): DownloadItem[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    return JSON.parse(raw) as DownloadItem[];
  } catch { return []; }
}

function saveHistory() {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(items.slice(0, MAX_HISTORY)));
  } catch {}
}

function notify() {
  for (const cb of listeners) cb([...items]);
}

function addItem(item: DownloadItem) {
  items.unshift(item);
  saveHistory();
  notify();
}

function updateItem(id: string, partial: Partial<DownloadItem>) {
  items = items.map((i) => (i.id === id ? { ...i, ...partial } : i));
  saveHistory();
  notify();
}

export const downloadStore = {
  init() {
    items = loadHistory();
    notify();
  },

  subscribe(cb: Listener): () => void {
    listeners.add(cb);
    cb([...items]);
    return () => { listeners.delete(cb); };
  },

  getItems(): DownloadItem[] {
    return [...items];
  },

  addPending(filename: string, url: string, totalBytes: number, id?: string): string {
    const itemId = id || `dl-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    addItem({ id: itemId, filename, url, totalBytes, received: 0, state: "downloading", startTime: Date.now() });
    return itemId;
  },

  updateProgress(id: string, received: number, totalBytes: number) {
    updateItem(id, { received, totalBytes });
  },

  complete(id: string, savePath?: string) {
    updateItem(id, { state: "completed", savePath });
  },

  fail(id: string, error: string) {
    updateItem(id, { state: "failed", error });
  },

  cancel(id: string) {
    updateItem(id, { state: "cancelled" });
  },

  clearHistory() {
    items = [];
    saveHistory();
    notify();
  },

  removeItem(id: string) {
    items = items.filter((i) => i.id !== id);
    saveHistory();
    notify();
  },
};
