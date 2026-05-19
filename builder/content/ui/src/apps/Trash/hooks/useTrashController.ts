import React from "react";
import { useFs } from "../../../services/fs/useFs";
import { TrashIndex, type TrashEntry } from "../trashIndex";
import { moveToTrash, restoreFromTrash, permanentlyDelete, secureEmptyTrash } from "../../../os/filesystem/fileActions";
import { fileSystemClient } from "../../../os/filesystem/fileSystemClient";
import { kernelClient } from "../../../os/kernel/kernelClient";

export type TimeGroup = "today" | "yesterday" | "thisWeek" | "older";

export type GroupedEntries = {
  group: TimeGroup;
  label: string;
  entries: TrashEntry[];
};

function getTimeGroup(deletedAt: string): TimeGroup {
  const now = new Date();
  const date = new Date(deletedAt);
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const yesterday = new Date(today);
  yesterday.setDate(yesterday.getDate() - 1);
  const thisWeekStart = new Date(today);
  thisWeekStart.setDate(thisWeekStart.getDate() - today.getDay());

  if (date >= today) return "today";
  if (date >= yesterday) return "yesterday";
  if (date >= thisWeekStart) return "thisWeek";
  return "older";
}

const GROUP_LABELS: Record<TimeGroup, string> = {
  today: "Today",
  yesterday: "Yesterday",
  thisWeek: "This Week",
  older: "Older",
};

const GROUP_ORDER: TimeGroup[] = ["today", "yesterday", "thisWeek", "older"];

export function useTrashController() {
  const fs = useFs();
  const [entries, setEntries] = React.useState<TrashEntry[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [selected, setSelected] = React.useState<Set<string>>(new Set());
  const [searchQuery, setSearchQuery] = React.useState("");
  const [emitting, setEmitting] = React.useState(false);
  const [progress, setProgress] = React.useState<{ total: number; current: number } | null>(null);

  const indexRef = React.useRef<TrashIndex | null>(null);

  const loadEntries = React.useCallback(async () => {
    const index = new TrashIndex(fs);
    indexRef.current = index;
    const all = await index.getAll();
    all.sort((a, b) => new Date(b.deletedAt).getTime() - new Date(a.deletedAt).getTime());
    setEntries(all);
    setLoading(false);
  }, [fs]);

  React.useEffect(() => {
    void loadEntries();
  }, [loadEntries]);

  React.useEffect(() => {
    fileSystemClient.watch("/User/Trash").catch(() => {});
    const unsubscribe = kernelClient.on<{ root?: string; path?: string }>("fs.watch.event", (event) => {
      if (event?.root === "/User/Trash" || event?.path?.startsWith("/User/Trash")) {
        void loadEntries();
      }
    });
    return () => {
      unsubscribe();
      fileSystemClient.unwatch("/User/Trash").catch(() => {});
    };
  }, [loadEntries]);

  const filteredEntries = React.useMemo(() => {
    if (!searchQuery.trim()) return entries;
    const q = searchQuery.toLowerCase();
    return entries.filter(
      (e) =>
        e.name.toLowerCase().includes(q) ||
        e.originalPath.toLowerCase().includes(q)
    );
  }, [entries, searchQuery]);

  const groupedEntries = React.useMemo<GroupedEntries[]>(() => {
    const groups = new Map<TimeGroup, TrashEntry[]>();
    for (const g of GROUP_ORDER) groups.set(g, []);
    for (const entry of filteredEntries) {
      const group = getTimeGroup(entry.deletedAt);
      const arr = groups.get(group);
      if (arr) arr.push(entry);
    }
    return GROUP_ORDER.filter((g) => (groups.get(g)?.length ?? 0) > 0).map((g) => ({
      group: g,
      label: GROUP_LABELS[g],
      entries: groups.get(g) || [],
    }));
  }, [filteredEntries]);

  const totalSize = React.useMemo(
    () => entries.reduce((sum, e) => sum + e.size, 0),
    [entries]
  );

  const toggleSelect = React.useCallback((name: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      if (next.has(name)) next.delete(name);
      else next.add(name);
      return next;
    });
  }, []);

  const selectAll = React.useCallback(() => {
    setSelected(new Set(entries.map((e) => e.name)));
  }, [entries]);

  const clearSelection = React.useCallback(() => {
    setSelected(new Set());
  }, []);

  const restoreSingle = React.useCallback(
    async (name: string) => {
      setEmitting(true);
      try {
        await restoreFromTrash(fs, `/User/Trash/${name}`);
        setSelected((prev) => {
          const next = new Set(prev);
          next.delete(name);
          return next;
        });
        await loadEntries();
      } finally {
        setEmitting(false);
      }
    },
    [fs, loadEntries]
  );

  const restoreSelected = React.useCallback(async () => {
    setEmitting(true);
    try {
      for (const name of selected) {
        await restoreFromTrash(fs, `/User/Trash/${name}`);
      }
      setSelected(new Set());
      await loadEntries();
    } finally {
      setEmitting(false);
    }
  }, [fs, selected, loadEntries]);

  const deleteSingle = React.useCallback(
    async (name: string) => {
      setEmitting(true);
      try {
        await permanentlyDelete(fs, `/User/Trash/${name}`);
        setSelected((prev) => {
          const next = new Set(prev);
          next.delete(name);
          return next;
        });
        await loadEntries();
      } finally {
        setEmitting(false);
      }
    },
    [fs, loadEntries]
  );

  const emptyTrash = React.useCallback(
    async (secure = false) => {
      const toDelete = entries.filter((e) => e.name !== ".trash_index.json");
      setProgress({ total: toDelete.length, current: 0 });
      setEmitting(true);
      try {
        let i = 0;
        for (const entry of toDelete) {
          await permanentlyDelete(fs, `/User/Trash/${entry.name}`, secure);
          i++;
          setProgress({ total: toDelete.length, current: i });
        }
        setSelected(new Set());
        setProgress(null);
        await loadEntries();
      } finally {
        setEmitting(false);
        setProgress(null);
      }
    },
    [fs, entries, loadEntries]
  );

  const restoreAll = React.useCallback(async () => {
    const toRestore = entries.filter((e) => e.name !== ".trash_index.json");
    setProgress({ total: toRestore.length, current: 0 });
    setEmitting(true);
    try {
      let i = 0;
      for (const entry of toRestore) {
        await restoreFromTrash(fs, `/User/Trash/${entry.name}`);
        i++;
        setProgress({ total: toRestore.length, current: i });
      }
      setSelected(new Set());
      setProgress(null);
      await loadEntries();
    } finally {
      setEmitting(false);
      setProgress(null);
    }
  }, [fs, entries, loadEntries]);

  return {
    entries,
    filteredEntries,
    groupedEntries,
    loading,
    selected,
    totalSize,
    searchQuery,
    emitting,
    progress,
    setSearchQuery,
    toggleSelect,
    selectAll,
    clearSelection,
    restoreSingle,
    restoreSelected,
    deleteSingle,
    emptyTrash,
    restoreAll,
    refresh: loadEntries,
  };
}
