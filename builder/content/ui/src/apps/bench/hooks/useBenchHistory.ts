import { useState, useEffect } from "react";
import type { BenchHistory } from "../types/bench";

const benchApi = typeof window !== "undefined" ? window.electronAPI?.bench : undefined;

export function useBenchHistory(): {
  history: BenchHistory | null;
  loading: boolean;
} {
  const [history, setHistory] = useState<BenchHistory | null>(null);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    if (!benchApi) { setLoading(false); return; }
    (async () => {
      try {
        setHistory(await benchApi.history() as BenchHistory | null);
      } catch { setHistory(null); }
      setLoading(false);
    })();
  }, []);
  return { history, loading };
}
