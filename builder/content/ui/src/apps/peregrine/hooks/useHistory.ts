import React, { useState, useCallback, useEffect } from "react";

export type HistoryEntry = { id: string; url: string; title: string; visitedAt: string; favicon?: string };

const HISTORY_KEY = "samaris.peregrine.history";
const BOOKMARKS_KEY = "samaris.peregrine.bookmarks";
const MAX_HISTORY = 200;

export function useHistory() {
  const [history, setHistory] = useState<HistoryEntry[]>(() => {
    try { return JSON.parse(localStorage.getItem(HISTORY_KEY) || "[]"); } catch { return []; }
  });
  const [bookmarks, setBookmarks] = useState<HistoryEntry[]>(() => {
    try { return JSON.parse(localStorage.getItem(BOOKMARKS_KEY) || "[]"); } catch { return []; }
  });

  const saveHistory = useCallback((entries: HistoryEntry[]) => {
    const trimmed = entries.slice(0, MAX_HISTORY);
    localStorage.setItem(HISTORY_KEY, JSON.stringify(trimmed));
    setHistory(trimmed);
  }, []);

  const addToHistory = useCallback((url: string, title: string, favicon?: string) => {
    setHistory((prev) => {
      const next: HistoryEntry[] = [
        { id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`, url, title, visitedAt: new Date().toISOString(), favicon },
        ...prev.filter((e) => e.url !== url),
      ];
      localStorage.setItem(HISTORY_KEY, JSON.stringify(next.slice(0, MAX_HISTORY)));
      return next.slice(0, MAX_HISTORY);
    });
  }, []);

  const addBookmark = useCallback((url: string, title: string) => {
    setBookmarks((prev) => {
      if (prev.some((b) => b.url === url)) return prev;
      const next = [{ id: `bm-${Date.now()}`, url, title, visitedAt: new Date().toISOString() }, ...prev];
      localStorage.setItem(BOOKMARKS_KEY, JSON.stringify(next));
      return next;
    });
  }, []);

  const removeBookmark = useCallback((url: string) => {
    setBookmarks((prev) => {
      const next = prev.filter((b) => b.url !== url);
      localStorage.setItem(BOOKMARKS_KEY, JSON.stringify(next));
      return next;
    });
  }, []);

  const isBookmarked = useCallback((url: string) => bookmarks.some((b) => b.url === url), [bookmarks]);

  const clearHistory = useCallback(() => {
    localStorage.removeItem(HISTORY_KEY);
    setHistory([]);
  }, []);

  return { history, bookmarks, addToHistory, addBookmark, removeBookmark, isBookmarked, saveHistory, clearHistory };
}
