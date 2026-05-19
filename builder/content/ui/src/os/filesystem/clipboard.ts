export type FileClipboardItem = {
  path: string;
  action: "copy" | "move";
};

type ClipboardEntry = {
  text: string;
  timestamp: number;
};

const FILE_CLIPBOARD_KEY = "samaris-os/file-clipboard";
const TEXT_HISTORY_KEY = "samaris-os/clipboard-history";
const MAX_HISTORY = 20;

const electronClipboard = (typeof window !== "undefined" && window.electronAPI?.clipboard) || null;

export const systemClipboard = {
  async readText(): Promise<string> {
    if (electronClipboard) return electronClipboard.readText();
    try { return await navigator.clipboard.readText(); } catch { return ""; }
  },

  async writeText(text: string): Promise<void> {
    if (!text) return;
    if (electronClipboard) await electronClipboard.writeText(text);
    else {
      try { await navigator.clipboard.writeText(text); } catch {}
    }
    addToHistory(text);
  },

  readFiles(): FileClipboardItem | null {
    try {
      const raw = window.localStorage.getItem(FILE_CLIPBOARD_KEY);
      if (!raw) return null;
      const parsed = JSON.parse(raw) as FileClipboardItem;
      if (!parsed?.path || !parsed?.action) return null;
      return parsed;
    } catch { return null; }
  },

  writeFiles(item: FileClipboardItem) {
    window.localStorage.setItem(FILE_CLIPBOARD_KEY, JSON.stringify(item));
  },

  clearFiles() {
    window.localStorage.removeItem(FILE_CLIPBOARD_KEY);
  },
};

function addToHistory(text: string) {
  try {
    const history = readHistory();
    const next = [{ text, timestamp: Date.now() }, ...history.filter((e) => e.text !== text)].slice(0, MAX_HISTORY);
    window.localStorage.setItem(TEXT_HISTORY_KEY, JSON.stringify(next));
  } catch {}
}

export function readHistory(): ClipboardEntry[] {
  try {
    const raw = window.localStorage.getItem(TEXT_HISTORY_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? parsed : [];
  } catch { return []; }
}

export function clearHistory() {
  window.localStorage.removeItem(TEXT_HISTORY_KEY);
}
