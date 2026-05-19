import { useState, useCallback } from "react";
import { systemClipboard, readHistory, clearHistory } from "../os/filesystem/clipboard";

export function useClipboard() {
  const [history, setHistory] = useState(() => readHistory());

  const refresh = useCallback(() => setHistory(readHistory()), []);

  const copy = useCallback(async (text: string) => {
    await systemClipboard.writeText(text);
    refresh();
  }, [refresh]);

  const paste = useCallback(async (): Promise<string> => {
    return systemClipboard.readText();
  }, []);

  const copyFile = useCallback((path: string, action: "copy" | "move") => {
    systemClipboard.writeFiles({ path, action });
  }, []);

  const pasteFile = useCallback(async (): Promise<{ path: string; action: "copy" | "move" } | null> => {
    return systemClipboard.readFiles();
  }, []);

  const clearClipHistory = useCallback(() => {
    clearHistory();
    refresh();
  }, [refresh]);

  return { copy, paste, copyFile, pasteFile, history, refresh, clearHistory: clearClipHistory };
}
